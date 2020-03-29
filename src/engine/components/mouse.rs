/// Allows the user to interact with this entity using the mouse.
pub struct MouseComponent {
    pub is_hovered: bool,
}

impl MouseComponent {
    pub fn new() -> MouseComponent {
        MouseComponent {
            is_hovered: false,
        }
    }
}

impl specs::Component for MouseComponent {
    type Storage = specs::VecStorage<Self>;
}
