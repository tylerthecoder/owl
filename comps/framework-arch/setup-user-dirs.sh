mkdir -p $HOME/downloads
mkdir -p $HOME/media/imgs

mkdir -p $HOME/media/videos
mkdir -p $HOME/media/music
mkdir -p $HOME/desktop
mkdir -p $HOME/documents
mkdir -p $HOME/documents/share
mkdir -p $HOME/documents/templates

# setup user dirs
xdg-user-dirs-update --set DOWNLOAD $HOME
xdg-user-dirs-update --set DESKTOP $HOME/desktop
xdg-user-dirs-update --set DOCUMENTS $HOME/documents
xdg-user-dirs-update --set PUBLICSHARE $HOME/documents/share
xdg-user-dirs-update --set TEMPLATES $HOME/documents/templates
xdg-user-dirs-update --set MUSIC $HOME/media/music
xdg-user-dirs-update --set VIDEOS $HOME/media/videos
xdg-user-dirs-update --set PICTURES $HOME/media/imgs

