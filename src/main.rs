use clap::{Parser, Subcommand};
use std::io::{self, Write};

mod kubo_manager;
mod kubo_config;
mod operations;
mod daemon;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds a dotfile to kubo.toml
    Add {
        /// The name associated with
        /// a set of dotfiles
        name: String, 
        /// The normal location of 
        /// a set of dotfiles
        src: String, 
        /// The target folder in 
        /// .kubo
        target: String 
    },
    /// Removes a dotfile from kubo.toml
    Rm { name: String },
    /// Lists all managed dotfiles
    Ls,
    /// Runs the Kubo daemon
    Daemon,
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add { name, src, target} => {
            let state = kubo_manager::KuboManager::new().lock();
            let res = kubo_config::add_dotfile(state, name, src, target);
            if res.is_ok() {
                println!("Added {} ({} -> {})", name, target, src);
            }
        }
        Commands::Rm { name } => {
            let state = kubo_manager::KuboManager::new().lock();
            let res = kubo_config::remove_dotfile(state, name);
            if res.is_ok() {
                println!("Removed {}", name);
            }
        }
        Commands::Ls => {
            let state = kubo_manager::KuboManager::new().lock();
            let list = kubo_config::list_dotfiles(state);

            // We use a buffer to reduce
            // the amount of times Rust flushes
            // the output; This helps when there's
            // a lot of dotfiles
            if let Ok(dots) = list {
                let stdout = io::stdout();
                let mut handle = io::BufWriter::new(stdout); 
                for d in dots {
                    let _ = writeln!(handle, "{}", d); 
                }
            }
        }
        Commands::Daemon => {
            // Create a lock by getting exclusive
            // write access to a lockfile
            let state = kubo_manager::KuboManager::new();
            let lockfile = daemon::LockFile::new(&state);
            if lockfile.is_err() {
                log::error!("Another Kubo instance is being ran!");
                return;
            }
            let mut lockfile = lockfile.unwrap();
            let mut lock = lockfile.0.write().unwrap();
            let _ = write!(lock, "{}", std::process::id());

            // Run the daemon
            let state = kubo_config::read_config(state); 
            let state = state.initial_copy();
            if let Err(error) = daemon::daemon(state.watch_paths(), state) {
                log::error!("Error: {error:?}");
            }
        }
    }
}
