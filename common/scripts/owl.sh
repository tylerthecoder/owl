#!/bin/bash

# This is the first script that is run,
# Assume nothing is availible yet

# Get owl path if not set
if [ -z "$OWL_PATH" ]; then
	echo "Enter the path to your owl installation: "
	read OWL_PATH
	export OWL_PATH="$OWL_PATH"
fi

function removeOwl() {
  echo "$1" | sed 's|'$OWL_PATH'||'
}


export -f removeOwl

case $1 in
  "link")
		bash "$OWL_PATH/common/scripts/owl-link.sh"
    ;;

	"sync")
		bash "$OWL_PATH/common/scripts/owl-sync.sh"
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


