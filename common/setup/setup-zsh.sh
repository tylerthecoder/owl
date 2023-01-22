# Install zsh autocomplete
git clone https://github.com/zsh-users/zsh-autosuggestions "$HOME/.config/zsh/zsh-autosuggestions"

# Install zsh syntax highlighting
git clone https://github.com/zsh-users/zsh-syntax-highlighting.git "$HOME/.config/zsh/zsh-syntax-highlighting"

# Install zsh history substring search
git clone https://github.com/zsh-users/zsh-history-substring-search "$HOME/.config/zsh/zsh-history-substring-search"

echo "export ZDOTDIR=$HOME/.config/zsh" >> "$HOME/.zshenv"
