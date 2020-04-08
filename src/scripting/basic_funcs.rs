use crate::log;

pub fn error(x: String) -> () {
    log::error(&x);
}

pub fn warning(x: String) -> () {
    log::warning(&x);
}

pub fn info(x: String) -> () {
    log::info(&x);
}

pub fn vec3(x: f64, y: f64, z: f64) -> cgmath::Vector3<f32> {
    cgmath::Vector3::new(x as f32, y as f32, z as f32)
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