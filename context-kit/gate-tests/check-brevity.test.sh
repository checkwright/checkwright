#!/usr/bin/env bash
# Behavioral test of the section-set resolution paths the good/bad pair cannot hold:
# the pair fixes CONTEXT_KIT_BREVITY_SECTIONS at the stock one-element default and
# always supplies a file carrying it, so neither case can express an element that
# resolves to nothing, an empty set, a repeated element, a multi-section set or the
# retired scalar knob. Each refusal is exit 2 (a gate whose target vanished is a
# broken machine, not a clean tree), which run-gate-tests reads as a harness error
# from a bad/ tree — so they live here.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # context-kit/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

cat >"$SANDBOX/renamed.md" <<'MD'
# governed file

## Conventions we since renamed

- **Over budget with pointer:** this bullet runs past the four-line budget
  across several lines of prose while admitting its detail already lives
  in the HANDBOOK §Some section that it cites, so a gate that resolved this
  section would flag it — the fix being to trim the bullet and lean on the
  pointer it already carries.
MD

cat >"$SANDBOX/empty.md" <<'MD'
# governed file

## Shared conventions

Prose, but not one list item.

## Next section
MD

cat >"$SANDBOX/two.md" <<'MD'
# governed file

## Shared conventions

- **Terse:** one line.

## Housekeeping

- `scratch/` is a plain bullet with no bold name that runs past the
  four-line budget across several lines of prose and cites the
  HANDBOOK §Some section, so only a gate reading the second governed
  section and every top-level item, not only bold-named ones, sees
  it at all.
MD

cat >"$SANDBOX/cfg.sh" <<'CFG'
CONTEXT_KIT_BREVITY_SECTIONS=("## Conventions we since renamed")
CFG

cat >"$SANDBOX/two-cfg.sh" <<'CFG'
CONTEXT_KIT_BREVITY_SECTIONS=("## Shared conventions" "## Housekeeping")
CFG

cat >"$SANDBOX/half-cfg.sh" <<'CFG'
CONTEXT_KIT_BREVITY_SECTIONS=("## Shared conventions" "## Housekeeping we since renamed")
CFG

cat >"$SANDBOX/dup-cfg.sh" <<'CFG'
CONTEXT_KIT_BREVITY_SECTIONS=("## Shared conventions" "## Shared conventions")
CFG

cat >"$SANDBOX/empty-cfg.sh" <<'CFG'
CONTEXT_KIT_BREVITY_SECTIONS=()
CFG

cat >"$SANDBOX/retired-cfg.sh" <<'CFG'
CONTEXT_KIT_BREVITY_SECTION="## Shared conventions"
CFG

# The stock-default cases pin an existing empty config (the strict loader
# exits 2 on a set-but-missing path, and /dev/null is not a regular file).
: >"$SANDBOX/noop-cfg.sh"

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=file  $5..=env assignments
    local label="$1" want="$2" sub="$3" file="$4"; shift 4
    local out rc
    # spec: gate-sdk/SPEC.md §lib/gate.sh — the overrides are exported rather than passed through
    # `env`, because the config bridge resolves this member's knobs before the argv it builds runs
    out="$(export "$@"; gate_run check-brevity "$DIR/checks" "$SANDBOX/$file" 2>&1)"; rc=$?
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

# A multi-section set reaches its second element, and the non-bold bullet there
# is measured and named by section, line and opening text.
check_case "second-section-non-bold-bullet" 1 "'## Housekeeping' line 9: \`scratch/\` is a plain bullet" two.md \
    CONTEXT_KIT_CONFIG_FILE="$SANDBOX/two-cfg.sh"

# One unmatched element refuses the whole set, naming that element — the resolved
# sibling must not report the rest clean.
check_case "one-unmatched-element-fails-closed" 2 "'## Housekeeping we since renamed'" two.md \
    CONTEXT_KIT_CONFIG_FILE="$SANDBOX/half-cfg.sh"

# A repeated element is scanned once: one bullet, not two.
check_case "repeated-element-scanned-once" 0 "BREVITY: clean (1 bullets, 1 within budget; '## Shared conventions' 1)" two.md \
    CONTEXT_KIT_CONFIG_FILE="$SANDBOX/dup-cfg.sh"

# An empty set governs nothing, which is the vacuous pass the fail-closed contract refuses.
check_case "empty-set-fails-closed" 2 "CONTEXT_KIT_BREVITY_SECTIONS is empty" two.md \
    CONTEXT_KIT_CONFIG_FILE="$SANDBOX/empty-cfg.sh"

# The retired scalar knob refuses rather than being silently ignored, naming its replacement.
check_case "retired-scalar-knob-refused" 2 "set CONTEXT_KIT_BREVITY_SECTIONS" two.md \
    CONTEXT_KIT_CONFIG_FILE="$SANDBOX/retired-cfg.sh"

if [[ "$fails" -gt 0 ]]; then
    echo "check-brevity.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-brevity.test.sh: clean (unmatched default + unmatched config + one unmatched element + empty set + retired knob exit 2, bulletless match clean, repointed knob sees the hidden bullet, second section's non-bold bullet seen, repeated element scanned once, 9 cases)"
exit 0
