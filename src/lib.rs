mod command;
mod date;
mod error;
mod markdown;
mod obsidian;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn onload(plugin: &obsidian::Plugin) {
    #[cfg(feature = "logging")]
    {
        use log;
        console_log::init_with_level(log::Level::Trace).expect("Could not initialize logging");
    }

    plugin.add_command(JsValue::from(command::Generate::default()));
}
