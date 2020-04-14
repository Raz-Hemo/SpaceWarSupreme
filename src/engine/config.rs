use std::collections::BTreeMap;
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
                                crate::log::error(&format!("Failed to parse config value {}", stringify!($field_name)));
                                $field_default
                            }
                        },
                        None => {
                            crate::log::error(&format!("No config value found for {}", stringify!($field_name)));
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
        resolution_x: u32 = consts::DEFAULT_RESOLUTION[0],
        resolution_y: u32 = consts::DEFAULT_RESOLUTION[1],
    }
}

impl Config {
    // Reads the configuration file. If it's invalid, default values are loaded instead.
    pub fn load() -> Config {
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

    pub fn dump(self: &Config) -> anyhow::Result<()> {
        use anyhow::Context;
        use std::io::Write;
        let file = std::fs::File::create(consts::CONFIG_FILE_PATH)
        .context("Failed to open config file")?;
        let mut file = std::io::LineWriter::new(file);

        for (k, v) in self.serialize().iter() {
            // Best effort saving of config
            if let Err(_) = file.write_all(format!("{}={}\n", k, v).as_bytes()) {
                continue;
            }
        }

        file.flush().context("Failed flushing config")
    }
}

