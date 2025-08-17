export ZDOTDIR="$HOME/.config/zsh"

# Source owl startup script
[ -f ~/owl/owl-start.sh ] && source ~/owl/owl-start.sh

# Enable colors and change prompt
autoload -U colors && colors
PS1="%B%{$fg[yellow]%}[%{$fg[red]%}%n%{$fg[green]%}@%{$fg[blue]%}%M %{$fg[magenta]%}%~%{$fg[yellow]%}]%{$reset_color%}$%b "

# History things
export HISTFILE="$XDG_STATE_HOME"/zsh/history
HISTSIZE=100000
SAVEHIST=100000
setopt INC_APPEND_HISTORY # History is saved to the file immediately, not just on exit
setopt EXTENDED_HISTORY # Save timestamps
setopt HIST_FIND_NO_DUPS
setopt HIST_IGNORE_ALL_DUPS
setopt autocd

# Set the cursor as a vertical line
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


# Set color
ZSH_AUTOSUGGEST_HIGHLIGHT_STYLE="fg=#ff0000,bg=white,bold,underline"

# Load Auto Complete
source "$ZDOTDIR/zsh-autosuggestions/zsh-autosuggestions.zsh"

# Load syntax highlighting
source "$ZDOTDIR/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"

# Load fuzzy history search
source "$ZDOTDIR/zsh-history-substring-search/zsh-history-substring-search.zsh"

# Fuzzy search history
bindkey "^[[A" history-substring-search-up
bindkey "^[[B" history-substring-search-down
bindkey "^[OA" history-substring-search-up
bindkey "^[OB" history-substring-search-down

# Ctrl-Backspace to delete word
bindkey '^H' backward-kill-word

# Ctrl-Delete to delete word
bindkey '5~' kill-word

# bun completions
[ -s "/home/tylord/.bun/_bun" ] && source "/home/tylord/.bun/_bun"

# uv completions
#
if command -v uv &> /dev/null; then
    eval "$(uv generate-shell-completion zsh)" # you should already have these two lines
    eval "$(uvx --generate-shell-completion zsh)"
fi

# you will need to add the lines below
# https://github.com/astral-sh/uv/issues/8432#issuecomment-2453494736
_uv_run_mod() {
    if [[ "$words[2]" == "run" && "$words[CURRENT]" != -* ]]; then
        _arguments '*:filename:_files'
    else
        _uv "$@"
    fi
}
compdef _uv_run_mod uv
