# Install Display server
sudo pacman -S xorg-server

# Insall window manager
sudo pacman -S i3 i3blocks picom

#Install dev packages
sudo pacman -Sy nodejs npm

# Install dotnet
sudo pacman -Sy dotnet-sdk dotnet-runtime aspnet-runtime

npm install --global yarn

#Disable the beep
echo "blacklist pcspkr" | tee /etc/modprobe.d/nobeep.conf