mkdir -p $HOME/downloads
mkdir -p $HOME/media/imgs
mkdir -p $HOME/media/videos
mkdir -p $HOME/media/music
mkdir -p $HOME/desktop
mkdir -p $HOME/docs
mkdir -p $HOME/docs/share
mkdir -p $HOME/docs/templates

xdg-user-dirs-update --set DOWNLOAD $HOME
xdg-user-dirs-update --set DESKTOP $HOME/desktop
xdg-user-dirs-update --set DOCUMENTS $HOME/docs
xdg-user-dirs-update --set PUBLICSHARE $HOME/docs/share
xdg-user-dirs-update --set TEMPLATES $HOME/docs/templates
xdg-user-dirs-update --set MUSIC $HOME/media/music
xdg-user-dirs-update --set VIDEOS $HOME/media/videos
xdg-user-dirs-update --set PICTURES $HOME/media/imgs
xdg-user-dirs-update --set STATE $HOME/.local/state
