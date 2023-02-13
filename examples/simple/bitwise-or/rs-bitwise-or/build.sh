#!/bin/sh

cargo \
  build \
  --target wasm32-unknown-unknown \
  --profile release-wasm \
  || exec echo Unable to build

original=./target/wasm32-unknown-unknown/release-wasm/rs_bitwise_or.wasm
test -f "${original}" || exec echo Original wasm file not found.

conv=./target/wasm32-unknown-unknown/release-wasm/rs_bitwise_or.converted.wasm

which wasm-opt > /dev/null || exec echo Skipping optimization.

wasm-opt -O3 -o "${conv}" "${original}"
