# Quill Task

A context-aware terminal task manager built in Rust that automatically organizes your tasks based on your current Git repository and branch.

## Features

- **Context-Aware Organization**: Tasks are automatically organized by Git organization, repository, and branch
- **Multiple Storage Options**: Choose between local file storage or MongoDB
- **Interactive Terminal UI**: Full-featured TUI built with ratatui
- **Real-time Context Switching**: Automatically switches task lists when you change directories or branches
- **Task Status Management**: Track tasks as Not Started, In Progress, or Completed
- **Undo Delete**: Restore up to 3 most recently deleted tasks with the 'u' key
- **Configuration Management**: Easy setup for storage preferences

## Installation

### Cargo Install (Recommended)

The easiest way to install Quill Task is using Cargo:

```bash
# Install directly from GitHub
cargo install --git https://github.com/MatthewMyrick/quill
```

This will download, compile, and install the `quill` binary to your Cargo bin directory (usually `~/.cargo/bin/`). Make sure this directory is in your PATH.

To update to the latest version:
```bash
cargo install --git https://github.com/MatthewMyrick/quill --force
```

### Download Pre-built Binaries

Download the latest release for your platform from the [GitHub releases page](https://github.com/MatthewMyrick/quill/releases).

Available platforms:
- **macOS (Intel)**: `quill-x86_64-apple-darwin.tar.gz`
- **macOS (Apple Silicon)**: `quill-aarch64-apple-darwin.tar.gz`
- **Linux (GNU)**: `quill-x86_64-unknown-linux-gnu.tar.gz`
- **Linux (musl)**: `quill-x86_64-unknown-linux-musl.tar.gz`
- **Windows**: `quill-x86_64-pc-windows-msvc.exe.zip`

#### Quick Install Script

**macOS/Linux:**
```bash
# Download and install the latest version
curl -L -o /tmp/quill.tar.gz "https://github.com/MatthewMyrick/quill/releases/latest/download/quill-$(uname -m)-$(uname -s | tr '[:upper:]' '[:lower:]').tar.gz"
tar -xzf /tmp/quill.tar.gz -C /tmp
sudo mv /tmp/quill-* /usr/local/bin/quill
chmod +x /usr/local/bin/quill
```

**Manual Installation:**
1. Download the appropriate binary for your platform
2. Extract the archive: `tar -xzf quill-*.tar.gz` (or unzip for Windows)
3. Move the binary to a directory in your PATH (e.g., `/usr/local/bin/` on macOS/Linux)
4. Make it executable: `chmod +x /path/to/quill` (macOS/Linux only)

### Build from Source

#### Prerequisites

- Rust 1.70 or later
- Git (for context awareness)
- MongoDB (optional, for MongoDB storage)

```bash
git clone https://github.com/MatthewMyrick/quill
cd quill
cargo build --release
```

The binary will be available at `target/release/quill`. You can copy it to your PATH:

```bash
# Copy to your PATH (optional)
sudo cp target/release/quill /usr/local/bin/quill
```

## Usage

### Basic Commands

```bash
# Start the task manager
quill
```

### Keyboard Shortcuts

**Task Management:**

- `a` - Add new task
- `e` - Edit selected task (not available for completed tasks)
- `d` - Delete selected task
- `u` - Undo delete (restores up to 3 most recently deleted tasks)
- `Space` - Toggle task status (cycles through Not Started → In Progress → Completed)
- `1` - Set task to Not Started
- `2` - Set task to In Progress  
- `3` - Set task to Completed

**Navigation:**

- `↑/k` - Move up in task list
- `↓/j` - Move down in task list

**General:**

- `c` - Open configuration
- `q` - Quit application

### Context Awareness

Quill automatically detects your current Git context and organizes tasks accordingly:

- **Organization**: Extracted from Git remote URL (e.g., "MatthewMyrick" from `git@github.com:MatthewMyrick/quill.git`)
- **Repository**: Current repository name
- **Branch**: Current Git branch

Tasks are scoped to this context, so switching between projects or branches will show you the relevant task list.

## Configuration

### Storage Options

#### Local Storage (Default)

Tasks are stored in JSON files on your local filesystem.

**Default path**: `~/.quill/storage/todos.json`

#### MongoDB Storage

Store tasks in a MongoDB database for persistence across devices.

**Default settings**:

- Connection: `mongodb://localhost:27017`
- Database: `quill`
- Collection: `tasks`

### Configuration File

Configuration is stored at `~/.quill/config.json`:

```json
{
  "storage_type": "Local",
  "local_config": {
    "path": "~/.quill/storage/todos.json"
  },
  "mongo_config": {
    "connection_string": "mongodb://localhost:27017",
    "database": "quill",
    "collection": "tasks"
  }
}
```

### Configuring Storage

1. Press `c` in the main interface
2. Navigate to "Configure Storage"
3. Select your preferred storage type
4. Configure the settings
5. Save and exit

## Architecture

### Core Components

- **App (`src/app.rs`)**: Main application loop and event handling
- **UI (`src/ui.rs`)**: Terminal user interface using ratatui
- **Storage (`src/storage/`)**: Pluggable storage backends
  - `local.rs`: Local JSON file storage
  - `mongodb.rs`: MongoDB storage
- **Git Context (`src/git.rs`)**: Git repository detection and context extraction
- **Config (`src/config.rs`)**: Configuration management

### Dependencies

- **ratatui**: Terminal UI framework
- **crossterm**: Cross-platform terminal manipulation
- **git2**: Git repository interaction
- **mongodb**: MongoDB driver
- **serde**: Serialization/deserialization
- **tokio**: Async runtime
- **chrono**: Date/time handling

## Development

### Running Tests

```bash
cargo test
```

### Running in Development

```bash
cargo run
```

### Code Structure

```
src/
├── main.rs           # Entry point
├── app.rs            # Main application logic
├── ui.rs             # User interface components
├── config.rs         # Configuration management
├── git.rs            # Git context detection
└── storage/
    ├── mod.rs        # Storage trait definition
    ├── local.rs      # Local file storage
    └── mongodb.rs    # MongoDB storage
```

## Task Data Structure

Each task contains:

```rust
pub struct Task {
    pub id: usize,
    pub text: String,
    pub status: TaskStatus,
    pub created_at: String,
}

pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
}
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## Roadmap

- [ ] Task due dates and reminders
- [ ] Task priorities and sorting
- [ ] Export/import functionality
- [ ] Team collaboration features
- [ ] Integration with external task management systems
- [ ] Custom task templates
- [ ] Search and filtering capabilities

