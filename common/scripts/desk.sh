#! /bin/bash

target=$(ls ~/.config/desks | \
        fzf --height=30 --layout=reverse --prompt="Select Desk: ")

echo "$target"

bash ~/.desks/$target

