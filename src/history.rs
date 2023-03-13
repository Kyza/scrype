use pomsky_macro::pomsky;
use rdev::{Event, EventType::KeyPress};

use lazy_static::lazy_static;

use rdev::Key;
use std::sync::{
	atomic::{AtomicUsize, Ordering},
	Mutex,
};

lazy_static! {
	#[derive(Debug, Clone)]
	static ref HISTORY: Mutex<String> = Mutex::new("".to_string());
	static ref VALID_KEY_REGEX: regex::Regex = regex::Regex::new(pomsky! {
		regex "(?u)" [horiz_space ascii_alnum ascii_punct]
	})
	.unwrap();
	static ref MAX_HISTORY_SIZE: AtomicUsize = AtomicUsize::new(30);
}

pub fn init() {
	// TODO: Initialize the settings to the values in the config.
}

pub fn backspace_history() {
	HISTORY.lock().unwrap().pop();
}
pub fn clear_history() {
	HISTORY.lock().unwrap().clear();
}
pub fn add_history(text: &str) {
	let mut history_mutex = HISTORY.lock().unwrap();
	history_mutex.push_str(text);

	let amount_over = history_mutex
		.len()
		.saturating_sub(MAX_HISTORY_SIZE.load(Ordering::SeqCst));

	if amount_over > 0 {
		history_mutex.drain(0..amount_over);
	}
}

pub fn handle_history(event: &Event) {
	match (event.event_type, event.name.clone()) {
		(event_type, Some(string)) => {
			if VALID_KEY_REGEX.is_match(string.as_str()) {
				add_history(&string);
			} else if event_type != KeyPress(Key::Backspace) {
				clear_history();
			} else {
				backspace_history();
			}
		}
		(KeyPress(Key::Backspace), _) => {
			backspace_history();
		}
		(
			KeyPress(Key::CapsLock)
			| KeyPress(Key::ShiftLeft)
			| KeyPress(Key::ShiftRight),
			_,
		) => {}
		(_, None) => {
			clear_history();
		}
	}
}

pub fn get_history() -> String {
	return HISTORY.lock().unwrap().clone();
}

pub fn get_max_history_size() -> usize {
	return MAX_HISTORY_SIZE.load(Ordering::SeqCst);
}
pub fn set_max_history_size(size: usize) {
	// Don't forget to remove characters from the start until the size matches the new max size.
	let old_size = MAX_HISTORY_SIZE.load(Ordering::SeqCst);
	MAX_HISTORY_SIZE.store(size, Ordering::SeqCst);
	if old_size > size {
		let amount_over = old_size.saturating_sub(size);
		HISTORY.lock().unwrap().drain(0..amount_over);
	}
}
