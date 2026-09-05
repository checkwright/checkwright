#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §run-gates — end-to-end lock-in for the runner's *contract* over a
# hermetic scratch registry: the argv refusals, the arm's output contract (the exact green phrase,
# each FAIL tail, the declared-omission line staying off the summary line), the `--only` argv
# channel and its single-member bound, and the determinism the worker pool owes.
# Run by run-gate-tests.sh.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$( { cd "$(git rev-parse --show-toplevel 2>/dev/null)" && pwd -P; } 2>/dev/null || pwd)"
RUN="$ROOT/gate-sdk/bin/run-gates.sh"
[[ -x "$RUN" ]] || { echo "run-arm-contract.test: runner not found: $RUN"; exit 2; }

scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT

fails=0
assert_has()    { grep -qF -- "$2" <<<"$3" || { echo "FAIL [$1]: expected present: $2"; fails=$((fails + 1)); }; }
assert_absent() { grep -qF -- "$2" <<<"$3" && { echo "FAIL [$1]: expected absent: $2"; fails=$((fails + 1)); }; return 0; }
assert_rc()     { [[ "$2" -eq "$3" ]] || { echo "FAIL [$1]: expected exit $3, got $2"; fails=$((fails + 1)); }; }

mk_gate() {
    local name="$1" rc="$2"
    { printf '#!/usr/bin/env bash\n'
      printf '# graph: couples=cfg dir=one valve=none tier=precommit trigger=*\n'
      printf 'echo "%s: a line of its own output"\n' "$name"
      printf 'exit %s\n' "$rc"
    } > "$scratch/$name.sh"
    chmod +x "$scratch/$name.sh"
}

mk_gate g_pass 0
mk_gate g_fail 1
# A declaration that resolves nowhere at run time, for the (unresolved) tail.
{ echo g_pass; echo g_fail; echo g_gone; } > "$scratch/gates.list"
printf '# omitted: g_lost substrate-unavailable\n' >> "$scratch/gates.list"

battery() {   # $@ = extra argv; env overrides come from the caller; streams kept apart
    GATE_SDK_GATES_DIR="$scratch" GATE_SDK_KIT_DIRS="$scratch" \
        GATE_SDK_TMP_DIR="$scratch/.tmp" bash "$RUN" "$@"
}
merged() { battery "$@" 2>&1; }

# ---- the arm's output contract -------------------------------------------------
out="$(GATE_SDK_VERBOSE=1 merged)"; rc=$?
assert_rc     contract "$rc" 1
assert_has    contract '  PASS: g_pass'                                        "$out"
assert_has    contract '  FAIL: g_fail (exit 1)'                               "$out"
assert_has    contract '  FAIL: g_gone (unresolved)'                           "$out"
assert_has    contract '2 of 3 gates FAILED: g_fail g_gone'                    "$out"
assert_has    contract '1 gate(s) omitted (substrate-unavailable)'             "$out"
# quiet green: a passing member's captured output is discarded unless the banner roll is on
out="$(merged)"
assert_absent contract 'g_pass: a line of its own output'                      "$out"
# spec: gate-sdk/SPEC.md §run-gates — the omission line carries none of the summary's text, because
# the consumer smokes match the green phrase against this output
assert_absent contract 'gates passed.'                                         "$out"

# The green phrase, exact, with the omission line beside it and not inside it.
{ echo g_pass; } > "$scratch/gates.list"
printf '# omitted: g_lost substrate-unavailable\n' >> "$scratch/gates.list"
out="$(merged)"; rc=$?
assert_rc  green "$rc" 0
assert_has green 'All 1 gates passed.'                             "$out"
assert_has green '1 gate(s) omitted (substrate-unavailable): no prebuilt gate binary is published for this platform.' "$out"

# ---- the front-end's refusals --------------------------------------------------
{ echo g_pass; echo g_fail; echo g_gone; } > "$scratch/gates.list"
out="$(merged --only g_pass no-such-gate)"; rc=$?
assert_rc  only-unregistered "$rc" 2
assert_has only-unregistered "run-gates: --only: 'no-such-gate' is not registered in $scratch/gates.list" "$out"

out="$(merged --only)"; rc=$?
assert_rc  only-empty "$rc" 2
assert_has only-empty 'run-gates: --only needs at least one gate name' "$out"

out="$(merged --for)"; rc=$?
assert_rc  for-empty "$rc" 2
assert_has for-empty 'run-gates: --for needs at least one path' "$out"

# spec: gate-sdk/SPEC.md §The bin/-tool contract — help on stdout at exit 0, an unrecognized leading-dash first argument as usage on stderr at exit 2
battery --help > "$scratch/h.out" 2> "$scratch/h.err"; rc=$?
assert_rc help "$rc" 0
assert_has help 'usage: run-gates.sh' "$(<"$scratch/h.out")"
[[ -s "$scratch/h.err" ]] && { echo "FAIL [help]: the help arm wrote to stderr"; fails=$((fails + 1)); }

battery --nope > "$scratch/u.out" 2> "$scratch/u.err"; rc=$?
assert_rc unrecognized "$rc" 2
assert_has    unrecognized 'run-gates: unrecognized option: --nope' "$(<"$scratch/u.err")"
assert_has    unrecognized 'usage: run-gates.sh'                    "$(<"$scratch/u.err")"
[[ -s "$scratch/u.out" ]] && { echo "FAIL [unrecognized]: the refusal wrote to stdout"; fails=$((fails + 1)); }

# spec: gate-sdk/SPEC.md §run-gates — an ungoverned path is a fact, not a failure: a note on stdout at exit 0
mk_gate g_narrow 0
{ echo g_narrow; } > "$scratch/gates.list"
sed -i 's|trigger=\*|trigger=alpha/*.txt|' "$scratch/g_narrow.sh"
out="$(merged --for zeta/nothing.md)"; rc=$?
assert_rc  for-note "$rc" 0
assert_has for-note 'run-gates: no registered gate couples to zeta/nothing.md' "$out"
assert_has for-note 'no coupled gate for the given path(s); nothing to run.'   "$out"

# ---- the determinism the pool owes ---------------------------------------------
{ echo g_pass; echo g_fail; echo g_gone; } > "$scratch/gates.list"
printf '# omitted: g_lost substrate-unavailable\n' >> "$scratch/gates.list"
GATE_SDK_VERBOSE=1 merged > "$scratch/par1.txt"
GATE_SDK_VERBOSE=1 merged > "$scratch/par2.txt"
GATE_SDK_VERBOSE=1 GATE_SDK_JOBS=1 merged > "$scratch/ser.txt"
diff -q "$scratch/par1.txt" "$scratch/par2.txt" >/dev/null \
    || { echo "FAIL [determinism]: two default runs produced different transcripts"; diff "$scratch/par1.txt" "$scratch/par2.txt"; fails=$((fails + 1)); }
diff -q "$scratch/par1.txt" "$scratch/ser.txt" >/dev/null \
    || { echo "FAIL [determinism]: GATE_SDK_JOBS=1 diverged from the default worker count"; diff "$scratch/par1.txt" "$scratch/ser.txt"; fails=$((fails + 1)); }

# ---- the --only argv channel and its single-member bound -----------------------
# spec: gate-sdk/SPEC.md §run-gates — argv after a `--` separator is forwarded to the selected gate,
# and only when the selection resolves to exactly one member; two or more is a refusal and never a
# broadcast, so the bound is asserted as executed behaviour rather than as documented intent.
{ printf '#!/usr/bin/env bash\n'
  printf '# graph: couples=cfg dir=one valve=none tier=precommit trigger=*\n'
  printf 'echo "g_args saw: $*"\n'
} > "$scratch/g_args.sh"
chmod +x "$scratch/g_args.sh"
{ echo g_args; echo g_pass; } > "$scratch/gates.list"

out="$(GATE_SDK_VERBOSE=1 merged --only g_args -- alpha beta)"; rc=$?
assert_rc  only-argv "$rc" 0
assert_has only-argv 'g_args saw: alpha beta' "$out"
assert_has only-argv 'All 1 gates passed.'    "$out"

# the separator with nothing after it still selects one member and forwards an empty vector
out="$(GATE_SDK_VERBOSE=1 merged --only g_args --)"; rc=$?
assert_rc  only-argv-empty "$rc" 0
assert_has only-argv-empty 'g_args saw: ' "$out"

out="$(merged --only g_args g_pass -- alpha)"; rc=$?
assert_rc  only-argv-broadcast "$rc" 2
assert_has only-argv-broadcast "run-gates: --only: a '--' separator forwards its arguments to one selected gate, and this selection resolves to 2: g_args g_pass" "$out"
assert_absent only-argv-broadcast 'g_args saw:' "$out"

# without the separator a selection of two is an ordinary narrower run, args or no args
out="$(GATE_SDK_VERBOSE=1 merged --only g_args g_pass)"; rc=$?
assert_rc  only-no-argv "$rc" 0
assert_has only-no-argv 'g_args saw: ' "$out"
assert_has only-no-argv 'All 2 gates passed.' "$out"

[[ "$fails" -eq 0 ]] || { echo "run-arm-contract.test: $fails assertion(s) failed"; exit 1; }
echo "run-arm-contract.test: clean (the three FAIL tails, the exact green phrase, the omission line beside the summary and not in it, the four argv refusals, the --only argv channel forwarded on a single-member selection and refused on a two-member one, and two default runs and a serial run byte-identical)"
exit 0
