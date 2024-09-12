if ! command -v zsh &> /dev/null
then
    sudo pacman -Sy zsh
else
    echo "zsh is already installed"
fi

mkdir -p ~/.cache/zsh
mkdir -p $XDG_STATE_HOME/zsh

if [ -d ~/.config/zsh/zsh-autosuggestions ]; then
    (cd ~/.config/zsh/zsh-autosuggestions && git pull)
else
    git clone https://github.com/zsh-users/zsh-autosuggestions ~/.config/zsh/zsh-autosuggestions
fi
if [ -d ~/.config/zsh/zsh-syntax-highlighting ]; then
    (cd ~/.config/zsh/zsh-syntax-highlighting && git pull)
else
    git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ~/.config/zsh/zsh-syntax-highlighting
fi
if [ -d ~/.config/zsh/zsh-history-substring-search ]; then
    (cd ~/.config/zsh/zsh-history-substring-search && git pull)
else
    git clone https://github.com/zsh-users/zsh-history-substring-search ~/.config/zsh/zsh-history-substring-search
fi

# Ask user if they want to change their default shell to zsh
echo "Would you like to set zsh as your default shell? (y/n)"
read answer

if [ "$answer" = "y" ] || [ "$answer" = "Y" ]; then
    echo "Changing default shell to zsh..."
    chsh -s "$(which zsh)"
    echo "Default shell changed to zsh. Please log out and log back in for the changes to take effect."
else
    echo "Keeping current default shell."
fi


