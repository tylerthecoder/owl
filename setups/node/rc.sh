export NODE_REPL_HISTORY="$XDG_DATA_HOME"/node_repl_history

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
    yarn "$@"
}

