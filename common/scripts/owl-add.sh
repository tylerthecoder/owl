#! /bin/bash

target="$OWL_DEFAULT_LINK"
if [ -z "$target" ]; then
  if command -v fzf; then
    target=$(ls "$OWL_PATH/links" | fzf --height=30 --layout=reverse --prompt="Select target: ")
    target="$OWL_PATH/links/$target"
  else
    echo "Enter the target: "
    read target
  fi
fi

echo "$1 to $2"

# Used to add a file to the owl tracking system
source=$(readlink -f "$1")
dest="$2"

# Remove owl path from source
source=${source#$OWL_PATH/}

echo "Adding owl link: $source --> $dest"

NEW_JSON=$(cat "$target" | jq ". += [{ \"source\": \"$source\", \"target\": \"$dest\" }]")

echo "$target $NEW_JSON"

# add the file to the owl link target
echo "$NEW_JSON" > "$target"













