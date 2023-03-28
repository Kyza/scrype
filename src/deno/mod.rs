use std::{
	fs,
	io::BufRead,
	process::{Command, Output, Stdio},
};

use path_absolutize::Absolutize;

use serde_json::Result;

use crate::{
	config,
	deno::actions::{handle_action, ScrypeAction},
};

pub mod actions;

pub fn start_macro(macro_name: &String, match_config: config::Match) {
	let macro_file_path = config::get_config_directory()
		.join(macro_name)
		.join(match_config.entry);
	let macro_file_path = macro_file_path
		.absolutize()
		.expect("Failed to absolutize macro path.");
	let macro_path =
		macro_file_path.parent().expect("Macro path has no parent.");

	let macro_file =
		fs::read_to_string(macro_file_path.to_string_lossy().to_string())
			.expect("Failed to read macro file.");

	let runtime = include_str!("runtime.ts");

	let macro_file_with_runtime = runtime.to_owned() + &macro_file;

	let deno = Command::new("deno")
		.args(&[
			"eval",
			// "--allow-all",
			"--unstable",
			&macro_file_with_runtime,
		])
		.current_dir(macro_path)
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.expect("Failed to launch Deno.");

	let output = deno.wait_with_output().expect("Failed to wait for Deno.");

	// match output.status {
	// 	ExitStatus => run_tasks(&output),
	// }
	run_tasks(&output);

	println!("{:#?}", output);
}

pub fn run_tasks(output: &Output) {
	let lines = output.stdout.lines();

	for line in lines {
		let line = line.expect("Failed to read script stdout lines.");

		if line.starts_with("[SCRYPE]:") {
			// It's an API action, so try to parse and run it.
			let idx = line
				.find(':')
				.expect(&format!("\"{}\" does not have a \":\".", line));
			let action_data = line.split_at(idx + 1).1.trim();

			let action: Result<ScrypeAction> =
				serde_json::from_str(action_data);

			match action {
				Err(err) => {
					println!(
						"Failed to deserialize action \"\n{}\".\n{}",
						action_data, err
					);
				}
				Ok(action) => {
					handle_action(&action);
				}
			}
		} else {
			// It's a normal line, so print it.
			println!("{}", line);
		}
	}
}
