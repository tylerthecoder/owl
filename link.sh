#! /bin/bash

target=$(find . -maxdepth 2 -type f -name "*.targets" | \
  fzf --height=30 --layout=reverse --prompt="Select target: ")

echo $target

while IFS= read -r line
do
  filePath=$(echo $line | cut -d " " -f1)
  targetPath=$(echo $line | cut -d " " -f2)

  absFilePath="${filePath/#~/$HOME}"
  absTargetPath="${targetPath/#~/$HOME}"

  echo "Linking $absFilePath to $absTargetPath"

  # make the target path if not exist
  mkdir -p $(dirname "$absTargetPath")

  # link my config file
  ln -f -T "$absFilePath" "$absTargetPath"
done < "$target"
