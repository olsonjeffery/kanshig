# AGENTS.md — beads_rust (br)

> Guidelines for AI coding agents working in this Rust codebase.

---

## RULE 0 - THE FUNDAMENTAL OVERRIDE PREROGATIVE

If I tell you to do something, even if it goes against what follows below, YOU MUST LISTEN TO ME. I AM IN CHARGE, NOT YOU.

---

## RULE NUMBER 1: NO FILE DELETION

**YOU ARE NEVER ALLOWED TO DELETE A FILE WITHOUT EXPRESS PERMISSION.** Even a new file that you yourself created, such as a test code file. You have a horrible track record of deleting critically important files or otherwise throwing away tons of expensive work. As a result, you have permanently lost any and all rights to determine that a file or folder should be deleted.

**YOU MUST ALWAYS ASK AND RECEIVE CLEAR, WRITTEN PERMISSION BEFORE EVER DELETING A FILE OR FOLDER OF ANY KIND.**

---

## Irreversible Git & Filesystem Actions — DO NOT EVER BREAK GLASS

1. **Absolutely forbidden commands:** `git reset --hard`, `git clean -fd`, `rm -rf`, or any command that can delete or overwrite code/data must never be run unless the user explicitly provides the exact command and states, in the same message, that they understand and want the irreversible consequences.
2. **No guessing:** If there is any uncertainty about what a command might delete or overwrite, stop immediately and ask the user for specific approval. "I think it's safe" is never acceptable.
3. **Safer alternatives first:** When cleanup or rollbacks are needed, request permission to use non-destructive options (`git status`, `git diff`, `git stash`, copying to backups) before ever considering a destructive command.
4. **Mandatory explicit plan:** Even after explicit user authorization, restate the command verbatim, list exactly what will be affected, and wait for a confirmation that your understanding is correct. Only then may you execute it—if anything remains ambiguous, refuse and escalate.
5. **Document the confirmation:** When running any approved destructive command, record (in the session notes / final response) the exact user text that authorized it, the command actually run, and the execution time. If that record is absent, the operation did not happen.

---

## Git Branch: ONLY Use `main`, NEVER `master`

**The default branch is `main`. The `master` branch exists only for legacy URL compatibility.**

- **All work happens on `main`** — commits, PRs, feature branches all merge to `main`
- **Never reference `master` in code or docs** — if you see `master` anywhere, it's a bug that needs fixing
- **The `master` branch must stay synchronized with `main`** — after pushing to `main`, also push to `master`:

  ```bash
  git push origin main:master
  ```

**If you see `master` referenced anywhere:**

1. Update it to `main`
2. Ensure `master` is synchronized: `git push origin main:master`

---

## Toolchain: Rust & Cargo

We only use **Cargo** in this project, NEVER any other package manager.

- **Edition:** Rust 2024 (nightly required — see `rust-toolchain.toml`)
- **Dependency versions:** Explicit versions for stability
- **Configuration:** Cargo.toml at the workspace-level with crate dependencies and versions. Each sub-folder that contains a rust crate contains its own Cargo.toml. All sub-crates in the workspace use `{workspace = true}` to specify crate version
- **Unsafe code:** Forbidden (`#![forbid(unsafe_code)]` via crate lints)

## Code Editing Discipline

### Use of beads_rs (bd)

Read the [BEADS.md](./BEADS.md) to learn how to use `beads_rs`/`bd`, the tool that drives asynchronous communication & issue tracking.

### No Script-Based Changes

NEVER run a script that processes/changes code files in this repo. Brittle regex-based transformations create far more problems than they solve.

- Always make code changes manually, even when there are many instances
- For many simple changes: use parallel subagents
- For subtle/complex changes: do them methodically yourself

### No File Proliferation

If you want to change something or add a feature, revise existing code files in place.

NEVER create variations like:

- mainV2.rs
- main_improved.rs
- main_enhanced.rs

New files are reserved for genuinely new functionality that makes zero sense to include in any existing file. The bar for creating new files is incredibly high.

## Compiler Checks (CRITICAL)

After any substantive code changes, you MUST verify no errors were introduced:

```bash
# Check for compiler errors and warnings
cargo check --all-targets

# Check for clippy lints (pedantic + nursery are enabled)
cargo clippy --all-targets -- -D warnings

# Verify formatting
cargo fmt --check

If you see errors, carefully understand and resolve each issue. Read sufficient context to fix them the RIGHT way.
```

## Testing

### Testing Policy

Every module includes inline `#[cfg(test)]` unit tests alongside the implementation. Tests must cover:

- Happy path
- Edge cases (empty input, max values, boundary conditions)
- Error conditions

Integration and end-to-end tests live in the `tests/` directory.

### Unit Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run tests for a specific module
cargo test storage
cargo test cli
cargo test sync
cargo test format
cargo test model
cargo test validation

# Run tests with all features enabled
cargo test --all-features
```

### Test Categories

| Directory / Pattern | Focus Areas |
|---------------------|-------------|
| `src/` (inline `#[cfg(test)]`) | Unit tests for each module: model, storage, sync, config, error, format, util, validation |
| `tests/e2e_*.rs` | End-to-end CLI tests: lifecycle, labels, deps, sync, history, search, comments, epics, workspaces, errors, completions |
| `tests/conformance*.rs` | Go/Rust parity: schema compatibility, text output matching, edge cases, labels+comments, workflows |
| `tests/storage_*.rs` | Storage layer: CRUD, list filters, ready queries, deps, history, blocked cache, export atomicity, invariants, ID/hash parity |
| `tests/proptest_*.rs` | Property-based tests: ID generation, hash determinism, time parsing, validation rules |
| `tests/repro_*.rs` | Regression tests: specific bugs reproduced and prevented |
| `tests/jsonl_import_export.rs` | JSONL round-trip fidelity |
| `tests/markdown_import.rs` | Markdown import parsing |
| `benches/storage_perf.rs` | Storage operation benchmarks (criterion) |

### Test Fixtures

Shared test fixtures live in `tests/fixtures/` and `tests/common/` for reusable test harness helpers (temp DB creation, test data builders).

---
