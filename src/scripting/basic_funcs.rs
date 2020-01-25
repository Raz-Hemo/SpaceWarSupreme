use crate::log::logger;

pub fn error(x: &str) -> () {
    logger().error(x);
}

pub fn warning(x: &str) -> () {
    logger().warning(x);
}

pub fn info(x: &str) -> () {
    logger().info(x);
}

pub fn rand_range<T>(min: T, max: T) -> T
    where T: PartialOrd + rand::distributions::uniform::SampleUniform {
    use rand::Rng;

    if min >= max {
        min
    } else {
        rand::thread_rng().gen_range(min, max)
    }
}