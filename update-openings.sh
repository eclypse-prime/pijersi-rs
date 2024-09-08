#!/bin/bash -eu

echo Checking remote openings...
LATEST_TAG="$(curl --silent  "https://api.github.com/repos/eclypse-prime/pijersi-toolbox/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')"
CURRENT_TAG="$(cat data/opening_tag)"
echo Remote latest tag: "$LATEST_TAG"
echo Local latest tag: "$CURRENT_TAG"

if ! (printf "%s\n%s" "$LATEST_TAG" "$CURRENT_TAG" | sort -V -C); then
    echo Remote tag is higher, updating local opening book...
elif [ ! -f data/openings ]; then
    echo No local openings, downloading remote opening book...
    wget https://github.com/eclypse-prime/pijersi-toolbox/releases/latest/download/openings -O data/openings -q
    echo "$LATEST_TAG" > data/opening_tag
    echo Remote opening book downloaded at version "$LATEST_TAG"
else
    echo Local tag equal or higher to remote tag. Nothing to be done.
fi
