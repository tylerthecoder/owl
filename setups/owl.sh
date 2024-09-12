curl -L https://github.com/tylerthecoder/owl/releases/download/main/owl -o owl
chmod +x owl
mkdir -p ~/.local/bin
mv owl ~/.local/bin/owl

if [ ! -d ~/owl ]; then
    git clone git@github.com:tylerthecoder/owl.git ~/owl
else
    echo "~/owl directory already exists. Skipping clone."
fi

~/.local/bin/owl install
