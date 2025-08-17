#!/bin/bash

# Source the rc.sh file to set up environment variables
SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/rc.sh"

# Skip installation if nvm is already installed
if [ -s "$NVM_DIR/nvm.sh" ]; then
    echo "nvm is already installed. Skipping installation."
else
    echo "Installing nvm..."
    # Install nvm using the official installation script
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
fi

# Source nvm to make it available in this script
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

# Install the latest LTS version of Node.js
echo "Installing latest LTS version of Node.js..."
nvm install --lts
nvm use --lts
nvm alias default node

echo "nvm setup completed successfully!"
echo "Node version: $(node --version)"
echo "npm version: $(npm --version)"