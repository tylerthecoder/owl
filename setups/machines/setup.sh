#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

case "$(uname -s)" in
    Linux)
        if command -v pacman &> /dev/null; then
            sudo pacman -S --needed --noconfirm sftpman
        fi
        ;;
    Darwin)
        # sftpman is Linux-only; skip on macOS.
        ;;
    *)
        echo "Unsupported OS: $(uname -s)"
        exit 1
        ;;
esac

if ! command -v op &> /dev/null; then
    case "$(uname -s)" in
        Darwin)
            if command -v brew &> /dev/null; then
                echo "Installing 1Password CLI via Homebrew..."
                brew install 1password-cli
            else
                echo "1Password CLI (op) not found. Install Homebrew or grab op from https://1password.com/downloads/command-line/"
                exit 1
            fi
            ;;
        Linux)
            if command -v yay &> /dev/null; then
                yay -S --noconfirm 1password-cli
            else
                echo "1Password CLI (op) not found. Install it first."
                exit 1
            fi
            ;;
    esac
fi

op inject --in-file "$SCRIPT_DIR/ssh_config_template" --out-file "$SCRIPT_DIR/ssh_config.secret"
