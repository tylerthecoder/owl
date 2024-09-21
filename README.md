# OWL

This repository contains all of my dotfiles, scripts, and configs I use across different computers and operating systems.

## Quick start

Download and run the setup script.
```
curl https://raw.githubusercontent.com/tylerthecoder/owl/main/setups/owl/setup.sh | sh
```

Link files. This will ask to select a nest file.
```
owl link
```

Setup software
```
owl setup base-shell
owl setup zsh
owl setup python
```

## Getting Started

Install the app
```
./setups/owl.sh
```

Build the app
```
cargo build
```

Perform first link
```
cargo run -- link

```

## Config
Config is stored in `~/.config/owl/config.json`

**owl_path**: The location of on your machine of this repository.
**nest_path**: The location of a nest file. Contains a list of files or link and programs to set up.


