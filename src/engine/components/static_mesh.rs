#[derive(Debug)]
pub struct StaticMeshComponent {
    mesh: String,
}

impl specs::Component for StaticMeshComponent {
    type Storage = specs::VecStorage<Self>;
}
