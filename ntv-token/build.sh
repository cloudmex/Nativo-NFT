#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo +stable build --all --target wasm32-unknown-unknown --release
rsync -u target/wasm32-unknown-unknown/release/ntv_token.wasm res/

set -ex
NETWORK=testnet
OWNER=.$NETWORK
MASTER_ACC=nativo_token.$NETWORK
CONTRACT_ACC=$nativo_token.MASTER_ACC
export NODE_ENV=$NETWORK

near dev-deploy --wasmFile res/ntv_token.wasm