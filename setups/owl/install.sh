# Owl bootstrap/install script

set -e

if ! command -v owl >/dev/null 2>&1; then
    echo "Installing owl binary from GitHub releases..."
    tmp=$(mktemp)
    curl -fsSL https://github.com/tylerthecoder/owl/releases/download/main/owl -o "$tmp"
    chmod +x "$tmp"
    sudo mv "$tmp" /usr/local/bin/owl
else
    echo "owl already installed"
fi

echo "Ensuring owl dependencies are installed via owl update..."
owl update --recursive || true
