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

# Used to add a file to the owl tracking system 
file_to_add="$1"

# ask user where to add the file in owl
echo "Enter the path to add the file to: "
echo "$OWL_PATH/"
read path

# add the file to the owl tracking system
mv "$file_to_add" "$OWL_PATH/$path"

NEW_JSON=$(cat "$target" | jq ". += [{ \"source\": \"$path\", \"target\": \"$file_to_add\" }]")

echo "$target $NEW_JSON"

# add the file to the owl link target
echo "$NEW_JSON" > "$target"













