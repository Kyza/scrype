use std::{thread, time::Duration};

use arboard::Clipboard;
use rdev::{simulate, EventType, Key, SimulateError};

pub fn send_key(
	key: Key,
	release: bool,
	delay: Duration,
) -> Result<(), SimulateError> {
	if release {
		simulate(&EventType::KeyRelease(key))?
	} else {
		simulate(&EventType::KeyPress(key))?
	}

	if !delay.is_zero() {
		thread::sleep(delay);
	}

	Ok(())
}

pub fn press_key(key: Key, delay: Duration) -> Result<(), SimulateError> {
	send_key(key, false, delay)
}
pub fn press_keys(
	keys: &Vec<Key>,
	delay: Duration,
) -> Result<(), SimulateError> {
	for key in keys {
		send_key(*key, false, delay)?;
	}
	Ok(())
}

pub fn release_key(key: Key, delay: Duration) -> Result<(), SimulateError> {
	send_key(key, true, delay)
}
pub fn release_keys(
	keys: &Vec<Key>,
	delay: Duration,
) -> Result<(), SimulateError> {
	for key in keys {
		send_key(*key, true, delay)?;
	}
	Ok(())
}

pub fn type_key(key: Key, delay: Duration) -> Result<(), SimulateError> {
	press_key(key, delay)?;
	release_key(key, delay)?;
	Ok(())
}
pub fn type_keys(
	keys: &Vec<Key>,
	delay: Duration,
) -> Result<(), SimulateError> {
	for key in keys {
		press_key(*key, delay)?;
		release_key(*key, delay)?;
	}
	Ok(())
}

pub fn paste_text(text: &str, delay: Duration) -> Result<(), SimulateError> {
	let mut clipboard = Clipboard::new().expect("Failed to get clipboard.");
	let old_text_result = clipboard.get_text();

	clipboard
		.set_text(text)
		.expect("Failed to set clipboard text.");

	press_keys(&vec![Key::ControlLeft, Key::KeyV], delay)?;
	release_keys(&vec![Key::KeyV, Key::ControlLeft], delay)?;

	clipboard.clear().expect("Failed to clear clipboard.");

	if let Ok(old_text) = old_text_result {
		clipboard
			.set_text(old_text)
			.expect("Failed to reset clipboard text.");
	}

	Ok(())
}
