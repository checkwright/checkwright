#!/usr/bin/env bash
# graph: couples=*SPEC*.md,*README.md,CLAUDE.md,scripts/*.sh,kit:*.sh dir=one valve=none tier=precommit
# spec: canon-kit/SPEC.md §check-docs-cmd — every fenced invoked repo-relative .sh path and every backticked/fenced kit-prefixed env knob in the governed doc set resolves against the tree
#
# usage: check-docs-cmd.sh [file...]
#   Defaults to the manifest set (lib/spec.sh) minus CANON_KIT_MDREF_EXCLUDE — the
#   same governed doc set check-md-refs guards (shared config, no second knob).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

TOP="$(git rev-parse --show-toplevel 2>/dev/null)" || {
    echo "check-docs-cmd: not a git repository — cannot verify tracked paths/knobs" >&2; exit 2; }

excluded() {
    local rel="$1" g
    for g in "${CANON_KIT_MDREF_EXCLUDE[@]+"${CANON_KIT_MDREF_EXCLUDE[@]}"}"; do
        # shellcheck disable=SC2053  # $g is the exclude glob, matched unquoted on purpose
        [[ "$rel" == $g ]] && return 0
    done
    return 1
}

if [[ $# -gt 0 ]]; then
    files=("$@")
else
    files=()
    while IFS= read -r f; do
        excluded "${f#./}" || files+=("$f")
    done < <(spec_manifest_files ".")
fi

# The kit-prefix roster (assertion B): each kit root's basename uppercased,
# hyphens to underscores, trailing '_' — 'gate-sdk' -> 'GATE_SDK_'. The prefix is
# the in-scope signal: a caps name carrying one is a namespaced knob to verify.
roots=(); prefixes=()
while IFS= read -r root; do
    [[ -n "$root" ]] || continue
    root="${root%/}"
    roots+=("$root")
    base="${root##*/}"
    p="${base^^}"; p="${p//-/_}_"
    prefixes+=("$p")
done < <(gate_kit_roots_rel)
pfxalt="$(IFS='|'; printf '%s' "${prefixes[*]}")"

declare -A defined=()
hits="$(git -C "$TOP" grep -h -E "($pfxalt)[A-Z0-9_]*" -- "${roots[@]}" ':!*.md' ':!*/gate-tests/*' 2>/dev/null)"; st=$?
[[ "$st" -le 1 ]] || { echo "check-docs-cmd: git grep failed (exit $st) building the knob set" >&2; exit 2; }
while IFS= read -r name; do
    [[ -n "$name" ]] && defined["$name"]=1
done < <(printf '%s\n' "$hits" | grep -oE "($pfxalt)[A-Z0-9_]*" || true)

knob_ok() {  # $1=knob name — exact code occurrence, or (family stem ending '_') any name under it
    local t="$1" k
    [[ -n "${defined[$t]:-}" ]] && return 0
    if [[ "$t" == *_ ]]; then
        for k in "${!defined[@]}"; do [[ "$k" == "$t"* ]] && return 0; done
    fi
    return 1
}

path_ok() {  # $1=doc dir  $2=invoked .sh token — resolves kit-relative (doc dir) or repo-relative
    local docdir="$1" tok="$2" base cand
    [[ "$tok" == *..* ]] && return 1
    for base in "$docdir" "."; do
        cand="$(realpath -m --relative-to=. -- "$base/$tok" 2>/dev/null)" || continue
        [[ -n "$cand" && "$cand" != ../* ]] || continue
        git ls-files --error-unmatch -- "$cand" >/dev/null 2>&1 && return 0
    done
    return 1
}

pfxre="^($pfxalt)"

bad=(); npath=0; nknob=0
for f in "${files[@]}"; do
    [[ -f "$f" ]] || continue
    docdir="$(dirname "$f")"
    while IFS=$'\t' read -r kind ln tok; do
        [[ -n "${kind:-}" ]] || continue
        if [[ "$kind" == A ]]; then
            npath=$((npath + 1))
            path_ok "$docdir" "$tok" \
                || bad+=("$f:$ln: invoked script '$tok' is not a tracked file")
        else
            nknob=$((nknob + 1))
            knob_ok "$tok" \
                || bad+=("$f:$ln: env knob '$tok' occurs in no tracked kit source")
        fi
    done < <(awk -v pfxre="$pfxre" '
        function emit_A(w, ln,   e) {
            e = w
            sub(/^[`"'"'"'(]+/, "", e); sub(/[`"'"'"');:,]+$/, "", e)
            if (e ~ /^\.?\/?([A-Za-z0-9._-]+\/)+[A-Za-z0-9._-]+\.sh$/)
                print "A\t" ln "\t" e
        }
        function scan_A(line, ln,   nseg, seg, i, cmd, nw, w, j, exe) {
            gsub(/&&|\|\||[;|&]/, "\x01", line)
            nseg = split(line, seg, "\x01")
            for (i = 1; i <= nseg; i++) {
                cmd = seg[i]
                sub(/^[[:space:]]+/, "", cmd)
                sub(/^[$#][[:space:]]+/, "", cmd)
                nw = split(cmd, w, /[[:space:]]+/)
                if (nw < 1 || w[1] == "") continue
                exe = w[1]
                if (exe == "bash" || exe == "sh" || exe == "source" || exe == ".") {
                    exe = ""
                    for (j = 2; j <= nw; j++) { if (w[j] ~ /^-/) continue; exe = w[j]; break }
                }
                if (exe != "") emit_A(exe, ln)
            }
        }
        function scan_B(text, ln,   s, run) {
            s = text
            while (match(s, /[A-Z][A-Z0-9_]+/)) {
                run = substr(s, RSTART, RLENGTH)
                if (run ~ pfxre) print "B\t" ln "\t" run
                s = substr(s, RSTART + RLENGTH)
            }
        }
        /^[[:space:]]*```/ { infence = !infence; next }
        {
            if (infence) { scan_A($0, FNR); scan_B($0, FNR) }
            else {
                s = $0
                while (match(s, /`[^`]*`/)) {
                    rs = RSTART; rl = RLENGTH      # scan_B'\''s match() clobbers the globals
                    scan_B(substr(s, rs + 1, rl - 2), FNR)
                    s = substr(s, rs + rl)
                }
            }
        }
    ' "$f")
done

if [[ ${#bad[@]} -gt 0 ]]; then
    echo "check-docs-cmd: unresolvable command path(s) or env knob(s) in the governed doc set:"
    printf '  %s\n' "${bad[@]}"
    echo "  help: fix the path (relative to the doc, or repo-relative) and track the script, or"
    echo "        correct the knob name. A hypothetical example goes outside a fence, or the doc"
    echo "        joins CANON_KIT_MDREF_EXCLUDE. Only invoked .sh paths and kit-prefixed knobs count."
    exit 1
fi

echo "DOCS-CMD: clean (${#files[@]} doc(s); $npath invoked path(s) + $nknob kit-prefixed knob(s) resolve)"
exit 0
