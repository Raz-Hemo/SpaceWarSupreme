use specs::WriteStorage;
use crate::engine::components::KeyboardComponent;
use crate::engine::input::KeyboardEvent;

pub struct KeyboardSystem {
    pub input_events: Vec<KeyboardEvent>,
}

/// State of the keyboard modifier keys
pub struct KeyboardState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

impl KeyboardSystem {
    pub fn new() -> KeyboardSystem {
        KeyboardSystem {
            input_events: Vec::new(),
        }
    }
    
    pub fn new_frame(&mut self, events: Vec<KeyboardEvent>) {
        self.input_events = events;
    }
}

impl<'a> specs::System<'a> for KeyboardSystem {
    type SystemData = WriteStorage<'a, KeyboardComponent>;

    fn run(&mut self, mut keybs: Self::SystemData) {
        use specs::Join;
        
        for keyb in (&mut keybs).join() {
            for e in self.input_events.iter() {
                if keyb.subscribed.contains(&e.key) {
                    keyb.events.push((e.key.clone(), e.is_down));
                }
            }
        }
    }
}