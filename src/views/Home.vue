<template>
  <v-container>
    <v-card class="mb-5">
      <v-card-text>
        <strong>
          Convert JSON into tablular data. Most common JSON structures will
          work, so give it a try!
        </strong>
        <br />
        Here is
        <a
          href="http://localhost:8080?main_table_name=prizes&array_key=prizes&url=https%3A%2F%2Fapi.nobelprize.org%2Fv1%2Fprize.json"
        >
          an example converting the noble prize winners JSON API </a
        >.If you need more flexibility try
        <a href="https://github.com/kindly/flatterer">the python libary/cli.</a>
        <br />
        <br />
        <strong>Why flatterer over other JSON to CSV converters?</strong>
        <ul>
          <li>
            One-to-Many relationships are delt with by creating multiple tables
            instead of a single wide table with lots of headers. This makes
            analyzing deeply nested JSON structures easier.
          </li>
          <li>
            The conversions are done in locally in the browser, so the data is
            not sent to a server.
          </li>
          <li>
            Fast and can deal with data ~1GB or more of JSON if you have enough
            RAM.
          </li>
          <li>Lots of options including an XLXS Converter.</li>
        </ul>
      </v-card-text>
    </v-card>
    <v-card>
      <v-row>
        <v-col>
          <v-card-title id="json-input" v-intersect="onIntersect"
            >JSON Input</v-card-title
          >
        </v-col>
        <v-col>
          <v-container>
            <v-btn
              class="float-right"
              color="deep-orange lighten-4"
              :disabled="formState === 'new'"
              @click="reset"
              >reset all</v-btn
            >
          </v-container>
        </v-col>
      </v-row>
      <v-container>
        <v-expansion-panels v-model="panel">
          <v-expansion-panel>
            <v-expansion-panel-header>Upload File</v-expansion-panel-header>
            <v-expansion-panel-content>
              <v-file-input
                v-model="fileUpload"
                label="File input"
                id="file-input"
                name="file"
                outlined
                dense
                required
              ></v-file-input>
            </v-expansion-panel-content>
          </v-expansion-panel>
          <v-expansion-panel>
            <v-expansion-panel-header
              >Download from URL</v-expansion-panel-header
            >
            <v-expansion-panel-content>
              <v-text-field
                outlined
                label="URL of JSON file"
                v-model="url"
                dense
                placeholder="https://link/to/file.json"
              ></v-text-field>
            </v-expansion-panel-content>
          </v-expansion-panel>
          <v-expansion-panel>
            <v-expansion-panel-header>Paste</v-expansion-panel-header>
            <v-expansion-panel-content>
              <v-textarea
                outlined
                v-model="paste"
                label="JSON data"
              ></v-textarea>
            </v-expansion-panel-content>
          </v-expansion-panel>
        </v-expansion-panels>
      </v-container>
      <v-container>
        <v-card>
          <v-container>
            <v-row>
              <v-col>
                <h3 id="options" v-intersect="onIntersect">Options</h3>
              </v-col>
            </v-row>
            <v-row class="mt-0">
              <v-col>
                <v-radio-group
                  class="mt-0"
                  v-model="arrayPosition"
                  row
                  mandatory
                  dense
                  messages="Position where main data array exists."
                >
                  <template v-slot:label>
                    <strong>Position in JSON:</strong>
                  </template>
                  <v-radio label="Object or Array" value="top"></v-radio>
                  <v-radio label="JSON stream" value="stream"></v-radio>
                  <v-radio label="Array in object" value="nested"></v-radio>
                </v-radio-group>
              </v-col>
              <v-col>
                <v-text-field
                  outlined
                  dense
                  label="Key in object of data array"
                  v-model="array_key"
                  messages="The key in the object where the main array of objects exists."
                  :style="{
                    visibility:
                      arrayPosition == 'nested' ? 'visible' : 'hidden',
                  }"
                ></v-text-field>
              </v-col>
            </v-row>
            <v-row>
              <v-col>
                <v-text-field
                  outlined
                  dense
                  messages="Table name that represents main data array in input.  Defaults to `main`."
                  label="Main Table Name"
                  v-model="main_table_name"
                  placeholder="main"
                ></v-text-field>
              </v-col>
              <v-col>
                <v-checkbox
                  outlined
                  dense
                  hide-details="true"
                  v-model="inline_one_to_one"
                  label="Inline arrays with only single item"
                ></v-checkbox>
              </v-col>
            </v-row>
            <v-row>
              <v-col>
                <v-text-field
                  outlined
                  dense
                  messages="Text prefixed to all output table names.  Defaults to no prefix."
                  label="Table Prefix"
                  v-model="table_prefix"
                ></v-text-field>
              </v-col>
              <v-col>
                <v-text-field
                  outlined
                  dense
                  label="Path Separator"
                  v-model="path_separator"
                  placeholder="_"
                  messages="Separator between each part of the output field and table name. Defaults to `_`."
                ></v-text-field>
              </v-col>
              <v-col>
                <v-text-field
                  outlined
                  dense
                  label="Pushdown"
                  v-model="pushdown"
                  placeholder="id"
                  messages="Field to pushdown to child tables"
                ></v-text-field>
              </v-col>
            </v-row>
            <v-row v-if="panel != 2">
              <v-col>
                <v-file-input
                  v-model="fieldsUpload"
                  label="fields.csv file"
                  id="fields-file"
                  name="fields"
                  outlined
                  dense
                  required
                ></v-file-input>
              </v-col>
              <v-col>
                <v-checkbox
                  outlined
                  dense
                  hide-details="true"
                  v-model="fields_only"
                  label="Only output fields in file"
                ></v-checkbox>
              </v-col>
              <v-col>
                <v-file-input
                  v-model="tablesUpload"
                  label="tables.csv file"
                  id="tables-file"
                  name="tables"
                  outlined
                  dense
                  required
                ></v-file-input>
              </v-col>
              <v-col>
                <v-checkbox
                  outlined
                  dense
                  hide-details="true"
                  v-model="tables_only"
                  label="Only output tables in file"
                ></v-checkbox>
              </v-col>
            </v-row>
          </v-container>
        </v-card>
      </v-container>
      <v-container>
        <v-btn
          color="success"
          :disabled="submitButtonDisabled || formState == 'submitted'"
          @click="preview"
          >{{ submitButtonText }}</v-btn
        >
        <v-progress-circular
          indeterminate
          color="grey"
          class="ml-4"
          v-if="!apiStatus && formState == 'submitted'"
        ></v-progress-circular>
      </v-container>
    </v-card>
    <v-card id="error" v-intersect="onIntersect" class="mt-4" v-if="apiError">
      <v-alert prominent type="error">
        Error processing data:
        <br />
        <strong> {{ apiError }} </strong>
        <br />
        Try again with different options or data.
      </v-alert>
    </v-card>
    <v-card class="mt-4" v-if="fileStart">
      <v-card-title id="input-data-preview" v-intersect="onIntersect"
        >Input data preview</v-card-title
      >
      <v-card-text
        >As the transformation failed, here is the initial part of the input
        file to check if it is as you expect:
        <v-sheet color="grey lighten-3 mt-1">
          <pre class="pa-2" style="white-space: pre-wrap">{{ fileStart }}</pre>
        </v-sheet>
      </v-card-text>
    </v-card>

    <v-card class="mt-4" v-if="apiResponse" id="success">
      <v-alert type="success">File Processed Successfully!</v-alert>
    </v-card>

    <v-card class="mt-4" v-if="apiResponse">
      <v-card-title id="tables-preview" v-intersect="onIntersect"
        >Tables Preview</v-card-title
      >
      <v-card-text>
        Below is a preview of the tables that will be created.
      </v-card-text>
      <v-container v-for="table in apiResponse.preview" :key="table.table_name">
        <v-card :id="'table-' + table.table_name" v-intersect="onIntersect">
          <v-card-title class="subtitle-1">
            {{ table.table_name }}
          </v-card-title>
          <v-data-table
            :headers="fieldHeaders"
            :items="table.fields"
            item-key="field_name"
            disable-pagination
            hide-default-footer
          ></v-data-table>
        </v-card>
      </v-container>
    </v-card>

    <v-card class="mt-4" v-if="apiResponse && linkURL" id="download">
      <v-card-title id="tables-preview" v-intersect="onIntersect"
        >Share link</v-card-title
      >
      <v-card-text>
        <a :href="linkURL">{{ linkURL }}</a>
      </v-card-text>
    </v-card>

    <v-card class="mt-4" v-if="apiResponse" id="download">
      <v-container>
        <v-row>
          <v-col>
            <v-btn
              id="output_zip"
              color="success"
              :href="generateDownload('output.zip')"
              download="output.zip"
              >Download Full Zip</v-btn
            >
          </v-col>
          <v-col v-if="apiResponse.files['output.xlsx']">
            <v-btn
              id="output_xlsx"
              color="success"
              :href="generateDownload('output.xlsx')"
              download="output.xlsx"
              >Download xlsx</v-btn
            >
          </v-col>
          <v-col>
            <v-btn
              id="fields_csv"
              color="success"
              :href="generateDownload('fields.csv')"
              download="fields.csv"
              >Download fields.csv</v-btn
            >
          </v-col>
          <v-col>
            <v-btn
              id="tables_csv"
              color="success"
              :href="generateDownload('tables.csv')"
              download="tables.csv"
              >Download tables.csv</v-btn
            >
          </v-col>
          <v-col>
            <v-btn
              id="datapackage_json"
              color="success"
              :href="generateDownload('datapackage.json')"
              download="datapackage.json"
              >Download datapackage.json</v-btn
            >
          </v-col>
        </v-row>
      </v-container>
    </v-card>
  </v-container>
</template>
<script>
let string_params = [
  "main_table_name",
  "table_prefix",
  "path_separator",
  "array_key",
  "json_schema",
  "pushdown",
];

let bool_params = ["inline_one_to_one", "fields_only", "tables_only"];

function defaultData(from_url) {
  let data = {
    panel: 0,
    fileUpload: null,
    url: "",
    paste: "",
    linkURL: "",
    useTitle: ["No Title", "Full Title", "Slug", "Underscore Slug"],
    arrayPosition: "top",
    array_key: "",
    path_separator: "",
    table_prefix: "",
    schemaTitle: "No Title",
    json_schema: "",
    main_table_name: "",
    inline_one_to_one: false,
    fieldsUpload: null,
    fields_only: false,
    tablesUpload: null,
    tables_only: false,
    pushdown: "",
    id: "",
    formState: "new",
    fileStart: "",
    submitType: "",
    apiError: "",
    apiResponse: null,
    apiStatus: null,

    fieldHeaders: [
      { text: "Field Name", value: "field_title" },
      { text: "Field Type", value: "field_type" },
      { text: "Row Count", value: "count" },
      { text: "Value in first row", value: "row 0" },
      { text: "Value in second row", value: "row 1" },
      { text: "Value in third row", value: "row 2" },
    ],
  };
  if (from_url) {
    let search = new URLSearchParams(new URL(window.location.href).search);
    let url = search.get("url");
    if (url) {
      data["panel"] = 1;
      data["url"] = url;
    }

    for (const element of string_params) {
      let value = search.get(element);
      if (value) {
        data[element] = value;
      }
    }

    for (const element of bool_params) {
      let value = search.get(element);
      if (value) {
        data[element] = true;
      }
    }
    let stream = search.get("stream");
    if (stream) {
      data["arrayPosition"] = "stream";
    }
    let nested = search.get("array_key");
    if (nested) {
      data["arrayPosition"] = "nested";
    }
  }
  return data;
}

export default {
  name: "Home",
  data: () => defaultData(true),
  mounted() {
    let run = async () => {
      await window.wasm_bindgen("/js/flatterer_lite_bg.wasm");
      window.wasm_bindgen.set_logger();
      if (this.url) {
        this.preview();
      }
    };
    run();
  },
  updated() {
    if (this.apiResponse) {
      let search = new URLSearchParams(new URL(window.location.href).search);
      let download = search.get("download");
      if (download) {
        let element = document.getElementById(download);
        element.click();
      }
    }
  },
  watch: {
    apiError(newError) {
      this.$store.commit("setSection", {
        name: "error",
        value: newError ? true : false,
      });
    },
    apiResponse(newResponse) {
      let value = false;
      if (newResponse) {
        value = newResponse.preview.map((value) => {
          return value.table_name;
        });
      }
      this.$store.commit("setSection", { name: "tables", value });
    },
    formChanged() {
      this.formState = "changed";
      this.id = "";
      this.fileStart = "";
      this.submitType = "";
      this.apiError = "";
      this.apiResponse = null;
      this.apiStatus = null;
    },
  },
  computed: {
    formChanged() {
      return [
        this.panel,
        this.fileUpload,
        this.url,
        this.paste,
        this.arrayPosition,
        this.array_key,
        this.path_separator,
        this.table_prefix,
        this.schemaTitle,
        this.json_schema,
        this.main_table_name,
        this.inline_one_to_one,
        this.fieldsUpload,
        this.fields_only,
        this.tablesUpload,
        this.tables_only,
        this.pushdown,
      ];
    },
    submitButtonText() {
      const lookup = {
        0: "Upload File and Preview",
        1: "Download URL and Preview",
        2: "Submit JSON and Preview",
      };
      return lookup[this.panel];
    },
    submitButtonDisabled() {
      const lookup = {
        0: this.fileUpload ? true : false,
        1: this.url.startsWith("http"),
        2: this.paste.length > 5,
      };
      return !lookup[this.panel];
    },
  },
  methods: {
    reset() {
      const data = defaultData();
      Object.keys(data).forEach((k) => (this[k] = data[k]));
      this.$nextTick(() => {
        this.formState = "new";
      });
    },
    dataToParams() {
      let params = {};
      let schema_title = {
        "No Title": undefined,
        "Full Title": "full",
        Slug: "slug",
        "Underscore Slug": "underscore_slug",
      }[this.schemaTitle];
      if (schema_title) {
        params.schema_titles = schema_title;
      }
      let simple_params = [
        "inline_one_to_one",
        "main_table_name",
        "table_prefix",
        "path_separator",
        "array_key",
        "json_schema",
        "fields_only",
        "tables_only",
        "pushdown",
      ];
      for (var i in simple_params) {
        let key = simple_params[i];
        if (this[key]) {
          params[key] = this[key];
        }
      }
      if (this.arrayPosition == "stream") {
        params["json_lines"] = true;
      }
      if (this.arrayPosition == "top") {
        params["array_key"] = "";
      }
      return params;
    },
    generateDownload(download) {
      if (!this.apiResponse.files[download]) {
        return "";
      }
      let blob = new File([this.apiResponse.files[download].bytes], download);
      return URL.createObjectURL(blob);
    },
    preview() {
      let params = this.dataToParams();
      params.output_format = "preview";
      const lookup = {
        0: this.upload,
        1: this.downloadURL,
        2: this.submitPaste,
      };

      let fields_finished = !this.fieldsUpload;
      let tables_finished = !this.tablesUpload;

      if (fields_finished && tables_finished) {
        lookup[this.panel](params);
      }

      if (this.fieldsUpload) {
        let fields_reader = new FileReader();
        fields_reader.onload = (evt) => {
          params.fields = evt.target.result;
          fields_finished = true;
          if (tables_finished) {
            lookup[this.panel](params);
          }
        };
        fields_reader.readAsText(this.fieldsUpload);
      }

      if (this.tablesUpload) {
        let tables_reader = new FileReader();
        tables_reader.onload = (evt) => {
          params.tables = evt.target.result;
          tables_finished = true;
          if (fields_finished) {
            lookup[this.panel](params);
          }
        };
        tables_reader.readAsText(this.tablesUpload);
      }

      let original_data = defaultData();

      let newParams = {};

      for (const element of string_params) {
        if (this[element] != original_data[element]) {
          newParams[element] = this[element];
        }
      }
      for (const element of bool_params) {
        if (this[element] != original_data[element]) {
          newParams[element] = "true";
        }
      }
      if (this.arrayPosition == "stream") {
        newParams.stream = "true";
        delete newParams.array_key;
      }
      if (this.arrayPosition == "top") {
        delete newParams.array_key;
      }

      if (this.panel == 1 && this.url) {
        newParams.url = this.url;
      }

      this.linkURL = "";
      if (
        this.panel == 1 &&
        this.url &&
        !this.fieldsUpload &&
        !this.tablesUpload
      ) {
        this.linkURL =
          window.location.origin +
          "?" +
          new URLSearchParams(newParams).toString();
      }
    },
    cleanStatus() {
      this.apiStatus = null;
      this.apiError = null;
      this.apiResponse = null;
      this.fileStart = null;
      this.formState = "submitted";
    },
    upload(params) {
      this.cleanStatus();
      let reader = new FileReader();
      reader.onload = async (evt) => {
        try {
          const result = await window.wasm_bindgen.from_bytes(
            new Uint8Array(evt.target.result),
            params
          );
          this.apiResponse = result;
          this.apiStatus = 200;
        } catch (e) {
          this.apiError = e;
          this.apiStatus = 400;
        }
      };
      reader.readAsArrayBuffer(this.fileUpload);
    },
    async downloadURL(params) {
      this.cleanStatus();

      try {
        const result = await window.wasm_bindgen.get_url(this.url, params);
        this.apiResponse = result;
        this.apiStatus = 200;
      } catch (e) {
        this.apiError = e;
        this.apiStatus = 400;
      }
    },
    async submitPaste(params) {
      this.cleanStatus();
      try {
        const result = await window.wasm_bindgen.from_string(
          this.paste,
          params
        );
        this.apiResponse = result;
        this.apiStatus = 200;
      } catch (e) {
        this.apiError = e;
        this.apiStatus = 400;
      }
    },
    onIntersect(entries) {
      if (entries[0].isIntersecting) {
        this.$store.commit("setListItem", entries[0].target.id);
      }
    },
  },
};
</script>
