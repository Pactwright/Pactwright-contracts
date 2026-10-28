#!/usr/bin/env bash
# Tier B: measure every pact entrypoint with real testnet transactions (ROADMAP C-21).
#
#   scripts/bench-testnet.sh            # writes dist/bench/<utc-date>.json and prints a table
#
# Uses a dedicated bench deployment (never the main testnet `pact`): a `PWBENCH` test token
# issued by `pactwright-sponsor`, bench buyer/seller accounts, and a bench `pact` whose token is
# that asset. Verdict attestations are signed with `pactwright-verifier`, usage vouchers with
# `pactwright-bench-meter`, through tools/pact-sign (the contract's own hashing). Secrets come
# from the stellar CLI keystore and are passed to pact-sign through the environment only.
#
# Network numbers per transaction: declared resources (instructions, disk read bytes, write
# bytes, footprint entries) and the fees actually charged, read back from RPC.
set -euo pipefail
cd "$(dirname "$0")/.."
CHECK=""
[ "${1:-}" = "--check" ] && CHECK=1

NETWORK=testnet
RPC="${STELLAR_RPC_URL:-https://soroban-testnet.stellar.org}"
ADMIN=pactwright-deployer
ISSUER=pactwright-sponsor
VERIFIER=pactwright-verifier
BUYER=pactwright-bench-buyer
SELLER=pactwright-bench-seller
METER=pactwright-bench-meter
CODE=PWBENCH
CAP=10000000            # 1 token, 7 decimals
UNIT_PRICE=1000
MAX_PACT_LEDGERS=120960
OUT="dist/bench"
mkdir -p "$OUT"
log() { echo "bench: $*" >&2; }

# ---- tools and identities -------------------------------------------------
cargo build -q --release -p pact-sign
SIGN=target/release/pact-sign
for id in "$BUYER" "$SELLER"; do
  stellar keys address "$id" >/dev/null 2>&1 || stellar keys generate "$id" --network "$NETWORK" --fund >&2
done
stellar keys address "$METER" >/dev/null 2>&1 || stellar keys generate "$METER" >&2
addr() { stellar keys address "$1"; }
ISSUER_G=$(addr "$ISSUER"); BUYER_G=$(addr "$BUYER"); SELLER_G=$(addr "$SELLER")
VERIFIER_HEX=$(PACT_SIGN_SECRET="$(stellar keys show "$VERIFIER")" "$SIGN" pubkey)
METER_HEX=$(PACT_SIGN_SECRET="$(stellar keys show "$METER")" "$SIGN" pubkey)

# ---- invoke + measure ---------------------------------------------------
RESULTS="$OUT/.rows.jsonl"; : > "$RESULTS"
TX_HASH=""
RET=""
# invoke <source> <contract> <fn> [args…]: sets RET (return value) and TX_HASH.
# Never call it inside $(…): a subshell would drop both globals.
invoke() {
  local source="$1" id="$2"; shift 2
  local err; err="$(mktemp)"
  local out
  out="$(stellar contract invoke --id "$id" --source "$source" --network "$NETWORK" -- "$@" 2>"$err")" || { cat "$err" >&2; exit 1; }
  TX_HASH="$(grep -oE 'Signing transaction: [0-9a-f]{64}' "$err" | tail -1 | awk '{print $3}')"
  [ -n "$TX_HASH" ] || { cat "$err" >&2; echo "bench: no transaction hash for $*" >&2; exit 1; }
  rm -f "$err"
  RET="$(echo "$out" | tr -d '"')"
}
rpc() { curl -fsS -X POST "$RPC" -H 'content-type: application/json' -d "$1"; }
tx_ledger() { rpc "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"getTransaction\",\"params\":{\"hash\":\"$1\"}}" | jq -r .result.ledger; }
# measure <label> <hash>: append one JSON row with resources and fees
measure() {
  local label="$1" hash="$2" tx env res
  tx="$(rpc "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"getTransaction\",\"params\":{\"hash\":\"$hash\"}}")"
  [ "$(echo "$tx" | jq -r .result.status)" = "SUCCESS" ] || { echo "$tx" >&2; exit 1; }
  env="$(echo "$tx" | jq -r .result.envelopeXdr | stellar xdr decode --type TransactionEnvelope --input single-base64 --output json)"
  res="$(echo "$tx" | jq -r .result.resultXdr | stellar xdr decode --type TransactionResult --input single-base64 --output json)"
  jq -nc --arg label "$label" --arg hash "$hash" --argjson ledger "$(echo "$tx" | jq .result.ledger)" \
    --argjson env "$env" --argjson res "$res" '{
      label: $label, tx: $hash, ledger: $ledger,
      instructions: $env.tx.tx.ext.v1.resources.instructions,
      disk_read_bytes: $env.tx.tx.ext.v1.resources.disk_read_bytes,
      write_bytes: $env.tx.tx.ext.v1.resources.write_bytes,
      read_only_entries: ($env.tx.tx.ext.v1.resources.footprint.read_only | length),
      read_write_entries: ($env.tx.tx.ext.v1.resources.footprint.read_write | length),
      resource_fee: ($env.tx.tx.ext.v1.resource_fee | tonumber),
      fee_charged: ($res.fee_charged | tonumber)
    }' >> "$RESULTS"
  log "measured $label ($hash)"
}

# ---- bench token ---------------------------------------------------------
SAC=$(stellar contract id asset --asset "$CODE:$ISSUER_G" --network "$NETWORK")
if ! stellar contract invoke --id "$SAC" --source "$ADMIN" --network "$NETWORK" --send=no -- symbol >/dev/null 2>&1; then
  log "deploying $CODE asset contract"
  stellar contract asset deploy --asset "$CODE:$ISSUER_G" --source "$ADMIN" --network "$NETWORK" >&2
fi
for id in "$BUYER" "$SELLER"; do
  stellar tx new change-trust --source "$id" --line "$CODE:$ISSUER_G" --network "$NETWORK" >/dev/null 2>&1 || true
done
invoke "$ISSUER" "$SAC" mint --to "$BUYER_G" --amount $((CAP * 10))

# ---- bench pact ------------------------------------------------------------
read -r WASM WASM_HASH < <(scripts/build.sh)
log "deploying bench pact ($WASM_HASH)"
PACT=$(stellar contract deploy --wasm "$WASM" --source "$ADMIN" --network "$NETWORK" -- \
  --admin "$(addr "$ADMIN")" --token "$SAC" --verifier "$VERIFIER_HEX" --max_pact_ledgers "$MAX_PACT_LEDGERS" 2>/dev/null)

COND=$(printf 'pactwright-bench' | shasum -a 256 | cut -d' ' -f1)
ENVELOPE=$(printf 'bench-envelope' | shasum -a 256 | cut -d' ' -f1)
NONCE=$(date +%s)
latest() { rpc '{"jsonrpc":"2.0","id":1,"method":"getLatestLedger"}' | jq .result.sequence; }
open_params() { # nonce meter-json
  local now; now=$(latest)
  jq -nc --arg b "$BUYER_G" --arg s "$SELLER_G" --arg c "$COND" --argjson n "$1" --argjson m "$2" \
    --argjson vd $((now + 120)) --argjson sd $((now + 720)) --arg cap "$CAP" \
    '{buyer:$b, seller:$s, cap:$cap, nonce:$n, condition_hash:$c, meter:$m, silence:1, verdict_deadline:$vd, settle_deadline:$sd}'
}
attestation() { # pact_id verdict(1|2) amount ledger
  jq -nc --arg p "$1" --arg c "$COND" --arg e "$ENVELOPE" --argjson v "$2" --arg a "$3" --argjson l "$4" \
    '{pact_id:$p, condition_hash:$c, envelope_hash:$e, verdict:$v, amount:$a, issued_ledger:$l}'
}
verifier_sign() { PACT_SIGN_SECRET="$(stellar keys show "$VERIFIER")" "$SIGN" verdict "$@"; }

# Flow A: open → ALLOW → partial settle (two transfers)
invoke "$BUYER" "$PACT" open --params "$(open_params "$NONCE" '"None"')"; ID_A="$RET"; measure open "$TX_HASH"
L=$(tx_ledger "$TX_HASH")
SIG=$(verifier_sign "$ID_A" "$COND" "$ENVELOPE" allow "$CAP" "$L")
invoke "$ADMIN" "$PACT" record_verdict --pact_id "$ID_A" --attestation "$(attestation "$ID_A" 1 "$CAP" "$L")" --signature "$SIG"
measure record_verdict "$TX_HASH"
invoke "$SELLER" "$PACT" settle --pact_id "$ID_A" --actual $((CAP / 2)); measure settle "$TX_HASH"

# Flow B: open → DENY → release
invoke "$BUYER" "$PACT" open --params "$(open_params $((NONCE + 1)) '"None"')"; ID_B="$RET"
L=$(tx_ledger "$TX_HASH")
SIG=$(verifier_sign "$ID_B" "$COND" "$ENVELOPE" deny 0 "$L")
invoke "$ADMIN" "$PACT" record_verdict --pact_id "$ID_B" --attestation "$(attestation "$ID_B" 2 0 "$L")" --signature "$SIG"
invoke "$ADMIN" "$PACT" release --pact_id "$ID_B"; measure release "$TX_HASH"

# Flow C: metered open → close_meter (500 units, two transfers)
METERING=$(jq -nc --arg k "$METER_HEX" --arg p "$UNIT_PRICE" '{Units:{meter_key:$k, unit_price:$p}}')
invoke "$BUYER" "$PACT" open --params "$(open_params $((NONCE + 2)) "$METERING")"; ID_C="$RET"
USAGE_SIG=$(PACT_SIGN_SECRET="$(stellar keys show "$METER")" "$SIGN" usage "$ID_C" 500)
invoke "$SELLER" "$PACT" close_meter --pact_id "$ID_C" --units 500 --signature "$USAGE_SIG"
measure close_meter "$TX_HASH"

# ---- report -----------------------------------------------------------------
REPORT="$OUT/$(date -u +%Y-%m-%d).json"
jq -s --arg pact "$PACT" --arg sac "$SAC" --arg wasm "$WASM_HASH" \
  '{network:"testnet", bench_pact:$pact, token:$sac, wasm:$wasm, rows:.}' "$RESULTS" > "$REPORT"
rm -f "$RESULTS"
jq -r '"| entrypoint | instructions | disk read B | write B | RO/RW entries | resource fee | fee charged | tx |",
       "|---|---|---|---|---|---|---|---|",
       (.rows[] | "| `\(.label)` | \(.instructions) | \(.disk_read_bytes) | \(.write_bytes) | \(.read_only_entries)/\(.read_write_entries) | \(.resource_fee) | \(.fee_charged) | [\(.tx[0:8])…](https://stellar.expert/explorer/testnet/tx/\(.tx)) |")' "$REPORT"
log "report: $REPORT (bench pact $PACT)"

# ---- --check: compare against the Tier B limits in budget.toml --------------
if [ -n "$CHECK" ]; then
  python3 - "$REPORT" <<'PY'
import json, re, sys
limits = {}
for name, body in re.findall(r"^(\w+)\s*=\s*\{([^}]*)\}", open("budget.toml").read(), flags=re.M):
    limits[name] = {k: int(v.replace("_", "")) for k, v in re.findall(r"(\w+)\s*=\s*([\d_]+)", body)}
failed = False
for row in json.load(open(sys.argv[1]))["rows"]:
    for metric, limit in limits.get(row["label"], {}).items():
        status = "FAIL" if row[metric] > limit else "ok"
        failed |= status == "FAIL"
        print(f"{status:4} {row['label']:15} {metric:16} {row[metric]:>10} <= {limit}")
sys.exit(1 if failed else 0)
PY
fi
