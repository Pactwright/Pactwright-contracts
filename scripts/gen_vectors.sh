#!/usr/bin/env bash
# Regenerate test-vectors/*.json from the contract crate (ADR-015).
# `cargo test` fails whenever the committed vectors drift from the code.
set -euo pipefail
cd "$(dirname "$0")/.."
PACTWRIGHT_WRITE_VECTORS=1 cargo test -p pactwright-pact vectors -- --quiet
echo "test-vectors/ regenerated. Review the diff before committing."
