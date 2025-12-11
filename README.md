# remn

A simple, terminal-based code editor written in Rust.

## Features

- 📝 Simple and clean text editing interface
- ⌨️ Keyboard-driven navigation and editing
- 📄 File loading and editing support
- 🎯 Status bar with file information
- 🚀 Fast and lightweight

## Prerequisites

- Rust 1.70+ (Rust Edition 2024)
- Cargo (comes with Rust)

## Installation

```bash
git clone https://github.com/mnnazrul/remn.git
cd remn
cargo build --release
```

The binary will be available at `target/release/remn`

## Usage

### Open editor without a file:
```bash
cargo run
```

### Open a specific file:
```bash
cargo run -- path/to/file.txt
```

### Run the compiled binary:
```bash
./target/release/remn [filename]
```

## Keyboard Controls

- **Ctrl+Q** - Quit editor
- **Arrow Keys** - Navigate through text
- Regular typing for text input

## Project Structure

```
remn/
├── src/
│   ├── main.rs              # Entry point
│   ├── editor.rs            # Core editor logic
│   └── editor/
│       ├── editorcommand.rs # Command handling
│       ├── statusbar.rs     # Status bar display
│       ├── terminal.rs      # Terminal interface
│       └── view/            # Text view components
│           ├── buffer.rs    # Text buffer management
│           ├── line.rs      # Line handling
│           ├── location.rs  # Cursor positioning
│           └── view.rs      # View rendering
└── Cargo.toml
```

## Dependencies

- [crossterm](https://crates.io/crates/crossterm) - Cross-platform terminal manipulation
- [unicode-segmentation](https://crates.io/crates/unicode-segmentation) - Unicode text segmentation
- [unicode-width](https://crates.io/crates/unicode-width) - Unicode character width

## Roadmap

Features currently under development:

- [ ] Fancy Status Bar - Enhanced status bar with more information
- [ ] Simple Message Bar - Display messages to users
- [ ] Expiring Messages - Auto-dismiss notifications
- [ ] Save as… - Save file with new name
- [ ] Search - Find text within files
- [ ] Syntax Highlighting - Language-specific color coding

## Development

```bash
# Run in development mode
cargo run

# Run tests
cargo test

# Check code
cargo clippy
```

## License

MIT

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
