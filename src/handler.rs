use std::{
	sync::atomic::{AtomicBool, Ordering},
	thread,
	time::Instant,
};

use cfg_if::cfg_if;
use lazy_static::lazy_static;
use rdev::{Event, EventType};
use windows::Win32::UI::Input::KeyboardAndMouse::VK_BACK;

use crate::{
	config,
	deno::start_macro,
	history,
	simulate::{self},
};

lazy_static! {
	static ref HANDLER_LOCKED: AtomicBool = AtomicBool::new(false);
}

pub fn lock_handler() {
	set_handler_locked(true);
}
pub fn unlock_handler() {
	set_handler_locked(false);
}
pub fn set_handler_locked(locked: bool) {
	HANDLER_LOCKED.store(locked, Ordering::SeqCst);
}
pub fn is_handler_locked() -> bool {
	return HANDLER_LOCKED.load(Ordering::SeqCst);
}

pub fn event_listener(event: Event) -> Option<Event> {
	if is_handler_locked() {
		return Some(event);
	}

	match event.event_type {
		EventType::KeyPress(_) => {
			history::handle_history(&event);

			let config = config::get_config();
			let mut history = history::get_history();

			// Ensure the history has the prefix.
			let prefix_index = history.find(&config.prefix);
			if prefix_index == None {
				return Some(event);
			}
			// Remove until the prefix so matching is faster.
			history.drain(0..(prefix_index.unwrap() + config.prefix.len()));

			// After the prefix is cut out, ensure it has the suffix.
			if !history.ends_with(&config.suffix) {
				return Some(event);
			}
			// Remove the suffix so matching is faster.
			history.truncate(history.len() - config.suffix.len());

			for macro_name in config.macros {
				let macro_config =
					config::get_macro_config(macro_name.to_string());

				for macro_config_match in macro_config.matches {
					match macro_config_match.r#type {
						config::MatchType::Text => {
							if history.ends_with(&macro_config_match.r#match)
							{
								lock_handler();

								// Erase the prefix, match, and suffix, minus the last character since it gets blocked.
								let backspace_amount =
									macro_config_match.r#match.len()
										+ config.prefix.len() + config
										.suffix
										.len() - 1;

								// Clone the name so it doesn't get moved.
								let macro_name = macro_name.clone();
								thread::spawn(move || {
									cfg_if! {
										if #[cfg(windows)] {
											// Optimized backspacer.
											// Windows can handle multiple presses without releases, but not all OSes can.
											for _ in 0..backspace_amount {
												simulate::win32::press_vk(VK_BACK);
												simulate::minimal_sleep();
											}
											simulate::win32::release_vk(VK_BACK);
											simulate::minimal_sleep();
										} else {
											simulate::type_keys(&vec![
												Key::Backspace; backspace_amount
											])
											.expect("Failed to backspace match.");
										}
									}

									let code_now = Instant::now();
									start_macro(
										&macro_name,
										macro_config_match,
									);
									println!(
										"{} code ran in {}ms.",
										macro_name,
										code_now.elapsed().as_millis()
									);

									history::clear_history();
									unlock_handler();
								});

								return None;
							}
						}
						match_type => {
							println!(
								"{} uses an unimplemented match type: {:?}",
								macro_name, match_type
							);
						}
					}
				}
			}
		}
		_ => (),
	}

	Some(event)
}
