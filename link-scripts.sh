

for f in ./common/scripts/*;  do
 echo ${f};
 # Get the filename without the path
 filename=$(basename "$f")
 echo ${filename};
 sudo ln -f -T ${f} /usr/local/bin/${filename}
done;

for f in ./common/rofi-scripts/*;  do
 echo ${f};
 # Get the filename without the path
 filename=$(basename "$f")
 echo ${filename};
 sudo ln -f -T ${f} /usr/local/bin/${filename}
done;

