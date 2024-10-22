use specs::{RunNow, WorldExt};
pub mod config;
pub mod input;
pub mod audio;
pub mod graphics;
pub mod camera;
pub mod components;
pub mod systems;
pub mod log;
pub mod utils;
pub mod consts;
pub mod localization;
pub mod scripting;
pub mod prelude;
use crate::gameplay::levels::Level;

pub enum TickResult {
    Continue,
    Exit,
}

pub struct Engine {
    level: Box<dyn Level>,
    last_tick: std::time::Instant,
    system_static_mesh: systems::StaticMeshSystem,
    system_scripting: systems::ScriptingSystem,
    system_mouse: systems::MouseSystem,
    system_keyboard: systems::KeyboardSystem,
    system_preload: systems::PreloadSystem,
    system_skybox: systems::StaticSkyboxSystem,
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
            system_static_mesh: systems::StaticMeshSystem::new(),
            system_scripting: systems::ScriptingSystem::new(),
            system_mouse: systems::MouseSystem::new(),
            system_keyboard: systems::KeyboardSystem::new(),
            system_preload: systems::PreloadSystem::new(),
            system_skybox: systems::StaticSkyboxSystem::new(),
            input: input::InputInfo::new(),
            cfg: config::Config::load(),
            audio: audio::AudioManager::new(),
            renderer,
        };
        result.renderer.resize_window([result.cfg.resolution_x, result.cfg.resolution_y]);

        for space in result.level.iter_spaces() {
            result.system_preload.run_now(space);
            for s in result.system_preload.used_scripts.iter() {
                result.system_scripting.add_script(&s);
            }
            for m in result.system_preload.used_meshes.iter() {
                if let Err(e) = result.renderer.load_model(&m) {
                    log::err(&e);
                }
            }
            for t in result.system_preload.used_textures.iter() {
                if let Err(e) = result.renderer.load_texture(&t) {
                    log::err(&e);
                }
            }
            for cm in result.system_preload.used_cubemaps.iter() {
                if let Err(e) = result.renderer.load_cubemap(&cm) {
                    log::err(&e);
                }
            }
        }

        result
    }

    pub fn tick(&mut self) -> TickResult {
        let dt = self.last_tick.elapsed();
        self.last_tick = std::time::Instant::now();
        let asd = self.input.drain_kb_events();
        for k in asd.iter() {
            if k.is_down {
                match &k.key[..] {
                    "X" => self.renderer.light_pos[0] += 0.03,
                    "Y" => self.renderer.light_pos[1] += 0.03,
                    "Z" => self.renderer.light_pos[2] += 0.03,
                    "SHIFT+X" => self.renderer.light_pos[0] -= 0.03,
                    "SHIFT+Y" => self.renderer.light_pos[1] -= 0.03,
                    "SHIFT+Z" => self.renderer.light_pos[2] -= 0.03,
                    _ => (),
                }
            }
        }

        // Keyboard input
        {
            self.system_keyboard.new_frame(asd);
            let space = self.level.get_active_space();

            let mut kbs = space.write_resource::<systems::KeyboardState>();
            kbs.ctrl = self.input.kb_modifiers().ctrl();
            kbs.shift = self.input.kb_modifiers().shift();
            kbs.alt = self.input.kb_modifiers().alt();
            
            self.system_keyboard.run_now(space);
        }
        
        
        for space in self.level.iter_spaces() {
            self.system_scripting.run_now(space);
        }

        for e in self.system_scripting.get_game_context().engine_event_rx.try_iter() {
            use crate::engine::scripting::EngineEvent;
            match e {
                EngineEvent::ExitGame => {
                    if let Err(e) = self.cfg.dump() {
                        log::error(&format!("{:?}", e));
                    }
                    return TickResult::Exit
                },
                EngineEvent::ChangeResolution(x, y) => {
                    self.renderer.resize_window([x, y]);
                    self.cfg.resolution_x = x;
                    self.cfg.resolution_y = y;
                },
                EngineEvent::SetActiveSpace(space) => {
                    self.level.set_active_space(&space);
                }
            }
        }

        TickResult::Continue
    }

    pub fn draw_frame(&mut self) {
        // Save CPU/GPU when game is minimized
        if !self.input.is_focused {
            std::thread::sleep(std::time::Duration::from_millis(10));
            return;
        }

        for space in self.level.iter_spaces() {
            self.system_static_mesh.run_now(&space);
            self.system_skybox.run_now(&space);
        }
        let (instances, pickables) = self.system_static_mesh.get_instances_and_flush();

        let mut framebuilder = graphics::FrameBuilder::new();
        framebuilder.with_meshes(instances);
        framebuilder.with_skybox(self.system_skybox.get_and_flush());

        let result = self.renderer.draw_frame(
            &framebuilder,
            &self.system_scripting.get_game_context().camera.get(),
            [self.input.mousex as u32,
             self.input.mousey as u32],
        );

        let picked_index = match self.renderer.get_pick_result() {
            Some(i) => {
                if pickables.len() >= i as usize {
                    Some(*pickables.get((i - 1) as usize).unwrap())
                } else {
                    None
                }
            },
            None => None
        };
        self.system_mouse.new_frame(picked_index, self.input.drain_mouse_events());
        for space in self.level.iter_spaces() {
            self.system_mouse.run_now(space);
        }

        result
    }
}
