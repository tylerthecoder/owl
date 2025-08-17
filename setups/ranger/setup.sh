#!/bin/bash

# Skip installation if ranger is already installed
if command -v ranger &> /dev/null; then
    echo "Ranger is already installed. Skipping installation."
else
    # Detect the operating system and install ranger accordingly
    if command -v pacman &> /dev/null; then
        # Arch Linux
        echo "Detected Arch Linux, installing ranger via pacman..."
        sudo pacman -S --needed ranger
    elif command -v apt &> /dev/null; then
        # Ubuntu/Debian
        echo "Detected Ubuntu/Debian, installing ranger via apt..."
        sudo apt update && sudo apt install -y ranger
    elif command -v dnf &> /dev/null; then
        # Fedora
        echo "Detected Fedora, installing ranger via dnf..."
        sudo dnf install -y ranger
    elif command -v zypper &> /dev/null; then
        # openSUSE
        echo "Detected openSUSE, installing ranger via zypper..."
        sudo zypper install -y ranger
    elif command -v brew &> /dev/null; then
        # macOS with Homebrew
        echo "Detected macOS with Homebrew, installing ranger..."
        brew install ranger
    else
        echo "Unsupported operating system. Please install ranger manually."
        echo "You can usually install it with your package manager or via pip:"
        echo "  pip install ranger-fm"
        exit 1
    fi
fi

echo "Ranger setup completed successfully!"
