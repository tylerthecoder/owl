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

# Note: Dependencies (git, rust) are handled by the nest/setup that depends on owl
# Running owl update --recursive here would cause circular dependency loops
