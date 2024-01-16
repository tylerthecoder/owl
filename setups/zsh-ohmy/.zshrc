
# Source environment variables
source ~/.shenv

export ZSH="$HOME/.oh-my-zsh"

ZSH_THEME="robbyrussell"

zstyle ':omz:update' mode auto # update automatically without asking

zstyle ':omz:update' frequency 13 # How often to auto-update (in days).

plugins=(git git-trim colored-man-pages colorize pip python brew zsh-autosuggestions zsh-syntax-highlighting)

source $ZSH/oh-my-zsh.sh

# Enable command auto-correction.
ENABLE_CORRECTION="true"

# Use ESC to edit the current command line:
autoload -U edit-command-line
zle -N edit-command-line
bindkey -M vicmd v edit-command-line

# Uncomment the following line to display red dots whilst waiting for completion.
# You can also set it to another string to have that shown instead of the default red dots.
# e.g. COMPLETION_WAITING_DOTS="%F{yellow}waiting...%f"
# Caution: this setting can cause issues with multiline prompts in zsh < 5.7.1 (see #5765)
COMPLETION_WAITING_DOTS="true"

# Kube completions
source <(kubectl completion zsh)


# bun completions
[ -s "/Users/tylertracy/.bun/_bun" ] && source "/Users/tylertracy/.bun/_bun"

### MANAGED BY RANCHER DESKTOP START (DO NOT EDIT)
export PATH="/Users/tylertracy/.rd/bin:$PATH"
