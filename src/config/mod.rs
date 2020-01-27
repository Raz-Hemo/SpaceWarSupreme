extern crate serde;
extern crate serde_json;

const config_file_path: &str = "./config.txt";

// Reads the configuration file. If it's invalid, default values are loaded instead.
pub fn read_config() -> crate::utils::SWSResult<serde_json::Value> {
    match crate::utils::read_file(config_file_path) { 
        Ok(cfg) => match serde_json::from_str(&cfg) {
            Ok(parsed_cfg) => parsed_cfg,
            Err(_) => Err(String::from("Failed to parse config file")),
        },
        Err() => Err("Failed to open config file");
    }
}

pub fn write_config(content: &serde_json::Value) -> crate::utils::SWSResult<()> {
    if let Ok(s) = serde_json::to_string(content) {
        std::fs::write(config_file_path, s);
        Ok(())
    } else {
        Err(String::from("Failed to serialize object"))
    }
}