#!/bin/bash

set -e

echo "cleaning client"
pushd client
cargo clean
popd
echo "client clean complete"

(
  echo "cleaning server"
  cd server
  cargo clean
  echo "server clean complete"
)


