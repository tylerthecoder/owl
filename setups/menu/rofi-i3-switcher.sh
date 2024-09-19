#!/bin/bash

$PATH=$PATH:~/.local/bin

# Function to list workspaces
list_workspaces() {
    i3-msg -t get_workspaces | jq -r '.[] | .name' | sort -n
}

# Function for 'go to workspace' mode
go_to_workspace() {
    if [ -z "$2" ]; then
        list_workspaces
    else
        i3-msg workspace "$2"
    fi
}

go_to_workspace_2() {
    WSP=$(list_workspaces | rofi -dmenu -p "Go to workspace")
    i3-msg workspace "$WSP"
}

# Function for 'move window to workspace' mode
move_window_to_workspace() {
    if [ -z "$2" ]; then
        list_workspaces
    else
        i3-msg "move container to workspace $2"
    fi
}

mount_tab() {
    workspace=$(~/.local/bin/mt list-workspaces | rofi -dmenu -i -p "Select workspace")

    if [ -z "$workspace" ]; then
        echo "No workspace selected"
        exit 1
    fi

    echo "Selected workspace: $workspace"

    ~/.local/bin/mt start $workspace
}

bench_menu() {
    workspace=$(bench list-workspaces | rofi -dmenu -i -p "Select workspace")
    if [ -z "$workspace" ]; then
        echo "No workspace selected"
        exit 1
    fi
    echo "Selected workspace: $workspace"
    bench open "$workspace"
}


move_window_to_workspace_2() {
    WSP=$(list_workspaces | rofi -dmenu -p "Move window to workspace")
    i3-msg "move container to workspace $WSP"
}

run_owl_script() {
    script=$(ls ~/owl/common/scripts | rofi -dmenu -i -p "Select script")
    if [ -n "$script" ]; then
        ~/owl/common/scripts/"$script"
    fi
}

menu() {
    GO_MSG="(j) Go to Workspace"
    MOVE_MSG="(k) Move Window"
    APPS_MSG="(a) Apps"
    WINDOW_MSG="(w) Window"
    EMOJI_MSG="(e) Emoji"
    CLIPBOARD_MSG="(c) Clipboard"
    BROWSER_MSG="(b) Browser"
    OWL_SCRIPT_MSG="(s) Scripts"
    QUIT_MSG="(q) Quit"

    MENU_MSG="$GO_MSG\n$MOVE_MSG\n$APPS_MSG\n$WINDOW_MSG\n$EMOJI_MSG\n$CLIPBOARD_MSG\n$BROWSER_MSG\n$OWL_SCRIPT_MSG\n$QUIT_MSG"

    ACITON=$(echo -e "$MENU_MSG" | rofi -dmenu -p "Action:" -kb-select-1 'j' -kb-select-2 'k' -kb-select-3 'a' -kb-select-4 'w' -kb-select-5 'e' -kb-select-6 'c' -kb-select-7 'b' -kb-select-8 's' -kb-select-9 'q')

    if [[ "$ACITON" = "$GO_MSG" ]]; then
        go_to_workspace_2 $@
    elif [[ "$ACITON" = "$MOVE_MSG" ]]; then
        move_window_to_workspace_2 $@
    elif [[ "$ACITON" = "$APPS_MSG" ]]; then
        rofi -show drun
    elif [[ "$ACITON" = "$QUIT_MSG" ]]; then
        exit 0
    elif [[ "$ACITON" = "$WINDOW_MSG" ]]; then
        rofi -show window
    elif [[ "$ACITON" = "$EMOJI_MSG" ]]; then
        rofi -modi "emoji" -show emoji
    elif [[ "$ACITON" = "$BROWSER_MSG" ]]; then
        bench_menu
    elif [[ "$ACITON" = "$CLIPBOARD_MSG" ]]; then
        rofi -modi "clipboard:greenclip print" -show clipboard -run-command '{cmd}'
    elif [[ "$ACITON" = "$OWL_SCRIPT_MSG" ]]; then
        run_owl_script
    fi
}

if [[ "$1" = "go" ]]; then
    go_to_workspace_2 $@
elif [[ "$1" = "move" ]]; then
    move_window_to_workspace_2 $@
elif [[ "$1" = "run" ]]; then
    rofi -show drun
elif [[ "$1" = "window" ]]; then
    rofi -show window
elif [[ "$1" = "emoji" ]]; then
    menu $@
else
    menu $@
fi


