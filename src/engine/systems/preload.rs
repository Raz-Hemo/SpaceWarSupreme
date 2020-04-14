use std::collections::HashSet;
use specs::ReadStorage;
use crate::engine::components::{
    ScriptingComponent,
    StaticMeshComponent,
    StaticSkyboxComponent
};


pub struct PreloadSystem {
    pub used_scripts: HashSet<String>,
    pub used_meshes: HashSet<String>,
    pub used_textures: HashSet<String>,
    pub used_cubemaps: HashSet<String>,
}
impl PreloadSystem {
    pub fn new() -> PreloadSystem {
        PreloadSystem {
            used_scripts: HashSet::new(),
            used_meshes: HashSet::new(),
            used_textures: HashSet::new(),
            used_cubemaps: HashSet::new(),
        }
    }
}

impl<'a> specs::System<'a> for PreloadSystem {
    type SystemData = (
        ReadStorage<'a, ScriptingComponent>,
        ReadStorage<'a, StaticMeshComponent>,
        ReadStorage<'a, StaticSkyboxComponent>,
    );

    fn run(&mut self, (scripts, meshes, skyboxes): Self::SystemData) {
        use specs::Join;
        self.used_scripts.clear();
        self.used_meshes.clear();
        self.used_textures.clear();
        self.used_cubemaps.clear();
        
        for script in scripts.join() {
            self.used_scripts.insert(script.path.clone());
        }

        for mesh in meshes.join() {
            self.used_meshes.insert(mesh.model.clone());
        }

        for skybox in skyboxes.join() {
            self.used_cubemaps.insert(skybox.skybox.clone());
        }
    }
}