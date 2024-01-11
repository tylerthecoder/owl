export NPM_CONFIG_USERCONFIG=$XDG_CONFIG_HOME/npm/npmrc
export NODE_REPL_HISTORY=$XDG_CONFIG_HOME/node/repl_hist

mkdir -p "$XDG_CONFIG_HOME/npm"
mkdir -p "$XDG_CONFIG_HOME/yarn"

# Don't use yarn global, use bun
# [ -x "$(command -v yarn)" ] && export PATH="$(yarn global bin):$PATH"

[ -z "$NVM_DIR" ] && export NVM_DIR="$XDG_DATA_HOME"/nvm

# Make local function node-setup
fn node-setup() {
    echo "Sourcing node..."
    source /usr/share/nvm/init-nvm.sh
}

fn node() {
    unset -f node
    node-setup
    node "$@"
}

fn yarn() {
    unset -f yarn
    node-setup
    [ ! -x "$(command -v yarn)" ] && npm install -g yarn
    # IDK but this doesn't work
    # alias yarn='yarn --use-yarnrc "$XDG_CONFIG_HOME/yarn/config"'
    yarn "$@"
}

