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


if [[ -z "$target" ]]; then
  echo "No target selected"
  exit 1
fi

function go_home() {
	cd $OWL_PATH || exit
}

function link_file() {
	local source="$1"
	local target="$2"

  # Expand tildes
  source="${source/#\~/$HOME}"
  target="${target/#\~/$HOME}"

  # Append owl path to end of target
  source="$OWL_PATH/$source"

  # make the target path if not exist
  sudo mkdir -p $(dirname "$target")
	wd=$(pwd)

	echo "Linking $source to $target"

	sudo ln -f "$source" "$target"
}

function link_dir() {
	local context="$1"
	local target="$2"

  # Append owl path to end of target
  context="$OWL_PATH/$context"

	echo "Linking directory $context to directory $target"

	mkdir -p "$target"

	cd $context || exit

	# find all files in directory
	for f in $(find * -type f); do
		link_file "$f" "$target/$f"
	done

  go_home
}

function link_dir_no_ext() {
	local context="$1"
	local target="$2"

	echo "Linking directory $context to directory $target"

	mkdir -p "$target"

	cd "$OWL_PATH" || exit

	# find all files in directory
	for f in $(find "$context" -type f); do
    targetFileName=$(basename "$f" | cut -d "." -f1)
		link_file "$f" "$target/$targetFileName"
	done

  go_home
}

# If the file is json then run it here
filename=$(basename -- "$target")
extension="${filename##*.}"

if [ "$extension" = "json" ]; then
  for row in $(cat "$target" | jq -r '.[] | @base64'); do
    link_data=$(echo "$row" | base64 --decode)
    source=$(echo "$link_data" | jq -r ".source")
    target=$(echo "$link_data" | jq -r ".target")
    type=$(echo "$link_data" | jq -r ".type")

    if [ $type = "null" ]; then
      link_file "$source" "$target"
    elif [ $type = "dir_no_ext" ]; then
      link_dir_no_ext "$source" "$target"
    elif [ $type = "dir" ]; then
      link_dir "$source" "$target"
    fi
  done
else
  . "$OWL_PATH/links/$target"
fi



