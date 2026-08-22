#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §compare-settings-allow — the breadth question is guard_allow_match with its arguments swapped, so a local glob that auto-allows a configured probe is reported with that probe as its witness; an empty probe set omits the section entirely rather than printing a clean line, which is what keeps a consumer that declared no vocabulary from reading silence as coverage
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT" || exit 2
CMP="guard-kit/bin/compare-settings-allow.sh"
[[ -x "$CMP" ]] || { echo "compare-settings-allow.test: tool not found: $CMP"; exit 2; }
command -v jq >/dev/null 2>&1 || { echo "compare-settings-allow.test: jq not found on PATH"; exit 2; }

sb="$(mktemp -d)"
trap 'rm -rf "$sb"' EXIT
mkdir -p "$sb/.claude"
printf '%s\n' '{ "permissions": { "allow": ["Bash(git status)"] } }' \
    > "$sb/.claude/settings.json"

fails=0
assert_has()    { grep -qF -- "$2" <<<"$3" || { echo "FAIL [$1]: expected present: $2"; fails=$((fails + 1)); }; }
assert_absent() { grep -qF -- "$2" <<<"$3" && { echo "FAIL [$1]: expected absent: $2"; fails=$((fails + 1)); }; return 0; }

# The consumer config is a sandbox file so this repo's own probe array cannot leak in.
run() {
    local overlay="$1" cfg="$2"; shift 2
    GUARD_KIT_CONFIG_FILE="$cfg" \
    GUARD_KIT_SETTINGS="$sb/.claude/settings.json" \
    GUARD_KIT_SETTINGS_LOCAL="$overlay" bash "$CMP" "$@"
}

printf '%s\n' '{ "permissions": { "allow": ["Bash(git *)"] } }' > "$sb/broad.json"
printf '%s\n' '{ "permissions": { "allow": ["Bash(git fetch --dry-run)"] } }' > "$sb/narrow.json"

printf '%s\n' 'GUARD_KIT_BREADTH_PROBES=("Bash(git reset --hard)")' > "$sb/probes.sh"
printf '%s\n' '# no probes declared' > "$sb/empty.sh"

# The declaration is keyed on the exact local allow-rule string; 'off-by-one.sh'
# differs from the overlay entry by one character and must not silence it.
two_probes='GUARD_KIT_BREADTH_PROBES=("Bash(git reset --hard)" "Bash(gh repo delete)")'
{
    printf '%s\n' "$two_probes"
    printf '%s\n' 'declare -A GUARD_KIT_BREADTH_DECLARED=(["Bash(git *)"]="sandbox repo, every git write is disposable")'
} > "$sb/declared.sh"
{
    printf '%s\n' "$two_probes"
    printf '%s\n' 'declare -A GUARD_KIT_BREADTH_DECLARED=(["Bash(git*)"]="one character off the overlay entry")'
} > "$sb/off-by-one.sh"

printf '%s\n' '{ "permissions": { "allow": ["Bash(git *)", "Bash(gh *)"] } }' > "$sb/mixed.json"

# Firing probe: the blanket glob auto-allows the destructive probe, reported with its witness.
firing="$(run "$sb/broad.json" "$sb/probes.sh")"
assert_has firing-section  'settings allowlist breadth' "$firing"
assert_has firing-glob     'Bash(git *)' "$firing"
assert_has firing-witness  'Bash(git reset --hard)' "$firing"

# Non-firing probe: a narrow local entry auto-allows nothing in the probe set.
nonfiring="$(run "$sb/narrow.json" "$sb/probes.sh")"
assert_has nonfiring-section 'settings allowlist breadth' "$nonfiring"
assert_has nonfiring-clean   'no over-broad local entries' "$nonfiring"

# Empty knob: the section is omitted entirely, not printed clean.
silent="$(run "$sb/broad.json" "$sb/empty.sh")"
assert_absent empty-knob-silent 'settings allowlist breadth' "$silent"
assert_has    empty-knob-redundancy 'settings allowlist redundancy' "$silent"

# --count carries both counts: redundancy first, breadth second.
c_firing="$(run "$sb/broad.json" "$sb/probes.sh" --count)"
[[ "$c_firing" == "0 1" ]] || { echo "FAIL [count-firing]: expected '0 1', got '$c_firing'"; fails=$((fails + 1)); }
c_empty="$(run "$sb/broad.json" "$sb/empty.sh" --count)"
[[ "$c_empty" == "0 0" ]] || { echo "FAIL [count-empty]: expected '0 0', got '$c_empty'"; fails=$((fails + 1)); }

# No overlay at all: the pre-existing early path keeps the same two-field count shape.
c_absent="$(run "$sb/does-not-exist.json" "$sb/probes.sh" --count)"
[[ "$c_absent" == "0 0" ]] || { echo "FAIL [count-absent]: expected '0 0', got '$c_absent'"; fails=$((fails + 1)); }

# Empty declaration map (every case above): the declared subsection is absent too.
assert_absent empty-declared-silent 'advisory — declared intended' "$firing"

# An over-broad set that is entirely declared: the declared section prints with its
# reason, and no narrowing section and no false clean line appear.
all_declared="$(run "$sb/broad.json" "$sb/declared.sh")"
assert_has    all-declared-section  'advisory — declared intended' "$all_declared"
assert_has    all-declared-reason   'sandbox repo, every git write is disposable' "$all_declared"
assert_absent all-declared-narrow   'advisory — narrowing candidates' "$all_declared"
assert_absent all-declared-clean    'no over-broad local entries' "$all_declared"

# A mixed set: the declared entry leaves the narrowing set, the undeclared one stays.
mixed="$(run "$sb/mixed.json" "$sb/declared.sh")"
assert_has mixed-narrowing 'advisory — narrowing candidates' "$mixed"
assert_has mixed-undeclared 'Bash(gh *)  ⊇  Bash(gh repo delete)' "$mixed"
assert_has mixed-declared  'Bash(git *)  ⊇  Bash(git reset --hard)  — sandbox repo' "$mixed"
# The narrowing lane prints the pair bare; only the declared lane appends the reason,
# so an exact whole-line match on the bare pair is the partition assertion.
grep -qxF -- '  Bash(git *)  ⊇  Bash(git reset --hard)' <<<"$mixed" \
    && { echo "FAIL [mixed-partition]: the declared glob is still in the narrowing set"; fails=$((fails + 1)); }

# --count: the breadth number counts narrowing candidates, so the declared entry is excluded.
c_mixed="$(run "$sb/mixed.json" "$sb/declared.sh" --count)"
[[ "$c_mixed" == "0 1" ]] || { echo "FAIL [count-declared]: expected '0 1', got '$c_mixed'"; fails=$((fails + 1)); }

# Exactness: a declaration one character off the overlay entry silences nothing.
off="$(run "$sb/broad.json" "$sb/off-by-one.sh")"
assert_has    off-by-one-narrowing 'advisory — narrowing candidates' "$off"
assert_has    off-by-one-entry     'Bash(git *)  ⊇  Bash(git reset --hard)' "$off"
assert_absent off-by-one-declared  'advisory — declared intended' "$off"

[[ "$fails" -eq 0 ]] || { echo "compare-settings-allow.test: $fails assertion(s) failed"; exit 1; }
echo "compare-settings-allow.test: clean (a firing probe names its witnessing glob, a non-firing probe reports clean, an empty probe set omits the section, a declared breadth moves out of the narrowing set and out of --count, and an exactness miss silences nothing)"
exit 0
