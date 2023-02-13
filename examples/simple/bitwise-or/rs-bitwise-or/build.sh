#!/bin/sh

cargo \
  build \
  --target wasm32-unknown-unknown \
  --profile release-wasm
