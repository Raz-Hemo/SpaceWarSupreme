extern crate specs;
extern crate cgmath;
use specs::{World, WorldExt};
use crate::gameplay::components;

mod config;
mod input;
mod audio;
mod camera;

pub struct Engine {
    pub input: input::InputInfo,
    pub cfg: config::Config,
    pub world: World,
    pub audio: audio::AudioManager,
}

impl Engine {
    pub fn new() -> Engine {
        let mut result = Engine {
            input: input::InputInfo::new(),
            cfg: config::Config::load(),
            audio: audio::AudioManager::new(),
            world: World::new(),
        };

        // ECS init
        result.world.register::<components::PositionComponent>();
        result.world.insert(camera::Camera::new());

        result
    }
}
