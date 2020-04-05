#!/bin/bash

set -e

echo "building client"
pushd client
cargo web build --release --target=wasm32-unknown-unknown
popd
echo "client build complete"

cp client/target/wasm32-unknown-unknown/release/client.js server/static/client.js
cp client/target/wasm32-unknown-unknown/release/client.wasm server/static/client.wasm

(
  echo "running server"
  cd server
  cargo run --release
)


