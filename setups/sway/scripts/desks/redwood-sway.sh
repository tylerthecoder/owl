#!/usr/bin/env bash
set -euo pipefail

# External big screen only at 3840x2160
swaymsg output eDP-1 disable
swaymsg output DP-2 enable mode 3840x2160 position 0 0

# Keyboard settings
swaymsg input type:keyboard repeat_delay 300
swaymsg input type:keyboard repeat_rate 35
swaymsg input type:keyboard xkb_options caps:super

