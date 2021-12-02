#!/bin/bash

cat /usr/share/dict/american-english | dmenu | xclip -selection clipboard

notify-send "Copied to clipboard"

