#!/usr/bin/env bash
# Direct unit test of check-docs-render-fidelity's batch renderer contract
# (SITE_KIT_RENDERER_BATCH), which the good/bad pair cannot express on its own.
# The pair sets no renderer knob, so lib/site.sh fills the batch default and the
# pair already runs the batch path — what it leaves untested is everything that
# needs a second configuration: the kit's two defaults agreeing, the delta-4
# fill condition, the per-document fallback, and the two exit-2 refusals.
#
# Six assertions:
#   1. parity      — the corpus rendered per-document and batched is byte-identical,
#                    which is what makes the zero-config agreement claim an
#                    assertion rather than a hope.
#   2. fill rule   — a consumer who pinned SITE_KIT_RENDERER gets an EMPTY batch
#                    knob, so an upgrade never swaps a pinned oracle for the
#                    kit's unpinned one; unpinned, the default arrives.
#   3/4. verdicts  — the batch path and the per-document fallback return the SAME
#                    verdict on the same pages, so the speedup changed no finding.
#   5. count       — a batch renderer that passes the probe but returns the wrong
#                    document count exits 2 (fail-closed), never 1.
#   6. probe       — an unresolvable batch command exits 2 rather than silently
#                    falling back to a different parser.
# The gate enumerates tracked pages via git ls-files, so the fixture is a
# throwaway git repo.
#
# A case's renderer configuration crosses as SITE_KIT_CONFIG_FILE rather than as
# the gate's second positional: the compiled member receives resolved knob values
# and never a config path, so the bridge is the one thing that reads a config file
# (gate-sdk/SPEC.md §lib/gate.sh). gate_run resolves it from this cwd.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # site-kit/
CHECKS="$DIR/checks"

fails=0
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT

(
    cd "$tmp" || exit 1
    git init -q
    git config user.email fixture@example.invalid
    git config user.name fixture
    mkdir -p docs
) || { echo "  FAIL: could not init fixture repo"; exit 1; }

cat > "$tmp/docs/leak.md" <<'MD'
# Heading

The consequence clause appends after the ` -> <verdict>` arrow, leaving it
disjoint from the invariant below.
MD

cat > "$tmp/docs/ok.md" <<'MD'
# Heading

The consequence clause appends after the `` -> <verdict> `` arrow, leaving it
disjoint from the invariant below.

| knob | default |
| --- | --- |
| one | two |
MD

for n in 1 2 3; do printf '# Page %s\n\nBody.\n' "$n" > "$tmp/docs/p$n.md"; done

# --- 1. parity: the kit's two defaults agree, document for document ------------
(
    # shellcheck source=../lib/site.sh
    source "$DIR/lib/site.sh"
    bodies=("$(cat "$tmp/docs/ok.md")" "$(cat "$tmp/docs/leak.md")")

    one=()
    for b in "${bodies[@]}"; do one+=("$(printf '%s\n' "$b" | "${SITE_KIT_RENDERER[@]}")"); done

    two=()
    while IFS= read -r -d '' h; do
        while [[ "$h" == *$'\n' ]]; do h="${h%$'\n'}"; done
        two+=("$h")
    done < <(printf '%s\n\000' "${bodies[@]}" | "${SITE_KIT_RENDERER_BATCH[@]}")

    [[ ${#two[@]} -eq ${#one[@]} ]] \
        || { echo "  FAIL: batch returned ${#two[@]} document(s) for ${#one[@]} page(s)"; exit 1; }
    for i in "${!one[@]}"; do
        [[ "${one[$i]}" == "${two[$i]}" ]] \
            || { echo "  FAIL: document $i diverges between the per-document and batch defaults"; exit 1; }
    done
    exit 0
) || fails=$((fails + 1))

# --- 2. the delta-4 fill condition: a pinned renderer suppresses the default ---
n_pinned="$( SITE_KIT_RENDERER=(true) ; source "$DIR/lib/site.sh" ; echo "${#SITE_KIT_RENDERER_BATCH[@]}" )"
[[ "$n_pinned" -eq 0 ]] \
    || { echo "  FAIL: a pinned SITE_KIT_RENDERER still got a batch default ($n_pinned element(s)) — an upgrade would replace the pinned oracle"; fails=$((fails + 1)); }
n_default="$( source "$DIR/lib/site.sh" ; echo "${#SITE_KIT_RENDERER_BATCH[@]}" )"
[[ "$n_default" -gt 0 ]] \
    || { echo "  FAIL: an unpinned SITE_KIT_RENDERER got no batch default — the zero-config speedup is not armed"; fails=$((fails + 1)); }

# A config that declares the batch knob empty leaves the per-document path in
# force with the kit's own renderer — the fallback branch, with no second copy
# of the renderer literal to drift.
printf '%s\n' '# shellcheck shell=bash' \
    '# comment-tier-exempt: fixture config — an empty batch knob forces the per-document fallback branch' \
    '# shellcheck disable=SC2034  # sourced by the gate under test' \
    'SITE_KIT_RENDERER_BATCH=()' > "$tmp/batch-off.sh"

# --- 3/4. both paths, same verdicts -------------------------------------------
git -C "$tmp" add docs/leak.md
for mode in batch per-document; do
    cfg=""; [[ "$mode" == per-document ]] && cfg="$tmp/batch-off.sh"
    out="$( cd "$tmp" && gate_env SITE_KIT_CONFIG_FILE="${cfg:-$SITE_KIT_CONFIG_FILE}" \
        && gate_run check-docs-render-fidelity "$CHECKS" docs 2>&1 )"; rc=$?
    [[ "$rc" -eq 1 ]] \
        || { echo "  FAIL: $mode path: severed span expected exit 1, got $rc: $out"; fails=$((fails + 1)); }
    grep -qF -- "leaked into rendered text" <<<"$out" \
        || { echo "  FAIL: $mode path: red output lacks the span finding: $out"; fails=$((fails + 1)); }
done

git -C "$tmp" rm -q --cached docs/leak.md
git -C "$tmp" add docs/ok.md
for mode in batch per-document; do
    cfg=""; [[ "$mode" == per-document ]] && cfg="$tmp/batch-off.sh"
    out="$( cd "$tmp" && gate_env SITE_KIT_CONFIG_FILE="${cfg:-$SITE_KIT_CONFIG_FILE}" \
        && gate_run check-docs-render-fidelity "$CHECKS" docs 2>&1 )"; rc=$?
    [[ "$rc" -eq 0 ]] \
        || { echo "  FAIL: $mode path: faithful page expected exit 0, got $rc: $out"; fails=$((fails + 1)); }
done

# --- 5. count mismatch is fail-closed (exit 2), not a finding -----------------
# The stub emits exactly two documents whatever it is fed, so it PASSES the
# two-document probe and only the corpus count catches it — which is the point:
# a stub the probe already rejected would never reach the count assertion.
printf '%s\n' '# shellcheck shell=bash' \
    '# comment-tier-exempt: fixture config — a batch renderer that always emits two documents' \
    '# shellcheck disable=SC2034  # sourced by the gate under test' \
    'SITE_KIT_RENDERER_BATCH=(bash -c '\''printf "<p>a</p>\000<p>b</p>\000"'\'')' > "$tmp/miscount.sh"

git -C "$tmp" add docs/p1.md docs/p2.md docs/p3.md
out="$( cd "$tmp" && gate_env SITE_KIT_CONFIG_FILE="$tmp/miscount.sh" \
    && gate_run check-docs-render-fidelity "$CHECKS" docs 2>&1 )"; rc=$?
[[ "$rc" -eq 2 ]] \
    || { echo "  FAIL: batch count mismatch expected exit 2 (fail-closed), got $rc: $out"; fails=$((fails + 1)); }
grep -qF -- "document(s) for" <<<"$out" \
    || { echo "  FAIL: count-mismatch refusal does not name the counts: $out"; fails=$((fails + 1)); }

# --- 6. an unresolvable batch renderer refuses, and does not fall back ---------
printf '%s\n' '# shellcheck shell=bash' \
    '# comment-tier-exempt: fixture config — an unresolvable batch renderer must refuse, never downgrade' \
    '# shellcheck disable=SC2034  # sourced by the gate under test' \
    'SITE_KIT_RENDERER_BATCH=(site-kit-no-such-batch-renderer)' > "$tmp/unresolvable.sh"

out="$( cd "$tmp" && gate_env SITE_KIT_CONFIG_FILE="$tmp/unresolvable.sh" \
    && gate_run check-docs-render-fidelity "$CHECKS" docs 2>&1 )"; rc=$?
[[ "$rc" -eq 2 ]] \
    || { echo "  FAIL: unresolvable batch renderer expected exit 2, got $rc: $out"; fails=$((fails + 1)); }
grep -qF -- "failed its probe" <<<"$out" \
    || { echo "  FAIL: batch probe refusal does not name the probe: $out"; fails=$((fails + 1)); }
grep -qF -- "DOCS-RENDER-FIDELITY: clean" <<<"$out" \
    && { echo "  FAIL: unresolvable batch renderer silently fell back to a clean verdict: $out"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "check-docs-render-fidelity-batch.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-docs-render-fidelity-batch.test: ok (defaults agree byte-for-byte; a pinned renderer suppresses the batch default; both paths agree on verdicts; count mismatch and probe failure both exit 2)"
exit 0
