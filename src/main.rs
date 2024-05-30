use clap::Parser;
use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::*;
use log::info;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, thread, time::Duration};
use utils::{string_to_key, string_to_keycode};

mod utils;

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
        // let from_keycode = string_to_keycode(&from_key);

        let enigo_keys: Vec<(String, u64, String)> = to_keys
            .into_iter()
            .map(|kp| {
                (
                    kp.key,
                    kp.delay.unwrap_or(0),
                    kp.direction.unwrap_or("Click".to_string()),
                )
            })
            .collect();

        key_mappings.insert(from_key, enigo_keys);
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

        let mouse = device_state.get_mouse();
        // println!("{:?}", mouse.button_pressed);
        let mouse_key = if mouse.button_pressed[1] {
            "LeftMouse"
        } else if mouse.button_pressed[2] {
            "RightMouse"
        } else {
            ""
        };

        for (from_key, to_keys) in &key_mappings {
            if (from_key.contains("Mouse") && from_key == mouse_key)
                || (!from_key.contains("Mouse")
                    && keys.contains(&string_to_keycode(from_key.as_str())))
            {
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

                    if key.contains("Mouse") {
                        let button = if *key == "LeftMouse" {
                            Some(Button::Left)
                        } else if *key == "RightMouse" {
                            Some(Button::Right)
                        } else {
                            None
                        };
                        if let Some(b) = button {
                            enigo
                                .button(b, key_direction)
                                .expect("Something wrong with clicking mouse!");
                        }
                    } else {
                        enigo
                            .key(string_to_key(&key), key_direction)
                            .expect("Something went wrong with key press");
                    }

                    // thread::sleep(Duration::from_millis(50)); // Small delay between key down and key up
                    // enigo.key_up(key);
                }

                // Add a delay to avoid repeated triggering
                // thread::sleep(Duration::from_millis(500));
            }
        }

        // Add a small sleep to avoid high CPU usage
        // thread::sleep(Duration::from_millis(50));
    }
}
