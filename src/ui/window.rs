slint::include_modules!();

use crate::fs::browser::{self, SortField, SortDirection};
use crate::search;
use crate::config;
use crate::viewers::{self, FileType};
use std::cell::RefCell;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Navigation state for back/forward history
struct NavState {
    history: Vec<PathBuf>,
    current_index: usize,
}

impl NavState {
    fn new(initial: PathBuf) -> Self {
        Self {
            history: vec![initial],
            current_index: 0,
        }
    }

    fn current(&self) -> &PathBuf {
        &self.history[self.current_index]
    }

    fn navigate(&mut self, path: PathBuf) {
        // Truncate forward history
        self.history.truncate(self.current_index + 1);
        self.history.push(path);
        self.current_index = self.history.len() - 1;
    }

    fn go_back(&mut self) -> Option<&PathBuf> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(&self.history[self.current_index])
        } else {
            None
        }
    }

    fn go_forward(&mut self) -> Option<&PathBuf> {
        if self.current_index < self.history.len() - 1 {
            self.current_index += 1;
            Some(&self.history[self.current_index])
        } else {
            None
        }
    }

    fn can_go_back(&self) -> bool {
        self.current_index > 0
    }

    fn can_go_forward(&self) -> bool {
        self.current_index < self.history.len() - 1
    }
}

/// Convert FileEntry to Slint FileEntry (with selection state)
fn to_slint_entry(entry: &browser::FileEntry, selected: bool) -> FileEntry {
    FileEntry {
        name: entry.name.clone().into(),
        path: entry.path.to_string_lossy().to_string().into(),
        is_dir: entry.is_dir,
        size: entry.size_display().into(),
        modified: entry.modified_display().into(),
        icon: entry.icon().into(),
        selected,
    }
}

/// Build default sidebar items
fn default_sidebar_items() -> Vec<SidebarItem> {
    let home = browser::home_dir();
    vec![
        SidebarItem {
            name: "Home".into(),
            icon: "🏠".into(),
            path: home.to_string_lossy().to_string().into(),
            section: "favorites".into(),
        },
        SidebarItem {
            name: "Desktop".into(),
            icon: "🖥️".into(),
            path: home.join("Desktop").to_string_lossy().to_string().into(),
            section: "favorites".into(),
        },
        SidebarItem {
            name: "Documents".into(),
            icon: "📁".into(),
            path: home.join("Documents").to_string_lossy().to_string().into(),
            section: "favorites".into(),
        },
        SidebarItem {
            name: "Downloads".into(),
            icon: "⬇️".into(),
            path: home.join("Downloads").to_string_lossy().to_string().into(),
            section: "favorites".into(),
        },
        SidebarItem {
            name: "Pictures".into(),
            icon: "🖼️".into(),
            path: home.join("Pictures").to_string_lossy().to_string().into(),
            section: "favorites".into(),
        },
        SidebarItem {
            name: "Music".into(),
            icon: "🎵".into(),
            path: home.join("Music").to_string_lossy().to_string().into(),
            section: "favorites".into(),
        },
        SidebarItem {
            name: "Videos".into(),
            icon: "🎬".into(),
            path: home.join("Videos").to_string_lossy().to_string().into(),
            section: "favorites".into(),
        },
        SidebarItem {
            name: "Root".into(),
            icon: "💻".into(),
            path: "/".into(),
            section: "devices".into(),
        },
    ]
}

fn hex_to_color(hex: &str) -> Option<slint::Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(slint::Color::from_rgb_u8(r, g, b))
    } else {
        None
    }
}

fn apply_theme_colors(window: &MainWindow, theme_colors: &config::ThemeColors) {
    let tokens = Tokens::get(window);
    if let Some(color) = &theme_colors.bg_window {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_bg_window(c);
            tokens.set_dark_bg_window(c);
        }
    }
    if let Some(color) = &theme_colors.bg_surface {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_bg_surface(c);
            tokens.set_dark_bg_surface(c);
        }
    }
    if let Some(color) = &theme_colors.bg_sidebar {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_bg_sidebar(c);
            tokens.set_dark_bg_sidebar(c);
        }
    }
    if let Some(color) = &theme_colors.bg_toolbar {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_bg_toolbar(c);
            tokens.set_dark_bg_toolbar(c);
        }
    }
    if let Some(color) = &theme_colors.bg_header {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_bg_header(c);
            tokens.set_dark_bg_header(c);
        }
    }
    if let Some(color) = &theme_colors.bg_status {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_bg_status(c);
            tokens.set_dark_bg_status(c);
        }
    }
    if let Some(color) = &theme_colors.bg_row_alt {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_bg_row_alt(c);
            tokens.set_dark_bg_row_alt(c);
        }
    }
    if let Some(color) = &theme_colors.bg_hover {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_bg_hover(c);
            tokens.set_dark_bg_hover(c);
        }
    }
    if let Some(color) = &theme_colors.bg_input {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_bg_input(c);
            tokens.set_dark_bg_input(c);
        }
    }
    if let Some(color) = &theme_colors.text_primary {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_text_primary(c);
            tokens.set_dark_text_primary(c);
        }
    }
    if let Some(color) = &theme_colors.text_secondary {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_text_secondary(c);
            tokens.set_dark_text_secondary(c);
        }
    }
    if let Some(color) = &theme_colors.text_tertiary {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_text_tertiary(c);
            tokens.set_dark_text_tertiary(c);
        }
    }
    if let Some(color) = &theme_colors.border {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_border(c);
            tokens.set_dark_border(c);
        }
    }
    if let Some(color) = &theme_colors.border_subtle {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_border_subtle(c);
            tokens.set_dark_border_subtle(c);
        }
    }
    if let Some(color) = &theme_colors.bg_tab_active {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_bg_tab_active(c);
            tokens.set_dark_bg_tab_active(c);
        }
    }
    if let Some(color) = &theme_colors.bg_tab_inactive {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_bg_tab_inactive(c);
            tokens.set_dark_bg_tab_inactive(c);
        }
    }
    if let Some(color) = &theme_colors.text_tab_active {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_text_tab_active(c);
            tokens.set_dark_text_tab_active(c);
        }
    }
    if let Some(color) = &theme_colors.text_tab_inactive {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_text_tab_inactive(c);
            tokens.set_dark_text_tab_inactive(c);
        }
    }
    if let Some(color) = &theme_colors.border_column {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_border_column(c);
            tokens.set_dark_border_column(c);
        }
    }
    if let Some(color) = &theme_colors.bg_column_active {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_bg_column_active(c);
            tokens.set_dark_bg_column_active(c);
        }
    }
    if let Some(color) = &theme_colors.bg_column_inactive {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_bg_column_inactive(c);
            tokens.set_dark_bg_column_inactive(c);
        }
    }
    if let Some(color) = &theme_colors.bg_inspector {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_bg_inspector(c);
            tokens.set_dark_bg_inspector(c);
        }
    }
    if let Some(color) = &theme_colors.border_inspector {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_border_inspector(c);
            tokens.set_dark_border_inspector(c);
        }
    }
    if let Some(color) = &theme_colors.shadow_inspector {
        if let Some(c) = hex_to_color(color) {
            tokens.set_light_shadow_inspector(c);
            tokens.set_dark_shadow_inspector(c);

        }
    }
}

fn apply_theme_file(window: &MainWindow, theme: &config::Theme) {
    let tokens = Tokens::get(window);
    
    let light = &theme.light;
    let dark = &theme.dark;
    
    if let Some(color) = &light.bg_window {
        if let Some(c) = hex_to_color(color) { tokens.set_light_bg_window(c); }
    }
    if let Some(color) = &dark.bg_window {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_bg_window(c); }
    }
    if let Some(color) = &light.bg_surface {
        if let Some(c) = hex_to_color(color) { tokens.set_light_bg_surface(c); }
    }
    if let Some(color) = &dark.bg_surface {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_bg_surface(c); }
    }
    if let Some(color) = &light.bg_sidebar {
        if let Some(c) = hex_to_color(color) { tokens.set_light_bg_sidebar(c); }
    }
    if let Some(color) = &dark.bg_sidebar {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_bg_sidebar(c); }
    }
    if let Some(color) = &light.bg_toolbar {
        if let Some(c) = hex_to_color(color) { tokens.set_light_bg_toolbar(c); }
    }
    if let Some(color) = &dark.bg_toolbar {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_bg_toolbar(c); }
    }
    if let Some(color) = &light.bg_header {
        if let Some(c) = hex_to_color(color) { tokens.set_light_bg_header(c); }
    }
    if let Some(color) = &dark.bg_header {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_bg_header(c); }
    }
    if let Some(color) = &light.bg_status {
        if let Some(c) = hex_to_color(color) { tokens.set_light_bg_status(c); }
    }
    if let Some(color) = &dark.bg_status {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_bg_status(c); }
    }
    if let Some(color) = &light.bg_row_alt {
        if let Some(c) = hex_to_color(color) { tokens.set_light_bg_row_alt(c); }
    }
    if let Some(color) = &dark.bg_row_alt {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_bg_row_alt(c); }
    }
    if let Some(color) = &light.bg_hover {
        if let Some(c) = hex_to_color(color) { tokens.set_light_bg_hover(c); }
    }
    if let Some(color) = &dark.bg_hover {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_bg_hover(c); }
    }
    if let Some(color) = &light.bg_selected {
        if let Some(c) = hex_to_color(color) { tokens.set_light_bg_selected(c); }
    }
    if let Some(color) = &dark.bg_selected {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_bg_selected(c); }
    }
    if let Some(color) = &light.bg_input {
        if let Some(c) = hex_to_color(color) { tokens.set_light_bg_input(c); }
    }
    if let Some(color) = &dark.bg_input {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_bg_input(c); }
    }
    if let Some(color) = &light.text_primary {
        if let Some(c) = hex_to_color(color) { tokens.set_light_text_primary(c); }
    }
    if let Some(color) = &dark.text_primary {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_text_primary(c); }
    }
    if let Some(color) = &light.text_secondary {
        if let Some(c) = hex_to_color(color) { tokens.set_light_text_secondary(c); }
    }
    if let Some(color) = &dark.text_secondary {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_text_secondary(c); }
    }
    if let Some(color) = &light.text_tertiary {
        if let Some(c) = hex_to_color(color) { tokens.set_light_text_tertiary(c); }
    }
    if let Some(color) = &dark.text_tertiary {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_text_tertiary(c); }
    }
    if let Some(color) = &light.text_on_primary {
        if let Some(c) = hex_to_color(color) { tokens.set_light_text_on_primary(c); }
    }
    if let Some(color) = &dark.text_on_primary {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_text_on_primary(c); }
    }
    if let Some(color) = &light.border {
        if let Some(c) = hex_to_color(color) { tokens.set_light_border(c); }
    }
    if let Some(color) = &dark.border {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_border(c); }
    }
    if let Some(color) = &light.border_subtle {
        if let Some(c) = hex_to_color(color) { tokens.set_light_border_subtle(c); }
    }
    if let Some(color) = &dark.border_subtle {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_border_subtle(c); }
    }
    if let Some(color) = &light.shadow {
        if let Some(c) = hex_to_color(color) { tokens.set_light_shadow(c); }
    }
    if let Some(color) = &dark.shadow {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_shadow(c); }
    }
    if let Some(color) = &light.bg_tab_active {
        if let Some(c) = hex_to_color(color) { tokens.set_light_bg_tab_active(c); }
    }
    if let Some(color) = &dark.bg_tab_active {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_bg_tab_active(c); }
    }
    if let Some(color) = &light.bg_tab_inactive {
        if let Some(c) = hex_to_color(color) { tokens.set_light_bg_tab_inactive(c); }
    }
    if let Some(color) = &dark.bg_tab_inactive {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_bg_tab_inactive(c); }
    }
    if let Some(color) = &light.text_tab_active {
        if let Some(c) = hex_to_color(color) { tokens.set_light_text_tab_active(c); }
    }
    if let Some(color) = &dark.text_tab_active {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_text_tab_active(c); }
    }
    if let Some(color) = &light.text_tab_inactive {
        if let Some(c) = hex_to_color(color) { tokens.set_light_text_tab_inactive(c); }
    }
    if let Some(color) = &dark.text_tab_inactive {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_text_tab_inactive(c); }
    }
    if let Some(color) = &light.border_column {
        if let Some(c) = hex_to_color(color) { tokens.set_light_border_column(c); }
    }
    if let Some(color) = &dark.border_column {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_border_column(c); }
    }
    if let Some(color) = &light.bg_column_active {
        if let Some(c) = hex_to_color(color) { tokens.set_light_bg_column_active(c); }
    }
    if let Some(color) = &dark.bg_column_active {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_bg_column_active(c); }
    }
    if let Some(color) = &light.bg_column_inactive {
        if let Some(c) = hex_to_color(color) { tokens.set_light_bg_column_inactive(c); }
    }
    if let Some(color) = &dark.bg_column_inactive {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_bg_column_inactive(c); }
    }
    if let Some(color) = &light.bg_inspector {
        if let Some(c) = hex_to_color(color) { tokens.set_light_bg_inspector(c); }
    }
    if let Some(color) = &dark.bg_inspector {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_bg_inspector(c); }
    }
    if let Some(color) = &light.border_inspector {
        if let Some(c) = hex_to_color(color) { tokens.set_light_border_inspector(c); }
    }
    if let Some(color) = &dark.border_inspector {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_border_inspector(c); }
    }
    if let Some(color) = &light.shadow_inspector {
        if let Some(c) = hex_to_color(color) { tokens.set_light_shadow_inspector(c); }
    }
    if let Some(color) = &dark.shadow_inspector {
        if let Some(c) = hex_to_color(color) { tokens.set_dark_shadow_inspector(c); }
    }
}

/// Launch the main application window
pub fn launch() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;

    let initial_cfg = config::load().unwrap_or_else(|_| {
        config::Config {
            general: config::GeneralConfig {
                show_hidden: false,
                confirm_delete: true,
                page_size: 100,
            },
            theme: config::ThemeConfig {
                mode: config::ThemeMode::System,
                accent_color: "#58a6ff".to_string(),
                light_colors: config::ThemeColors::default(),
                dark_colors: config::ThemeColors::default(),
            },
            tools: config::ToolsConfig {
                enabled: vec!["markdown".to_string(), "pdf".to_string()],
                markdown_preview: true,
                pdf_preview: true,
                docx_preview: false,
                pptx_preview: false,
            },
            sidebar: config::SidebarConfig {
                favorites: Vec::new(),
                show_devices: true,
                show_bookmarks: true,
            },
            ui: config::UiConfig::default(),
            viewers: config::ViewerConfig::default(),
        }
    });

    // Navigation state
    let nav = Rc::new(RefCell::new(NavState::new(browser::home_dir())));
    let files_cache: Rc<RefCell<Vec<browser::FileEntry>>> = Rc::new(RefCell::new(Vec::new()));
    
    // Sorting state: track current sort field and direction
    let sort_state: Rc<RefCell<(SortField, SortDirection)>> = 
        Rc::new(RefCell::new((SortField::Name, SortDirection::Ascending)));
    
    // Selection state: track selected indices and anchor for shift-selection
    let selected_indices: Rc<RefCell<std::collections::HashSet<usize>>> = 
        Rc::new(RefCell::new(std::collections::HashSet::new()));
    let anchor_index: Rc<RefCell<Option<usize>>> = 
        Rc::new(RefCell::new(None));

    // Current config (shared, mutable)
    let sidebar_init_width = initial_cfg.ui.sidebar_width;
    let sidebar_init_collapsed = initial_cfg.ui.sidebar_collapsed;
    let current_cfg: Rc<RefCell<config::Config>> = Rc::new(RefCell::new(initial_cfg));

    let theme_mode = current_cfg.borrow().theme.mode.clone();
    let colors_to_apply = match theme_mode {
        config::ThemeMode::Light => current_cfg.borrow().theme.light_colors.clone(),
        config::ThemeMode::Dark => current_cfg.borrow().theme.dark_colors.clone(),
        config::ThemeMode::System => {
            #[cfg(target_os = "macos")]
            {
                use std::process::Command;
                if let Ok(output) = Command::new("defaults").args(["read", "-g", "AppleInterfaceStyle"]).output() {
                    if output.stdout.trim().to_lowercase() == "dark" {
                        current_cfg.borrow().theme.dark_colors.clone()
                    } else {
                        current_cfg.borrow().theme.light_colors.clone()
                    }
                } else {
                    current_cfg.borrow().theme.light_colors.clone()
                }
            }
            #[cfg(not(target_os = "macos"))]
            {
                current_cfg.borrow().theme.light_colors.clone()
            }
        }
    };
    apply_theme_colors(&window, &colors_to_apply);
    
    let theme_file = config::load_theme();
    if theme_file.name.is_some() || !theme_file.light.bg_window.is_none() || !theme_file.brand.primary.is_none() {
        log::info!("Loading custom theme from file");
        apply_theme_file(&window, &theme_file);
    }

    // Initial load
    let initial_path = browser::home_dir();
    load_directory(&window, &nav, &files_cache, &selected_indices, &anchor_index, &initial_path, &current_cfg);

    // Set sidebar items
    let sidebar: slint::ModelRc<SidebarItem> =
        Rc::new(slint::VecModel::from(default_sidebar_items())).into();
    window.set_sidebar_items(sidebar);

    window.set_sidebar_width(sidebar_init_width as f32);
    window.set_sidebar_collapsed(sidebar_init_collapsed);

    let tabs_data = Rc::new(RefCell::new(vec![
        TabItem {
            id: "1".into(),
            title: "Home".into(),
            path: browser::home_dir().to_string_lossy().to_string().into(),
        }
    ]));
    let active_tab_index = Rc::new(RefCell::new(0));

    let tabs_vec = tabs_data.borrow();
    let tabs_model: slint::ModelRc<TabItem> =
        Rc::new(slint::VecModel::from(tabs_vec.clone())).into();
    window.set_tabs(tabs_model);
    window.set_active_tab_index(0);

    // ─── Callbacks ─────────────────────────────────────────────────────

    let w = window.as_weak();
    let nav_ref = nav.clone();
    let files_ref = files_cache.clone();
    let sel_back = selected_indices.clone();
    let anc_back = anchor_index.clone();
    let cfg_back = current_cfg.clone();
    window.on_go_back(move || {
        let window = w.unwrap();
        let mut nav = nav_ref.borrow_mut();
        if let Some(path) = nav.go_back() {
            let path = path.clone();
            drop(nav);
            load_directory(&window, &nav_ref, &files_ref, &sel_back, &anc_back, &path, &cfg_back);
        }
    });

    let w = window.as_weak();
    window.on_details_panel_toggle(move || {
        let window = w.unwrap();
        window.set_show_details_panel(!window.get_show_details_panel());
    });

    let w = window.as_weak();
    let nav_ref = nav.clone();
    let files_for_search = files_cache.clone();
    window.on_search_changed(move |query| {
        let window = w.unwrap();
        let query = query.to_string();

        if query.is_empty() {
            // Restore cached directory listing
            let slint_files: Vec<FileEntry> =
                files_for_search.borrow().iter().map(|e| to_slint_entry(e, false)).collect();
            let model: slint::ModelRc<FileEntry> =
                Rc::new(slint::VecModel::from(slint_files)).into();
            window.set_files(model);
            window.set_status_message("".into());
        } else {
            // Recursive filename search from current directory
            let current = nav_ref.borrow().current().clone();
            match search::filename::search_filenames(&current, &query, 200) {
                Ok(paths) => {
                    let slint_files: Vec<FileEntry> = paths
                        .iter()
                        .filter_map(|p| browser::FileEntry::from_path(p))
                        .map(|e| to_slint_entry(&e, false))
                        .collect();
                    let count = slint_files.len();
                    let model: slint::ModelRc<FileEntry> =
                        Rc::new(slint::VecModel::from(slint_files)).into();
                    window.set_files(model);
                    window.set_status_message(
                        format!(
                            "{} result{} for \"{}\"",
                            count,
                            if count == 1 { "" } else { "s" },
                            query
                        )
                        .into(),
                    );
                }
                Err(e) => {
                    log::error!("Search error: {}", e);
                    window.set_status_message(format!("Search error: {}", e).into());
                }
            }
        }
    });

    let sort_ref = sort_state.clone();
    let files_for_sort = files_cache.clone();
    let w = window.as_weak();
    window.on_sort_requested(move |column| {
        let window = w.unwrap();
        let mut sort = sort_ref.borrow_mut();
        
        // Determine the field to sort by
        let field = match column.as_str() {
            "name" => SortField::Name,
            "size" => SortField::Size,
            "modified" => SortField::Modified,
            _ => return,
        };
        
        // Toggle direction if same field, otherwise default to ascending
        if sort.0 == field {
            sort.1 = if sort.1 == SortDirection::Ascending {
                SortDirection::Descending
            } else {
                SortDirection::Ascending
            };
        } else {
            sort.0 = field;
            sort.1 = SortDirection::Ascending;
        }
        
        // Sort the cached entries
        let mut files = files_for_sort.borrow_mut();
        browser::sort_entries(&mut files, sort.0, sort.1);
        
        // Update UI
        let slint_files: Vec<FileEntry> = files.iter().map(|e| to_slint_entry(e, false)).collect();
        let model: slint::ModelRc<FileEntry> =
            Rc::new(slint::VecModel::from(slint_files)).into();
        window.set_files(model);
        
        // Show sort indicator in status
        let direction = if sort.1 == SortDirection::Ascending { "↑" } else { "↓" };
        let field_name = match sort.0 {
            SortField::Name => "name",
            SortField::Size => "size",
            SortField::Modified => "modified",
        };
        window.set_status_message(format!("Sorted by {} {}", field_name, direction).into());
    });

    // ─── Shortcuts dialog ───────────────────────────────────────────────────
    let w_shortcuts = window.as_weak();
    window.on_shortcuts_clicked(move || {
        let window = w_shortcuts.unwrap();
        
        match ShortcutsDialog::new() {
            Ok(dialog) => {
                let w4: slint::Weak<MainWindow> = window.as_weak();
                dialog.on_close(move || {
                    if let Some(w) = w4.upgrade() {
                        w.set_status_message("".into());
                    }
                });
                
                let _ = dialog.show();
            }
            Err(e) => {
                log::error!("Failed to create shortcuts dialog: {}", e);
                window.set_status_message("Error: Cannot open shortcuts".into());
            }
        }
    });

    let cfg_for_settings = current_cfg.clone();
    let nav_for_settings = nav.clone();
    let files_for_settings = files_cache.clone();
    let sel_for_settings = selected_indices.clone();
    let anc_for_settings = anchor_index.clone();
    // ─── Settings dialog ───────────────────────────────────────────────────
    let w = window.as_weak();
    window.on_settings_clicked(move || {
        let window = w.unwrap();

        match config::load() {
            Ok(cfg) => {
                match SettingsDialog::new() {
                    Ok(dialog) => {
                        // Populate form with current config
                        dialog.set_show_hidden(cfg.general.show_hidden);
                        dialog.set_confirm_delete(cfg.general.confirm_delete);
                        dialog.set_page_size(cfg.general.page_size as i32);

                        // Theme mode: 0=System, 1=Light, 2=Dark
                        let theme_mode = match cfg.theme.mode {
                            config::ThemeMode::System => 0,
                            config::ThemeMode::Light => 1,
                            config::ThemeMode::Dark => 2,
                        };
                        dialog.set_theme_mode(theme_mode);
                        dialog.set_accent_color(cfg.theme.accent_color.clone().into());

                        // Tools
                        dialog.set_markdown_preview(cfg.tools.markdown_preview);
                        dialog.set_pdf_preview(cfg.tools.pdf_preview);
                        dialog.set_docx_preview(cfg.tools.docx_preview);
                        dialog.set_pptx_preview(cfg.tools.pptx_preview);

                        // Sidebar
                        dialog.set_show_devices(cfg.sidebar.show_devices);
                        dialog.set_show_bookmarks(cfg.sidebar.show_bookmarks);

                        // Viewers
                        dialog.set_image_viewer(cfg.viewers.image_viewer.clone().into());
                        dialog.set_video_viewer(cfg.viewers.video_viewer.clone().into());
                        dialog.set_pdf_viewer(cfg.viewers.pdf_viewer.clone().into());

                        // Save callback
                        let w2: slint::Weak<MainWindow> = window.as_weak();
                        let dialog_weak = dialog.as_weak();
                        let cfg_ref = cfg_for_settings.clone();
                        let nav_ref = nav_for_settings.clone();
                        let files_ref = files_for_settings.clone();
                        let sel_ref = sel_for_settings.clone();
                        let anc_ref = anc_for_settings.clone();
                        dialog.on_save_config(move || {
                            let dialog = dialog_weak.upgrade().unwrap();
                            let new_cfg = config::Config {
                                general: config::GeneralConfig {
                                    show_hidden: dialog.get_show_hidden(),
                                    confirm_delete: dialog.get_confirm_delete(),
                                    page_size: dialog.get_page_size() as usize,
                                },
                                theme: config::ThemeConfig {
                                    mode: match dialog.get_theme_mode() {
                                        0 => config::ThemeMode::System,
                                        1 => config::ThemeMode::Light,
                                        _ => config::ThemeMode::Dark,
                                    },
                                    accent_color: dialog.get_accent_color().to_string(),
                                    light_colors: cfg_ref.borrow().theme.light_colors.clone(),
                                    dark_colors: cfg_ref.borrow().theme.dark_colors.clone(),
                                },
                                tools: config::ToolsConfig {
                                    enabled: {
                                        let mut tools = Vec::new();
                                        if dialog.get_markdown_preview() { tools.push("markdown".to_string()); }
                                        if dialog.get_pdf_preview() { tools.push("pdf".to_string()); }
                                        if dialog.get_docx_preview() { tools.push("docx".to_string()); }
                                        if dialog.get_pptx_preview() { tools.push("pptx".to_string()); }
                                        tools
                                    },
                                    markdown_preview: dialog.get_markdown_preview(),
                                    pdf_preview: dialog.get_pdf_preview(),
                                    docx_preview: dialog.get_docx_preview(),
                                    pptx_preview: dialog.get_pptx_preview(),
                                },
                                sidebar: config::SidebarConfig {
                                    favorites: cfg_ref.borrow().sidebar.favorites.clone(),
                                    show_devices: dialog.get_show_devices(),
                                    show_bookmarks: dialog.get_show_bookmarks(),
                                },
                                ui: {
                                    let w_ui = w2.upgrade();
                                    let sidebar_w = w_ui.as_ref().map(|w| w.get_sidebar_width() as u32).unwrap_or(220);
                                    let sidebar_coll = w_ui.as_ref().map(|w| w.get_sidebar_collapsed()).unwrap_or(false);
                                    config::UiConfig {
                                        default_path: cfg_ref.borrow().ui.default_path.clone(),
                                        window_width: cfg_ref.borrow().ui.window_width,
                                        window_height: cfg_ref.borrow().ui.window_height,
                                        sidebar_width: sidebar_w,
                                        sidebar_collapsed: sidebar_coll,
                                    }
                                },
                                viewers: config::ViewerConfig {
                                    image_viewer: dialog.get_image_viewer().to_string(),
                                    video_viewer: dialog.get_video_viewer().to_string(),
                                    pdf_viewer: dialog.get_pdf_viewer().to_string(),
                                },
                            };

                            if let Err(e) = config::save(&new_cfg) {
                                log::error!("Failed to save config: {}", e);
                                if let Some(w) = w2.upgrade() {
                                    w.set_status_message(format!("Error saving config: {}", e).into());
                                }
                            } else {
                                *cfg_ref.borrow_mut() = new_cfg.clone();
                                
                                let colors_to_apply = match new_cfg.theme.mode {
                                    config::ThemeMode::Light => new_cfg.theme.light_colors.clone(),
                                    config::ThemeMode::Dark => new_cfg.theme.dark_colors.clone(),
                                    config::ThemeMode::System => {
                                        #[cfg(target_os = "macos")]
                                        {
                                            use std::process::Command;
                                            if let Ok(output) = Command::new("defaults").args(["read", "-g", "AppleInterfaceStyle"]).output() {
                                                if output.stdout.trim().to_lowercase() == "dark" {
                                                    new_cfg.theme.dark_colors.clone()
                                                } else {
                                                    new_cfg.theme.light_colors.clone()
                                                }
                                            } else {
                                                new_cfg.theme.light_colors.clone()
                                            }
                                        }
                                        #[cfg(not(target_os = "macos"))]
                                        {
                                            new_cfg.theme.light_colors.clone()
                                        }
                                    }
                                };
                                if let Some(w) = w2.upgrade() {
                                    apply_theme_colors(&w, &colors_to_apply);
                                    let tokens = Tokens::get(&w);
                                    let theme_mode_value = match new_cfg.theme.mode {
                                        config::ThemeMode::System => 0,
                                        config::ThemeMode::Light => 1,
                                        config::ThemeMode::Dark => 2,
                                    };
                                    tokens.set_theme_mode(theme_mode_value);
                                }
                                
                                let current_path = nav_ref.borrow().current().clone();
                                load_directory(
                                    &w2.upgrade().unwrap(),
                                    &nav_ref,
                                    &files_ref,
                                    &sel_ref,
                                    &anc_ref,
                                    &current_path,
                                    &cfg_ref,
                                );
                                
                                let show_devices = new_cfg.sidebar.show_devices;
                                let show_bookmarks = new_cfg.sidebar.show_bookmarks;
                                let mut items = default_sidebar_items();
                                items.retain(|item| {
                                    if item.section == "devices" && !show_devices {
                                        return false;
                                    }
                                    if item.section == "bookmarks" && !show_bookmarks {
                                        return false;
                                    }
                                    true
                                });
                                let sidebar: slint::ModelRc<SidebarItem> =
                                    Rc::new(slint::VecModel::from(items)).into();
                                if let Some(w) = w2.upgrade() {
                                    w.set_sidebar_items(sidebar);
                                    w.set_status_message("Settings saved!".into());
                                }
                                
                                let _ = dialog.hide();
                            }
                        });

                        dialog.on_close_dialog(move || {
                            if let Some(w) = window.as_weak().upgrade() {
                                w.set_status_message("".into());
                            }
                        });

                        let _ = dialog.show();
                    }
                    Err(e) => {
                        log::error!("Failed to create settings dialog: {}", e);
                        window.set_status_message("Error: Cannot open settings".into());
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to load config: {}", e);
                window.set_status_message(format!("Error loading config: {}", e).into());
            }
        }
    });

    let window_tabs = window.as_weak();
    let tabs_for_click = tabs_data.clone();
    let active_for_click = active_tab_index.clone();
    let nav_for_click = nav.clone();
    let files_for_click = files_cache.clone();
    let sel_for_click = selected_indices.clone();
    let anc_for_click = anchor_index.clone();
    let cfg_for_click = current_cfg.clone();
    window.on_tab_clicked(move |index| {
        let window = window_tabs.unwrap();
        let index = index as i32;
        *active_for_click.borrow_mut() = index;
        window.set_active_tab_index(index);
        let tabs = tabs_for_click.borrow();
        if let Some(tab) = tabs.get(index as usize) {
            let path = PathBuf::from(tab.path.as_str());
            let mut nav = nav_for_click.borrow_mut();
            nav.navigate(path.clone());
            drop(nav);
            load_directory(&window, &nav_for_click, &files_for_click, &sel_for_click, &anc_for_click, &path, &cfg_for_click);
        }
    });

    let window_close = window.as_weak();
    let active_close = active_tab_index.clone();
    let tabs_close = tabs_data.clone();
    let nav_close = nav.clone();
    let files_close = files_cache.clone();
    let sel_close = selected_indices.clone();
    let anc_close = anchor_index.clone();
    let cfg_close = current_cfg.clone();
    window.on_tab_closed(move |index| {
        let index = index as usize;
        {
            let mut tabs = tabs_close.borrow_mut();
            if tabs.len() > 1 {
                tabs.remove(index);
            }
        }
        let was_active = *active_close.borrow() as usize;
        let new_active = if was_active >= index && was_active > 0 {
            was_active - 1
        } else {
            was_active
        };
        *active_close.borrow_mut() = new_active as i32;
        if let Some(w) = window_close.upgrade() {
            w.set_active_tab_index(new_active as i32);
            let tabs = tabs_close.borrow();
            let new_model: slint::ModelRc<TabItem> =
                Rc::new(slint::VecModel::from(tabs.clone())).into();
            w.set_tabs(new_model);
            if let Some(tab) = tabs.get(new_active) {
                let path = PathBuf::from(tab.path.as_str());
                drop(tabs);
                let mut nav = nav_close.borrow_mut();
                nav.navigate(path.clone());
                drop(nav);
                load_directory(&w, &nav_close, &files_close, &sel_close, &anc_close, &path, &cfg_close);
            }
        }
    });

    let window_new = window.as_weak();
    let active_new = active_tab_index.clone();
    let tabs_new = tabs_data.clone();
    let nav_new = nav.clone();
    let files_new = files_cache.clone();
    let sel_new = selected_indices.clone();
    let anc_new = anchor_index.clone();
    let cfg_new = current_cfg.clone();
    window.on_new_tab_clicked(move || {
        let home = browser::home_dir();
        let new_tab = TabItem {
            id: format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()).into(),
            title: "Home".into(),
            path: home.to_string_lossy().to_string().into(),
        };
        let new_index = {
            let tabs = tabs_new.borrow();
            tabs.len()
        };
        tabs_new.borrow_mut().push(new_tab);
        *active_new.borrow_mut() = new_index as i32;
        if let Some(w) = window_new.upgrade() {
            w.set_active_tab_index(new_index as i32);
            let tabs = tabs_new.borrow();
            let new_model: slint::ModelRc<TabItem> =
                Rc::new(slint::VecModel::from(tabs.clone())).into();
            w.set_tabs(new_model);
            drop(tabs);
            let mut nav = nav_new.borrow_mut();
            nav.navigate(home.clone());
            drop(nav);
            load_directory(&w, &nav_new, &files_new, &sel_new, &anc_new, &home, &cfg_new);
        }
    });

    let w = window.as_weak();
    let files_for_inspector = files_cache.clone();
    window.on_file_selected(move |index, _ctrl, _shift| {
        let window = w.unwrap();
        let index = index as usize;
        let files = files_for_inspector.borrow();
        
        if let Some(entry) = files.get(index) {
            let data = InspectorData {
                file_name: entry.name.clone().into(),
                file_path: entry.path.to_string_lossy().to_string().into(),
                file_size: entry.size_display().into(),
                file_modified: entry.modified_display().into(),
                file_created: entry.created_display().into(),
                file_permissions: entry.permissions_display().into(),
                is_dir: entry.is_dir,
            };
            window.set_inspector_data(data);
            window.set_inspector_visible(true);
        }
    });

    window.run()
}

/// Load a directory and update the UI
fn load_directory(
    window: &MainWindow,
    nav: &Rc<RefCell<NavState>>,
    files_cache: &Rc<RefCell<Vec<browser::FileEntry>>>,
    selected_indices: &Rc<RefCell<HashSet<usize>>>,
    anchor_index: &Rc<RefCell<Option<usize>>>,
    path: &Path,
    config: &Rc<RefCell<config::Config>>,
) {
    let show_hidden = config.borrow().general.show_hidden;
    match browser::list_directory(path, show_hidden) {
        Ok(entries) => {
            // Update navigation
            nav.borrow_mut().navigate(path.to_path_buf());

            // Update cache
            *files_cache.borrow_mut() = entries.clone();

            // Clear selection on directory change
            selected_indices.borrow_mut().clear();
            *anchor_index.borrow_mut() = None;

            // Convert to Slint models
            let slint_files: Vec<FileEntry> = entries.iter().map(|e| to_slint_entry(e, false)).collect();
            let model: slint::ModelRc<FileEntry> =
                Rc::new(slint::VecModel::from(slint_files)).into();

            // Update UI
            window.set_files(model);
            window.set_current_path(path.to_string_lossy().to_string().into());
            window.set_can_go_back(nav.borrow().can_go_back());
            window.set_can_go_forward(nav.borrow().can_go_forward());
            window.set_sidebar_active_path(path.to_string_lossy().to_string().into());
            window.set_selected_indices(
                Rc::new(slint::VecModel::from(vec![])).into()
            );
            window.set_search_text("".into());
        }
        Err(e) => {
            log::error!("Failed to list directory {}: {}", path.display(), e);
            window.set_status_message(format!("Error: {}", e).into());
        }
    }
}
