use specs::WriteStorage;
use crate::engine::components::MouseComponent;
use crate::engine::input::{MouseEvent, MouseClickType};

pub struct MouseSystem {
    pub input_events: Vec<MouseEvent>,
    pub current_world: u32,
    pub current_pick: Option<(u32, specs::Entity)>,
    last_pick: Option<(u32, specs::Entity)>,
}

impl MouseSystem {
    pub fn new() -> MouseSystem {
        MouseSystem {
            input_events: Vec::new(),
            current_world: 0,
            current_pick: None,
            last_pick: None,
        }
    }
    
    pub fn new_frame(&mut self, new_pick: Option<(u32, specs::Entity)>, events: Vec<MouseEvent>) {
        self.last_pick = self.current_pick;
        self.current_pick = new_pick;
        self.current_world = 0;
        self.input_events = events;
    }
}

impl<'a> specs::System<'a> for MouseSystem {
    type SystemData = WriteStorage<'a, MouseComponent>;

    fn run(&mut self, mut mouses: Self::SystemData) {
        if let Some(p) = self.current_pick {
            if p.0 == self.current_world {
                if let Some(c) = mouses.get_mut(p.1) {
                    c.is_hovered = true;
                    for e in self.input_events.drain(..) {
                        if e.is_down {
                            if e.key == MouseClickType::Left {
                                c.l_is_held = true;
                            }
                            if e.key == MouseClickType::Right {
                                c.r_is_held = true;
                            }
                            
                        }
                        else {
                            if e.key == MouseClickType::Left && c.l_is_held {
                                c.l_is_clicked = true;
                            }
                            if e.key == MouseClickType::Right && c.r_is_held {
                                c.r_is_clicked = true;
                            }
                        }
                    }
                }
            }
        }

        if let Some(p) = self.last_pick {
            if self.current_pick != self.last_pick {
                if let Some(c) = mouses.get_mut(p.1) {
                    c.is_hovered = false;
                    c.l_is_held = false;
                }
            }
        }

        self.current_world += 1;
    }
}