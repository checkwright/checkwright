#!/usr/bin/env bash
# Direct unit test of the --run-gate-tests arm's expect.txt semantics — every
# non-blank line of an expect file is a separate assertion, and a case fails
# when any one of them is absent. A fixture pair cannot cover this: the runner
# is a test layer parallel to the gates, never a gates.list member, so it owes
# no good/+bad/ pair, and a pair here would sit outside
# check-gate-fixture-coverage's registry authority set and be audited by
# nothing.
#
# The arm is reached through gate_arm_run, the arm counterpart of gate_run: the
# battery front-end cds to the git toplevel and refuses outside a repository,
# and the write-side row below drives the runner from a non-git sandbox cwd.
#
# The self-invocation is bounded by an inner tree that cannot reach the runner
# again: rows below hand it fixture dirs and no *.test.sh at all, and the one
# tree that ships a *.test.sh ships a leaf that reads its environment and exits.
# A scratch *.test.sh that re-invokes the runner would not be bounded.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT

mkdir -p "$scratch/checks"
# The stub gate replays a case dir's own say/rc files, so each case dictates the
# output and exit status run_case sees.
{ printf '#!/usr/bin/env bash\n'
  printf 'cat ./say\n'
  printf 'exit "$(cat ./rc)"\n'
} > "$scratch/checks/stub-gate.sh"
chmod +x "$scratch/checks/stub-gate.sh"

# mk_tree <name> <good-say> <good-expect> <bad-say> <bad-expect>; an empty
# good-expect ships no good/expect.txt (the optional side of the asymmetry).
# Each say gets the output-contract line its exit code owes (§Output contract,
# asserted at runtime by run_case) appended, so these rows keep testing the
# expect.txt conjunction rather than doubling as output-contract cases.
mk_tree() {
    local name="$1" good_say="$2" good_expect="$3" bad_say="$4" bad_expect="$5"
    local dir="$scratch/$name/stub-gate"
    mkdir -p "$dir/good" "$dir/bad"
    printf '%s\nSTUB-GATE: clean (stub)\n' "$good_say" > "$dir/good/say"
    printf '0\n' > "$dir/good/rc"
    [[ -n "$good_expect" ]] && printf '%s\n' "$good_expect" > "$dir/good/expect.txt"
    printf '%s\n  help: stub remedy\n' "$bad_say" > "$dir/bad/say"
    printf '1\n' > "$dir/bad/rc"
    printf '%s\n' "$bad_expect" > "$dir/bad/expect.txt"
    echo "$scratch/$name"
}

run_tree() { gate_arm_run --run-gate-tests "$1" "$scratch/checks" 2>&1; }

fails=0
assert_has()    { grep -qF -- "$2" <<<"$3" || { echo "FAIL [$1]: expected present: $2"; fails=$((fails + 1)); }; }
assert_absent() { grep -qF -- "$2" <<<"$3" && { echo "FAIL [$1]: expected absent: $2"; fails=$((fails + 1)); }; return 0; }
assert_rc()     { [[ "$2" -eq "$3" ]] || { echo "FAIL [$1]: expected exit $3, got $2"; fails=$((fails + 1)); }; }

# Row 1 — the unit: two expect lines, the gate prints only the first. Under the
# disjunction this passed; the second line must now be reported missing.
t="$(mk_tree first-only 'good ok' '' 'alpha fired' $'alpha fired\nbeta fired')"
out="$(run_tree "$t")"; rc=$?
assert_rc  first-only "$rc" 1
assert_has first-only 'missing: beta fired' "$out"
assert_absent first-only 'missing: alpha fired' "$out"

# Row 3 — the symmetric half: only the second line prints. A fix that checked
# just the first line would pass row 1 and leave the defect live here.
t="$(mk_tree second-only 'good ok' '' 'beta fired' $'alpha fired\nbeta fired')"
out="$(run_tree "$t")"; rc=$?
assert_rc  second-only "$rc" 1
assert_has second-only 'missing: alpha fired' "$out"
assert_absent second-only 'missing: beta fired' "$out"

# Row 2 — both lines print: the conjunction is satisfied, order irrelevant (the
# gate prints them in the reverse of the expect file's order).
t="$(mk_tree both 'good ok' '' $'beta fired\nalpha fired' $'alpha fired\nbeta fired')"
out="$(run_tree "$t")"; rc=$?
assert_rc  both "$rc" 0
assert_has both 'GATE-TESTS: clean' "$out"

# Row 4 — the single-line case is untouched: the tightening must not regress the
# shape nearly every tracked expect file uses.
t="$(mk_tree single 'good ok' '' 'alpha fired' 'alpha fired')"
out="$(run_tree "$t")"; rc=$?
assert_rc  single "$rc" 0
assert_has single 'GATE-TESTS: clean' "$out"

# Row 5 — a blank line between two real ones asserts nothing: it is a separator,
# and reporting it as a pin would name an empty missing line.
t="$(mk_tree blank 'good ok' '' $'alpha fired\nbeta fired' $'alpha fired\n\nbeta fired')"
out="$(run_tree "$t")"; rc=$?
assert_rc  blank "$rc" 0
assert_has blank 'GATE-TESTS: clean' "$out"
assert_absent blank 'missing: ' "$out"

# Row 6 — good/expect.txt reads identically: optional to supply, conjunctive
# once supplied.
t="$(mk_tree good-side 'gamma ok' $'gamma ok\ndelta ok' 'alpha fired' 'alpha fired')"
out="$(run_tree "$t")"; rc=$?
assert_rc  good-side "$rc" 1
assert_has good-side 'missing: delta ok' "$out"

# Every missing line is named in one run: an author fixing one pin per re-run is
# the same under-assertion defect at a smaller scale.
t="$(mk_tree all-missing 'good ok' '' 'alpha fired' $'alpha fired\nbeta fired\ngamma fired')"
out="$(run_tree "$t")"; rc=$?
assert_rc  all-missing "$rc" 1
assert_has all-missing 'missing: beta fired' "$out"
assert_has all-missing 'missing: gamma fired' "$out"

# The output contract, asserted at runtime rather than by grepping a gate's
# source (§Output contract). A clean case that satisfies every expect line but
# emits no canonical clean line is still a failure — this is the assertion that
# survives a gate's source becoming a compiled subcommand there is nothing to
# grep.
mk_no_clean_line() {
    local dir="$scratch/no-clean/stub-gate"
    mkdir -p "$dir/good" "$dir/bad"
    printf 'gamma ok\n' > "$dir/good/say"; printf '0\n' > "$dir/good/rc"
    printf 'gamma ok\n' > "$dir/good/expect.txt"
    printf 'alpha fired\n  help: stub remedy\n' > "$dir/bad/say"; printf '1\n' > "$dir/bad/rc"
    printf 'alpha fired\n' > "$dir/bad/expect.txt"
    echo "$scratch/no-clean"
}
out="$(run_tree "$(mk_no_clean_line)")"; rc=$?
assert_rc  no-clean "$rc" 1
assert_has no-clean "emitted no '<NAME>: clean" "$out"

# The symmetric half: a violating case with no help: remedy line.
mk_no_help_line() {
    local dir="$scratch/no-help/stub-gate"
    mkdir -p "$dir/good" "$dir/bad"
    printf 'STUB-GATE: clean (stub)\n' > "$dir/good/say"; printf '0\n' > "$dir/good/rc"
    printf 'alpha fired\n' > "$dir/bad/say"; printf '1\n' > "$dir/bad/rc"
    printf 'alpha fired\n' > "$dir/bad/expect.txt"
    echo "$scratch/no-help"
}
out="$(run_tree "$(mk_no_help_line)")"; rc=$?
assert_rc  no-help "$rc" 1
assert_has no-help "no 'help:' remedy line" "$out"

# Write-side hermeticity (§run-gate-tests): the `cd` into the case dir puts a
# member's cwd inside a *tracked* corpus, so a member writing runtime scratch
# under GATE_SDK_TMP_DIR's repo-relative default would deposit it there — ignored,
# surviving the run, and riding a verbatim vendor of the kit tree. The runner
# absolutizes the knob for the case invocation, so the write lands in scratch and
# the corpus is untouched. Asserted three ways: the write must land (a one-sided
# "corpus is clean" row passes just as well when it never happened), the corpus
# must stay clean, and a bespoke *.test.sh must NOT inherit the pin — it runs at
# the invoker's cwd and sandboxes its own trees off the relative default, so a
# process-wide export hands it the invoker's live scratch instead.
#
# Both dispatch spellings are exercised, because the pin reaches them by two
# different routes and a one-sided fix passes either row alone. A `.sh` member
# reads the *bare* name out of the per-case child environment. A `.gate` member
# reads only GATE_SDK_KNOB_GATE_SDK_TMP_DIR, which nothing sets directly — the
# bridge derives it inside the argv-resolving shell from the bare value that
# shell resolved, so the pin has to enter that shell rather than the case child.
# Before the pin did, the bridged half baked the repo-relative default into the
# `env` prefix and every .gate member wrote inside the case dir.
mk_writer() {
    local dir="$scratch/writer/stub-writer" bdir="$scratch/writer/stub-bridged"
    mkdir -p "$dir/good" "$dir/bad" "$bdir/good" "$bdir/bad"
    printf 'STUB-WRITER: clean (stub)\n' > "$dir/good/expect.txt"
    printf 'alpha fired\n' > "$dir/bad/expect.txt"
    printf 'STUB-BRIDGED: clean (stub)\n' > "$bdir/good/expect.txt"
    printf 'alpha fired\n' > "$bdir/bad/expect.txt"
    { printf '#!/usr/bin/env bash\n'
      printf 'printf "%%s\\n" "${GATE_SDK_TMP_DIR-<unset>}" > "%s/env-probe.seen"\n' "$scratch"
      printf 'exit 0\n'
    } > "$scratch/writer/env-probe.test.sh"
    echo "$scratch/writer"
}
# The stub writes under the knob's own inline spelling — the one check-crate-arms
# uses — and the inner run is handed no GATE_SDK_TMP_DIR at all, so the default is
# what the runner has to absolutize. Its cwd is the scratch root, which is
# therefore where a pinned write lands and where an unpinned one does not.
{ printf '#!/usr/bin/env bash\n'
  printf 'D="${GATE_SDK_TMP_DIR:-.tmp}"\n'
  printf 'mkdir -p "$D" && : >"$D/stub-writer.marker"\n'
  printf 'if [[ "$PWD" == */bad ]]; then echo "alpha fired"; echo "  help: stub remedy"; exit 1; fi\n'
  printf 'echo "STUB-WRITER: clean (stub)"\n'
} > "$scratch/checks/stub-writer.sh"
chmod +x "$scratch/checks/stub-writer.sh"
# The .gate half needs a binary that knows the stub member, so the runner is
# driven through a stand-in that answers --knobs and runs stub-bridged itself and
# forwards every other argv — the arm's own included — to the real binary. That
# keeps the row an end-to-end exercise of the live arm rather than a replay of
# its resolver: the stand-in is what GATE_SDK_NATIVE_BIN names, so the runner
# resolves the case through it exactly as it resolves a real .gate member.
: >"$scratch/checks/stub-bridged.gate"
{ printf '#!/usr/bin/env bash\n'
  printf 'if [[ "$1" == --knobs && "$2" == stub-bridged ]]; then printf "GATE_SDK_TMP_DIR\\n"; exit 0; fi\n'
  printf 'if [[ "$1" == stub-bridged ]]; then\n'
  # The member reads the PREFIXED spelling and only that one, the way
  # walk::knob_scalar does; falling back to the relative default is what makes an
  # unpinned bridge observable as a write inside the case dir.
  printf '    D="${GATE_SDK_KNOB_GATE_SDK_TMP_DIR:-.tmp}"\n'
  printf '    mkdir -p "$D" && : >"$D/stub-bridged.marker"\n'
  printf '    if [[ "$PWD" == */bad ]]; then echo "alpha fired"; echo "  help: stub remedy"; exit 1; fi\n'
  printf '    echo "STUB-BRIDGED: clean (stub)"; exit 0\n'
  printf 'fi\n'
  printf 'exec "%s" "$@"\n' "$GATE_SDK_NATIVE_BIN"
} > "$scratch/bridgedbin"
chmod +x "$scratch/bridgedbin"
t="$(mk_writer)"
# gate_arm_run is a shell function, so the override is an `unset` inside the
# subshell rather than an `env -u` prefix — the reason §run-gate-tests gives for
# gate_env existing at all: env cannot invoke a shell function.
out="$( cd "$scratch" && unset GATE_SDK_TMP_DIR \
    && GATE_SDK_NATIVE_BIN="$scratch/bridgedbin" gate_arm_run --run-gate-tests "$t" "$scratch/checks" 2>&1 )"; rc=$?
assert_rc  writer "$rc" 0
for marker in stub-writer stub-bridged; do
    [[ -f "$scratch/.tmp/$marker.marker" ]] || {
        echo "FAIL [writer]: $marker's scratch write did not land at the invoker's root — the corpus assertion below would pass on a write that never happened"
        fails=$((fails + 1))
    }
done
for case_dir in "$t"/stub-writer/*/ "$t"/stub-bridged/*/; do
    [[ -e "$case_dir/.tmp" ]] && {
        echo "FAIL [writer]: ${case_dir#"$t"/} acquired a .tmp — the runner let a member write into the tracked corpus"
        fails=$((fails + 1))
    }
done
seen="$(cat "$scratch/env-probe.seen" 2>/dev/null)"
[[ "$seen" == "<unset>" ]] || {
    echo "FAIL [writer]: a bespoke test saw GATE_SDK_TMP_DIR=$seen — the case pin leaked process-wide, handing every *.test.sh the invoker's live scratch in place of its own sandbox"
    fails=$((fails + 1))
}

[[ "$fails" -eq 0 ]] || { echo "run-gate-tests.test: $fails assertion(s) failed"; exit 1; }
echo "run-gate-tests.test: clean (expect.txt is a per-line conjunction, order-independent, blanks inert, all missing lines named; the output contract is asserted at runtime on both cases; scratch writes land outside the case dir on both dispatch spellings)"
exit 0
