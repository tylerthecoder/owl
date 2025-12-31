#!/bin/bash
set -euo pipefail

if command -v zoxide &>/dev/null; then
    echo "zoxide is already installed. Skipping installation."
else
    echo "Installing zoxide..."
    sudo pacman -S --noconfirm zoxide
fi

echo "zoxide setup completed!"
echo "zoxide version: $(zoxide --version)"
