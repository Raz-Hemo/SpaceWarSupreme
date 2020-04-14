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

pub fn clamp<T: PartialOrd> (x: T, min: T, max: T) -> T {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn read_file_lines<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Vec<String>> {
    use std::io::BufRead;
    use anyhow::Context;
    Ok(
        std::io::BufReader::new(
            std::fs::File::open(path.as_ref())
            .context(format!("Failed opening file {}", path.as_ref().to_string_lossy()))?
        ).lines().filter_map(|x| x.ok()).collect()
    )
}

pub fn load_image<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<image::DynamicImage> {
    use anyhow::Context;
    image::open(path.as_ref()).context(
        format!("Failed opening image {}", path.as_ref().to_string_lossy()))
}

pub fn get_engine_dependencies() -> Vec<String> {
    let cargo_toml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "\\Cargo.toml"));
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

/// Checks if the given file has an up to date optimized cache.
/// The engine uses this form on files that take a while to load, like "model.obj"->"model.cache".
pub fn should_load_from_cache<P: AsRef<std::path::Path>>(filename: P) -> (bool, Option<std::path::PathBuf>) {
    let mut filename_cache = filename.as_ref().to_string_lossy().into_owned();
    filename_cache.push_str(".cache");

    if let Ok(Ok(time_file)) = std::fs::metadata(filename).map(|md| md.modified()) {
        if let Ok(Ok(time_cache)) = std::fs::metadata(filename_cache.clone()).map(|md| md.modified()) {
            return (time_file < time_cache, Some(std::path::PathBuf::from(filename_cache)))
        }
    }

    (false, Some(std::path::PathBuf::from(filename_cache)))
}

pub fn extend_filename<P: AsRef<std::path::Path>>(path: P, suffix: &str) -> std::path::PathBuf {
    let filename = path.as_ref().file_stem().and_then(std::ffi::OsStr::to_str).unwrap_or("");
    let extension = path.as_ref().extension().and_then(std::ffi::OsStr::to_str).unwrap_or("");
    let mut extended = String::from(filename);
    extended.push_str(suffix);
    extended.push_str(".");
    extended.push_str(extension);
    std::path::PathBuf::from(path.as_ref().with_file_name(extended))
}