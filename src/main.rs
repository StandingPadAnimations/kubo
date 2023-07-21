use clap::{Parser, Subcommand, CommandFactory, Command, ValueHint};
use std::{io::{self, Write}, path::PathBuf};
use clap_complete::{generate, Generator, Shell};

mod daemon;
mod kubo_config;
mod kubo_manager;
mod operations;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long = "generate", value_enum)]
    generator: Option<Shell>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds a dotfile to kubo.toml
    Add {
        /// The name associated with
        /// a set of dotfiles
        #[arg(short, long, value_hint = ValueHint::Unknown)]
        name: String,
        /// The normal location of
        /// a set of dotfiles
        #[arg(short, long, value_hint = ValueHint::AnyPath)]
        src: PathBuf,
        /// The target folder in
        /// .kubo
        #[arg(short, long, value_hint = ValueHint::Unknown)]
        target: String,
    },
    /// Removes a dotfile from kubo.toml
    Rm { 
        #[arg(value_hint = ValueHint::Unknown)]
        name: String 
    },
    /// Lists all managed dotfiles
    Ls,
    /// Runs the Kubo daemon
    Daemon,
}


fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let cli = Cli::parse();
    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();
        print_completions(generator, &mut cmd);
    } else {
        if cli.command.is_some() {
            match &cli.command.unwrap() {
                Commands::Add { name, src, target } => {
                    let state = kubo_manager::KuboManager::new().lock();
                    let res = kubo_config::add_dotfile(state, name, src, target);
                    if res.is_ok() {
                        println!("Added {} ({} -> {})", name, target, src.to_str().unwrap());
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
    }
}
