#!/usr/bin/env bash
set -euo pipefail

# Get wallpaper directory from environment or use default
WALL_DIR="${WALL_DIR:-$HOME/docs/media/wallpapers}"

if [ ! -d "$WALL_DIR" ]; then
    echo "Wallpaper directory not found: $WALL_DIR"
    exit 1
fi

# Find all image files and show in rofi with just the filename
selected=$(find "$WALL_DIR" -type f \( -iname "*.jpg" -o -iname "*.jpeg" -o -iname "*.png" -o -iname "*.webp" \) -printf "%f\n" | sort | rofi -dmenu -i -p "Select Background")

if [ -n "$selected" ]; then
    # Find the full path of the selected file
    full_path=$(find "$WALL_DIR" -type f -name "$selected" | head -1)

    if [ -n "$full_path" ]; then
        # Set wallpaper using swww
        swww img "$full_path" --transition-type fade --transition-duration 1
    fi
fi
