#!/bin/sh

# My personal directories
export DEV_FOLDER="$HOME/dev"

# Set XDG variables
export XDG_CONFIG_HOME="$HOME/.config/"
export XDG_DATA_HOME="$HOME/.local/share"
export XDG_CACHE_HOME="$HOME/.cache"
export XDG_DOWNLOAD_DIR="$HOME/downloads"

#Java
export JAVA_HOME=/opt/jdk-18


# All things added to path
export PATH="$HOME/.local/bin:$PATH" # Personal bin
export PATH="$XDG_DATA_HOME/npm/bin:$PATH" # Add npm to path
export PATH="/var/lib/snapd/snap/bin:$PATH" # I don't want to have to do this but the world is working against me
export PATH="$HOME/.cargo/bin:$PATH" # Add rust to path
export PATH="$JAVA_HOME/bin:$PATH" # Add java to path

export EDITOR="vim"
export VISUAL="vim"
export TERMINAL="st"
export ANDROID_HOME="/usr/lib/Android/Sdk"
export BROWSER="/usr/bin/brave --force-device-scale-factor=2"
export COMPOSITOR="/usr/bin/picom --experimental-backends"

# Configure ZSH
export ZDOTDIR="$HOME/.config/zsh"

#History file variables
export LESSHISTFILE=/dev/null # disable the less history file

# Configure Node, NPM, Yarn
export NPM_CONFIG_USERCONFIG=$XDG_CONFIG_HOME/npm/npmrc
export NODE_REPL_HISTORY=$XDG_CONFIG_HOME/node/repl_hist

# Configure dotnet
# They don't let us move the ".dotnet" folder, what a shame
export NUGET_PACKAGES="$XDG_CACHE_HOME/nuget"

# Moving things out of the home directory
export PLATFORMIO_HOME_DIR="$XDG_DATA_HOME/platformio"

# This messed up a bunch of my keysigning
# export GNUPGHOME="$XDG_DATA_HOME/gnupg"

# To scale applications up
export GDK_SCALE=2
export GDK_DPI_SCALE=0.5

# To make unity intellisence to work in vscode
export FrameworkPathOverride="/lib/mono/4.7.1-a"


