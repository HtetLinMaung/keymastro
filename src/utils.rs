use std::str::FromStr;

use device_query::Keycode;
use enigo::Key;

pub fn string_to_char(s: &str) -> char {
    s.chars().next().expect("String is empty")
}

pub fn string_to_keycode(s: &str) -> Keycode {
    match s {
        "0" => Keycode::Key0,
        "1" => Keycode::Key1,
        "2" => Keycode::Key2,
        "3" => Keycode::Key3,
        "4" => Keycode::Key4,
        "5" => Keycode::Key5,
        "6" => Keycode::Key6,
        "7" => Keycode::Key7,
        "8" => Keycode::Key8,
        "9" => Keycode::Key9,
        "UpArrow" => Keycode::Up,
        "DownArrow" => Keycode::Down,
        "LeftArrow" => Keycode::Left,
        "RightArrow" => Keycode::Right,
        _ => match Keycode::from_str(s) {
            Ok(kc) => kc,
            Err(_) => panic!("Unsupported key: {}", s),
        },
    }
}

pub fn string_to_key(s: &str) -> Key {
    match s {
        "Alt" => Key::Alt,
        "Backspace" => Key::Backspace,
        "CapsLock" => Key::CapsLock,
        "Control" => Key::Control,
        "LControl" => Key::LControl,
        "Delete" => Key::Delete,
        "DownArrow" => Key::DownArrow,
        "End" => Key::End,
        "Escape" => Key::Escape,
        "F1" => Key::F1,
        "F2" => Key::F2,
        "F3" => Key::F3,
        "F4" => Key::F4,
        "F5" => Key::F5,
        "F6" => Key::F6,
        "F7" => Key::F7,
        "F8" => Key::F8,
        "F9" => Key::F9,
        "F10" => Key::F10,
        "F11" => Key::F11,
        "F12" => Key::F12,
        "F13" => Key::F13,
        "F14" => Key::F14,
        "F15" => Key::F15,
        "F16" => Key::F16,
        "F17" => Key::F17,
        "F18" => Key::F18,
        "F19" => Key::F19,
        "F20" => Key::F20,
        "Help" => Key::Help,
        "Home" => Key::Home,
        "LeftArrow" => Key::LeftArrow,
        "MediaNextTrack" => Key::MediaNextTrack,
        "MediaPlayPause" => Key::MediaPlayPause,
        "MediaPrevTrack" => Key::MediaPrevTrack,
        "PageDown" => Key::PageDown,
        "PageUp" => Key::PageUp,
        "RControl" => Key::RControl,
        "Return" | "Enter" => Key::Return,
        "RightArrow" => Key::RightArrow,
        "RShift" => Key::RShift,
        "Shift" => Key::Shift,
        "LShift" => Key::LShift,
        "Space" => Key::Space,
        "Tab" => Key::Tab,
        "UpArrow" | "Up" => Key::UpArrow,
        "VolumeDown" => Key::VolumeDown,
        "VolumeUp" => Key::VolumeUp,
        "VolumeMute" => Key::VolumeMute,
        "Command" | "Super" | "Windows" => Key::Meta,
        _ => Key::Unicode(string_to_char(s)),
    }
}
