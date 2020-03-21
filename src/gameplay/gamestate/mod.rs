use std::time::{Instant, Duration};
pub mod main_menu;

/// Returned from a game state's tick(). Determines what the stack should do next
pub enum GameStateAction {
    Push(Box<dyn GameState>),
    Pop,
    Nothing
}

/// This trait represents a point in the game's state machine. For example:
/// main_menu, in_game, new_game_creator all implement GameState.
pub trait GameState {
    fn cleanup(&mut self, engine: &mut crate::engine::Engine);
    fn init(&mut self, engine: &mut crate::engine::Engine);

    fn tick(&mut self, engine: &mut crate::engine::Engine, delta: Duration) -> GameStateAction;
}

/// Object that manages game states, wrapping the tick and render logic while
/// allowing the states to manipulate the state stack safely.
pub struct GameStateManager<'a> {
    /// A stack of states - for example [main_menu, in_game, options_menu]
    states: Vec<Box<dyn GameState + 'a>>,

    /// Internal clock for ticking the game
    last_tick_time: Instant,

    /// Set this to true to terminate the game on the end of the tick
    pub should_exit: bool,
}

impl<'a> GameStateManager<'a> {
    pub fn new(engine: &mut crate::engine::Engine, mut initial_state: Box<dyn GameState>) -> GameStateManager<'a> {
        initial_state.init(engine);
        GameStateManager {
            states: vec![initial_state],
            last_tick_time: Instant::now(),
            should_exit: false,
        }
    }

    pub fn tick(&mut self, engine: &mut crate::engine::Engine) {
        let elapsed_time = self.last_tick_time.elapsed();
        self.last_tick_time = Instant::now();

        match self.states.last_mut()
                .expect("Trying to tick empty game state stack")
                .tick(engine, elapsed_time) {
            GameStateAction::Pop => {self.states.pop().unwrap().cleanup(engine);},
            GameStateAction::Push(mut new_state) => {
                new_state.init(engine);
                self.states.push(new_state);
            },
            GameStateAction::Nothing => (),
        };
    }
}
