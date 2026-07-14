#!/usr/bin/env bash
# Direct unit test of gate-sdk/lib/gate.sh — the runtime lock-in for the
# fail-closed contract. A green static check cannot prove fail-closed; this
# exercises both branches of the helper directly.
#
# Why a direct test and not per-gate input fixtures: a well-formed awk cannot be
# crashed on present, readable input, and a *missing* input trips each gate's
# pre-`-f` guard before reaching the capture fail_closed protects. So the helper
# is where the fail-closed contract is actually verifiable. Per-gate *wiring*
# (does the gate call fail_closed after its capture) is a structural property,
# verified by check-gate-fail-closed + check-shellcheck.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
# shellcheck source=../lib/gate.sh
source "$DIR/lib/gate.sh"

fails=0

# Branch 1: zero status -> no-op (returns 0, does NOT exit the caller).
fail_closed 0 check-x awk
reached=yes
[[ "$reached" == yes ]] || { echo "  FAIL: fail_closed 0 aborted the caller"; fails=$((fails + 1)); }

# Branch 2: non-zero status -> exit 2 + diagnostic on stderr (run in a subshell
# so the exit doesn't abort this test).
out="$( ( fail_closed 2 check-x awk ) 2>&1 )"; rc=$?
[[ "$rc" -eq 2 ]] || { echo "  FAIL: fail_closed 2 should exit 2, exited $rc"; fails=$((fails + 1)); }
grep -qF 'the check could not run' <<<"$out" \
    || { echo "  FAIL: fail_closed 2 missing 'the check could not run': $out"; fails=$((fails + 1)); }

# --- discovery prune set: gate_path_pruned (pure predicate) ------------------
for p in 'a/target/b' './node_modules/x' 'kit/gate-tests/g/SPEC.md' '.tmp/s'; do
    gate_path_pruned "$p" || { echo "  FAIL: gate_path_pruned missed '$p'"; fails=$((fails + 1)); }
done
for p in 'src/main.rs' 'a/targets/b' 'some-service/proto/x.proto'; do
    gate_path_pruned "$p" && { echo "  FAIL: gate_path_pruned over-pruned '$p'"; fails=$((fails + 1)); }
done

# --- GATE_GREP_EXCLUDES: one --exclude-dir per pruned dir --------------------
for d in target gate-tests node_modules; do
    printf '%s\n' "${GATE_GREP_EXCLUDES[@]}" | grep -qxF -- "--exclude-dir=$d" \
        || { echo "  FAIL: GATE_GREP_EXCLUDES missing $d"; fails=$((fails + 1)); }
done

# --- gate_find: prunes the set, returns everything else ----------------------
sandbox="$(mktemp -d)"; trap 'rm -rf "$sandbox"' EXIT
mkdir -p "$sandbox/sub" "$sandbox/target" "$sandbox/gate-tests/x" "$sandbox/node_modules"
: >"$sandbox/a.proto"; : >"$sandbox/sub/b.proto"
: >"$sandbox/target/c.proto"; : >"$sandbox/gate-tests/x/d.proto"; : >"$sandbox/node_modules/e.proto"
got="$(gate_find "$sandbox" -name '*.proto' | sed "s#^$sandbox/##" | sort | paste -sd, -)"
[[ "$got" == 'a.proto,sub/b.proto' ]] \
    || { echo "  FAIL: gate_find returned '$got' (want 'a.proto,sub/b.proto')"; fails=$((fails + 1)); }

# --- registry + resolution (gate-sdk additions) -------------------------------
list="$sandbox/gates.list"
printf '# comment\n\ncheck-one\ncheck-two\n' > "$list"
got="$(gates_list_members "$list" | paste -sd, -)"
[[ "$got" == 'check-one,check-two' ]] \
    || { echo "  FAIL: gates_list_members returned '$got' (want 'check-one,check-two')"; fails=$((fails + 1)); }

mkdir -p "$sandbox/a" "$sandbox/b"
: >"$sandbox/b/check-one.sh"
got="$(gate_resolve check-one "$sandbox/a" "$sandbox/b")"
[[ "$got" == "$sandbox/b/check-one.sh" ]] \
    || { echo "  FAIL: gate_resolve returned '$got' (want '$sandbox/b/check-one.sh')"; fails=$((fails + 1)); }
gate_resolve check-missing "$sandbox/a" "$sandbox/b" >/dev/null \
    && { echo "  FAIL: gate_resolve found a nonexistent gate"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "lib-gate.test: $fails assertion(s) failed"
    exit 1
fi
echo "lib-gate.test: ok (fail_closed branches; gate_path_pruned; GATE_GREP_EXCLUDES; gate_find prune; registry + resolution)"
exit 0
