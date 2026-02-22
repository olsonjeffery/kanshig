# kanshig Utility - Implementation Plan

## Overview

kanshig is a Rust-based Text User Interface (TUI) application that generates and updates Kanshi configs from the current state of your windows, based on what your wayland window manager (niri) reports.

## Project Structure

```
kanshig/
├── Cargo.toml              # Workspace configuration
├── src/
│   ├── main.rs            # Application entry point
│   ├── model/             # Core data models
│   │   ├── mod.rs
│   │   ├── output.rs      # Display output models
│   │   ├── profile.rs     # Kanshi profile models
│   │   └── config.rs      # Kanshi config models
│   ├── parser/            # Niri JSON output parsing
│   │   ├── mod.rs
│   │   └── json.rs        # JSON parsing logic
│   ├── ui/                # TUI components using ratatui
│   │   ├── mod.rs
│   │   ├── components.rs    # Reusable UI components
│   │   └── screens.rs     # Screen layouts
│   ├── storage/           # Config file handling
│   │   ├── mod.rs
│   │   ├── kanshi.rs      # Kanshi config generation
│   │   └── backup.rs      # Backup management
│   ├── cli/               # Command-line interface
│   │   ├── mod.rs
│   │   └── args.rs        # Argument parsing
│   └── error.rs           # Error handling
├── tests/
│   ├── integration.rs     # Integration tests
│   └── fixtures/          # Test fixtures
└── assets/                # Static assets (icons, templates)
```

## Core Dependencies

### Required Crates

```toml
[workspace.dependencies]
ratatui = "0.28"           # TUI framework
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"         # JSON parsing
thiserror = "2.0"          # Error handling
clap = { version = "4.5", features = ["derive"] }
tokio = { version = "1.37", features = ["full"] }  # Async runtime
anyhow = "1.0"             # Error context
log = "0.4"                # Logging
env_logger = "0.11"        # Logger initialization

[dependencies]
ratatui.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
clap.workspace = true
tokio.workspace = true
anyhow.workspace = true
log.workspace = true
env_logger.workspace = true
```

### Optional Dependencies

```toml
[features]
default = []
watch = ["notify"]         # File system watching
```

## Core Data Models

### Output Model (src/model/output.rs)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub name: String,
    pub make: Option<String>,
    pub model: Option<String>,
    pub serial: Option<String>,
    pub physical_size: Option<[u32; 2]>,
    pub modes: Vec<Mode>,
    pub current_mode: Option<usize>,
    pub is_custom_mode: bool,
    pub vrr_supported: bool,
    pub vrr_enabled: bool,
    pub logical: Option<Logical>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub is_preferred: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logical {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale: f64,
    pub transform: Transform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transform {
    Normal,
    _90,
    _180,
    _270,
}
```

### Profile Model (src/model/profile.rs)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub outputs: Vec<OutputAssignment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputAssignment {
    pub alias: String,
    pub enabled: bool,
}
```

### Config Model (src/model/config.rs)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanshiConfig {
    pub outputs: Vec<OutputDefinition>,
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputDefinition {
    pub name: String,
    pub mode: String,
    pub position: String,
    pub scale: f64,
    pub alias: Option<String>,
}
```

## Parser Module (src/parser/json.rs)

### Functionality

1. Parse niri JSON output from `niri msg --json outputs`
2. Handle parsing errors gracefully
3. Validate parsed data
4. Convert to internal model format

```rust
pub fn parse_niri_output(json_str: &str) -> Result<Vec<Output>, ParserError> {
    // Implementation details...
}
```

## UI Module (src/ui/)

### Screen Layouts

1. **Main Screen**: Display current outputs with status indicators
2. **Profile Selection**: Choose/create profiles
3. **Config Preview**: View generated Kanshi config
4. **Help Screen**: Documentation and usage info

### UI Components

1. **Output Widget**: Display output information
2. **Status Indicator**: Visual feedback for enabled/disabled states
3. **Profile List**: Scrollable list of profiles
4. **Config Editor**: Text-based config preview

## Storage Module (src/storage/)

### Kanshi Config Generation (src/storage/kanshi.rs)

```rust
pub fn generate_kanshi_config(outputs: &[Output], profile: &Profile) -> String {
    // Generate TOML format for Kanshi
}
```

### Backup Management (src/storage/backup.rs)

```rust
pub fn backup_existing_config() -> Result<(), StorageError> {
    // Create timestamped backup of existing kanshi config
}
```

## CLI Module (src/cli/)

### Command Structure

```bash
kanshig [-c ..]
```

### Options

```bash
kanshig
  -c, --config <FILE>   Load the kanshi config from a custom location
```

## Error Handling (src/error.rs)

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Failed to parse niri output: {0}")]
    ParseError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
}
```

## Application Flow

### Main Loop

```
1. Initialize logger and configuration
2. Parse command-line arguments
3. Load existing Kanshi config file (if any)
  3a. Check if `-c` flag passed to kanshig, if so load config from that exact path
  3b. If no `-c` passed, look at ~/.config/kanshi/config
4. Get current outputs from niri
5. Display UI based on kanshig config
6. Handle user input
7. Generate/update Kanshi config
8. Save config with backup if enabled
9. Exit
```

### Watch Mode Flow

```
1. Initialize logger and configuration
2. Load existing Kanshi config
3. Get current outputs from niri
4. Start file system watcher for niri output
5. Enter event loop:
   - Wait for display changes
   - Update outputs from niri
   - Auto-generate config if needed
   - Notify user of changes
```

## Testing Strategy

### Unit Tests (src/)

- Parser tests: `src/parser/json.rs`
- Model tests: `src/model/*.rs`
- Storage tests: `src/storage/*.rs`

### Integration Tests (tests/)

- CLI argument parsing
- Full workflow: parse → generate → save
- Edge cases: no outputs, single output, multiple outputs

### Test Fixtures (tests/fixtures/)

- Sample niri JSON outputs
- Expected Kanshi configs
- Test backup files

## Implementation Steps

### Phase 1: Foundation

1. Set up Cargo workspace structure
2. Implement core data models
3. Create error handling types
4. Write unit tests for models

### Phase 2: Parsing

1. Implement JSON parser for niri output
2. Add validation logic
3. Write parser tests with fixtures

### Phase 3: Storage

1. Implement Kanshi config generation
2. Add backup functionality
3. Write storage tests

### Phase 4: UI

1. Set up ratatui framework
2. Implement main screen
3. Add profile selection
4. Create config preview screen

### Phase 5: CLI

1. Implement command structure with clap
2. Add generate command
3. Add watch mode
4. Add help documentation

### Phase 6: Testing

1. Write integration tests
2. Add regression tests
3. Test edge cases

## Configuration

### Default Paths

- Kanshi config: `~/.config/kanshi/config`
- Backup directory: `~/.config/kanshi/backups/`
- Cache directory: `~/.cache/kanshig/`

### Configuration File (optional)

```toml
# ~/.config/kanshig/config.toml
backup = true
backup_dir = "~/.config/kanshi/backups"
auto_generate = false
```

## Future Enhancements

1. Multiple profile support with UI selection
2. Automatic profile detection based on network/hardware
3. Config validation before saving
4. Template system for common setups
5. Export/import profiles
6. History/undo functionality

## Documentation

- README.md: Project overview and quick start
- USAGE.md: Detailed usage instructions
- ARCHITECTURE.md: Design decisions and system architecture
- API.md: Public API documentation
