use rdev::grab;

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
