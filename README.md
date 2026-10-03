# FileExplorer

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)
[![Platform: Windows](https://img.shields.io/badge/Platform-Windows-0078D6.svg?logo=windows&logoColor=white)](#)

A native cross-platform file manager built in Rust using the **iced** GUI framework. It renders a 900×700 window with a three-panel layout (toolbar, sidebar, and file list) and supports navigation, path-bar editing, and double-click file/folder opening.

---

## Features

- **iced Application Loop**: Implements the `Application` trait fully, with a typed `Message` enum driving all state transitions via the `update` method.
- **Double-Click Detection**: Tracks `Instant` timestamps per click and compares against a 500ms `Duration` threshold to differentiate single selection from double-click navigation or file opening.
- **`opener` Integration**: Double-clicking a non-directory file dispatches an async `Command::perform` calling `opener::open(path)`, deferring to the OS default application.
- **Live Path Bar**: The `PathChanged` and `PathSubmitted` messages keep the toolbar text input synchronized with `current_path`, allowing manual path entry.

---

## Quick Start

1. Clone or download the repository.
2. Install the Rust toolchain via `rustup`.
3. Run `cargo run` to compile and launch the explorer window.

---

## Configuration Details

The application launches at the current working directory (`std::env::current_dir()`). No configuration files are needed. The window is set to 900×700 and `resizable: true` by default.

---

## Usage Guidelines

- **Browse**: Double-click any directory to navigate into it.
- **Go Up**: Use the toolbar "Up" button to ascend one level.
- **Manual Navigation**: Edit the path bar directly and press Enter to jump to any absolute path.
- **Open Files**: Double-click any file to open it in the OS-default application.

---

## Technical Documentation

For developers interested in directory structures, code architecture, or compilation guidelines, please refer to the **[Documentation.md](Documentation.md)** file.

---

## License

This project is licensed under the **GNU Affero General Public License Version 3 (AGPLv3)**. See the LICENSE file for details.
