use std::{
	fs::{self, File},
	path::{Path, PathBuf},
};

use home::home_dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrypeSettings {
	#[serde(default)]
	pub prefix: String,
	#[serde(default)]
	pub suffix: String,
	#[serde(default)]
	pub macros: Vec<String>,
}

pub fn get_config() -> ScrypeSettings {
	let config_dir = get_config_directory();
	let config_file_path = config_dir.join("config.json");
	let config_file = File::open(&config_file_path);

	if let Err(err) = config_file {
		panic!(
            "Failed to read config file. Please create one at \"{:?}\" if it doesn't exist.\n{}",
            config_file_path, err
        );
	}
	let config_file = config_file.unwrap();

	let config_data = serde_json::from_reader(config_file);

	if let Err(err) = config_data {
		panic!("Failed to deserialize config file. \n{}", err);
	}

	config_data.unwrap()
}

pub fn get_config_directory() -> PathBuf {
	match home_dir() {
		Some(home) => {
			let dir = Path::new(&home).join(".scrype");
			fs::create_dir_all(&dir).unwrap_or(());
			return dir;
		}
		None => {
			panic!("Failed to get home directory. Something has gone horribly wrong.");
		}
	}
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MatchType {
	Text,
	RegEx,
	Pomsky,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Match {
	#[serde(default)]
	pub r#match: String,
	#[serde(default = "default_match_type")]
	pub r#type: MatchType,
	#[serde(default = "default_macro_entry")]
	pub entry: String,
}

fn default_match_type() -> MatchType {
	MatchType::Text
}
fn default_macro_entry() -> String {
	"index.js".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MacroConfig {
	#[serde(default)]
	pub matches: Vec<Match>,
}

pub fn get_macro_config(name: &str) -> MacroConfig {
	let config_dir = get_config_directory();
	let config_file_path = config_dir.join(name).join("config.json");
	let config_file = File::open(&config_file_path);

	if let Err(err) = config_file {
		panic!(
            "Failed to read config file. Please create one at \"{:?}\" if it doesn't exist.\n{}",
            config_file_path, err
        );
	}
	let config_file = config_file.unwrap();

	let config_data: serde_json::Result<MacroConfig> =
		serde_json::from_reader(config_file);

	if let Err(err) = config_data {
		panic!("Failed to deserialize config file. \n{}", err);
	}
	let config_data = config_data.unwrap();

	config_data
}
