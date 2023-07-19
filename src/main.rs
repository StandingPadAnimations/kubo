use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
mod kubo_manager;
mod kubo_config;
mod operations;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let state = kubo_config::read_config(kubo_manager::KuboManager::new()); 
    let state = state.initial_copy();
    if let Err(error) = daemon(state.watch_paths(), state) {
        log::error!("Error: {error:?}");
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
                        operations::copy_to_kubo(&path, &state);
                    }
                }
            },
            Ok(notify::Event { kind: notify::EventKind::Remove(_), paths, .. }) => {
                log::info!("Change: {paths:?}");
                for path in paths {
                    log::info!("{path:?}");
                    // TODO: Don't perform a full copy on config change
                    if path.file_name().is_some() && path.file_name().unwrap() == "kubo.toml" {
                        log::error!("kubo.toml was just removed!")
                    } else {
                        operations::delete_from_kubo(&path, &state);
                    }
                }
            },
            Ok(_) => log::info!(""),
            Err(error) => log::error!("Error: {error:?}"),
        }
    }
    Ok(())
}
