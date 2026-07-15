#!/usr/bin/env bash
# Behavioral test of the section-resolution paths the good/bad pair cannot hold:
# the pair fixes CONTEXT_KIT_BREVITY_SECTION at the stock default and always
# supplies a file carrying it, so neither case can express a knob that resolves
# to nothing. An unmatched section is exit 2 (a gate whose target vanished is a
# broken machine, not a clean tree), which run-gate-tests reads as a harness
# error from a bad/ tree — so it lives here.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # context-kit/
GATE="$DIR/checks/check-brevity.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

cat >"$SANDBOX/renamed.md" <<'EOF'
# governed file

## Conventions we since renamed

- **Over budget with pointer:** this bullet runs past the four-line budget
  across several lines of prose while admitting its detail already lives
  in the HANDBOOK §Some section that it cites, so a gate that resolved this
  section would flag it — the fix being to trim the bullet and lean on the
  pointer it already carries.
EOF

cat >"$SANDBOX/empty.md" <<'EOF'
# governed file

## Shared conventions

Prose, but not one `- **name:**` bullet.

## Next section
EOF

cat >"$SANDBOX/cfg.sh" <<'EOF'
CONTEXT_KIT_BREVITY_SECTION="## Conventions we since renamed"
EOF

# The stock-default cases pin an existing empty config (the strict loader
# exits 2 on a set-but-missing path, and /dev/null is not a regular file).
: >"$SANDBOX/noop-cfg.sh"

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=file  $5..=env assignments
    local label="$1" want="$2" sub="$3" file="$4"; shift 4
    local out rc
    out="$(env "$@" "$GATE" "$SANDBOX/$file" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# The regression: the stock section name matches no heading in a file that
# renamed it, and the over-budget bullet below it goes unseen. Before the fix
# this exited 0 reporting "0 bullets" — a disarmed gate reading as a clean tree.
check_case "renamed-section-fails-closed" 2 "no heading matches" renamed.md \
    CONTEXT_KIT_CONFIG_FILE="$SANDBOX/noop-cfg.sh"

# Symmetrically, a consumer config naming a heading the governed file lacks is
# the same broken machine, reached through the config seam rather than the default.
check_case "config-names-absent-heading" 2 "no heading matches" empty.md \
    CONTEXT_KIT_CONFIG_FILE="$SANDBOX/cfg.sh"

# Resolution is what fails closed, not emptiness: a section that exists and
# holds no bullets is a clean tree, and the help line stays out of the way.
check_case "matched-but-bulletless-clean" 0 "BREVITY: clean (0 bullets" empty.md \
    CONTEXT_KIT_CONFIG_FILE="$SANDBOX/noop-cfg.sh"

# The renamed file is clean once the knob is repointed at the live heading —
# and the bullet it was hiding is now seen, so the knob really did resolve.
check_case "repointed-knob-sees-bullets" 1 "Over budget with pointer" renamed.md \
    CONTEXT_KIT_CONFIG_FILE="$SANDBOX/cfg.sh"

if [[ "$fails" -gt 0 ]]; then
    echo "check-brevity.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-brevity.test.sh: clean (unmatched default + unmatched config exit 2, bulletless match clean, repointed knob sees the hidden bullet, 4 cases)"
exit 0
