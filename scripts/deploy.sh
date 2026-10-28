#!/usr/bin/env bash
# Deploy Pactwright contracts. ROADMAP C-17 (pact), C-20 (receipt-anchor).
#
#   scripts/deploy.sh <testnet|pubnet> [pact|receipt-anchor|all]
#
# Reads inputs from deployments/<network>.env and writes the contract IDs and WASM hashes
# back into it. Pubnet refuses to run while any input is empty and requires
# PACTWRIGHT_CONFIRM_PUBNET=yes. Secrets never leave the stellar CLI keystore.
set -euo pipefail
cd "$(dirname "$0")/.."

target="${1:?usage: deploy.sh <testnet|pubnet> [pact|receipt-anchor|all]}"
what="${2:-all}"
envfile="deployments/${target}.env"
[ -f "$envfile" ] || { echo "error: $envfile not found" >&2; exit 1; }
# shellcheck disable=SC1090
source "$envfile"

missing=()
for key in NETWORK SOURCE ANCHORER USDC_SAC_ID VERIFIER_PUBLIC_KEY MAX_PACT_LEDGERS; do
  [ -n "${!key:-}" ] || missing+=("$key")
done
if [ ${#missing[@]} -gt 0 ]; then
  echo "error: $envfile has empty inputs: ${missing[*]}" >&2
  exit 1
fi
if [ "$target" = "pubnet" ] && [ "${PACTWRIGHT_CONFIRM_PUBNET:-}" != "yes" ]; then
  echo "error: refusing to deploy to pubnet without PACTWRIGHT_CONFIRM_PUBNET=yes" >&2
  exit 1
fi
[[ "$VERIFIER_PUBLIC_KEY" =~ ^[0-9a-f]{64}$ ]] || { echo "error: VERIFIER_PUBLIC_KEY must be 64 lowercase hex chars" >&2; exit 1; }

set_output() { # key value
  local tmp; tmp="$(mktemp)"
  awk -v k="$1" -v v="$2" 'BEGIN{FS=OFS="="} $1==k{$0=k"="v} {print}' "$envfile" > "$tmp" && mv "$tmp" "$envfile"
}

sha256() { (command -v sha256sum >/dev/null && sha256sum "$1" || shasum -a 256 "$1") | cut -d' ' -f1; }

deploy_pact() {
  read -r wasm hash < <(scripts/build.sh)
  local admin; admin="$(stellar keys address "$SOURCE")"
  echo "deploying pact ($hash) to $NETWORK as $admin" >&2
  local id
  id="$(stellar contract deploy --wasm "$wasm" --source "$SOURCE" --network "$NETWORK" -- \
    --admin "$admin" --token "$USDC_SAC_ID" --verifier "$VERIFIER_PUBLIC_KEY" \
    --max_pact_ledgers "$MAX_PACT_LEDGERS")"
  set_output PACT_CONTRACT_ID "$id"
  set_output PACT_WASM_HASH "$hash"
  echo "pact: $id" >&2
}

deploy_receipt_anchor() {
  scripts/check_vendor_hashes.sh >&2
  local wasm="vendor/wasm/receipt_anchor.wasm" hash id merchant
  hash="$(sha256 "$wasm")"
  merchant="$(stellar keys address "$ANCHORER")"
  echo "deploying receipt-anchor ($hash) to $NETWORK, admin $merchant" >&2
  id="$(stellar contract deploy --wasm "$wasm" --source "$SOURCE" --network "$NETWORK")"
  stellar contract invoke --id "$id" --source "$SOURCE" --network "$NETWORK" -- initialize --merchant "$merchant" >&2
  set_output RECEIPT_ANCHOR_ID "$id"
  set_output RECEIPT_ANCHOR_WASM_HASH "$hash"
  echo "receipt-anchor: $id" >&2
}

case "$what" in
  pact) deploy_pact ;;
  receipt-anchor) deploy_receipt_anchor ;;
  all) deploy_pact; deploy_receipt_anchor ;;
  *) echo "error: unknown target '$what'" >&2; exit 1 ;;
esac
echo "updated $envfile; record provenance in DEPLOYMENTS.md" >&2
