#!/usr/bin/env bash
# graph: couples=*SPEC*.md,*README.md,CLAUDE.md,.gitignore dir=one valve=none tier=precommit
# spec: canon-kit/SPEC.md §check-tracking-claim — every fixed-vocabulary tracking claim on a governed manifest surface agrees with git
#
# usage: check-tracking-claim.sh [scan-root]   (default '.')
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-tracking-claim: not a directory: $ROOT" >&2; exit 2; }
git -C "$ROOT" rev-parse --git-dir >/dev/null 2>&1 || {
    echo "check-tracking-claim: $ROOT is not a git repository — a tracking claim is unverifiable" >&2; exit 2; }

# spec: canon-kit/SPEC.md §check-tracking-claim — shape-only extraction over the
# blank-line paragraph join: a predicate outside inline code, bound by adjacency
# to the backticked path it follows; verification happens in the shell below.
read -r -d '' EXTRACT <<'AWK' || true
function blank_code(s,   out, i, c, incode) {
    out = ""; incode = 0
    for (i = 1; i <= length(s); i++) {
        c = substr(s, i, 1)
        if (c == "`") { incode = !incode; out = out " "; continue }
        out = out (incode ? " " : c)
    }
    return out
}
function is_pathish(t) {
    if (t !~ /^[A-Za-z0-9._~-]+(\/[A-Za-z0-9._~-]+)*\/?$/) return 0
    return (index(t, "/") > 0 || index(t, ".") > 0)
}
function bound_path(prefix,   p, i, tok) {
    # the binding is adjacency: only whitespace may sit between the path's
    # closing backtick and the predicate, so an intervening clause unbinds it
    p = prefix
    sub(/[[:space:]]+$/, "", p)
    if (substr(p, length(p), 1) != "`") return ""
    p = substr(p, 1, length(p) - 1)
    i = length(p)
    while (i >= 1 && substr(p, i, 1) != "`") i--
    if (i < 1) return ""
    tok = substr(p, i + 1)
    return is_pathish(tok) ? tok : ""
}
function flush(   i, joined, scan, rest, off, ms, me, pred, prefix, path, li) {
    if (np == 0) return
    joined = ""
    for (i = 1; i <= np; i++) { lstart[i] = length(joined) + 1; joined = joined (i > 1 ? " " : "") ptext[i] }
    scan = blank_code(joined)
    rest = scan; off = 0
    while (match(rest, PRED) > 0) {
        ms = off + RSTART; me = ms + RLENGTH - 1
        pred = substr(scan, ms, RLENGTH)
        sub(/^[^A-Za-z]*is[[:space:]]+/, "", pred)
        sub(/[^A-Za-z-]+$/, "", pred)
        prefix = substr(joined, 1, ms - 1)
        path = bound_path(prefix)
        if (path != "") {
            li = 1
            for (i = 1; i <= np; i++) if (lstart[i] <= ms) li = i
            printf "%s\t%d\t%s\t%s\n", cf, pfnr[li], path, pred
        }
        off = me; rest = substr(scan, me + 1)
    }
    np = 0
}
FNR == 1 { flush(); fence = 0 }
{
    cf = FILENAME
    if ($0 ~ /^[[:space:]]*```/) { flush(); fence = !fence; next }
    if (fence) { flush(); next }
    if ($0 ~ /^[[:space:]]*$/) { flush(); next }
    np++; pfnr[np] = FNR; ptext[np] = $0
}
END { flush() }
AWK

PRED='(^|[^A-Za-z])is[[:space:]]+(committed|tracked|gitignored|local-only|two-tier)([^A-Za-z]|$)'

mapfile -t manifest_files < <(spec_manifest_files "$ROOT")
if [[ ${#manifest_files[@]} -eq 0 ]]; then
    echo "TRACKING-CLAIM: clean (0 governed manifest surface(s))"
    exit 0
fi

out="$(awk -v PRED="$PRED" "$EXTRACT" "${manifest_files[@]}")"; st=$?
fail_closed "$st" TRACKING-CLAIM "awk claim extract"

errors=()
claims=0
while IFS=$'\t' read -r file lineno path pred; do
    [[ -n "$path" ]] || continue
    claims=$((claims + 1))
    rel="${file#"$ROOT"/}"; rel="${rel#./}"

    tracked="$(git -C "$ROOT" ls-files -- "$path")"; gst=$?
    [[ "$gst" -eq 0 ]] || { echo "check-tracking-claim: git ls-files failed for '$path'" >&2; exit 2; }
    ntracked=0; [[ -n "$tracked" ]] && ntracked=1

    # spec: canon-kit/SPEC.md §check-tracking-claim — the ignored side is
    #   rule-based (check-ignore --no-index), not presence-based, so it resolves
    #   in a fresh checkout; that section owns why --no-index is load-bearing.
    nignored=0
    git -C "$ROOT" check-ignore -q --no-index -- "$path" && nignored=1

    if [[ "$ntracked" == 0 && "$nignored" == 0 && ! -e "$ROOT/$path" ]]; then
        errors+=("$rel:$lineno: '$path is $pred' — the path is in neither the index nor the ignore rules nor the working tree, so the claim is unverifiable")
        continue
    fi

    case "$pred" in
        committed|tracked)
            [[ "$ntracked" == 1 && "$nignored" == 0 ]] \
                || errors+=("$rel:$lineno: '$path is $pred' is false — tracked members: $ntracked, ignored members: $nignored") ;;
        gitignored|local-only)
            [[ "$ntracked" == 0 && "$nignored" == 1 ]] \
                || errors+=("$rel:$lineno: '$path is $pred' is false — tracked members: $ntracked, ignored members: $nignored") ;;
        two-tier)
            [[ "$ntracked" == 1 && "$nignored" == 1 ]] \
                || errors+=("$rel:$lineno: '$path is $pred' is false — tracked members: $ntracked, ignored members: $nignored") ;;
    esac
done <<<"$out"

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "check-tracking-claim: ${#errors[@]} prose claim(s) git disagrees with:"
    printf '  %s\n' "${errors[@]}"
    echo "  help: git owns a path's tracking status, so prose states the rule and this gate verifies it. Reword the sentence to the predicate git actually supports — 'is committed'/'is tracked' (every member tracked, none ignored), 'is gitignored'/'is local-only' (no member tracked), or 'is two-tier' (both classes non-empty) — or fix the tracking. There is no per-site valve: a claim that cannot be made true is a claim that must be reworded."
    exit 1
fi
echo "TRACKING-CLAIM: clean ($claims tracking claim(s) across ${#manifest_files[@]} manifest surface(s); every predicate agrees with git)"
exit 0
