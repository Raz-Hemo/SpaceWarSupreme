use crate::engine::graphics::ModelID;
use crate::engine::systems::MeshInstance;

pub struct StaticMeshComponent {
    pub model: ModelID,
    pub mesh_instance: MeshInstance,
}

impl StaticMeshComponent {
    pub fn new(model: ModelID) -> StaticMeshComponent {
        StaticMeshComponent {
            model,
            mesh_instance: MeshInstance::new(),
        }
    }
}

impl specs::Component for StaticMeshComponent {
    type Storage = specs::VecStorage<Self>;
}
