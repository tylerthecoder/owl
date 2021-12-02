#!/bin/bash

IMG_PATH=$(curl https://apod.nasa.gov/apod/astropix.html | grep -i img | grep -o '".*"' | sed 's/"//g')

IMG_URL="https://apod.nasa.gov/apod/$IMG_PATH"

curl $IMG_URL -o /tmp/nasa-img-of-day

feh --no-fehbg --bg-scale /tmp/nasa-img-of-day

