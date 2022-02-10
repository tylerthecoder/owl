#!/bin/bash
set -x

# Setup displays
xrandr --auto
xrandr --output eDP-1 --left-of HDMI-2

# Keyboard go brrrr
xset r rate 300 35
xinput --map-to-output 'Wacom Pen and multitouch sensor Finger' "eDP-1-1"

setxkbmap -option caps:super

# Disable the trackpad
xinput --disable "Synaptics TM3276-031"
