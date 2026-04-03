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
    let current_cfg: Rc<RefCell<config::Config>> = Rc::new(RefCell::new(initial_cfg));

    // Initial load
    let initial_path = browser::home_dir();
    load_directory(&window, &nav, &files_cache, &selected_indices, &anchor_index, &initial_path, &current_cfg);

    // Set sidebar items
    let sidebar: slint::ModelRc<SidebarItem> =
        Rc::new(slint::VecModel::from(default_sidebar_items())).into();
    window.set_sidebar_items(sidebar);

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
    let nav_ref = nav.clone();
    let files_ref = files_cache.clone();
    let sel_fwd = selected_indices.clone();
    let anc_fwd = anchor_index.clone();
    let cfg_fwd = current_cfg.clone();
    window.on_go_forward(move || {
        let window = w.unwrap();
        let mut nav = nav_ref.borrow_mut();
        if let Some(path) = nav.go_forward() {
            let path = path.clone();
            drop(nav);
            load_directory(&window, &nav_ref, &files_ref, &sel_fwd, &anc_fwd, &path, &cfg_fwd);
        }
    });

    let w = window.as_weak();
    let nav_ref = nav.clone();
    let files_ref = files_cache.clone();
    let sel_up = selected_indices.clone();
    let anc_up = anchor_index.clone();
    let cfg_up = current_cfg.clone();
    window.on_go_up(move || {
        let window = w.unwrap();
        let current = nav_ref.borrow().current().clone();
        if let Some(parent) = browser::parent_dir(&current) {
            load_directory(&window, &nav_ref, &files_ref, &sel_up, &anc_up, &parent, &cfg_up);
        }
    });

    let w = window.as_weak();
    let nav_ref = nav.clone();
    let files_ref = files_cache.clone();
    let sel_path = selected_indices.clone();
    let anc_path = anchor_index.clone();
    let cfg_path = current_cfg.clone();
    window.on_path_entered(move |path_str| {
        let window = w.unwrap();
        let path = PathBuf::from(path_str.to_string());
        if path.is_dir() {
            load_directory(&window, &nav_ref, &files_ref, &sel_path, &anc_path, &path, &cfg_path);
        }
    });

    let w = window.as_weak();
    let nav_ref = nav.clone();
    let files_ref = files_cache.clone();
    let sel_side = selected_indices.clone();
    let anc_side = anchor_index.clone();
    let cfg_side = current_cfg.clone();
    window.on_sidebar_item_clicked(move |path_str| {
        let window = w.unwrap();
        let path = PathBuf::from(path_str.to_string());
        load_directory(&window, &nav_ref, &files_ref, &sel_side, &anc_side, &path, &cfg_side);
    });

    let w = window.as_weak();
    let nav_for_activated = nav.clone();
    let files_ref = files_cache.clone();
    let sel_act = selected_indices.clone();
    let anc_act = anchor_index.clone();
    let cfg_act = current_cfg.clone();
    window.on_file_activated(move |index| {
        let window = w.unwrap();
        let files = files_ref.borrow();
        if let Some(entry) = files.get(index as usize) {
            if entry.is_dir {
                let path = entry.path.clone();
                drop(files);
                let nav_ref2 = nav_for_activated.clone();
                let files_ref2 = files_ref.clone();
                let sel_act2 = sel_act.clone();
                let anc_act2 = anc_act.clone();
                let cfg_ref2 = cfg_act.clone();
                load_directory(&window, &nav_ref2, &files_ref2, &sel_act2, &anc_act2, &path, &cfg_ref2);
            } else {
                let path = entry.path.clone();
                drop(files);
                let file_type = viewers::detect_file_type(&path);
                match config::load() {
                    Ok(cfg) => {
                        match viewers::launch_viewer(file_type, &path, &cfg.viewers) {
                            Ok(_) => {
                                let viewer_name = match file_type {
                                    FileType::Image => &cfg.viewers.image_viewer,
                                    FileType::Video => &cfg.viewers.video_viewer,
                                    FileType::Pdf => &cfg.viewers.pdf_viewer,
                                    _ => "file",
                                };
                                window.set_status_message(format!("Opening with {}...", viewer_name).into());
                            }
                            Err(e) => {
                                window.set_status_message(format!("Error: {}", e).into());
                            }
                        }
                    }
                    Err(e) => {
                        window.set_status_message(format!("Error loading config: {}", e).into());
                    }
                }
            }
        }
    });

    let sel_indices = selected_indices.clone();
    let anchor = anchor_index.clone();
    let files_sel = files_cache.clone();
    let w = window.as_weak();
    window.on_file_selected(move |index, ctrl, shift| {
        let window = w.unwrap();
        update_selection(
            &window,
            index as usize,
            ctrl,
            shift,
            &sel_indices,
            &anchor,
            &files_sel,
        );
    });

fn update_selection(
    window: &MainWindow,
    clicked_index: usize,
    ctrl: bool,
    shift: bool,
    selected_indices: &Rc<RefCell<HashSet<usize>>>,
    anchor_index: &Rc<RefCell<Option<usize>>>,
    files_cache: &Rc<RefCell<Vec<browser::FileEntry>>>,
) {
    let mut indices = selected_indices.borrow_mut();
    let mut anchor = anchor_index.borrow_mut();
    let files = files_cache.borrow();
    let file_count = files.len();
    
    if shift {
        // Shift+click: select range from anchor to clicked
        let anchor_pos = anchor.as_ref().unwrap_or(&clicked_index);
        let start = (*anchor_pos).min(clicked_index);
        let end = (*anchor_pos).max(clicked_index);
        
        if ctrl {
            // Ctrl+Shift: extend range (add to existing selection)
            for i in start..=end {
                indices.insert(i);
            }
        } else {
            // Shift only: new range selection
            indices.clear();
            for i in start..=end {
                indices.insert(i);
            }
        }
    } else if ctrl {
        // Ctrl+click: toggle individual selection
        if indices.contains(&clicked_index) {
            indices.remove(&clicked_index);
        } else {
            indices.insert(clicked_index);
        }
        *anchor = Some(clicked_index);
    } else {
        // Plain click: single selection
        indices.clear();
        indices.insert(clicked_index);
        *anchor = Some(clicked_index);
    }
    
    // Update UI - convert files with selection state
    let selected_vec: Vec<i32> = indices.iter().map(|&i| i as i32).collect();
    let slint_files: Vec<FileEntry> = files
        .iter()
        .enumerate()
        .map(|(i, e)| to_slint_entry(e, indices.contains(&i)))
        .collect();
    
    let model: slint::ModelRc<FileEntry> =
        Rc::new(slint::VecModel::from(slint_files)).into();
    window.set_files(model);
    window.set_selected_indices(
        Rc::new(slint::VecModel::from(selected_vec)).into()
    );
    
    let count = indices.len();
    window.set_status_message(
        format!("{} selected", count).into()
    );
    
    if let Some(first_selected) = indices.iter().min() {
        if let Some(entry) = files.get(*first_selected) {
            window.set_selected_file_path(entry.path.to_string_lossy().to_string().into());
            window.set_selected_file_name(entry.name.clone().into());
            window.set_selected_file_size(entry.size_display().into());
            window.set_selected_file_modified(entry.modified_display().into());
            window.set_selected_file_created("--".into());
            
            let mime = if entry.is_dir {
                "directory".to_string()
            } else {
                entry.path.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| match e {
                        "pdf" => "application/pdf",
                        "md" | "markdown" => "text/markdown",
                        "txt" => "text/plain",
                        "jpg" | "jpeg" => "image/jpeg",
                        "png" => "image/png",
                        "gif" => "image/gif",
                        "svg" => "image/svg+xml",
                        "mp4" => "video/mp4",
                        "mkv" => "video/x-matroska",
                        "mp3" => "audio/mpeg",
                        "wav" => "audio/wav",
                        "zip" => "application/zip",
                        "tar" => "application/x-tar",
                        "gz" => "application/gzip",
                        "rs" => "text/x-rust",
                        "py" => "text/x-python",
                        "js" => "text/javascript",
                        "ts" => "text/typescript",
                        "json" => "application/json",
                        "yaml" | "yml" => "application/x-yaml",
                        "toml" => "application/toml",
                        _ => "application/octet-stream",
                    })
                    .unwrap_or("application/octet-stream")
                    .to_string()
            };
            window.set_selected_file_mime(mime.into());
            window.set_selected_file_is_dir(entry.is_dir);
            window.set_show_details_panel(true);
        }
    } else {
        window.set_show_details_panel(false);
    }
}

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
                        let cfg_ref = current_cfg.clone();
                        let nav_ref = nav.clone();
                        let files_ref = files_cache.clone();
                        let sel_ref = selected_indices.clone();
                        let anc_ref = anchor_index.clone();
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
                                ui: cfg_ref.borrow().ui.clone(),
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
