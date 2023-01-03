#!/bin/sh
xrandr --output eDP-1 --primary --mode 1920x1080 --pos 0x0 --rotate normal --output DP-1 --off --output HDMI-1 --off --output DP-2 --off --output HDMI-2 --off

# Keyboard go brrrr
xset r rate 300 35

# The REAL super key
setxkbmap -option caps:super

# Disable the trackpad
xinput --disable "Synaptics TM3276-031"
