#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
#wasm-build --target=wasm32-unknown-unknown ./target pwasm_tutorial_contract
wasm-build --target=wasm32-unknown-unknown --skip-optimization ./target pwasm_tutorial_contract
