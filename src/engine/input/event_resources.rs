#[derive(PartialEq)]
pub enum MouseClickType {
    Left,
    Right,
    Middle,
    Other
}

pub struct KeyboardEvent {
    pub key: String,
    pub is_down: bool,
}

pub struct MouseEvent {
    pub key: MouseClickType,
    pub is_down: bool,
}

impl MouseEvent {
    pub fn from(b: winit::event::MouseButton, is_down: bool) -> MouseEvent {
        MouseEvent {
            key: match b {
                winit::event::MouseButton::Left => MouseClickType::Left,
                winit::event::MouseButton::Right => MouseClickType::Right,
                winit::event::MouseButton::Middle => MouseClickType::Middle,
                _ => MouseClickType::Other,
            },
            is_down
        }
    }
}
