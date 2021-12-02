#!/bin/bash


echo "$1"

numRe='^[+-]?[0-9]+$'

if ! [[ $1 =~ $numRe ]] ; then
  echo "Error: first arg must be a number">&2; exit 1
fi

currentBrightness=$(cat /sys/class/backlight/intel_backlight/brightness);
maxBrightness=$(cat /sys/class/backlight/intel_backlight/max_brightness);


currentBrightness=$((currentBrightness + $1))

if [ "$currentBrightness" -gt "$maxBrightness" ]; then
  currentBrightness=$maxBrightness
fi

brightnessPercent=$((($currentBrightness * 100) / $maxBrightness))

notify-send "Brightness set to $brightnessPercent%"

echo "$currentBrightness" | tee /sys/class/backlight/intel_backlight/brightness


