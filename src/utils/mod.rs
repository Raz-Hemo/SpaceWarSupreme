pub type SWSResult<T> = Result<T, String>;

mod localization;
pub use localization::Localization;

pub fn error_msgbox(message: &str) {
    #[cfg(target_os = "windows")]
    {
        use std::io::Write;

        // The win_msgbox script creates a messagebox from stdin.
        if let Ok(mut p) = std::process::Command::new("cscript")
                    .arg("win_msgbox.vbs")
                    .stdin(std::process::Stdio::piped())
                    .spawn() {
            if let Some(stdin) = p.stdin.as_mut() {
                if stdin.write_all(message.as_bytes()).is_ok() {
                    if p.wait().is_err() {
                        // nothing to do here.
                    }
                }
            }
        }
    }
}

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

pub fn get_game_dependencies() -> Vec<String> {
    let cargo_toml = include_str!("../../Cargo.toml");
    let mut result = Vec::<String>::new();
    let mut dependencies_found = false;

    for line in cargo_toml.lines() {
        if dependencies_found {
            let split_line = line.split(" = ").collect::<Vec<&str>>();
            if split_line.len() != 2 {
                break;
            }
            
            result.push(String::from(line));
        }
        if line == "[dependencies]" {
            dependencies_found = true;
        }
    }
    result
}

pub fn get_files_with_extension_from<P>(dir: P, extensions: Vec<&str>) -> Vec<std::path::PathBuf> 
        where P: AsRef<std::path::Path> {
    match std::fs::read_dir(dir) {
        Err(_) => vec![],                                                      // Directory opened?
        Ok(dir) => dir.filter_map(|p| p.ok())                                  // Entry successfully read?
                      .map(|p| p.path())                                       // DirEntry -> Path
                      .filter(|p| p.file_stem().is_some())                     // Remove extension
                      .filter(|p| extensions.iter().any(|&e| e == p.extension().unwrap().to_string_lossy().as_ref()))   // Path is a json?
                      .collect()
    }
}
