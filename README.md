This is my setup for all of my computers
Have configs and scripts for every different OS I use
Any file in a script's directory will be added to path


## Getting Started

Multiple environment variables are used to control the setup.
`OWL_PATH` is the location of this directory.
`OWL_DEFAULT_LINK` is the location of the default link file.


Run the owl script `./common/scripts/owl.sh`



## Terms
**Link files** are json config files that map files in this repo to their location on the system.


Desks are organizations of hardware.



## Ideas

I'd like to have setup files that setup each piece of software that I use.

Setups can append to the links, run commands, and append to the shellenv file. That way they can be isolated functions that run on computer.
