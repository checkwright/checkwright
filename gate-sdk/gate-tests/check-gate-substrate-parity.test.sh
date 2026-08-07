#!/usr/bin/env bash
# Behavioral test of checks/check-gate-substrate-parity.sh over the descriptor
# configurations the good/+bad/ pair cannot hold — it is one invocation each, both
# "descriptors and a binary". Two of these were previously assigned to live trees
# (this repo's battery, the consumer smoke) and are held here instead, so they stay
# reachable whatever those trees happen to be; the third is the corrected
# load-bearing predicate's own subject.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
GATE="$DIR/checks/check-gate-substrate-parity.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
mkdir -p "$SANDBOX/scripts" "$SANDBOX/kitroot/checks"
printf '#!/usr/bin/env bash\n# graph: couples=docs/*.md dir=one valve=none tier=precommit\necho "ALPHA: clean (stub)"\n' \
    > "$SANDBOX/scripts/check-alpha.sh"
cat > "$SANDBOX/conservation.md" <<'EOF'
## Meta-gate conservation for the binary substrate

| Subcommand | Disposition |
|---|---|
| `check-reference` | Reference-only — carried by the binary with no descriptor. |

## Next section
EOF
cat > "$SANDBOX/stub-bin" <<'EOF'
#!/usr/bin/env bash
[[ "${1:-}" == --list ]] && { echo check-reference; exit 0; }
exit 2
EOF
# A binary carrying the fixture's descriptors too, for the cases where one dispatches.
cat > "$SANDBOX/stub-bin-full" <<'EOF'
#!/usr/bin/env bash
[[ "${1:-}" == --list ]] && { echo check-ported; echo check-vendored; echo check-reference; exit 0; }
exit 2
EOF
chmod +x "$SANDBOX/stub-bin" "$SANDBOX/stub-bin-full"
DESC='# graph: couples=docs/*.md dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-gate-substrate-parity — fixture descriptor'

run() {  # run <bin>
    ( cd "$SANDBOX" && env GATE_SDK_KIT_DIRS=kitroot GATE_SDK_NATIVE_BIN="$1" \
        GATE_SDK_NATIVE_CRATE=crate GATE_SDK_NATIVE_SRC=impl \
        "$GATE" scripts conservation.md 2>&1 )
}

expect() {  # expect <label> <want-rc> <substring> <got-rc> <output>
    if [[ "$4" -ne "$2" ]]; then
        echo "  FAIL [$1]: want exit $2, got $4 -- $5"; fails=$((fails + 1))
    elif ! grep -qF -- "$3" <<<"$5"; then
        echo "  FAIL [$1]: exit $2 but output lacks '$3': $5"; fails=$((fails + 1))
    fi
}

# --- no descriptors, binary present: the roster half is the only live half ---
# The post-revert tree. Assertion B still compares both directions, so the
# reference-only allowance is what keeps the binary's subcommand from reading as
# a stranded implementation — the half a descriptor-count guard would blank out.
printf 'check-alpha\n' > "$SANDBOX/scripts/gates.list"
out="$(run ./stub-bin)"; rc=$?
expect no-desc-binary 0 '0 descriptor(s) in parity with the 1-subcommand roster, 1 reference-only' "$rc" "$out"

# --- no descriptors, no binary: clean, and it says why the roster went unread ---
out="$(run ./nonexistent-bin)"; rc=$?
expect no-desc-no-binary 0 'no binary at ./nonexistent-bin so no subcommand roster to compare' "$rc" "$out"

# --- descriptors present, none dispatching, no binary, no target roster ---
# Every vendored tree after the first cohort lands. Two arms ride the corrected
# predicate and both must stay quiet here: assertion B's fail-closed arm, and
# assertion F's missing-roster arm (a consumer receives kit roots, never the crate,
# so its roster is absent by construction).
printf '%s\n' "$DESC" > "$SANDBOX/kitroot/checks/check-vendored.gate"
printf '%s\n' "$DESC" > "$SANDBOX/scripts/check-ported.gate"
out="$(run ./nonexistent-bin)"; rc=$?
expect desc-none-dispatching 0 '1 member(s) with one declaration each, 0 of them dispatching to the binary' "$rc" "$out"
if grep -qF 'no target roster' <<<"$out"; then
    echo "  FAIL [desc-none-dispatching-roster]: assertion F red on a declaration nothing dispatches to: $out"
    fails=$((fails + 1))
fi

# --- the near miss: a registered member resolves to a descriptor, no binary ---
# Exit 2, never 0 — the one way a too-loose predicate passes its own good/ case.
printf 'check-alpha\ncheck-ported\n' > "$SANDBOX/scripts/gates.list"
out="$(run ./nonexistent-bin)"; rc=$?
expect registered-no-binary 2 'registered member(s) dispatch to it' "$rc" "$out"

# --- a consumer that received a binary: dispatching, no roster, no crate source ---
# The vendored-tree shape the consumer smoke is in. Assertion F's missing-roster arm
# must stay quiet: declaring platform support is the publishing tree's act, and a
# consumer receives kit roots and an artifact but never the crate.
out="$(run ./stub-bin-full)"; rc=$?
expect consumer-with-binary 0 '2 descriptor(s) in parity with the 3-subcommand roster, 1 reference-only' "$rc" "$out"
if grep -qF 'no target roster' <<<"$out"; then
    echo "  FAIL [consumer-with-binary-roster]: assertion F red in a tree that builds nothing: $out"
    fails=$((fails + 1))
fi

# --- the counterpart: a publishing tree dispatching with no roster still reds ---
# Same configuration but the crate's source is tracked here, which is what makes the
# tree the one that asserts platform support. Source, not directory presence: build
# output under the crate root must not read as a publisher.
PUB="$SANDBOX/pub"
mkdir -p "$PUB/crate"
cp -R "$SANDBOX/scripts" "$SANDBOX/kitroot" "$SANDBOX/conservation.md" "$SANDBOX/stub-bin-full" "$PUB/"
git -C "$PUB" init -q
printf 'fn main() {}\n' > "$PUB/crate/main.src"
git -C "$PUB" add crate/main.src
out="$( cd "$PUB" && env GATE_SDK_KIT_DIRS=kitroot GATE_SDK_NATIVE_BIN=./stub-bin-full \
    GATE_SDK_NATIVE_CRATE=crate GATE_SDK_NATIVE_SRC=impl \
    "$GATE" scripts conservation.md 2>&1 )"; rc=$?
expect publishing-tree-no-roster 1 'no target roster' "$rc" "$out"

if [[ "$fails" -gt 0 ]]; then
    echo "check-gate-substrate-parity.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-gate-substrate-parity.test.sh: clean (descriptor configurations: none with a binary, none without, present-but-none-dispatching with neither binary nor roster, a registered dispatch with no binary, a consumer dispatching to a placed binary with no crate, and the publishing counterpart that still reds — 8 assertions over 6 cases)"
exit 0
