use crate::engine::prelude::*;
use crate::engine::camera::Camera;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use rhai::{Engine, RegisterFn};
use nalgebra::{Point3, Vector3};

mod basic_funcs;
pub mod interpolate;

#[derive(Debug, Clone)]
/// Game events that must affect the engine, and not just the game.
/// For example, adding an entity affects the game, but changing settings affects the engine.
/// Therefore changing settings must be done through an EngineEvent.
/// This is equivalent to a system call in a regular application.
pub enum EngineEvent {
    ChangeResolution(u32, u32),
    ExitGame,
    SetActiveSpace(String),
}

/// An event sent between entities (for example, "lclick", "kill_all_zombies", etc)
pub struct GameEvent {
    pub name: String,
    pub args: rhai::Map,
}

/// The entire game-only state that sits on top of the engine, not caring about
/// the engine's implementation.
#[derive(Debug, Clone)]
pub struct GameContext {
    // I/O channels between the game and the engine
    pub engine_event_rx: crossbeam_channel::Receiver<EngineEvent>,
    pub engine_event_tx: crossbeam_channel::Sender<EngineEvent>,

    // I/O channels between entities in the game, or external events (like input)
    pub game_event_rx: crossbeam_channel::Receiver<GameEvent>,
    pub game_event_tx: crossbeam_channel::Sender<GameEvent>,
    pub camera: interpolate::Interpolated<Camera>,
    game_event_handlers: Arc<Mutex<HashMap<String, Vec<specs::Entity>>>>,
}
impl GameContext {
    pub fn new() -> GameContext {
        let (engine_event_tx, engine_event_rx) = crossbeam_channel::unbounded();
        let (game_event_tx, game_event_rx) = crossbeam_channel::unbounded();
        GameContext {
            engine_event_tx,
            engine_event_rx,
            game_event_tx,
            game_event_rx,
            camera: interpolate::Interpolated::new(
                Camera::new(
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(0.0, 0.0, 1.0),
                    Vector3::new(0.0, 1.0, 0.0),
                )
            ),
            game_event_handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn vec3_to_point3(v: Vector3<f32>) -> Point3<f32> {
        Point3::new(v.x, v.y, v.z)
    }

    pub fn get_event_subscribers(&self, name: &str) -> Option<&Vec<specs::Entity>> {
        self.game_event_handlers.get(name)
    }

    /// Signals the engine to change the resolution
    pub fn change_resolution(self: &mut Arc<GameContext>, x: i64, y: i64) {
        self.engine_event_tx.send(EngineEvent::ChangeResolution(x as u32, y as u32)).unwrap();
    }

    /// Signals the engine to exit
    pub fn exit_game(self: &mut Arc<GameContext>) {
        self.engine_event_tx.send(EngineEvent::ExitGame).unwrap();
    }

    /// Tells the engine which space consumes keyboard input.
    pub fn set_active_space(self: &mut Arc<GameContext>, space: String) {
        self.engine_event_tx.send(EngineEvent::SetActiveSpace(space)).unwrap();
    }

    /// Interpolates the camera over a given time
    pub fn camera_smoothstep_lookat(
    self: &mut Arc<GameContext>,
    pos: Vector3<f32>,
    lookat: Vector3<f32>,
    up: Vector3<f32>,
    duration: f64,
    ) {
        // self.camera.set(
        //     Camera::new(
        //         GameContext::vec3_to_point3(pos),
        //         GameContext::vec3_to_point3(lookat),
        //         up
        //     ),
        //     interpolate::InterpType::Smoothstep,
        //     duration as f32
        // );
    }

    /// Tells the game to send `name` events to your entity
    pub fn subscribe_event(self: &mut Arc<GameContext>, id: specs::Entity, name: String) {
        self.game_event_handlers.lock().unwrap().entry(name).or_default().push(id);
    }

    /// Tells the game to stop sending `name` events to your entity
    pub fn unsubscribe_event(self: &mut Arc<GameContext>, id: specs::Entity, name: String) {
        if let Some(subs) = self.game_event_handlers.lock().unwrap().get_mut(&name) {
            subs.retain(|&ent| ent != id);
        }
    }
}

pub fn new_engine() -> Engine<'static> {
    let mut engine = Engine::new();

    engine.register_fn("error", basic_funcs::error);
    engine.register_fn("warning", basic_funcs::warning);
    engine.register_fn("info", basic_funcs::info);
    engine.register_fn("rand_range", basic_funcs::rand_range as fn(i64, i64) -> i64);
    engine.register_fn("rand_range", basic_funcs::rand_range as fn(f64, f64) -> f64);

    engine.register_type::<Arc<GameContext>>();
    engine.register_fn("change_resolution", GameContext::change_resolution);
    engine.register_fn("exit_game", GameContext::exit_game);
    engine.register_fn("camera_smoothstep_lookat", GameContext::camera_smoothstep_lookat);
    engine.register_fn("set_active_space", GameContext::set_active_space);
    engine.register_fn("subscribe_event", GameContext::subscribe_event);
    engine.register_fn("unsubscribe_event", GameContext::unsubscribe_event);

    engine.register_type::<Vector3<f32>>();
    engine.register_fn("vec3", basic_funcs::vec3);

    engine
}

pub fn get_scripts_in_folder<P: AsRef<std::path::Path>>(path: P) -> Vec<std::path::PathBuf> {
    utils::get_files_with_extension_from(
        path,
        vec![consts::SCRIPT_FILE_EXTENSION]
    )
}
