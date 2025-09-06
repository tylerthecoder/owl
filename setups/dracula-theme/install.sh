#!/usr/bin/env bash
set -euo pipefail

# Install Dracula GTK theme and icons (system-wide)
sudo pacman -Syu --needed unzip curl || true
curl -L https://github.com/dracula/gtk/archive/master.zip -o /tmp/dracula-theme.zip
sudo unzip -o /tmp/dracula-theme.zip -d /usr/share/themes
sudo rm -f /tmp/dracula-theme.zip
sudo mv -f /usr/share/themes/gtk-master /usr/share/themes/Dracula || true

curl -L https://github.com/dracula/gtk/files/5214870/Dracula.zip -o /tmp/dracula-icons.zip
sudo unzip -o /tmp/dracula-icons.zip -d /usr/share/icons
sudo rm -f /tmp/dracula-icons.zip

echo "Dracula theme installed. Link settings via owl nest setup to apply per-user GTK config."


