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

# --- a live agent worktree is out of the walk -------------------------------
# The default carries the leaf basename `worktrees`, not the path `.claude/worktrees`:
# a basename matcher cannot match a slash, so the path spelling would silently
# restore the second-copy-of-the-repo the member exists to exclude.
mkdir -p "$sandbox/.claude/worktrees/agent-x" "$sandbox/.claude/commands"
: >"$sandbox/.claude/worktrees/agent-x/f.proto"; : >"$sandbox/.claude/commands/g.proto"
got="$(gate_find "$sandbox" -name '*.proto' | sed "s#^$sandbox/##" | sort | paste -sd, -)"
[[ "$got" == '.claude/commands/g.proto,a.proto,sub/b.proto' ]] \
    || { echo "  FAIL: worktree prune returned '$got' (want '.claude/commands/g.proto,a.proto,sub/b.proto')"; fails=$((fails + 1)); }

# --- GATE_SDK_PRUNE_EXTRA_DIRS appends to whichever set was resolved ---------
# Both branches, because the interaction with the replacing knob is the contract:
# an implementation appending only to the default passes the first case alone.
extra_probe() {
    # shellcheck source=../lib/gate.sh
    ( source "$DIR/lib/gate.sh"; printf '%s\n' "${GATE_PRUNE_DIRS[@]}" | paste -sd, - )
}
got="$(GATE_SDK_PRUNE_EXTRA_DIRS='vendor' extra_probe)"
[[ ",$got," == *",target,"* && ",$got," == *",vendor,"* ]] \
    || { echo "  FAIL: EXTRA_DIRS did not append to the default set (got '$got')"; fails=$((fails + 1)); }
got="$(GATE_SDK_PRUNE_DIRS='only' GATE_SDK_PRUNE_EXTRA_DIRS='vendor' extra_probe)"
[[ "$got" == 'only,vendor' ]] \
    || { echo "  FAIL: EXTRA_DIRS did not append to an explicit PRUNE_DIRS (got '$got', want 'only,vendor')"; fails=$((fails + 1)); }

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

# --- declaration path vs invocation argv (the .gate dispatch seam) ------------
: >"$sandbox/b/check-ported.gate"
got="$(gate_resolve check-ported "$sandbox/a" "$sandbox/b")"
[[ "$got" == "$sandbox/b/check-ported.gate" ]] \
    || { echo "  FAIL: gate_resolve did not find a .gate declaration (got '$got')"; fails=$((fails + 1)); }

# .sh beats .gate within a dir — the shadowing property the registry contract rests on
: >"$sandbox/b/check-both.sh"; : >"$sandbox/b/check-both.gate"
got="$(gate_resolve check-both "$sandbox/a" "$sandbox/b")"
[[ "$got" == "$sandbox/b/check-both.sh" ]] \
    || { echo "  FAIL: gate_resolve let .gate win inside a dir (got '$got')"; fails=$((fails + 1)); }

# an earlier dir's .gate still beats a later dir's .sh — dir order is the outer loop
: >"$sandbox/a/check-order.gate"; : >"$sandbox/b/check-order.sh"
got="$(gate_resolve check-order "$sandbox/a" "$sandbox/b")"
[[ "$got" == "$sandbox/a/check-order.gate" ]] \
    || { echo "  FAIL: gate_resolve broke consumer-first dir order (got '$got')"; fails=$((fails + 1)); }

# gate_command: a shell gate yields its one-element argv
got="$(gate_command check-one "$sandbox/a" "$sandbox/b" | paste -sd' ' -)"
[[ "$got" == "$sandbox/b/check-one.sh" ]] \
    || { echo "  FAIL: gate_command shell argv was '$got'"; fails=$((fails + 1)); }

# gate_command: a .gate member yields <binary> <name> when the binary is executable
printf '#!/bin/sh\n' > "$sandbox/fakebin"; chmod +x "$sandbox/fakebin"
got="$(GATE_SDK_NATIVE_BIN="$sandbox/fakebin" gate_command check-ported "$sandbox/a" "$sandbox/b" | paste -sd' ' -)"
[[ "$got" == "$sandbox/fakebin check-ported" ]] \
    || { echo "  FAIL: gate_command native argv was '$got'"; fails=$((fails + 1)); }

# fail-closed: an absent binary is exit 2, never a skip and never a pass
( GATE_SDK_NATIVE_BIN="$sandbox/nope" gate_command check-ported "$sandbox/a" "$sandbox/b" >/dev/null 2>&1 )
[[ "$?" -eq 2 ]] \
    || { echo "  FAIL: gate_command did not exit 2 on an absent native binary"; fails=$((fails + 1)); }

# fail-closed: present but non-executable is the same harness error
: >"$sandbox/notexec"; chmod -x "$sandbox/notexec"
( GATE_SDK_NATIVE_BIN="$sandbox/notexec" gate_command check-ported "$sandbox/a" "$sandbox/b" >/dev/null 2>&1 )
[[ "$?" -eq 2 ]] \
    || { echo "  FAIL: gate_command did not exit 2 on a non-executable native binary"; fails=$((fails + 1)); }

# --- the array-knob config bridge ---------------------------------------------
# A stand-in binary reporting whatever knob the case declares, and a stand-in kit
# whose library defines those knobs: the bridge's own refusals are unreachable
# through the live members, whose one knob is well formed by construction.
cat > "$sandbox/knobbin" <<'FAKE'
#!/usr/bin/env bash
[[ "$1" == --knobs ]] && { [[ -n "${PROBE_KNOB:-}" ]] && printf '%s\n' "$PROBE_KNOB"; exit 0; }
exit 0
FAKE
chmod +x "$sandbox/knobbin"
mkdir -p "$sandbox/probe-kit/lib" "$sandbox/probe-kit/checks"
cat > "$sandbox/probe-kit/lib/probe.sh" <<'PROBE'
# shellcheck shell=bash
PROBE_KIT_SPACED=(alpha "two words")
PROBE_KIT_SCALAR=solo
PROBE_KIT_TABBED=($'has\ttab')
PROBE_KIT_NEWLINED=($'has\nnewline')
PROBE_KIT_RUN_alpha='run alpha'
PROBE_KIT_RUN_beta='run beta'
PROBE_KIT_BADRUN_tabbed=($'has\ttab')
declare -A PROBE_KIT_MAP=([zeta]=last [align]=scope [mid]="two words" [eq]="a=b")
declare -A PROBE_KIT_EMPTYMAP=()
declare -A PROBE_KIT_TABKEY=([$'has\ttab']=v)
declare -A PROBE_KIT_TABVAL=([k]=$'has\ttab')
declare -A PROBE_KIT_NLKEY=([$'has\nnewline']=v)
declare -A PROBE_KIT_NLVAL=([k]=$'has\nnewline')
declare -A PROBE_KIT_EQKEY=([a=b]=v)
for _p in one two; do declare "PROBE_KIT_RUN_loop_$_p=made by the loop"; done
unset _p
PROBE
knob_argv() {
    PROBE_KNOB="$1" GATE_SDK_KIT_DIRS="$sandbox/probe-kit" \
        GATE_SDK_NATIVE_BIN="$sandbox/knobbin" \
        gate_command check-ported "$sandbox/a" "$sandbox/b" 2>&1
}

# whitespace inside an element survives, which is the whole reason the serialization
# is tab-joined rather than the space-separated scalar shape it sits beside
got="$(knob_argv PROBE_KIT_SPACED | paste -sd'|' -)"
want="env|GATE_SDK_KNOB_PROBE_KIT_SPACED=alpha"$'\t'"two words|$sandbox/knobbin|check-ported"
[[ "$got" == "$want" ]] \
    || { echo "  FAIL: bridged argv was '$got' (want '$want')"; fails=$((fails + 1)); }

# a scalar knob is a one-element array — the two cases share one grammar
got="$(knob_argv PROBE_KIT_SCALAR | paste -sd'|' -)"
[[ "$got" == "env|GATE_SDK_KNOB_PROBE_KIT_SCALAR=solo|$sandbox/knobbin|check-ported" ]] \
    || { echo "  FAIL: scalar knob argv was '$got'"; fails=$((fails + 1)); }

# a member declaring no knob emits the two-element argv exactly as before the bridge
got="$(PROBE_KNOB="" GATE_SDK_KIT_DIRS="$sandbox/probe-kit" GATE_SDK_NATIVE_BIN="$sandbox/knobbin" \
    gate_command check-ported "$sandbox/a" "$sandbox/b" | paste -sd'|' -)"
[[ "$got" == "$sandbox/knobbin|check-ported" ]] \
    || { echo "  FAIL: knobless argv gained an env prefix: '$got'"; fails=$((fails + 1)); }

# a knob matching no kit's <KIT>_ prefix resolves to gate-sdk itself, the kit every
# .gate dispatch already runs inside — GATE_PRUNE_DIRS is exactly that case, and the
# expected value is read from this process rather than restated as a second literal
want_prune="$(IFS=$'\t'; printf '%s' "${GATE_PRUNE_DIRS[*]}")"
got="$(knob_argv GATE_PRUNE_DIRS | sed -n '2p')"
[[ "$got" == "GATE_SDK_KNOB_GATE_PRUNE_DIRS=$want_prune" ]] \
    || { echo "  FAIL: prefixless knob did not fall back to gate-sdk: '$got'"; fails=$((fails + 1)); }

# the three refusals, each exit 2 naming the knob
for probe in TABBED:tab NEWLINED:newline; do
    knob="PROBE_KIT_${probe%%:*}"; what="${probe##*:}"
    out="$(knob_argv "$knob")"; rc=$?
    [[ "$rc" -eq 2 ]] \
        || { echo "  FAIL: a $what in a knob element exited $rc, want 2"; fails=$((fails + 1)); }
    grep -qF -- "knob $knob" <<<"$out" \
        || { echo "  FAIL: the $what refusal did not name $knob: $out"; fails=$((fails + 1)); }
done
out="$(knob_argv PROBE_KIT_ABSENT)"; rc=$?
[[ "$rc" -eq 2 ]] \
    || { echo "  FAIL: a knob no kit library defines exited $rc, want 2"; fails=$((fails + 1)); }
grep -qF -- 'PROBE_KIT_ABSENT' <<<"$out" \
    || { echo "  FAIL: the undeclared-knob refusal did not name the knob: $out"; fails=$((fails + 1)); }

# --- the keyed form: an associative knob serializes its key-value pairs ----------
# Which arm a knob takes is derived from its own `declare -p`, so the case declares no
# shape: the same map that would have silently lost its keys before now crosses whole.
got="$(knob_argv PROBE_KIT_MAP | sed -n '2p')"
want="GATE_SDK_KNOB_PROBE_KIT_MAP=align=scope"$'\t'"eq=a=b"$'\t'"mid=two words"$'\t'"zeta=last"
[[ "$got" == "$want" ]] \
    || { echo "  FAIL: keyed knob argv was '$got' (want '$want')"; fails=$((fails + 1)); }

# sorted by key, not by bash's hash order — the resolved argv is baked verbatim into the
# tracked pre-commit hook, so an unsorted emission would churn that file for no change
[[ "$got" == "$(knob_argv PROBE_KIT_MAP | sed -n '2p')" ]] \
    || { echo "  FAIL: keyed form is not deterministic across runs"; fails=$((fails + 1)); }

# the split is on the FIRST '=', so a value carries '=' freely and only the key is
# constrained — the rule `env` itself applies one level out, not a second convention
grep -qF -- 'eq=a=b' <<<"$got" \
    || { echo "  FAIL: a value containing '=' did not survive the keyed wire: $got"; fails=$((fails + 1)); }

# an empty map serializes to the empty string and is a resolved-empty map: absent and
# empty part company here exactly as they do for an indexed array
got="$(knob_argv PROBE_KIT_EMPTYMAP | sed -n '2p')"
[[ "$got" == 'GATE_SDK_KNOB_PROBE_KIT_EMPTYMAP=' ]] \
    || { echo "  FAIL: an empty map serialized to '$got'"; fails=$((fails + 1)); }

# the three element-shape refusals, applied to the key and the value of every pair and
# naming the offending KEY rather than the knob alone
for probe in TABKEY:tab TABVAL:tab NLKEY:newline NLVAL:newline EQKEY:'"="'; do
    knob="PROBE_KIT_${probe%%:*}"; what="${probe##*:}"
    out="$(knob_argv "$knob")"; rc=$?
    [[ "$rc" -eq 2 ]] \
        || { echo "  FAIL: $knob (a $what in a pair) exited $rc, want 2"; fails=$((fails + 1)); }
    grep -qF -- "knob $knob has key " <<<"$out" \
        || { echo "  FAIL: the $what refusal on $knob did not name the offending key: $out"; fails=$((fails + 1)); }
done

# --- the prefix form: a declared name ending in '*' resolves the whole family ---
# The load-bearing case is the LOOP-DECLARED member: the family this exists for is
# built by a consumer config's `while`/`declare`, so a reader that parsed the file
# instead of resolving it would see the static names and miss these two entirely.
prefix_env() { knob_argv 'PROBE_KIT_RUN_*' | grep '^GATE_SDK_KNOB_'; }
got="$(prefix_env)"
for want in \
    'GATE_SDK_KNOB_PROBE_KIT_RUN_alpha=run alpha' \
    'GATE_SDK_KNOB_PROBE_KIT_RUN_beta=run beta' \
    'GATE_SDK_KNOB_PROBE_KIT_RUN_loop_one=made by the loop' \
    'GATE_SDK_KNOB_PROBE_KIT_RUN_loop_two=made by the loop'; do
    grep -qxF -- "$want" <<<"$got" \
        || { echo "  FAIL: prefix form missed '$want', got: $got"; fails=$((fails + 1)); }
done
[[ "$(wc -l <<<"$got")" -eq 4 ]] \
    || { echo "  FAIL: prefix form emitted $(wc -l <<<"$got") element(s), want 4: $got"; fails=$((fails + 1)); }

# a resolution set is not a roster: the prefix takes only what sits under it, so a
# sibling family under a different prefix stays out of this member's environment
grep -q 'PROBE_KIT_BADRUN' <<<"$got" \
    && { echo "  FAIL: prefix form swept in a sibling family: $got"; fails=$((fails + 1)); }

# deterministic: the emitted environment is sorted, so two runs agree byte for byte
[[ "$got" == "$(prefix_env)" ]] \
    || { echo "  FAIL: prefix form is not deterministic across runs"; fails=$((fails + 1)); }

# a prefix matching nothing resolves to an EMPTY FAMILY and passes: the bridge holds no
# roster, so it has no expectation to fail closed on. Refusing here would collapse
# not-adopted (empty roster, no lookups, section drops) into adopted-but-broken's arm --
# the regression that reached a real consumer before this rule was corrected.
out="$(knob_argv 'PROBE_KIT_NOSUCH_*')"; rc=$?
[[ "$rc" -eq 0 ]] \
    || { echo "  FAIL: a prefix matching nothing exited $rc, want 0: $out"; fails=$((fails + 1)); }
grep -q 'GATE_SDK_KNOB_PROBE_KIT_NOSUCH' <<<"$out" \
    && { echo "  FAIL: an empty family emitted an element: $out"; fails=$((fails + 1)); }
# and it stays inert rather than swallowing the argv: the two-element form survives
[[ "$(paste -sd'|' - <<<"$out")" == "$sandbox/knobbin|check-ported" ]] \
    || { echo "  FAIL: an empty family disturbed the argv: $out"; fails=$((fails + 1)); }

# the element-shape refusals apply per match, naming the offending family member
out="$(knob_argv 'PROBE_KIT_BADRUN_*')"; rc=$?
[[ "$rc" -eq 2 ]] \
    || { echo "  FAIL: a tab inside a prefix-matched element exited $rc, want 2"; fails=$((fails + 1)); }
grep -qF -- 'knob PROBE_KIT_BADRUN_tabbed' <<<"$out" \
    || { echo "  FAIL: the prefix element refusal did not name the member: $out"; fails=$((fails + 1)); }

# --- the knob-owner lookup drains its producer --------------------------------
# The candidate roots are read to EOF *before* the match loop, so an early prefix
# hit never leaves the producer writing into a closed pipe. The oracle is SIGPIPE
# *ignored* — the disposition a CI runner inherits from its supervisor — under
# which the abandoned write fails EPIPE and bash reports it on stderr. Under the
# default disposition the producer dies silently and the identical defect shows
# nothing, which is precisely why a green local battery could not see it; the
# `trap '' PIPE` is what makes this deterministic rather than environmental.
many="$sandbox/probe-kit"
for ((_i = 0; _i < 200; _i++)); do many="$many $sandbox/pad-kit"; done
owner_probe() {
    ( trap '' PIPE
      GATE_SDK_KIT_DIRS="$many" _gate_knob_owning_kit PROBE_KIT_SPACED "$@" )
}
got="$(owner_probe 2>&1 1>/dev/null)"
[[ -z "$got" ]] \
    || { echo "  FAIL: _gate_knob_owning_kit abandoned its producer: $got"; fails=$((fails + 1)); }
got="$(owner_probe 2>/dev/null)"
[[ "$got" == "$sandbox/probe-kit" ]] \
    || { echo "  FAIL: knob owner was '$got' (want '$sandbox/probe-kit')"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "lib-gate.test: $fails assertion(s) failed"
    exit 1
fi
echo "lib-gate.test: ok (fail_closed branches; gate_path_pruned; GATE_GREP_EXCLUDES; gate_find prune incl. the worktrees leaf; PRUNE_EXTRA_DIRS append over both branches; registry + resolution; .gate declaration/argv split + dispatch fail-closed; the knob bridge's serialization, scalar/knobless/prefixless arms and its three refusals; the keyed form's derived shape, sorted pairs, first-'=' split, empty map, and its refusals over both halves of a pair; the prefix form over a loop-declared family, its sibling-family exclusion, determinism, empty-family resolution leaving the argv inert, and per-match element refusal; the knob-owner lookup draining its producer on an early match under SIGPIPE-ignored)"
exit 0
