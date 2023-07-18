use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
mod kubo_manager;
mod kubo_config;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let state = kubo_config::read_config(kubo_manager::KuboManager::new()); 
    let state = state.initial_copy();
    if let Err(error) = daemon(state.watch_paths(), state) {
        log::error!("Error: {error:?}");
    }
}

/// Copies files to $HOME/.kubo
///
/// Expects a locked state; if state
/// needs to be unlocked, it will be unlocked
/// here.
fn copy_to_kubo(dot_path: &Path, state: &kubo_manager::KuboManager::<kubo_manager::Locked>) {
    // Get the Kubo path
    let kubo_path = state.get_kubo_dir();
    let mut kubo_dir = Path::new(&kubo_path);
    if !kubo_dir.exists() {
        let res = std::fs::create_dir(kubo_dir);
        if let Err(err) = res {
            panic!("Kubo creation failed: {}", err);
        }
    }

    // Get target directory
    log::info!("Dot path: {dot_path:?}");
    let target = state.get_target(dot_path);
    if target.is_err() {
        log::error!("Path not managed: {target:?}");
    }
    let target = kubo_dir.join(target.unwrap());
    if target.exists() {
        kubo_dir = &target;
    }

    // Perform the actual copying
    log::info!("Kubo Dir: {kubo_dir:?}");
    if dot_path.is_dir() {
        let options = fs_extra::dir::CopyOptions::new()
                        .overwrite(true);
        let res = fs_extra::dir::copy(dot_path, kubo_dir, &options);
        if let Err(err) = res {
            log::error!("Copying dir failed: {err:?}");
        }
    } else {
        let res = std::fs::copy(dot_path, kubo_dir);
        if let Err(err) = res {
            log::error!("Copying file failed: {err:?}");
        }   
    }
}

/// Actual daemon that watches files for changes
fn daemon<P: AsRef<Path>>(paths: Vec<P>, mut state: kubo_manager::KuboManager::<kubo_manager::Locked>) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    for p in paths {
        watcher.watch(p.as_ref(), RecursiveMode::Recursive)?;
    }

    // Add Kubo config
    let kubo_config = PathBuf::from(state.get_kubo_dir() + "/kubo.toml");
    watcher.watch(kubo_config.as_ref(), RecursiveMode::NonRecursive)?;
    for res in rx {
        match res {
            Ok(notify::Event { kind: notify::EventKind::Modify(_), paths, .. }) => {
                log::info!("Change: {paths:?}");
                for path in paths {
                    log::info!("{path:?}");
                    // TODO: Don't perform a full copy on config change
                    if path.file_name().is_some() && path.file_name().unwrap() == "kubo.toml" {
                        let nstate = state.unlock();
                        let nstate = nstate.clear_paths();
                        state = kubo_config::read_config(nstate);
                        state = state.initial_copy();
                    } else {
                        copy_to_kubo(&path, &state);
                    }
                }
            },
            Ok(_) => log::info!(""),
            Err(error) => log::error!("Error: {error:?}"),
        }
    }
    Ok(())
}
