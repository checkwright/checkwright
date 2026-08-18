#!/usr/bin/env bash
# Behavioral test of check-gate-binary-fresh over the configurations the
# good/+bad/ pair cannot hold: the pair is one invocation each and both are
# "descriptors present, binary readable". These are the load-bearing predicate's
# own boundary — a descriptor on disk is a declaration, a registered member
# resolving to one is a dispatch — plus the two no-dispatch configurations this
# repo's battery and the consumer smoke used to be the only oracles for.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
# shellcheck source=../lib/gate.sh
source "$DIR/lib/gate.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
STAMP=4f1c2a90bb7e5d3168ac04e2f5b7391dd0c6a812

# One sandbox shape, re-pointed per case: a gates dir, a kit resolve dir, a stub
# binary whose baked stamp matches stamp.txt, and a registry the case rewrites.
mkdir -p "$SANDBOX/scripts" "$SANDBOX/kitroot/checks"
printf '%s\n' "$STAMP" > "$SANDBOX/stamp.txt"
cat > "$SANDBOX/stub-bin" <<EOF
#!/usr/bin/env bash
[[ "\${1:-}" == --source-stamp ]] && { echo $STAMP; exit 0; }
exit 2
EOF
chmod +x "$SANDBOX/stub-bin"
DESC='# graph: couples=docs/*.md dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-gate-binary-fresh — fixture descriptor'

# spec: gate-sdk/SPEC.md §check-gate-binary-fresh — this member's subject is the
# dispatch knob itself, so a case pointing GATE_SDK_NATIVE_BIN at a missing binary
# cannot reach the rule through an untouched dispatch: gate_command resolves the
# binary through that same knob and refuses first. The argv is resolved through
# gate_command (never a declaration path) and the one bridged element the case
# varies is substituted after, which is also what makes the case substrate-agnostic.
gate_argv() {  # gate_argv <knob> <value> -> ARGV
    local knob="$1" val="$2" e
    local -a resolved=()
    mapfile -t resolved < <(gate_command check-gate-binary-fresh "$DIR/checks")
    [[ ${#resolved[@]} -gt 0 ]] || return 2
    ARGV=()
    for e in ${resolved[@]+"${resolved[@]}"}; do
        [[ "$e" == "GATE_SDK_KNOB_$knob="* ]] && e="GATE_SDK_KNOB_$knob=$val"
        ARGV+=("$e")
    done
    export "$knob=$val"
}

run() {  # run <bin> ; registry + descriptors are already in place
    ( cd "$SANDBOX" \
        && gate_env GATE_SDK_KIT_DIRS=kitroot GATE_SDK_NATIVE_CRATE=crate \
        && gate_argv GATE_SDK_NATIVE_BIN "$1" \
        && "${ARGV[@]}" scripts stamp.txt 2>&1 )
}

expect() {  # expect <label> <want-rc> <substring> <got-rc> <output>
    if [[ "$4" -ne "$2" ]]; then
        echo "  FAIL [$1]: want exit $2, got $4 -- $5"; fails=$((fails + 1))
    elif ! grep -qF -- "$3" <<<"$5"; then
        echo "  FAIL [$1]: exit $2 but output lacks '$3': $5"; fails=$((fails + 1))
    fi
}

# --- zero descriptors: clean, and the report says zero of each ---
printf 'check-shell\n' > "$SANDBOX/scripts/gates.list"
printf '#!/usr/bin/env bash\n' > "$SANDBOX/scripts/check-shell.sh"
out="$(run ./stub-bin)"; rc=$?
expect zero-descriptors 0 '0 .gate descriptor(s)' "$rc" "$out"
expect zero-descriptors-dispatch 0 '0 dispatched to by a live member' "$rc" "$out"

# --- descriptors present, none dispatching: clean, and the two counts differ ---
# The configuration every vendored tree is in after the first cohort lands: the
# descriptors ship with the kit root, no consumer registry names them, and no
# binary exists. Reading a descriptor's presence as a dispatch reds all of them.
printf '%s\n' "$DESC" > "$SANDBOX/scripts/check-ported.gate"
printf '%s\n' "$DESC" > "$SANDBOX/kitroot/checks/check-vendored.gate"
out="$(run ./nonexistent-bin)"; rc=$?
expect present-none-dispatching 0 '2 .gate descriptor(s)' "$rc" "$out"
expect present-none-dispatching-count 0 '0 dispatched to by a live member' "$rc" "$out"

# --- the near miss: a registered member resolves to a descriptor, no binary ---
# Exit 2, never 0: this is the state the corrected predicate must still fail
# closed on, and the one way a too-loose predicate passes its own good/ case.
printf 'check-shell\ncheck-ported\n' > "$SANDBOX/scripts/gates.list"
out="$(run ./nonexistent-bin)"; rc=$?
expect registered-no-binary 2 'registered member(s) dispatch to it' "$rc" "$out"

# --- absent registry: cannot verify, so exit 2 rather than a clean report ---
rm -f "$SANDBOX/scripts/gates.list"
out="$(run ./stub-bin)"; rc=$?
expect absent-registry 2 'no gate registry at' "$rc" "$out"

if [[ "$fails" -gt 0 ]]; then
    echo "check-gate-binary-fresh.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-gate-binary-fresh.test.sh: clean (load-bearing predicate: zero descriptors, descriptors with none dispatching, a registered dispatch with no binary, and an absent registry — 6 assertions over 4 cases)"
exit 0
