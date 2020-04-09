/// Allows the user to interact with this entity using the mouse.
pub struct MouseComponent {
    pub is_hovered: bool, // mouse is on the entity

    pub l_is_held: bool,    // LMB is clicked down but not released
    pub l_is_clicked: bool, // LMB was released which means a click event occurred

    pub r_is_held: bool,    // RMB is clicked down but not released
    pub r_is_clicked: bool, // RMB was released which means a click event occurred
}

impl MouseComponent {
    pub fn new() -> MouseComponent {
        MouseComponent {
            is_hovered: false,

            l_is_held: false,
            l_is_clicked: false,

            r_is_held: false,
            r_is_clicked: false,
        }
    }
}

impl specs::Component for MouseComponent {
    type Storage = specs::VecStorage<Self>;
}
