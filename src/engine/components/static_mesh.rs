use cgmath::SquareMatrix;
use crate::engine::graphics::ModelID;

pub struct StaticMeshComponent {
    pub model: ModelID,
    pub rel_transform: cgmath::Matrix4<f32>,
}

impl StaticMeshComponent {
    pub fn new(model: ModelID) -> StaticMeshComponent {
        StaticMeshComponent {
            model,
            rel_transform: cgmath::Matrix4::identity(),
        }
    }
}

impl specs::Component for StaticMeshComponent {
    type Storage = specs::VecStorage<Self>;
}
