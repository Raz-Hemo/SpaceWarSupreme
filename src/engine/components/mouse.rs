/// Allows the user to interact with this entity using the mouse.
pub struct MouseComponent {
    pub is_hovered: bool, // mouse is on the entity
    pub is_held: bool,    // mouse is clicked down but not released
    pub is_clicked: bool, // mouse was released which means a click event occurred
}

impl MouseComponent {
    pub fn new() -> MouseComponent {
        MouseComponent {
            is_hovered: false,
            is_held: false,
            is_clicked: false,
        }
    }
}

impl specs::Component for MouseComponent {
    type Storage = specs::VecStorage<Self>;
}
