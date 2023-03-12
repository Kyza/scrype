use arboard::Clipboard;
use deno_core::{op, OpDecl};
use rdev::Key;

use crate::{config, history, simulate};

#[op]
fn pressKey(key: Key) -> Result<(), deno_core::error::AnyError> {
    simulate::press_key(key);
    Ok(())
}
#[op]
fn pressKeys(keys: Vec<Key>) -> Result<(), deno_core::error::AnyError> {
    simulate::press_keys(keys);
    Ok(())
}

#[op]
fn releaseKey(key: Key) -> Result<(), deno_core::error::AnyError> {
    simulate::release_key(key);
    Ok(())
}
#[op]
fn releaseKeys(keys: Vec<Key>) -> Result<(), deno_core::error::AnyError> {
    simulate::release_keys(keys);
    Ok(())
}

#[op]
fn typeKey(key: Key) -> Result<(), deno_core::error::AnyError> {
    simulate::type_key(key);
    Ok(())
}

#[op]
fn typeKeys(keys: Vec<Key>) -> Result<(), deno_core::error::AnyError> {
    simulate::type_keys(keys);
    Ok(())
}

#[op]
fn sendText(text: String) -> Result<(), deno_core::error::AnyError> {
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
fn getHistory() -> Result<String, deno_core::error::AnyError> {
    Ok(history::get_history())
}

#[op]
fn getConfigDirectory() -> Result<String, deno_core::error::AnyError> {
    Ok(config::get_config_directory().to_string_lossy().to_string())
}

pub(crate) fn get_all_ops() -> Vec<OpDecl> {
    vec![
        pressKey::decl(),
        pressKeys::decl(),
        releaseKey::decl(),
        releaseKeys::decl(),
        typeKey::decl(),
        typeKeys::decl(),
        sendText::decl(),
        getHistory::decl(),
        getConfigDirectory::decl(),
    ]
}
