use specs::ReadStorage;
use crate::engine::components::{ScriptingComponent, StaticMeshComponent};


pub struct PreloadSystem {
    pub used_scripts: Vec<String>,
    pub used_meshes: Vec<String>,
}
impl PreloadSystem {
    pub fn new() -> PreloadSystem {
        PreloadSystem {
            used_scripts: Vec::new(),
            used_meshes: Vec::new(),
        }
    }
}

impl<'a> specs::System<'a> for PreloadSystem {
    type SystemData = (
        ReadStorage<'a, ScriptingComponent>,
        ReadStorage<'a, StaticMeshComponent>,
    );

    fn run(&mut self, (scripts, meshes): Self::SystemData) {
        use specs::Join;

        // First call the mouse functions (on_click, etc.)
        for script in scripts.join() {
            self.used_scripts.push(script.path.clone());
        }

        for mesh in meshes.join() {
            self.used_meshes.push(mesh.model.clone());
        }
    }
}