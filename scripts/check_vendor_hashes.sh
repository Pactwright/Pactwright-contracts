#!/usr/bin/env bash
# Verify vendor/wasm/*.wasm against SHA256SUMS (ADR-008).
# Fails if a listed file is missing or changed, or if an unlisted .wasm is present.
set -euo pipefail
cd "$(dirname "$0")/../vendor/wasm"

if command -v sha256sum >/dev/null 2>&1; then
  sha256sum --check --strict SHA256SUMS
else
  shasum -a 256 --check --strict SHA256SUMS
fi

for f in *.wasm; do
  if ! grep -q "  ${f}\$" SHA256SUMS; then
    echo "error: ${f} is not listed in SHA256SUMS" >&2
    exit 1
  fi
done
echo "vendor/wasm: all hashes match."
