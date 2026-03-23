mod ui;
mod fs;
mod tools;
mod config;
mod search;
mod archive;

fn main() {
    env_logger::init();
    log::info!("Starting felix v{}", env!("CARGO_PKG_VERSION"));

    // TODO: Load config from ~/.config/felix/config.toml
    // TODO: First-launch setup if no config exists
    // TODO: Initialize Slint UI
    // TODO: Show main window

    println!("felix - file manager coming soon!");
}
