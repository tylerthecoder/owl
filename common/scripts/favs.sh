#!/bin/bash

# Check if tmux is installed
if ! command -v tmux &> /dev/null; then
    echo "tmux could not be found, please install it."
    exit 1
fi

input_file="$HOME/.favs"

# Check if input file exists
if [ ! -f "$input_file" ]; then
    echo "Input file not found: $input_file"
    exit 1
fi

# Display options to the user
while IFS= read -r line; do
    letter=$(echo "$line" | awk '{print $1}')
    path=$(echo "$line" | awk '{print $2}')
    echo "($letter) $path"
done < "$input_file"

# Wait for user input
read -p "> " user_input

# Find the corresponding path
selected_path=$(grep "^$user_input " "$input_file" | awk '{print $2}')
selected_path=$(eval echo "$selected_path")

if [ -z "$selected_path" ]; then
    echo "Invalid selection."
    exit 1
fi

# Check if a tmux session already exists for the path
session_name=$(basename "$selected_path")

if tmux has-session -t "$session_name" 2>/dev/null; then
    # Switch to the existing session
    if [ -n "$TMUX" ]; then
        # Inside a tmux session, switch to the target session
        tmux switch-client -t "$session_name"
    else
        # Outside a tmux session, attach to the target session
        tmux attach-session -t "$session_name"
    fi
else
    # Create a new session
    if [ -n "$TMUX" ]; then
        # Inside a tmux session, create a new session and switch to it
        tmux new-session -ds "$session_name" -c "$selected_path"
        tmux switch-client -t "$session_name"
    else
        # Outside a tmux session, create and attach to a new session
        tmux new-session -s "$session_name" -c "$selected_path"
    fi
fi

