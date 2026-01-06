# task-manager-tui

A lightweight, keyboard-driven terminal task manager built with [Ratatui] and Rust. Features persistent SQLite storage and Vim-like navigation.

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)

## Features

- **Task Management** - Create, complete, and delete tasks
- **Persistent Storage** - SQLite database keeps your tasks between sessions
- **Vim-like Navigation** - Familiar keybindings for efficient navigation
- **Visual Feedback** - Strikethrough styling for completed tasks, highlighted selection
- **Minimal & Fast** - Lightweight TUI with no bloat

## Installation

```bash
git clone https://github.com/yourusername/task-manager-tui.git
cd task-manager-tui
cargo build --release
```

## Usage

```bash
cargo run
```

## Keyboard Shortcuts

### Navigation

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `g` | Jump to first task |
| `G` | Jump to last task |

### Task Operations

| Key | Action |
|-----|--------|
| `a` | Add new task (enters input mode) |
| `Space` / `Enter` | Toggle task completion |
| `d` | Delete selected task |
| `q` | Quit application |

### Input Mode

| Key | Action |
|-----|--------|
| `Enter` | Save task |
| `Esc` | Cancel without saving |
| `Backspace` | Delete character |

## Tech Stack

- [Ratatui] - Terminal UI framework
- [Crossterm] - Cross-platform terminal manipulation
- [Rusqlite] - SQLite database interface
- [Color-eyre] - Error handling

## License

Copyright (c) thestackdev <shanmukeshwar@icloud.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[Ratatui]: https://ratatui.rs
[Crossterm]: https://github.com/crossterm-rs/crossterm
[Rusqlite]: https://github.com/rusqlite/rusqlite
[Color-eyre]: https://github.com/eyre-rs/eyre
[LICENSE]: ./LICENSE
