#!/bin/bash

IMG_DIR="$HOME/docs/media/wallpapers"

# Get a random image from the directory
IMG=$(ls $IMG_DIR | shuf -n 1)

# Set the background to the image
feh --no-fehbg --bg-scale "$IMG_DIR/$IMG"