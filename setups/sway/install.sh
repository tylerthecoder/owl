#!/usr/bin/env bash

set -euo pipefail

# Install SwayFX core and essentials
sudo pacman -Syu --needed base-devel swaybg swayidle swaylock brightnessctl grim slurp jq python notify-send

# Install SwayFX and rofi-wayland from AUR
if ! command -v yay >/dev/null 2>&1; then
  echo "Please install yay or your preferred AUR helper, then re-run this setup."
  exit 1
fi

yay -S --needed swayfx rofi-wayland

echo "SwayFX environment installed. Configure LightDM to offer sway session: sway.desktop is installed under /usr/share/wayland-sessions."



