use std::time::Duration;

use rdev::Key;
use serde::{Deserialize, Serialize};

use crate::simulate::{paste_text, type_keys};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasteAction {
	pub text: String,
	#[serde(default = "default_duration")]
	pub delay: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAction {
	pub keys: Vec<Key>,
	#[serde(default = "default_duration")]
	pub delay: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ScrypeAction {
	#[serde(rename = "PASTE")]
	Paste(PasteAction),
	#[serde(rename = "TYPE")]
	Type(TypeAction),
	None,
}

pub fn default_duration() -> Duration {
	Duration::NANOSECOND
}

pub fn handle_action(action: &ScrypeAction) {
	match action {
		ScrypeAction::Paste(action) => {
			paste_text(&action.text, action.delay)
				.expect("Failed to run PASTE action.");
		}
		ScrypeAction::Type(action) => {
			type_keys(&action.keys, action.delay)
				.expect("Failed to run TYPE action.");
		}
		ScrypeAction::None => {
			println!("Invalid/unimplemented action \"{:#?}\".", action)
		}
	}
}
