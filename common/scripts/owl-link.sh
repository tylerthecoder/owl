#! /bin/bash

if command -v fzf; then
  target=$(ls "$OWL_PATH/links" | \
    fzf --height=30 --layout=reverse --prompt="Select target: ")
else
  echo "Enter the target: ";
  read target
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
  # make the target path if not exist
  sudo mkdir -p $(dirname "$target")
	wd=$(pwd)
	echo -e "\e[31Linking $wd/$source to $target\e[31"
	sudo ln -f -T $source $target
}

function link_dir() {
	local context="$1"
	local target="$2"

	echo "Linking directory $context to directory $target"

	mkdir -p "$target"

	cd $context || exit

	# find all files in directory
	for f in $(find * -type f); do
		link_file $f "$target/$f"
	done

  go_home
}

function link_dir_no_ext() {
	local context="$1"
	local target="$2"

	echo "Linking directory $context to directory $target"

	mkdir -p "$target"

	cd $context || exit

	# find all files in directory
	for f in $(find * -type f); do
    targetFileName=$(basename "$f" | cut -d "." -f1)
		link_file $f "$target/$targetFileName"
	done

  go_home
}



. "$OWL_PATH/links/$target"
