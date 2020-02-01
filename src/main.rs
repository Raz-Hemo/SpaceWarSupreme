#[macro_use]
mod log;
mod gameplay;
mod scripting;
mod utils;
mod graphics;
mod config;

fn main()
{
    log::logger().info("Starting Space War Supreme!");


    if let Err(e) = config::save_config() {
        log::logger().error(&format!("Failed saving config, {}", e));
    }
}