use std::time::Duration;

pub struct MainMenuGameState {

}

impl MainMenuGameState {
    pub fn new() -> MainMenuGameState {
        MainMenuGameState {
            
        }
    }
}

impl super::GameState for MainMenuGameState {
    fn cleanup(&self) {

    }

    fn init(&self) {

    }

    fn render(&self, renderer: &crate::graphics::renderer::Renderer) {

    }

    fn tick(&self, delta: Duration) -> super::GameStateAction {
        super::GameStateAction::Nothing
    }
}