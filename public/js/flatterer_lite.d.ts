declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	*/
	export function set_logger(): void;
	/**
	* @param {string} url
	* @param {any} options
	* @returns {Promise<any>}
	*/
	export function get_url(url: string, options: any): Promise<any>;
	/**
	* @param {string} content
	* @param {any} options
	* @returns {Promise<any>}
	*/
	export function from_string(content: string, options: any): Promise<any>;
	/**
	* @param {Uint8Array} content
	* @param {any} options
	* @returns {Promise<any>}
	*/
	export function from_bytes(content: Uint8Array, options: any): Promise<any>;
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly set_logger: () => void;
  readonly get_url: (a: number, b: number, c: number) => number;
  readonly from_string: (a: number, b: number, c: number) => number;
  readonly from_bytes: (a: number, b: number, c: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9b7c48c8c0c95c22: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__hb85c522e197492cf: (a: number, b: number, c: number, d: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
