#!/bin/bash
# Make script directory the working directory
cd "$(dirname "$0")"
# source ./helpers.sh

go_home || exit

cd common/config || exit

link_file .vimrc ~/.vimrc
link_file rc.conf ~/.config/ranger/rc.conf
link_file gitconfig ~/.config/git/config
link_file .alias ~/.config/alias/main
link_file .alias-git ~/.config/alias/git

go_home || exit
cd home-pi

link_file .profile ~/.profile
link_file .bashrc ~/.bashrc
link_file nginx.conf /etc/nginx/nginx.conf

go_home || exit
link_file ./common/scripts/owl.sh /usr/local/bin/owl

