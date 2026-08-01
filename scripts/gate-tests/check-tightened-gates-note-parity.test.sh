#!/usr/bin/env bash
# Behavioral test of scripts/check-tightened-gates-note-parity.sh — the arms the
# one good/bad pair cannot hold. The pair proves the equal state and a note
# violating the set equality in both directions at once; this isolates each
# direction so neither can pass on the other's finding, exercises the arming
# predicate against real tags in scratch repositories, and covers every refusal.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GATE="$ROOT/scripts/check-tightened-gates-note-parity.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# $1=dir  $2=version  $3=tightened-gates section body
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

None.

## Behavior changes

None.
EOF
}

# $1=dir  $2=surface body (empty => header only)
write_surface() {
    mkdir -p "$1"
    printf '# contract: gate-sdk/SPEC.md §upgrade-smoke — the accumulating tightened-gates declaration.\n' >"$1/tightened-gates.txt"
    [[ -n "$2" ]] && printf '%s\n' "$2" >>"$1/tightened-gates.txt"
    return 0
}

# $1=label $2=dir $3=want-rc $4=want-substring
check_case() {
    local out rc
    out="$(cd "$2" && "$GATE" posts tightened-gates.txt 2>&1)"; rc=$?
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
write_surface "$a" 'check-alpha
check-beta'
check_case "dropped-name-direction" "$a" 1 "on the surface, missing from the note"

# B — a name in the note and missing from the surface: declares a gate that never
# tightened. Containment in one direction only would miss this entirely.
b="$tmp/added"
write_note "$b" "99.1.0" '- `check-alpha` — landed new.
- `check-gamma` — never tightened.'
write_surface "$b" 'check-alpha'
check_case "added-name-direction" "$b" 1 "in the note, missing from the surface"

# C — equal sets: clean.
c="$tmp/equal"
write_note "$c" "99.1.0" '- `check-alpha` — landed new.'
write_surface "$c" 'check-alpha'
check_case "equal-sets-clean" "$c" 0 "TIGHTENED-GATES-NOTE-PARITY: clean"

# D — both sides empty: an explicit None note against a drained surface is equal,
# not a refusal. This is the state a no-tightening release composes in.
d="$tmp/bothempty"
write_note "$d" "99.1.0" 'None.'
write_surface "$d" ''
check_case "both-empty-clean" "$d" 0 "TIGHTENED-GATES-NOTE-PARITY: clean"

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

# E — every note tagged: dormant, and it says so. Comparing anyway would red on
# every clone forever, since the surface is drained by contract at the tag.
e="$tmp/dormant"
mkgit "$e"
write_note "$e" "99.1.0" '- `check-alpha` — landed new.'
write_surface "$e" ''
git -C "$e" tag -a v99.1.0 -m v99.1.0
check_case "all-tagged-dormant" "$e" 0 "TIGHTENED-GATES-NOTE-PARITY: dormant"

# F — the same tree before the tag: armed, and the disagreement reds. This is the
# live window the whole design turns on, proven against real tag resolution.
f="$tmp/armed"
mkgit "$f"
write_note "$f" "99.1.0" '- `check-alpha` — landed new.'
write_surface "$f" ''
check_case "untagged-arms" "$f" 1 "in the note, missing from the surface"

# G — more than one untagged note: a state the choreography does not admit.
g="$tmp/twoflight"
write_note "$g" "99.1.0" 'None.'
write_note "$g" "99.2.0" 'None.'
write_surface "$g" ''
check_case "two-untagged-refuses" "$g" 2 "more than one untagged release note"

# H — a post with no release: key is not a note and must not be counted as one.
# The announcement post is exactly this, and counting it would trip G forever.
h="$tmp/nonnote"
write_note "$h" "99.1.0" 'None.'
write_surface "$h" ''
printf '# An announcement post, no front matter, not a release note.\n' >"$h/posts/announcement.md"
check_case "non-note-post-ignored" "$h" 0 "TIGHTENED-GATES-NOTE-PARITY: clean"

# --- refusals: exit 2, never a pass ------------------------------------------
i="$tmp/noheader"
write_note "$i" "99.1.0" 'None.'
mkdir -p "$i"
printf 'check-alpha\n' >"$i/tightened-gates.txt"
check_case "surface-missing-header" "$i" 2 "missing its required header line"

j="$tmp/nosection"
mkdir -p "$j/posts"
cat >"$j/posts/note.md" <<'EOF'
---
release: v99.1.0
---

# Fixture

## Renamed knobs

None.
EOF
write_surface "$j" ''
check_case "note-missing-section" "$j" 2 "has no 'Tightened gates' section"

k="$tmp/unparseable"
write_note "$k" "99.1.0" '- **check-alpha** — bolded, which is not the canonical spelling.'
write_surface "$k" 'check-alpha'
check_case "note-section-unparseable" "$k" 2 "does not parse"

if [[ "$fails" -gt 0 ]]; then
    echo "check-tightened-gates-note-parity.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-tightened-gates-note-parity.test: ok (each direction of the set inequality reds in isolation; equal and both-empty sets pass; the arming predicate resolves real tags, going dormant once tagged and arming before; two untagged notes refuse and a post without a release: key is not counted; a headerless surface, an absent section, and an unparseable section each fail closed)"
exit 0
