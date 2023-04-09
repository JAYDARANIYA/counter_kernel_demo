#!/bin/bash

# Install Xcode command line tools
xcode-select --install

# Install Homebrew
# /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# rustup
rustup target add wasm32-unknown-unknown

# Install OPAM
brew install opam

# Initialize OPAM
opam init --disable-sandboxing

# Set up environment variables
opam switch create for_tezos 4.14.1

# Evalute the environment variables
eval $(opam env --switch=for_tezos --set-switch)

# Install octez-smart-rollup-wasm-debugger
CC=emcc AR=emar opam install octez-smart-rollup-wasm-debugger