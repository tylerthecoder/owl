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
- Setups should have a file that defines their links. Then each user has an "owl config" file that points to setups and has its own links. Then owl link loops through all the setups and links their files

