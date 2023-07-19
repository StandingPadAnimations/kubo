mod kubo_manager;
mod kubo_config;
mod operations;
mod daemon;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let state = kubo_config::read_config(kubo_manager::KuboManager::new()); 
    let state = state.initial_copy();
    if let Err(error) = daemon::daemon(state.watch_paths(), state) {
        log::error!("Error: {error:?}");
    }
}
