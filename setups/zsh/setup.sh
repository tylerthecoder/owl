sudo pacman -Sy zsh

mkdir -p ~/.cache/zsh
mkdir -p $XDG_STATE_HOME/zsh

git clone https://github.com/zsh-users/zsh-autosuggestions ~/.config/zsh/zsh-autosuggestions
git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ~/.config/zsh/zsh-syntax-highlighting
git clone https://github.com/zsh-users/zsh-history-substring-search ~/.config/zsh/zsh-history-substring-search

