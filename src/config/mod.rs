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
    #[serde(default)]
    pub viewers: ViewerConfig,
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
    #[serde(default)]
    pub light_colors: ThemeColors,
    #[serde(default)]
    pub dark_colors: ThemeColors,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeColors {
    #[serde(default)]
    pub bg_window: Option<String>,
    #[serde(default)]
    pub bg_surface: Option<String>,
    #[serde(default)]
    pub bg_sidebar: Option<String>,
    #[serde(default)]
    pub bg_toolbar: Option<String>,
    #[serde(default)]
    pub bg_header: Option<String>,
    #[serde(default)]
    pub bg_status: Option<String>,
    #[serde(default)]
    pub bg_row_alt: Option<String>,
    #[serde(default)]
    pub bg_hover: Option<String>,
    #[serde(default)]
    pub bg_selected: Option<String>,
    #[serde(default)]
    pub bg_input: Option<String>,
    #[serde(default)]
    pub text_primary: Option<String>,
    #[serde(default)]
    pub text_secondary: Option<String>,
    #[serde(default)]
    pub text_tertiary: Option<String>,
    #[serde(default)]
    pub border: Option<String>,
    #[serde(default)]
    pub border_subtle: Option<String>,
    #[serde(default)]
    pub bg_tab_active: Option<String>,
    #[serde(default)]
    pub bg_tab_inactive: Option<String>,
    #[serde(default)]
    pub text_tab_active: Option<String>,
    #[serde(default)]
    pub text_tab_inactive: Option<String>,
    #[serde(default)]
    pub border_column: Option<String>,
    #[serde(default)]
    pub bg_column_active: Option<String>,
    #[serde(default)]
    pub bg_column_inactive: Option<String>,
    #[serde(default)]
    pub bg_inspector: Option<String>,
    #[serde(default)]
    pub border_inspector: Option<String>,
    #[serde(default)]
    pub shadow_inspector: Option<String>,
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
    #[serde(default = "default_sidebar_width")]
    pub sidebar_width: u32,
    #[serde(default)]
    pub sidebar_collapsed: bool,
}

fn default_sidebar_width() -> u32 { 220 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewerConfig {
    #[serde(default = "default_image_viewer")]
    pub image_viewer: String,
    #[serde(default = "default_video_viewer")]
    pub video_viewer: String,
    #[serde(default = "default_pdf_viewer")]
    pub pdf_viewer: String,
}

fn default_image_viewer() -> String { "zathura".to_string() }
fn default_video_viewer() -> String { "zathura".to_string() }
fn default_pdf_viewer() -> String { "zathura".to_string() }

impl Default for ViewerConfig {
    fn default() -> Self {
        Self {
            image_viewer: default_image_viewer(),
            video_viewer: default_video_viewer(),
            pdf_viewer: default_pdf_viewer(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeFileColors {
    #[serde(default)]
    pub bg_window: Option<String>,
    #[serde(default)]
    pub bg_surface: Option<String>,
    #[serde(default)]
    pub bg_sidebar: Option<String>,
    #[serde(default)]
    pub bg_toolbar: Option<String>,
    #[serde(default)]
    pub bg_header: Option<String>,
    #[serde(default)]
    pub bg_status: Option<String>,
    #[serde(default)]
    pub bg_row_alt: Option<String>,
    #[serde(default)]
    pub bg_hover: Option<String>,
    #[serde(default)]
    pub bg_selected: Option<String>,
    #[serde(default)]
    pub bg_input: Option<String>,
    #[serde(default)]
    pub text_primary: Option<String>,
    #[serde(default)]
    pub text_secondary: Option<String>,
    #[serde(default)]
    pub text_tertiary: Option<String>,
    #[serde(default)]
    pub text_on_primary: Option<String>,
    #[serde(default)]
    pub border: Option<String>,
    #[serde(default)]
    pub border_subtle: Option<String>,
    #[serde(default)]
    pub shadow: Option<String>,
    #[serde(default)]
    pub bg_tab_active: Option<String>,
    #[serde(default)]
    pub bg_tab_inactive: Option<String>,
    #[serde(default)]
    pub text_tab_active: Option<String>,
    #[serde(default)]
    pub text_tab_inactive: Option<String>,
    #[serde(default)]
    pub border_column: Option<String>,
    #[serde(default)]
    pub bg_column_active: Option<String>,
    #[serde(default)]
    pub bg_column_inactive: Option<String>,
    #[serde(default)]
    pub bg_inspector: Option<String>,
    #[serde(default)]
    pub border_inspector: Option<String>,
    #[serde(default)]
    pub shadow_inspector: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Typography {
    #[serde(default)]
    pub font_xs: Option<f32>,
    #[serde(default)]
    pub font_sm: Option<f32>,
    #[serde(default)]
    pub font_md: Option<f32>,
    #[serde(default)]
    pub font_lg: Option<f32>,
    #[serde(default)]
    pub font_xl: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BrandColors {
    #[serde(default)]
    pub primary: Option<String>,
    #[serde(default)]
    pub secondary: Option<String>,
    #[serde(default)]
    pub accent: Option<String>,
    #[serde(default)]
    pub primary_soft: Option<String>,
    #[serde(default)]
    pub primary_hover: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Theme {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub light: ThemeFileColors,
    #[serde(default)]
    pub dark: ThemeFileColors,
    #[serde(default)]
    pub brand: BrandColors,
    #[serde(default)]
    pub typography: Typography,
}

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("felix")
}

pub fn theme_toml_path() -> PathBuf {
    config_dir().join("theme.toml")
}

pub fn theme_json_path() -> PathBuf {
    config_dir().join("theme.json")
}

pub fn load_theme_toml() -> Result<Theme, Box<dyn std::error::Error>> {
    let path = theme_toml_path();
    if !path.exists() {
        return Ok(Theme::default());
    }
    let content = std::fs::read_to_string(&path)?;
    let theme: Theme = toml::from_str(&content)?;
    Ok(theme)
}

pub fn load_theme_json() -> Result<Theme, Box<dyn std::error::Error>> {
    let path = theme_json_path();
    if !path.exists() {
        return Ok(Theme::default());
    }
    let content = std::fs::read_to_string(&path)?;
    let theme: Theme = serde_json::from_str(&content)?;
    Ok(theme)
}

pub fn load_theme() -> Theme {
    load_theme_toml().or_else(|_| load_theme_json()).unwrap_or_default()
}

pub fn toml_config_path() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("felix.config.yml")
}

pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
    let path = config_path();
    if !path.exists() {
        return Ok(default_config());
    }
    let content = std::fs::read_to_string(&path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}

pub fn save(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let path = config_path();
    let content = serde_yaml::to_string(config)?;
    std::fs::write(path, content)?;
    Ok(())
}

pub fn to_yaml(config: &Config) -> Result<String, Box<dyn std::error::Error>> {
    Ok(serde_yaml::to_string(config)?)
}

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
            light_colors: ThemeColors::default(),
            dark_colors: ThemeColors::default(),
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
        viewers: ViewerConfig::default(),
    }
}

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
