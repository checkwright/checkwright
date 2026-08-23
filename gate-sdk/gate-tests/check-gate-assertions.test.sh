#!/usr/bin/env bash
# Behavioral test of check-gate-assertions over the arms the good/+bad/ pair cannot
# reach. Both cases pass a second positional, which short-circuits resolution to the
# scripts-dir-plus-`.sh` branch, so the registry walk, the descriptor-to-module
# redirection and the no-crate skip are reached by no case at all — and the `args`
# file carries positionals only, so no case can set the knobs those arms resolve
# through. The instrument is a throwaway mini-consumer, on the precedent
# check-graph's own tree test set: it is preferred to a per-case config file because
# it reaches the no-crate arm, which needs a tree with no crate manifest, a state a
# case dir inside this repository cannot have.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
CHECKS="$DIR/checks"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

expect() {  # expect <label> <want-rc> <substring> <got-rc> <output>
    if [[ "$4" -ne "$2" ]]; then
        echo "  FAIL [$1]: want exit $2, got $4 -- $5"; fails=$((fails + 1))
    elif ! grep -qF -- "$3" <<<"$5"; then
        echo "  FAIL [$1]: exit $2 but output lacks '$3': $5"; fails=$((fails + 1))
    fi
}

# A mini-consumer: two kit roots, each with its own SPEC.md, so the kit-roots walk
# has more than one spec to union; a gates dir with no SPEC.md of its own, so the
# optional consumer spec is proved optional; one `.gate`-declared member whose
# markers live in its implementation module, and one shell-declared member whose
# markers live in the script.
C="$SANDBOX/consumer"
mkdir -p "$C/scripts" "$C/kit-a/checks" "$C/kit-b/checks" "$C/native/src/gates"
cat > "$C/kit-a/SPEC.md" <<'EOF'
# kit-a

### check-alpha

Invariant: alpha holds on two axes: (A) the first; (B) the second.
EOF
cat > "$C/kit-b/SPEC.md" <<'EOF'
# kit-b

### check-beta

Invariant: beta holds on two checks: (A) the first; (B) the second.
EOF
printf '# graph: couples=X dir=one valve=none tier=precommit\n' > "$C/kit-a/checks/check-alpha.gate"
cat > "$C/kit-b/checks/check-beta.sh" <<'EOF'
#!/usr/bin/env bash
# assertion A: the first beta check
# assertion B: the second beta check
EOF
cat > "$C/native/src/gates/alpha.rs" <<'EOF'
// assertion A: the first alpha axis
    // assertion B: the second alpha axis, its marker indented as a module body's is
EOF
printf '[package]\nname = "checkwright-gates"\n' > "$C/native/Cargo.toml"

kitdirs="$C/kit-a $C/kit-b"

# --- the registry resolution arm, the descriptor-to-module redirection and the
# multi-SPEC kit-roots walk, all of which need the no-positional invocation ---
out="$( cd "$C" \
    && gate_env GATE_SDK_KIT_DIRS="$kitdirs" GATE_SDK_GATES_DIR=scripts GATE_SDK_NATIVE_CRATE=native \
    && gate_run check-gate-assertions "$CHECKS" 2>&1 )"; rc=$?
expect registry-resolution 0 'GATE-ASSERTIONS: clean (2 of 2 enumerated contract(s) coupled)' "$rc" "$out"

# --- the redirection is load-bearing, not vacuous: drift the module's marker set
# and the red must name the module path the crate's own naming convention derives ---
printf '// assertion A: the only surviving alpha marker\n' > "$C/native/src/gates/alpha.rs"
out="$( cd "$C" \
    && gate_env GATE_SDK_KIT_DIRS="$kitdirs" GATE_SDK_GATES_DIR=scripts GATE_SDK_NATIVE_CRATE=native \
    && gate_run check-gate-assertions "$CHECKS" 2>&1 )"; rc=$?
expect module-redirection 1 'missing marker(s): B' "$rc" "$out"
if ! grep -qF 'native/src/gates/alpha.rs' <<<"$out" && ! grep -qF '§check-alpha' <<<"$out"; then
    echo "  FAIL [module-redirection]: the red named neither the module nor the member -- $out"
    fails=$((fails + 1))
fi

# --- the no-crate skip and its clean-line segment: a vendored consumer receives the
# descriptor and never the crate, so the member is declared out of reach and counted
# rather than reddened. Restored to a coupled module first, so the skip is what
# clears it and not the drift above.
cat > "$C/native/src/gates/alpha.rs" <<'EOF'
// assertion A: the first alpha axis
// assertion B: the second alpha axis
EOF
rm "$C/native/Cargo.toml"
out="$( cd "$C" \
    && gate_env GATE_SDK_KIT_DIRS="$kitdirs" GATE_SDK_GATES_DIR=scripts GATE_SDK_NATIVE_CRATE=native \
    && gate_run check-gate-assertions "$CHECKS" 2>&1 )"; rc=$?
expect no-crate-skip 0 'declared out of reach with no crate at native/Cargo.toml' "$rc" "$out"
expect no-crate-count 0 'clean (1 of 2 enumerated contract(s) coupled, 1 declared out of reach' "$rc" "$out"
expect no-crate-named 0 'check-alpha' "$rc" "$out"

# --- the two fail-closed entry arms, neither of which any case dir can reach:
# a named spec that does not exist, and a tree carrying no SPEC.md at all ---
out="$( cd "$C" && gate_run check-gate-assertions "$CHECKS" absent-spec.md 2>&1 )"; rc=$?
expect spec-not-found 2 'not found: absent-spec.md' "$rc" "$out"

mkdir -p "$SANDBOX/bare/scripts" "$SANDBOX/bare/kit-c"
out="$( cd "$SANDBOX/bare" \
    && gate_env GATE_SDK_KIT_DIRS="$SANDBOX/bare/kit-c" GATE_SDK_GATES_DIR=scripts \
    && gate_run check-gate-assertions "$CHECKS" 2>&1 )"; rc=$?
expect no-spec-at-all 2 'no SPEC.md found' "$rc" "$out"

if [[ "$fails" -gt 0 ]]; then
    echo "check-gate-assertions.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-gate-assertions.test.sh: clean (the arms a case dir cannot reach: the registry resolution walk over two kit SPECs, the descriptor-to-module redirection and its load-bearing red, the no-crate skip with its clean-line segment, and the two fail-closed entry arms — 8 assertions over 5 invocations)"
exit 0
