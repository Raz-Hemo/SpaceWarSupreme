use std::sync::Arc;
use crate::engine::graphics::Model;

pub struct StaticMeshComponent {
    model: Arc<Model>,
}

impl StaticMeshComponent {
    pub fn new(model: Arc<Model>) -> StaticMeshComponent {
        StaticMeshComponent {
            model
        }
    }
}

impl specs::Component for StaticMeshComponent {
    type Storage = specs::VecStorage<Self>;
}
