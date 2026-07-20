#!/usr/bin/env bash
# Behavioral test of scripts/check-release-bump.sh — the Behavior-changes leg the
# one good/bad pair cannot hold. The pair covers presence + the tightened-gates
# floor; this covers the third fixed section: its fail-closed presence assertion,
# its minor-bump floor when non-empty, and the clean cases.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GATE="$ROOT/scripts/check-release-bump.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# $1=dir  $2=version  $3=tightened-body  $4=behavior-body
write_note() {
    mkdir -p "$1"
    cat >"$1/note-$2.md" <<EOF
---
release: v$2
---

# Fixture v$2

## Tightened gates

$3

## Renamed knobs

None.

## Behavior changes

$4
EOF
}

# $1=label $2=posts-dir $3=want-rc $4=want-substring
check_case() {
    local out rc
    out="$(cd "$2" && "$GATE" posts 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$4" ]] && ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$4': $out"; fails=$((fails + 1))
    fi
}

# A — newest note missing the Behavior changes section: fail-closed (exit 2).
a="$tmp/absent/posts"
write_note "$a" "0.1.0" "None." "None."
mkdir -p "$tmp/absent/posts"
cat >"$tmp/absent/posts/note-0.2.0.md" <<'EOF'
---
release: v0.2.0
---

# Fixture v0.2.0

## Tightened gates

None.

## Renamed knobs

None.
EOF
check_case "absent-behavior-section" "$tmp/absent" 2 "no 'Behavior changes' section"

# B — non-empty Behavior changes on a patch-only bump: the new floor reds (exit 1).
b="$tmp/patch/posts"
write_note "$b" "0.1.0" "None." "None."
write_note "$b" "0.1.1" "None." "- **bin/run-validate.sh** now fails closed on an unbaselined failure."
check_case "behavior-floor-patch-red" "$tmp/patch" 1 "behavior-change bullet(s)"

# C — non-empty Behavior changes on a minor bump: within the floor, clean.
c="$tmp/minor/posts"
write_note "$c" "0.1.0" "None." "None."
write_note "$c" "0.2.0" "None." "- **bin/run-validate.sh** now fails closed on an unbaselined failure."
check_case "behavior-floor-minor-clean" "$tmp/minor" 0 "RELEASE-BUMP: clean"

# D — all three sections None on a patch bump: floor-neutral, clean.
d="$tmp/allnone/posts"
write_note "$d" "0.1.0" "None." "None."
write_note "$d" "0.1.1" "None." "None."
check_case "all-none-patch-clean" "$tmp/allnone" 0 "RELEASE-BUMP: clean"

# $1=label $2=root $3=disposition-body $4=want-rc $5=want-substring
check_deferral() {
    local out rc
    printf '%s' "$3" > "$2/disposition.txt"
    out="$(cd "$2" && "$GATE" posts disposition.txt 2>&1)"; rc=$?
    if [[ "$rc" -ne "$4" ]]; then
        echo "  FAIL [$1]: want exit $4, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$5" ]] && ! grep -qF -- "$5" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$5': $out"; fails=$((fails + 1))
    fi
}

# E — an outstanding deferral floors an otherwise floor-neutral patch bump to minor.
e="$tmp/deferred"
write_note "$e/posts" "0.1.0" "None." "None."
write_note "$e/posts" "0.1.1" "None." "None."
check_deferral "outstanding-deferral-patch-red" "$e" \
    'alpha release deferred:v0.2.0 — held back
' 1 "outstanding deferred release (v0.2.0)"

# F — a later line releasing at or above the deferred version discharges it: clean.
check_deferral "discharged-deferral-patch-clean" "$e" \
    'alpha release deferred:v0.2.0 — held back
beta release v0.2.0 — shipped
' 0 "RELEASE-BUMP: clean"

# G — an outstanding deferral is no red on a minor bump, which already clears its floor.
g="$tmp/deferred-minor"
write_note "$g/posts" "0.1.0" "None." "None."
write_note "$g/posts" "0.2.0" "None." "None."
check_deferral "outstanding-deferral-minor-clean" "$g" \
    'alpha release deferred:v0.2.0 — held back
' 0 "inheriting outstanding deferral v0.2.0"

# H — the deferred floor derives ahead of the two-note early return, so a single-note
# tree carrying an outstanding deferral reds rather than exiting clean for lack of a
# predecessor — the case a fresh consumer meets on note one.
h="$tmp/deferred-single"
write_note "$h/posts" "0.1.0" "None." "None."
check_deferral "single-note-deferral-red" "$h" \
    'alpha release deferred:v0.2.0 — held back
' 1 "single-note tree cannot ride it out"

if [[ "$fails" -gt 0 ]]; then
    echo "check-release-bump.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-release-bump.test: ok (absent Behavior-changes section fails closed; the non-empty section floors a patch red and passes a minor; all-None patch stays clean; an outstanding deferral floors a patch red and a single-note tree too, discharges on a later release line, and passes a minor)"
exit 0
