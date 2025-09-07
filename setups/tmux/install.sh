if [ -d ~/.local/share/tmux/plugins/tpm ]; then
    (cd ~/.local/share/tmux/plugins/tpm && git pull)
else
    git clone https://github.com/tmux-plugins/tpm ~/.local/share/tmux/plugins/tpm
fi
