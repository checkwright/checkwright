#!/usr/bin/env bash
# Direct unit test of gate-sdk/lib/declaration.sh — the two container arms and
# the resolve/refuse trichotomy (gate-sdk/SPEC.md §lib/declaration.sh).
#
# Why a direct test: bin/upgrade-smoke.sh's record-arm caller is a bin tool
# whose contract forgoes a good/bad pair, so this is that branch's runtime
# lock-in independent of scripts/check-tightened-gates-note-parity's own
# good/bad pair on the record arm (gate-sdk/SPEC.md §lib/declaration.sh). The
# markdown arm's callers do carry fixtures; its status-2 and empty-section arms
# are still checked here, being unreachable from a well-formed corpus.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
# shellcheck source=../lib/declaration.sh
source "$DIR/lib/declaration.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

note() { printf '%s\n' "$2" > "$tmp/$1.md"; printf '%s\n' "$tmp/$1.md"; }

# $1=label  $2=want-status  $3=want-stdout  $4...=the call
expect() {
    local label="$1" want_st="$2" want_out="$3"; shift 3
    local out st
    out="$("$@")"; st=$?
    if [[ "$st" -ne "$want_st" ]]; then
        echo "  FAIL [$label]: want status $want_st, got $st -- $out"; fails=$((fails + 1)); return
    fi
    if [[ "$out" != "$want_out" ]]; then
        echo "  FAIL [$label]: want stdout '$want_out', got '$out'"; fails=$((fails + 1))
    fi
}

# --- the markdown arm ------------------------------------------------------

f="$(note canonical '## Tightened gates

- `check-alpha` — tightened.
- `check-beta` — landed new.

## Renamed knobs

None.')"
expect "markdown/token-list" 0 'check-alpha
check-beta' decl_section_tokens "$f" "Tightened gates"
expect "markdown/container-keeps-bullets-only" 0 '- `check-alpha` — tightened.
- `check-beta` — landed new.' decl_section_bullets "$f" "Tightened gates"

f="$(note none '## Tightened gates

None. A trailing clause ruling out a near-miss rides the explicit empty set.

## Renamed knobs

None.')"
expect "markdown/explicit-none" 0 '' decl_section_tokens "$f" "Tightened gates"

# The two spellings the corpus actually carried: bolded, and bold-and-backticked.
# Neither may be silently stripped to a token — that is the whole defect.
f="$(note bolded '## Tightened gates

- **check-alpha** — bolded.
- **`check-beta`** — bold-and-backticked.')"
expect "markdown/bolded-refused" 1 '- **check-alpha** — bolded.
- **`check-beta`** — bold-and-backticked.' decl_section_tokens "$f" "Tightened gates"

# A non-`None` section holding no bullet at all is the silent-empty case: it
# refuses with nothing to print, rather than resolving to an empty declared set.
f="$(note prose-only '## Tightened gates

Several gates moved this release.')"
expect "markdown/prose-only-refused" 1 '' decl_section_tokens "$f" "Tightened gates"

f="$(note absent '## Renamed knobs

None.')"
expect "markdown/section-absent" 2 '' decl_section_tokens "$f" "Tightened gates"

# `None` is read off the section's first non-blank line, so a later line opening
# with the word does not turn a real declaration into an empty set.
f="$(note none-later '## Tightened gates

- `check-alpha` — tightened.

None of the above affects a vendored tree that shadows it.')"
expect "markdown/none-only-at-head" 0 'check-alpha' decl_section_tokens "$f" "Tightened gates"

# --- the record arm --------------------------------------------------------

printf '# contract: x.md §y\ncheck-alpha\ncheck-beta\n' > "$tmp/decl.txt"
expect "record/data-lines" 0 'check-alpha
check-beta' decl_record_tokens "$tmp/decl.txt"

printf '# contract: x.md §y\n' > "$tmp/header-only.txt"
expect "record/header-only-is-empty-set" 0 '' decl_record_tokens "$tmp/header-only.txt"

expect "record/missing-file-is-empty-set" 0 '' decl_record_tokens "$tmp/nonexistent.txt"

printf '# contract: x.md §y\n- `check-alpha` — markup\n' > "$tmp/marked-up.txt"
expect "record/markup-refused" 1 '- `check-alpha` — markup' decl_record_tokens "$tmp/marked-up.txt"

if [[ "$fails" -gt 0 ]]; then
    echo "lib-declaration.test: $fails assertion(s) failed"
    exit 1
fi
echo "lib-declaration.test: ok (markdown arm resolves a token list and an explicit None, refuses the bolded and bold-and-backticked spellings and a bulletless non-None section, reports an absent section; None is read at the section head only; record arm resolves data lines, treats a header-only and a missing file as the empty set, and refuses a marked-up line)"
exit 0
