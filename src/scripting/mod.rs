mod basic_funcs;
use rhai::{Engine, RegisterFn};

pub fn new_engine() -> Engine {
    let mut engine = Engine::new();

    engine.register_fn("error", basic_funcs::error);
    engine.register_fn("warning", basic_funcs::warning);
    engine.register_fn("info", basic_funcs::info);
    engine.register_fn("rand_range", basic_funcs::rand_range as fn(i64, i64) -> i64);
    engine.register_fn("rand_range", basic_funcs::rand_range as fn(f64, f64) -> f64);

    engine
}

pub fn get_scripts_in_folder<P: AsRef<std::path::Path>>(path: P) -> Vec<std::path::PathBuf> {
    crate::utils::get_files_with_extension_from(
        path,
        vec![crate::consts::SCRIPT_FILE_EXTENSION]
    )
}
