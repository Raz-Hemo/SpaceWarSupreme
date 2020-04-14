use winit::event::{WindowEvent, DeviceEvent, KeyboardInput, ModifiersState, ElementState,
                   VirtualKeyCode};
use std::collections::{HashSet};
mod keycode_to_str;
mod event_resources;
pub use event_resources::{KeyboardEvent, MouseEvent, MouseClickType};

pub struct InputInfo {
    // Ctrl, Alt and Shift state
    modifiers: ModifiersState,

    // Prevent key repeat by keeping track of which keys are already pressed
    pressed_keys: HashSet<VirtualKeyCode>,

    // Pixel position of the mouse relative to top left
    pub mousex: f64,
    pub mousey: f64,

    // Tracks window focus
    pub is_focused: bool,

    keyboard_events: Vec<KeyboardEvent>,
    mouse_events: Vec<MouseEvent>,
}

impl InputInfo {
    pub fn new() -> InputInfo {
        InputInfo {
            modifiers: ModifiersState::empty(),
            pressed_keys: HashSet::new(),
            mousex: 0.0,
            mousey: 0.0,
            is_focused: true,
            keyboard_events: Vec::new(),
            mouse_events: Vec::new(),
        }
    }

    pub fn drain_mouse_events(&mut self) -> Vec<MouseEvent> {
        std::mem::replace(&mut self.mouse_events, Vec::new())
    }

    pub fn drain_kb_events(&mut self) -> Vec<KeyboardEvent> {
        std::mem::replace(&mut self.keyboard_events, Vec::new())
    }

    pub fn kb_modifiers(&self) -> &ModifiersState {
        &self.modifiers
    }

    pub fn handle_device_event(&mut self, _e: &DeviceEvent) {
    }

    pub fn handle_window_event(&mut self, e: &WindowEvent, resolution_x: u32, resolution_y: u32) {
        match e {
            WindowEvent::ModifiersChanged(new_mod) => self.modifiers = new_mod.clone(),
            WindowEvent::KeyboardInput { 
                input: KeyboardInput { 
                    state,
                    virtual_keycode: Some(keycode),
                    .. 
                }, .. 
            } => {
                let input_str = input_to_string(keycode, &self.modifiers);

                // Disallow CTRL, SHIFT and the like
                if keycode_to_str::NON_STANDALONE_KEYS.contains(&keycode) { 
                    return ();
                }

                // Skip repeats
                if self.pressed_keys.contains(&keycode) && *state == ElementState::Pressed {
                    return ();
                }

                // Maintain pressed list
                if *state == ElementState::Pressed {
                    self.pressed_keys.insert(keycode.clone());
                } else {
                    self.pressed_keys.remove(keycode);
                }

                self.keyboard_events.push(KeyboardEvent {key: input_str, is_down: *state == ElementState::Pressed});

            },
            WindowEvent::CursorMoved {
                position: pos,
                ..
            } => {
                if pos.x < 0.0 {
                    self.mousex = 0.0;
                } else if pos.x > resolution_x as f64 {
                    self.mousex = (resolution_x - 1) as f64;
                } else {
                    self.mousex = pos.x;
                }

                if pos.y < 0.0 {
                    self.mousey = 0.0;
                } else if pos.y > resolution_y as f64 {
                    self.mousey = (resolution_y - 1) as f64;
                } else {
                    self.mousey = pos.y;
                }
            },
            WindowEvent::MouseInput {
                button,
                state,
                ..
            } => {
                self.mouse_events.push(MouseEvent::from(*button, *state == ElementState::Pressed));
            },
            WindowEvent::Focused(is_focused) => {
                self.is_focused = *is_focused;
            },
            _ => ()
        }
    }
}


fn input_to_string(keycode: &VirtualKeyCode, modifiers: &ModifiersState) -> String {
    let mut parts: Vec<&str> = Vec::with_capacity(4);

    if modifiers.ctrl() {
        parts.push("CTRL");
    }
    if modifiers.shift() {
        parts.push("SHIFT");
    }
    if modifiers.alt() {
        parts.push("ALT");
    }

    let keycode_str: String = keycode_to_str::keycode_to_str(keycode);
    parts.push(&keycode_str);

    parts.join("+")
}
