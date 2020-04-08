/// Allows a script to receive keyboard events.
pub struct KeyboardComponent {
    pub subscribed: Vec<String>,
    pub events: Vec<(String, bool)>,
}

impl KeyboardComponent {
    pub fn new(subscribed: Vec<String>) -> KeyboardComponent {
        KeyboardComponent {
            subscribed,
            events: Vec::new(),
        }
    }
}

impl specs::Component for KeyboardComponent {
    type Storage = specs::VecStorage<Self>;
}
