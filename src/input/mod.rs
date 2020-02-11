use winit::event::{WindowEvent, DeviceEvent, KeyboardInput, ModifiersState, ElementState,
                   VirtualKeyCode};
use std::collections::{HashMap, HashSet};
mod keycode_to_str;

pub struct InputInfo<'a> {
    // Ctrl, Alt and Shift state
    modifiers: ModifiersState,

    // Pixel position of the mouse relative to top left
    mouse_pos: winit::dpi::PhysicalPosition<i32>,

    // Maps keybinds to their handlers
    handlers: HashMap<String, Box<dyn Fn() + 'a>>,

    // Prevent key repeat by keeping track of which keys are already pressed
    pressed_keys: HashSet<VirtualKeyCode>,
}

impl<'a> InputInfo<'a> {
    pub fn new() -> InputInfo<'a> {
        InputInfo {
            modifiers: ModifiersState::empty(),
            handlers: HashMap::new(),
            pressed_keys: HashSet::new(),
            mouse_pos: winit::dpi::PhysicalPosition::new(0, 0),
        }
    }

    pub fn add_handler(self: &mut InputInfo<'a>, keybind: &str, handler: Box<dyn Fn() + 'a>) {
        self.handlers.insert(String::from(keybind), handler);
    }

    pub fn remove_handler(self: &mut InputInfo<'a>, keybind: &str) {
        self.handlers.remove(keybind);
    }
}

pub fn handle_device_event(input_info: &mut InputInfo, e: &DeviceEvent) {
    match e {
        DeviceEvent::ModifiersChanged(new_mod) => input_info.modifiers = new_mod.clone(),
        _ => ()
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

pub fn handle_event(input_info: &mut InputInfo, e: &WindowEvent) {
    match e {
        WindowEvent::KeyboardInput { 
            input: KeyboardInput { 
                state: ElementState::Pressed,
                virtual_keycode: Some(keycode),
                .. 
            }, .. 
        } => {
            let input_str = input_to_string(keycode, &input_info.modifiers);
            
            if !keycode_to_str::NON_STANDALONE_KEYS.contains(&keycode) && // Disallow CTRL, SHIFT and the like
                    !input_info.pressed_keys.contains(&keycode) &&        // No repeats
                    input_info.handlers.contains_key(&input_str) {        // only if a handler exists
                input_info.handlers[&input_str]();
                input_info.pressed_keys.insert(keycode.clone());
            }
        },
        WindowEvent::KeyboardInput { 
            input: KeyboardInput { 
                state: ElementState::Released,
                virtual_keycode: Some(keycode),
                .. 
            }, .. 
        } => {
            input_info.pressed_keys.remove(keycode);
        },
        WindowEvent::CursorMoved {
            position: pos,
            ..
        } => {
            input_info.mouse_pos = pos.clone();
        },
        _ => ()
    }
}