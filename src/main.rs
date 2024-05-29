use clap::Parser;
use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::*;
use log::info;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, str::FromStr, thread, time::Duration};

fn string_to_char(s: &str) -> char {
    s.chars().next().expect("String is empty")
}

fn string_to_keycode(s: &str) -> Keycode {
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

fn string_to_key(s: &str) -> Key {
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

#[derive(Debug, Serialize, Deserialize)]
struct KeyPress {
    key: String,
    delay: Option<u64>,
    direction: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    global_delay: Option<u64>,
    mappings: HashMap<String, Vec<KeyPress>>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long)]
    config: String,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    info!("Starting the KeyMastro program");

    // Read the configuration file
    let config_content =
        fs::read_to_string(args.config.clone()).expect("Failed to read config file");
    info!("Configuration file loaded");

    let config: Config = if args.config.ends_with(".json") {
        serde_json::from_str(&config_content).expect("Invalid JSON format")
    } else if args.config.ends_with(".yaml") || args.config.ends_with(".yml") {
        serde_yaml::from_str(&config_content).expect("Invalid YAML format")
    } else {
        panic!("Unsupported config file format. Use JSON or YAML.");
    };
    info!("Configuration parsed successfully");

    let global_delay = config.global_delay.unwrap_or(0);
    info!("Global delay set to {} ms", global_delay);

    // Convert string keys to Keycode and store mappings
    let mut key_mappings = HashMap::new();
    for (from_key, to_keys) in config.mappings {
        let from_keycode = string_to_keycode(&from_key);

        let enigo_keys: Vec<(Key, u64, String)> = to_keys
            .into_iter()
            .map(|kp| {
                (
                    string_to_key(&kp.key),
                    kp.delay.unwrap_or(0),
                    kp.direction.unwrap_or("Click".to_string()),
                )
            })
            .collect();

        key_mappings.insert(from_keycode, enigo_keys);
    }
    info!("Key mappings loaded successfully");

    let device_state = DeviceState::new();
    let mut enigo = match Enigo::new(&Settings::default()) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to create Enigo instance: {}", e);
            return;
        }
    };

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        // if keys.len() > 0 {
        //     println!("{:?}", keys);
        // }

        for (from_key, to_keys) in &key_mappings {
            if keys.contains(from_key) {
                for (key, delay, direction) in to_keys {
                    if *delay > 0 || global_delay > 0 {
                        thread::sleep(Duration::from_millis(delay + global_delay));
                        // Delay before pressing the key
                    }
                    info!("{direction}ing key: {:?} with delay: {}ms", key, delay);
                    let key_direction = if direction == "Press" {
                        Direction::Press
                    } else if direction == "Release" {
                        Direction::Release
                    } else {
                        Direction::Click
                    };
                    enigo
                        .key(*key, key_direction)
                        .expect("Something went wrong with key press");
                    // thread::sleep(Duration::from_millis(50)); // Small delay between key down and key up
                    // enigo.key_up(key);
                }

                // Add a delay to avoid repeated triggering
                // thread::sleep(Duration::from_millis(500));
            }
        }

        // Add a small sleep to avoid high CPU usage
        thread::sleep(Duration::from_millis(50));
    }
}
