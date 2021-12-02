
# Enable colors and change prompt
autoload -U colors && colors
PS1="%B%{$fg[yellow]%}[%{$fg[red]%}%n%{$fg[green]%}@%{$fg[blue]%}%M %{$fg[magenta]%}%~%{$fg[yellow]%}]%{$reset_color%}$%b "

# History things
HISTFILE=~/.config/zsh/history
HISTSIZE=100000
SAVEHIST=100000
setopt autocd

# Set the cursor as a verticla line
echo -ne '\e[5 q'

# Callback for vim mode change
function zle-keymap-select () {
    if [ $KEYMAP = vicmd ]; then
        echo -ne '\e[1 q'
    else
        # Set beam cursor
        echo -ne '\e[5 q'
    fi

    if typeset -f prompt_pure_update_vim_prompt_widget > /dev/null; then
        # Refresh prompt and call Pure super function
        prompt_pure_update_vim_prompt_widget
    fi
}
zle -N zle-keymap-select


# Auto/tab complete
autoload -U compinit
zstyle ':completion:*' menu select
zmodload zsh/complist
compinit
_comp_options+=(globdots)		# Include hidden files.
setopt COMPLETE_ALIASES
zstyle ':completion:*' completer _expand_alias _complete _ignored

setopt correct

# Expand alias on space
function expand-alias() {
	zle _expand_alias
	zle self-insert
}
zle -N expand-alias
bindkey -M main ' ' expand-alias


# Use vim keys in tab complete menu:
bindkey -M menuselect 'h' vi-backward-char
bindkey -M menuselect 'k' vi-up-line-or-history
bindkey -M menuselect 'l' vi-forward-char
bindkey -M menuselect 'j' vi-down-line-or-history
bindkey -v '^?' backward-delete-char

# Edit line in vim with ctrl-e:
autoload edit-command-line; zle -N edit-command-line
bindkey '^e' edit-command-line

# Load Alias
[ -f "$HOME/.config/alias/main" ] && source ~/.config/alias/main
[ -f "$HOME/.config/alias/git" ] && source ~/.config/alias/git

# Load Auto Complete
source "$ZDOTDIR/zsh-autosuggestions/zsh-autosuggestions.zsh"

# Load syntaxt highlighting (needs to be last)
source "$ZDOTDIR/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"

# Load fuzzy history search
source "$ZDOTDIR/zsh-history-substring-search/zsh-history-substring-search.zsh"

# Fuzzy search history
bindkey '^[[A' history-substring-search-up
bindkey '^[[B' history-substring-search-down
