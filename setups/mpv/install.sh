#!/usr/bin/env bash

set -euo pipefail

# Install mpv
sudo pacman -Syu --needed mpv

# Make it the default video and audio player
xdg-mime default mpv.desktop video/mp4 video/x-matroska video/x-msvideo video/x-ms-wmv video/quicktime video/x-flv video/webm video/ogg video/avi video/mpeg video/x-m4v audio/mpeg audio/mp3 audio/x-wav audio/ogg audio/flac audio/aac audio/x-m4a audio/vnd.rn-realaudio audio/vnd.wave audio/webm

