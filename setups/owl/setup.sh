set -e

if [ ! -d "$HOME/owl" ]; then
    echo "Installing owl..."
    git clone https://github.com/tylerthecoder/owl.git "$HOME/owl"
else
    echo "owl already installed, updating..."
    git -C "$HOME/owl" pull --ff-only || true
fi

# Build owl from source
echo "Building owl from source..."
if ! command -v cargo >/dev/null 2>&1; then
    echo "Rust not found, installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

cargo build --release --bin owl --no-default-features --manifest-path "$HOME/owl/Cargo.toml"

# Link the binary
mkdir -p "$HOME/.local/bin"
ln -sf "$HOME/owl/target/release/owl" "$HOME/.local/bin/owl"
echo "Linked owl to ~/.local/bin/owl"

# Ensure ~/.local/bin is in PATH for this session
export PATH="$HOME/.local/bin:$PATH"

owl nest all

