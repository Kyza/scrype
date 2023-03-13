use core::time;
use std::thread;

use rdev::{simulate, EventType, Key};

pub fn send_key(key: Key, release: bool) {
    let delay = time::Duration::from_millis(1);

    if release {
        match simulate(&EventType::KeyRelease(key)) {
            Ok(()) => (),
            Err(simulate_error) => {
                println!("Failed to send key {:?}", key);
                println!("{:?}", simulate_error);
            }
        }
    } else {
        match simulate(&EventType::KeyPress(key)) {
            Ok(()) => (),
            Err(simulate_error) => {
                println!("Failed to send key {:?}", key);
                println!("{:?}", simulate_error);
            }
        }
    }

    thread::sleep(delay);
}

pub fn press_key(key: Key) {
    send_key(key, false);
}
pub fn press_keys(keys: Vec<Key>) {
    for key in keys {
        send_key(key, false);
    }
}

pub fn release_key(key: Key) {
    send_key(key, true);
}
pub fn release_keys(keys: Vec<Key>) {
    for key in keys {
        send_key(key, true);
    }
}

pub fn type_key(key: Key) {
    press_key(key);
    release_key(key);
}
pub fn type_keys(keys: Vec<Key>) {
    for key in keys {
        press_key(key);
        release_key(key);
    }
}
