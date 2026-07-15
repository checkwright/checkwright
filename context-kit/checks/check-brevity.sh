#!/usr/bin/env bash
# graph: couples=CLAUDE.md dir=one valve=none tier=precommit
# spec: context-kit/SPEC.md §The brevity gate — over-budget bullets in the budgeted always-loaded section that cite a deeper doc
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

_ck_cfg="${CONTEXT_KIT_CONFIG_FILE:-}"
if [[ -n "$_ck_cfg" ]]; then
    [[ -f "$_ck_cfg" ]] || {
        echo "context-kit: CONTEXT_KIT_CONFIG_FILE not found: $_ck_cfg" >&2
        exit 2
    }
    # shellcheck source=/dev/null  # consumer config path is resolved at runtime
    source "$_ck_cfg"
else
    _ck_cfg="${GATE_SDK_GATES_DIR:-scripts}/context-config.sh"
    if [[ -f "$_ck_cfg" ]]; then
        # shellcheck source=/dev/null  # consumer config path is resolved at runtime
        source "$_ck_cfg"
    fi
fi
unset _ck_cfg
: "${CONTEXT_KIT_BREVITY_FILE:=CLAUDE.md}"
: "${CONTEXT_KIT_BREVITY_SECTION:=## Shared conventions}"
: "${CONTEXT_KIT_BREVITY_BUDGET:=4}"
: "${CONTEXT_KIT_BREVITY_POINTER_RE:=§}"
[[ "$CONTEXT_KIT_BREVITY_BUDGET" =~ ^[0-9]+$ ]] \
    || { echo "check-brevity: CONTEXT_KIT_BREVITY_BUDGET must be an integer: $CONTEXT_KIT_BREVITY_BUDGET" >&2; exit 2; }

if [[ $# -gt 0 ]]; then
    BREVITY_FILE="$1"
else
    REPO_ROOT="$(git rev-parse --show-toplevel)" \
        || { echo "check-brevity: not inside a git repository" >&2; exit 2; }
    BREVITY_FILE="$REPO_ROOT/$CONTEXT_KIT_BREVITY_FILE"
fi
[[ -f "$BREVITY_FILE" ]] || { echo "check-brevity: file not found: $BREVITY_FILE" >&2; exit 2; }

BUDGET="$CONTEXT_KIT_BREVITY_BUDGET"

read -r -d '' BREVITY_AWK <<'AWK' || true
function hlevel(line,   n) {
    if (line !~ /^#+[[:space:]]/) return 0
    n = 0
    while (substr(line, n + 1, 1) == "#") n++
    return n
}
function flush() {
    if (!have) return
    pointer = (body ~ pointer_re) ? 1 : 0
    # last = span at the final non-blank line, so a trailing blank before the
    # next heading (or EOF) never inflates the line count of the last bullet.
    printf "%d\t%d\t%d\t%s\n", last, pointer, exempt, name
    have = 0
}
# Enter the governed section: heading line whose text starts with the knob.
!insec {
    if (hlevel($0) > 0 && substr($0, 1, length(section)) == section) {
        insec = 1; seen = 1; start_lvl = hlevel($0); prev = $0
    }
    next
}
# A heading at the same level or higher closes the section.
insec && hlevel($0) > 0 && hlevel($0) <= start_lvl { flush(); insec = 0; next }
insec {
    if ($0 ~ /^- \*\*/) {
        flush()
        have = 1
        name = $0
        sub(/^- \*\*/, "", name)
        sub(/\*\*.*/, "", name)
        span = 1
        last = 1
        body = $0
        exempt = ($0 ~ /brevity-exempt/ || prev ~ /brevity-exempt/) ? 1 : 0
    } else if (have) {
        span++
        if ($0 ~ /[^[:space:]]/) last = span
        body = body " " $0
    }
    prev = $0
    next
}
END { flush(); if (!seen) print "@@NOSECTION" }
AWK

records="$(awk \
    -v section="$CONTEXT_KIT_BREVITY_SECTION" \
    -v pointer_re="$CONTEXT_KIT_BREVITY_POINTER_RE" \
    "$BREVITY_AWK" "$BREVITY_FILE")"; st=$?
fail_closed "$st" check-brevity awk

if [[ "$records" == "@@NOSECTION" ]]; then
    printf 'check-brevity: no heading matches %s in %s: %s\n' \
        CONTEXT_KIT_BREVITY_SECTION "$BREVITY_FILE" "'$CONTEXT_KIT_BREVITY_SECTION'" >&2
    printf 'check-brevity: help: a renamed or deleted section silently disarms this gate — repoint CONTEXT_KIT_BREVITY_SECTION at the live heading, or restore the heading it names\n' >&2
    exit 2
fi

total=0
within=0
findings=()
while IFS=$'\t' read -r span pointer exempt name; do
    [[ -n "$span" ]] || continue
    total=$((total + 1))
    [[ "$span" -le "$BUDGET" ]] && within=$((within + 1))
    if [[ "$span" -gt "$BUDGET" && "$pointer" -eq 1 && "$exempt" -ne 1 ]]; then
        findings+=("$name — $span lines AND cites a deeper doc (over the ${BUDGET}-line budget while admitting its detail lives elsewhere)")
    fi
done <<< "$records"

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "BREVITY: ${#findings[@]} bullet(s) over budget in '$CONTEXT_KIT_BREVITY_SECTION':"
    printf '  %s\n' "${findings[@]}"
    echo "  help: cut each to ≤${BUDGET} lines by pushing detail into the section it already points to, or add <!-- brevity-exempt: <reason> --> on the bullet's first line / the line above if every line is load-bearing"
    exit 1
fi
echo "BREVITY: clean ($total bullets, $within within budget)"
exit 0
