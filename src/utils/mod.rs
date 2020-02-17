extern crate image;
use std::collections::HashMap;

pub type SWSResult<T> = Result<T, String>;

pub fn read_file<P: AsRef<std::path::Path>>(path: P) -> SWSResult<String> {
    if let Ok(content) = std::fs::read_to_string(path.as_ref()) {
        Ok(content)
    } else {
        Err(format!("Failed opening file {}", path.as_ref().to_string_lossy()))
    }
}

pub fn read_file_lines<P: AsRef<std::path::Path>>(path: P) -> SWSResult<Vec<String>> {
    use std::io::BufRead;
    if let Ok(file) = std::fs::File::open(path.as_ref()) {
        Ok(std::io::BufReader::new(file).lines().filter_map(|x| x.ok()).collect())
    } else {
        Err(format!("Failed opening file {}", path.as_ref().to_string_lossy()))
    }
}

pub fn load_image<P: AsRef<std::path::Path>>(path: P) -> SWSResult<image::DynamicImage> {
    if let Ok(img) = image::open(path.as_ref()) {
        Ok(img)
    } else {
        Err(format!("Failed opening image {}", path.as_ref().to_string_lossy()))
    }
}

pub fn get_game_dependencies() -> HashMap<String, String> {
    let cargo_toml = include_str!("../../Cargo.toml");
    let mut result = HashMap::<String, String>::new();
    let mut dependencies_found = false;

    for line in cargo_toml.lines() {
        if dependencies_found && line == "\n" {
            break;
        }
        if dependencies_found {
            result.insert(line.split(" = ")[0], line.split(" = ")[1].trim("\""));
        }
        if line == "[dependencies]" {
            dependencies_found = true;
        }
    }
    result
}