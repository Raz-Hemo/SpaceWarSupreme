extern crate specs;
extern crate cgmath;
use specs::{World, WorldExt};

pub mod config;
mod input;
mod audio;
mod camera;
mod components;
mod models;

pub struct Engine {
    pub renderer: crate::graphics::renderer::Renderer,
    pub input: input::InputInfo,
    pub cfg: config::Config,
    pub world: World,
    pub audio: audio::AudioManager,
}

impl Engine {
    pub fn new(eventloop: &winit::event_loop::EventLoop<()>) -> Engine {
        let mut result = Engine {
            input: input::InputInfo::new(),
            cfg: config::Config::load(),
            audio: audio::AudioManager::new(),
            world: World::new(),
            renderer: crate::graphics::renderer::Renderer::new(eventloop)
        };

        // ECS init
        result.world.register::<components::PositionComponent>();
        result.world.register::<components::StaticMeshComponent>();
        //result.world.insert(camera::Camera::new());

        result
    }

    pub fn draw_frame(&mut self) {
        match self.renderer.draw_frame() {
            crate::graphics::renderer::DrawResult::ResizeNeeded => {
                self.renderer.resize_window([self.cfg.resolution_x, self.cfg.resolution_y]);
                self.renderer.draw_frame();
            }
            _ => ()
        }
    }
}
