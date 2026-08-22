#!/usr/bin/env bash
# Behavioral test of the arms the one-pair good/bad harness cannot spell: the
# three fail-closed exits and the empty-corpus clean. A pair asserts one exit
# code per case dir, so exit 2 has no pair spelling at all, and the
# no-amendment clean needs a case dir with no amendment in it — which the pair's
# own good/ cannot also be.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # canon-kit/
SANDBOX="$(mktemp -d)"
trap 'chmod -R u+rwX "$SANDBOX" 2>/dev/null; rm -rf "$SANDBOX"' EXIT

fails=0

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=scan root
    local label="$1" want="$2" sub="$3" root="$4"
    local out rc
    out="$(cd "$SANDBOX" && gate_run check-amendment-update-target "$DIR/checks" "$root" 2>&1)"
    rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# An empty corpus is clean, and it is the state a fresh consumer installs into.
# It is *only* safe to call clean because the finder below refuses rather than
# returning empty when the walk fails.
mkdir -p "$SANDBOX/empty"
check_case "no-amendment-is-clean" 0 "0 amendment(s)" "$SANDBOX/empty"

# A scan root that is not a directory at all.
mkdir -p "$SANDBOX/notdir"
: >"$SANDBOX/notdir/file"
check_case "non-directory-root-fails-closed" 2 "not a directory" "$SANDBOX/notdir/file"

# An amendment listing update targets with no delta section: every entry is
# unowned and the gate cannot say which arm to blame, so it refuses.
mkdir -p "$SANDBOX/nowhat"
cat >"$SANDBOX/nowhat/SPEC-x.md" <<'EOF'
# SPEC amendment: x

## Existing sections updated

- **component/SPEC.md** — an update target in an amendment with no deltas.
EOF
check_case "updated-without-what-changes-fails-closed" 2 "but no '## What changes'" "$SANDBOX/nowhat"

# The two permission-shaped refusals. A tree where the sandbox owner can read
# what it just made unreadable proves nothing, so each is probed before it is
# asserted rather than assumed to hold for whoever runs this.
mkdir -p "$SANDBOX/unread"
printf '%s\n' '# SPEC amendment: y' >"$SANDBOX/unread/SPEC-y.md"
chmod 000 "$SANDBOX/unread/SPEC-y.md"
if cat "$SANDBOX/unread/SPEC-y.md" >/dev/null 2>&1; then
    echo "  SKIP [unreadable-amendment-fails-closed]: this user reads a 0000 file"
else
    check_case "unreadable-amendment-fails-closed" 2 "cannot read" "$SANDBOX/unread"
fi

mkdir -p "$SANDBOX/unwalk/sub"
chmod 000 "$SANDBOX/unwalk/sub"
if ls "$SANDBOX/unwalk/sub" >/dev/null 2>&1; then
    echo "  SKIP [unwalkable-root-fails-closed]: this user walks a 0000 directory"
else
    # The divergence from check-amendment-queue's best-effort finder, executable:
    # an empty amendment set hides every violation here, so the walk refuses.
    check_case "unwalkable-root-fails-closed" 2 "cannot read directory" "$SANDBOX/unwalk"
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-amendment-update-target.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-amendment-update-target.test.sh: clean (an empty corpus is clean; a bad root, an unreadable amendment and an update-target section with no deltas each refuse)"
exit 0
