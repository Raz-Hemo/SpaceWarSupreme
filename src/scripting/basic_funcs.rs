use crate::log;

pub fn error(x: &str) -> () {
    logger!().error(x);
}

pub fn warning(x: &str) -> () {
    logger!().warning(x);
}

pub fn info(x: &str) -> () {
    logger!().info(x);
}