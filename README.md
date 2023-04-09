# Counter kernel demo

This is a simple demo counter wasm kernel for [Tezos SORU](http://tezos.gitlab.io/alpha/smart_rollups.html).

Kernal have 3 user actions 
- increment -> 00
- decrement -> 01
- reset     -> 02

## How to setup for mac

```bash
chmod +x setup_mac.sh
./setup_mac.sh
```

or

```bash
xcode-select --install

# Install OPAM
brew install opam

# Initialize OPAM
opam init --disable-sandboxing

# Set up environment variables
opam switch create for_tezos 4.14.1

# Evalute the environment variables
eval $(opam env --switch=for_tezos --set-switch)

# Install octez-smart-rollup-wasm-debugger
opam install octez-smart-rollup-wasm-debugger
```

## How to build make)
```bash
chmod +x build.sh
./build.sh
```
or
```bash
CC=emcc AR=emar cargo build --release --target wasm32-unknown-unknown
```


