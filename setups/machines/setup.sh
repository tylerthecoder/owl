sudo pacman -S sftpman

SCRIPT_DIR="$(dirname "$0")"


op inject --in-file $SCRIPT_DIR/ssh_config_template --out-file $SCRIPT_DIR/ssh_config.secret
