use std::path::{Path, PathBuf};

use crate::operations;

/// Represents a locked
/// KuboManager
pub struct Locked;

/// Represents an unlocked
/// KuboManager
pub struct Unlocked;

/// Stores information about
/// dotfiles and their equivilent
/// in $HOME/.kubo
///
/// In addition, KuboManager has a
/// state that decides whether new paths
/// should be added or not.
pub struct KuboManager<State = Unlocked> {
    /// A list of source paths to
    /// their target paths.
    paths: Vec<PathBuf>,
    kubo_dir: PathBuf,
    state: std::marker::PhantomData<State>,
}

impl KuboManager {
    /// Create a new KuboManager
    /// with no paths to manage.
    ///
    /// Returns: KuboManager<Unlocked>
    pub fn new() -> Self {
        // If this fails, I have several questions
        let home_dir = std::env::var("HOME");
        if home_dir.is_err() {
            panic!("Home directory not found!")
        }
        KuboManager {
            paths: Vec::new(),
            kubo_dir: PathBuf::from(home_dir.unwrap() + "/.kubo"),
            state: Default::default(),
        }
    }
}

impl<State> KuboManager<State> {
    /// Given a path, return the 
    /// Kubo equivilent
    ///
    /// Returns: 
    /// - PathBuf if path is managed by Kubo
    /// - () otherwise
    pub fn convert_path(&self, path: &Path) -> Result<PathBuf, ()> {
        for managed_path in &self.paths {
            if path.starts_with(managed_path){
                let home_dir = PathBuf::from(std::env::var("HOME").unwrap());
                return Ok(
                    self.kubo_dir.join(path 
                            .strip_prefix(home_dir) 
                            .unwrap()));
            }
        }
        Err(())
    }

    /// Return the kubo folder path
    ///
    /// Returns: String
    pub fn get_kubo_dir(&self) -> PathBuf {
        self.kubo_dir.clone()
    }
}

impl KuboManager<Unlocked> {
    /// Add a path for KuboManager
    /// to manage.
    ///
    /// Returns: KuboManager<Unlocked>
    ///
    /// path: the path of the actual configuration
    pub fn add_path(mut self, path: PathBuf) -> KuboManager<Unlocked> {
        self.paths.push(path);
        self
    }

    /// Clears all paths stored in
    /// the manager.
    ///
    /// Returns: KuboManager<Unlocked>
    pub fn clear_paths(mut self) -> KuboManager<Unlocked> {
        self.paths = Vec::new();
        self
    }

    /// Makes KuboManager read only.
    ///
    /// Returns: KuboManager<Locked>
    pub fn lock(self) -> KuboManager<Locked> {
        KuboManager::<Locked> {
            paths: self.paths,
            kubo_dir: self.kubo_dir,
            state: std::marker::PhantomData::<Locked>,
        }
    }
}

impl KuboManager<Locked> {
    /// Makes KuboManager writeable.
    ///
    /// Returns: KuboManager<Unlocked>
    pub fn unlock(self) -> KuboManager<Unlocked> {
        KuboManager::<Unlocked> {
            paths: self.paths,
            kubo_dir: self.kubo_dir,
            state: std::marker::PhantomData::<Unlocked>,
        }
    }

    /// Creates an initial copy of all
    /// files; this is to be ran once at
    /// startup.
    ///
    /// Returns: KuboManager<Locked>
    pub fn initial_copy(self) -> KuboManager<Locked> {
        for path in &self.paths {
            operations::copy_to_kubo(Path::new(path), &self);
        }
        self
    }

    /// Returns source paths as Path objects.
    ///
    /// Return: Vec<AsRef<Path>>
    pub fn watch_paths(&self) -> Vec<PathBuf> {
        let mut wpaths = Vec::new();
        for path in &self.paths {
            wpaths.push(PathBuf::from(path));
        }
        wpaths
    }
}
