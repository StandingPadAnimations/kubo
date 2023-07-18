use std::{fs::File, io::Read};
use toml::Table;
use crate::kubo_manager;

// Stolen from Stackoverflow (https://stackoverflow.com/questions/65976432/how-to-remove-first-and-last-character-of-a-string-in-rust/65976629#65976629)
fn rem_first_and_last(value: String) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}

pub fn read_config(state: kubo_manager::KuboManager::<kubo_manager::Unlocked>) -> kubo_manager::KuboManager::<kubo_manager::Locked> {
    let mut toml_config = String::new();
    File::open(state.get_kubo_dir() + "/kubo.toml")
        .and_then(|mut f| f.read_to_string(&mut toml_config))
        .unwrap();

    let toml_config: Table = toml_config.parse().unwrap();
    log::info!("{toml_config:?}");
    let mut temp_state = state;
    for item in toml_config {
        let src = rem_first_and_last(item.1["source"].to_string());
        let dst = rem_first_and_last(item.1["target"].to_string());
        temp_state = temp_state.add_path(src.to_string(), dst.to_string());
    }
    temp_state.lock()
}
