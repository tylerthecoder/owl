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

    fzfopen() {
        local dir=${1:-.} # Default to current directory if no argument is given
        local file
        file=$(find "$dir" -type f 2> /dev/null \
            | grep -vE '(/\.git/|/\.git$)' \
            | grep -vFf <(git -C "$dir" ls-files --exclude-standard -oi --directory) \
            | sed "s|${dir}/||" \
            | fzf --color=dark +m) \
        && nvim "$dir/$file"
    }

    fo() {
        fzfopen ~/owl
    }

    fn() {
        fzfopen ~/notes
    }
fi