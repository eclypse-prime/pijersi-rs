#!/bin/bash -eu

mkdir -p .git/hooks
cp hooks/* .git/
chmod +x .git/hooks/*
