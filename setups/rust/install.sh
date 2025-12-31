#!/bin/bash

# Set XDG_DATA_HOME default before sourcing rc.sh
export XDG_DATA_HOME="${XDG_DATA_HOME:-$HOME/.local/share}"

# Source the rc.sh file to set up environment variables
SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/rc.sh"

# Skip installation if rust is already installed
if command -v rustc &> /dev/null; then
    echo "Rust is already installed. Skipping installation."
else
    # Detect the operating system and install rustup accordingly
    if command -v pacman &> /dev/null; then
        # Arch Linux
        echo "Detected Arch Linux, installing rustup via pacman..."
        sudo pacman -Sy rustup
    elif command -v apt &> /dev/null; then
        # Ubuntu/Debian
        echo "Detected Ubuntu/Debian, installing rustup via official installer..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    else
        echo "Unsupported operating system. Please install rustup manually."
        exit 1
    fi
fi

rustup default stable

