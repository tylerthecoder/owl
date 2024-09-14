curl -L https://github.com/tylerthecoder/owl/releases/download/main/owl -o owl-bin
chmod +x owl-bin
mkdir -p ~/.local/bin

if [ ! -f ~/.local/bin/owl ]; then
    mv owl-bin ~/.local/bin/owl
fi

if [ ! -d ~/owl ]; then
    git clone git@github.com:tylerthecoder/owl.git ~/owl
else
    git -C ~/owl pull
fi

~/.local/bin/owl
