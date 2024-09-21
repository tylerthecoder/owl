if [ ! -d ~/owl ]; then
    echo "installing owl..."
    curl -L https://github.com/tylerthecoder/owl/releases/download/main/owl -o owl-bin
    chmod +x owl-bin
    mv owl-bin /usr/local/bin/owl
    git clone git@github.com:tylerthecoder/owl.git ~/owl
else
    echo "owl already installed, updating..."
    git -C ~/owl pull
fi
