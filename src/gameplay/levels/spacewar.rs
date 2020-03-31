use specs::{WorldExt, Builder, World};
use std::sync::Arc;
use crate::engine::systems::MeshInstance;
use crate::engine::graphics::ModelID;
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
        result.camera.pos = cgmath::Point3::new(-5.0, 0.0, 0.0);

        result
    }
}
impl super::Level for SpaceWarLevel {
    fn iter_render(&self) -> super::SpaceIterator {
        use std::iter::once;
        if self.is_in_menu {
            Box::new(once(&self.main_menu_space).chain(once(&self.game_space)))
        } else {
            Box::new(once(&self.game_space))
        }
    }
    fn iter_tickable(&self) -> super::SpaceIterator {
        use std::iter::once;
        if self.is_in_menu {
            Box::new(once(&self.main_menu_space))
        } else {
            Box::new(once(&self.game_space))
        }
    }
    fn get_camera(&self) -> Camera {
        self.camera.clone()
    }
}
impl SpaceWarLevel {
    fn create_menu(&mut self) {
        for i in 0 .. 10 {
            self.main_menu_space.create_entity()
            .with(components::StaticMeshComponent{
                model: ModelID::from("testmodel.obj"),
                rel_transform: cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.0, 1.0 * i as f32, 0.0))
            })
            .with(components::MouseComponent::new())
            .with(components::TransformComponent::new())
            .build();
        }
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
