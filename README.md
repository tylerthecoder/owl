# OWL

A modular dotfiles and environment management system that allows you to manage configurations, setups, and run scripts across different machines and environments.

## Terms

- **Nest**: A configuration that represents a specific machine or environment (e.g., laptop, server, development machine). Contains links, setups, and run scripts.
- **Setup**: A modular package that can install software and manage its configuration (e.g., git, zsh, rust, nvim).
- **RC Scripts**: Shell scripts that get sourced during shell initialization to configure environment variables, aliases, and functions.

## Quick Start

1. Download and run the setup script:

```bash
curl https://raw.githubusercontent.com/tylerthecoder/owl/main/setups/owl/setup.sh | sh
```

1. Set up your nest configuration:

```bash
owl nest link
```

1. Setup software:

```bash
owl setup git
owl setup zsh
owl setup rust
```

## Architecture

### Nests (`nests/`)

Each nest is just a root setup: it uses the same `setup.json` schema as any other setup, and typically declares other setups via `dependencies`.

Example nest `setup.json`:

```json
{
  "links": [
    { "source": "common:config/.vimrc", "target": "~/.vimrc" },
    { "source": "local:.xprofile", "target": "~/.xprofile" }
  ],
  "dependencies": ["git", "zsh", "rust"],
  "rc_scripts": ["common:fzf.sh", "common:base-aliases.sh", "local:.shenv"]
}
```

### Setups (`setups/`)

Modules that handle software installation and configuration:

- **setup.json**: Defines optional fields for a setup
  - `name` (string)
  - `links` (array of { source, target, root? })
  - `rc_scripts` (array of strings; supports `common:` and `local:`)
  - `install` (string path to install script)
  - `services` (array of { path, type } where type is `user` or `system`; daemon-reload is triggered automatically when linking services)
  - `dependencies` (array of setup names)
- **install.sh**: Installation script with OS detection
- **rc_scripts**: Shell scripts that get loaded per setup

### RC Scripts (`common/rc/`)

Reusable shell scripts for environment configuration:

- `fzf.sh`: FZF fuzzy finder configuration
- `git-aliases.sh`: Git aliases and functions
- `base-aliases.sh`: Common aliases used across machines
- `bun.sh`: Bun runtime environment variables

Naming and destination:

- Linked as `rc-<setup>-<script-file>` and placed in `~/.config/owl/rc/`.

### Path Syntax

Nest and setup configurations support tokenized paths with context:

- rc_scripts: `common:filename.sh` → `common/rc/filename.sh`
- menu_scripts: `common:emoji.sh` → `common/menu-scripts/emoji.sh`
- services: `common:greenclip.service` → `common/services/greenclip.service`
- links: `common:path/inside/common` → `common/path/inside/common`
- `local:relative/path` → relative to the setup directory (either `setups/<name>` or `nests/<name>`, depending on context)
- Relative paths without tokens resolve from the repo root (`owl_path`)

All source paths are validated to exist during setup validation; target paths need not exist and will be created as needed.

### Initialization (`owl-start.sh`)

The simplified startup script that:

1. Sets XDG environment variables
2. Adds local bin to PATH
3. Sources all scripts from `~/.config/owl/rc/`
4. Sources machine-specific environment from `~/.shenv`

## Commands

### Nest Management

- `owl nest link [--shallow]`: Link files, rc scripts, menu scripts, and services
- `owl nest install [--shallow]`: Run install scripts with dependency resolution
- `owl nest systemd [--shallow]`: Link and enable/restart services
- `owl nest info [--shallow]`: Show what would be linked
- `owl nest edit`: Open the active root setup for editing
- `owl nest switch`: Switch the active nest interactively

### Setup Management

- `owl setup <name> <link|install|systemd|info|edit|all> [--shallow]`

### System

- `owl config`: Show current configuration
- `owl sync`: Run synchronization scripts
- `owl setups-validate`: Validate all setups and nests
- `owl update`: Update owl itself

Note: owl is designed for interactive use. Prompts during first-run config are expected.

## Conventions

The project follows a validate-first pattern and path tokens as described below.

## Configuration

Config stored in `~/.config/owl/config.json`:

- **owl_path**: Location of this repository
- **nest_path**: Path to your active root setup directory (e.g., `nests/<name>`)

## Quick Start on a new machine

1. Install dependencies (Arch example):

```bash
sudo pacman -S --needed git base-devel curl
```

1. Clone owl and set the nest:

```bash
git clone https://github.com/tylerthecoder/owl ~/owl
cd ~/owl
cargo build --release
~/.local/bin/owl nest switch
```

1. Link configs and services:

```bash
owl nest link
```

1. Install software (with dependencies):

```bash
owl nest install
```

## Local Development

Build and test:

```bash
cargo build
cargo run -- nest link
```
