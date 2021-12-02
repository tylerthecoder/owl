#!/bin/bash

FILE=$(find "$CONFIG_PATH" -type f -not -path '*/\.git/*' | dmenu)

urxvt -e vim $FILE


