#!/bin/bash

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

move_window_to_workspace_2() {
    WSP=$(list_workspaces | rofi -dmenu -p "Move window to workspace")
    i3-msg "move container to workspace $WSP"
}




menu() {
    GO_MSG="(j) Go to Workspace"
    MOVE_MSG="(k) Move Window"
    APPS_MSG="(a) Apps"
    QUIT_MSG="(q) Quit"
    ACITON=$(echo -e "$GO_MSG\n$MOVE_MSG\n$APPS_MSG\n$QUIT_MSG" | rofi -dmenu -p "Action:" -kb-select-1 'j' -kb-select-2 'k' -kb-select-3 'a' -kb-select-4 'q')

    if [[ "$ACITON" = "$GO_MSG" ]]; then
        go_to_workspace_2 $@
    elif [[ "$ACITON" = "$MOVE_MSG" ]]; then
        move_window_to_workspace_2 $@
    elif [[ "$ACITON" = "$APPS_MSG" ]]; then
        rofi -show drun
    elif [[ "$ACITON" = "$QUIT_MSG" ]]; then
        exit 0
    fi
}

if [[ "$1" = "go" ]]; then
    go_to_workspace_2 $@
elif [[ "$1" = "move" ]]; then
    move_window_to_workspace_2 $@
else
    menu $@
fi


