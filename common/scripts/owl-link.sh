#! /bin/bash

if command -v fzf; then
  target=$(find . -maxdepth 2 -type f -name "*.targets" | \
    fzf --height=30 --layout=reverse --prompt="Select target: ")
else
  echo "Enter the target: ";
  read target
fi

if [[ -z "$target" ]]; then
  echo "No target selected"
  exit 1
fi

# Link Config
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

# Link Scripts
echo "Linking scripts"

for f in ./common/scripts/*;  do
 targetFileName=$(basename "$f" | cut -d "." -f1)
 targetFilePath="/usr/local/bin/$targetFileName"
 echo "Linking $f to $targetFilePath";
 sudo ln -f -T ${f} ${targetFilePath}
done;

for f in ./common/rofi-scripts/*;  do
 targetFileName=$(basename "$f" | cut -d "." -f1)
 targetFilePath="/usr/local/bin/$targetFileName"
 echo "Linking $f to $targetFilePath";
 sudo ln -f -T ${f} ${targetFilePath}
done;

# Link Desks
echo "Linking desks"

mkdir -p ~/.desks

for f in ./ubuntu/desks/*; do
	echo "Linking $f"
	ln -f -T ${f} ~/.desks/$(basename ${f})
done

