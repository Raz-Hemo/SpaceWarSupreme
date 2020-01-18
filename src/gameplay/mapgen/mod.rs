use std::fs;

// Returns a list of all the installed scripts for generating a map
pub fn get_mapgen_scripts() -> Vec<String>
{
    let dir = fs::read_dir("./scripts/mapgen/");

    // Return empty list on error
    if dir.is_err() {
        return vec![];
    }
    
    let mut result: Vec<String> = vec![];
    for e in dir.unwrap() {
        // Skip failures of individual file reading
        if e.is_err() {
            continue;
        }
        let p = e.unwrap().path();

        // Skip failures of converting to string
        let path: Option<&str> = p.to_str();
        if path.is_none() {
            continue;
        }

        result.push(path.unwrap().to_owned());
    }

    result
}