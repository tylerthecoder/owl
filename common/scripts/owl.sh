#!/bin/bash

# This is the first script that is run,
# Assume nothing is availible yet

echo "$OWL_PATH"

# Get owl path if not set
if [ -z "$OWL_PATH" ]; then
	echo "Enter the path to your owl installation: "
	read OWL_PATH
fi

# Install dependencies

# Install fzf
if ! command -v fzf 2>&1 /dev/null; then
	echo "Fzf isn't installed. Would you like to install? (y)"
	read answer
	if [ "$answer" = "y" ]; then
		if command -v pacman; then
			sudo pacman -S fzf
		elif command -v brew; then
			brew install fzf
		fi
	fi
fi

# Install jq
if ! command -v jq 2>&1 /dev/null; then
	echo "jq isn't installed. Would you like to install? (y)"
	read answer
	if [ "$answer" = "y" ]; then
		echo "Installing..."
		if command -v pacman; then
			sudo pacman -S jq
		elif command -v brew; then
			brew install jq
		fi
	fi
fi

function removeOwl() {
  echo "$1" | sed 's|'$OWL_PATH'||'
}

export -f removeOwl

case $1 in
  "link")
		. "$OWL_PATH/common/scripts/owl-link.sh"
    ;;

	"sync")
		bash "$OWL_PATH/common/scripts/owl-sync.sh"
    ;;

	"add")
		bash "$OWL_PATH/common/scripts/owl-add.sh"
		;;

	"go")
		ranger "$OWL_PATH"
		;;

  "vim")
    vim "$OWL_PATH"
    ;;

	"code")
		code "$OWL_PATH"
		;;

  "e")
    fname=$(
      find "$OWL_PATH" -path "$OWL_PATH/.git" -prune -o -exec bash -c "removeOwl \"{}\"" \; |
        fzf --height=30 --layout=reverse --prompt="Select File: ")
    [[ -f "$OWL_PATH/$fname" ]] && vim "$OWL_PATH/$fname"
    ;;
esac


