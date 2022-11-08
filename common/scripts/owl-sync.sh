#!/bin/bash

# First thing to do. Make sure SCRIPTS_PATH and CONFIG_PATH are defined, if not then prompt for them

sync_directory() {
  pathVarStr=$1

  # Get the value of the environment variable
  # Using variable indirection https://www.gnu.org/savannah-checkouts/gnu/bash/manual/bash.html#index-expansion_002c-parameter
  envPath="${!pathVarStr}"

  if [ -z "$envPath" ]; then
    echo "$pathVarStr doesn't exist. Enter a path"
    while true; do
      read -p ": " envPath
      [ -d "$envPath" ] && break
      echo "Not a directory, enter a path"
    done
  fi

  cd "$envPath"

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

    echo "$sel"

    if [ "$sel" = "Push Them All" ]; then
      git add .
      git commit
      git push
      break
    fi
  done

  echo "$envPath synced"
}

sync_directory OWL_PATH
