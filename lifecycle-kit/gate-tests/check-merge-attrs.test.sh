#!/usr/bin/env bash
# Behavioral test of checks/check-merge-attrs.sh — the scenarios the one-pair
# good/bad harness cannot hold. The good/bad fixture pair (run-gate-tests.sh)
# covers the reverse-direction safety edge (a merge=iteration-scoped attribute
# on a path outside the derived set); this file covers the forward direction (a
# derived surface with no attribute), the missing-file case, and the end-to-end
# proof that the keep-ours driver resolves an attributed surface to the arriving
# (checked-out) side across a real two-branch merge.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
GATE="$DIR/checks/check-merge-attrs.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

check_case() {  # $1=label  $2=dir  $3=want-rc  $4=want-substring
    local out rc
    out="$(cd "$2" && "$GATE" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$4" ]] && ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$4':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# --- forward direction: a derived surface with no attribute (default config: the
#     state + lesson-evidence files are always in the set) ---
fwd="$SANDBOX/forward"
mkdir -p "$fwd"
cat >"$fwd/.gitattributes" <<'EOF'
.workflow/WORKFLOW-STATE.txt merge=iteration-scoped
EOF
check_case "forward-missing-lesson" "$fwd" 1 "no merge=iteration-scoped attribute"

# --- missing .gitattributes entirely: every derived surface is unattributed ---
none="$SANDBOX/none"
mkdir -p "$none"
check_case "missing-gitattributes" "$none" 1 "the merge-supersede rule is unmechanized"

# --- the keep-ours driver resolves an attributed surface to the arriving side ---
# A real two-branch merge in a sandbox git repo: the driver definition is `true`
# (writes nothing, so %A — the checked-out/ours version — survives).
repo="$SANDBOX/repo"
mkdir -p "$repo/.workflow"
git -C "$repo" init -q
git -C "$repo" config merge.iteration-scoped.driver true
cat >"$repo/.gitattributes" <<'EOF'
.workflow/WORKFLOW-STATE.txt merge=iteration-scoped
EOF
printf 'iter-base state\n' >"$repo/.workflow/WORKFLOW-STATE.txt"
printf 'shared line\n' >"$repo/shared.txt"
git -C "$repo" add -A
git -C "$repo" -c user.email=t@t.invalid -c user.name=t commit -q -m base

git -C "$repo" checkout -q -b feature
printf 'iter-feature state\n' >"$repo/.workflow/WORKFLOW-STATE.txt"
printf 'shared line\nfeature addition\n' >"$repo/shared.txt"
git -C "$repo" -c user.email=t@t.invalid -c user.name=t commit -qam feature

git -C "$repo" checkout -q main 2>/dev/null || git -C "$repo" checkout -q master
printf 'iter-main state\n' >"$repo/.workflow/WORKFLOW-STATE.txt"
git -C "$repo" -c user.email=t@t.invalid -c user.name=t commit -qam main

# merge feature into the arriving (checked-out) main; the driver keeps ours on
# the attributed state file, ordinary three-way merge folds shared.txt.
git -C "$repo" -c user.email=t@t.invalid -c user.name=t merge -q --no-edit feature >/dev/null 2>&1
merged_state="$(cat "$repo/.workflow/WORKFLOW-STATE.txt")"
if [[ "$merged_state" != "iter-main state" ]]; then
    echo "  FAIL [driver-keeps-ours]: attributed state file resolved to '$merged_state', want 'iter-main state' (the arriving/ours side)"
    fails=$((fails + 1))
fi
if ! grep -qF 'feature addition' "$repo/shared.txt"; then
    echo "  FAIL [driver-keeps-ours]: the non-attributed shared.txt lost the feature-side three-way merge"
    fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-merge-attrs.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-merge-attrs.test.sh: clean (forward-missing + missing-file findings + real two-branch keep-ours driver resolution, 3 cases)"
exit 0
