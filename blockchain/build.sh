#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo +stable build --all --target wasm32-unknown-unknown --release
rsync -u target/wasm32-unknown-unknown/release/nativo_marketplace.wasm res/


near dev-deploy --wasmFile res/ntv_token.wasm