pub struct StaticMeshComponent {
    pub model: String,
    pub rel_transform: nalgebra::Matrix4<f32>,
}

impl StaticMeshComponent {
    pub fn new(model: &str, rel_transform: nalgebra::Matrix4<f32>) -> StaticMeshComponent {
        StaticMeshComponent {
            model: String::from(model),
            rel_transform,
        }
    }
}

impl specs::Component for StaticMeshComponent {
    type Storage = specs::VecStorage<Self>;
}
