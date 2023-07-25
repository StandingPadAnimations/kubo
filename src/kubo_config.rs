use crate::kubo_manager;
use std::{fs::File, io::Read, path::{Path, PathBuf}};
use toml::Table;
use toml_edit::{value, Document};

// Stolen from Stackoverflow (https://stackoverflow.com/questions/65976432/how-to-remove-first-and-last-character-of-a-string-in-rust/65976629#65976629)
fn rem_first_and_last(value: String) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}

pub fn read_config(
    state: kubo_manager::KuboManager<kubo_manager::Unlocked>,
) -> kubo_manager::KuboManager<kubo_manager::Locked> {
    let mut toml_config = String::new();
    File::open(state.get_kubo_dir()
            .join::<PathBuf>("kubo.toml".into()))
        .and_then(|mut f| f.read_to_string(&mut toml_config))
        .unwrap();

    let toml_config = toml_config.parse::<Table>().unwrap();
    log::info!("{toml_config:?}");
    let mut temp_state = state;
    for item in toml_config {
        let src = rem_first_and_last(item.1["source"].to_string());
        temp_state = temp_state.add_path(src.into());
    }
    temp_state.lock()
}

pub fn add_dotfile(
    state: kubo_manager::KuboManager<kubo_manager::Locked>,
    name: &str,
    src: &Path,
) -> Result<(), ()> {
    let mut toml_config = String::new();
    File::open(state.get_kubo_dir().join::<PathBuf>("kubo.toml".into()))
        .and_then(|mut f| f.read_to_string(&mut toml_config))
        .unwrap();

    let mut toml_config = toml_config.parse::<Document>().unwrap();
    toml_config[name]["source"] = value(src.to_str().unwrap());
    let res = std::fs::write(state.get_kubo_dir()
            .join::<PathBuf>("kubo.toml".into()), 
        toml_config.to_string());
    if res.is_err() {
        return Err(());
    }
    Ok(())
}

pub fn remove_dotfile(
    state: kubo_manager::KuboManager<kubo_manager::Locked>,
    name: &str,
) -> Result<(), ()> {
    let mut toml_config = String::new();
    File::open(state.get_kubo_dir().join::<PathBuf>("kubo.toml".into()))
        .and_then(|mut f| f.read_to_string(&mut toml_config))
        .unwrap();

    let mut toml_config = toml_config.parse::<Document>().unwrap();
    toml_config.remove(name);
    let res = std::fs::write(state.get_kubo_dir()
            .join::<PathBuf>("kubo.toml".into()), 
        toml_config.to_string());
    if res.is_err() {
        return Err(());
    }
    Ok(())
}

pub fn list_dotfiles(
    state: kubo_manager::KuboManager<kubo_manager::Locked>,
) -> Result<Vec<String>, ()> {
    let mut toml_config = String::new();
    File::open(state.get_kubo_dir().join::<PathBuf>("kubo.toml".into()))
        .and_then(|mut f| f.read_to_string(&mut toml_config))
        .unwrap();

    let toml_config = toml_config.parse::<Table>().unwrap();
    let mut res: Vec<String> = Vec::new();
    for dot in toml_config {
        res.push(dot.0);
    }
    Ok(res)
}
