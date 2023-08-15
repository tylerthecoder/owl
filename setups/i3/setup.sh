sudo pacman -Syu i3-gaps i3blocks i3lock numlockx network-manager-applet blueman-applet dunst volumeicon

git clone https://github.com/vivien/i3blocks-contrib ~/.config/i3blocks


owl add "$OWL_PATH/setups/i3/i3-config" "~/.config/i3/config"
owl add "$OWL_PATH/setups/i3/i3blocks-config" "~/.config/i3blocks/config"