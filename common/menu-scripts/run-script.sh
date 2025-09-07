#!/usr/bin/env bash
set -euo pipefail
script=$(ls ~/owl/common/scripts | rofi -dmenu -i -p "Select script")
[[ -z "${script:-}" ]] && exit 0
exec "~/owl/common/scripts/$script"
