use specs::{WorldExt, Builder, World};
use crate::engine::components;

pub struct SpaceWarLevel {
    main_menu_space: World,
    game_space: World,
    is_in_menu: bool,
}
impl SpaceWarLevel {
    pub fn new() -> SpaceWarLevel {
        let mut result = SpaceWarLevel {
            main_menu_space: super::create_space(),
            game_space: super::create_space(),
            is_in_menu: true,
        };

        result.create_menu();

        result
    }
}
impl super::Level for SpaceWarLevel {
    fn iter_render(&mut self) -> super::SpaceIterator {
        use std::iter::once;
        if self.is_in_menu {
            Box::new(once(&mut self.main_menu_space).chain(once(&mut self.game_space)))
        } else {
            Box::new(once(&mut self.game_space))
        }
    }
    fn iter_tickable(&mut self) -> super::SpaceIterator {
        use std::iter::once;
        if self.is_in_menu {
            Box::new(once(&mut self.main_menu_space))
        } else {
            Box::new(once(&mut self.game_space))
        }
    }
    fn iter_all(&mut self) -> super::SpaceIterator {
        use std::iter::once;
        Box::new(once(&mut self.main_menu_space).chain(once(&mut self.game_space)))
    }
}
impl SpaceWarLevel {
    fn create_menu(&mut self) {
        self.main_menu_space.create_entity()
        .with(components::StaticMeshComponent::new(
            "mainmenu.obj", 
            nalgebra::Matrix4::new_translation(&nalgebra::Vector3::new(0.0, 0.0, 0.0))))
        .with(components::MouseComponent::new())
        .with(components::TransformComponent::new())
        .with(components::ScriptingComponent::new("test.rhai"))
        .build();

        self.main_menu_space.create_entity()
        .with(components::ScriptingComponent::new("mainmenu.rhai"))
        .with(components::KeyboardComponent::new(vec![String::from("Escape")]))
        .with(components::StaticSkyboxComponent::new("./resources/skybox/skybox.png"))
        .build();
    }

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
}
