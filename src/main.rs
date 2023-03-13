#![feature(pin_macro)]

use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

use lazy_static::lazy_static;
use rdev::{grab, Event, EventType, Key};

use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::{futures::executor, Extension};

pub mod config;
pub mod deno;
pub mod handler;
pub mod history;
pub mod macros;
pub mod simulate;

fn main() {
    // This will block.
    if let Err(error) = grab(handler::event_listener) {
        println!("Error: {:?}", error)
    }
}
