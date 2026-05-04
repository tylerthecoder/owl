#!/bin/bash
set -euo pipefail

# Make sure bun is on PATH (the rc script that adds it isn't sourced inside install scripts)
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if [ -f "$SCRIPT_DIR/../bun/bun-rc.sh" ]; then
    # shellcheck disable=SC1091
    source "$SCRIPT_DIR/../bun/bun-rc.sh"
fi

# Install OpenAI Codex CLI globally via bun
bun install -g @openai/codex
