extern crate rhai;

mod basic_funcs;
use rhai::{Engine, RegisterFn};

pub fn new_engine() -> Engine {
    let mut engine = Engine::new();

    engine.register_fn("error", basic_funcs::error);
    engine.register_fn("warning", basic_funcs::warning);
    engine.register_fn("info", basic_funcs::info);

    engine
}