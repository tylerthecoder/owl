#!/bin/bash
# Make script directory the working directory
cd "$(dirname "$0")"
# source ./helpers.sh

go_home || exit

cd common/config || exit

link_file .vimrc ~/.vim/vimrc
link_file .vimrc ~/.config/nvim/init.vim
link_file i3-config ~/.config/i3/config
link_file .bashrc ~/.bashrc
link_file rc.conf ~/.config/ranger/rc.conf
link_file .profile ~/.profile.main
link_file config.fish ~/.config/fish/config.fish
link_file user-dirs.dirs ~/.config/user-dirs.dirs
link_file i3blocks-config ~/.config/i3blocks/config
link_file compton.conf ~/.config/compton.conf
link_file gitconfig ~/.config/git/config
link_file .alias ~/.config/alias/main
link_file .alias-git ~/.config/alias/git
link_file npmrc ~/.config/npm/npmrc
link_file rofi.config ~/.config/rofi/config.rasi
link_file starship.toml ~/.config/starship.toml
link_file picom.config ~/.config/picom.conf

go_home || exit

cd ubuntu/config || exit

link_file .profile ~/.profile
link_file .xprofile ~/.xprofile
link_file .Xresources ~/.Xresources
link_file udev.rules /etc/udev/rules.d/10-owl.rules

go_home || exit

link_dir ./ubuntu/desks ~/.desks
link_dir_no_ext ./common/scripts /usr/local/bin
link_dir_no_ext ./common/rofi-scripts /usr/local/bin
link_dir ./common/services/ /usr/lib/systemd/user
