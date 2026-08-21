#!/usr/bin/env bash
# Behavioral test of check-graph's whole-registry path — assertions A (manifest
# well-formedness), B (couples<->trigger parity), C (the cycle-valve rule),
# D (pre-commit + commit-msg hook freshness), E (graph-artifact freshness) and
# F (emitted asset hrefs resolve). None is reachable from the good/bad pair: the
# pair's cwd is a case dir and a `good/` case must exit 0, while D and E always
# run on the full path — D's `gen-pre-commit.sh --emit` cds to
# `git rev-parse --show-toplevel`, so satisfying it from a case dir would need a
# committed byte-copy of the real repo's generated hook inside gate-tests/.
# So the corpus is a mini-consumer tree built here and thrown away: the port
# cannot add or remove a file in it, which is the property criterion 4 asks of a
# gate-source auditor's oracle (gate-sdk/SPEC.md §The port-candidate criteria).
#
# The tree carries both declaration spellings (a `.sh` member and a `.gate` one),
# all four of assertion B's coverage branches, and all three cycle-valve branches.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
CHECKS="$DIR/checks"
GEN="$DIR/bin/gen-pre-commit.sh"

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT
repo="$SANDBOX/repo"
fails=0
rc=0
out=""

# the mini-consumer's own kit set, so member resolution and the emitted hook stay
# inside the sandbox instead of reaching this repo's kits
export GATE_SDK_KIT_DIRS="$repo"

member() {  # member() <name> <manifest> — a `.sh` declaration in the sandbox registry
    printf '#!/usr/bin/env bash\n# graph: %s\necho "X: clean (nothing)"\n' "$2" \
        >"$repo/scripts/$1.sh"
    chmod +x "$repo/scripts/$1.sh"
}

seed() {
    rm -rf "$repo"
    mkdir -p "$repo/scripts"
    git -C "$repo" init -q
    cat >"$repo/scripts/gates.list" <<'LIST'
check-alpha
check-beta
check-gamma
check-delta
check-epsilon
LIST
    # B branch 2 (exact token) and branch 4 (a `*.<ext>` trigger over a globbed couple)
    member check-alpha 'couples=scripts/gates.list,scripts/*.sh trigger=scripts/gates.list,*.sh dir=one valve=none tier=precommit'
    # B branch 1 (a `*` trigger covers everything); tier=commit-msg drives D's second hook
    member check-beta 'couples=docs/site.md trigger=* dir=one valve=none tier=commit-msg'
    # C branch 1: a dir=bi member spanning a leading and a lagging surface takes PROPOSED
    member check-gamma 'couples=SPEC.md,scripts/*.sh dir=bi valve=PROPOSED tier=precommit'
    # the `.gate` declaration spelling, so assertion A resolves and reads a
    # descriptor's manifest; tier=align-only keeps it out of both hooks, so the
    # sandbox needs no binary to generate them. C branch 2: a leading surface with
    # no lagging one takes either valve
    printf '# graph: %s\n' 'couples=SPEC.md dir=bi valve=none tier=align-only' \
        >"$repo/scripts/check-delta.gate"
    # B branch 3 (a literal couple read against the trigger as a bash pattern)
    # and C branch 3 (no leading surface — valve must be none)
    member check-epsilon 'couples=docs/site.md trigger=docs/*.md dir=bi valve=none tier=precommit'
    cat >"$repo/scripts/graph-vocab.sh" <<'VOCAB'
GRAPH_VOCAB=('scripts/gates.list' 'scripts/*.sh' '*.sh' 'docs/site.md' 'docs/*.md' 'SPEC.md')
GRAPH_LEADING=('SPEC.md')
GRAPH_LAGGING=('scripts/*.sh')
GRAPH_LAYERS=('surfaces:governed surfaces')
VOCAB
}

regen() {  # regen() -> rewrite both hooks and the graph artifact from the manifests
    ( cd "$repo" && bash "$GEN" --write >/dev/null 2>&1 ) || return 1
    ( cd "$repo" && bash "$DIR/bin/run-gates.sh" --emit graph >scripts/CHECK-GRAPH.html 2>/dev/null ) || return 1
}

run() {  # run() -> the full whole-registry gate over the sandbox; sets rc/out
    out="$( cd "$repo" && gate_run check-graph "$CHECKS" 2>&1 )"; rc=$?
}

want_red() {  # want_red() <label> <substring>
    if [[ "$rc" -ne 1 ]]; then
        echo "  FAIL [$1]: want exit 1, got $rc -- $out"; fails=$((fails + 1))
    elif ! grep -qF -- "$2" <<<"$out"; then
        echo "  FAIL [$1]: exit 1 but output lacks '$2' -- $out"; fails=$((fails + 1))
    fi
}

want_green() {  # want_green() <label>
    if [[ "$rc" -ne 0 ]]; then
        echo "  FAIL [$1]: want exit 0, got $rc -- $out"; fails=$((fails + 1))
    elif ! grep -qF 'CHECK-GRAPH: clean' <<<"$out"; then
        echo "  FAIL [$1]: exit 0 but output lacks the clean line -- $out"; fails=$((fails + 1))
    fi
}

# --- baseline: five members, both spellings, every B and C branch, clean --------
seed
regen || { echo "  FAIL [baseline]: could not generate the sandbox hooks/artifact"; fails=$((fails + 1)); }
run; want_green baseline

# --- assertion A: manifest well-formedness -------------------------------------
seed
member check-alpha 'couples=scripts/gates.list trigger=* dir=one valve=none tier=precommit bogus=1'
regen || true; run
want_red a-unknown-key "unknown manifest key 'bogus=1'"

seed
member check-alpha 'couples=scripts/gates.list trigger=* dir=mono valve=none tier=precommit'
regen || true; run
want_red a-enum "dir= must be bi|one (got 'mono')"

seed
printf 'check-zeta\n' >>"$repo/scripts/gates.list"
regen || true; run
want_red a-unresolved "check-zeta is in gates.list but resolves in none of"

seed
member check-alpha 'couples=scripts/gates.list,lib/x.rs trigger=* dir=one valve=none tier=precommit'
regen || true; run
want_red a-vocab "couples surface 'lib/x.rs' not in the declared GRAPH_VOCAB"

# --- assertion B: a couple no trigger glob would fire on ------------------------
seed
member check-epsilon 'couples=docs/site.md trigger=scripts/*.sh dir=bi valve=none tier=precommit'
regen || true; run
want_red b-parity "PARITY: check-epsilon couples 'docs/site.md' but its trigger"

# --- assertion C: the two reds the three branches leave -------------------------
seed
member check-gamma 'couples=SPEC.md,scripts/*.sh dir=bi valve=none tier=precommit'
regen || true; run
want_red c-cycle-needs-valve "check-gamma is a design<->code bi cycle"

seed
member check-epsilon 'couples=docs/site.md trigger=docs/*.md dir=bi valve=PROPOSED tier=precommit'
regen || true; run
want_red c-no-leading "check-epsilon is a dir=bi bijection with no leading design surface"

# --- assertion D: both generated hooks -----------------------------------------
seed; regen || true
printf '# hand-edited\n' >>"$repo/scripts/git-hooks/pre-commit"
run; want_red d-hook-stale "scripts/git-hooks/pre-commit is stale"

seed; regen || true
rm -f "$repo/scripts/git-hooks/pre-commit"
run; want_red d-hook-absent "scripts/git-hooks/pre-commit does not exist"

seed; regen || true
printf '# hand-edited\n' >>"$repo/scripts/git-hooks/commit-msg"
run; want_red d-msg-hook-stale "scripts/git-hooks/commit-msg is stale"

# --- assertion E: the graph artifact -------------------------------------------
seed; regen || true
printf '<!-- hand-edited -->\n' >>"$repo/scripts/CHECK-GRAPH.html"
run; want_red e-artifact-stale "scripts/CHECK-GRAPH.html is stale"

seed; regen || true
rm -f "$repo/scripts/CHECK-GRAPH.html"
run; want_red e-artifact-absent "scripts/CHECK-GRAPH.html does not exist"

# --- assertion F: an emitted local asset href that resolves, and one that does not
# The kit's own emission carries no local href/src at all (its one external
# reference is the mermaid CDN import, which this assertion's scan excludes), so
# the failure path is only reachable through a theme — the shortcut the assertion
# exists to refuse, a chrome fragment linking an asset beside the site rather than
# inlining it. The injection is a GATE_SDK_GRAPH_THEME_DIR part file, inlined byte
# verbatim at the point after <body> the retired graph_theme_header() fed.
seed
mkdir -p "$repo/theme"
printf '  <img src="chrome-logo.png" alt="">\n' >"$repo/theme/header.html"
export GATE_SDK_GRAPH_THEME_DIR="$repo/theme"
regen || true; run
want_red f-asset-href "emitted asset 'chrome-logo.png' does not resolve to a file under scripts/"

: >"$repo/scripts/chrome-logo.png"
regen || true; run
want_green f-asset-href-resolves
unset GATE_SDK_GRAPH_THEME_DIR

if [[ "$fails" -gt 0 ]]; then
    echo "check-graph-tree.test: $fails case(s) failed"
    exit 1
fi
echo "check-graph-tree.test: ok (constructed consumer tree: A/B/C/D/E/F, both declaration spellings, 4 parity branches, 3 cycle-valve branches, 15 cases)"
exit 0
