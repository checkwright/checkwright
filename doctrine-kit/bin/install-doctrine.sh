#!/usr/bin/env bash
# spec: doctrine-kit/SPEC.md §install-doctrine — idempotent insert/replace of the doctrine reference block between fixed markers in the always-loaded agent file, carrying the consumer's declared trims across the rewrite
set -euo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../lib/doctrine.sh
source "$KIT/lib/doctrine.sh"
# shellcheck source=../../gate-sdk/lib/inject.sh
source "$SDK/lib/inject.sh"

AGENT_FILE="${1:-$DOCTRINE_KIT_AGENT_FILE}"
DOCTRINE_FILE="${2:-$DOCTRINE_KIT_DOCTRINE_FILE}"
[[ -f "$AGENT_FILE" ]] \
    || { echo "install-doctrine: agent file not found: $AGENT_FILE — nothing to install into" >&2; exit 2; }

BEGIN="<!-- doctrine-kit:begin -->"
END="<!-- doctrine-kit:end -->"

# spec: doctrine-kit/SPEC.md §install-doctrine — the untrimmed digest: the always-loaded shape applied to the doctrine itself, a one-line-per-rule digest plus the markdown link to the doctrine file
digest() {
    cat <<EOF
## Delivery doctrine

The cross-kit delivery rules live in [$DOCTRINE_FILE]($DOCTRINE_FILE) — re-vendor
to upgrade. The always-loaded maintenance rules, one line each; the doctrine adds
an engineering-craft section behind the link:

- **Content-tiering / SSOT** — one content tier per surface; point, never restate.
- **Enforcement-first** — the fix and the gate that catches it land in one unit; removing the duplication outranks gating it.
- **De-literalization** — prose cites names; code or the owning SPEC owns values.
- **Derivation-first** — derive the derivable (a roster, a count), never maintain it; a needed copy is generated and freshness-gated.
- **Always-loaded shape** — one line per rule here; the mechanism behind the pointer.
- **Load-trigger residency** — resident only when no stage, skill, or tool loads it.
- **Widest-true-tier placement** — the widest tier true for every reader of it.
- **Oracle-first** — run the gate, never emulate it; a red run is the feedback channel.
- **Spec-over-precedent** — the owner doc is ground truth; history answers what happened, never what is correct.
- **Gap disposition** — a gap you defer is costed and filed, never flagged-and-skipped.
- **Scope-gated intake** — a mid-session initiative is filed as a costed Deferred entry by default, never started; work enters only through scope.
EOF
}

# spec: doctrine-kit/SPEC.md §install-doctrine — a digest bullet's rule name, extracted exactly as check-doctrine-registration extracts it (assertion C), so the substitution keys on the same string the gate keys on
bullet_name() {   # $1 = a digest line -> its bold lead-in name, non-zero when the line is not a rule bullet
    local l="$1"
    [[ "$l" == "- **"*"**"* ]] || return 1
    l="${l#- \*\*}"
    printf '%s\n' "${l%%\*\**}"
}

# spec: doctrine-kit/SPEC.md §install-doctrine — the declared-trim grammar read exactly as check-doctrine-registration reads it (assertion B), so installer and gate never disagree about which rule a marker names
harvest_trims() {   # block content on stdin -> one '<rule name><TAB><marker line verbatim>' per declared trim, in declaration order
    awk '
        /doctrine-digest-trim:/ {
            name = $0
            sub(/^.*doctrine-digest-trim:[[:space:]]*/, "", name)
            sub(/[[:space:]]*—.*$/, "", name)
            print name "\t" $0
        }
    '
}

# spec: doctrine-kit/SPEC.md §install-doctrine — the round-trip's read half: the block as it stands is the only record of what the consumer declared, so it is harvested before the rewrite that would erase it
CURRENT="$(read_marker_block "$AGENT_FILE" "$BEGIN" "$END")" || exit $?

declare -A TRIM_LINE=()
TRIM_ORDER=()
DUPES=()
while IFS=$'\t' read -r tname tline; do
    [[ -n "$tname" ]] || continue
    if [[ -n "${TRIM_LINE[$tname]+set}" ]]; then
        DUPES+=("$tname")
        continue
    fi
    TRIM_LINE["$tname"]="$tline"
    TRIM_ORDER+=("$tname")
done < <(printf '%s\n' "$CURRENT" | harvest_trims)

declare -A IS_RULE=()
while IFS= read -r dline; do
    if rname="$(bullet_name "$dline")"; then IS_RULE["$rname"]=1; fi
done < <(digest)

ORPHANS=()
if [[ ${#TRIM_ORDER[@]} -gt 0 ]]; then
    for tname in "${TRIM_ORDER[@]}"; do
        [[ -n "${IS_RULE[$tname]:-}" ]] || ORPHANS+=("$tname")
    done
fi

# spec: doctrine-kit/SPEC.md §install-doctrine — the emit: a trimmed rule's marker replaces its bullet in place, because a block carrying both would satisfy the gate while handing back the rule the consumer removed
block() {
    local line name
    while IFS= read -r line; do
        if name="$(bullet_name "$line")" && [[ -n "${TRIM_LINE[$name]+set}" ]]; then
            printf '%s\n' "${TRIM_LINE[$name]}"
            continue
        fi
        printf '%s\n' "$line"
    done < <(digest)
    # spec: doctrine-kit/SPEC.md §install-doctrine — a trim naming no live rule has no bullet position to take, so it is carried at the digest's end rather than dropped
    if [[ ${#ORPHANS[@]} -gt 0 ]]; then
        for name in "${ORPHANS[@]}"; do printf '%s\n' "${TRIM_LINE[$name]}"; done
    fi
}

action="$(block | inject_marker_block "$AGENT_FILE" "$BEGIN" "$END")" || exit $?

# spec: doctrine-kit/SPEC.md §install-doctrine — findings go to stderr, the one channel init does not discard, so a reconciliation the consumer owes is never silent on the install path
if [[ ${#ORPHANS[@]} -gt 0 ]]; then
    for tname in "${ORPHANS[@]}"; do
        echo "install-doctrine: declared trim names '$tname', which no rule in the current doctrine digest matches — carried forward unchanged; adopt the renamed rule or drop the marker" >&2
    done
fi
if [[ ${#DUPES[@]} -gt 0 ]]; then
    for tname in "${DUPES[@]}"; do
        echo "install-doctrine: duplicate declared trim for '$tname' — the first is carried, the duplicate dropped" >&2
    done
fi

findings=""
if [[ ${#ORPHANS[@]} -gt 0 ]]; then findings="$findings, ${#ORPHANS[@]} unmatched"; fi
if [[ ${#DUPES[@]} -gt 0 ]]; then findings="$findings, ${#DUPES[@]} duplicate"; fi
echo "install-doctrine: $action the doctrine reference block in $AGENT_FILE (link → $DOCTRINE_FILE); ${#TRIM_ORDER[@]} declared trim(s) carried$findings"
