use specs::{WorldExt, Builder, World};
use crate::engine::components;

enum ActiveSpace {
    MainMenu,
    GalaxyMap,
}

pub struct SpaceWarLevel {
    main_menu_space: World,
    galaxy_map_space: World,
    active_space: ActiveSpace,
}
impl SpaceWarLevel {
    pub fn new() -> SpaceWarLevel {
        let mut result = SpaceWarLevel {
            main_menu_space: super::create_space(),
            galaxy_map_space: super::create_space(),
            active_space: ActiveSpace::MainMenu,
        };

        result.create_menu();

        result
    }

    fn load_game(&mut self, name: &str) {
        self.galaxy_map_space = super::create_space();

        // for star in crate::gameplay::mapgen::execute_map_generator(mapgen_path: &str, stargen_path: &str, namegen_path: &str){

        // }
    }
}
impl super::Level for SpaceWarLevel {
    fn iter_spaces(&mut self) -> super::SpaceIterator {
        use std::iter::once;
        Box::new(once(&mut self.main_menu_space).chain(once(&mut self.galaxy_map_space)))
    }

    fn get_active_space(&mut self) -> &mut specs::World {
        match self.active_space {
            ActiveSpace::MainMenu => &mut self.main_menu_space,
            ActiveSpace::GalaxyMap => &mut self.galaxy_map_space,
        }
    }

    fn set_active_space(&mut self, space: &str) {
        if space == "mainmenu" {
            self.active_space = ActiveSpace::MainMenu;
        } else if space == "galaxymap" {
            self.active_space = ActiveSpace::GalaxyMap;
        }
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

        self.galaxy_map_space.create_entity()
        .with(components::ScriptingComponent::new("galaxymap.rhai"))
        .with(components::KeyboardComponent::new(vec![String::from("Escape")]))
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
