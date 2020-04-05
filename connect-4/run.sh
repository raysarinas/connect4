#!/bin/bash

set -e

echo "building client"
pushd client
cargo web deploy
popd
echo "client build complete"

cp client/target/deploy/* server/static/

(
  echo "running server"
  cd server
  cargo run
)


