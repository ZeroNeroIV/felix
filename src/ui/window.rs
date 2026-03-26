//! UI module — Slint-based macOS Finder-style interface

slint::include_modules!();

use crate::fs::browser::{self, SortField, SortDirection};
use crate::search;
use std::cell::RefCell;
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

/// Convert FileEntry to Slint FileEntry
fn to_slint_entry(entry: &browser::FileEntry) -> FileEntry {
    FileEntry {
        name: entry.name.clone().into(),
        path: entry.path.to_string_lossy().to_string().into(),
        is_dir: entry.is_dir,
        size: entry.size_display().into(),
        modified: entry.modified_display().into(),
        icon: entry.icon().into(),
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

    // Navigation state
    let nav = Rc::new(RefCell::new(NavState::new(browser::home_dir())));
    let files_cache: Rc<RefCell<Vec<browser::FileEntry>>> = Rc::new(RefCell::new(Vec::new()));
    
    // Sorting state: track current sort field and direction
    let sort_state: Rc<RefCell<(SortField, SortDirection)>> = 
        Rc::new(RefCell::new((SortField::Name, SortDirection::Ascending)));

    // Initial load
    let initial_path = browser::home_dir();
    load_directory(&window, &nav, &files_cache, &initial_path);

    // Set sidebar items
    let sidebar: slint::ModelRc<SidebarItem> =
        Rc::new(slint::VecModel::from(default_sidebar_items())).into();
    window.set_sidebar_items(sidebar);

    // ─── Callbacks ─────────────────────────────────────────────────────

    let w = window.as_weak();
    let nav_ref = nav.clone();
    let files_ref = files_cache.clone();
    window.on_go_back(move || {
        let window = w.unwrap();
        let mut nav = nav_ref.borrow_mut();
        if let Some(path) = nav.go_back() {
            let path = path.clone();
            drop(nav);
            load_directory(&window, &nav_ref, &files_ref, &path);
        }
    });

    let w = window.as_weak();
    let nav_ref = nav.clone();
    let files_ref = files_cache.clone();
    window.on_go_forward(move || {
        let window = w.unwrap();
        let mut nav = nav_ref.borrow_mut();
        if let Some(path) = nav.go_forward() {
            let path = path.clone();
            drop(nav);
            load_directory(&window, &nav_ref, &files_ref, &path);
        }
    });

    let w = window.as_weak();
    let nav_ref = nav.clone();
    let files_ref = files_cache.clone();
    window.on_go_up(move || {
        let window = w.unwrap();
        let current = nav_ref.borrow().current().clone();
        if let Some(parent) = browser::parent_dir(&current) {
            load_directory(&window, &nav_ref, &files_ref, &parent);
        }
    });

    let w = window.as_weak();
    let nav_ref = nav.clone();
    let files_ref = files_cache.clone();
    window.on_path_entered(move |path_str| {
        let window = w.unwrap();
        let path = PathBuf::from(path_str.to_string());
        if path.is_dir() {
            load_directory(&window, &nav_ref, &files_ref, &path);
        }
    });

    let w = window.as_weak();
    let nav_ref = nav.clone();
    let files_ref = files_cache.clone();
    window.on_sidebar_item_clicked(move |path_str| {
        let window = w.unwrap();
        let path = PathBuf::from(path_str.to_string());
        load_directory(&window, &nav_ref, &files_ref, &path);
    });

    let w = window.as_weak();
    let nav_for_activated = nav.clone();
    let files_ref = files_cache.clone();
    window.on_file_activated(move |index| {
        let window = w.unwrap();
        let files = files_ref.borrow();
        if let Some(entry) = files.get(index as usize) {
            if entry.is_dir {
                let path = entry.path.clone();
                drop(files);
                let nav_ref2 = nav_for_activated.clone();
                let files_ref2 = files_ref.clone();
                load_directory(&window, &nav_ref2, &files_ref2, &path);
            }
            // TODO: Open files with built-in tools
        }
    });

    window.on_file_selected(|_index| {
        // Update status bar with selection info
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
                files_for_search.borrow().iter().map(to_slint_entry).collect();
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
                        .map(|e| to_slint_entry(&e))
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
        let slint_files: Vec<FileEntry> = files.iter().map(to_slint_entry).collect();
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

    window.run()
}

/// Load a directory and update the UI
fn load_directory(
    window: &MainWindow,
    nav: &Rc<RefCell<NavState>>,
    files_cache: &Rc<RefCell<Vec<browser::FileEntry>>>,
    path: &Path,
) {
    match browser::list_directory(path) {
        Ok(entries) => {
            // Update navigation
            nav.borrow_mut().navigate(path.to_path_buf());

            // Update cache
            *files_cache.borrow_mut() = entries.clone();

            // Convert to Slint models
            let slint_files: Vec<FileEntry> = entries.iter().map(to_slint_entry).collect();
            let model: slint::ModelRc<FileEntry> =
                Rc::new(slint::VecModel::from(slint_files)).into();

            // Update UI
            window.set_files(model);
            window.set_current_path(path.to_string_lossy().to_string().into());
            window.set_can_go_back(nav.borrow().can_go_back());
            window.set_can_go_forward(nav.borrow().can_go_forward());
            window.set_sidebar_active_path(path.to_string_lossy().to_string().into());
            window.set_selected_index(-1);
            window.set_search_text("".into());
        }
        Err(e) => {
            log::error!("Failed to list directory {}: {}", path.display(), e);
            window.set_status_message(format!("Error: {}", e).into());
        }
    }
}
