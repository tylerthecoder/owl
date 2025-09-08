set -e

if [ ! -d "$HOME/owl" ]; then
    echo "Installing owl..."
    git clone https://github.com/tylerthecoder/owl.git "$HOME/owl"
else
    echo "owl already installed, updating..."
    git -C "$HOME/owl" pull --ff-only || true
fi

# install owl if not in path
if ! command -v owl >/dev/null 2>&1; then
    echo "owl not in path, installing..."
    tmp=$(mktemp)
    curl -fsSL https://github.com/tylerthecoder/owl/releases/download/main/owl -o "$tmp"
    chmod +x "$tmp"
    sudo mv "$tmp" /usr/local/bin/owl
fi

owl update --recursive
owl nest all

