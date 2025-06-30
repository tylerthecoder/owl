
# Check if op cli is installed
if ! command -v op &> /dev/null
then
    yay -S 1password-cli 
fi

op inject --in-file example.secrets.sh --out-file secrets.sh
