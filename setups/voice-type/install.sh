#!/usr/bin/env bash
set -euo pipefail

# Install system dependencies
sudo pacman -Syu --needed wtype python python-pipx libsndfile portaudio

# Ensure pipx path is set up
pipx ensurepath

# Install voice-type from GitHub
pipx install git+https://github.com/tylerthecoder/voice-type.git --force

echo "voice-type installed!"
