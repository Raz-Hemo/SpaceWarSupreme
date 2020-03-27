use specs::{WorldExt, Builder};
use crate::engine::systems::MeshInstance;
use crate::engine::components;

pub struct SpaceWarLevel;
impl SpaceWarLevel {
    pub fn new() -> SpaceWarLevel {
        SpaceWarLevel
    }
}
impl super::Level for SpaceWarLevel {
    fn load_level(&mut self, engine: &mut crate::engine::Engine) {
        engine.world.create_entity().with(components::StaticMeshComponent{
            model: engine.renderer.models_manager.get_id("testmodel.obj"),
            mesh_instance: MeshInstance::new()
        }).build();
    }
}

impl SpaceWarLevel {
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
