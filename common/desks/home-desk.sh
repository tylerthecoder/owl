#!/bin/bash
set -x

# Just the BIG screen
xrandr --output eDP-1 --off --output DP-1 --off --output DP-2 --off --output DP-3 --mode 2560x1440 --pos 0x0 --rotate normal --output DP-4 --off

# Keyboard go brrrr
xset r rate 300 35

# The REAL super key
setxkbmap -option caps:super