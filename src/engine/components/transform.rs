use cgmath::SquareMatrix;

#[derive(Debug)]
pub struct TransformComponent {
    pub transform: cgmath::Matrix4<f32>,
}

impl TransformComponent {
    pub fn new() -> TransformComponent {
        TransformComponent {
            transform: cgmath::Matrix4::identity()
        }
    }
}

impl specs::Component for TransformComponent {
    type Storage = specs::VecStorage<Self>;
}
