if ! command -v zsh &> /dev/null
then
    # check user's package manager
    if command -v apt &> /dev/null; then
        package_manager="apt"
    elif command -v pacman &> /dev/null; then
        package_manager="pacman"
    else
        echo "Unsupported package manager"
        exit 1
    fi
    echo "Would you like to install zsh? (y/n)"
    read answer
    if [ "$answer" = "y" ] || [ "$answer" = "Y" ]; then
        if [ "$package_manager" = "pacman" ]; then
            sudo pacman -Sy zsh
        elif [ "$package_manager" = "apt" ]; then
            sudo apt install zsh
        fi
    fi
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


# Check the current default shell
current_shell=$(basename "$SHELL")
if [ "$current_shell" != "zsh" ]; then
    echo "Would you like to change your default shell to zsh? (y/n)"
    read answer

    if [ "$answer" = "y" ] || [ "$answer" = "Y" ]; then
        echo "Changing default shell to zsh..."
        zsh_path=$(which zsh)
        echo "zsh path: $zsh_path"
        sudo chsh -s "$zsh_path"
        echo "Default shell changed to zsh. Please log out and log back in for the changes to take effect."
    else
        echo "Keeping current default shell."
    fi
fi


