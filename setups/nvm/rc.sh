# NVM configuration
export NVM_DIR="$XDG_CONFIG_HOME/nvm"

# Load nvm
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

# Load nvm bash_completion
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"

# Add npm global packages to PATH
export PATH="$NVM_DIR/versions/node/$(nvm version)/bin:$PATH"