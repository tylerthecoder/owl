#!/bin/bash
set -euo pipefail

if command -v nvim &> /dev/null; then
    echo "nvim already installed at $(command -v nvim). Skipping."
    exit 0
fi

NVIM_VERSION="v0.11.4"
mkdir -p "$HOME/.local/bin"

case "$(uname -s)" in
    Linux)
        ARCH=$(uname -m)
        URL="https://github.com/neovim/neovim/releases/download/${NVIM_VERSION}/nvim-linux-${ARCH}.appimage"
        echo "Downloading neovim AppImage for Linux ($ARCH)..."
        curl -L "$URL" -o /tmp/nvim-download
        chmod +x /tmp/nvim-download
        mv /tmp/nvim-download "$HOME/.local/bin/nvim"
        ;;
    Darwin)
        if command -v brew &> /dev/null; then
            echo "Installing neovim via Homebrew..."
            brew install neovim
        else
            ARCH=$(uname -m)
            case "$ARCH" in
                arm64) ASSET="nvim-macos-arm64" ;;
                x86_64) ASSET="nvim-macos-x86_64" ;;
                *) echo "Unsupported macOS arch: $ARCH"; exit 1 ;;
            esac
            URL="https://github.com/neovim/neovim/releases/download/${NVIM_VERSION}/${ASSET}.tar.gz"
            echo "Downloading neovim tarball for macOS ($ARCH)..."
            TMPDIR=$(mktemp -d)
            curl -L "$URL" -o "$TMPDIR/nvim.tar.gz"
            tar -xzf "$TMPDIR/nvim.tar.gz" -C "$TMPDIR"
            mkdir -p "$HOME/.local/share/nvim-dist"
            rm -rf "$HOME/.local/share/nvim-dist/${ASSET}"
            mv "$TMPDIR/${ASSET}" "$HOME/.local/share/nvim-dist/${ASSET}"
            ln -sf "$HOME/.local/share/nvim-dist/${ASSET}/bin/nvim" "$HOME/.local/bin/nvim"
            rm -rf "$TMPDIR"
        fi
        ;;
    *)
        echo "Unsupported OS: $(uname -s)"
        exit 1
        ;;
esac
