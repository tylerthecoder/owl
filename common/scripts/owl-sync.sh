#!/bin/bash

cd "$OWL_PATH"

if git merge --ff-only; then
  printf "$envPath updated successfully\n"
else
  while true; do
      printf 'Git merge did not succed \n How should we proceed?\n'

      options=("Resolve in VS code" "Try Again" "Exit")
      sel=$(printf "%s\n" "${options[@]}" | fzf --height 40% --reverse -0)

      [ "$sel" = "Resolve in VS code" ] && code . && exit
      [ "$sel" = "Exit" ] && exit
  done
fi

while true; do
  untracked=$(git status --porcelain)

  [ -z untracked ] && echo "All files pushed" && break

  printf "You have untracked files:\n What do you want to do?"

  options=("Resolve is VS code" "Push Them All" "Try Again" "Exit")
  sel=$(printf "%s\n" "${options[@]}" | fzf --height 40% --reverse -0)
  [ "$sel" = "Resolve is VS code" ] && code . && exit
  [ "$sel" = "Exit" ] && exit

  if [ "$sel" = "Push Them All" ]; then
      git add .
      git commit
      git push
      break
  fi
done

echo "$envPath synced"
