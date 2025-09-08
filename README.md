# OWL

A modular dotfiles and environment management system that allows you to manage configurations, setups, and run scripts across different machines and environments.

## Quick Start

```bash
curl https://raw.githubusercontent.com/tylerthecoder/owl/main/setups/owl/setup.sh | sh
```

This installs owl to `~/owl`

- Then it runs `owl nest all` which will prompt you to select a nest and then it will install the nest to you computer.

The machine is now ready. (might need a reboot)

## Architecture

### Setups

Stored in `setups/`,

Modules that handle software installation and configuration:

- **setup.json**: Defines optional fields for a setup
  - `name` (string)
  - `links` (array of { source, target, root? })
  - `rc_scripts` (array of strings; supports `common:` and `local:`)
  - `install` (string path to install script)
  - `services` (array of { path, type } where type is `user` or `system`; daemon-reload is triggered automatically when linking services)
  - `dependencies` (array of setup names)

### Nests

Stored in `nests/`, nests are collections of setups that define a complete environment for a specific machine or purpose.

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

### Links

Files that are symlinked from the repository to the home directory or system locations. They can be defined in setups or nests.

How owl uses it:

- Created during `owl setup <name> link` or `owl nest link`.
- Existing targets are safely replaced (symlinks/files removed; non-empty directories are refused).
- If `root: true` is set for a link, owl will create parent directories and the symlink with `sudo`.

How to specify in `setup.json`:

```json
{
  "links": [
    { "source": "common:config/.vimrc", "target": "~/.vimrc" },
    { "source": "local:kitty.conf", "target": "~/.config/kitty/kitty.conf" },
    { "source": "setups/git/gitconfig", "target": "~/.gitconfig" }
  ]
}
```

Path tokens:

- `common:<path>` → `common/<path>` in the repo
- `local:<path>` → relative to the setup directory
- paths without tokens resolve from the repo root (`owl_path`)

### RC Scripts

Shell scripts that are sourced during shell initialization. They can be defined in setups or nests.

How owl uses it:

- Linked into `~/.config/owl/rc/` as `rc-<setup>-<filename>` during `link`.
- `owl-start.sh` sources all scripts in `~/.config/owl/rc/`.

How to specify in `setup.json`:

```json
{
  "rc_scripts": [
    "common:fzf.sh",
    "common:base-aliases.sh",
    "local:rc.sh"
  ]
}
```

Path tokens:

- `common:<file>` → `common/rc/<file>`
- `local:<file>` → relative to the setup directory

### Menu Scripts

Scripts that appear in application menus (like rofi or dmenu). They can be defined in setups or nests.

How owl uses it:

- Linked into `~/.config/owl/menu-scripts/`. Simple entries use the filename; detailed entries can set a custom `name`.

How to specify in `setup.json`:

```json
{
  "menu_scripts": [
    "common:emoji.sh",
    { "path": "local:cliphist.sh", "name": "clipboard" }
  ]
}
```

Path tokens:

- `common:<file>` → `common/menu-scripts/<file>`
- `local:<file>` → relative to the setup directory

### Services

Systemd unit files to link and enable.

How owl uses it:

- Owl links service files to the appropriate systemd directory.
- During `systemd`, owl enables and starts them (and triggers daemon-reload as part of enable).

How to specify in `setup.json`:

```json
{
  "services": [
    { "path": "common:greenclip.service", "type": "user" },
    { "path": "local:my-daemon.service", "type": "system" }
  ]
}
```

Path tokens:

- `common:<file>` → `common/services/<file>`
- `local:<file>` → relative to the setup directory

Type values:

- `user` (default) → `~/.config/systemd/user`
- `system` → `/etc/systemd/system` (requires sudo)

### Initialization (`owl-start.sh`)

The simplified startup script that:

1. Sets XDG environment variables
2. Adds local bin to PATH
3. Sources all scripts from `~/.config/owl/rc/`
4. Sources machine-specific environment from `~/.shenv`

## Commands

### Nest Commands

- `owl nest link [--shallow]`: Link files, rc scripts, menu scripts, and services
- `owl nest install [--shallow]`: Run install scripts with dependency resolution
- `owl nest systemd [--shallow]`: Link and enable/restart services
- `owl nest info [--shallow]`: Show what would be linked
- `owl nest edit`: Open the active root setup for editing
- `owl nest switch`: Switch the active nest interactively

### Setup Commands

- `owl setup <name> <link|install|systemd|info|edit|all> [--shallow]`

### System Commands

- `owl config`: Show current configuration
- `owl sync`: Sync repository (fetch, fast-forward, and optionally push changes)
- `owl setups-validate`: Validate all setups and nests
- `owl update [--recursive]`: Update owl itself. Uses the `setups/owl` install script.

## Configuration

Config stored in `~/.config/owl/config.json`:

- **owl_path**: Location of this repository
- **nest_path**: Path to your active root setup directory (e.g., `nests/<name>`)

## Local Development

Build and test:

```bash
cargo build
cargo run -- nest link
```
