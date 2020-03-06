#[derive(Debug)]
pub struct PositionComponent {
    pos: [f32; 3]
}

impl specs::Component for PositionComponent {
    type Storage = specs::VecStorage<Self>;
}
