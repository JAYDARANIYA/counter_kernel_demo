eval $(opam env --switch=for_tezos --set-switch)
octez-smart-rollup-wasm-debugger target/wasm32-unknown-unknown/release/counter_kernel_demo.wasm --inputs inputs.json