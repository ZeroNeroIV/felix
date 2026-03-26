mod archive;
mod config;
mod fs;
mod search;
mod tools;
mod ui;

fn main() {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting felix v{}", env!("CARGO_PKG_VERSION"));

    // Initialize config (creates default if doesn't exist)
    let config = match config::init() {
        Ok(cfg) => {
            log::info!("Config loaded from {}", config::config_path().display());
            cfg
        }
        Err(e) => {
            log::error!("Failed to initialize config: {}", e);
            std::process::exit(1);
        }
    };

    log::info!("Enabled tools: {:?}", config.tools.enabled);
    log::info!("Theme mode: {:?}", config.theme.mode);
    log::info!("Show hidden: {:?}", config.general.show_hidden);

    // Launch the UI
    log::info!("Launching UI...");
    if let Err(e) = ui::window::launch() {
        log::error!("UI error: {}", e);
        std::process::exit(1);
    }

    log::info!("felix exited.");
}
