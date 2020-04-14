use crate::engine::prelude::*;

#[derive(serde::Deserialize)]
pub struct Localization {
    dict: std::collections::HashMap<String, String>,
}

impl Localization {
    pub fn from(language: &str) -> anyhow::Result<Localization> {
        use anyhow::Context;
        Ok(serde_json::from_str::<'_, Localization>(
            &std::fs::read_to_string(
                std::path::Path::new(consts::LOCALIZATION_PATH)
                .join(language).with_extension(consts::LOCALIZATION_EXTENSION)
            ).context(format!("Localization for {} not found", language))?
        )?)
    }

    pub fn get<'a>(&'a self, key: &'a str) -> &'a str {
        self.dict.get(key).map(|s| &s[..]).unwrap_or(key)
    }

    pub fn get_available_languages() -> Vec<String> {
        utils::get_files_with_extension_from(
            consts::LOCALIZATION_PATH, vec![consts::LOCALIZATION_EXTENSION]
        ).into_iter()
        .map(|p| String::from(p.file_stem().unwrap().to_string_lossy()))
        .collect()
    }
}
