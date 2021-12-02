# Install Usefull Packages
sudo pacman -S i3 i3blocks

sudo pacman -Sy ranger highlight picom

sudo pacman -Sy zsh zsh-completions zsh-syntax-highlighting
mkdir ~/.cache/zsh


#Install dev packages
sudo pacman -Sy nodejs npm

sudo pacman -Sy dotnet-sdk dotnet-runtime aspnet-runtime

npm install --global yarn


# Make a viminfo file to remove it fromt he home directory
touch ~/.vim/viminfo

#Disable the beep
echo "blacklist pcspkr" | tee /etc/modprobe.d/nobeep.conf


