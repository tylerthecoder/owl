SCRIPT_DIR="$(dirname "$0")"


op inject --in-file $SCRIPT_DIR/example.secrets.sh --out-file $SCRIPT_DIR/secrets.sh
