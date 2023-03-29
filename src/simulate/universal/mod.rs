use std::thread;
use std::time::Duration;

use arboard::Clipboard;
use rdev::{simulate, EventType, Key, SimulateError};

pub fn send_string(string: &str) -> Result<(), SimulateError> {
	let mut clipboard = Clipboard::new().expect("Failed to get clipboard.");
	let old_clipboard_image = clipboard.get_image();
	let old_clipboard_text = clipboard.get_text();

	clipboard
		.set_text(string)
		.expect("Failed to set clipboard text.");

	simulate(&EventType::KeyPress(Key::ControlLeft))?;
	simulate(&EventType::KeyPress(Key::KeyV))?;
	simulate(&EventType::KeyPress(Key::KeyV))?;
	simulate(&EventType::KeyPress(Key::ControlLeft))?;

	// This should remove the pasted text from the clipboard history as well as clearing the clipboard.
	clipboard.clear().expect("Failed to clear clipboard.");

	if let Ok(old_text) = old_clipboard_text {
		clipboard
			.set_text(old_text)
			.expect("Failed to reset clipboard text.");
	} else if let Ok(old_image) = old_clipboard_image {
		clipboard
			.set_image(old_image)
			.expect("Failed to reset clipboard image.");
	}

	Ok(())
}

pub fn press_keys(keys: &Vec<Key>) -> Result<(), SimulateError> {
	for key in keys {
		simulate(&EventType::KeyPress(*key))?;
		thread::sleep(Duration::NANOSECOND);
	}
	Ok(())
}
pub fn press_key(key: Key) -> Result<(), SimulateError> {
	simulate(&EventType::KeyPress(key))?;
	thread::sleep(Duration::NANOSECOND);
	Ok(())
}

pub fn release_keys(keys: &Vec<Key>) -> Result<(), SimulateError> {
	for key in keys {
		simulate(&EventType::KeyRelease(*key))?;
		thread::sleep(Duration::NANOSECOND);
	}
	Ok(())
}
pub fn release_key(key: Key) -> Result<(), SimulateError> {
	simulate(&EventType::KeyRelease(key))?;
	thread::sleep(Duration::NANOSECOND);
	Ok(())
}

pub fn type_keys(keys: &Vec<Key>) -> Result<(), SimulateError> {
	for key in keys {
		simulate(&EventType::KeyPress(*key))?;
		thread::sleep(Duration::NANOSECOND);
		simulate(&EventType::KeyRelease(*key))?;
		thread::sleep(Duration::NANOSECOND);
	}
	Ok(())
}
pub fn type_key(key: Key) -> Result<(), SimulateError> {
	simulate(&EventType::KeyPress(key))?;
	thread::sleep(Duration::NANOSECOND);
	simulate(&EventType::KeyRelease(key))?;
	thread::sleep(Duration::NANOSECOND);
	Ok(())
}
