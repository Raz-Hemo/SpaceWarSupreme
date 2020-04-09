use rhai::{Engine, RegisterFn};
use crate::engine::camera::Camera;
use nalgebra::{Point3, Vector3};

mod basic_funcs;
pub mod interpolate;

#[derive(Debug, Clone, Copy)]
/// Game events that must affect the engine, and not just the game.
/// For example, adding an entity affects the game, but changing settings affects the engine.
/// Therefore changing settings must be done through a GameEvent.
pub enum GameEvent {
    ChangeResolution(u32, u32),
    ExitGame,
}

/// The entire game-only state that sits on top of the engine, not caring about
/// the engine's implementation.
#[derive(Debug, Clone)]
pub struct GameContext {
    pub events: Vec<GameEvent>,
    pub camera: interpolate::Interpolated<Camera>,
}
impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            events: Vec::new(),
            camera: interpolate::Interpolated::new(
                Camera::new(
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(0.0, 0.0, 1.0),
                    Vector3::new(0.0, 1.0, 0.0),
                )
            ),
        }
    }

    fn vec3_to_point3(v: Vector3<f32>) -> Point3<f32> {
        Point3::new(v.x, v.y, v.z)
    }

    /// Signals the engine to change the resolution
    pub fn change_resolution(&mut self, x: i64, y: i64) {
        self.events.push(GameEvent::ChangeResolution(x as u32, y as u32));
    }

    /// Signals the engine to exit
    pub fn exit_game(&mut self) {
        self.events.push(GameEvent::ExitGame);
    }

    /// Interpolates the camera over a given time
    pub fn camera_smoothstep_lookat(
    &mut self,
    pos: Vector3<f32>,
    lookat: Vector3<f32>,
    up: Vector3<f32>,
    duration: f64,
    ) {
        self.camera.set(
            Camera::new(
                GameContext::vec3_to_point3(pos),
                GameContext::vec3_to_point3(lookat),
                up
            ),
            interpolate::InterpType::Smoothstep,
            duration as f32
        );
    }
}

pub fn new_engine() -> Engine<'static> {
    let mut engine = Engine::new();

    engine.register_fn("error", basic_funcs::error);
    engine.register_fn("warning", basic_funcs::warning);
    engine.register_fn("info", basic_funcs::info);
    engine.register_fn("rand_range", basic_funcs::rand_range as fn(i64, i64) -> i64);
    engine.register_fn("rand_range", basic_funcs::rand_range as fn(f64, f64) -> f64);

    engine.register_type::<GameContext>();
    engine.register_fn("change_resolution", GameContext::change_resolution);
    engine.register_fn("exit_game", GameContext::exit_game);
    engine.register_fn("camera_smoothstep_lookat", GameContext::camera_smoothstep_lookat);

    engine.register_type::<Vector3<f32>>();
    engine.register_fn("vec3", basic_funcs::vec3);

    engine
}

pub fn get_scripts_in_folder<P: AsRef<std::path::Path>>(path: P) -> Vec<std::path::PathBuf> {
    crate::utils::get_files_with_extension_from(
        path,
        vec![crate::consts::SCRIPT_FILE_EXTENSION]
    )
}
