extern crate serde_json;
use std::collections::HashMap;

const LOCALIZATION_PATH: &str = "./resources/localization";
const LOCALIZATION_EXTENSION: &str = "json";

pub fn load_localization(language: &str) -> super::SWSResult<HashMap<String, String>> {
    let json_result: serde_json::Result<HashMap<String, String>> = serde_json::from_str(
        &super::read_file(std::path::Path::new(LOCALIZATION_PATH)
        .join(language).with_extension(LOCALIZATION_EXTENSION))?
    );
    if json_result.is_ok() {
        Ok(json_result.unwrap())
    } else {
        Err(format!("Localization for {} not found", language))
    }
}

pub fn get_available_languages() -> Vec<String> {
    super::get_files_with_extension_from(LOCALIZATION_PATH, LOCALIZATION_EXTENSION)
           .into_iter()
           .map(|p| String::from(p.file_stem().unwrap().to_string_lossy()))
           .collect()
}