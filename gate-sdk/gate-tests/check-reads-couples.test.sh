#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §check-reads-couples — the `.gate` arm, which the one good/bad pair
# cannot hold: the pair models exit 0 and exit 1 against a shell source, while this arm reads a
# registry member's declared roots and its refusals are exit 2 by the fail-closed contract. A
# live member exercises the covered path in the battery, but nothing there reaches the '?' skip,
# the filter being applied rather than ignored, or either refusal.
#
# The cases below name **real registry members** and let the substrate answer for them. Before
# §The sixth budget batch they drove a stub binary answering `--reads`, because the reader was a
# shell script asking a separate process; the compiled reader answers from its own registry, so a
# stub would be testing a provider the gate no longer has. Two consequences are recorded rather
# than dropped silently: the absent-binary refusal is gone because the binary is running by
# construction, and the non-zero-`--reads` refusal is now a descriptor whose name the registry
# does not carry — the same condition reached the way the compiled form can reach it.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
fails=0
cases=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

MANIFEST_NARROW='# graph: couples=corpus/SPEC*.md dir=one valve=none tier=precommit'
MANIFEST_WIDE='# graph: couples=corpus/*.md dir=one valve=none tier=precommit'

# A case dir holding one descriptor named for a real member, plus a tracked corpus whose
# shape distinguishes a filter that is applied from one that is ignored: `other.txt` sits at
# the uncovered depth and is excluded by every filter the named member declares.
make_case() {  # $1=label  $2=descriptor-basename  $3=manifest
    local dir="$tmp/$1"
    mkdir -p "$dir/corpus/sub"
    printf '%s\n' "$3" > "$dir/$2"
    : > "$dir/corpus/SPEC.md"
    : > "$dir/corpus/SPEC-amend.md"
    : > "$dir/corpus/sub/SPEC.md"
    : > "$dir/corpus/sub/other.txt"
    ( cd "$dir" && git init -q . && git add -A ) >/dev/null 2>&1
}

run_case() {  # $1=label  $2=descriptor-basename  $3=want-rc  $4=want-substring
    local label="$1" desc="$2" want="$3" substr="$4" out rc
    cases=$((cases + 1))
    out="$( cd "$tmp/$label" && gate_run check-reads-couples "$DIR/checks" "$desc" 2>&1 )"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$substr" ]] && ! grep -qF -- "$substr" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$substr': $out"; fails=$((fails + 1))
    fi
}

# A — the declared roots' tracked reads are covered. The named member declares its scan root
# twice under two filter knobs, and one `corpus/*.md` couple covers both levels of both walks.
make_case covered check-stage-entry.gate "$MANIFEST_WIDE"
run_case covered check-stage-entry.gate 0 '2 resolvable walk(s) covered'

# A2 — a pattern of one segment covers a tracked file three segments deep: `*` crosses '/'. A
# narrowed couples matcher reds here.
make_case span check-stage-entry.gate '# graph: couples=*.md dir=one valve=none tier=precommit'
run_case span check-stage-entry.gate 0 '2 resolvable walk(s) covered'

# B — the same roots, a couple whose string prefix stops short of `corpus/sub/`, so the deeper
# tracked file is uncovered and the finding must name it.
make_case uncovered check-stage-entry.gate "$MANIFEST_NARROW"
run_case uncovered check-stage-entry.gate 1 "corpus/sub/SPEC.md"

# B2 — the same red, read the other way: `corpus/sub/other.txt` is equally uncovered and is
# equally deep, so its ABSENCE from the finding is what proves the filter is applied rather
# than ignored. A filter dropped on the floor would flag it too.
cases=$((cases + 1))
out="$( cd "$tmp/uncovered" && gate_run check-reads-couples "$DIR/checks" check-stage-entry.gate 2>&1 )"
if grep -qF -- 'corpus/sub/other.txt' <<<"$out"; then
    echo "  FAIL [filter-applied]: a file no declared filter selects was still demanded: $out"
    fails=$((fails + 1))
fi

# C — a root the member cannot bound statically. It is counted, never assumed empty: the clean
# line reports it in the same skip counter the shell arm's unresolvable roots use. The member named
# here must be one whose root genuinely does not resolve: `check-path-dialect` walks the crate source
# tree, whose location is a knob's value. It replaces `check-action-pinning`, which the
# statically-resolvable-root refusal moved off `?` — a fixture whose premise the rule retired.
make_case unbounded check-path-dialect.gate "$MANIFEST_NARROW"
run_case unbounded check-path-dialect.gate 0 '1 undecidable walk(s) skipped-and-counted'

# D — a descriptor claiming the removed opt-out. There is deliberately no descriptor-level
# exemption, so the uncovered read is still a finding: the line buys nothing.
make_case claimed_exemption check-stage-entry.gate \
    "$MANIFEST_NARROW
# reads-couples-exempt: the walks are covered elsewhere"
run_case claimed_exemption check-stage-entry.gate 1 "corpus/sub/SPEC.md"

# E — surviving refusal one: a descriptor whose name the registry does not carry. The read set
# is unavailable, and "unavailable" must never read as "reads nothing".
make_case unknown_member sandbox.gate "$MANIFEST_NARROW"
run_case unknown_member sandbox.gate 2 'no registered subcommand answers'

# F — surviving refusal two, and it is fail-closed by the same contract: a declared filter knob
# the gate cannot resolve is exit 2, never an empty filter silently widening the demand back to
# the whole root. Driven by a knob file that refuses the filter knob: the filter knob is static, so
# it resolves in process and the unresolvable state is a malformed line rather than a withheld one.
cases=$((cases + 1))
BIN="${GATE_SDK_NATIVE_BIN:-}"
make_case withheld check-evidence-baseline.gate "$MANIFEST_NARROW"
if [[ -x "$BIN" ]]; then
    printf 'EVIDENCE_KIT_SCENARIO_GLOBS = not-a-pair\n' >"$tmp/withheld.knobs"
    out="$( cd "$tmp/withheld" && EVIDENCE_KIT_KNOB_FILE="$tmp/withheld.knobs" "$BIN" check-reads-couples check-evidence-baseline.gate 2>&1 )"
    rc=$?
    if [[ "$rc" -ne 2 ]]; then
        echo "  FAIL [filter-unresolvable]: want exit 2, got $rc -- $out"; fails=$((fails + 1))
    elif ! grep -qF -- 'EVIDENCE_KIT_SCENARIO_GLOBS' <<<"$out"; then
        echo "  FAIL [filter-unresolvable]: the refusal does not name the knob: $out"
        fails=$((fails + 1))
    fi
else
    echo "  FAIL [filter-unresolvable]: no binary to withhold a knob from"; fails=$((fails + 1))
fi

# G — the shell arm is unaffected: a `.sh` source with a covered walk still analyzes, so a
# refusal above cannot be passing for a parse failure.
mkdir -p "$tmp/shell/corpus"
: > "$tmp/shell/corpus/a.md"
printf '%s\n' \
    "$MANIFEST_WIDE" \
    'gate_find "corpus" -name '"'"'*.md'"'"' -type f' > "$tmp/shell/sandbox-gate.sh"
( cd "$tmp/shell" && git init -q . && git add -A ) >/dev/null 2>&1
cases=$((cases + 1))
out="$( cd "$tmp/shell" && gate_run check-reads-couples "$DIR/checks" sandbox-gate.sh 2>&1 )"; rc=$?
if [[ "$rc" -ne 0 ]]; then
    echo "  FAIL [shell-arm]: want exit 0, got $rc -- $out"; fails=$((fails + 1))
elif ! grep -qF -- 'READS-COUPLES: clean' <<<"$out"; then
    echo "  FAIL [shell-arm]: exit 0 but not clean: $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-reads-couples.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-reads-couples.test.sh: clean ($cases cases over real registry members)"
exit 0
