#!/usr/bin/env bash
# Direct unit test of check-gate-output's out-of-reach branch — the one verdict its
# fixture pair cannot hold. Both cases of that pair ship a crate dir (they must: the
# pair's whole job is proving the module-grep arm accepts and rejects), so neither can
# also stand up the tree where the crate is absent. That tree is not hypothetical: it
# is every consumer, because native/ is not a kit root and the payload vendors kit
# roots only, so a vendored .gate member's implementation module is never delivered.
# It also holds the two trees that decide what "the crate is absent" means: one carrying
# the binary at its default path under the crate directory (out of reach), and one
# carrying a manifest but no gates module at all (red).
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
GATE="$SDK/checks/check-gate-output.sh"

fails=0
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

# A consumer-shaped tree: the descriptor vendors, the crate does not exist.
mkdir -p "$TMP/consumer"
printf 'check-vendored-sample\n' > "$TMP/consumer/gates.list"
cat > "$TMP/consumer/check-vendored-sample.gate" <<'EOF'
# graph: couples=TASK-QUEUE.md dir=one valve=none tier=precommit
# no-fixture: a .gate-dispatched no-fixture member whose implementation module this tree does not carry
# spec: gate-sdk/SPEC.md §check-gate-output — the out-of-reach branch
EOF

out="$(cd "$TMP/consumer" && GATE_SDK_KIT_DIRS="$SDK" bash "$GATE" . 2>&1)"; rc=$?
if [[ "$rc" -ne 0 ]]; then
    echo "  FAIL: a member whose crate is absent reddened (exit $rc) — every consumer's battery"
    printf '    %s\n' "$out"
    fails=$((fails + 1))
fi
if ! grep -q 'declared out of reach' <<<"$out"; then
    echo "  FAIL: the omission was not declared in the success line — a silently shrinking count"
    printf '    %s\n' "$out"
    fails=$((fails + 1))
fi
if ! grep -q 'check-vendored-sample' <<<"$out"; then
    echo "  FAIL: the out-of-reach member was not named, so the declaration says nothing"
    printf '    %s\n' "$out"
    fails=$((fails + 1))
fi
if ! grep -q '0 source-grepped as no-fixture members' <<<"$out"; then
    echo "  FAIL: an out-of-reach member was still counted as source-grepped"
    printf '    %s\n' "$out"
    fails=$((fails + 1))
fi

# spec: gate-sdk/SPEC.md §check-gate-output — the regression case for "why the manifest
# and not the directory": a consumer that holds only the gate binary still has a native/
# directory, because GATE_SDK_NATIVE_BIN defaults inside the crate path. Under a
# directory-presence probe this tree reddened and no consumer could clear it.
cp -R "$TMP/consumer" "$TMP/binonly"
mkdir -p "$TMP/binonly/native/target/release"
: > "$TMP/binonly/native/target/release/checkwright-gates"
out="$(cd "$TMP/binonly" && GATE_SDK_KIT_DIRS="$SDK" bash "$GATE" . 2>&1)"; rc=$?
if [[ "$rc" -ne 0 ]]; then
    echo "  FAIL: a tree holding only the binary reddened (exit $rc) — the crate path is where"
    echo "        the artifact lands, so directory presence is not crate presence"
    printf '    %s\n' "$out"
    fails=$((fails + 1))
fi
if ! grep -q 'declared out of reach' <<<"$out"; then
    echo "  FAIL: the binary-only tree was not declared out of reach"
    printf '    %s\n' "$out"
    fails=$((fails + 1))
fi

# The same tree WITH a crate present and the module missing is the half-landed port,
# and it must red — this is what keeps the branch above from being an escape hatch.
cp -R "$TMP/consumer" "$TMP/upstream"
mkdir -p "$TMP/upstream/native/src/gates"
printf '[package]\nname = "checkwright-gates"\n' > "$TMP/upstream/native/Cargo.toml"
out="$(cd "$TMP/upstream" && GATE_SDK_KIT_DIRS="$SDK" bash "$GATE" . 2>&1)"; rc=$?
if [[ "$rc" -ne 1 ]]; then
    echo "  FAIL: a crate present with no module for a dispatching member did not red (exit $rc)"
    printf '    %s\n' "$out"
    fails=$((fails + 1))
fi
if ! grep -q 'no module at native/src/gates/vendored_sample.rs' <<<"$out"; then
    echo "  FAIL: the red did not name the module path the name convention derives"
    printf '    %s\n' "$out"
    fails=$((fails + 1))
fi

# spec: gate-sdk/SPEC.md §check-gate-output — "why not the gates module directory either":
# a crate whose gates module is gone entirely must stay red rather than going quietly out
# of reach. This is the case a src/gates/ probe would have converted into a false green,
# so it is asserted rather than left to the SPEC's argument.
cp -R "$TMP/consumer" "$TMP/gutted"
mkdir -p "$TMP/gutted/native"
printf '[package]\nname = "checkwright-gates"\n' > "$TMP/gutted/native/Cargo.toml"
out="$(cd "$TMP/gutted" && GATE_SDK_KIT_DIRS="$SDK" bash "$GATE" . 2>&1)"; rc=$?
if [[ "$rc" -ne 1 ]]; then
    echo "  FAIL: a crate whose gates module is absent did not red (exit $rc) — crate presence"
    echo "        must not be inferred from the module set it happens to carry"
    printf '    %s\n' "$out"
    fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-gate-output.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-gate-output.test: ok (out-of-reach declared, binary-only out of reach, missing-module red, gutted-crate red)"
exit 0
