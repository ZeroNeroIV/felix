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

    // Load or create config
    let config = match config::load() {
        Ok(cfg) => {
            log::info!("Config loaded from {}", config::config_path().display());
            cfg
        }
        Err(e) => {
            log::warn!("Failed to load config: {}. Using defaults.", e);
            // Save default config for next time
            let default = config::Config {
                theme: config::ThemeConfig {
                    mode: config::ThemeMode::System,
                },
                tools: config::ToolsConfig {
                    enabled: vec!["markdown".into(), "pdf".into()],
                },
                sidebar: config::SidebarConfig {
                    favorites: Vec::new(),
                    show_devices: true,
                },
            };
            if let Err(e) = config::save(&default) {
                log::error!("Failed to save default config: {}", e);
            }
            default
        }
    };

    log::info!("Enabled tools: {:?}", config.tools.enabled);
    log::info!("Theme mode: {:?}", config.theme.mode);

    // Launch the UI
    log::info!("Launching UI...");
    if let Err(e) = ui::window::launch() {
        log::error!("UI error: {}", e);
        std::process::exit(1);
    }

    log::info!("felix exited.");
}
