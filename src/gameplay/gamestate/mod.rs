use std::time::{Instant, Duration};

/// This trait represents a point in the game's state machine. For example:
/// main_menu, in_game, new_game_creator all implement GameState.
pub trait GameState {
    fn cleanup(&self);
    fn init(&self);

    fn render(&self, renderer: &crate::graphics::renderer::Renderer);
    fn tick(&self, delta: Duration);
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
    pub fn new() -> GameStateManager<'a> {
        GameStateManager {
            states: Vec::new(),
            last_tick_time: Instant::now(),
            should_exit: false,
        }
    }

    pub fn tick(&mut self) {
        let elapsed_time = self.last_tick_time.elapsed();
        self.last_tick_time = Instant::now();

        self.states.last().expect("Trying to tick empty game state stack").tick(
            elapsed_time
        );

    }

    // TODO what happens if a gamestate pops itself? something bad probs
    pub fn pop_state(&mut self) {
        self.states.pop().expect("Trying to pop empty game state stack").cleanup();
    }

    pub fn push_state(&mut self, state: Box<dyn GameState>) {
        self.states.push(state);
    }
}
