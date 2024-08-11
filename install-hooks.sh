#!/bin/bash -eu

mkdir -p .git/hooks
cp hooks/* .git/hooks/
chmod +x .git/hooks/*
