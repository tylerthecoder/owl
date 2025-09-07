#!/usr/bin/env bash
set -euo pipefail

# Internal display only, 2880x1920
# Disable all other outputs
for out in $(swaymsg -t get_outputs | jq -r '.[].name'); do
  if [ "$out" != "eDP-1" ]; then
    swaymsg output "$out" disable
  fi
done

swaymsg output eDP-1 enable mode 2880x1920 position 0 0

# Key repeat and caps as super in Wayland
swaymsg input type:keyboard repeat_delay 300
swaymsg input type:keyboard repeat_rate 35
swaymsg input type:keyboard xkb_options caps:super

