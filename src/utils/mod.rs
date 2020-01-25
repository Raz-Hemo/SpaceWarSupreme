pub type SWSResult<T> = Result<T, String>;

pub fn read_file(path: &str) -> SWSResult<String> {
    if let Ok(content) = std::fs::read_to_string(path) {
        Ok(content)
    } else {
        Err(format!("Failed opening file {}", path))
    }
}

pub fn read_file_lines(path: &str) -> SWSResult<Vec<String>> {
    use std::io::BufRead;
    if let Ok(file) = std::fs::File::open(path) {
        Ok(std::io::BufReader::new(file).lines().filter_map(|x| x.ok()).collect())
    } else {
        Err(format!("Failed opening file {}", path))
    }
}
