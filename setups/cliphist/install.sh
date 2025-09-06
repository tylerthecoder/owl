#!/usr/bin/env bash
set -euo pipefail
sudo pacman -Syu --needed wl-clipboard cliphist rofi-wayland
systemctl --user daemon-reload || true
systemctl --user enable --now cliphist.service || true
echo "cliphist installed"


