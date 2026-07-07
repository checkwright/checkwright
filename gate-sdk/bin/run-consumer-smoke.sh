#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — scratch-consumer install+violation harness (a bin/ tool, never a registered gate)
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

roots=()
if [[ ${#kit_args[@]} -gt 0 ]]; then
    for r in "${kit_args[@]}"; do
        [[ -d "$r" ]] || { echo "run-consumer-smoke: not a directory: $r" >&2; exit 2; }
        roots+=("$(cd "$r" && pwd)")
    done
else
    while IFS= read -r r; do roots+=("$r"); done < <(gate_kit_roots)
fi

ordered=("$SDK")
for r in "${roots[@]}"; do
    [[ "$r" == "$SDK" ]] && continue
    ordered+=("$r")
done
roots=("${ordered[@]}")

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

for r in "${roots[@]}"; do
    cp -R "$r" "$SCRATCH/$(basename "$r")"
done

installed=0
for r in "${roots[@]}"; do
    kit="$(basename "$r")"
    if ! ( cd "$SCRATCH" && SMOKE_KIT_ROOT="$SCRATCH/$kit" bash "$SCRATCH/$kit/smoke/install.sh" ); then
        echo "run-consumer-smoke: $kit/smoke/install.sh failed (a broken installer is an environment failure)" >&2
        exit 2
    fi
    installed=$((installed + 1))
done

git -C "$SCRATCH" add -A
git -C "$SCRATCH" -c user.email=smoke@example.invalid -c user.name=smoke \
    commit -q --no-verify -m "installed baseline"

run_battery() { ( cd "$SCRATCH" && bash gate-sdk/bin/run-gates.sh ) 2>&1; }

out="$(run_battery)"; rc=$?
if [[ "$rc" -ne 0 ]] || ! grep -qE 'All [0-9]+ gates passed' <<<"$out"; then
    echo "CONSUMER-SMOKE: FAIL — the battery is not green on the freshly installed consumer (zero config)"
    printf '%s\n' "$out"
    echo "  help: an install step left a gate red; reproduce with --keep and run gate-sdk/bin/run-gates.sh in the scratch dir."
    exit 1
fi

# spec: gate-sdk/SPEC.md §Consumer smoke — hard reset (not checkout) so a violation that staged its shape is unstaged too
restore() { ( cd "$SCRATCH" && git reset -q --hard && git clean -qfd ); }

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

out="$(run_battery)"; rc=$?
if [[ "$rc" -ne 0 ]] || ! grep -qE 'All [0-9]+ gates passed' <<<"$out"; then
    echo "CONSUMER-SMOKE: FAIL — the battery did not return to green after the final restore"
    printf '%s\n' "$out"
    exit 1
fi

echo "CONSUMER-SMOKE: clean ($installed kits installed, $fired violations fired)"
exit 0
