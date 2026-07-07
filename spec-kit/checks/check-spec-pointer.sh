#!/usr/bin/env bash
# graph: couples=scripts/*.sh,gate-sdk/*.sh,lifecycle-kit/*.sh,queue-kit/*.sh,spec-kit/*.sh,delegation-kit/*.sh,guard-kit/*.sh,context-kit/*.sh,.workflow/*.txt dir=one valve=none tier=precommit
# spec: spec-kit/SPEC.md §check-spec-pointer — every spec:/contract: pointer directive on a governed surface resolves: its target file is tracked and a §heading fragment names a heading the file carries
#
# usage: check-spec-pointer.sh [scan-root]
#   For every full-line spec:/contract: directive on the governed sources under
#   the root (default '.', the same surface check-comment-tier blesses by shape),
#   resolve its `<path> [§<heading>]` target — forward direction only: <path>
#   (root-relative) must be a tracked file, and a `§<heading>` fragment must name
#   a markdown heading the file carries. A pointer without `§` resolves file-only.
#   Reddens on a missing/untracked target, an absent named heading, or a directive
#   that matched the pointer shape but carries no target path (fail-closed).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-spec-pointer: not a directory: $ROOT" >&2; exit 2; }
git -C "$ROOT" rev-parse --git-dir >/dev/null 2>&1 || {
    echo "check-spec-pointer: $ROOT is not a git repository — cannot verify tracked targets" >&2; exit 2; }

# spec: spec-kit/SPEC.md §check-spec-pointer — shape-only extraction of the
# spec:/contract: directive lines across the hash (# / .txt) and slash (// , block
# *) comment surfaces; resolution happens in the shell below.
read -r -d '' EXTRACT <<'AWK' || true
{
    L = $0
    # the colon needs trailing whitespace (a real "<keyword>: <path>" directive) or
    # end-of-line (a degenerate empty directive, flagged downstream); this parts a
    # pointer from prose that merely opens with "spec:/contract:".
    if      (L ~ /^[ \t]*#+[ \t]*(spec|contract):([ \t]|$)/)   sub(/^[ \t]*#+[ \t]*/, "", L)
    else if (L ~ /^[ \t]*\/\/+[ \t]*(spec|contract):([ \t]|$)/) sub(/^[ \t]*\/\/+[ \t]*/, "", L)
    else if (L ~ /^[ \t]*\*[ \t]*(spec|contract):([ \t]|$)/)    sub(/^[ \t]*\*[ \t]*/, "", L)
    else next
    print FILENAME "\t" FNR "\t" L
}
AWK

heading_present() {  # $1=target file  $2=heading fragment -> 0 when a heading matches
    local file="$1" frag="$2" res st
    # spec: spec-kit/SPEC.md §check-spec-pointer — heading match tolerates a
    # trailing "(qualifier)" on either side: match the fragment and each heading
    # both verbatim and with a trailing "(...)" stripped.
    res="$(awk -v a="$frag" '
        function strippar(s) { sub(/[[:space:]]*\([^)]*\)[[:space:]]*$/, "", s); return s }
        BEGIN { as = strippar(a) }
        /^#{1,6}[[:space:]]/ {
            h = $0; sub(/^#{1,6}[[:space:]]+/, "", h); sub(/[[:space:]]+$/, "", h)
            hs = strippar(h)
            if (h == a || hs == a || h == as || hs == as) { found = 1 }
        }
        END { print found + 0 }
    ' "$file")"; st=$?
    fail_closed "$st" SPEC-POINTER "awk heading match"
    [[ "$res" == "1" ]]
}

errors=()
scanned=0
pointers=0
while IFS= read -r f; do
    [[ -n "$f" ]] || continue
    rel="${f#"$ROOT"/}"; rel="${rel#./}"
    spec_comment_whitelisted "$rel" && continue
    scanned=$((scanned + 1))
    out="$(awk "$EXTRACT" "$f")"; st=$?
    fail_closed "$st" SPEC-POINTER "awk extract ($rel)"
    while IFS=$'\t' read -r _file lineno body; do
        [[ -n "$body" ]] || continue
        pointers=$((pointers + 1))
        if [[ "$body" == spec:* ]]; then after="${body#spec:}"; else after="${body#contract:}"; fi
        # comment-tier-exempt: a § inside the em-dash prose tail is not a heading marker
        sig="${after%% — *}"; sig="${sig%% -- *}"
        read -r path prest <<< "$sig"
        frag=""
        if [[ "$path" == *§* ]]; then
            frag="${path#*§}"; path="${path%%§*}"
        elif [[ "$prest" == *§* ]]; then
            frag="${prest#*§}"
        fi
        frag="${frag#"${frag%%[![:space:]]*}"}"
        frag="${frag%"${frag##*[![:space:]]}"}"

        if [[ -z "$path" || "$path" == §* ]]; then
            errors+=("$rel:$lineno: pointer directive carries no target path: $body")
            continue
        fi
        if [[ ! -f "$ROOT/$path" ]]; then
            errors+=("$rel:$lineno: target file not found: $path")
            continue
        fi
        if ! git -C "$ROOT" ls-files --error-unmatch -- "$path" >/dev/null 2>&1; then
            errors+=("$rel:$lineno: target file untracked: $path")
            continue
        fi
        if [[ -n "$frag" ]] && ! heading_present "$ROOT/$path" "$frag"; then
            errors+=("$rel:$lineno: §heading not found in $path: §$frag")
        fi
    done <<< "$out"
done < <(spec_comment_surface "$ROOT")

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "SPEC-POINTER: ${#errors[@]} dangling pointer(s):"
    printf '  %s\n' "${errors[@]}"
    echo "  help: a spec:/contract: pointer binds this site to the requirement that governs it — the binding is only live if it resolves. Fix the <path> (repo-relative, tracked) or the §<heading> to name the current target, or drop the § fragment for a file-only pointer. A renamed heading updates every inbound pointer in the same commit."
    exit 1
fi
echo "SPEC-POINTER: clean ($pointers pointer(s) across $scanned governed source(s); every target file tracked and named §heading present)"
exit 0
