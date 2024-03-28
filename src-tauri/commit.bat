@echo off

cargo fmt -- --emit files
git add --all
git commit
