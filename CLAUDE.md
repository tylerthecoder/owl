# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Owl is a modular dotfiles and environment management CLI written in Rust. It manages configurations across different machines through:
- **Setups**: Modular units that handle software configuration (in `setups/`)
- **Nests**: Machine-specific environments that bundle setups together (in `nests/`)
- **Common**: Shared configurations and scripts (in `common/`)

## Development Commands

### Building and Running
```bash
cargo build                    # Build the project
cargo run -- <args>           # Run with arguments
cargo build --release         # Production build

# Example: Test nest linking
cargo run -- nest link
cargo run -- setup git info
```

### Testing Changes
```bash
# Install the built binary
cargo build && cargo run -- nest link

# The binary is linked to ~/.local/bin/owl via nest link
# After linking, you can use `owl` directly
```

### Validation
```bash
owl setups-validate           # Validate all setup.json files
```

## Architecture

### Core Data Model

The codebase follows a **validate-first pattern**:

1. **SetupFile** (`setup.json`): Raw JSON with all optional fields
   - Parsed directly from disk using serde
   - All fields are `Option<T>` to handle missing values

2. **Setup**: Validated, resolved in-memory representation
   - Created by validating a `SetupFile`
   - All paths are resolved (no more `local:` or `common:` tokens)
   - Used for all operations (link, install, systemd)

3. **Validated types**: Specific structs for each component
   - `ValidatedSetupLink`: Resolved source/target paths with root flag
   - `ValidatedRunScript`: RC scripts with resolved paths
   - `ValidatedSetupMenuScriptItem`: Menu scripts with paths and names
   - `ValidatedSetupService`: Services with resolved unit files and types

### Path Resolution

The system uses path tokens that are resolved during validation:

- `local:<path>` → Relative to the setup directory (e.g., `setups/<name>/...`)
- `common:<path>` → Relative to `common/` in the repo root
- `~` → Expands to user home directory
- Absolute paths pass through as-is

Example:
```json
{
  "source": "local:config/sway.conf",      // → setups/sway/config/sway.conf
  "source": "common:config/.vimrc",        // → common/config/.vimrc
  "source": "~/custom/.zshrc"              // → /home/user/custom/.zshrc
}
```

### Setup System

**setup.json schema** (all fields optional):
```json
{
  "name": "git",
  "dependencies": ["base-shell"],
  "install": "local:install.sh",
  "links": [
    {
      "source": "local:gitconfig",
      "target": "~/.gitconfig",
      "root": false
    }
  ],
  "rc_scripts": ["common:git-aliases.sh", "local:rc.sh"],
  "menu_scripts": ["local:menu-helper.sh"],
  "services": [
    {
      "path": "local:my-service.service",
      "service_type": "User"
    }
  ]
}
```

**Key concepts:**
- **dependencies**: Other setups to install first (recursive)
- **links**: Files/directories to symlink to target locations
- **rc_scripts**: Shell scripts sourced at startup (linked to `~/.config/owl/rc/`)
- **menu_scripts**: Scripts for dmenu/rofi (linked to `~/.config/owl/menu-scripts/`)
- **services**: Systemd units to link and enable (User or System scope)

### Nest System

Nests are root setups in `nests/` that define complete machine environments. They use the same `setup.json` format but typically:
- Declare many dependencies
- Include machine-specific links and rc_scripts
- Use `local:` paths relative to the nest directory

The active nest is tracked in `~/.config/owl/config.json` as `nest_path`.

### CLI Structure

Entry point: `src/main.rs`

Main command groups:
- **setup**: Operations on individual setups
  - `owl setup <name> link|install|systemd|info|edit|all [--shallow]`
- **nest**: Operations on the active nest (shorthand for root setup)
  - `owl nest link|install|systemd|info|edit|switch|all [--shallow]`
- **system**: Configuration and maintenance
  - `owl config`: Show current config
  - `owl sync`: Sync repository (fetch, fast-forward, optional push)
  - `owl update [--recursive]`: Update owl binary
  - `owl setups-validate`: Validate all setups

The `--shallow` flag prevents recursive dependency processing.

## Code Conventions

### Rust Patterns
- Keep all `setup.json` fields optional - validation happens separately
- Use free functions over large impls for CLI orchestration
- Fail fast at validation time with clear, user-friendly errors
- Print structured progress with emojis and colors during operations
- Remove dead code; prefer `quiet: bool` parameters over duplicate functions

### Shell Scripts
- Use `set -euo pipefail` unless interactive
- Quote variable expansions: `"${VAR}"`
- Make install scripts idempotent (check before installing)
- Scripts receive resolved absolute paths (no `local:` or `common:` tokens)

### Path Resolution
- Use the single unified resolver for all path token expansion
- All validated types store resolved `PathBuf`s
- Never operate on raw string paths from JSON

## Common Setup Patterns

### Adding a new setup
1. Create `setups/<name>/setup.json`
2. Add configuration files to the setup directory
3. Use `local:` for files in the setup, `common:` for shared files
4. Define dependencies on other setups if needed
5. Run `owl setups-validate` to check correctness

### Adding to a nest
Edit `nests/<machine>/setup.json` and add the setup name to `dependencies`.

### Service management
Services are linked during `link` and enabled/started during `systemd`:
- User services → `~/.config/systemd/user/`
- System services → `/etc/systemd/system/` (requires sudo)

### Debugging
- Use `owl setup <name> info` to see what would be linked
- Check `~/.config/owl/config.json` for active configuration
- RC scripts are in `~/.config/owl/rc/` as `rc-<setup>-<filename>`
- Menu scripts are in `~/.config/owl/menu-scripts/`

## Omni-Menu

Omni-menu is a GTK4-based application launcher integrated into owl as a second binary. It provides a unified interface for common desktop tasks.

### Architecture

**Location**: `src/omni_menu/`

**Binary**: Built as `omni-menu` alongside the `owl` binary in Cargo.toml

**Modules**:
- `main.rs` - Entry point with subcommand routing
- `main_menu.rs` - Main menu UI with keyboard shortcuts
- `search_menu.rs` - Web search (Google, ChatGPT, Notes)
- `projects_menu.rs` - Local/remote dev project launcher
- `scripts_menu.rs` - Owl scripts menu
- `launch_tool_menu.rs` - Launch tools in current workspace
- `switch_bench_menu.rs` - Yard bench switcher
- `desk_menu.rs` - Owl desk configuration switcher
- `emoji_menu.rs` - Emoji picker (rofi wrapper)
- `utils.rs` - Shared utilities for list population and filtering

### Usage

```bash
omni-menu              # Main menu (default)
omni-menu search       # Web search
omni-menu projects     # Project launcher
omni-menu scripts      # Owl scripts
omni-menu launch_tool  # Launch tools
omni-menu switch_bench # Switch yard bench
omni-menu desk         # Switch desk configuration
omni-menu emoji        # Pick emoji
```

### Integration with Owl

- **Desk Integration**: Detects Sway/i3 via `SWAYSOCK` env var, lists desk scripts from `~/.config/desks-sway/` or `~/.config/desks-i3/`
- **Scripts Integration**: Reads from owl's menu scripts directory
- **Setup**: Linked via `setups/menu/setup.json` to `~/.local/bin/omni-menu`

### Building

```bash
cargo build --bin omni-menu    # Build omni-menu binary
cargo build                     # Build both owl and omni-menu
owl setup menu link             # Link binary to ~/.local/bin
```

### Design Patterns

- Each menu module is self-contained with its own GTK4 application
- Modules follow the `pub mod <name> { pub fn run_app() -> glib::ExitCode }` pattern
- Main menu spawns submenus by re-invoking itself with different arguments
- Utilities module provides shared list filtering and fuzzy matching functionality
