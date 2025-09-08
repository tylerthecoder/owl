#!/usr/bin/env bash
set -euo pipefail

# External big screen only at 2560x1440
# Adjust output name (e.g., DP-1/HDMI-A-1) to match swaymsg -t get_outputs
swaymsg output eDP-1 disable
swaymsg output DP-3 enable mode 2560x1440 position 0 0

# Keyboard settings
swaymsg input type:keyboard repeat_delay 300
swaymsg input type:keyboard repeat_rate 35
swaymsg input type:keyboard xkb_options caps:super
