#!/bin/sh

# This is the first script that is run,
# Assume nothing is availible yet

# Get owl path if not set
if [ -z "$OWL_PATH" ]; then
	echo "Enter the path to your owl installation: "
	read OWL_PATH
	export OWL_PATH="$OWL_PATH"
fi

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

	"code")
		vim "$OWL_PATH"
		;;
esac

