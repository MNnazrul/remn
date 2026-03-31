# remn

A lightweight, terminal-based text editor written in Rust with full Unicode support.

## Features

- Keyboard-driven text editing with arrow key navigation
- Full Unicode support (wide characters, combining marks, grapheme clusters)
- Status bar with file name, line count, position, and modification indicator
- Message bar for user notifications
- Page-based scrolling and Home/End navigation
- File loading and saving

## Prerequisites

- Rust (Edition 2024)
- Cargo

## Installation

```bash
git clone https://github.com/mnnazrul/remn.git
cd remn
cargo build --release
```

The binary will be at `target/release/remn`.

## Usage

```bash
# Open editor
cargo run

# Open a file
cargo run -- path/to/file.txt

# Or use the compiled binary
./target/release/remn [filename]
```

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Ctrl+S` | Save file |
| `Ctrl+D` | Save as (new filename) |
| `Ctrl+Q` | Quit editor |
| `Arrow Keys` | Move cursor |
| `Home` / `End` | Jump to start / end of line |
| `PageUp` / `PageDown` | Scroll by page |
| `Backspace` / `Delete` | Delete characters |
| `Enter` | Insert new line |
| `Tab` | Insert tab |

## Project Structure

```
src/
├── main.rs                    # Entry point
├── editor.rs                  # Main editor loop and event handling
└── editor/
    ├── editorcommand.rs       # Key event to command mapping
    ├── documentstatus.rs      # Document metadata (name, line count, position)
    ├── fileinfo.rs            # File path management
    ├── statusbar.rs           # Status bar rendering
    ├── messagebar.rs          # Message bar rendering
    ├── terminal.rs            # Terminal abstraction (crossterm)
    ├── uicomponent.rs         # UIComponent trait for rendering
    ├── view.rs                # Text view, cursor, scrolling
    └── view/
        ├── buffer.rs          # Text buffer (load, save, insert, delete)
        ├── line.rs            # Line with grapheme fragment handling
        └── location.rs        # Cursor location (line, grapheme index)
```

## Dependencies

- [crossterm](https://crates.io/crates/crossterm) - Cross-platform terminal manipulation
- [unicode-segmentation](https://crates.io/crates/unicode-segmentation) - Unicode grapheme cluster segmentation
- [unicode-width](https://crates.io/crates/unicode-width) - Unicode display width calculation

## Roadmap

- [x] Save as (save file with new name)
- [ ] Search (find text within files)
- [ ] Syntax highlighting
- [ ] Expiring messages (auto-dismiss notifications)

## Development

```bash
cargo run          # Run in dev mode
cargo test         # Run tests
cargo clippy       # Lint
```

## License

MIT
