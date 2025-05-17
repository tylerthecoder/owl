curl https://github.com/dracula/gtk/archive/master.zip -L -o dracula-theme.zip
sudo unzip dracula-theme.zip -d /usr/share/themes
rm dracula-theme.zip
sudo mv /usr/share/themes/gtk-master /usr/share/themes/Dracula

gsettings set org.gnome.desktop.interface gtk-theme "Dracula"
gsettings set org.gnome.desktop.wm.preferences theme "Dracula"

curl https://github.com/dracula/gtk/files/5214870/Dracula.zip -L -o dracula-icons.zip
sudo unzip dracula-icons.zip -d /usr/share/icons
rm dracula-icons.zip
sudo mv /usr/share/icons/Dracula /usr/share/icons/Dracula

gsettings set org.gnome.desktop.interface icon-theme "Dracula"

echo "Done"
