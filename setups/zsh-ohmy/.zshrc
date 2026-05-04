# Source owl startup script (XDG vars, PATH, and ~/.config/owl/rc/*)
[ -f "$HOME/owl/owl-start.sh" ] && source "$HOME/owl/owl-start.sh"

export ZSH="$HOME/.oh-my-zsh"

ZSH_THEME="robbyrussell"

zstyle ':omz:update' mode auto # update automatically without asking

zstyle ':omz:update' frequency 13 # How often to auto-update (in days).

plugins=(git colored-man-pages colorize pip python brew zsh-autosuggestions zsh-syntax-highlighting)

[ -f "$ZSH/oh-my-zsh.sh" ] && source "$ZSH/oh-my-zsh.sh"

# Enable command auto-correction.
ENABLE_CORRECTION="true"

# Use ESC to edit the current command line:
autoload -U edit-command-line
zle -N edit-command-line
bindkey -M vicmd v edit-command-line

COMPLETION_WAITING_DOTS="true"

# Kubectl completions (only if installed)
command -v kubectl >/dev/null 2>&1 && source <(kubectl completion zsh)

# bun completions
[ -s "$HOME/.bun/_bun" ] && source "$HOME/.bun/_bun"
