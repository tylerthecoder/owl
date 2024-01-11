# OWL

This repository contains all of my dotfiles, scripts, and configs I use across different computers and operating systems. 


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

### Important environment variable

**OWL_PATH**: The location of on your machine of this repository. 

**OWL_CONFIG_PATH**: The location of an owl config. Contains a list of files or link and programs to set up.

## Terms
**Link files** are json config files that map files in this repo to their location on the system.

Desks are organizations of hardware.


## Ideas
- Setups should have a file that defines their links. Then each user has an "owl config" file that points to setups and has its own links. Then owl link loops through all the setups and links their files

