extern crate specs;
extern crate cgmath;
use specs::{World, WorldExt, RunNow, Builder};
use systems::MeshInstance;

mod config;
mod graphics;
mod input;
mod audio;
mod camera;
mod components;
mod systems;

pub struct Engine {
    pub renderer: graphics::Renderer,
    pub input: input::InputInfo,
    pub cfg: config::Config,
    pub world: World,
    pub audio: audio::AudioManager,
    system_static_mesh: systems::StaticMeshSystem,
    start_time: std::time::Instant
}

impl Engine {
    pub fn new(eventloop: &winit::event_loop::EventLoop<()>) -> Engine {
        let renderer = graphics::Renderer::new(eventloop);
        let mut result = Engine {
            start_time: std::time::Instant::now(),
            input: input::InputInfo::new(),
            cfg: config::Config::load(),
            audio: audio::AudioManager::new(),
            world: World::new(),
            system_static_mesh: systems::StaticMeshSystem::new(
                renderer.device.clone(), renderer.queue.clone()),
            renderer,
        };

        // ECS init
        result.world.register::<components::PositionComponent>();
        result.world.register::<components::StaticMeshComponent>();
        result.world.create_entity().with(components::StaticMeshComponent{
            model: result.renderer.models_manager.get_id("cube"),
            mesh_instance: MeshInstance::new()
        }).build();
        let mut cam = camera::Camera::new(
            result.cfg.resolution_x,
            result.cfg.resolution_y,
            70.0,
        );
        cam.look_at = cgmath::Point3 { x: 0.0, y: 0.0, z: 0.0 };
        result.world.insert(cam);

        result
    }

    fn renderer_draw_frame(&mut self) -> graphics::DrawResult {
        let cam = self.world.read_resource::<camera::Camera>();
        self.renderer.draw_frame(
            &self.system_static_mesh.next_instance_buffers,
            (*cam).get_view_matrix(),
            (*cam).get_projection_matrix()
        )
    }

    pub fn draw_frame(&mut self) {
        self.system_static_mesh.run_now(&self.world);

        {
            let mut cam = self.world.write_resource::<camera::Camera>();
            let t = self.start_time.elapsed().as_millis() as f32 * 0.002;
            (*cam).pos = cgmath::Point3 { x: 5.0 * t.cos(), y: 2.0 * t.sin(), z: 5.0 * t.sin() };
        }

        match self.renderer_draw_frame() {
            graphics::DrawResult::ResizeNeeded => {
                self.renderer.resize_window([self.cfg.resolution_x, self.cfg.resolution_y]);
                self.renderer_draw_frame();
            }
            _ => ()
        }
    }
}
