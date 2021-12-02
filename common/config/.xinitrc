# Set the cursor speed (wait time, dups per second)
xset r rate 300 35

# Set the "theme"
[[ -f ~/.config/X/.Xresources ]] && xrdb -merge -I$HOME ~/.config/X/.Xresources

setxkbmap -option caps:super

xrandr --setprovideroutputsource modesetting NVIDIA-0
xrandr --auto

# Start GNOME keyring
eval $(/usr/bin/gnome-keyring-daemon --start --components=pkcs11,secrets,ssh)
export SSH_AUTH_SOCK

# Start greenclip
greenclip daemon &

# Start i3
exec i3
