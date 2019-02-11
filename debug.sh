#!/usr/bin/env bash
cargo build --target wasm32-unknown-unknown
mkdir -p ./pkg
wasm-bindgen target/wasm32-unknown-unknown/debug/festera_seed.wasm --no-modules --out-dir ./pkg --out-name package