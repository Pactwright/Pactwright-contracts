#!/usr/bin/env bash
# Generate TypeScript bindings for the pact contract into dist/bindings/ and pack them as
# dist/pactwright-pact-bindings.tgz (release asset, consumed by Pactwright-Kit K-05).
# ROADMAP C-17, C-18.
set -euo pipefail
cd "$(dirname "$0")/.."
read -r wasm _hash < <(scripts/build.sh)
rm -rf dist/bindings && mkdir -p dist
stellar contract bindings typescript --wasm "$wasm" --output-dir dist/bindings --overwrite >&2
tar -czf dist/pactwright-pact-bindings.tgz -C dist bindings
echo "dist/pactwright-pact-bindings.tgz"
