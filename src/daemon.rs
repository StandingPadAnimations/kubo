use crate::{kubo_config, kubo_manager, operations};
use fd_lock::RwLock;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

pub struct LockFile(pub RwLock<File>);
impl LockFile {
    pub fn new(state: &kubo_manager::KuboManager<kubo_manager::Unlocked>) -> Result<Self, ()> {
        let path = state.get_kubo_dir() + "/kubo.lockfile";
        if Path::new(&path).exists() == false {
            if let Err(_) = File::create(&path) {
                return Err(());
            }
        }
        let mut lock = RwLock::new(File::open(&path).unwrap());
        {
            let write_lock = lock.try_write();
            if write_lock.is_err() {
                return Err(());
            }
        }
        Ok(LockFile(lock))
    }
}

/// Actual daemon that watches files for changes
pub fn daemon<P: AsRef<Path>>(
    paths: Vec<P>,
    mut state: kubo_manager::KuboManager<kubo_manager::Locked>,
) -> notify::Result<()> {
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
            Ok(notify::Event {
                kind: notify::EventKind::Modify(_),
                paths,
                ..
            }) => {
                // Sleep for a 500ms to account for editor shenanigans
                std::thread::sleep(std::time::Duration::from_millis(50));
                let mut processed_paths = Vec::new();
                for path in paths {
                    if !path.exists() {
                        continue;
                    } else if processed_paths.contains(&path) {
                        // Let's skip since it's
                        // not humanly possible to
                        // edit and save configurations
                        // that fast
                        continue;
                    }
                    processed_paths.push(path.clone());
                    log::info!("{path:?}");
                    // TODO: Don't perform a full copy on config change
                    if path.file_name().is_some() && path.file_name().unwrap() == "kubo.toml" {
                        let nstate = state.unlock();
                        let nstate = nstate.clear_paths();
                        state = kubo_config::read_config(nstate);
                        state = state.initial_copy();
                    } else {
                        operations::copy_to_kubo(&path, &state, operations::WithTarget::Yay);
                    }
                }
            }
            Ok(notify::Event {
                kind: notify::EventKind::Remove(_),
                paths,
                ..
            }) => {
                // Sleep for a 500ms to account for editor shenanigans
                std::thread::sleep(std::time::Duration::from_millis(50));
                let mut processed_paths = Vec::new();
                for path in paths {
                    // If the path still exists, then
                    // why would we need to remove the
                    // file?
                    if path.exists() {
                        continue;
                    } else if processed_paths.contains(&path) {
                        // Let's skip since it's
                        // not humanly possible to
                        // edit and save configurations
                        // that fast
                        continue;
                    }
                    processed_paths.push(path.clone());
                    log::info!("{path:?}");
                    // TODO: Don't perform a full copy on config change
                    if path.file_name().is_some() && path.file_name().unwrap() == "kubo.toml" {
                        log::error!("kubo.toml was just removed!")
                    } else {
                        operations::delete_from_kubo(&path, &state);
                    }
                }
            }
            Ok(_) => continue,
            Err(error) => log::error!("Error: {error:?}"),
        }
    }
    Ok(())
}
