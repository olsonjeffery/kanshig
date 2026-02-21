# BEADS GUIDE

All of is this is derived from the [README.md at the beads_rs repo][1].

NOTE: This guide assumes `beads-rs` is accessed as the `bd` command.

## Quick Example

```bash
# Initialize bd in your project
cd my-project
bd init

# Add agent instructions to AGENTS.md (creates file if needed)
bd agents --add --force

# Create issues with priority (0=critical, 4=backlog)
bd create "Implement user auth" --type feature --priority 1
# Created: bd-7f3a2c

bd create "Set up database schema" --type task --priority 1
# Created: bd-e9b1d4

# Auth depends on database schema
bd dep add bd-7f3a2c bd-e9b1d4

# See what's ready to work on (not blocked)
bd ready
# bd-e9b1d4  P1  task     Set up database schema

# Claim and complete work
bd update bd-e9b1d4 --status in_progress
bd close bd-e9b1d4 --reason "Schema implemented"

# Now auth is unblocked
bd ready
# bd-7f3a2c  P1  feature  Implement user auth

# Export to JSONL for git commit
bd sync --flush-only
git add .beads/ && git commit -m "Update issues"
```

## Agent-First Design

Every command supports `--json` for AI coding agents:

```bash
bd list --json | jq '.[] | select(.priority <= 1)'
bd ready --json  # Structured output for agents
bd show bd-abc123 --json
```

---

## Quick Start

### 1. Verify Installation

```bash
bd --version
# bd 0.1.0 (rustc 1.85.0-nightly)
```

### 2. Create Your First Issue

```bash
bd create "Fix login timeout bug" \
  --type bug \
  --priority 1 \
  --description "Users report login times out after 30 seconds"
# Created: bd-a1b2c3
```

### 3. Add Labels

```bash
bd label add bd-a1b2c3 backend auth
```

### 4. Check Ready Work

```bash
bd ready
# Shows issues that are open, not blocked, not deferred
```

### 5. Claim and Work

```bash
bd update bd-a1b2c3 --status in_progress --assignee "$(git config user.email)"
```

### 6. Close When Done

```bash
bd close bd-a1b2c3 --reason "Increased timeout to 60s, added retry logic"
```

### 7. Sync to Git

```bash
bd sync --flush-only        # Export DB to JSONL
git add .beads/             # Stage changes
git commit -m "Fix: login timeout (bd-a1b2c3)"
```

---

## Commands

### Issue Lifecycle

| Command | Description | Example |
|---------|-------------|---------|
| `init` | Initialize workspace | `bd init` |
| `create` | Create issue | `bd create "Title" -p 1 --type bug` |
| `q` | Quick capture (ID only) | `bd q "Fix typo"` |
| `show` | Show issue details | `bd show bd-abc123` |
| `update` | Update issue | `bd update bd-abc123 --priority 0` |
| `close` | Close issue | `bd close bd-abc123 --reason "Done"` |
| `reopen` | Reopen closed issue | `bd reopen bd-abc123` |
| `delete` | Delete issue (tombstone) | `bd delete bd-abc123` |

### Querying

| Command | Description | Example |
|---------|-------------|---------|
| `list` | List issues | `bd list --status open --priority 0-1` |
| `ready` | Actionable work | `bd ready` |
| `blocked` | Blocked issues | `bd blocked` |
| `search` | Full-text search | `bd search "authentication"` |
| `stale` | Stale issues | `bd stale --days 30` |
| `count` | Count with grouping | `bd count --by status` |

### Dependencies

| Command | Description | Example |
|---------|-------------|---------|
| `dep add` | Add dependency | `bd dep add bd-child bd-parent` |
| `dep remove` | Remove dependency | `bd dep remove bd-child bd-parent` |
| `dep list` | List dependencies | `bd dep list bd-abc123` |
| `dep tree` | Dependency tree | `bd dep tree bd-abc123` |
| `dep cycles` | Find cycles | `bd dep cycles` |

### Labels

| Command | Description | Example |
|---------|-------------|---------|
| `label add` | Add labels | `bd label add bd-abc123 backend urgent` |
| `label remove` | Remove label | `bd label remove bd-abc123 urgent` |
| `label list` | List issue labels | `bd label list bd-abc123` |
| `label list-all` | All labels in project | `bd label list-all` |

### Comments

| Command | Description | Example |
|---------|-------------|---------|
| `comments add` | Add comment | `bd comments add bd-abc123 "Found root cause"` |
| `comments list` | List comments | `bd comments list bd-abc123` |

### Sync & System

| Command | Description | Example |
|---------|-------------|---------|
| `sync` | Sync DB ↔ JSONL | `bd sync --flush-only` |
| `doctor` | Run diagnostics | `bd doctor` |
| `stats` | Project statistics | `bd stats` |
| `config` | Manage config | `bd config --list` |
| `upgrade` | Self-update | `bd upgrade` |
| `version` | Show version | `bd version` |

### Global Flags

| Flag | Description |
|------|-------------|
| `--json` | JSON output (machine-readable) |
| `--quiet` / `-q` | Suppress output |
| `--verbose` / `-v` | Increase verbosity (-vv for debug) |
| `--no-color` | Disable colored output |
| `--db <path>` | Override database path |

[1]: https://raw.githubusercontent.com/Dicklesworthstone/beads_rust/refs/heads/main/README.md
