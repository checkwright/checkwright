#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §run-gates — end-to-end lock-in for `run-gates.sh --for`
# selection over a hermetic scratch registry: the two behaviors the live gate
# set cannot exercise (the no-match note on an ungoverned path, a mode=staged
# gate's git-pathspec exact-or-subtree match with its matching paths passed as
# args) plus the trigger='*'/plain-glob cases. Run by run-gate-tests.sh.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
RUN="$ROOT/gate-sdk/bin/run-gates.sh"
[[ -x "$RUN" ]] || { echo "run-for-path.test: runner not found: $RUN"; exit 2; }

scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT

# A trivial gate: prints its name + any args, exits 0.
mk_gate() {
    local name="$1" manifest="$2"
    { printf '#!/usr/bin/env bash\n'
      printf '%s\n' "$manifest"
      printf 'echo "%s ran args=[$*]"\n' "$name"
    } > "$scratch/$name.sh"
    chmod +x "$scratch/$name.sh"
}

# Registry with a trigger='*' gate, a plain-glob gate, and a mode=staged gate.
mk_gate g_star   '# graph: couples=cfg dir=one valve=none tier=precommit trigger=*'
mk_gate g_glob   '# graph: couples=alpha/*.txt dir=one valve=none tier=precommit'
mk_gate g_staged '# graph: couples=beta dir=one valve=none tier=precommit mode=staged'
{ echo g_star; echo g_glob; echo g_staged; } > "$scratch/gates.list"

# Isolate the runner onto the scratch registry: no kit roots, tmp off-tree.
# Verbose: selection is observed through each stub gate's echoed banner output.
run_for() {
    GATE_SDK_GATES_DIR="$scratch" GATE_SDK_KIT_DIRS="$scratch" \
        GATE_SDK_TMP_DIR="$scratch/.tmp" GATE_SDK_VERBOSE=1 bash "$RUN" --for "$@" 2>&1
}

fails=0
assert_has()    { grep -qF -- "$2" <<<"$3" || { echo "FAIL [$1]: expected present: $2"; fails=$((fails + 1)); }; }
assert_absent() { grep -qF -- "$2" <<<"$3" && { echo "FAIL [$1]: expected absent: $2"; fails=$((fails + 1)); }; return 0; }

# A plain-glob path: g_glob (alpha/*.txt) selected, g_star always, g_staged not.
out="$(run_for alpha/foo.txt)"
assert_has   glob "g_glob ran"   "$out"
assert_has   glob "g_star ran"   "$out"
assert_absent glob "g_staged ran" "$out"
assert_absent glob "no registered gate couples to" "$out"

# A mode=staged gate matches by git pathspec (exact-or-subtree) and receives its
# matching paths as args; a plain-glob gate would not match the subtree path.
out="$(run_for beta/inner/x)"
assert_has   staged "g_staged ran args=[beta/inner/x]" "$out"
assert_has   staged "g_star ran"  "$out"
assert_absent staged "g_glob ran" "$out"

# An ungoverned path with a trigger='*' gate present: g_star covers it, so no note.
out="$(run_for zzz/nowhere.md)"
assert_has   covered "g_star ran" "$out"
assert_absent covered "no registered gate couples to" "$out"

# Registry with no trigger='*' gate: an ungoverned path yields the note + nothing.
{ echo g_glob; echo g_staged; } > "$scratch/gates.list"
out="$(run_for zzz/nowhere.md)"; rc=$?
assert_has   nomatch "no registered gate couples to zzz/nowhere.md" "$out"
assert_has   nomatch "nothing to run" "$out"
[[ "$rc" -eq 0 ]] || { echo "FAIL [nomatch]: expected exit 0, got $rc"; fails=$((fails + 1)); }

# --for with no path is a usage error (exit 2).
GATE_SDK_GATES_DIR="$scratch" GATE_SDK_KIT_DIRS="$scratch" \
    GATE_SDK_TMP_DIR="$scratch/.tmp" bash "$RUN" --for >/dev/null 2>&1
[[ $? -eq 2 ]] || { echo "FAIL [usage]: --for with no path should exit 2"; fails=$((fails + 1)); }

[[ "$fails" -eq 0 ]] || { echo "run-for-path.test: $fails assertion(s) failed"; exit 1; }
echo "run-for-path.test: clean (--for selects by shared matcher; trigger='*' universal, mode=staged pathspec passes args, ungoverned path noted)"
exit 0
