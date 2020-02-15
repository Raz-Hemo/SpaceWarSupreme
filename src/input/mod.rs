use winit::event::{WindowEvent, DeviceEvent, KeyboardInput, ModifiersState, ElementState,
                   VirtualKeyCode, MouseButton};
use std::collections::{HashMap, HashSet};
mod keycode_to_str;

pub struct InputInfo<'a> {
    // Ctrl, Alt and Shift state
    modifiers: ModifiersState,

    // Pixel position of the mouse relative to top left
    mousex: f64,
    mousey: f64,

    // Mouse click handlers taking is_down, mouse_x, mouse_y
    lclick_handler: Box<dyn Fn(bool, f64, f64) + 'a>,
    rclick_handler: Box<dyn Fn(bool, f64, f64) + 'a>,

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
            mousex: 0.0,
            mousey:0.0,
            lclick_handler: Box::new(|_, _, _| ()),
            rclick_handler: Box::new(|_, _, _| ()),
        }
    }

    pub fn add_handler(self: &mut InputInfo<'a>, keybind: &str, handler: Box<dyn Fn() + 'a>) {
        self.handlers.insert(String::from(keybind), handler);
    }

    pub fn remove_handler(self: &mut InputInfo<'a>, keybind: &str) {
        self.handlers.remove(keybind);
    }
    
    pub fn set_lclick_handler(self: &mut InputInfo<'a>, handler: Box<dyn Fn(bool, f64, f64) + 'a>) {
        self.lclick_handler = handler;
    }

    pub fn clear_lclick_handler(self: &mut InputInfo<'a>) {
        self.lclick_handler = Box::new(|_, _, _| ());
    }

    pub fn set_rclick_handler(self: &mut InputInfo<'a>, handler: Box<dyn Fn(bool, f64, f64) + 'a>) {
        self.rclick_handler = handler;
    }

    pub fn clear_rclick_handler(self: &mut InputInfo<'a>) {
        self.rclick_handler = Box::new(|_, _, _| ());
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
            input_info.mousex = pos.x;
            input_info.mousey = pos.y;
        },
        WindowEvent::MouseInput {
            button,
            state,
            ..
        } => {
            let h = if *button == MouseButton::Left 
                        {&input_info.lclick_handler} 
                    else 
                        {&input_info.rclick_handler};
            h(*state == ElementState::Pressed, input_info.mousex, input_info.mousey);
        }
        _ => ()
    }
}