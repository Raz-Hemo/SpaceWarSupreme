use std::collections::BTreeMap;
use lazy_static::lazy_static;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::log;
use crate::consts;

// Define a struct that can be created at runtime from a string.
macro_rules! deserializable_struct {
    (pub struct $name:ident {
        $($field_name:ident: $field_type:ty = $field_default:expr,)*
    }) => {
        pub struct $name {
            $(pub $field_name: $field_type,)*
        }

        impl $name {
            fn deserialize(m: &BTreeMap<String, String>) -> $name {
                $name {
                    $($field_name: match m.get(stringify!($field_name)) {
                        Some(s) => match s.parse::<$field_type>() {
                            Ok(v) => v,
                            Err(_) => {
                                log::error(&format!("Failed to parse config value {}", stringify!($field_name)));
                                $field_default
                            }
                        },
                        None => {
                            log::error(&format!("No config value found for {}", stringify!($field_name)));
                            $field_default
                        },
                    },)*
                }
            }

            fn serialize(&self) -> BTreeMap<String, String> {
                let mut m: BTreeMap<String, String> = BTreeMap::new();
                $(m.insert(String::from(stringify!($field_name)), self.$field_name.to_string());)*
                m
            }
        }
    }
}

// TODO implement the following types using the trait FromStr:
// bool - done by default
// slider - done by default (maybe add the min and max to the def?)
// enums (auto implement with macro?)
// keybinds (struct with bool ctrl, bool alt, bool shift, and the key)
deserializable_struct! {
    pub struct Config {
        // V1.0.0
        //resolution_x: i32 = 1920,
        //resolution_y: i32 = 1080,

        // Future versions
    }
}

lazy_static! {
    pub static ref CFG: RwLock<Config> = RwLock::new(load_config());
}

pub fn config() -> RwLockReadGuard<'static, Config> {
    CFG.read().expect("Config object is poisoned")
}

pub fn config_mut() -> RwLockWriteGuard<'static, Config> {
    CFG.write().expect("Config object is poisoned")
}

// Reads the configuration file. If it's invalid, default values are loaded instead.
pub fn load_config() -> Config {
    if let Ok(lines) = crate::utils::read_file_lines(consts::CONFIG_FILE_PATH) {
        let mut map: BTreeMap<String, String> = BTreeMap::new();
        for line in lines {
            let split_line: Vec<&str> = line.split('=').collect();
            if split_line.len() != 2 {
                continue;
            }
            map.insert(split_line[0].trim().to_owned(), split_line[1].trim().to_owned());
        }
        Config::deserialize(&map)
    } else {
        Config::deserialize(&BTreeMap::new())
    }
}

pub fn save_config() -> crate::utils::SWSResult<()> {
    if let Ok(file) = std::fs::File::create(consts::CONFIG_FILE_PATH) {
        use std::io::Write;
        let mut file = std::io::LineWriter::new(file);

        for (k, v) in config().serialize().iter() {
            // Best effort saving of config
            if let Err(_) = file.write_all(format!("{}={}\n", k, v).as_bytes()) {
                continue;
            }
        }

        match file.flush() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed flushing config: {}", e)),
        }
    } else {
        Err(String::from("Failed to open config file"))
    }
}