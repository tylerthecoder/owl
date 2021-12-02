#! /bin/bash


# Get all the desk configuration files
desk=$(ls ~/.desk)


for f in ~/.desk; do
		bash $f
done


