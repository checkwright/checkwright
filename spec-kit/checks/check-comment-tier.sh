#!/usr/bin/env bash
# graph: couples=scripts/*.sh,gate-sdk/*.sh,lifecycle-kit/*.sh,queue-kit/*.sh,spec-kit/*.sh,delegation-kit/*.sh,guard-kit/*.sh,context-kit/*.sh,drift-kit/*.sh,.workflow/*.txt dir=one valve=none tier=precommit
# spec: spec-kit/SPEC.md §check-comment-tier — every full-line comment on a governed surface is a machine/reason directive, rides a directive's bounded window, is comment-tier-exempt, or justifies a positional construct
#
# usage: check-comment-tier.sh [scan-root]
#   Classifies the full-line comments on every governed source under scan-root
#   (default '.'); the tiering rule it enforces is the §check-comment-tier spec.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-comment-tier: not a directory: $ROOT" >&2; exit 2; }

ere_escape() {  # bracket classes, not backslashes: survive awk's -v escape processing
    local s="$1" out="" i c
    for ((i = 0; i < ${#s}; i++)); do
        c="${s:i:1}"
        case "$c" in
            '(') out+='[(]' ;;
            ')') out+='[)]' ;;
            '[') out+='[[]' ;;
            ']') out+='[]]' ;;
            '.') out+='[.]' ;;
            '*') out+='[*]' ;;
            '+') out+='[+]' ;;
            '?') out+='[?]' ;;
            '|') out+='[|]' ;;
            '{') out+='[{]' ;;
            '}') out+='[}]' ;;
            '$') out+='[$]' ;;
            *)   out+="$c" ;;
        esac
    done
    printf '%s' "$out"
}

join_alt() {  # a '|'-joined ERE alternation of bracket-escaped literals
    local out="" n
    for n in "$@"; do
        [[ -z "$n" ]] && continue
        out+="${out:+|}$(ere_escape "$n")"
    done
    printf '%s' "$out"
}

# spec: spec-kit/SPEC.md §check-comment-tier — the built-in kit-mechanism roster
# (Checkwright's own directive names); the SPEC_KIT_COMMENT_* knobs append a
# consumer's extras.
shell_colon=(graph: spec: contract: usage: exception-list: no-fixture: permanent: comment-tier-exempt: "TODO(task:" "TODO(spec-ambiguity)")
# shellcheck disable=SC2034  # nameref-consumed by build_bless
shell_word=(shellcheck assertion)
shell_colon+=("${SPEC_KIT_COMMENT_MACHINE[@]}" "${SPEC_KIT_COMMENT_REASON[@]}")

# The state-file (.txt) surface blesses only contract:/see headers (plus the
# universal exempt escape).
txt_colon=(contract: comment-tier-exempt:)
# shellcheck disable=SC2034  # nameref-consumed by build_bless
txt_word=(see)
txt_colon+=("${SPEC_KIT_COMMENT_MACHINE[@]}" "${SPEC_KIT_COMMENT_REASON[@]}")

build_bless() {  # $1=name of colon array  $2=name of word array -> prints ERE
    local -n colons="$1" words="$2"
    local lit wrd re=""
    lit="$(join_alt "${colons[@]}")"
    wrd="$(join_alt "${words[@]}")"
    [[ -n "$lit" ]] && re="($lit)"
    [[ -n "$wrd" ]] && re="${re:+$re|}(^|[^[:alnum:]])($wrd)([^[:alnum:]]|\$)"
    printf '%s' "$re"
}

SHELL_BLESS="$(build_bless shell_colon shell_word)"
TXT_BLESS="$(build_bless txt_colon txt_word)"
POS_RE="$(join_alt "${SPEC_KIT_COMMENT_POSITIONAL[@]}")"

# spec: spec-kit/SPEC.md §check-comment-tier — the classifier walks full-line
# comments (heredoc/block bodies skipped); a directive opens a CAP-wide window
# blessing its line plus continuations, and POS rescues a block above a construct.
read -r -d '' CLASSIFY <<'AWK' || true
function trim(s) { gsub(/^[ \t]+|[ \t]+$/, "", s); return s }
function flush_block(rescue,   i) {
    if (nb == 0) return
    if (rescue && nb >= 1 && bflag[nb]) bflag[nb] = 0
    for (i = 1; i <= nb; i++)
        if (bflag[i]) print bfile[i] ":" bline[i] ": " btext[i]
    nb = 0; window = 0
}
function detect_hd(   s, m, ch) {
    s = $0
    if (s !~ /<</) return
    gsub(/<<</, "\001", s)
    if (s !~ /<</) return
    m = s; sub(/.*<<-?[ \t]*/, "", m)
    ch = substr(m, 1, 1)
    if (ch == SQ || ch == DQ) m = substr(m, 2)
    if (match(m, /^[A-Za-z_][A-Za-z0-9_]*/)) hd = substr(m, 1, RLENGTH)
}
function record(bodytext,   blessed) {
    nb++
    bfile[nb] = FILENAME; bline[nb] = FNR; btext[nb] = trim(bodytext)
    if (bodytext ~ BLESS) window = CAP
    blessed = (window > 0)
    if (window > 0) window--
    bflag[nb] = blessed ? 0 : 1
}
function noncomment(   ispos, isblank) {
    isblank = ($0 ~ /^[ \t]*$/)
    ispos = (POS != "" && !isblank && $0 ~ POS)
    flush_block(ispos ? 1 : 0)
    if (STYLE == "hash") detect_hd()
}
FNR == 1 { flush_block(0); hd = ""; inblock = 0 }
STYLE == "hash" && hd != "" {
    if (trim($0) == hd) hd = ""
    flush_block(0); next
}
STYLE == "hash" && FNR == 1 && /^#!/ { noncomment(); next }
STYLE == "hash" {
    if ($0 ~ /^[ \t]*#/) { b = $0; sub(/^[ \t]*#/, "", b); record(b) }
    else noncomment()
    next
}
# --- slash (//, /* */) surface ---
STYLE == "slash" && inblock {
    b = $0
    if (b ~ /\*\//) { sub(/\*\/.*/, "", b); inblock = 0 }
    sub(/^[ \t]*\*?/, "", b)
    record(b)
    if (!inblock && $0 ~ /\*\/[ \t]*[^ \t]/) { sub(/.*\*\//, "", $0); noncomment() }
    next
}
STYLE == "slash" {
    if ($0 ~ /^[ \t]*\/\//) { b = $0; sub(/^[ \t]*\/\/+/, "", b); record(b); next }
    if ($0 ~ /^[ \t]*\/\*/) {
        b = $0; sub(/^[ \t]*\/\*/, "", b)
        if (b ~ /\*\//) sub(/\*\/.*/, "", b); else inblock = 1
        record(b); next
    }
    noncomment(); next
}
END { flush_block(0) }
AWK

# spec: spec-kit/SPEC.md §check-comment-tier — the governed surface (shared with
# check-spec-pointer via lib/spec.sh: SPEC_KIT_COMMENT_SURFACE globs, else derived
# shell sources plus the workflow *.txt state files).
declare -a SURFACE=()
while IFS= read -r f; do
    [[ -n "$f" ]] && SURFACE+=("$f")
done < <(spec_comment_surface "$ROOT")

errors=()
scanned=0
for f in "${SURFACE[@]}"; do
    rel="${f#"$ROOT"/}"; rel="${rel#./}"
    spec_comment_whitelisted "$rel" && continue
    case "$f" in
        *.sh|*.bash)              style="hash";  bless="$SHELL_BLESS"; pos="$POS_RE" ;;
        *.txt)                    style="hash";  bless="$TXT_BLESS";   pos="" ;;
        *.rs|*.ts|*.tsx|*.js|*.go|*.c|*.h|*.rego) style="slash"; bless="$SHELL_BLESS"; pos="$POS_RE" ;;
        *)                        style="hash";  bless="$SHELL_BLESS"; pos="$POS_RE" ;;
    esac
    scanned=$((scanned + 1))
    out="$(awk -v STYLE="$style" -v BLESS="$bless" -v POS="$pos" -v CAP="$SPEC_KIT_COMMENT_RUN_CAP" -v SQ="'" -v DQ='"' "$CLASSIFY" "$f")"; st=$?
    fail_closed "$st" COMMENT-TIER "awk classifier ($rel)"
    while IFS= read -r line; do
        [[ -n "$line" ]] && errors+=("${line#"$ROOT"/}")
    done <<< "$out"
done

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "COMMENT-TIER: ${#errors[@]} violation(s):"
    printf '  %s\n' "${errors[@]}"
    echo "  help: code is the WHAT, its SPEC the WHY — lead the block with a bare 'spec: <SPEC> §<section>' pointer (or another roster directive), delete prose that restates the code or paraphrases the SPEC it points at, or tag '# comment-tier-exempt: <reason>' for a genuinely-local fact below SPEC altitude. A directive blesses only its own window (its line plus continuations up to SPEC_KIT_COMMENT_RUN_CAP=$SPEC_KIT_COMMENT_RUN_CAP physical comment lines, blank '#' lines included); prose beyond the window re-anchors on its own directive, trims, or exempts. Not-yet-swept components ride the COMMENT_TIER_WHITELIST with a '# until:' drain task."
    exit 1
fi
echo "COMMENT-TIER: clean ($scanned governed source(s); every full-line comment is a directive, rides a directive window, is exempt, or justifies a positional construct)"
exit 0
