extern crate serde_json;

const LOCALIZATION_PATH: &str = "./resources/localization";
const LOCALIZATION_EXTENSION: &str = "json";
type LocalizationDict = std::collections::HashMap<String, String>;
pub struct Localization {
    pub dict: LocalizationDict,
}

impl<'a> Localization {
    pub fn from(language: &str) -> super::SWSResult<Localization> {
        let json_result: serde_json::Result<LocalizationDict> = serde_json::from_str(
            &super::read_file(std::path::Path::new(LOCALIZATION_PATH)
            .join(language).with_extension(LOCALIZATION_EXTENSION))?
        );
        if json_result.is_ok() {
            Ok(Localization{ dict: json_result.unwrap() })
        } else {
            Err(format!("Localization for {} not found", language))
        }
    }

    pub fn get(self: &'a Localization, key: &'a str) -> &str {
        if let Some(val) = self.dict.get(key) {
            &val
        } else {
            key
        }
    }

    pub fn get_available_languages() -> Vec<String> {
        super::get_files_with_extension_from(LOCALIZATION_PATH, LOCALIZATION_EXTENSION)
               .into_iter()
               .map(|p| String::from(p.file_stem().unwrap().to_string_lossy()))
               .collect()
    }
}
