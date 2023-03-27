use core::time;
use std::thread;

use arboard::Clipboard;
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

pub fn paste_text(text: &str) {
	let mut clipboard = Clipboard::new().expect("Couldn't get clipboard.");
	let old_text_result = clipboard.get_text();

	clipboard
		.set_text(text)
		.expect("Couldn't set clipboard text.");

	press_key(Key::ControlLeft);
	press_key(Key::KeyV);
	release_key(Key::ControlLeft);
	release_key(Key::KeyV);

	if let Ok(old_text) = old_text_result {
		clipboard
			.set_text(old_text)
			.expect("Couldn't reset clipboard text.");
	}
}
