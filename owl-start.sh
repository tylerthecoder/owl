# This script should be sourced by the running shell to set up owl
# It handles common envs and loading scripts

# XDG vars
export XDG_CONFIG_HOME="$HOME/.config"
export XDG_CACHE_HOME="$HOME/.cache"
export XDG_DATA_HOME="$HOME/.local/share"
export XDG_STATE_HOME="$HOME/.local/state"


# Local bin
export PATH="$HOME/.local/bin:$PATH"

# Run owl-rc (all rc scripts)
if [ -d ~/.config/owl/rc ]; then
    for file in ~/.config/owl/rc/*; do
        [ -f "$file" ] && source "$file"
    done
fi