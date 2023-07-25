use crate::kubo_manager;
use std::path::{Path, PathBuf};

/// Get the target directory given a
/// source directory or path
///
/// Returns: PathBuf
fn get_target_dir(
    dot_path: &Path,
    state: &kubo_manager::KuboManager<kubo_manager::Locked>,
) -> PathBuf {
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
    let target = state.convert_path(dot_path);
    if target.is_err() {
        log::error!("Path not managed: {target:?}");
    }
    let target = target.unwrap();
    if dot_path.is_dir() && !target.exists() {
        fs_extra::dir::create_all(target.clone(), false).unwrap();
        log::info!("Dir created, {}", target.clone().to_str().unwrap());
    }
    kubo_dir = &target;
    kubo_dir.to_owned()
}

/// Copies files to $HOME/.kubo
///
/// Expects a locked state; if state
/// needs to be unlocked, it will be unlocked
/// here.
pub fn copy_to_kubo(
    dot_path: &Path,
    state: &kubo_manager::KuboManager<kubo_manager::Locked>,
) {
    // Perform the actual copying
    let kubo_dir = get_target_dir(dot_path, state);
    log::info!("Kubo Dir (C): {kubo_dir:?}");
    if dot_path.is_dir() {
        let options = fs_extra::dir::CopyOptions::new()
            .overwrite(true);
        let res = fs_extra::dir::copy(dot_path, kubo_dir, &options);
        if let Err(err) = res {
            log::error!("Copying dir failed (C): {err:?}");
        }
    } else {
        let options = fs_extra::file::CopyOptions::new().overwrite(true);
        let res = fs_extra::file::copy(dot_path, kubo_dir, &options);
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
pub fn delete_from_kubo(dot_path: &Path, state: &kubo_manager::KuboManager<kubo_manager::Locked>) {
    // Perform the actual removing
    let kubo_dir = get_target_dir(dot_path, state);
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
