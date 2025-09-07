#!/usr/bin/env bash

set -euo pipefail

sudo pacman -Syu --needed polkit polkit-gnome

echo "Polkit installed. If you use a polkit agent, ensure it runs in your session."



