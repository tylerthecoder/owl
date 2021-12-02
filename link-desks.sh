mkdir -p ~/.desks

for f in ./ubuntu-thinkpad/desks/*; do
	echo "Linking $f"
	ln -f -T ${f} ~/.desks/$(basename ${f})
done
