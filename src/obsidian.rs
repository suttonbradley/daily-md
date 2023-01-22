use js_sys::{JsString, Promise};
use wasm_bindgen::prelude::*;

// #[macro_export]
// macro_rules! console_log {
//     ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
// }

// #[macro_export]
// macro_rules! console_debug {
//     ($($t:tt)*) => (web_sys::console::debug_1(&format_args!($($t)*).to_string().into()))
// }

#[wasm_bindgen(module = "obsidian")]
extern "C" {
    pub type Plugin;
    pub type Notice;
    pub type App;
    pub type Vault;
    #[derive(Debug)]
    pub type TAbstractFile;
    pub type TFile;
    pub type TFolder;

    #[wasm_bindgen(method, js_name = addCommand)]
    pub fn add_command(this: &Plugin, command: JsValue);

    #[wasm_bindgen(constructor)]
    pub fn new(message: &str) -> Notice;

    #[wasm_bindgen(method, getter)]
    pub fn app(this: &Plugin) -> App;

    #[wasm_bindgen(method, getter)]
    pub fn vault(this: &App) -> Vault;

    #[wasm_bindgen(method, catch)]
    pub fn create(this: &Vault, path: &str, data: &str) -> Result<Promise, JsValue>;

    #[wasm_bindgen(method)]
    pub fn read(this: &Vault, file: &TFile) -> Promise;

    #[wasm_bindgen(method, js_name = getMarkdownFiles)]
    pub fn get_markdown_files(this: &Vault) -> js_sys::Array;

    #[wasm_bindgen(method, getter)]
    pub fn children(this: &TFolder) -> js_sys::Array;

    #[wasm_bindgen(method, getter)]
    pub fn basename(this: &TFile) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn extension(this: &TFile) -> JsString;

    #[wasm_bindgen(method, catch, js_name=getAbstractFileByPath)]
    pub fn get_abstract_file_by_path(
        this: &Vault,
        path: &str,
    ) -> Result<Option<TAbstractFile>, JsValue>;
}

// Function to get obsidian plugin
#[wasm_bindgen(inline_js = "export function plugin() { return app.plugins.plugins['daily-md']; }")]
extern "C" {
    pub fn plugin() -> Plugin;
}
