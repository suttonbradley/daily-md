use crate::error::GenerateError;
use crate::markdown::{TodoLine, DAILY_LINE};
use crate::obsidian::{self, TFile};
use crate::{date::Date, obsidian::Vault};

use js_sys::{self, JsString, Promise};
use log::{debug, trace};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::{future_to_promise, JsFuture};

#[wasm_bindgen]
pub struct Generate {
    id: JsString,
    name: JsString,
}

impl Default for Generate {
    fn default() -> Self {
        Generate {
            id: JsString::from("generate"),
            name: JsString::from("Generate today's note"),
        }
    }
}

#[wasm_bindgen]
impl Generate {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> JsString {
        self.id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, id: &str) {
        self.id = JsString::from(id)
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> JsString {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: &str) {
        self.name = JsString::from(name)
    }

    async fn get_file_contents(v: &Vault, f: Option<&TFile>) -> Option<String> {
        if let Some(f) = f {
            Some(
                JsFuture::from(v.read(f))
                    .await
                    .expect("Could not poll JsFuture")
                    .dyn_into::<JsString>()
                    .expect("Could not cast JsValue to JsString")
                    .as_string()
                    .expect("Could not convert from JsString to Rust String"),
            )
        } else {
            None
        }
    }

    async fn generate_today() -> Result<(), GenerateError> {
        // TODO make this configurable
        let dir_name = "daily";

        // Get vault
        let vault = obsidian::plugin().app().vault();

        // Get daily dir and cast to TFolder
        let daily_dir = vault
            .get_abstract_file_by_path(dir_name)
            .map_err(|_| GenerateError::JsFunctionError("Vault.getAbstractFileByPath"))?
            .ok_or(GenerateError::DailyDirNotFound(dir_name.to_string()))?;
        trace!("Got \"{dir_name}\" as TAbstractFile");
        let daily_dir: obsidian::TFolder = daily_dir
            .dyn_into()
            .map_err(|_| GenerateError::Misc("Failed to cast from TAbstractFile to TFolder"))?;

        // Collect TFile-type children
        let files: Vec<TFile> = daily_dir
            .children()
            .iter()
            // Only children that correctly cast to TFile
            .filter_map(|child| child.dyn_into::<obsidian::TFile>().ok())
            .collect();

        // Find latest daily md file, if exists
        let latest_file = files
            .iter()
            // Map valid yyyy-mm-dd.md files to their date and handle
            .filter_map(|f| {
                if f.extension() != "md" {
                    return None;
                }
                if let Ok(d) = Date::try_from(f.basename().as_string().unwrap().as_str()) {
                    Some((f, d))
                } else {
                    None
                }
            })
            .max_by(|(_, d1), (_, d2)| d1.cmp(d2));
        if let Some(latest_file) = &latest_file {
            debug!("Latest date is {:?}", latest_file.1);
        } else {
            debug!("Could not find any valid files of the form \"yyyy-mm-dd.md\"");
        }
        // TODO do this map attached to the max_by once this works
        let latest_file = latest_file.map(|x| x.0);

        // Ensure today's file does not already exist
        if let Some(latest_file) = latest_file {
            if latest_file.basename() == String::from(Date::today()) {
                return Err(GenerateError::DailyFileAlreadyExists);
            }
        }

        // Get latest daily md file contents
        let latest_file_contents = Generate::get_file_contents(&vault, latest_file).await;
        debug!("latest daily md file contents:\n{latest_file_contents:?}");
        // Resolve lines carried over from the previous day
        let carryover_lines = if let Some(c) = latest_file_contents {
            TodoLine::from_file_contents(c)
        } else {
            vec![]
        };

        // Find every-day file, if exists, and get contents
        let every_day_file = files
            .iter()
            .find(|f| f.basename() == "every-day" && f.extension() == "md");
        let every_day_file_contents = Generate::get_file_contents(&vault, every_day_file).await;
        debug!("every-day file contents:\n{every_day_file_contents:?}");

        // Form file today's file
        // Pulls every-day file content first, then carryover from most recent day
        let mut contents = String::new();
        if let Some(c) = every_day_file_contents {
            contents.push_str(DAILY_LINE);
            contents.push_str(c.as_str());
            contents.push('\n');
        }
        for line in carryover_lines {
            contents.push_str(line.to_string().as_str());
            contents.push('\n');
        }

        // Create today's file
        JsFuture::from(
            vault
                .create(
                    format!("{dir_name}/{}.md", String::from(Date::today())).as_str(),
                    contents.as_str(),
                )
                .map_err(|_| GenerateError::JsFunctionError("Vault.create"))?,
        )
        .await
        .map_err(|_| GenerateError::Misc("Failed to write daily file"))?;

        Ok(())
    }

    pub fn callback(&self) -> Promise {
        future_to_promise(async move {
            match Generate::generate_today().await {
                Ok(_) => Ok(JsValue::undefined()),
                Err(e) => {
                    let msg = format!("ERROR (daily-md): {e}");
                    obsidian::Notice::new(&msg);
                    Err(JsValue::from(js_sys::Error::new(&msg)))
                }
            }
        })
    }
}
