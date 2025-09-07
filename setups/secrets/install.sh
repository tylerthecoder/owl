sudo pacman -S polkit

# Check if op cli is installed
if ! command -v op &> /dev/null
then
    yay -S 1password-cli 
fi

op account add --shorthand personal
