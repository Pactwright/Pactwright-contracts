#!/usr/bin/env bash
# Build every release asset into dist/release/ (ROADMAP C-18):
#   pactwright_pact.wasm, pactwright_pact.wasm.sha256, pactwright-pact-bindings.tgz,
#   test-vectors.tgz, and NOTES.md (CHANGELOG [Unreleased] + hashes).
# Usage: scripts/release-assets.sh <tag>
set -euo pipefail
cd "$(dirname "$0")/.."
tag="${1:?usage: release-assets.sh <tag>}"
out="dist/release"
rm -rf "$out" && mkdir -p "$out"

read -r wasm hash < <(scripts/build.sh)
cp "$wasm" "$out/pactwright_pact.wasm"
echo "$hash  pactwright_pact.wasm" > "$out/pactwright_pact.wasm.sha256"

bindings="$(scripts/bindings.sh)"
cp "$bindings" "$out/"

tar -czf "$out/test-vectors.tgz" test-vectors

{
  echo "# Pactwright contracts ${tag}"
  echo
  echo "| Asset | sha256 |"
  echo "|---|---|"
  for f in pactwright_pact.wasm pactwright-pact-bindings.tgz test-vectors.tgz; do
    echo "| \`$f\` | \`$( (command -v sha256sum >/dev/null && sha256sum "$out/$f" || shasum -a 256 "$out/$f") | cut -d' ' -f1)\` |"
  done
  echo
  echo "The WASM sha256 is the on-chain WASM hash. Rebuild with \`scripts/build.sh\` (stellar-cli 28.1.0, rustc 1.98.1) to verify."
  echo
  awk '/^## \[Unreleased\]/{f=1; next} /^## \[/{f=0} f' CHANGELOG.md
} > "$out/NOTES.md"

ls "$out"
