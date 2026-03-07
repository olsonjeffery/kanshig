# Changes

## 2026-03-07 - bd-1dm: Add popup help screen tied to ?

- Added `?` keybinding to toggle help popup
- Help popup displays all keyboard shortcuts grouped by function
- Any key press closes the popup and returns to outputs view
- Designed with extensible `HelpEntry` struct for easy future additions
- Popup renders centered on screen with yellow border

## 2026-03-07 - bd-3mi: Fix TUI focus/toggle behavior

- Fix LLM jank on input handling
- Fix LLM jank on highlighting/focus/tab paradigm in the TUI
- Please take note of how input works and behaves, do not break with future changes

## 2026-02-24 - bd-2kj: Add TUI Frame, mouse support

- Implemented TUI application using ratatui with mouse support
- Added event handling for keyboard input (q to quit, Escape to quit)
- Enabled mouse capture mode for the terminal
- Created a two-panel display showing Kanshi config and Niri outputs
- Integrated existing tui.rs display functions into main application lifecycle

## 2026-02-24 - sync: Update beads DB and AGENTS.md

- Updated beads database with latest issue tracking data
- Updated AGENTS.md documentation for AI coding agents

## 2026-02-23 - bd-tbi: Add TUI display to list all profiles and outputs in the kanshi config

- Implemented TUI display for listing all profiles and outputs from kanshi config
- Created two separate tables showing outputs-from-config and profiles-from-config
- Added cursor navigation with tab key to cycle focus between tables

## 2026-02-23 - bd-bqv: Call niri msg outputs json output and capture

- Implemented Rust's std::process::Command system to invoke niri msg --json outputs
- Added JSON parsing via serde_json into niri outputs data model structs
- Added comprehensive tests for the niri integration

## 2026-02-23 - bd-3eh: Load validated kanshi config into data model structs

- Implemented code to load validated kanshi config into Rust data model structs
- Added comprehensive tests comparing text format with loaded data models

## 2026-02-23 - bd-1iy: Validate kanshi config

- Implemented validation logic for kanshi config files
- Validates matching braces, valid section types (output and profile)
- Validates valid parameters within each section type
- Added comprehensive tests with example file content

## 2026-02-23 - bd-sua: Load kanshi config from fs as string content and display

- Implemented loading of kanshi config file from filesystem
- Added display of config content via STDOUT logging

## 2026-02-22 - fd6478c: chore: add several beads for kalshig impl

- Added multiple beads for kanshig implementation tracking

## 2026-02-22 - bd-1m2: Create kanshig crate and CLI parsing per design

- Created kanshig-cli crate with CLI argument parsing
- Implemented -c/--config flag for custom config file location
- Added ability to read and load kanshi config from filesystem

## 2026-02-22 - cd09b85: rename plan markdown doc

- Renamed kansig-plan.md to KANSHIG-PLAN.md

## 2026-02-22 - fdcf1cd: feat: Create kansig implementation plan document

- Created detailed implementation plan for kanshig utility
- Documented architectural code design, dependencies, and functionality

## 2026-02-22 - initial commit (5e7576c)

- Initial repository setup
- Created README.md with project description
