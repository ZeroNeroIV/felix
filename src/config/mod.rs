//! Config module — XDG-compliant config at ~/.config/felix/config.toml

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub theme: ThemeConfig,
    pub tools: ToolsConfig,
    pub sidebar: SidebarConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub mode: ThemeMode,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolsConfig {
    pub enabled: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SidebarConfig {
    pub favorites: Vec<PathBuf>,
    pub show_devices: bool,
}

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("felix")
}

pub fn config_path() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
    let path = config_path();
    if !path.exists() {
        return Ok(default_config());
    }
    let content = std::fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn save(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let dir = config_dir();
    std::fs::create_dir_all(&dir)?;
    let content = toml::to_string_pretty(config)?;
    std::fs::write(config_path(), content)?;
    Ok(())
}

fn default_config() -> Config {
    Config {
        theme: ThemeConfig {
            mode: ThemeMode::System,
        },
        tools: ToolsConfig {
            enabled: vec!["markdown".into(), "pdf".into()],
        },
        sidebar: SidebarConfig {
            favorites: Vec::new(),
            show_devices: true,
        },
    }
}
