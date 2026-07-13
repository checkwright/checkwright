#!/usr/bin/env bash
# spec: context-kit/SPEC.md §Testing — the AGENTS.md agent-file adapter smoke (a standalone validate suite: run-consumer-smoke asserts zero-config defaults, this sets nondefault agent-file knobs)
# usage: agents-md.sh [--keep]
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk" && pwd)"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../../gate-sdk/lib/consumer-smoke.sh
source "$SDK/lib/consumer-smoke.sh"

KEEP=0
for a in "$@"; do
    case "$a" in
        --keep) KEEP=1 ;;
        *) echo "agents-md: unknown option: $a" >&2; exit 2 ;;
    esac
done

fail() { echo "AGENTS-MD-SMOKE: FAIL — $1"; exit 1; }

roots=()
while IFS= read -r r; do roots+=("$r"); done < <(gate_kit_roots)
ordered=("$SDK")
for r in "${roots[@]}"; do [[ "$r" == "$SDK" ]] && continue; ordered+=("$r"); done
roots=("${ordered[@]}")

for r in "${roots[@]}"; do
    [[ -f "$r/smoke/install.sh" ]] || { echo "agents-md: $(basename "$r") has no smoke/install.sh" >&2; exit 2; }
done

SCRATCH=""
cleanup() {
    [[ -n "$SCRATCH" ]] || return 0
    if [[ "$KEEP" == "1" ]]; then echo "AGENTS-MD-SMOKE: --keep, scratch retained at $SCRATCH"; else rm -rf "$SCRATCH"; fi
}
trap cleanup EXIT

csmoke_vendor_and_install "${roots[@]}" || exit 2

# comment-tier-exempt: the installers write the consumer's agent file as CLAUDE.md; the smoke converts it to AGENTS.md and points each kit's agent-file knob at it, as an AGENTS.md adopter would
[[ -f "$SCRATCH/CLAUDE.md" ]] || fail "the installed baseline wrote no CLAUDE.md to convert"
git -C "$SCRATCH" mv CLAUDE.md AGENTS.md

cat > "$SCRATCH/scripts/gate-sdk-config.sh" <<'EOF'
# shellcheck shell=bash
# shellcheck disable=SC2034  # read by check-root-tiering after lib/gate.sh sources this seam
GATE_SDK_AGENT_FILE="AGENTS.md"
EOF

cat >> "$SCRATCH/scripts/context-config.sh" <<'EOF'
# shellcheck disable=SC2034  # read by context-kit bins and check-brevity
CONTEXT_KIT_SURFACES=("AGENTS.md")
CONTEXT_KIT_BREVITY_FILE="AGENTS.md"
EOF

cat > "$SCRATCH/scripts/doctrine-config.sh" <<'EOF'
# shellcheck shell=bash disable=SC2034
DOCTRINE_KIT_AGENT_FILE="AGENTS.md"
EOF

cat > "$SCRATCH/scripts/canon-config.sh" <<'EOF'
# shellcheck shell=bash disable=SC2034
CANON_KIT_MANIFEST_FILES=("AGENTS.md" "README.md" "*/SPEC.md" "*/README.md")
EOF

git -C "$SCRATCH" add -A
git -C "$SCRATCH" -c user.email=smoke@example.invalid -c user.name=smoke \
    commit -q --no-verify -m "convert to AGENTS.md"

# comment-tier-exempt: the lifecycle knob is scalar with no default config file, and canon resolves its manifest only via CANON_KIT_CONFIG_FILE — both ride the battery env, which propagates to every gate subprocess
battery_env=(
    LIFECYCLE_KIT_AGENT_FILE="AGENTS.md"
    CANON_KIT_CONFIG_FILE="scripts/canon-config.sh"
)

run_battery() { ( cd "$SCRATCH" && env "${battery_env[@]}" bash gate-sdk/bin/run-gates.sh ) 2>&1; }

out="$(run_battery)"; rc=$?
if [[ "$rc" -ne 0 ]] || ! grep -qE 'All [0-9]+ gates passed' <<<"$out"; then
    echo "AGENTS-MD-SMOKE: FAIL — the battery is not green on the AGENTS.md consumer"
    printf '%s\n' "$out"
    echo "  help: reproduce with --keep and run the battery in the scratch dir with the knobs set."
    exit 1
fi

# comment-tier-exempt: proves always-loaded reads AGENTS.md — its surface count equals AGENTS.md's line count, not the vanished CLAUDE.md's
agent_lines="$(wc -l < "$SCRATCH/AGENTS.md" | tr -d ' ')"
al_out="$( ( cd "$SCRATCH" && bash context-kit/bin/always-loaded.sh ) )"
grep -qE "surfaces $agent_lines( |·)" <<<"$al_out" \
    || fail "always-loaded did not measure the AGENTS.md surface (${agent_lines}l): $al_out"

# comment-tier-exempt: proves footprint reads AGENTS.md — a non-zero always-loaded total; against the CLAUDE.md default it would find no surface file and measure zero
fp_out="$( ( cd "$SCRATCH" && bash context-kit/bin/footprint.sh ) )"
fp_total="$(grep -E '^\| \*\*total\*\*' <<<"$fp_out")"
grep -qE '\*\*total\*\* \| [1-9][0-9]*l' <<<"$fp_total" \
    || fail "footprint measured no AGENTS.md always-loaded surface: $fp_total"

# comment-tier-exempt: run in a dedicated orientation-clean repo — the vendored scratch is a kit monorepo whose built-in root check reds on the kit dirs regardless of agent file; the knob's real surface is the zero-allowlist fallback set
RT="$(mktemp -d "${TMPDIR:-/tmp}/agents-md-rt.XXXXXX")"
trap 'rm -rf "$SCRATCH" "$RT"' EXIT
git -C "$RT" init -q
: > "$RT/README.md"; : > "$RT/AGENTS.md"
git -C "$RT" add -A
git -C "$RT" -c user.email=smoke@example.invalid -c user.name=smoke commit -q -m seed
rt_ok="$( ( cd "$RT" && GATE_SDK_AGENT_FILE="AGENTS.md" bash "$SCRATCH/gate-sdk/checks/check-root-tiering.sh" ) 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || fail "check-root-tiering rejected an orientation-clean AGENTS.md root: $rt_ok"

: > "$RT/CLAUDE.md"
git -C "$RT" add CLAUDE.md
rt_stray="$( ( cd "$RT" && GATE_SDK_AGENT_FILE="AGENTS.md" bash "$SCRATCH/gate-sdk/checks/check-root-tiering.sh" ) 2>&1 )"; rc=$?
[[ "$rc" -ne 0 ]] || fail "check-root-tiering accepted a stray second agent file (CLAUDE.md beside AGENTS.md)"
grep -qF "CLAUDE.md" <<<"$rt_stray" || fail "check-root-tiering rejected the wrong entry (expected the stray CLAUDE.md): $rt_stray"

echo "AGENTS-MD-SMOKE: clean (battery green, always-loaded + footprint measure AGENTS.md, root-tiering accepts one agent file and rejects a second)"
exit 0
