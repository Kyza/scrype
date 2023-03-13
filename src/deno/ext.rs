use arboard::Clipboard;
use deno_core::{include_js_files, op, Extension, ExtensionBuilder};
use rdev::Key;

use crate::{config, history, simulate};

#[op]
fn scrype_pressKey(key: Key) -> Result<(), deno_core::error::AnyError> {
	simulate::press_key(key);
	Ok(())
}
#[op]
fn scrype_pressKeys(
	keys: Vec<Key>,
) -> Result<(), deno_core::error::AnyError> {
	simulate::press_keys(keys);
	Ok(())
}

#[op]
fn scrype_releaseKey(key: Key) -> Result<(), deno_core::error::AnyError> {
	simulate::release_key(key);
	Ok(())
}
#[op]
fn scrype_releaseKeys(
	keys: Vec<Key>,
) -> Result<(), deno_core::error::AnyError> {
	simulate::release_keys(keys);
	Ok(())
}

#[op]
fn scrype_typeKey(key: Key) -> Result<(), deno_core::error::AnyError> {
	simulate::type_key(key);
	Ok(())
}

#[op]
fn scrype_typeKeys(keys: Vec<Key>) -> Result<(), deno_core::error::AnyError> {
	simulate::type_keys(keys);
	Ok(())
}

#[op]
fn scrype_pasteText(text: String) -> Result<(), deno_core::error::AnyError> {
	let mut clipboard = Clipboard::new()?;
	let previous = clipboard.get_text()?;

	clipboard.set_text(text)?;

	simulate::press_key(Key::ControlLeft);
	simulate::press_key(Key::KeyV);
	simulate::release_key(Key::KeyV);
	simulate::release_key(Key::ControlLeft);

	clipboard.set_text(previous)?;

	Ok(())
}

#[op]
fn scrype_getHistory() -> Result<String, deno_core::error::AnyError> {
	Ok(history::get_history())
}

#[op]
fn scrype_getConfigDirectory() -> Result<String, deno_core::error::AnyError> {
	Ok(config::get_config_directory().to_string_lossy().to_string())
}

#[op]
fn scrype_getConfigOptions(
) -> Result<config::ScrypeSettings, deno_core::error::AnyError> {
	Ok(config::get_config())
}

fn ext() -> ExtensionBuilder {
	Extension::builder(env!("CARGO_PKG_NAME"))
}

pub(crate) fn ops(ext: &mut ExtensionBuilder) -> &mut ExtensionBuilder {
	ext.ops(vec![
		scrype_pressKey::decl(),
		scrype_pressKeys::decl(),
		scrype_releaseKey::decl(),
		scrype_releaseKeys::decl(),
		scrype_typeKey::decl(),
		scrype_typeKeys::decl(),
		scrype_pasteText::decl(),
		scrype_getHistory::decl(),
		scrype_getConfigDirectory::decl(),
		scrype_getConfigOptions::decl(),
	])
}

pub fn init_ops_and_esm() -> Extension {
	ops(&mut ext())
		.esm(include_js_files!("runtime.js",))
		.build()
}

pub fn init_ops() -> Extension {
	ops(&mut ext()).build()
}
