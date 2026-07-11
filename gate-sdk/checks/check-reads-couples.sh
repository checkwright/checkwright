#!/usr/bin/env bash
# graph: couples=kit:checks/*.sh,scripts/*.sh,scripts/gates.list dir=one valve=none tier=precommit trigger=*
# spec: gate-sdk/SPEC.md §check-reads-couples — every statically resolvable recursive walk in a registered gate has its tracked read set covered by the gate's expanded couples; the undecidable remainder is skipped-and-counted
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

GATES_DIR="$(gate_sdk_gates_dir)"
LIST="$GATES_DIR/gates.list"

# spec: gate-sdk/SPEC.md §check-reads-couples — per source, one record per
# command-position recursive walk: "<lineno>\t<cmd>\t<exempt>\t<rawline>". A
# leading-comment or trailing '# reads-couples-exempt:' marker sets <exempt>.
extract_walks() {
    awk '
        function is_marker(s) { return (s ~ /reads-couples-exempt:/) }
        {
            cur = $0
            if (cur ~ /^[[:space:]]*#/) { prevmarker = is_marker(cur); next }
            cmd = ""
            if (match(cur, /(^|[|&;({`])[ \t]*gate_find[ \t]/)) cmd = "gate_find"
            else if (match(cur, /(^|[|&;({`])[ \t]*find[ \t]/)) cmd = "find"
            if (cmd != "") {
                ex = (prevmarker || is_marker(cur)) ? 1 : 0
                print FNR "\t" cmd "\t" ex "\t" cur
            }
            prevmarker = 0  # a trailing marker excuses only its own line, never the next walk
        }
    ' "$1"
}

# spec: gate-sdk/SPEC.md §check-reads-couples — the resolvable-root class; emits
# the repo-relative root, or status 1 when the first argument is undecidable.
resolve_root() {
    local cmd="$1" line="$2" src="$3" rest tok sub d1 kit
    rest="${line#*"$cmd" }"
    tok="${rest%%[[:space:]]*}"
    if [[ "$tok" == '"'*'"' && "$tok" != *'$'* ]]; then
        tok="${tok#\"}"; tok="${tok%\"}"; tok="${tok#./}"; tok="${tok%/}"
        printf '%s\n' "${tok:-.}"; return 0
    fi
    if [[ "$tok" == '"$KIT"'* ]]; then
        sub="${tok#'"$KIT"'}"; sub="${sub#/}"; sub="${sub%/}"
        d1="${src%/*}"; kit="${d1%/*}"
        [[ -n "$sub" ]] && printf '%s\n' "$kit/$sub" || printf '%s\n' "$kit"
        return 0
    fi
    if [[ "$tok" == '"$REPO_ROOT"'* ]]; then
        sub="${tok#'"$REPO_ROOT"'}"; sub="${sub#/}"; sub="${sub%/}"
        [[ -n "$sub" ]] && printf '%s\n' "$sub" || printf '%s\n' "."
        return 0
    fi
    return 1
}

# spec: gate-sdk/SPEC.md §check-reads-couples — the literal '-name <pat>' primary
# when one is extractable from the same invocation (a variable pattern is not).
name_pattern() {
    local line="$1" pat=""
    if [[ "$line" =~ -name[[:space:]]+\'([^\']*)\' ]]; then
        pat="${BASH_REMATCH[1]}"
    elif [[ "$line" =~ -name[[:space:]]+\"([^\"]*)\" ]]; then
        pat="${BASH_REMATCH[1]}"; [[ "$pat" == *'$'* ]] && pat=""
    fi
    printf '%s\n' "$pat"
}

# spec: gate-sdk/SPEC.md §check-reads-couples — couple glob semantics: segments
# never cross '/', so path and glob must have equal segment count.
path_matches_glob() {
    local path="$1" glob="$2" i
    [[ "$glob" == '*' ]] && return 0
    local -a ps gs
    IFS='/' read -ra ps <<<"$path"
    IFS='/' read -ra gs <<<"$glob"
    [[ ${#ps[@]} -eq ${#gs[@]} ]] || return 1
    for ((i = 0; i < ${#ps[@]}; i++)); do
        # shellcheck disable=SC2053  # gs[i] is the glob, deliberately unquoted
        [[ "${ps[i]}" == ${gs[i]} ]] || return 1
    done
    return 0
}

sources=()
if [[ $# -gt 0 ]]; then
    sources=("$@")
else
    [[ -f "$LIST" ]] || { echo "check-reads-couples: no registry at $LIST" >&2; exit 2; }
    mapfile -t RESOLVE_DIRS < <(gate_check_dirs)
    while IFS= read -r c; do
        src="$(gate_resolve "$c" "${RESOLVE_DIRS[@]}" || true)"
        [[ -n "$src" ]] && sources+=("$src")
    done < <(gates_list_members "$LIST")
fi

analyzed=0
skipped=0
exempt=0
findings=()

for src in "${sources[@]+"${sources[@]}"}"; do
    [[ -f "$src" ]] || continue
    couples="$(gate_expand_couples "$(gate_manifest_field "$src" couples)")"
    IFS=',' read -ra COUPLE_GLOBS <<<"$couples"
    walks="$(extract_walks "$src")"; st=$?
    fail_closed "$st" READS-COUPLES "awk walk-scan($src)"
    [[ -n "$walks" ]] || continue
    while IFS=$'\t' read -r lno cmd ex rawline; do
        [[ -n "$lno" ]] || continue
        if [[ "$ex" == 1 ]]; then exempt=$((exempt + 1)); continue; fi
        if ! root="$(resolve_root "$cmd" "$rawline" "$src")"; then
            skipped=$((skipped + 1)); continue
        fi
        analyzed=$((analyzed + 1))
        namepat="$(name_pattern "$rawline")"
        if [[ "$root" == "." ]]; then
            listing="$(git ls-files)"; gst=$?
        else
            listing="$(git ls-files -- "$root")"; gst=$?
        fi
        [[ "$gst" -eq 0 ]] || { echo "check-reads-couples: git ls-files failed for root '$root'" >&2; exit 2; }
        while IFS= read -r f; do
            [[ -n "$f" ]] || continue
            [[ "$cmd" == gate_find ]] && gate_path_pruned "$f" && continue
            if [[ -n "$namepat" ]]; then
                # shellcheck disable=SC2053  # namepat is the glob, deliberately unquoted
                [[ "${f##*/}" == $namepat ]] || continue
            fi
            covered=0
            for g in "${COUPLE_GLOBS[@]}"; do
                path_matches_glob "$f" "$g" && { covered=1; break; }
            done
            [[ "$covered" == 1 ]] || \
                findings+=("$(basename "$src" .sh): recursive walk over '$root' (line $lno) reads tracked '$f' — no couple covers it (couples: $couples)")
        done <<<"$listing"
    done <<<"$walks"
done

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-reads-couples: a resolvable recursive walk reads a tracked path its '# graph: couples=' does not cover:"
    printf '  %s\n' "${findings[@]}"
    echo "  help: add the covering sibling glob to the gate's '# graph: couples=' — a '<dir>/<sub>/*.ext' that matches the deeper path (globs never cross '/', so a shallow one-level couple misses a file one level down), then regenerate the hook + graph artifacts; or mark the walk '# reads-couples-exempt: <reason>' (same line, or the line directly above) when the uncoupled read is deliberate. Never widen a glob to cross '/' to pass a near-miss."
    exit 1
fi
echo "READS-COUPLES: clean ($analyzed resolvable walk(s) covered; $skipped undecidable walk(s) skipped-and-counted; $exempt exempt; across ${#sources[@]} gate(s))"
exit 0
