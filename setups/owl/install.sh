# Owl bootstrap/install script

set -e

if command -v owl >/dev/null 2>&1 || [ -x "$HOME/.local/bin/owl" ] || [ -x "/usr/local/bin/owl" ]; then
    echo "owl already installed"
else
    echo "Installing owl binary from GitHub releases..."
    tmp=$(mktemp)
    curl -fsSL https://github.com/tylerthecoder/owl/releases/download/main/owl -o "$tmp"
    chmod +x "$tmp"
    mkdir -p "$HOME/.local/bin"
    mv "$tmp" "$HOME/.local/bin/owl"
fi

# Note: Dependencies (git, rust) are handled by the nest/setup that depends on owl
# Running owl update --recursive here would cause circular dependency loops
