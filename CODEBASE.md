## Project Overview

Kanshig is a Rust-based TUI application for generating and updating [Kanshi](https://github.com/hyprlinux/kanshi) configs based on the current state of displays reported by the [niri](https://niri.kirinyaga.org/) Wayland window manager.

## Architecture

The project uses a workspace structure with a single member crate `kanshig`.

### Module Structure

```
kanshig/src/
├── main.rs          # CLI entry point, TUI loop, input handling
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
5. **TUI Display**: `tui.rs` renders unified view combining config outputs with detected niri outputs

### Key Data Types

- `KanshiConfig`: Container for `Vec<OutputDefinition>` and `Vec<Profile>`
- `OutputDefinition`: name, mode, position, scale, optional alias
- `Profile`: name + `Vec<OutputAssignment>` (alias + enabled flag)
- `UnifiedOutput`: Combines config data with detection status (`configured`/`detected` flags)
- `NiriOutputs`: `HashMap<String, NiriOutput>` - keyed by output name (e.g., "DP-8", "eDP-1")

### TUI State

`KanshigTuiState` enum tracks focus:

- `OutputsFocused(i32, i32)` - navigating outputs list
- `ProfilesFocused(i32, i32)` - navigating profiles list
- `QuitNow` - exit flag

Navigation uses vi-style keys (hjkl/wasd) plus arrow keys and Tab to switch focus between panels.

## Build & Test Commands

```bash
# Build
cargo build

# Run (requires niri WM running)
cargo run

# Run with custom config path
cargo run -- -c /path/to/config

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

- `clap` - CLI argument parsing
- `serde`/`serde_json` - JSON serialization for niri output
- `ratatui` with `crossterm` - TUI framework
- `thiserror` - Error type definitions
- `tokio` - Async runtime (workspace dependency)
- `anyhow`, `log`, `env_logger` - Error handling and logging

