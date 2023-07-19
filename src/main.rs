use clap::{Parser, Subcommand};

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
    /// Adds files to myapp
    Add { name: String, src: String, target: String },
    Rm { name: Option<String> },
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
            println!("rm {name:?}")
        }
        Commands::Daemon => {
            // Start the daemon
            let state = kubo_config::read_config(kubo_manager::KuboManager::new()); 
            let state = state.initial_copy();
            if let Err(error) = daemon::daemon(state.watch_paths(), state) {
                log::error!("Error: {error:?}");
            }
        }
    }
}
