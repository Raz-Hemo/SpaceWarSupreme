extern crate specs;
extern crate cgmath;
use specs::{World, WorldExt, RunNow};

mod config;
mod graphics;
mod input;
mod audio;
pub mod camera;
pub mod components;
pub mod systems;

pub enum TickResult {
    Continue,
    Exit,
}

pub struct Engine {
    pub renderer: graphics::Renderer,
    pub input: input::InputInfo,
    pub cfg: config::Config,
    pub world: World,
    pub audio: audio::AudioManager,
    system_static_mesh: systems::StaticMeshSystem,
    last_tick: std::time::Instant
}

impl Engine {
    pub fn new(eventloop: &winit::event_loop::EventLoop<()>) -> Engine {
        let renderer = graphics::Renderer::new(eventloop);
        let mut result = Engine {
            last_tick: std::time::Instant::now(),
            input: input::InputInfo::new(),
            cfg: config::Config::load(),
            audio: audio::AudioManager::new(),
            world: World::new(),
            system_static_mesh: systems::StaticMeshSystem::new(renderer.queue.clone()),
            renderer,
        };

        // ECS init
        result.world.register::<components::PositionComponent>();
        result.world.register::<components::StaticMeshComponent>();
        result.world.register::<components::MouseComponent>();
        let mut cam = camera::Camera::new(
            result.cfg.resolution_x,
            result.cfg.resolution_y,
            65.0,
        );
        cam.look_at = cgmath::Point3 { x: 0.0, y: 0.0, z: 0.0 };
        result.world.insert(cam);

        result
    }

    pub fn tick(&mut self) -> TickResult {
        let dt = self.last_tick.elapsed();
        self.last_tick = std::time::Instant::now();
        crate::log::info(&format!("{:?}", self.system_static_mesh.pickables.get(self.renderer.latest_pick_result as usize)));

        // Update camera
        {
        let mut cam = self.world.write_resource::<camera::Camera>();
        let t = dt.as_millis() as f32 * 0.002;
            (*cam).pos = cgmath::Point3 { x: 5.0 * t.cos(), y: 0.0, z: 5.0 * t.sin() };
        }

        TickResult::Continue
    }

    fn renderer_draw_frame(&mut self) -> graphics::DrawResult {
        let cam = self.world.read_resource::<camera::Camera>();
        self.renderer.draw_frame(
            &self.system_static_mesh.next_instance_buffers,
            (*cam).get_view_matrix(),
            (*cam).get_projection_matrix(),
            [self.input.mousex as i32,
             self.input.mousey as i32],
        )
    }

    pub fn draw_frame(&mut self) {
        if !self.input.is_focused {
            return;
        }
        
        self.system_static_mesh.run_now(&self.world);

        match self.renderer_draw_frame() {
            graphics::DrawResult::ResizeNeeded => {
                self.renderer.resize_window([self.cfg.resolution_x, self.cfg.resolution_y]);
                self.renderer_draw_frame();
            }
            _ => ()
        }
    }
}
