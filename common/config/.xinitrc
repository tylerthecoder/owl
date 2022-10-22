# Set the cursor speed (wait time, dups per second)
xset r rate 300 35

# Set the "theme"
[[ -f ~/.config/X/.Xresources ]] && xrdb -merge -I$HOME ~/.config/X/.Xresources

setxkbmap -option caps:super

# Start greenclip
greenclip daemon &

# Start i3
exec i3
