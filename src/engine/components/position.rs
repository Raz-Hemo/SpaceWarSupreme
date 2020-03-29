#[derive(Debug)]
pub struct PositionComponent {
    pub pos: [f32; 3]
}

impl PositionComponent {
    pub fn new() -> PositionComponent {
        PositionComponent {
            pos: [0.0; 3]
        }
    }
}

impl specs::Component for PositionComponent {
    type Storage = specs::VecStorage<Self>;
}
