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

if [[ "$fails" -gt 0 ]]; then
    echo "check-release-bump.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-release-bump.test: ok (absent Behavior-changes section fails closed; the non-empty section floors a patch red and passes a minor; all-None patch stays clean, 4 cases)"
exit 0
