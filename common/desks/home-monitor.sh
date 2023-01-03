#!/bin/sh
xrandr --output eDP-1 --primary --mode 1920x1080 --pos 0x739 --rotate normal --output DP-1 --mode 1920x1080 --pos 1920x200 --rotate normal --output HDMI-1 --off --output DP-2 --off --output HDMI-2 --mode 1360x768 --pos 3840x0 --rotate right

# Keyboard go brrrr
xset r rate 300 35

# The REAL super key
setxkbmap -option caps:super