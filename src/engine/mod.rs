extern crate specs;
use specs::{World, WorldExt};
use crate::gameplay::components;

mod config;
mod input;
mod audio;

pub struct Engine<'a> {
    pub input: input::InputInfo<'a>,
    pub cfg: config::Config,
    pub world: World,
    pub audio: audio::AudioManager,
}

impl<'a> Engine<'a> {
    pub fn new() -> Engine<'a> {
        let result = Engine {
            input: input::InputInfo::new(),
            cfg: config::Config::load(),
            audio: audio::AudioManager::new(),
            world: World::new(),
        };

        // ECS init
        result.world.register::<components::PositionComponent>();

        result
    }
}
