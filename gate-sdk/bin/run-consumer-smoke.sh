#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — scratch-consumer install+violation harness (a bin/ tool, never a registered gate)
#
# usage: run-consumer-smoke.sh [--keep] [kit-root...]
#   Builds a scratch consumer in a fresh temp dir, vendors each kit root by copy,
#   runs each kit's smoke/install.sh (gate-sdk first, then argument order), and
#   asserts the full battery is green under ZERO consumer config. Then, per kit
#   shipping smoke/violation.sh, fires one crafted violation, asserts the battery
#   goes red at the named gate, and restores the tree; asserts green once more
#   after the last restore. Kit roots default to gate_kit_roots resolution.
#
#   It builds a repo and runs the battery repeatedly, so it is pre-commit-unfit
#   by runtime budget and is never a registered gate. Exit 0 all assertions
#   hold, 1 an assertion failed, 2 usage/environment.
#
#   --keep retains the temp dir and prints its path (the temp-dir write's named
#   reclaim path); otherwise it is removed on exit.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

KEEP=0
kit_args=()
for a in "$@"; do
    case "$a" in
        --keep) KEEP=1 ;;
        -*) echo "run-consumer-smoke: unknown option: $a" >&2; exit 2 ;;
        *) kit_args+=("$a") ;;
    esac
done

# Resolve kit roots: explicit args, else gate_kit_roots. gate-sdk (this kit)
# installs first; the rest follow in resolution/argument order.
roots=()
if [[ ${#kit_args[@]} -gt 0 ]]; then
    for r in "${kit_args[@]}"; do
        [[ -d "$r" ]] || { echo "run-consumer-smoke: not a directory: $r" >&2; exit 2; }
        roots+=("$(cd "$r" && pwd)")
    done
else
    while IFS= read -r r; do roots+=("$r"); done < <(gate_kit_roots)
fi

# Order gate-sdk first.
ordered=("$SDK")
for r in "${roots[@]}"; do
    [[ "$r" == "$SDK" ]] && continue
    ordered+=("$r")
done
roots=("${ordered[@]}")

# Every kit root must ship smoke/install.sh (the kit-landing contract).
for r in "${roots[@]}"; do
    [[ -f "$r/smoke/install.sh" ]] || {
        echo "run-consumer-smoke: $r has no smoke/install.sh — a vendored kit must ship one" >&2
        echo "  help: add smoke/install.sh (+ optional smoke/violation.sh); see gate-sdk/SPEC.md §Consumer smoke." >&2
        exit 2
    }
done

SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/consumer-smoke.XXXXXX")"
cleanup() {
    if [[ "$KEEP" == "1" ]]; then
        echo "CONSUMER-SMOKE: --keep, scratch retained at $SCRATCH"
    else
        rm -rf "$SCRATCH"
    fi
}
trap cleanup EXIT

git -C "$SCRATCH" init -q
printf '.tmp/\n' > "$SCRATCH/.gitignore"
git -C "$SCRATCH" add -A
git -C "$SCRATCH" -c user.email=smoke@example.invalid -c user.name=smoke \
    commit -q --allow-empty -m "seed"

# Vendor each kit root by copy (basename under the scratch root).
for r in "${roots[@]}"; do
    cp -R "$r" "$SCRATCH/$(basename "$r")"
done

# --- install phase --------------------------------------------------------
installed=0
for r in "${roots[@]}"; do
    kit="$(basename "$r")"
    if ! ( cd "$SCRATCH" && SMOKE_KIT_ROOT="$SCRATCH/$kit" bash "$SCRATCH/$kit/smoke/install.sh" ); then
        echo "run-consumer-smoke: $kit/smoke/install.sh failed (a broken installer is an environment failure)" >&2
        exit 2
    fi
    installed=$((installed + 1))
done

# Commit the installed baseline (so a violation restore returns to green).
git -C "$SCRATCH" add -A
git -C "$SCRATCH" -c user.email=smoke@example.invalid -c user.name=smoke \
    commit -q --no-verify -m "installed baseline"

# --- assert green under zero config ---------------------------------------
run_battery() { ( cd "$SCRATCH" && bash gate-sdk/bin/run-gates.sh ) 2>&1; }

out="$(run_battery)"; rc=$?
if [[ "$rc" -ne 0 ]] || ! grep -qE 'All [0-9]+ gates passed' <<<"$out"; then
    echo "CONSUMER-SMOKE: FAIL — the battery is not green on the freshly installed consumer (zero config)"
    printf '%s\n' "$out"
    echo "  help: an install step left a gate red; reproduce with --keep and run gate-sdk/bin/run-gates.sh in the scratch dir."
    exit 1
fi

# --- violation phase ------------------------------------------------------
restore() { ( cd "$SCRATCH" && git checkout -q -- . && git clean -qfd ); }

fired=0
for r in "${roots[@]}"; do
    kit="$(basename "$r")"
    vio="$SCRATCH/$kit/smoke/violation.sh"
    if [[ ! -f "$vio" ]]; then
        echo "CONSUMER-SMOKE: $kit has no violation script — install coverage only"
        continue
    fi
    expected="$( ( cd "$SCRATCH" && SMOKE_KIT_ROOT="$SCRATCH/$kit" bash "$vio" ) | head -n1 )"
    if [[ -z "$expected" ]]; then
        echo "CONSUMER-SMOKE: FAIL — $kit/smoke/violation.sh printed no expected-gate name on line 1"
        restore
        exit 1
    fi
    out="$(run_battery)"; rc=$?
    if [[ "$rc" -eq 0 ]]; then
        echo "CONSUMER-SMOKE: FAIL — $kit violation did not turn the battery red (expected gate $expected)"
        printf '%s\n' "$out"
        restore
        exit 1
    fi
    if ! grep -qF "FAIL: $expected" <<<"$out"; then
        echo "CONSUMER-SMOKE: FAIL — $kit violation fired, but no 'FAIL: $expected' line (wrong gate caught it)"
        printf '%s\n' "$out"
        restore
        exit 1
    fi
    restore
    fired=$((fired + 1))
done

# Green once more after the last restore.
out="$(run_battery)"; rc=$?
if [[ "$rc" -ne 0 ]] || ! grep -qE 'All [0-9]+ gates passed' <<<"$out"; then
    echo "CONSUMER-SMOKE: FAIL — the battery did not return to green after the final restore"
    printf '%s\n' "$out"
    exit 1
fi

echo "CONSUMER-SMOKE: clean ($installed kits installed, $fired violations fired)"
exit 0
