#!/usr/bin/env bash
set -euo pipefail
choice=$(cliphist list | rofi -dmenu -i -p "Clipboard")
[[ -z "${choice:-}" ]] && exit 0
echo "$choice" | cliphist decode | wl-copy
