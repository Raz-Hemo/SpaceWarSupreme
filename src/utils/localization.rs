extern crate serde_json;
use std::path::Path;
use std::collections::HashMap;
use std::ffi::OsStr;

const LOCALIZATION_PATH: &str = "./resources/localization";

pub fn load_localization(language: &str) -> super::SWSResult<HashMap<String, String>> {
    let json_result: serde_json::Result<HashMap<String, String>> = serde_json::from_str(
        &super::read_file(Path::new(LOCALIZATION_PATH)
        .join(language).with_extension("json"))?
    );
    if json_result.is_ok() {
        Ok(json_result.unwrap())
    } else {
        Err(format!("Localization for {} not found", language))
    }
}

pub fn get_available_languages() -> Vec<String> {
    match std::fs::read_dir(LOCALIZATION_PATH) {
        Err(_) => vec![],                                                      // Directory opened?
        Ok(dir) => dir.filter_map(|p| p.ok())                                  // Entry successfully read?
                      .map(|p| p.path())                                       // DirEntry -> Path
                      .filter(|p| p.extension() == Some(OsStr::new("json")))   // Path is a json?
                      .filter(|p| p.file_stem().is_some())                     // Remove extension
                      .map(|p| String::from(p.file_stem().unwrap().to_string_lossy()))
                      .collect()
    }
}