#!/usr/bin/env bash
# Cross-implementation parity for the one derivation gate-sdk holds twice: the
# shell library gate-sdk/lib/declaration.sh and its compiled counterpart in
# native/src/declaration.rs. The shell form's non-test caller set is empty since
# the upgrade suite's declaration resolve moved in-crate, so the library is
# takeable under its own section; until it is taken both holders exist, and this
# is criterion 6's machine-held disposition holding them equal in the meantime
# (gate-sdk/SPEC.md §The port-candidate criteria, criterion 6; §lib/declaration.sh).
#
# A port-time byte-identity proof does not discharge that clause: it proves the
# two agreed once and expires at the next edit to either side. What is compared
# is *classification* over one canned corpus — the two holders share no data
# shape, the shell reporting the trichotomy as an exit status and the crate as a
# type — never derived literals. A against B directly, with no committed expected
# file: a maintained golden would be a third copy to drift.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
# shellcheck source=../lib/gate.sh
source "$DIR/lib/gate.sh"
# shellcheck source=../lib/declaration.sh
source "$DIR/lib/declaration.sh"

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
checks=0

# spec: gate-sdk/SPEC.md §check-gate-binary-fresh — a declaration is not a dispatch: a consumer
# on an uncovered platform vendors the shell library with no artifact behind it, and a parity
# assertion there would be vacuous rather than true. The skip is declared on the clean line so a
# reader can tell "no binary here" from "parity holds"; a binary that is present and refuses the
# arm is a stale binary rather than an absent one, so it fails here.
BIN="$(gate_native_bin)"
if [[ ! -x "$BIN" ]]; then
    echo "declaration-lib-parity.test: ok (0 assertions; skipped — no gate binary at $BIN, so nothing dispatches to the compiled twin)"
    exit 0
fi

# The corpus is the grammar's trichotomy rather than a sample of any tree's notes,
# and it carries every arm both holders can reach: a container that is absent, one
# whose body is an explicit `None`, one resolving to tokens, and three that are
# none of these — the empty container, the prose-only container, and the container
# whose bullets carry no readable lead token. Section names are generic on purpose:
# the library takes its section as an argument and carries no vocabulary of its own
# (gate-sdk/SPEC.md §lib/declaration.sh).
cat >"$SANDBOX/note.md" <<'EOF'
---
release: v0.0.0
---

# Corpus

## Alpha

- `alpha-one` — resolved.
- `alpha-two` — resolved.

### Deeper

- `alpha-three` — a deeper heading does not close the container.

A trailing prose paragraph carries no lead token and is read past.

## Beta

None, and the trailing clause ruling out a near-miss rides the explicit empty set.

## Gamma

## Delta

- **delta-one** — bolded, so the parser reads no token here.
- **`delta-two`** — bold-and-backticked, the same silence by a second spelling.

## Epsilon

- `epsilon-one` — readable.
- see `epsilon-two` — a lead token not directly after the marker.

## Zeta

Prose only: not `None`, and no bullet at all, so the parse resolves to an empty
set the section contradicts.
EOF

printf '# header\nrec-one\nrec-two\n'        >"$SANDBOX/record-clean.txt"
printf '# header\nrec one\nrec-two\n'        >"$SANDBOX/record-malformed.txt"
printf '# header\nrec-one\nrec-two'          >"$SANDBOX/record-unterminated.txt"
printf '# header only, no data line\n'       >"$SANDBOX/record-empty.txt"

# The shell side's answer is the trichotomy as its callers read it: branch on the
# status, then — for status 0 — on stdout emptiness, which is exactly the shape
# §lib/declaration.sh records as having made the silently-empty declaration
# possible. Rendering it as records is what lets the compiled side, where the same
# trichotomy is a type, be compared without either holder's representation leaking
# into the comparison.
shell_section() {   # $1=file  $2=section
    local out st n
    out="$(decl_section_bullets "$1" "$2")"; st=$?
    if [[ "$st" -ne 0 ]]; then
        printf 'bullets\tabsent\n'
    else
        n=0
        [[ -n "$out" ]] && n="$(grep -c . <<<"$out")"
        printf 'bullets\t%s\n' "$n"
    fi
    out="$(decl_section_tokens "$1" "$2")"; st=$?
    case "$st" in
        2) printf 'verdict\tabsent\n' ;;
        0)
            if [[ -z "$out" ]]; then
                printf 'verdict\tnone\n'
            else
                printf 'verdict\ttokens\n'
                while IFS= read -r line; do printf 'token\t%s\n' "$line"; done <<<"$out"
            fi
            ;;
        *)
            printf 'verdict\tunparsed\n'
            if [[ -n "$out" ]]; then
                while IFS= read -r line; do printf 'unparsed\t%s\n' "$line"; done <<<"$out"
            fi
            ;;
    esac
    return 0
}

shell_record() {   # $1=file
    local out st
    out="$(decl_record_tokens "$1")"; st=$?
    if [[ "$st" -eq 0 ]]; then
        printf 'record\tok\n'
        [[ -n "$out" ]] && while IFS= read -r line; do printf 'token\t%s\n' "$line"; done <<<"$out"
    else
        printf 'record\tmalformed\n'
        [[ -n "$out" ]] && while IFS= read -r line; do printf 'malformed\t%s\n' "$line"; done <<<"$out"
    fi
    return 0
}

compare() {  # $1=label  $2..=binary argv after --declaration-parity
    local label="$1"; shift
    local a b arc brc
    checks=$((checks + 1))
    if [[ "$1" == "section" ]]; then
        a="$(shell_section "$2" "$3")"; arc=$?
    else
        a="$(shell_record "$2")"; arc=$?
    fi
    b="$("$BIN" --declaration-parity "$@")"; brc=$?
    if [[ "$arc" -ne 0 || "$brc" -ne 0 ]]; then
        echo "  FAIL [$label]: a side could not report (shell exit $arc, binary exit $brc)"
        fails=$((fails + 1))
        return
    fi
    if [[ -z "$a" ]]; then
        echo "  FAIL [$label]: the shell side classified nothing — a vacuous agreement, not a parity hold"
        fails=$((fails + 1))
        return
    fi
    if [[ "$a" != "$b" ]]; then
        echo "  FAIL [$label]: the two implementations disagree about the same input:"
        diff <(printf '%s\n' "$a") <(printf '%s\n' "$b") | sed 's/^/    /'
        fails=$((fails + 1))
    fi
}

compare "section-absent"       section "$SANDBOX/note.md" "Missing"
compare "section-tokens"       section "$SANDBOX/note.md" "Alpha"
compare "section-explicitnone" section "$SANDBOX/note.md" "Beta"
compare "section-emptybullets" section "$SANDBOX/note.md" "Gamma"
compare "section-unparsed"     section "$SANDBOX/note.md" "Delta"
compare "section-mixed"        section "$SANDBOX/note.md" "Epsilon"
compare "section-proseonly"    section "$SANDBOX/note.md" "Zeta"
compare "record-clean"         record  "$SANDBOX/record-clean.txt"
compare "record-malformed"     record  "$SANDBOX/record-malformed.txt"
compare "record-unterminated"  record  "$SANDBOX/record-unterminated.txt"
compare "record-empty"         record  "$SANDBOX/record-empty.txt"

# The corpus must actually reach the arms the comparison is bought for: an
# agreement over a corpus that classifies nothing is the vacuity this unit exists
# to end, arriving one layer up. Each is read off the shell side, the holder under
# obligation — including the empty-stdout status 1, the arm whose invisibility to a
# status-then-emptiness caller is what §lib/declaration.sh's refusal was written for.
T=$'\t'
have() {   # $1=label  $2=grep -E pattern  $3=classification
    checks=$((checks + 1))
    grep -qE "$2" <<<"$3" || {
        echo "  FAIL [$1]: the corpus no longer exercises this arm"
        fails=$((fails + 1))
    }
}
have "arm-absent"       "^verdict${T}absent$"          "$(shell_section "$SANDBOX/note.md" Missing)"
have "arm-tokens"       "^token${T}alpha-three$"       "$(shell_section "$SANDBOX/note.md" Alpha)"
have "arm-none"         "^verdict${T}none$"            "$(shell_section "$SANDBOX/note.md" Beta)"
have "arm-empty-quiet"  "^bullets${T}0$"               "$(shell_section "$SANDBOX/note.md" Gamma)"
have "arm-unparsed"     "^unparsed${T}- \*\*delta-one" "$(shell_section "$SANDBOX/note.md" Delta)"
have "arm-record-ok"    "^token${T}rec-two$"           "$(shell_record "$SANDBOX/record-clean.txt")"
have "arm-record-bad"   "^malformed${T}rec one$"       "$(shell_record "$SANDBOX/record-malformed.txt")"

if [[ "$fails" -gt 0 ]]; then
    echo "declaration-lib-parity.test: $fails of $checks assertion(s) failed"
    exit 1
fi
echo "declaration-lib-parity.test: ok ($checks assertions; both container arms and the token predicate held to the compiled twin over one corpus carrying every arm of the trichotomy)"
exit 0
