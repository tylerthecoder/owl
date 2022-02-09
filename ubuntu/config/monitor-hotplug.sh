#!/bin/bash

logFile="/var/log/monitor-hotplug.log"

touch "$logFile"


for output in HDMI-A-1 HDMI-A-2; do

  STATUS=/sys/class/drm/card0-$output/status

	if [ STATUS = connected ]; then
		xrandr --output eDP-1 --primary --mode 1920x1080 --pos 0x0 --rotate normal --output DP-1 --off --output HDMI-1 --off --output DP-2 --off --output HDMI-2 --mode 2560x1440 --pos 1920x0 --rotate normal
	fi
done