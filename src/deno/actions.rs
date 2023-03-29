use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::simulate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeStringAction {
	pub text: String,
	#[serde(default)]
	pub shift_return: bool,
	#[serde(default = "default_duration")]
	pub delay: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeKeysAction {
	pub keys: Vec<u16>,
	#[serde(default = "default_duration")]
	pub delay: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ScrypeAction {
	#[serde(rename = "TYPE_STRING")]
	TypeString(TypeStringAction),
	#[serde(rename = "TYPE_KEYS")]
	TypeKeys(TypeKeysAction),
	None,
}

pub fn default_duration() -> Duration {
	Duration::NANOSECOND
}

pub fn handle_action(action: &ScrypeAction) {
	match action {
		ScrypeAction::TypeString(action) => {
			simulate::send_string(&action.text)
				.expect("Failed to send string.");
		}
		ScrypeAction::TypeKeys(_action) => {
			// simulate::type_keycodes_delay(&action.keys, 1);
		}
		ScrypeAction::None => {
			println!("Invalid/unimplemented action \"{:#?}\".", action)
		}
	}
}
