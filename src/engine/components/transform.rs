#[derive(Debug)]
pub struct TransformComponent {
    pub transform: nalgebra::Matrix4<f32>,
}

impl TransformComponent {
    pub fn new() -> TransformComponent {
        TransformComponent {
            transform: nalgebra::Matrix4::identity()
        }
    }
    pub fn from(transform: nalgebra::Matrix4<f32>) -> TransformComponent {
        TransformComponent {
            transform
        }
    }
}

impl specs::Component for TransformComponent {
    type Storage = specs::VecStorage<Self>;
}
