use wasm_bindgen::prelude::*;
use std::io::BufReader;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use libflatterer::{flatten_to_memory, Options, FlatFiles};
use std::collections::HashMap;
use csv::Reader;
use serde_json::json;
use wasm_bindgen_console_logger::DEFAULT_LOGGER;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Efficient {
    #[serde(with = "serde_bytes")]
    pub bytes: Vec<u8>,
}


#[derive(Serialize, Deserialize)]
pub struct FlattererReturn {
    pub files: HashMap<String, Efficient>,
    pub preview: serde_json::Value,
    pub fields: Vec<HashMap<String, String>>,
}


fn fields_output(flat_files: &FlatFiles) -> Vec<HashMap<String, String>> {

    let mut csv_reader = Reader::from_reader(flat_files.files_memory.get("fields.csv").expect("fields csv trusted").as_slice());

    let mut all_fields = vec![];

    for result in csv_reader.deserialize() {
        let record: HashMap<String, String> = result.expect("fields csv trusted");
        all_fields.push(record)
    }
    all_fields
}


fn preview_output(
    flat_files: &FlatFiles,
    fields: &Vec<HashMap<String, String>>,
) -> serde_json::Value {
    let mut previews = vec![];

    let mut tables_reader = Reader::from_reader(flat_files.files_memory.get("tables.csv").expect("table csv trusted").as_slice());

    for row in tables_reader.deserialize() {
        let table_row: HashMap<String, String> = row.expect("tables csv trusted");
        let table = table_row.get("table_name").expect("tables csv trusted").clone();
        let table_title = table_row.get("table_title").expect("tables csv trusted").clone();


        let mut table_fields = vec![];

        for field in fields.iter() {
            if field.get("table_name").expect("table csv trusted") == &table {
                table_fields.push(field.clone());
            }
        }

        let mut reader = Reader::from_reader(flat_files.csv_memory.get(&format!("{}.csv", table_title)).unwrap().as_slice());

        for (row_num, row) in reader.deserialize().enumerate() {
            let row: Vec<String> = row.expect("files trusted");
            for (col_num, item) in row.iter().enumerate() {
                table_fields[col_num].insert(format!("row {}", row_num), item.clone());
            }
            if row_num > 10 {
                break
            }
        }

        let preview = json!({"table_name": table_title, "fields": table_fields});

        previews.push(preview);
    }
    serde_json::to_value(previews).expect("should be valid JSON")
}


fn zip_output(flatfile: &mut  FlatFiles) -> () {

    let mut output = std::io::Cursor::new(vec![]);

    {
        let mut zip = zip::ZipWriter::new(&mut output);

        let options = zip::write::FileOptions::default();

        zip.add_directory(
            "csv",
            options,
        ).unwrap();

        for (key, value) in flatfile.csv_memory.drain(..) {
            zip.start_file(
                format!("csv/{key}"), options
            ).unwrap();
            std::io::copy(&mut value.as_slice(), &mut zip).unwrap();
        }

        for (key, value) in flatfile.files_memory.iter() {
            zip.start_file(
                key, options
            ).unwrap();
            std::io::copy(&mut value.as_slice(), &mut zip).unwrap();
        }

        zip.finish().unwrap();
    }

    flatfile.files_memory.insert("output.zip".into(), output.into_inner());

}


fn get_options(js_options: JsValue) -> Result<Options, JsValue> {
    let value: serde_json::Value = serde_wasm_bindgen::from_value(js_options)?;

    let mut options = Options::builder().build();

    options.csv = true;
    options.xlsx = true;

    options.memory = true;

    options.main_table_name = value["main_table_name"].as_str().unwrap_or("main").into();

    options.inline_one_to_one = value["inline_one_to_one"].as_bool().unwrap_or(false).into();

    options.table_prefix = value["table_prefix"].as_str().unwrap_or("").into();

    options.json_stream = value["json_lines"].as_bool().unwrap_or(false).into();

    options.path_separator = value["path_separator"].as_str().unwrap_or("_").into();

    options.fields_csv_string = value["fields"].as_str().unwrap_or("").into();

    options.tables_csv_string = value["tables"].as_str().unwrap_or("").into();

    options.only_fields = value["fields_only"].as_bool().unwrap_or(false).into();

    options.only_tables = value["tables_only"].as_bool().unwrap_or(false).into();

    let pushdown: String = value["pushdown"].as_str().unwrap_or("").into();

    if !pushdown.is_empty() {
        options.pushdown = vec![pushdown];
    }

    let mut path_vec = vec![];

    let path: String = value["array_key"].as_str().unwrap_or("").into();
    if !path.is_empty() && !options.json_stream {
        path_vec.push(path);
    }
    options.path = path_vec;

    Ok(options)
}

#[wasm_bindgen]
pub fn set_logger() {
    log::set_logger(&DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
}

#[wasm_bindgen]
pub async fn get_url(url: String, options: JsValue) -> Result<JsValue, JsValue> {

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let content = JsFuture::from(resp.text()?).await?.as_string().unwrap_or_default();

    log::info!("");

    return from_string(content, options).await
}


#[wasm_bindgen]
pub async fn from_string(content: String, options: JsValue) -> Result<JsValue, JsValue> {

    log::info!("recieved JSON from javascript");

    let options = get_options(options)?;

    let flatfiles_result = flatten_to_memory(BufReader::new(content.as_bytes()), options);

    log::info!("flatterer finished");

    if let Err(error) = flatfiles_result {
        return Err(JsValue::from_str(&error.to_string()))
    };

    let mut flatfiles = flatfiles_result.unwrap();

    let fields = fields_output(&flatfiles);
    let preview = preview_output(&flatfiles, &fields);

    zip_output(&mut flatfiles);

    log::info!("zipped files");

    let mut files = HashMap::new();
    for (key, value) in flatfiles.files_memory {
        files.insert(key, Efficient {bytes: value});
    }

    let flatterer_return = FlattererReturn {files, preview, fields};

    let serializer = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);

    let output = flatterer_return.serialize(&serializer)?;
    log::info!("serialized JSON");

    Ok(output)
}