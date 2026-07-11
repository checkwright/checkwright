#!/usr/bin/env bash
# graph: couples=kit:README.md dir=one valve=none tier=precommit trigger=*
# spec: canon-kit/SPEC.md §Layout and configuration — every kit-name reference in a tracked file resolves to a live kit root: a slash/line-anchored <name>-kit or gate-sdk path segment names a gate_kit_roots dir, and a live-prefix kit knob resolves to a tracked kit knob (check-docs-cmd's resolver)
#
# usage: check-kit-ref-liveness.sh [scan-root]
#   Scans tracked files under scan-root (default '.'), valving the pruned dirs,
#   docs/posts/*, the trajectory data, SPEC-*.md amendments, and the queue.
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

SCANROOT="${1:-.}"

TOP="$(git rev-parse --show-toplevel 2>/dev/null)" || {
    echo "check-kit-ref-liveness: not a git repository — cannot enumerate tracked paths/knobs" >&2; exit 2; }

live_list=""; roots=(); prefixes=()
while IFS= read -r root; do
    [[ -n "$root" ]] || continue
    root="${root%/}"
    live_list+="${root##*/} "
    roots+=("$root")
    base="${root##*/}"; p="${base^^}"; prefixes+=("${p//-/_}_")
done < <(gate_kit_roots_rel)
[[ -n "$live_list" ]] || { echo "check-kit-ref-liveness: gate_kit_roots enumerated no roots" >&2; exit 2; }
pfxalt="$(IFS='|'; printf '%s' "${prefixes[*]}")"

declare -A defined=()
hits="$(git -C "$TOP" grep -h -E "($pfxalt)[A-Z0-9_]*" -- "${roots[@]}" ':!*.md' ':!*/gate-tests/*' 2>/dev/null)"; st=$?
[[ "$st" -le 1 ]] || { echo "check-kit-ref-liveness: git grep failed (exit $st) building the knob set" >&2; exit 2; }
while IFS= read -r name; do
    [[ -n "$name" ]] && defined["$name"]=1
done < <(printf '%s\n' "$hits" | grep -oE "($pfxalt)[A-Z0-9_]*" || true)

knob_ok() {  # $1=knob token — exact code occurrence, or (family stem ending '_') any name under it
    local t="$1" k
    [[ -n "${defined[$t]:-}" ]] && return 0
    if [[ "$t" == *_ ]]; then
        for k in "${!defined[@]}"; do [[ "$k" == "$t"* ]] && return 0; done
    fi
    return 1
}

listing="$(git ls-files -- "$SCANROOT")"; st=$?
fail_closed "$st" KIT-REF-LIVENESS git-ls-files

files=()
scanned=0
while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    gate_path_pruned "$path" && continue                    # target/.git/node_modules/.tmp/gate-tests
    case "$path" in
        docs/posts/*) continue ;;                           # immutable published artifacts
        docs/evidence-data.md) continue ;;                  # generated trajectory data (immutable brand)
    esac
    [[ "${path##*/}" == SPEC-*.md ]] && continue            # amendments name retired/future paths
    qf="${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}"
    [[ "${path##*/}" == "${qf##*/}" ]] && continue          # queue is design-ahead: names future knobs/paths
    [[ -f "$path" ]] || continue
    files+=("$path")
    scanned=$((scanned + 1))
done <<< "$listing"

if [[ ${#files[@]} -eq 0 ]]; then
    echo "KIT-REF-LIVENESS: clean (0 tracked file(s) under $SCANROOT after valves; nothing to resolve)"
    exit 0
fi

raw="$(awk -v live_list="$live_list" -v pfxre="^($pfxalt)" '
    BEGIN {
        n = split(live_list, a, " ")
        for (i = 1; i <= n; i++) if (a[i] != "") live[a[i]] = 1
    }
    function scan_path(line,   s, tok, before, afterpos, after) {
        s = line
        while (match(s, /[a-z0-9]+(-[a-z0-9]+)*-kit|gate-sdk/)) {
            tok = substr(s, RSTART, RLENGTH)
            before = (RSTART > 1) ? substr(s, RSTART - 1, 1) : "/"
            afterpos = RSTART + RLENGTH
            after = (afterpos <= length(s)) ? substr(s, afterpos, 1) : "/"
            if (before == "/" && after == "/" && !(tok in live))
                print "P\t" FILENAME ":" FNR ": path segment <" tok "> names no live kit root"
            s = substr(s, RSTART + RLENGTH)
        }
    }
    function scan_knob(line,   s, run) {
        s = line
        while (match(s, /[A-Z][A-Z0-9_]+/)) {
            run = substr(s, RSTART, RLENGTH)
            if (run ~ pfxre) print "K\t" FILENAME ":" FNR "\t" run
            s = substr(s, RSTART + RLENGTH)
        }
    }
    { scan_path($0); scan_knob($0) }
' "${files[@]}")"; st=$?
fail_closed "$st" KIT-REF-LIVENESS awk

bad=()
while IFS=$'\t' read -r kind loc tok; do
    [[ -n "${kind:-}" ]] || continue
    if [[ "$kind" == P ]]; then
        bad+=("$loc")
    else
        knob_ok "$tok" || bad+=("$loc: kit knob <$tok> resolves to no tracked kit knob")
    fi
done <<< "$raw"

if [[ ${#bad[@]} -gt 0 ]]; then
    echo "check-kit-ref-liveness: tracked file(s) reference a kit that names no live root:"
    printf '  %s\n' "${bad[@]}"
    echo "  help: a kit was renamed or retired and a reference dangles — update the path"
    echo "        segment or knob to a live kit (gate_kit_roots), or delete the reference."
    exit 1
fi

echo "KIT-REF-LIVENESS: clean ($scanned tracked file(s) scanned under $SCANROOT; every kit path segment + live-prefix knob resolves)"
exit 0
