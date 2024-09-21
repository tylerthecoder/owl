if [ ! -d ~/owl ]; then
    echo "installing owl..."
    git clone git@github.com:tylerthecoder/owl.git ~/owl
else
    echo "owl already installed, updating..."
    git -C ~/owl pull
fi

# install owl if not in path
if ! command -v owl &> /dev/null; then
    echo "owl not in path, installing..."
    curl -L https://github.com/tylerthecoder/owl/releases/download/main/owl -o owl-bin
    chmod +x owl-bin
    mv owl-bin /usr/local/bin/owl
fi

