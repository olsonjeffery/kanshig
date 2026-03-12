## Project Overview

Kanshig is a Rust-based TUI application for generating and updating [Kanshi](https://github.com/hyprlinux/kanshi) configs based on the current state of displays reported by the [niri](https://niri.kirinyaga.org/) Wayland window manager.

## Architecture

The project uses a workspace structure with a single member crate `kanshig`.

### Module Structure

```
kanshig/src/
├── main.rs          # CLI entry point, TUI loop, unified output building, reload daemon
├── input.rs         # Input handling, keybindings, editor mode, help popup
├── model/           # Data structures
│   ├── mod.rs       # Re-exports from submodules
│   ├── config.rs    # KanshiConfig (outputs + profiles)
│   ├── output.rs    # OutputDefinition, UnifiedOutput
│   ├── profile.rs   # Profile, OutputAssignment
│   └── niri_output.rs # NiriOutput, NiriMode, NiriLogical (from niri JSON)
├── parser/          # Config parsing
│   └── mod.rs       # parse_config() - converts kanshi config text to model structs
├── validation/      # Config validation
│   └── mod.rs       # validate_config() - checks braces, section types, params
├── niri.rs          # Calls `niri msg --json outputs`, parses JSON response
└── tui.rs           # ratatui rendering functions
```

### Data Flow

1. **Config Loading**: `main.rs` loads kanshi config file (default: `~/.config/kanshi/config`)
2. **Validation**: `validation::validate_config()` checks for matching braces, valid section types, valid parameters
3. **Parsing**: `parser::parse_config()` converts validated config text into `KanshiConfig` struct
4. **Niri Integration**: `niri::get_niri_outputs()` executes `niri msg --json outputs` and parses into `NiriOutputs` (HashMap)
5. **Unified Output Building**: `build_unified_outputs()` merges config outputs with detected niri outputs
6. **TUI Display**: `tui.rs` renders unified view with outputs list, profiles list, and details pane

### Key Data Types

- `KanshiConfig`: Container for `Vec<OutputDefinition>` and `Vec<Profile>`
- `OutputDefinition`: name, mode, position, scale, optional alias
- `Profile`: name + `Vec<OutputAssignment>` (alias + enabled flag)
- `OutputAssignment`: alias (e.g., `$HOME_0`) + enabled (bool)
- `UnifiedOutput`: Combines config data with detection status (`configured`/`detected` flags) and optional `NiriOutput`
- `NiriOutputs`: `HashMap<String, NiriOutput>` - keyed by output name (e.g., "DP-8", "eDP-1")
- `NiriOutput`: name, make, model, serial, physical_size, modes, current_mode, vrr_supported, vrr_enabled, logical
- `NiriMode`: width, height, refresh_rate (in millihertz), is_preferred
- `NiriLogical`: x, y, width, height, scale, transform

### TUI State

`KanshigTuiState` enum tracks UI focus:

- `OutputsFocused(i32, i32)` - navigating outputs list (stores current index + previous for Tab switching)
- `ProfilesFocused(i32, i32)` - navigating profiles list
- `HelpPopup` - help popup is displayed
- `EditConfig { textarea, original_content }` - in-place config editor mode
- `QuitNow` - exit flag

### TUI Layout

The screen is split into:
- **Left column (40%)**: Two vertically stacked lists
  - Top (50%): Outputs list showing unified outputs with labels
  - Bottom (50%): Profiles list showing profile names with output counts
- **Right column (60%)**: Details pane showing selected item details

### Input Handling (`input.rs`)

Keybindings:
- `j`, `s`, `↓` - Move down in current list
- `k`, `w`, `↑` - Move up in current list
- `Tab` - Switch focus between outputs and profiles lists
- `?` - Toggle help popup
- `e` - Open config editor (in-place textarea editing)
- `q`, `Esc` - Quit (or discard editor changes)
- `Ctrl+S` - Save config in editor mode
- `Ctrl+D` - Discard changes in editor mode

### Output Detection & Labels

Unified outputs are marked with labels:
- `CONFIGURED` - Output exists in kanshi config
- `DETECTED` - Output is currently detected by niri

Color coding:
- Yellow: Both configured and detected
- Green: Detected only (not in config)
- White: Configured only (not currently detected)

Detection matches niri output model/name against config output name.

### Editor Mode

Pressing `e` opens an in-place editor using `ratatui-textarea`:
- Full config content is loaded into a textarea
- `Ctrl+S` saves and returns to outputs list
- `Ctrl+D` or `Esc` discards changes and returns

## Build & Test Commands

```bash
# Build
cargo build

# Run (requires niri WM running)
cargo run

# Run with custom config path
cargo run -- -c /path/to/config

# Output unified outputs as JSON (skip TUI)
cargo run -- --json

# Reload kanshi daemon (restart systemd service)
cargo run -- --reload

# Run all tests
cargo test

# Run specific test
cargo test test_name

# Check code
cargo check

# Format code
cargo fmt

# Check formatting without modifying
cargo fmt --check
```

## CLI Arguments

- `-c`, `--config <PATH>` - Load kanshi config from custom location (default: `~/.config/kanshi/config`)
- `--json` - Output unified outputs as JSON and exit (skip TUI)
- `--reload` - Reload kanshi config by restarting the kanshi systemd daemon

## Kanshi Config Format

The parser handles this config format:

```
output "Display Name" {
 mode 2560x1440@119.998
 position 0,1
 scale 1.25
 alias $HOME_0
}

profile home_dock {
 output $INTERNAL disable
 output $HOME_0 enable
}
```

Valid output parameters: `mode`, `position`, `scale`, `alias`
Valid profile parameters: `output` (followed by alias and enable/disable)

## Dependencies

Workspace dependencies in `Cargo.toml`:

- `clap` 4.5 - CLI argument parsing with derive macros
- `serde`/`serde_json` - JSON serialization for niri output and model structs
- `ratatui` 0.30 - TUI framework with crossterm backend and all-widgets feature
- `ratatui-textarea` 0.8 - Text editing widget for config editor
- `thiserror` 2 - Error type definitions
- `tokio` 1 - Async runtime (workspace dependency)
- `log`, `env_logger` 0.11 - Logging infrastructure
- `anyhow` 1.0 - Error handling

## Technical Notes

- **Mouse support**: Enabled via `EnableMouseCapture` on TUI startup, disabled on exit
- **Logging**: Initialized via `env_logger::init()` at startup
- **Error handling**: Uses `thiserror` for domain errors (`ValidationError`, `NiriError`, `ParseError`)
- **Serialization**: All model structs derive `Serialize`/`Deserialize` for JSON support

