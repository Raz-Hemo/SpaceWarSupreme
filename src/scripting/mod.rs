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
pub struct GameEventQueue {
    pub events: Vec<GameEvent>,
}
impl GameEventQueue {
    pub fn new() -> GameEventQueue {
        GameEventQueue {
            events: Vec::new(),
        }
    }

    pub fn merge(&mut self, other: &mut GameEventQueue) {
        self.events.append(&mut other.events);
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn change_resolution(&mut self, x: u32, y: u32) {
        self.events.push(GameEvent::ChangeResolution(x, y));
    }

    pub fn exit_game(&mut self) {
        self.events.push(GameEvent::ExitGame);
    }
}

pub fn new_engine() -> Engine<'static> {
    let mut engine = Engine::new();

    engine.register_fn("error", basic_funcs::error);
    engine.register_fn("warning", basic_funcs::warning);
    engine.register_fn("info", basic_funcs::info);
    engine.register_fn("rand_range", basic_funcs::rand_range as fn(i64, i64) -> i64);
    engine.register_fn("rand_range", basic_funcs::rand_range as fn(f64, f64) -> f64);

    engine.register_type::<GameEventQueue>();
    engine.register_fn("change_resolution", GameEventQueue::change_resolution);
    engine.register_fn("exit_game", GameEventQueue::exit_game);

    engine
}

pub fn get_scripts_in_folder<P: AsRef<std::path::Path>>(path: P) -> Vec<std::path::PathBuf> {
    crate::utils::get_files_with_extension_from(
        path,
        vec![crate::consts::SCRIPT_FILE_EXTENSION]
    )
}
