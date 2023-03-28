use std::{
	fs,
	io::BufRead,
	process::{Command, Output, Stdio},
};

use path_absolutize::Absolutize;
use rdev::Key;

use crate::{
	config,
	simulate::{paste_text, release_keys},
};

pub fn start_macro(macro_name: &String, match_config: config::Match) {
	let macro_file_path = config::get_config_directory()
		.join(macro_name)
		.join(match_config.entry);
	let macro_file_path = macro_file_path
		.absolutize()
		.expect("Couldn't absolutize macro path.");
	let macro_path =
		macro_file_path.parent().expect("Macro path has no parent.");

	let macro_file =
		fs::read_to_string(macro_file_path.to_string_lossy().to_string())
			.expect("Couldn't read macro file.");

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
		.expect("Couldn't launch Deno.");

	let output = deno.wait_with_output().expect("Couldn't wait for Deno.");

	// match output.status {
	// 	ExitStatus => run_tasks(&output),
	// }
	run_tasks(&output);

	println!("{:#?}", output);
}

pub fn run_tasks(output: &Output) {
	// Release any important modifier keys.
	release_keys(vec![
		Key::ShiftLeft,
		Key::ShiftRight,
		Key::ControlLeft,
		Key::ControlRight,
		Key::MetaLeft,
		Key::MetaRight,
		Key::Alt,
		Key::AltGr,
	]);

	let lines = output.stdout.lines();

	for line in lines {
		let line = line.expect("Couldn't read script stdout lines.");

		println!("{}", line);

		if line.starts_with("scrype_paste_text:") {
			println!("PASTING");
			let idx = line
				.find(':')
				.expect(&format!("\"{}\" does not have a \":\".", line));
			let (_, text) = line.split_at(idx + 1);
			paste_text(text);
		}
	}
}
