# felix

A lightweight, fast file manager with built-in document tools — built with Rust + Slint.

[![Release](https://img.shields.io/github/v/release/ZeroNeroIV/felix)](https://github.com/ZeroNeroIV/felix/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-blue)](https://github.com/ZeroNeroIV/felix)

## Features

- **Finder-style UI** — clean single-pane layout with sidebar, toolbar, and path bar
- **Dark/Light themes** — system-following with customizable accent color
- **Built-in document tools** — markdown, PDF, DOCX, PPTX viewers (via feature flags)
- **Archive browsing** — browse inside .zip/.tar.gz/.7z like folders
- **Fast search** — filename + content search powered by ripgrep's engine
- **Column sorting** — click headers to sort by name, size, or modified date
- **File operations** — copy, move, delete, rename files and directories
- **Configurable** — YAML config at `~/.config/felix.config.yml`

## Screenshots

```
┌─────────────────────────────────────────────────────────────┐
│ ◀ ▶ ▲  /home/user/Documents              🔍 Search... ⚙   │
├──────────┬────────────────────────────────────────────────┤
│ Favorites│  Name          Size     Modified               │
│ ──────── │  📁 projects   --      2026-03-25 14:30       │
│ 🏠 Home  │  📁 downloads   --      2026-03-24 09:15       │
│ 🖥️ Desktop│  📄 report.md   24.5 KB 2026-03-23 16:45      │
│ 📁 Documents│ 📊 data.xlsx   128.2 KB 2026-03-22 11:20    │
│ 📥 Downloads│ 📄 readme.md   4.2 KB  2026-03-21 08:00      │
│ 🖼️ Pictures│ 📦 archive.zip  2.1 MB  2026-03-20 19:30      │
│ 🎵 Music  │                                                │
│ 🎬 Videos │                                                │
│ ──────── │                                                │
│ Devices  │                                                │
│ 💻 Root   │                                                │
├──────────┴────────────────────────────────────────────────┤
│ 7 items                                 Sorted by name ↑  │
└─────────────────────────────────────────────────────────────┘
```

## Installation

### Linux (DEB)

```bash
sudo dpkg -i felix_*.deb
sudo apt-get install -f
```

### Linux (RPM)

```bash
sudo rpm -i felix-*.rpm
```

### Linux (AppImage)

```bash
chmod +x felix-*.AppImage
./felix-*.AppImage
```

### From crates.io

```bash
cargo install felix
```

### Build from source

```bash
git clone https://github.com/ZeroNeroIV/felix.git
cd felix
cargo build --release
./target/release/felix
```

## Feature Flags

felix uses Cargo feature flags to include only the tools you need:

| Feature   | Description                    | Default |
|-----------|--------------------------------|---------|
| `markdown`| Markdown viewer                | ✅ Yes  |
| `pdf`     | PDF viewer                     | ✅ Yes  |
| `docx`    | DOCX viewer                    | ❌ No   |
| `pptx`    | PPTX viewer                    | ❌ No   |
| `all-tools`| All document viewers         | ❌ No   |

```bash
# Default installation (markdown + pdf)
cargo install felix

# All tools
cargo install felix --features all-tools

# Only specific tools
cargo install felix --features markdown,docx --no-default-features
```

## Configuration

Config file: `~/.config/felix.config.yml` (auto-created on first launch)

```yaml
general:
  show_hidden: false
  confirm_delete: true
  page_size: 100

theme:
  mode: System  # Light, Dark, or System
  accent_color: "#58a6ff"
  # Optional: custom color overrides for each theme
  light_colors:
    bg_window: "#f5f5f0"
    text_primary: "#3a3a3c"
  dark_colors:
    bg_window: "#1a1b26"
    text_primary: "#c0caf5"

tools:
  enabled:
    - markdown
    - pdf
  markdown_preview: true
  pdf_preview: true
  docx_preview: false
  pptx_preview: false

sidebar:
  favorites: []
  show_devices: true
  show_bookmarks: true

ui:
  default_path: null
  window_width: null
  window_height: null
```

## Keyboard Shortcuts

### Navigation

| Key         | Action                    |
|-------------|---------------------------|
| `Enter`     | Open file/folder          |
| `Backspace` | Go to parent directory   |
| `h`         | Go back in history       |
| `l`         | Go forward in history    |
| `j`         | Select next file         |
| `k`         | Select previous file     |

### Selection

| Key              | Action                    |
|------------------|---------------------------|
| `Click`          | Select single file        |
| `Ctrl+Click`     | Toggle selection          |
| `Shift+Click`    | Select range              |
| `Ctrl+Shift+Click` | Extend range            |
| `Ctrl+A`         | Select all                |

### File Operations

| Key         | Action                    |
|-------------|---------------------------|
| `Delete`    | Delete selected           |
| `F2`        | Rename selected           |
| `Ctrl+C`    | Copy selected             |
| `Ctrl+X`    | Cut selected              |
| `Ctrl+V`    | Paste                     |

### Other

| Key         | Action                    |
|-------------|---------------------------|
| `Ctrl+F`    | Focus search              |
| `Ctrl+,`    | Open settings             |
| `?`         | Show keyboard shortcuts  |
| `F5`        | Refresh                  |

## Architecture

```
src/
├── main.rs          # Entry point
├── config/          # YAML config management
├── fs/
│   ├── browser.rs   # Directory listing, sorting
│   └── operations.rs # Copy, move, delete, rename
├── search/
│   ├── filename.rs  # Filename search (ignore crate)
│   └── content.rs   # Content search (grep-searcher)
├── archive/
│   └── virtual_fs.rs # Archive virtual FS
├── tools/
│   ├── markdown/    # Markdown rendering
│   ├── pdf/         # PDF viewing
│   ├── docx/       # DOCX parsing
│   └── pptx/       # PPTX parsing
└── ui/
    ├── mod.rs       # UI exports
    └── window.rs    # Main window logic
```

## Development

```bash
# Run in development mode
cargo run

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run

# Build release
cargo build --release
```

## License

MIT License — see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please open an issue or pull request on GitHub.

## Roadmap

### Completed ✅
- [x] Finder-style UI with sidebar, toolbar, path bar
- [x] Directory browsing with file metadata
- [x] Dark/Light themes with system detection
- [x] Filename search (ripgrep engine)
- [x] Content search backend
- [x] YAML config system with settings UI
- [x] Column sorting (name, size, modified)
- [x] File operations (copy, move, delete, rename)
- [x] Archive browsing (.zip, .tar.gz, .7z)

### In Progress 🚧
- [ ] Document viewers (markdown, PDF, DOCX, PPTX)
- [ ] Quick preview with Spacebar

### Planned 📋
- [ ] Vim-style navigation (h/j/k/l)
- [ ] Drag and drop support
- [ ] Custom bookmarks
- [ ] macOS native build
- [ ] Windows native build
- [ ] File type icons (SVG instead of emoji)
