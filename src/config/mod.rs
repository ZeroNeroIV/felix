//! Config module — YAML config at ~/.config/felix.config.yml

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub theme: ThemeConfig,
    pub tools: ToolsConfig,
    pub sidebar: SidebarConfig,
    #[serde(default)]
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_true")]
    pub show_hidden: bool,
    #[serde(default = "default_true")]
    pub confirm_delete: bool,
    #[serde(default = "default_page_size")]
    pub page_size: usize,
}

fn default_true() -> bool { true }
fn default_page_size() -> usize { 100 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    #[serde(default = "default_theme_mode")]
    pub mode: ThemeMode,
    #[serde(default = "default_accent_string")]
    pub accent_color: String,
}

fn default_theme_mode() -> ThemeMode { ThemeMode::System }
fn default_accent_string() -> String { "#58a6ff".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsConfig {
    #[serde(default)]
    pub enabled: Vec<String>,
    #[serde(default = "default_true")]
    pub markdown_preview: bool,
    #[serde(default = "default_true")]
    pub pdf_preview: bool,
    #[serde(default)]
    pub docx_preview: bool,
    #[serde(default)]
    pub pptx_preview: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidebarConfig {
    #[serde(default)]
    pub favorites: Vec<String>,
    #[serde(default = "default_true")]
    pub show_devices: bool,
    #[serde(default = "default_true")]
    pub show_bookmarks: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiConfig {
    #[serde(default)]
    pub default_path: Option<String>,
    #[serde(default)]
    pub window_width: Option<u32>,
    #[serde(default)]
    pub window_height: Option<u32>,
}

/// Get config directory (for standard config)
pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("felix")
}

/// Get the standard config path (~/.config/felix/config.toml)
pub fn toml_config_path() -> PathBuf {
    config_dir().join("config.toml")
}

/// Get the user-requested config path (~/.config/felix.config.yml)
pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("felix.config.yml")
}

/// Load config from YAML file
pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
    let path = config_path();
    if !path.exists() {
        return Ok(default_config());
    }
    let content = std::fs::read_to_string(&path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}

/// Save config to YAML file
pub fn save(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let path = config_path();
    let content = serde_yaml::to_string(config)?;
    std::fs::write(path, content)?;
    Ok(())
}

/// Get config as YAML string for editing
pub fn to_yaml(config: &Config) -> Result<String, Box<dyn std::error::Error>> {
    Ok(serde_yaml::to_string(config)?)
}

/// Parse config from YAML string
pub fn from_yaml(yaml: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config: Config = serde_yaml::from_str(yaml)?;
    Ok(config)
}

fn default_config() -> Config {
    Config {
        general: GeneralConfig {
            show_hidden: false,
            confirm_delete: true,
            page_size: 100,
        },
        theme: ThemeConfig {
            mode: ThemeMode::System,
            accent_color: "#58a6ff".to_string(),
        },
        tools: ToolsConfig {
            enabled: vec!["markdown".to_string(), "pdf".to_string()],
            markdown_preview: true,
            pdf_preview: true,
            docx_preview: false,
            pptx_preview: false,
        },
        sidebar: SidebarConfig {
            favorites: Vec::new(),
            show_devices: true,
            show_bookmarks: true,
        },
        ui: UiConfig::default(),
    }
}

/// Initialize config file if it doesn't exist
pub fn init() -> Result<Config, Box<dyn std::error::Error>> {
    let path = config_path();
    if !path.exists() {
        let config = default_config();
        save(&config)?;
        log::info!("Created config at {}", path.display());
        Ok(config)
    } else {
        load()
    }
}
