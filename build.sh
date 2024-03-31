#!/bin/sh
# dependencies:
# - cargo
# - rustc
# - wasm-bindgen
cargo build --lib --release --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir ./docs/wasm ./target/wasm32-unknown-unknown/release/crossyword.wasm
