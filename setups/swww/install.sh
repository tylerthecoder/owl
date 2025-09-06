#!/usr/bin/env bash
set -euo pipefail
if ! command -v yay >/dev/null 2>&1; then
  echo "Please install yay to install swww from AUR"
  exit 1
fi
yay -S --needed swww
systemctl --user daemon-reload || true
systemctl --user enable --now bg-rotate.timer || true
echo "swww installed"


