extern crate rhai;

use std::fs;
use rhai::{Engine, RegisterFn};


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

#[derive(Debug)]
#[derive(Clone)]
pub struct Star {
    x: f32,
}

pub fn generate_star() -> Star {
    Star {
        x: 5.0,
    }
}

pub fn execute_map_generator(script_path: &str) -> Result<Vec<Star>, Box<dyn std::error::Error + 'static>> {
    let script: String = fs::read_to_string(script_path)?;
    let mut engine = Engine::new();

    engine.register_fn("make_random_star", generate_star);
    engine.register_fn("make_star_vector", Vec::new as fn()->Vec<Star>);
    engine.register_fn("push", Vec::push as fn(&mut Vec<Star>, Star));

    match engine.eval::<Vec<Star>>(&script) {
        Ok(result) => Ok(result),
        Err(e) => Err(std::boxed::Box::from(e)),
    }
}