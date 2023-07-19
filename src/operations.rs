use crate::kubo_manager;
use std::path::Path;

/// Copies files to $HOME/.kubo
///
/// Expects a locked state; if state
/// needs to be unlocked, it will be unlocked
/// here.
pub fn copy_to_kubo(dot_path: &Path, state: &kubo_manager::KuboManager::<kubo_manager::Locked>) {
    // Get the Kubo path
    let kubo_path = state.get_kubo_dir();
    let mut kubo_dir = Path::new(&kubo_path);
    if !kubo_dir.exists() {
        let res = std::fs::create_dir(kubo_dir);
        if let Err(err) = res {
            panic!("Kubo creation failed (C): {}", err);
        }
    }

    // Get target directory
    log::info!("Dot path: {dot_path:?}");
    let target = state.get_target(dot_path);
    if target.is_err() {
        log::error!("Path not managed (C): {target:?}");
    }
    let target = kubo_dir.join(target.unwrap());
    if target.exists() {
        kubo_dir = &target;
    }

    // Perform the actual copying
    log::info!("Kubo Dir (C): {kubo_dir:?}");
    if dot_path.is_dir() {
        let options = fs_extra::dir::CopyOptions::new()
                        .overwrite(true);
        let res = fs_extra::dir::copy(dot_path, kubo_dir, &options);
        if let Err(err) = res {
            log::error!("Copying dir failed (C): {err:?}");
        }
    } else {
        let res = std::fs::copy(dot_path, kubo_dir);
        if let Err(err) = res {
            log::error!("Copying file failed (C): {err:?}");
        }   
    }
}

/// Deletes files to $HOME/.kubo
///
/// Expects a locked state; if state
/// needs to be unlocked, it will be unlocked
/// here.
///
/// Almost identical to copying but with deleting 
/// instead.
pub fn delete_from_kubo(dot_path: &Path, state: &kubo_manager::KuboManager::<kubo_manager::Locked>) {
    // Get the Kubo path
    let kubo_path = state.get_kubo_dir();
    let mut kubo_dir = Path::new(&kubo_path);
    if !kubo_dir.exists() {
        let res = std::fs::create_dir(kubo_dir);
        if let Err(err) = res {
            panic!("Kubo creation failed (R): {}", err);
        }
    }

    // Get target directory
    log::info!("Dot path: {dot_path:?}");
    let target = state.get_target(dot_path);
    if target.is_err() {
        log::error!("Path not managed (R): {target:?}");
    }
    let target = kubo_dir.join(target.unwrap());
    if target.exists() {
        kubo_dir = &target;
    }

    // Perform the actual removing
    log::info!("Kubo Dir: {kubo_dir:?}");
    if dot_path.is_dir() {
        let res = std::fs::remove_dir_all(kubo_dir);
        if let Err(err) = res {
            log::error!("Removing dir failed (R): {err:?}");
        }
    } else {
        let res = std::fs::remove_file(kubo_dir);
        if let Err(err) = res {
            log::error!("Removing file failed (R): {err:?}");
        }   
    }
}
