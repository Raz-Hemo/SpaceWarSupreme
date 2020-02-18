use crate::log;

pub fn error(x: &str) -> () {
    log::error(x);
}

pub fn warning(x: &str) -> () {
    log::warning(x);
}

pub fn info(x: &str) -> () {
    log::info(x);
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