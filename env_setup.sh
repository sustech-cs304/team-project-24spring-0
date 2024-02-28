#!/bin/sh

cd src-ui && npm install
cargo install tauri-cli
echo -e "\n\n==========================================================================\nYou need to add $HOME/.cargo/bin to your PATH maunally !!!\nAdd the following command to your .bashrc or .zshrc file\n\nexport PATH=\$PATH:\$HOME/.cargo/bin\n\nThen run 'source ~/.bashrc' or 'source ~/.zshrc' to apply the changes"
