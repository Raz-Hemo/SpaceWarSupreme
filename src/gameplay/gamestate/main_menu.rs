use std::time::Duration;

pub struct Button {

}

pub struct MainMenuGameState {
    //ui: crate::ui::UIElementCore<crate::ui::Checkbox>,
    should_exit: bool,
    //credits_window: ui::Window;
}

impl MainMenuGameState {
    fn get_credits() -> String {
        if let Ok(credits) = crate::utils::read_file("./resources/credits.txt") {
            credits +
            "\n# Rust packages\n" +
            &crate::utils::get_game_dependencies()
                .iter()
                .map(|s| format!("{}\n", s))
                .collect::<String>()
        } else {
            String::from("Credits file not found")
        }
    }

    pub fn new() -> MainMenuGameState {
        MainMenuGameState {
            //buttons: Vec::new(),
            should_exit: false,
            //credits_window: ui::Window::new([ui::TextBox::new(include_string!())]),
        }
    }
}

impl super::GameState for MainMenuGameState {
    fn cleanup(&mut self, engine: &mut crate::engine::Engine) {

    }

    fn init(&mut self, engine: &mut crate::engine::Engine) {
        //let mut b = Button::new(); b.set_pos();
    }

    fn render(&self, renderer: &crate::graphics::renderer::Renderer) {

    }

    fn tick(&mut self, engine: &mut crate::engine::Engine, delta: Duration) -> super::GameStateAction {
        if self.should_exit {
            super::GameStateAction::Pop
        } else {
            super::GameStateAction::Nothing
        }
    }
}