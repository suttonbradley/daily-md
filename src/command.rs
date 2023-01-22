use crate::obsidian::{self, TFile};
use crate::{date::Date, obsidian::Vault};

use js_sys::{JsString, Promise};
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

    async fn get_file_contents(v: &Vault, f: &TFile) -> String {
        JsFuture::from(v.read(f))
            .await
            .expect("Could not poll JsFuture")
            .dyn_into::<JsString>()
            .expect("Could not cast JsValue to JsString")
            .as_string()
            .expect("Could not convert from JsString to Rust String")
    }

    pub fn callback(&self) -> Promise {
        future_to_promise(async move {
            let daily_dir = "daily";
            // Get vault
            let vault = obsidian::plugin().app().vault();

            // Find most recent daily markdown file
            if let Some(daily) = vault
                .get_abstract_file_by_path(daily_dir)
                .expect("Could not make getAbstractFileByPath call")
            {
                trace!("Got \"{daily_dir}\" as TAbstractFile");

                // Cast daily to TFolder
                let daily: obsidian::TFolder =
                    daily.dyn_into().expect("Could not coerce \"{daily_dir}\"");

                // Collect children that succeed in casting to TFile
                let files: Vec<obsidian::TFile> = daily
                    .children()
                    .iter()
                    .filter_map(|child| child.dyn_into::<obsidian::TFile>().ok())
                    .collect();
                trace!("Iterating {} files under the \"daily\" folder", files.len());

                // Get template file contents
                if let Some(template_file) = files
                    .iter()
                    .find(|file| file.basename() == "template" && file.extension() == "md")
                {
                    trace!("Found template file");
                    let contents = Generate::get_file_contents(&vault, template_file).await;
                    debug!("Template file contents:\n{}", contents);
                } else {
                    obsidian::Notice::new(
                        format!("ERROR (daily-md): could not find {daily_dir}/template.md")
                            .as_str(),
                    );
                    // TODO leave function
                }

                // Get latest daily md file
                let latest = files
                    .iter()
                    // Map only valid yyyy-mm-dd.md files to their date and handle
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
                if let Some(latest) = latest {
                    debug!("Latest date is {:?}", latest.1);
                } else {
                    debug!("Could not find any valid files of the form \"yyyy-mm-dd.md\"");
                }

                // TODO deal with None (emtpy iter)
                JsFuture::from(
                    vault
                        .create(
                            format!("{daily_dir}/{}.md", String::from(Date::today())).as_str(),
                            "",
                        )
                        .expect("Could not create daily note"),
                )
                .await
                .expect("Could not poll JsFuture");
            } else {
                obsidian::Notice::new(
                    format!("ERROR (daily-md): \"{daily_dir}\" directory not found").as_str(),
                );
                // TODO leave function
            }

            // Output markdown
            // TODO don't need to return future
            JsFuture::from(
                vault
                    .create("foo.md", "foo\nbar\n")
                    .expect("Could not create daily note"),
            )
            .await
        })
    }
}
