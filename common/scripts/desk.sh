#! /bin/bash

desks_path=~/.config/desks

target=$(ls "$desks_path" | \
        fzf --height=30 --layout=reverse --prompt="Select Desk: ")

echo "$target"

bash "$desks_path/$target"

