use crate::engine::systems::MeshInstance;

pub struct FrameBuilder {
    pub meshes: std::collections::HashMap<String, Vec<MeshInstance>>,
    pub skybox: Option<String>,
}

/// Fully describes a single frame to be rendered
impl FrameBuilder {
    pub fn new() -> Self {
        Self {
            meshes: std::collections::HashMap::new(),
            skybox: None,
        }
    }

    pub fn with_meshes(&mut self, meshes: std::collections::HashMap<String, Vec<MeshInstance>>)
    -> &mut Self {
        self.meshes = meshes;
        self
    }

    pub fn with_skybox(mut self, skybox: String) -> Self {
       self.skybox = Some(skybox);
       self
    }
}