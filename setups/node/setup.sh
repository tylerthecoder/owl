export VOLTA_HOME="$HOME/.config/volta"

curl https://get.volta.sh | bash -s -- --skip-setup

volta install node
volta install yarn@1

