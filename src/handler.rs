use std::{
	sync::atomic::{AtomicBool, Ordering},
	thread,
};

use lazy_static::lazy_static;
use rdev::{grab, Event, EventType, Key};

use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::{futures::executor, Extension};

use crate::{config, deno::runner::start_macro, history, simulate};

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
	if !is_handler_locked() {
		match event.event_type {
			EventType::KeyPress(_) => {
				history::handle_history(&event);

				let config = config::get_config();
				let mut history = history::get_history();

				if history.ends_with(&config.suffix) {
					// Remove the suffix so matching is easier.
					history.truncate(history.len() - config.suffix.len());

					for macro_name in config.macros {
						let macro_config =
							config::get_macro_config(&macro_name);
						if macro_config.match_type == config::MatchType::Text
						{
							if history.ends_with(&macro_config.r#match) {
								lock_handler();

								let backspace_amount =
									macro_config.r#match.len()
										+ config.prefix.len() + config
										.suffix
										.len();
								thread::spawn(move || {
									simulate::type_keys(vec![
										Key::Backspace;
										backspace_amount
									]);

									start_macro(&macro_name);

									history::clear_history();
									unlock_handler();
									// // Build a deno_core::Extension providing custom ops
									// let ext = Extension::builder("my_ext")
									//     .ops(js::ops::get_all_ops())
									//     .build();

									// // Initialize a runtime instance
									// let mut runtime = JsRuntime::new(RuntimeOptions {
									//     extensions: vec![ext],
									//     ..Default::default()
									// });

									// // Now we see how to invoke the op we just defined. The runtime automatically
									// // contains a Deno.core object with several functions for interacting with it.
									// // You can find its definition in core.js.

									// runtime
									//     .execute_script("<usage>", "Deno.core.ops.sendText('works');")
									//     .unwrap();

									//   executor::block_on(runtime.run_event_loop(true)).unwrap();
								});
							}
						}
					}
				}
			}
			_ => (),
		}
	}
	Some(event)
}
