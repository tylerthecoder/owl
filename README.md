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
owl nest setup
```

1. Setup software:

```bash
owl setup git
owl setup zsh
owl setup rust
```

## Architecture

### Nests (`nests/`)

Each nest represents a machine configuration with:

- **links**: File symlinks from owl to system locations
- **setups**: List of setup modules to install/configure
- **rc_scripts**: Common run scripts to load during shell initialization

Example nest structure:

```json
{
  "links": [
    {
      "source": "common/config/.vimrc",
      "target": "~/.vimrc"
    },
    {
      "source": "local:.xprofile",
      "target": "~/.xprofile"
    }
  ],
  "setups": ["git", "zsh", "rust"],
  "rc_scripts": [
    "common:fzf.sh",
    "common:base-aliases.sh",
    "local:.shenv"
  ]
}
```

### Setups (`setups/`)

Modules that handle software installation and configuration:

- **setup.json**: Defines optional fields for a setup
  - `name` (string)
  - `links` (array of { source, target, root? })
  - `rc_scripts` (array of strings; supports `common:` and `local:`)
  - `actions` (array of scripts to run during linking)
  - `install` (string path to install script)
  - `services` (array of { path, type } where type is `user` or `system`)
  - `dependencies` (array of setup names)
- **install.sh**: Installation script with OS detection
- **rc_scripts**: Shell scripts that get loaded per setup

### RC Scripts (`common/rc/`)

Reusable shell scripts for environment configuration:

- `fzf.sh`: FZF fuzzy finder configuration
- `git-aliases.sh`: Git aliases and functions
- `base-aliases.sh`: Common aliases used across machines
- `bun.sh`: Bun runtime environment variables

### Path Syntax

Nest and setup configurations support clean path syntax:

- `common:filename.sh` → `common/rc/filename.sh`
- `local:filename` → `nests/{nest-name}/filename`
- Regular absolute/relative paths work as before; if no token is provided and the path is relative, it resolves from the owl repo root (`owl_path`).

### Initialization (`owl-start.sh`)

The simplified startup script that:

1. Sets XDG environment variables
2. Adds local bin to PATH
3. Sources all scripts from `~/.config/owl/rc/`
4. Sources machine-specific environment from `~/.shenv`

## Commands

### Nest Management

- `owl nest setup`: Link nest files and setup configurations (and services)
- `owl nest install`: Run install scripts with dependency resolution
- `owl nest info`: Show what files would be linked (dry run)
- `owl nest switch [NAME]`: Switch the active nest by name (interactive if omitted)

### Setup Management

- `owl setup <name>`: Link and run actions for a specific setup (not install)
- `owl setups`: Interactive UI to view/edit/install setups

### System

- `owl config`: Show current configuration
- `owl sync`: Run synchronization scripts
- `owl update`: Update owl itself

## Configuration

Config stored in `~/.config/owl/config.json`:

- **owl_path**: Location of this repository
- **nest_path**: Path to your nest.json file

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
~/.local/bin/owl nest switch framework-sway
```

1. Link configs and services:

```bash
owl nest setup
```

1. Install software (with dependencies):

```bash
owl nest install
```

## Local Development

Build and test:

```bash
cargo build
cargo run -- nest setup
```
