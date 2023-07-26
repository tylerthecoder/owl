
# Install Display server
sudo pacman -S xorg-server


# Insall window manager
sudo pacman -S i3 i3blocks picom

# Install command line utils
sudo pacman -Sy ranger highlight



sudo pacman -Sy zsh zsh-completions zsh-syntax-highlighting

#Install dev packages
sudo pacman -Sy nodejs npm

# Install dotnet
sudo pacman -Sy dotnet-sdk dotnet-runtime aspnet-runtime

npm install --global yarn


# Make a viminfo file to remove it fromt he home directory
touch ~/.vim/viminfo

#Disable the beep
echo "blacklist pcspkr" | tee /etc/modprobe.d/nobeep.conf



# Install owl


