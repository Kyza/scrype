// TODO: Use own clipboard implementation.
use arboard::Clipboard;

use windows::Win32::{
	System::Threading::Sleep,
	UI::Input::KeyboardAndMouse::{
		SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT,
		KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, VIRTUAL_KEY, VK_CONTROL, VK_V,
	},
};

pub fn send_string(string: &str) -> (u32, u32) {
	let mut clipboard = Clipboard::new().expect("Failed to get clipboard.");
	let old_clipboard_image = clipboard.get_image();
	let old_clipboard_text = clipboard.get_text();

	clipboard
		.set_text(string)
		.expect("Failed to set clipboard text.");

	let a = press_vks(&vec![VK_CONTROL, VK_V]);
	unsafe { Sleep(1) };
	let b = release_vks(&vec![VK_V, VK_CONTROL]);
	unsafe { Sleep(1) };

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

	(a, b)
}

pub fn press_vks(keys: &Vec<VIRTUAL_KEY>) -> u32 {
	let mut inputs = Vec::with_capacity(keys.len() * 2);

	for key in keys {
		let input = INPUT {
			r#type: INPUT_KEYBOARD,
			Anonymous: INPUT_0 {
				ki: KEYBDINPUT {
					wVk: *key,
					wScan: 0,
					dwFlags: KEYEVENTF_UNICODE,
					time: 0,
					dwExtraInfo: 0,
				},
			},
		};
		inputs.push(input);
	}

	unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) }
}
pub fn press_vk(key: VIRTUAL_KEY) -> u32 {
	let mut inputs = Vec::with_capacity(1);

	let input = INPUT {
		r#type: INPUT_KEYBOARD,
		Anonymous: INPUT_0 {
			ki: KEYBDINPUT {
				wVk: key,
				wScan: 0,
				dwFlags: KEYEVENTF_UNICODE,
				time: 0,
				dwExtraInfo: 0,
			},
		},
	};
	inputs.push(input);

	unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) }
}

pub fn release_vks(keys: &Vec<VIRTUAL_KEY>) -> u32 {
	let mut inputs = Vec::with_capacity(keys.len());

	for key in keys {
		let input = INPUT {
			r#type: INPUT_KEYBOARD,
			Anonymous: INPUT_0 {
				ki: KEYBDINPUT {
					wVk: *key,
					wScan: 0,
					dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
					time: 0,
					dwExtraInfo: 0,
				},
			},
		};
		inputs.push(input);
	}

	unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) }
}
pub fn release_vk(key: VIRTUAL_KEY) -> u32 {
	let mut inputs = Vec::with_capacity(1);

	let input = INPUT {
		r#type: INPUT_KEYBOARD,
		Anonymous: INPUT_0 {
			ki: KEYBDINPUT {
				wVk: key,
				wScan: 0,
				dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
				time: 0,
				dwExtraInfo: 0,
			},
		},
	};
	inputs.push(input);

	unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) }
}

pub fn type_vks(keys: &Vec<VIRTUAL_KEY>) -> u32 {
	let mut inputs = Vec::with_capacity(keys.len());

	for key in keys {
		let input = INPUT {
			r#type: INPUT_KEYBOARD,
			Anonymous: INPUT_0 {
				ki: KEYBDINPUT {
					wVk: *key,
					wScan: 0,
					dwFlags: KEYEVENTF_UNICODE,
					time: 0,
					dwExtraInfo: 0,
				},
			},
		};
		inputs.push(input);

		let input = INPUT {
			r#type: INPUT_KEYBOARD,
			Anonymous: INPUT_0 {
				ki: KEYBDINPUT {
					wVk: *key,
					wScan: 0,
					dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
					time: 0,
					dwExtraInfo: 0,
				},
			},
		};
		inputs.push(input);
	}

	unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) }
}
pub fn type_vk(key: VIRTUAL_KEY) -> u32 {
	let mut inputs = Vec::with_capacity(2);

	let input = INPUT {
		r#type: INPUT_KEYBOARD,
		Anonymous: INPUT_0 {
			ki: KEYBDINPUT {
				wVk: key,
				wScan: 0,
				dwFlags: KEYEVENTF_UNICODE,
				time: 0,
				dwExtraInfo: 0,
			},
		},
	};
	inputs.push(input);

	let input = INPUT {
		r#type: INPUT_KEYBOARD,
		Anonymous: INPUT_0 {
			ki: KEYBDINPUT {
				wVk: key,
				wScan: 0,
				dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
				time: 0,
				dwExtraInfo: 0,
			},
		},
	};
	inputs.push(input);

	unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) }
}
