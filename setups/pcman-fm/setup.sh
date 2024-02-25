sudo pacman -Syu pcmanfm

xdg-mime default pcmanfm.desktop inode/directory application/x-gnome-saved-search
update-mime-database ~/.local/share/mime
xdg-mime query default inode/directory


