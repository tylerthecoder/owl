#!/usr/bin/env bash
set -euo pipefail

sudo pacman -Syu --needed playerctl

systemctl --user daemon-reload || true
systemctl --user enable --now playerctld.service || true

echo "playerctl installed and playerctld service enabled."


