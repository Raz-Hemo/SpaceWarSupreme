use specs::{WorldExt, Builder, World};
use crate::engine::{components, camera::Camera};

pub struct SpaceWarLevel {
    main_menu_space: World,
    game_space: World,
    is_in_menu: bool,
    camera: Camera,
}
impl SpaceWarLevel {
    pub fn new() -> SpaceWarLevel {
        let mut result = SpaceWarLevel {
            main_menu_space: super::create_space(),
            game_space: super::create_space(),
            is_in_menu: true,
            camera: Camera::new(),
        };

        result.create_menu();
        result.camera.pos = cgmath::Point3::new(0.0, 1.3, -2.6);
        result.camera.look_at = cgmath::Point3::new(0.0, -1.0, 0.5);

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
    fn get_camera(&self) -> Camera {
        self.camera.clone()
    }
}
impl SpaceWarLevel {
    fn create_menu(&mut self) {
        self.main_menu_space.create_entity()
        .with(components::StaticMeshComponent::new(
            "mainmenu.obj", 
            cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.0, -1.0, 0.0))))
        .with(components::MouseComponent::new())
        .with(components::TransformComponent::new())
        .with(components::ScriptingComponent::new("test.rhai"))
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
