#!/bin/sh

# Set the cursor speed (wait time, dups per second)
xset r rate 300 35

# Disable the touch pad
# xinput --disable 16

xinput --map-to-output 'Wacom Pen and multitouch sensor Finger' "eDP-1-1"

setxkbmap -option caps:super

# Make nvidia not screen tear
nvidia-settings --assign CurrentMetaMode="nvidia-auto-select +0+0 { ForceFullCompositionPipeline = On }"

xrandr --output DP-0 --primary --mode 3840x2160 --pos 0x0 --rotate normal --output DP-1 --mode 3840x2160 --pos 3840x0 --rotate right --output HDMI-0 --off --output eDP-1-1 --mode 3840x2160 --pos 0x2160 --rotate normal

# set the background image
set-bg-to-nasa-iotd.sh
