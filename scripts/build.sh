#!/usr/bin/env bash
# Build the pact contract WASM and print its sha256 (= the on-chain WASM hash).
# ROADMAP C-17.
set -euo pipefail
cd "$(dirname "$0")/.."
stellar contract build --locked --package pactwright-pact >&2
wasm="target/wasm32v1-none/release/pactwright_pact.wasm"
hash="$( (command -v sha256sum >/dev/null && sha256sum "$wasm" || shasum -a 256 "$wasm") | cut -d' ' -f1)"
echo "$wasm $hash"
