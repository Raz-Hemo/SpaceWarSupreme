use specs::ReadStorage;
use crate::engine::components::{
    ScriptingComponent,
    StaticMeshComponent,
    StaticSkyboxComponent
};


pub struct PreloadSystem {
    pub used_scripts: Vec<String>,
    pub used_meshes: Vec<String>,
    pub used_textures: Vec<String>,
    pub used_cubemaps: Vec<String>,
}
impl PreloadSystem {
    pub fn new() -> PreloadSystem {
        PreloadSystem {
            used_scripts: Vec::new(),
            used_meshes: Vec::new(),
            used_textures: Vec::new(),
            used_cubemaps: Vec::new(),
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

        for script in scripts.join() {
            self.used_scripts.push(script.path.clone());
        }

        for mesh in meshes.join() {
            self.used_meshes.push(mesh.model.clone());
        }

        for skybox in skyboxes.join() {
            self.used_cubemaps.push(skybox.skybox.clone());
        }
    }
}