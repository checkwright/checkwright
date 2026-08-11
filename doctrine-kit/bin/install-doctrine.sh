#!/usr/bin/env bash
# spec: doctrine-kit/SPEC.md §install-doctrine — idempotent insert/replace of the doctrine reference block between fixed markers in the always-loaded agent file, carrying the consumer's declared trims across the rewrite
set -euo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../lib/doctrine.sh
source "$KIT/lib/doctrine.sh"
# shellcheck source=../../gate-sdk/lib/inject.sh
source "$SDK/lib/inject.sh"

BEGIN="<!-- doctrine-kit:begin -->"
END="<!-- doctrine-kit:end -->"

# spec: doctrine-kit/SPEC.md §install-doctrine — --remove: the reverse of the insert path over the same marker pair, riding gate-sdk's remove_marker_block (gate-sdk/SPEC.md §lib/inject.sh). It harvests no trims and emits no digest — a removal has nothing to carry forward — and reports what it did on the same channel (stdout) the insert path reports on. A missing agent file stays exit 2, unchanged.
if [[ "${1:-}" == "--remove" ]]; then
    shift
    AGENT_FILE="${1:-$DOCTRINE_KIT_AGENT_FILE}"
    [[ -f "$AGENT_FILE" ]] \
        || { echo "install-doctrine: agent file not found: $AGENT_FILE — nothing to remove" >&2; exit 2; }

    result="$(remove_marker_block "$AGENT_FILE" "$BEGIN" "$END")" || exit $?
    if [[ "$result" == "removed" ]]; then
        echo "install-doctrine: removed the doctrine reference block from $AGENT_FILE"
    else
        echo "install-doctrine: no doctrine reference block found in $AGENT_FILE — nothing to remove"
    fi
    exit 0
fi

AGENT_FILE="${1:-$DOCTRINE_KIT_AGENT_FILE}"
DOCTRINE_FILE="${2:-$DOCTRINE_KIT_DOCTRINE_FILE}"
[[ -f "$AGENT_FILE" ]] \
    || { echo "install-doctrine: agent file not found: $AGENT_FILE — nothing to install into" >&2; exit 2; }

# spec: doctrine-kit/SPEC.md §check-doctrine-registration — the doctrine-side section heading is kit mechanism (the kit ships DOCTRINE.md), never config
METH_SECTION="## Methodology-maintenance rules"

[[ -f "$DOCTRINE_FILE" ]] \
    || { echo "install-doctrine: doctrine file not found: $DOCTRINE_FILE — nothing to derive the digest from" >&2; exit 2; }

# spec: doctrine-kit/SPEC.md §install-doctrine — the rule walk: name read exactly as check-doctrine-registration assertion C reads it, summary read from the rule's own *Digest:* trailer exactly as assertion E counts it
read -r -d '' DIGEST_WALK <<'AWK' || true
function hlevel(line,   n) {
    if (line !~ /^#+[[:space:]]/) return 0
    n = 0
    while (substr(line, n + 1, 1) == "#") n++
    return n
}
function flush() {
    if (have) print cur_name "\t" dcount "\t" summary
}
!insec {
    if (hlevel($0) > 0 && substr($0, 1, length(section)) == section) {
        insec = 1; seen = 1; start_lvl = hlevel($0)
    }
    next
}
insec && hlevel($0) > 0 && hlevel($0) <= start_lvl { flush(); insec = 0; have = 0; next }
insec {
    if ($0 ~ /^[0-9]+\.[[:space:]]+\*\*/) {
        flush()
        name = $0
        sub(/^[0-9]+\.[[:space:]]+\*\*/, "", name)
        sub(/\*\*.*/, "", name)
        sub(/\.$/, "", name)
        cur_name = name; have = 1; dcount = 0; summary = ""
    } else if ($0 ~ /^[[:space:]]*\*Digest:\*/) {
        dcount++
        val = $0
        sub(/^[[:space:]]*\*Digest:\*[[:space:]]*/, "", val)
        sub(/[[:space:]]+$/, "", val)
        if (dcount == 1) summary = val
    }
    next
}
END { flush(); if (!seen) print "@@NOSECTION" }
AWK

# spec: doctrine-kit/SPEC.md §install-doctrine — derived once at top level, never inside digest(): digest() is read through process substitution, where an exit would end the subshell and let a malformed doctrine through as a silently short digest
WALK_OUT="$(awk -v section="$METH_SECTION" "$DIGEST_WALK" "$DOCTRINE_FILE")" \
    || { echo "install-doctrine: awk failed reading $DOCTRINE_FILE" >&2; exit 2; }
if [[ "$WALK_OUT" == "@@NOSECTION" ]]; then
    echo "install-doctrine: no '$METH_SECTION' section in $DOCTRINE_FILE — cannot derive the digest from an unreadable rule set" >&2
    exit 2
fi

BULLETS=()
while IFS=$'\t' read -r rname dcount summary; do
    [[ -n "$rname" ]] || continue
    if [[ "$dcount" != 1 ]]; then
        echo "install-doctrine: methodology rule '$rname' carries $dcount *Digest:* trailer(s) in $DOCTRINE_FILE, want exactly one — refusing to emit a digest missing or double-sourcing its bullet" >&2
        exit 2
    fi
    BULLETS+=("- **$rname** — $summary")
done <<< "$WALK_OUT"
[[ ${#BULLETS[@]} -gt 0 ]] \
    || { echo "install-doctrine: no methodology rules found under '$METH_SECTION' in $DOCTRINE_FILE — refusing to install an empty digest" >&2; exit 2; }

# spec: doctrine-kit/SPEC.md §install-doctrine — the untrimmed digest: the always-loaded shape applied to the doctrine itself, a one-line-per-rule digest plus the markdown link to the doctrine file
digest() {
    cat <<EOF
## Delivery doctrine

The cross-kit delivery rules live in [$DOCTRINE_FILE]($DOCTRINE_FILE) — re-vendor
to upgrade. The always-loaded maintenance rules, one line each; the doctrine adds
an engineering-craft section behind the link:

EOF
    printf '%s\n' "${BULLETS[@]}"
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
