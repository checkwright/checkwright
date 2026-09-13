#!/usr/bin/env bash
# Behavioral test of check-release-declaration-parity — the arms the
# one good/bad pair cannot hold. The pair proves the equal state and a note
# disagreeing in every section at once; this isolates each direction per section
# so neither can pass on another's finding, exercises the arming predicate
# against real tags in scratch repositories, and covers every refusal.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GATES_DIR="$ROOT/scripts"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# $1=dir  $2=version  $3=tightened-gates body  $4=renamed-knobs body  $5=behavior-changes body
write_note() {
    mkdir -p "$1/posts"
    cat >"$1/posts/note-$2.md" <<EOF
---
release: v$2
---

# Fixture v$2

## In brief

- Fixture note.

## Tightened gates

$3

## Renamed knobs

${4:-None.}

## Behavior changes

${5:-None.}
EOF
}

# $1=dir  $2=surface body below the header (empty => header only, the drained surface)
write_surface() {
    mkdir -p "$1"
    printf '# contract: gate-sdk/SPEC.md §upgrade-smoke — the accumulating release declaration surface.\n' >"$1/release-declarations.md"
    [[ -n "$2" ]] && printf '\n%s\n' "$2" >>"$1/release-declarations.md"
    return 0
}

# $1=label $2=dir $3=want-rc $4=want-substring
check_case() {
    local out rc
    out="$(cd "$2" && gate_run check-release-declaration-parity "$GATES_DIR" posts release-declarations.md 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$4" ]] && ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$4': $out"; fails=$((fails + 1))
    fi
}

# A — a name on the surface and missing from the note: the gate tightened and is
# shipping undeclared, which licenses a red the upgrade smoke waves through.
a="$tmp/dropped"
write_note "$a" "99.1.0" '- `check-alpha` — landed new.'
write_surface "$a" '## Tightened gates

- `check-alpha`
- `check-beta`'
check_case "dropped-name-direction" "$a" 1 "Tightened gates: on the surface, missing from the note"

# B — a name in the note and missing from the surface: declares a gate that never
# tightened. Containment in one direction only would miss this entirely.
b="$tmp/added"
write_note "$b" "99.1.0" '- `check-alpha` — landed new.
- `check-gamma` — never tightened.'
write_surface "$b" '## Tightened gates

- `check-alpha`'
check_case "added-name-direction" "$b" 1 "Tightened gates: in the note, missing from the surface"

# C — equal sets in every section, the surface's sections in a different order
# from the note's: clean, since order carries no meaning.
c="$tmp/equal"
write_note "$c" "99.1.0" '- `check-alpha` — landed new.' '- `KIT_OLD` → `KIT_NEW` — renamed.' '- **`kit/bin/tool.sh`** — refuses.'
write_surface "$c" '## Behavior changes

- **`kit/bin/tool.sh`** — refuses a bare pair.

## Renamed knobs

- `KIT_OLD` → `KIT_NEW`

## Tightened gates

- `check-alpha`'
check_case "equal-sets-clean" "$c" 0 "RELEASE-DECLARATION-PARITY: clean"

# D — both sides empty: explicit None notes against a drained, header-only surface
# are equal, not a refusal. This is the state a release declaring nothing composes in.
d="$tmp/bothempty"
write_note "$d" "99.1.0" 'None.'
write_surface "$d" ''
check_case "both-empty-clean" "$d" 0 "RELEASE-DECLARATION-PARITY: clean"

# E — a rename on the surface the note dropped: a declared change lost at composition.
e="$tmp/knobdropped"
write_note "$e" "99.1.0" 'None.'
write_surface "$e" '## Renamed knobs

- `KIT_OLD` → `KIT_NEW`'
check_case "knob-dropped-direction" "$e" 1 "Renamed knobs: on the surface, missing from the note"

# F — a behavior change in the note the surface never held: close reconstructed it
# instead of the landing session declaring it.
f="$tmp/behavioradded"
write_note "$f" "99.1.0" 'None.' '' '- **`kit/bin/tool.sh`** — refuses.'
write_surface "$f" '## Behavior changes'
check_case "behavior-added-direction" "$f" 1 "Behavior changes: in the note, missing from the surface"

# --- the arming predicate, against real tags ---------------------------------
mkgit() {
    mkdir -p "$1"
    git -C "$1" init -q 2>/dev/null
    git -C "$1" config user.email t@example.invalid
    git -C "$1" config user.name t
    : >"$1/seed"
    git -C "$1" add seed
    git -C "$1" commit -qm seed
}

# G — every note tagged: dormant, and it says so. Comparing anyway would red on
# every clone forever, since the surface is drained by contract at the tag.
g="$tmp/dormant"
mkgit "$g"
write_note "$g" "99.1.0" '- `check-alpha` — landed new.'
write_surface "$g" ''
git -C "$g" tag -a v99.1.0 -m v99.1.0
check_case "all-tagged-dormant" "$g" 0 "RELEASE-DECLARATION-PARITY: dormant"

# H — the same tree before the tag: armed, and the disagreement reds. This is the
# live window the whole design turns on, proven against real tag resolution.
h="$tmp/armed"
mkgit "$h"
write_note "$h" "99.1.0" '- `check-alpha` — landed new.'
write_surface "$h" ''
check_case "untagged-arms" "$h" 1 "in the note, missing from the surface"

# I — more than one untagged note: a state the choreography does not admit.
i="$tmp/twoflight"
write_note "$i" "99.1.0" 'None.'
write_note "$i" "99.2.0" 'None.'
write_surface "$i" ''
check_case "two-untagged-refuses" "$i" 2 "more than one untagged release note"

# J — a post with no release: key is not a note and must not be counted as one.
# The announcement post is exactly this, and counting it would trip I forever.
j="$tmp/nonnote"
write_note "$j" "99.1.0" 'None.'
write_surface "$j" ''
printf '# An announcement post, no front matter, not a release note.\n' >"$j/posts/announcement.md"
check_case "non-note-post-ignored" "$j" 0 "RELEASE-DECLARATION-PARITY: clean"

# --- refusals: exit 2, never a pass ------------------------------------------
k="$tmp/noheader"
write_note "$k" "99.1.0" 'None.'
printf '## Tightened gates\n\n- `check-alpha`\n' >"$k/release-declarations.md"
check_case "surface-missing-header" "$k" 2 "missing its required header line"

l="$tmp/nosection"
mkdir -p "$l/posts"
cat >"$l/posts/note.md" <<'EOF'
---
release: v99.1.0
---

# Fixture

## Tightened gates

None.

## Behavior changes

None.
EOF
write_surface "$l" ''
check_case "note-missing-section" "$l" 2 "has no 'Renamed knobs' section"

m="$tmp/unparseable"
write_note "$m" "99.1.0" '- **check-alpha** — bolded, which is not the canonical spelling.'
write_surface "$m" '## Tightened gates

- `check-alpha`'
check_case "note-section-unparseable" "$m" 2 "does not parse"

n="$tmp/surfaceunparseable"
write_note "$n" "99.1.0" 'None.'
write_surface "$n" '## Behavior changes

- kit/bin/tool.sh — no bolded lead, so no token.'
check_case "surface-section-unparseable" "$n" 2 "carries unreadable bullet(s)"

if [[ "$fails" -gt 0 ]]; then
    echo "check-release-declaration-parity.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-release-declaration-parity.test: ok (each direction of the set inequality reds in isolation, in Tightened gates and in each prose section; equal sets in any section order and both-empty sets pass; the arming predicate resolves real tags, going dormant once tagged and arming before; two untagged notes refuse and a post without a release: key is not counted; a headerless surface, an absent note section, and an unparseable note or surface section each fail closed)"
exit 0
