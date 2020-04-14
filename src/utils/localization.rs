const LOCALIZATION_PATH: &str = "./resources/localization";
const LOCALIZATION_EXTENSION: &str = "json";

#[derive(serde::Deserialize)]
pub struct Localization {
    dict: std::collections::HashMap<String, String>,
}

impl Localization {
    pub fn from(language: &str) -> anyhow::Result<Localization> {
        use anyhow::Context;
        Ok(serde_json::from_str::<'_, Localization>(
            &std::fs::read_to_string(
                std::path::Path::new(LOCALIZATION_PATH)
                .join(language).with_extension(LOCALIZATION_EXTENSION)
            ).context(format!("Localization for {} not found", language))?
        )?)
    }

    pub fn get<'a>(&'a self, key: &'a str) -> &'a str {
        self.dict.get(key).map(|s| &s[..]).unwrap_or(key)
    }

    pub fn get_available_languages() -> Vec<String> {
        super::get_files_with_extension_from(LOCALIZATION_PATH, vec![LOCALIZATION_EXTENSION])
               .into_iter()
               .map(|p| String::from(p.file_stem().unwrap().to_string_lossy()))
               .collect()
    }
}
