#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the declaration configurations the
# good/+bad/ pair cannot hold, it being one invocation each. Every case is a sandbox rather than
# a live tree, so a coverage claim here cannot quietly stop being true when a cohort lands.
#
# The cases name **real registry members** and let the substrate answer for them. Before this
# member's own port they drove a stub binary answering `--list`, because the reader was a shell
# script asking a separate process; the compiled reader answers from its own registry, so a stub
# would be testing a provider the gate no longer has. What that retires is recorded rather than
# dropped silently: the absent-binary refusal and the roster-unread clean line are gone because
# the binary is running by construction, and the owner-column fallback is gone because a binary
# without the column is a binary without this subcommand. What it moves is assertion B's roster
# matrix — the subset vendoring, its near miss, the consumer sentinel in both directions and the
# reference-only allowance — into the crate unit tests beside the rule, where a roster is a value
# rather than a process.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
REPO="$(cd "$DIR/.." && pwd)"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

fails=0
cases=0

DESC='# graph: couples=docs/*.md dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-gate-substrate-parity — fixture descriptor'
SHELL_DECL='#!/usr/bin/env bash
# graph: couples=docs/*.md dir=one valve=none tier=precommit
echo "ALPHA: clean (stub)"'

make_sandbox() {  # make_sandbox <label>
    local d="$tmp/$1"
    mkdir -p "$d/scripts" "$d/kitroot/checks" "$d/impl"
    printf '%s\n' "$SHELL_DECL" > "$d/scripts/check-alpha.sh"
    printf 'fn f() {}\n' > "$d/impl/gate.rs"
    cat > "$d/conservation.md" <<'EOF'
## Meta-gate conservation for the binary substrate

| Meta-gate | Disposition |
|---|---|
| `check-unrelated` | Retained. |

## Next section
EOF
    printf '%s\n' "$d"
}

# The case's output lands in OUT rather than in a command substitution, so an assertion made
# after the run still increments this script's own counters: a `$( … )` capture would run the
# whole case in a subshell and lose every failure it recorded.
OUT="$tmp/case-output"

run_case() {  # run_case <label> <sandbox> <want-rc> <substring>
    local label="$1" d="$2" want="$3" substr="$4" rc
    cases=$((cases + 1))
    ( cd "$d" && env GATE_SDK_KIT_DIRS=kitroot GATE_SDK_NATIVE_CRATE=crate \
        GATE_SDK_NATIVE_SRC=impl \
        bash -c 'source "$1"; gate_run check-gate-substrate-parity "$2" scripts conservation.md' \
        bash "$DIR/lib/test-hermetic.sh" "$DIR/checks" ) > "$OUT" 2>&1
    rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc --"; sed 's/^/    /' "$OUT"
        fails=$((fails + 1)); return 1
    fi
    if [[ -n "$substr" ]] && ! grep -qF -- "$substr" < "$OUT"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$substr':"; sed 's/^/    /' "$OUT"
        fails=$((fails + 1)); return 1
    fi
    return 0
}

expect_absent() {  # expect_absent <label> <substring>
    cases=$((cases + 1))
    if grep -qF -- "$2" < "$OUT"; then
        echo "  FAIL [$1]: output carries '$2' when it must not:"; sed 's/^/    /' "$OUT"
        fails=$((fails + 1))
    fi
}

expect_present() {  # expect_present <label> <substring>
    cases=$((cases + 1))
    if ! grep -qF -- "$2" < "$OUT"; then
        echo "  FAIL [$1]: output lacks '$2':"; sed 's/^/    /' "$OUT"
        fails=$((fails + 1))
    fi
}

# --- A: no descriptors at all — the roster half is the only half with anything to say ---
# The post-revert tree, and the state a descriptor-count guard would blank out. Assertion E's
# sibling half scans nothing and says so; the roster half runs regardless.
A="$(make_sandbox no-descriptors)"
printf 'check-alpha\n' > "$A/scripts/gates.list"
run_case no-descriptors "$A" 0 '0 descriptor(s) in parity with the'

# --- B: descriptors present, none dispatching, no target roster ---
# Every vendored tree once a cohort's descriptors ship. Assertion F's missing-roster arm must
# stay quiet: a consumer receives kit roots, never the crate, so its roster is absent by
# construction.
B="$(make_sandbox none-dispatching)"
printf 'check-alpha\n' > "$B/scripts/gates.list"
printf '%s\n' "$DESC" > "$B/kitroot/checks/check-stage-entry.gate"
run_case none-dispatching "$B" 0 '1 member(s) with one declaration each, 0 of them dispatching to the binary' \
    && expect_absent none-dispatching-roster 'no target roster'

# --- C: a consumer dispatching to a placed binary with no crate, and no roster ---
# The vendored-tree shape the consumer smoke is in: assertion F stays quiet because declaring
# platform support is the publishing tree's act. The same run is assertion G's **empty corpus**:
# every registered member resolves to a descriptor, so the shell declaration set is zero and the
# verdict is green with a counted zero rather than a red for finding none.
C="$(make_sandbox consumer-dispatching)"
rm "$C/scripts/check-alpha.sh"
printf 'check-stage-entry\n' > "$C/scripts/gates.list"
printf '%s\n' "$DESC" > "$C/kitroot/checks/check-stage-entry.gate"
if run_case consumer-dispatching "$C" 0 '1 member(s) with one declaration each, 1 of them dispatching to the binary'; then
    expect_absent consumer-dispatching-roster 'no target roster'
    expect_present empty-declaration-corpus \
        "0 of the 0 shell declaration(s) declare '# no-port:' with a cause and 0 declare '# port-until:' with a slug"
fi

# --- D: the publishing counterpart — dispatching, crate source tracked, still no roster ---
# Source, not directory presence: build output under the crate root must not read as a publisher.
# The consumer's own descriptors are copied in because the publishing tree is precisely where the
# sentinel-owned subcommands come into scope, and the finding under test must be the only one.
D="$(make_sandbox publishing-no-roster)"
rm "$D/scripts/check-alpha.sh"
printf 'check-stage-entry\n' > "$D/scripts/gates.list"
printf '%s\n' "$DESC" > "$D/kitroot/checks/check-stage-entry.gate"
cp "$REPO"/scripts/*.gate "$D/scripts/"
mkdir -p "$D/crate"
printf 'fn main() {}\n' > "$D/crate/main.src"
git -C "$D" init -q
git -C "$D" add crate/main.src
run_case publishing-no-roster "$D" 1 'no target roster'

if [[ "$fails" -gt 0 ]]; then
    echo "check-gate-substrate-parity.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-gate-substrate-parity.test.sh: clean (declaration configurations: no descriptors, where the roster half is the only live half; descriptors present with none dispatching and no roster, where assertion F stays quiet; a consumer dispatching with no crate, which is also assertion G's empty shell-declaration corpus reported as a counted zero; and the publishing counterpart, where the same absent roster reds — $cases assertions over 4 sandboxes, with assertion B's roster matrix held in the crate unit tests beside the rule)"
exit 0
