#!/bin/bash
set -x

# Setup displays
xrandr --output eDP-1 --off --output DP-1 --off --output HDMI-1 --off --output DP-2 --off --output HDMI-2 --mode 2560x1440 --pos 0x0 --rotate normal

# Keyboard go brrrr
xset r rate 300 35

setxkbmap -option caps:super

# Disable the trackpad
xinput --disable "Synaptics TM3276-031"
