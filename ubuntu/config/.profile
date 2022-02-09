#!/bin/sh

source ~/.profile.main

export OWL_PATH=/home/tyler/owl
export BROWSER=/usr/bin/brave-browser
export COMPOSITOR=/usr/bin/compton
export TERMINAL=/usr/bin/gnome-terminal

export XDG_PICTURES_DIR="$HOME/img"
export DEV_FOLDER="$HOME/p"

export GDK_SCALE=1
export GDK_DPI_SCALE=1

. "$HOME/.cargo/env"
