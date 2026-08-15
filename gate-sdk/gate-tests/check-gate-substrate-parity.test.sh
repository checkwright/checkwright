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
# This one speaks the owner column; stub-bin above deliberately does not, so the
# one-column fallback is exercised by a real run rather than asserted in prose.
cat > "$SANDBOX/stub-bin-full" <<'EOF'
#!/usr/bin/env bash
[[ "${1:-}" == --list ]] && { printf 'check-ported\tkitroot\ncheck-vendored\tkitroot\ncheck-reference\tkitroot\n'; exit 0; }
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
expect no-desc-binary 0 '0 descriptor(s) in parity with the 1-subcommand roster (1 in scope, 0 out of scope — an unvendored kit, or a consumer declaration from another tree — owner column unavailable), 1 reference-only' "$rc" "$out"

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
expect consumer-with-binary 0 '2 descriptor(s) in parity with the 3-subcommand roster (3 in scope, 0 out of scope — an unvendored kit, or a consumer declaration from another tree — owner column scoped), 1 reference-only' "$rc" "$out"
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

# --- the subset vendoring: descriptors for one kit, a binary carrying two kits' ---
# Every consumer that vendors fewer kits than the shared binary carries subcommands
# for, which is every consumer once a second kit ports. A subcommand whose owning kit
# is absent is not a stranded implementation; it is out of scope, counted, and said so.
SUB="$SANDBOX/subset"
mkdir -p "$SUB/scripts" "$SUB/kitroot/checks"
cp "$SANDBOX/conservation.md" "$SUB/"
printf '%s\n' "$DESC" > "$SUB/kitroot/checks/check-vendored.gate"
printf 'check-vendored\n' > "$SUB/scripts/gates.list"
cat > "$SUB/stub-bin-subset" <<'EOF'
#!/usr/bin/env bash
[[ "${1:-}" == --list ]] && { printf 'check-vendored\tkitroot\ncheck-foreign\totherkit\n'; exit 0; }
exit 2
EOF
cat > "$SUB/stub-bin-subset-orphan" <<'EOF'
#!/usr/bin/env bash
[[ "${1:-}" == --list ]] && { printf 'check-vendored\tkitroot\ncheck-orphan\tkitroot\n'; exit 0; }
exit 2
EOF
# The same subset roster from a binary predating the owner column — the fallback's own
# subject, and the proof that the case below is clean because of the scoping rather
# than for some other reason the sandbox happens to supply.
cat > "$SUB/stub-bin-subset-flat" <<'EOF'
#!/usr/bin/env bash
[[ "${1:-}" == --list ]] && { printf 'check-vendored\ncheck-foreign\n'; exit 0; }
exit 2
EOF
chmod +x "$SUB/stub-bin-subset" "$SUB/stub-bin-subset-orphan" "$SUB/stub-bin-subset-flat"

run_subset() {  # run_subset <bin>
    ( cd "$SUB" && env GATE_SDK_KIT_DIRS=kitroot GATE_SDK_NATIVE_BIN="$1" \
        GATE_SDK_NATIVE_CRATE=crate GATE_SDK_NATIVE_SRC=impl \
        "$GATE" scripts conservation.md 2>&1 )
}

out="$(run_subset ./stub-bin-subset)"; rc=$?
expect subset-vendoring 0 '1 descriptor(s) in parity with the 2-subcommand roster (1 in scope, 1 out of scope — an unvendored kit, or a consumer declaration from another tree — owner column scoped), 0 reference-only' "$rc" "$out"

# --- its near miss: the in-scope kit is the one missing a descriptor ---
# The scoping must narrow what the assertion speaks for and nothing else. A rule that
# scoped by "no descriptor" rather than by owner passes the case above and this one too.
out="$(run_subset ./stub-bin-subset-orphan)"; rc=$?
expect subset-near-miss 1 "the binary carries 'check-orphan' with no check-orphan.gate descriptor" "$rc" "$out"

# --- the fallback is a return to today's behavior, never a false green ---
# The identical subset roster from a one-column binary reds on the foreign subcommand,
# which is exactly what this gate did before the column existed.
out="$(run_subset ./stub-bin-subset-flat)"; rc=$?
expect subset-fallback-reds 1 "the binary carries 'check-foreign' with no check-foreign.gate descriptor" "$rc" "$out"

# --- the consumer sentinel in an adopter: out of scope, counted, clean ---
# A subcommand the publisher's own gates directory declares. §upgrade-smoke's defining property
# of that directory is that a gate living solely there cannot appear in a vendored tree, while
# the payload ships the prebuilt binary — so an adopter holds the subcommand and can never hold a
# descriptor for it. Scoping it in would reinstate the unsatisfiable equality the scope rule
# removed. This is the direction an adopter depends on, and the one an over-tight predicate
# passes by accident, so it is a run rather than an inspection.
cat > "$SUB/stub-bin-consumer" <<'EOF'
#!/usr/bin/env bash
[[ "${1:-}" == --list ]] && { printf 'check-vendored\tkitroot\ncheck-consumer\t-\n'; exit 0; }
exit 2
EOF
chmod +x "$SUB/stub-bin-consumer"
out="$(run_subset ./stub-bin-consumer)"; rc=$?
expect consumer-owner-adopter 0 '1 descriptor(s) in parity with the 2-subcommand roster (1 in scope, 1 out of scope — an unvendored kit, or a consumer declaration from another tree — owner column scoped), 0 reference-only' "$rc" "$out"

# --- the same roster in the publishing tree: in scope, and red with no descriptor ---
# The tree carrying the crate's tracked source is the tree whose registry decides which
# subcommands exist, so it is the only one where a stranded implementation can be created — and
# ruling these members permanently out of scope would end, for the whole consumer-declared
# corpus, the one assertion assertion B exists for. The sandbox carries a target roster so
# assertion F stays quiet and the finding under test is the only one.
PUBC="$SANDBOX/pubc"
mkdir -p "$PUBC/crate"
cp -R "$SUB/scripts" "$SUB/kitroot" "$SUB/conservation.md" "$SUB/stub-bin-consumer" "$PUBC/"
git -C "$PUBC" init -q
printf 'fn main() {}\n' > "$PUBC/crate/main.src"
printf 'x86_64-unknown-linux-gnu\n' > "$PUBC/crate/targets.list"
git -C "$PUBC" add crate/main.src

run_pubc() {
    ( cd "$PUBC" && env GATE_SDK_KIT_DIRS=kitroot GATE_SDK_NATIVE_BIN=./stub-bin-consumer \
        GATE_SDK_NATIVE_CRATE=crate GATE_SDK_NATIVE_SRC=impl \
        "$GATE" scripts conservation.md 2>&1 )
}

out="$(run_pubc)"; rc=$?
expect consumer-owner-publisher 1 "the binary carries 'check-consumer' with no check-consumer.gate descriptor" "$rc" "$out"

# --- and what discharges it is a descriptor in the gates directory, not under a kit root ---
# The declaring root a consumer-declared member resolves through, proved by the finding clearing.
printf '%s\n' "$DESC" > "$PUBC/scripts/check-consumer.gate"
out="$(run_pubc)"; rc=$?
expect consumer-owner-declared 0 '2 descriptor(s) in parity with the 2-subcommand roster (2 in scope, 0 out of scope' "$rc" "$out"

# --- the other direction stays unrestricted under the scoped path ---
# The obvious implementation restricts one loop and accidentally restricts both. A
# vendored descriptor is in scope by definition, so a descriptor naming no subcommand
# reds whatever the owner column says — proved here, not reasoned about: the pair's own
# bad/ case reaches this finding only through a one-column binary.
printf '%s\n' "$DESC" > "$SUB/kitroot/checks/check-extra.gate"
out="$(run_subset ./stub-bin-subset)"; rc=$?
expect subset-descriptor-half 1 'descriptor names no subcommand: check-extra.gate' "$rc" "$out"

if [[ "$fails" -gt 0 ]]; then
    echo "check-gate-substrate-parity.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-gate-substrate-parity.test.sh: clean (descriptor configurations: none with a binary, none without, present-but-none-dispatching with neither binary nor roster, a registered dispatch with no binary, a consumer dispatching to a placed binary with no crate, the publishing counterpart that still reds, a subset vendoring whose binary carries a second kit's subcommand, its near miss where the in-scope kit is the one missing a descriptor, that same subset roster from a one-column binary, which still reds, a consumer-declared subcommand out of scope in an adopter, its counterpart in scope in the publishing tree where it reds with no descriptor and clears with one under the gates directory, and a vendored descriptor naming no subcommand under the scoped path — 15 assertions over 13 cases, two of them driving a single-column binary so the owner-column fallback is run rather than asserted)"
exit 0
