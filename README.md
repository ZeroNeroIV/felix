# felix

A lightweight, fast file manager with built-in document tools — built with Rust + Slint.

## Features

- **macOS Finder-style UI** — clean single-pane layout with sidebar, toolbar, and path bar
- **Built-in document tools** — markdown, PDF, DOCX, PPTX viewers/editors (embedded)
- **Quick preview** — Spacebar to preview files instantly
- **Archive browsing** — browse inside .zip/.tar.gz/.7z like folders
- **Fast search** — filename + content search powered by ripgrep's engine
- **Cross-platform** — Linux first, macOS and Windows coming soon
- **Lightweight** — Rust-powered, minimal resource usage
- **Customizable** — Light/Dark themes (follows system), configurable sidebar

## Install

### From crates.io

```bash
cargo install felix
```

### Quick install (curl)

```bash
curl -sSL https://raw.githubusercontent.com/ZeroNeroIV/felix/main/install.sh | sh
```

### Build from source

```bash
git clone https://github.com/ZeroNeroIV/felix.git
cd felix
cargo build --release
```

## Feature Flags

felix uses Cargo feature flags so you can choose which built-in tools to include:

```bash
# Default: markdown + pdf
cargo install felix

# All tools
cargo install felix --features all-tools

# Only markdown
cargo install felix --features markdown --no-default-features
```

| Feature   | Description                    |
|-----------|--------------------------------|
| `markdown`| Markdown viewer/editor         |
| `pdf`     | PDF viewer                     |
| `docx`    | DOCX viewer/editor             |
| `pptx`    | PPTX viewer/editor             |
| `all-tools`| All of the above              |

## Configuration

Config lives at `~/.config/felix/config.toml`. Created automatically on first launch.

## Keyboard Shortcuts

| Key         | Action                  |
|-------------|-------------------------|
| `Space`     | Quick preview           |
| `h/j/k/l`  | Vim-style navigation    |
| `Ctrl+C`    | Copy                    |
| `Ctrl+X`    | Cut                     |
| `Ctrl+V`    | Paste                   |
| `Delete`    | Delete selected         |
| `F2`        | Rename                  |
| `Ctrl+F`    | Search                  |

## License

MIT

## Roadmap

### In Progress
- [x] macOS Finder-style UI with sidebar, toolbar, path bar
- [x] Directory browsing with file metadata (size, modified)
- [x] Recursive filename search (ripgrep engine)
- [x] Content search backend (grep-searcher)
- [x] XDG-compliant config system

### Planned
- [ ] **File operations** — copy, cut, paste, delete, rename via keyboard shortcuts
- [ ] **Archive browsing** — browse inside .zip/.tar.gz/.7z like folders
- [ ] **Quick preview** — Spacebar to preview files instantly
- [ ] **Markdown viewer** — render markdown with pulldown-cmark
- [ ] **PDF viewer** — embedded PDF rendering
- [ ] **DOCX viewer** — read and display .docx files
- [ ] **PPTX viewer** — read and display .pptx files
- [ ] **Sorting** — click column headers to sort by name/size/modified
- [ ] **Vim-style navigation** — h/j/k/l keybindings
- [ ] **Dark theme** — system-following light/dark mode toggle
- [ ] **Content search UI** — wire content search results into the search bar
- [ ] **File type icons** — proper SVG icons instead of emoji
- [ ] **Drag and drop** — drag files between windows
- [ ] **Bookmarks** — save and manage custom sidebar favorites
- [ ] **macOS support** — native macOS build
- [ ] **Windows support** — native Windows build
