#!/usr/bin/env bash
set -euo pipefail
sudo pacman -Syu --needed waybar jq pamixer upower lm_sensors bluez bluez-utils blueman
echo "waybar installed"


