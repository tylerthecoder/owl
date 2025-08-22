if command -v bun &> /dev/null; then
    exit 0
fi

# Source the rc.sh file to set up environment variables
SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/bun-rc.sh"

curl -fsSL https://bun.sh/install | bash
