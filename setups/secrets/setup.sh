#!/bin/bash
set -e

SCRIPT_DIR="$(dirname "$0")"

# Check if 1Password CLI is installed
if ! command -v op &>/dev/null; then
    echo "❌ 1Password CLI not found. Install it first:"
    echo "   Arch: yay -S 1password-cli"
    echo "   Ubuntu: See https://developer.1password.com/docs/cli/get-started/"
    exit 1
fi

# Check if 1Password is authenticated
if ! op account list &>/dev/null; then
    echo "❌ 1Password not authenticated. Run:"
    echo "   op signin"
    exit 1
fi

echo "🔄 Injecting secrets from 1Password..."

# Inject secrets from 1Password
if ! op inject --in-file "$SCRIPT_DIR/example.secrets.sh" --out-file "$SCRIPT_DIR/secrets.sh"; then
    echo "❌ Failed to inject secrets from 1Password"
    exit 1
fi

# Set restrictive permissions
chmod 600 "$SCRIPT_DIR/secrets.sh"

echo "✅ Secrets loaded successfully"
echo "📝 File: $SCRIPT_DIR/secrets.sh (mode 600)"
