#!/usr/bin/env bash
# Behavioral test of check-release-bump — the arms the one good/bad pair cannot
# hold. The pair covers the gates-section floor; this covers roster presence on a
# note under composition (each missing section fails closed), the summary table,
# a prose section's minor-bump floor and the clean cases, the alias count on a
# tagged note, the deferral floor, the composition predicate, the roster knob's
# refusal, and the version comparator's refusal, which no `bad/` case can pin
# because a case dir's rejection arm is exit 1 and a refusal is exit 2
# (gate-sdk/SPEC.md §The declaration cohort).
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GATES_DIR="$ROOT/scripts"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# $1=dir  $2=version  $3=gates-body  $4=behavior-body — the six roster sections, the
# summary table counting each body's bullets
write_note() {
    local g b
    mkdir -p "$1"
    g="$(grep -c '^- ' <<<"$3")"
    b="$(grep -c '^- ' <<<"$4")"
    cat >"$1/note-$2.md" <<EOF
---
release: v$2
---

# Fixture v$2

## In brief

| Section | Entries | Who acts |
|---|---|---|
| [New and tightened gates](#new-and-tightened-gates) | $g | everyone |
| [Knob changes](#knob-changes) | 0 | knob setters |
| [Gate-authoring changes](#gate-authoring-changes) | 0 | gate authors |
| [Platforms](#platforms) | 0 | installers |
| [Behavior changes](#behavior-changes) | $b | dependants |

- Fixture note: the six roster sections, In brief first.

## New and tightened gates

$3

## Knob changes

None.

## Gate-authoring changes

None.

## Platforms

None.

## Behavior changes

$4
EOF
}

# $1=file  $2=literal  $3=replacement — one composed note, edited into a case's defect
edit_note() {
    local text
    text="$(<"$1")"
    printf '%s\n' "${text/"$2"/"$3"}" >"$1"
}

# $1=label $2=posts-dir $3=want-rc $4=want-substring
check_case() {
    local out rc
    out="$(cd "$2" && gate_run check-release-bump "$GATES_DIR" posts 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$4" ]] && ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$4': $out"; fails=$((fails + 1))
    fi
}

# A — a note under composition missing a roster section: fail-closed (exit 2), for
# the last section and for the derived Platforms one.
a="$tmp/absent/posts"
write_note "$a" "0.1.0" "None." "None."
write_note "$a" "0.2.0" "None." "None."
edit_note "$a/note-0.2.0.md" "## Behavior changes" "## Something else"
check_case "absent-behavior-section" "$tmp/absent" 2 "no 'Behavior changes' section"
a2="$tmp/absent-platforms/posts"
write_note "$a2" "0.1.0" "None." "None."
write_note "$a2" "0.2.0" "None." "None."
edit_note "$a2/note-0.2.0.md" "## Platforms" "## Something else"
check_case "absent-platforms-section" "$tmp/absent-platforms" 2 "no 'Platforms' section"

# A' — a former heading does not stand in for a roster heading on a note under
# composition: the alias reads history, and a note being composed takes the new name.
a3="$tmp/alias-composing/posts"
write_note "$a3" "0.1.0" "None." "None."
write_note "$a3" "0.2.0" "None." "None."
edit_note "$a3/note-0.2.0.md" "## Knob changes" "## Renamed knobs"
check_case "alias-under-composition-refused" "$tmp/alias-composing" 2 "no 'Knob changes' section"

# --- the summary table, on a note under composition ---------------------------
t1="$tmp/table-absent/posts"
write_note "$t1" "0.1.0" "None." "None."
write_note "$t1" "0.2.0" "None." "None."
edit_note "$t1/note-0.2.0.md" "| Section | Entries | Who acts |" "| Part | Entries | Who acts |"
check_case "table-absent" "$tmp/table-absent" 1 "carries no table under the header"
t2="$tmp/table-count/posts"
write_note "$t2" "0.1.0" "None." "None."
write_note "$t2" "0.2.0" "- \`check-alpha\` — landed new." "None."
edit_note "$t2/note-0.2.0.md" "(#new-and-tightened-gates) | 1 |" "(#new-and-tightened-gates) | 2 |"
check_case "table-count-wrong" "$tmp/table-count" 1 "Entries cell '2' is not the 1 bullet(s)"
check_case "table-prints-derived-rows" "$tmp/table-count" 1 "| [New and tightened gates](#new-and-tightened-gates) | 1 | <who acts> |"
t3="$tmp/table-anchor/posts"
write_note "$t3" "0.1.0" "None." "None."
write_note "$t3" "0.2.0" "None." "None."
edit_note "$t3/note-0.2.0.md" "(#knob-changes)" "(#knobs)"
check_case "table-anchor-wrong" "$tmp/table-anchor" 1 "row 2: Section cell '[Knob changes](#knobs)'"
t4="$tmp/table-rows/posts"
write_note "$t4" "0.1.0" "None." "None."
write_note "$t4" "0.2.0" "None." "None."
edit_note "$t4/note-0.2.0.md" "| [Platforms](#platforms) | 0 | installers |"$'\n' ""
check_case "table-row-missing" "$tmp/table-rows" 1 "the table holds 4 row(s) for 5 declaration-bearing section(s)"

# --- the roster knob's refusal: exit 2 where the knob is read ------------------
r="$tmp/roster/posts"
write_note "$r" "0.1.0" "None." "None."
write_note "$r" "0.2.0" "None." "None."
printf 'GATE_SDK_RELEASE_SECTIONS[] = bogus: X\n' >"$tmp/roster/gate-sdk-config.knobs"
out="$(cd "$tmp/roster" && gate_env GATE_SDK_KNOB_FILE="$tmp/roster/gate-sdk-config.knobs" && gate_run check-release-bump "$GATES_DIR" posts 2>&1)"; rc=$?
if [[ "$rc" -ne 2 ]] || ! grep -qF "unknown role 'bogus'" <<<"$out"; then
    echo "  FAIL [roster-unknown-role]: want exit 2 naming the role, got $rc -- $out"; fails=$((fails + 1))
fi

# B — non-empty Behavior changes on a patch-only bump: the new floor reds (exit 1).
b="$tmp/patch/posts"
write_note "$b" "0.1.0" "None." "None."
write_note "$b" "0.1.1" "None." "- **bin/run-validate.sh** now fails closed on an unbaselined failure."
check_case "behavior-floor-patch-red" "$tmp/patch" 1 "1 bullet(s) under 'Behavior changes'"

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
    out="$(cd "$2" && gate_run check-release-bump "$GATES_DIR" posts disposition.txt 2>&1)"; rc=$?
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

# I' — a deferred floor above what a non-patch bump reaches: the gap
# release-bump-deferred-floor-unenforced named. patch_only is false (nmin
# differs), so the phase-B block above never fires, and the numeric floor is
# the only thing that can catch a deferred major discharged as a minor.
l="$tmp/deferred-minor-under-floor"
write_note "$l/posts" "0.1.0" "None." "None."
write_note "$l/posts" "0.2.0" "None." "None."
check_deferral "outstanding-deferral-minor-under-floor-red" "$l" \
    'alpha release deferred:v0.3.0 — held back
' 1 "falls below an outstanding deferred release (v0.3.0)"

# J' — the same higher deferral, but the bump clears it numerically: clean.
m="$tmp/deferred-minor-clears-floor"
write_note "$m/posts" "0.1.0" "None." "None."
write_note "$m/posts" "0.3.0" "None." "None."
check_deferral "outstanding-deferral-minor-clears-floor-clean" "$m" \
    'alpha release deferred:v0.3.0 — held back
' 0 "inheriting outstanding deferral v0.3.0"

# --- the roster presence assertion, and the predicate that arms it ------------
# It binds a note under composition (declared version carries no tag) and is
# dormant on a published note, which is what keeps the historical corpus free of
# retro-fabricated sections. The good/bad pair cannot hold either arm: it runs
# inside this repository, where the fixtures' versions resolve against real tags.

# I — newest note under composition and missing In brief: fail-closed (exit 2).
i="$tmp/inbrief-absent/posts"
write_note "$i" "0.1.0" "None." "None."
write_note "$i" "0.2.0" "None." "None."
edit_note "$i/note-0.2.0.md" "## In brief" "## Opener"
check_case "inbrief-absent-under-composition" "$tmp/inbrief-absent" 2 "has no 'In brief' section"

# J — the same tree with every section present: clean, and the state is reported.
j="$tmp/inbrief-present/posts"
write_note "$j" "0.1.0" "None." "None."
write_note "$j" "0.2.0" "None." "None."
check_case "inbrief-present-under-composition" "$tmp/inbrief-present" 0 "section roster and summary table asserted"

# K — the newest note's version is tagged, so it is published history: the
# assertion is dormant and says so rather than demanding a fabricated summary,
# and the roles its former headings lack count zero.
k="$tmp/inbrief-dormant"
mkdir -p "$k"
git -C "$k" init -q 2>/dev/null
git -C "$k" config user.email t@example.invalid
git -C "$k" config user.name t
: >"$k/seed"
git -C "$k" add seed
git -C "$k" commit -qm seed
git -C "$k" tag -a v0.2.0 -m v0.2.0
write_note "$k/posts" "0.1.0" "None." "None."
cat >"$k/posts/note-0.2.0.md" <<'EOF'
---
release: v0.2.0
---

# Fixture v0.2.0

## Tightened gates

None.

## Renamed knobs

None.

## Behavior changes

None.
EOF
check_case "inbrief-dormant-when-tagged" "$k" 0 "section roster and summary table dormant"

# K' — a tagged patch note declaring under the former gates heading: the alias
# counts, so its bullet still floors the bump to minor.
k2="$tmp/alias-tagged"
mkdir -p "$k2"
git -C "$k2" init -q 2>/dev/null
git -C "$k2" config user.email t@example.invalid
git -C "$k2" config user.name t
: >"$k2/seed"
git -C "$k2" add seed
git -C "$k2" commit -qm seed
git -C "$k2" tag -a v0.1.1 -m v0.1.1
write_note "$k2/posts" "0.1.0" "None." "None."
cat >"$k2/posts/note-0.1.1.md" <<'EOF'
---
release: v0.1.1
---

# Fixture v0.1.1

## Tightened gates

- `check-alpha` — tightened.

## Renamed knobs

None.

## Behavior changes

None.
EOF
check_case "alias-counts-on-tagged-note" "$k2" 1 "1 bullet(s) under 'New and tightened gates'"

# --- the version comparator's refusal ----------------------------------------
# Ordering is defined over <major>.<minor>.<patch> and a token outside it is exit
# 2 naming the token, its source and the grammar. This is the one arm the good/bad
# pair structurally cannot hold: a rejection case is exit 1 by contract. Both
# producers are covered, because a refusal that names the wrong file sends its
# reader hunting (gate-sdk/SPEC.md §The declaration cohort).

# L — a prerelease token in a note's release: key. sort -V would order it *below*
# the release it postdates under semver, so ordering it at all is a rule this port
# has no authority to invent.
n="$tmp/prerelease/posts"
write_note "$n" "0.1.0" "None." "None."
write_note "$n" "0.2.0-rc1" "None." "None."
check_case "prerelease-token-refused" "$tmp/prerelease" 2 "is outside the grammar this gate orders"
check_case "prerelease-refusal-names-token" "$tmp/prerelease" 2 "'0.2.0-rc1'"
check_case "prerelease-refusal-names-source" "$tmp/prerelease" 2 "posts/note-0.2.0-rc1.md"
check_case "prerelease-refusal-names-grammar" "$tmp/prerelease" 2 "<major>.<minor>.<patch>"

# M — the other producer: a disposition data line. Its refusal names the line, not
# a note file, because that is where the reader has to go.
o="$tmp/dispgrammar"
write_note "$o/posts" "0.1.0" "None." "None."
write_note "$o/posts" "0.2.0" "None." "None."
check_deferral "disposition-token-refused" "$o" \
    'alpha release deferred:v0.2 — a two-field token
' 2 "alpha release deferred:v0.2"

if [[ "$fails" -gt 0 ]]; then
    echo "check-release-bump.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-release-bump.test: ok (a note under composition missing any roster section, Platforms included, fails closed and a former heading does not stand in there; its summary table reds when absent, miscounted, misanchored or short a row, printing the rows it derives; an unknown roster role refuses at exit 2; a tagged note's former gates heading still counts toward the floor; the non-empty section floors a patch red and passes a minor; all-None patch stays clean; an outstanding deferral floors a patch red and a single-note tree too, discharges on a later release line, passes a minor that clears it, and reds a minor bump the numeric floor still sits above; the roster presence assertion arms on a note under composition, reds when the section is absent there, and reports itself dormant once the note's version is tagged; a version token outside the triple refuses at exit 2 naming the token, its source and the grammar, from a note key and from a disposition line alike)"
exit 0
