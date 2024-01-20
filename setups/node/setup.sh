export VOLTA_HOME="$HOME/.config/volta"

curl https://get.volta.sh | bash -s -- --skip-setup

export PATH="$VOLTA_HOME/bin:$PATH"

volta install node
volta install yarn@1

