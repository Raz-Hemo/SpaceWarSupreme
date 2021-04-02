use winit::event::VirtualKeyCode;
use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref NON_STANDALONE_KEYS: HashSet<VirtualKeyCode> = vec![
        VirtualKeyCode::LControl,
        VirtualKeyCode::LShift,
        VirtualKeyCode::LAlt,
        VirtualKeyCode::LWin,
        VirtualKeyCode::RControl,
        VirtualKeyCode::RShift,
        VirtualKeyCode::RAlt,
        VirtualKeyCode::RWin,
    ].iter().copied().collect();

    static ref KEYNAMES: HashMap<VirtualKeyCode, &'static str> = [
        (VirtualKeyCode::Grave, "~"),
        (VirtualKeyCode::Backslash, "\\"),
        (VirtualKeyCode::Slash, "/"),
        (VirtualKeyCode::Equals, "="),
        (VirtualKeyCode::Minus, "-"),
        (VirtualKeyCode::Capital, "CapsLock"),
        (VirtualKeyCode::Return, "Enter"),
        (VirtualKeyCode::Back, "Backspace"),
        (VirtualKeyCode::Apostrophe, "'"),
        (VirtualKeyCode::LBracket, "["),
        (VirtualKeyCode::RBracket, "]"),
        (VirtualKeyCode::Semicolon, ";"),
        (VirtualKeyCode::Period, "."),
        (VirtualKeyCode::Comma, ","),
        (VirtualKeyCode::Key0, "0"),
        (VirtualKeyCode::Key1, "1"),
        (VirtualKeyCode::Key2, "2"),
        (VirtualKeyCode::Key3, "3"),
        (VirtualKeyCode::Key4, "4"),
        (VirtualKeyCode::Key5, "5"),
        (VirtualKeyCode::Key6, "6"),
        (VirtualKeyCode::Key7, "7"),
        (VirtualKeyCode::Key8, "8"),
        (VirtualKeyCode::Key9, "9"),
        (VirtualKeyCode::Numpad0, "Numpad0"),
        (VirtualKeyCode::Numpad1, "Numpad1"),
        (VirtualKeyCode::Numpad2, "Numpad2"),
        (VirtualKeyCode::Numpad3, "Numpad3"),
        (VirtualKeyCode::Numpad4, "Numpad4"),
        (VirtualKeyCode::Numpad5, "Numpad5"),
        (VirtualKeyCode::Numpad6, "Numpad6"),
        (VirtualKeyCode::Numpad7, "Numpad7"),
        (VirtualKeyCode::Numpad8, "Numpad8"),
        (VirtualKeyCode::Numpad9, "Numpad9"),
        (VirtualKeyCode::NumpadDecimal, "Numpad ."),
        (VirtualKeyCode::NumpadComma, "Numpad ,"),
        (VirtualKeyCode::NumpadEnter, "Numpad Enter"),
        (VirtualKeyCode::NumpadEquals, "Numpad ="),
        (VirtualKeyCode::NumpadAdd, "Numpad +"),
        (VirtualKeyCode::NumpadSubtract, "Numpad -"),
        (VirtualKeyCode::NumpadMultiply, "Numpad *"),
        (VirtualKeyCode::NumpadDivide, "Numpad /"),
        (VirtualKeyCode::Left, "LeftArrow"),
        (VirtualKeyCode::Right, "RightArrow"),
        (VirtualKeyCode::Up, "UpArrow"),
        (VirtualKeyCode::Down, "DownArrow"),
    ].iter().copied().collect();
}

pub fn keycode_to_str(keycode: &VirtualKeyCode) -> String {
    if let Some(&name) = KEYNAMES.get(keycode) {
        String::from(name)
    } else {
        format!("{:?}", keycode)
    }
}
