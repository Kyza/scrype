


use arboard::Clipboard;
use cfg_if::cfg_if;
use rdev::{simulate, EventType, Key, SimulateError};
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_CONTROL, VK_V};

cfg_if! {
	if #[cfg(windows)] {
		use windows::Win32::System::Threading::Sleep;
		pub mod win32;
	}
}

pub fn minimal_sleep() {
	cfg_if! {
		if #[cfg(windows)] {
			unsafe { Sleep(1) };
		} else {
			thread:sleep(Duration::NANOSECOND);
		}
	}
}

pub fn send_string(string: &str) -> Result<(), SimulateError> {
	let mut clipboard = Clipboard::new().expect("Failed to get clipboard.");
	let old_clipboard_image = clipboard.get_image();
	let old_clipboard_text = clipboard.get_text();

	clipboard
		.set_text(string)
		.expect("Failed to set clipboard text.");

	cfg_if! {
		if #[cfg(windows)] {
			win32::press_vks(&vec![VK_CONTROL, VK_V]);
			minimal_sleep();
			win32::release_vks(&vec![VK_CONTROL, VK_V]);
			minimal_sleep();
		} else {
			simulate(&EventType::KeyPress(Key::ControlLeft))?;
			simulate(&EventType::KeyPress(Key::KeyV))?;
			simulate(&EventType::KeyPress(Key::KeyV))?;
			simulate(&EventType::KeyPress(Key::ControlLeft))?;
		}
	}

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
		minimal_sleep();
	}
	Ok(())
}
pub fn press_key(key: Key) -> Result<(), SimulateError> {
	simulate(&EventType::KeyPress(key))?;
	minimal_sleep();
	Ok(())
}

pub fn release_keys(keys: &Vec<Key>) -> Result<(), SimulateError> {
	for key in keys {
		simulate(&EventType::KeyRelease(*key))?;
		minimal_sleep();
	}
	Ok(())
}
pub fn release_key(key: Key) -> Result<(), SimulateError> {
	simulate(&EventType::KeyRelease(key))?;
	minimal_sleep();
	Ok(())
}

pub fn type_keys(keys: &Vec<Key>) -> Result<(), SimulateError> {
	for key in keys {
		simulate(&EventType::KeyPress(*key))?;
		minimal_sleep();
		simulate(&EventType::KeyRelease(*key))?;
		minimal_sleep();
	}
	Ok(())
}
pub fn type_key(key: Key) -> Result<(), SimulateError> {
	simulate(&EventType::KeyPress(key))?;
	minimal_sleep();
	simulate(&EventType::KeyRelease(key))?;
	minimal_sleep();
	Ok(())
}
