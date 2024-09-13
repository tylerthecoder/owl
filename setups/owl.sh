curl -L https://github.com/tylerthecoder/owl/releases/download/main/owl -o owl
chmod +x owl
mkdir -p ~/.local/bin

if [ ! -f ~/.local/bin/owl ]; then
    mv owl ~/.local/bin/owl
fi

if [ ! -d ~/owl ]; then
    git clone git@github.com:tylerthecoder/owl.git ~/owl
else
    echo "~/owl directory already exists. Skipping clone."
fi

~/.local/bin/owl
