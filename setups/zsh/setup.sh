
sudo pacman -Sy zsh zsh-completions zsh-syntax-highlighting
owl add "$OWL_PATH/common/config/.zshrc" "~/.zshrc"
mkdir -p ~/.cache/zsh

git clone https://github.com/zsh-users/zsh-autosuggestions ~/.config/zsh/zsh-autosuggestions
git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ~/.config/zsh/zsh-syntax-highlighting
git clone https://github.com/zsh-users/zsh-history-substring-search ~/.config/zsh/zsh-history-substring-search