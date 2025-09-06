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
echo "Make sure to set zsh as your default shell"
echo "You can do this by running 'chsh -s $(which zsh)'"

