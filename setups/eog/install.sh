#!/usr/bin/env bash

set -euo pipefail

# Install Eye of GNOME
sudo pacman -Syu --needed eog

# Make it the default image viewer
xdg-mime default org.gnome.eog.desktop image/png image/jpeg image/jpg image/gif image/bmp image/svg+xml image/webp image/x-icon image/tiff image/x-tiff

