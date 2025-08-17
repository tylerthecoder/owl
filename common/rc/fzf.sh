# FZF configuration and key bindings
if command -v fzf >/dev/null 2>&1; then
    # FZF default options
    export FZF_DEFAULT_OPTS='--height 40% --layout=reverse --border'

    # Use fd if available
    if command -v fd >/dev/null 2>&1; then
        export FZF_DEFAULT_COMMAND='fd --type f --hidden --follow --exclude .git'
        export FZF_CTRL_T_COMMAND="$FZF_DEFAULT_COMMAND"
    fi

    # Key bindings
    if [[ -f ~/.fzf.bash ]]; then
        source ~/.fzf.bash
    elif [[ -f ~/.fzf.zsh ]]; then
        source ~/.fzf.zsh
    fi
fi