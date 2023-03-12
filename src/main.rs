use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

use lazy_static::lazy_static;
use rdev::{
    listen, Event,
    EventType::{self},
    Key,
};

use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::{futures::executor, Extension};

pub mod config;
pub mod deno_ops;
pub mod history;
pub mod simulate;

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

fn run_deno() {
    thread::spawn(move || {
        simulate::press_keys(vec![
            Key::Backspace,
            Key::Backspace,
            Key::Backspace,
            Key::Backspace,
        ]);

        // Build a deno_core::Extension providing custom ops
        let ext = Extension::builder("my_ext")
            .ops(deno_ops::get_all_ops())
            .build();

        // Initialize a runtime instance
        let mut runtime = JsRuntime::new(RuntimeOptions {
            extensions: vec![ext],
            ..Default::default()
        });

        // Now we see how to invoke the op we just defined. The runtime automatically
        // contains a Deno.core object with several functions for interacting with it.
        // You can find its definition in core.js.

        runtime
            .execute_script("<usage>", "Deno.core.ops.sendText('works');")
            .unwrap();

        executor::block_on(runtime.run_event_loop(true)).unwrap();
        history::clear_history();
        unlock_handler();
    });
}

fn events(event: Event) {
    if !is_handler_locked() {
        match event.event_type {
            EventType::KeyPress(_) => {
                history::handle_history(&event);
                if history::get_history().contains("test") {
                    lock_handler();
                    run_deno();
                }
            }
            _ => (),
        }
    }
}

fn main() {
    // This will block.
    if let Err(error) = listen(events) {
        println!("Error: {:?}", error)
    }
}
