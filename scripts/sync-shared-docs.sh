#!/usr/bin/env bash
# XC-09: Synchronize shared design documents across Pactwright repositories on release.
# Shared docs: ARCHITECTURE_ESSENTIALS.md, ARCHITECTURE.md, PRD.md, project_structure.md.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKSPACE_ROOT="$(cd "${REPO_ROOT}/.." && pwd)"

DOCS=(
  "ARCHITECTURE_ESSENTIALS.md"
  "ARCHITECTURE.md"
  "PRD.md"
  "project_structure.md"
)

echo "==> Verifying and syncing shared design documents..."
for doc in "${DOCS[@]}"; do
  src="${WORKSPACE_ROOT}/pactwright.github.io/${doc}"
  dst="${REPO_ROOT}/docs/${doc}"
  if [[ -f "$src" && -f "$dst" ]]; then
    if ! cmp -s "$src" "$dst"; then
      echo "Syncing ${doc} from master docs..."
      cp "$src" "$dst"
    else
      echo "✓ ${doc} is in sync."
    fi
  elif [[ -f "$src" ]]; then
    echo "Copying new shared doc ${doc}..."
    cp "$src" "$dst"
  fi
done

echo "==> Shared documents verified."
