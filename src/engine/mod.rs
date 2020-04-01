use specs::RunNow;
pub mod config;
pub mod input;
pub mod audio;
pub mod graphics;
pub mod camera;
pub mod components;
pub mod systems;
use crate::gameplay::levels::Level;

pub enum TickResult {
    Continue,
    Exit,
}

pub struct Engine {
    level: Box<dyn Level>,
    last_tick: std::time::Instant,
    picked_index: Option<(u32, specs::Entity)>,
    system_static_mesh: systems::StaticMeshSystem,
    system_scripting: systems::ScriptingSystem,
    system_mouse: systems::MouseSystem,
    pub input: input::InputInfo,
    pub cfg: config::Config,
    pub audio: audio::AudioManager,
    pub renderer: graphics::Renderer,
}

impl Engine {
    pub fn new(eventloop: &winit::event_loop::EventLoop<()>, level: Box<dyn Level>) -> Engine {
        let renderer = graphics::Renderer::new(eventloop);
        Engine {
            level,
            last_tick: std::time::Instant::now(),
            picked_index: None,
            system_static_mesh: systems::StaticMeshSystem::new(renderer.queue.clone()),
            system_scripting: systems::ScriptingSystem::new(),
            system_mouse: systems::MouseSystem::new(),
            input: input::InputInfo::new(),
            cfg: config::Config::load(),
            audio: audio::AudioManager::new(),
            renderer,
        }
    }

    pub fn tick(&mut self) -> TickResult {
        let dt = self.last_tick.elapsed();
        self.last_tick = std::time::Instant::now();

        // iter_render is guaranteed not to change since the last call to draw_frame
        // so it is okay to use the world indices again
        self.system_mouse.new_frame(self.picked_index, self.input.drain_mouse_events());
        for space in self.level.iter_render() {
            self.system_mouse.run_now(space);
        }

        for space in self.level.iter_tickable() {
            // TODO run all the systems
            // TODO if script system requests exit, return TickResult::Exit
        }

        TickResult::Continue
    }

    fn renderer_draw_frame(&mut self, cam: &camera::Camera) -> graphics::DrawResult {
        for space in self.level.iter_render() {
            self.system_static_mesh.run_now(&space);
        }
        let (instance_buffers, pickables) = self.system_static_mesh.get_instances_and_flush();
        let result = self.renderer.draw_frame(
            &instance_buffers,
            cam.get_view_matrix(),
            [self.input.mousex as u32,
             self.input.mousey as u32],
        );

        self.picked_index = pickables.get(self.renderer.latest_pick_result as usize).copied();
        crate::log::info(&format!("{:?}", self.picked_index));

        result
    }

    pub fn draw_frame(&mut self) {
        // Save CPU/GPU when game is minimized
        if !self.input.is_focused {
            return;
        }

        let cam = self.level.get_camera();
        match self.renderer_draw_frame(&cam) {
            graphics::DrawResult::ResizeNeeded => {
                self.renderer.resize_window([self.cfg.resolution_x, self.cfg.resolution_y]);
                self.renderer_draw_frame(&cam);
            }
            _ => ()
        }
    }
}
