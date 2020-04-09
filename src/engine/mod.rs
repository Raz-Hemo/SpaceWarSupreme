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
    system_preload: systems::PreloadSystem,
    pub input: input::InputInfo,
    pub cfg: config::Config,
    pub audio: audio::AudioManager,
    pub renderer: graphics::Renderer,
}

impl Engine {
    pub fn new(eventloop: &winit::event_loop::EventLoop<()>, level: Box<dyn Level>) -> Engine {
        let renderer = graphics::Renderer::new(eventloop);
        let mut result = Engine {
            level,
            last_tick: std::time::Instant::now(),
            picked_index: None,
            system_static_mesh: systems::StaticMeshSystem::new(),
            system_scripting: systems::ScriptingSystem::new(),
            system_mouse: systems::MouseSystem::new(),
            system_preload: systems::PreloadSystem::new(),
            input: input::InputInfo::new(),
            cfg: config::Config::load(),
            audio: audio::AudioManager::new(),
            renderer,
        };

        for space in result.level.iter_all() {
            result.system_preload.run_now(space);
            for s in result.system_preload.used_scripts.iter() {
                result.system_scripting.add_script(&s);
            }
            for m in result.system_preload.used_meshes.iter() {
                result.renderer.load_model(&m);
            }
        }

        result
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
            self.system_scripting.run_now(space);

            use crate::scripting::GameEvent;
            for e in self.system_scripting.events.events.iter() {
                match e {
                    GameEvent::ExitGame => return TickResult::Exit,
                    _ => ()
                }
            }
        }

        TickResult::Continue
    }

    pub fn draw_frame(&mut self) {
        // Save CPU/GPU when game is minimized
        if !self.input.is_focused {
            return;
        }

        for space in self.level.iter_render() {
            self.system_static_mesh.run_now(&space);
        }
        let (instances, pickables) = self.system_static_mesh.get_instances_and_flush();
        let result = self.renderer.draw_frame(
            &instances,
            self.system_scripting.get_game_context().camera.get().get_view_matrix(),
            [self.input.mousex as u32,
             self.input.mousey as u32],
        );

        self.picked_index = match self.renderer.latest_pick_result {
            Some(i) => {
                if pickables.len() >= i as usize {
                    Some(*pickables.get((i - 1) as usize).unwrap())
                } else {
                    None
                }
            },
            None => None
        };

        result
    }
}
