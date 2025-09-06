#!/usr/bin/env bash

set -euo pipefail

# Install PipeWire stack and tools
sudo pacman -Syu --needed pipewire pipewire-pulse pipewire-jack pipewire-alsa wireplumber pavucontrol pamixer

echo "PipeWire stack installed."


