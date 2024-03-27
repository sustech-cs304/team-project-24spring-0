#!/bin/sh

cargo fmt -- --emit files
git add --all
git commit
