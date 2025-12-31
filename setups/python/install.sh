curl https://pyenv.run | bash

# Ask user for python version
read -p "Enter the Python version you want to install: " python_version

pyenv install $python_version
pyenv global $python_version
