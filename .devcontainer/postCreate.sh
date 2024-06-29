#!/usr/bin/env bash

git config --global --add safe.directory "/workspaces/*"

curl -fsSL https://dprint.dev/install.sh | sh
cargo install cargo-run-script
cargo install --git https://github.com/xbcsmith/ymlfxr

cat<<'A'>> $HOME/.zshrc
export GPG_TTY="$(tty)"

export DPRINT_INSTALL="$HOME/.dprint"
export PATH="$DPRINT_INSTALL/bin:$PATH"
A
