#!/usr/bin/env bash
# graph: couples=.github/workflows/*.yml,.github/workflows/*.yaml,.github/ISSUE_TEMPLATE/*.yml,docs/_config.yml,kit:templates/*.yml,kit:templates/*.yaml dir=one valve=none tier=precommit
# install: on-surface
# spec: gate-sdk/SPEC.md §check-action-run-shell — every GitHub Actions `run:` literal block scalar in an Actions-shaped YAML file is ShellCheck-clean at -S warning under the dialect the step actually runs
#
# usage: check-action-run-shell.sh [scan-root]
#   scan-root: the walked tree (default '.').
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

SCANROOT="${1:-.}"
[[ -d "$SCANROOT" ]] || { echo "check-action-run-shell: scan root not found: $SCANROOT" >&2; exit 2; }

if ! command -v shellcheck >/dev/null 2>&1; then
    echo "check-action-run-shell: shellcheck not found on PATH — the gate cannot run." >&2
    echo "  A gate that cannot run is not clean (fail-closed)." >&2
    echo "  help: install ShellCheck (e.g. 'apt install shellcheck' / 'brew install shellcheck')." >&2
    exit 2
fi

listing="$(gate_find "$SCANROOT" -type f \( -name '*.yml' -o -name '*.yaml' \))"; st=$?
fail_closed "$st" ACTION-RUN-SHELL gate_find
mapfile -t files < <(printf '%s' "$listing")

if [[ ${#files[@]} -eq 0 ]]; then
    echo "ACTION-RUN-SHELL: clean (no YAML under $SCANROOT — 0 run: block(s) to lint)"
    exit 0
fi

WORK="$(mktemp -d)" || { echo "check-action-run-shell: could not create a scratch dir" >&2; exit 2; }
cleanup() { rm -rf "$WORK"; }
trap cleanup EXIT

# spec: gate-sdk/SPEC.md §check-action-run-shell — the extractor stays inline: a lib/ helper earns its place at a second consumer and there is none
EXTRACT='
function ind(s,   n) { n = match(s, /[^ ]/); return (n == 0) ? -1 : n - 1 }

function ghexpr(s,   out, i, j) {
    out = ""
    while ((i = index(s, "${{")) > 0) {
        out = out substr(s, 1, i - 1) "${GHEXPR}"
        s = substr(s, i + 3)
        j = index(s, "}}")
        if (j == 0) return UNBAL
        s = substr(s, j + 2)
    }
    return out s
}

function refuse(ln, what) {
    printf "R\t%d\t%s\n", ln, what
    refused = 1
    exit
}

function startblock(col, ln) {
    inblock = 1; keycol = col; bodyindent = -1; nbuf = 0; blockstart = ln
}

function endblock(   i, last, sline, path) {
    inblock = 0
    last = nbuf
    while (last > 0 && buf[last - 1] == "") last--
    if (last == 0) return
    nblk++
    path = OUTDIR "/block-" nblk ".sh"
    for (i = 0; i < last; i++) {
        sline = ghexpr(buf[i])
        if (sline == UNBAL)
            refuse(blockstart + 1 + i, "an unbalanced GitHub expression — ${{ with no closing }} on a run: body line")
        print sline > path
    }
    close(path)
    np++
    pend_n[np] = nblk; pend_line[np] = blockstart
}

function flushstep(   i) {
    for (i = 1; i <= np; i++) printf "B\t%d\t%d\t%s\n", pend_n[i], pend_line[i], stepshell
    np = 0
}

function processkey(rest, col,   v) {
    if (rest ~ /^shell:([ \t]|$)/) {
        v = rest; sub(/^shell:[ \t]*/, "", v); sub(/[ \t]*$/, "", v)
        stepshell = v
        return
    }
    if (rest !~ /^run:/) return
    v = rest; sub(/^run:[ \t]*/, "", v); sub(/[ \t]*$/, "", v)
    if (v ~ /^>/)        refuse(FNR, "a folded block scalar (run: " v ")")
    if (v ~ /^\|[0-9]/)  refuse(FNR, "an explicit block-scalar indentation indicator (run: " v ")")
    if (v ~ /^\*/)       refuse(FNR, "a YAML alias as the run: value (run: " v ")")
    if (v ~ /^&/)        refuse(FNR, "a YAML anchor on the run: value (run: " v ")")
    if (v ~ /^\|[-+]?$/) { startblock(col, FNR); return }
    if (v == "") return
    printf "P\t%d\n", FNR
}

BEGIN { stepcol = -1; np = 0; nblk = 0; UNBAL = "\001UNBAL\001" }

{
    # spec: gate-sdk/SPEC.md §check-action-run-shell — no block header is recognised while inside a block
    if (inblock) {
        if ($0 ~ /^[ \t]*$/) { buf[nbuf++] = ""; next }
        bcol = ind($0)
        if (bcol > keycol) {
            if (bodyindent < 0) bodyindent = bcol
            buf[nbuf++] = substr($0, bodyindent + 1)
            next
        }
        endblock()
    }
    if ($0 ~ /^[ \t]*$/) next
    # spec: gate-sdk/SPEC.md §check-action-run-shell — a comment line is never a header
    if ($0 ~ /^[ ]*#/) next
    # spec: gate-sdk/SPEC.md §check-action-run-shell — the key column is the column of the key token, never the list dash
    if (match($0, /^[ ]*-[ ]+/)) {
        flushstep()
        stepcol = RLENGTH
        stepshell = ""
        processkey(substr($0, stepcol + 1), stepcol)
        next
    }
    kcol = ind($0)
    if (stepcol >= 0 && kcol < stepcol) { flushstep(); stepcol = -1; stepshell = "" }
    if (stepcol >= 0 && kcol == stepcol) { processkey(substr($0, kcol + 1), kcol); next }
    if (stepcol < 0) {
        tail = substr($0, kcol + 1)
        if (tail ~ /^run:/) { stepcol = kcol; stepshell = ""; processkey(tail, kcol) }
    }
}

END { if (!refused) { if (inblock) endblock(); flushstep() } }
'

# spec: gate-sdk/SPEC.md §check-action-run-shell — absent resolves to bash on GitHub's documented runner default; a dialect ShellCheck has no theory of is skipped and counted, never linted as shell
dialect_of() {
    local raw="$1" first
    first="${raw%%[[:space:]]*}"
    first="${first#\"}"; first="${first%\"}"
    first="${first#\'}"; first="${first%\'}"
    case "$first" in
        "")               printf 'bash' ;;
        bash)             printf 'bash' ;;
        sh|dash|ksh)      printf '%s' "$first" ;;
        *)                printf '' ;;
    esac
}

walked=0; subject=0; skipped_files=0; linted=0; plain=0; skipped_dialect=0
findings=()

for f in "${files[@]}"; do
    [[ -n "$f" ]] || continue
    walked=$((walked + 1))
    # spec: gate-sdk/SPEC.md §check-action-run-shell — the Actions-shape predicate governs extraction and refusal alike: a file it skips is neither linted nor refused, because `run:` is an ordinary word serving as a key in more than one CI schema
    if ! grep -qE '^(jobs|runs):' "$f"; then
        skipped_files=$((skipped_files + 1))
        continue
    fi
    subject=$((subject + 1))

    fdir="$WORK/f$subject"
    mkdir -p "$fdir"
    index="$(awk -v OUTDIR="$fdir" "$EXTRACT" "$f")"; st=$?
    fail_closed "$st" ACTION-RUN-SHELL "awk extract($f)"

    while IFS=$'\t' read -r kind a b c; do
        case "$kind" in
            R)
                echo "check-action-run-shell: the extractor met a construct it does not handle, so it" >&2
                echo "refuses rather than linting a mangled fragment (fail-closed):" >&2
                echo "  $f:$a: $b" >&2
                echo "  help: a multi-line run: body in an Actions-shaped file must be a literal block" >&2
                echo "        scalar written 'run: |' (or '|-' / '|+'), with no explicit indentation" >&2
                echo "        indicator and no YAML anchor or alias, and every \${{ }} on a body line" >&2
                echo "        balanced." >&2
                exit 2
                ;;
            P) plain=$((plain + 1)) ;;
            B)
                dialect="$(dialect_of "$c")"
                if [[ -z "$dialect" ]]; then
                    skipped_dialect=$((skipped_dialect + 1))
                    continue
                fi
                linted=$((linted + 1))
                frag="$fdir/block-$a.sh"
                out="$(shellcheck -f gcc -S warning -s "$dialect" "$frag" 2>&1)"; rc=$?
                [[ "$rc" -le 1 ]] || { echo "check-action-run-shell: shellcheck exited $rc on $f (block at line $b)" >&2; exit 2; }
                [[ "$rc" -eq 0 ]] && continue
                while IFS= read -r hit; do
                    [[ -n "$hit" ]] || continue
                    rest="${hit#"$frag":}"
                    fline="${rest%%:*}"
                    if [[ "$fline" =~ ^[0-9]+$ ]]; then
                        findings+=("$f:$((b + fline)):${rest#*:}")
                    else
                        findings+=("$f (run: block at line $b): $hit")
                    fi
                done <<< "$out"
                ;;
        esac
    done <<< "$index"
done

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-action-run-shell: ShellCheck finding(s) in a workflow run: block — nothing else"
    echo "in the battery reaches this shell, and it executes only on a tag or a push:"
    printf '  %s\n' "${findings[@]}"
    echo "  help: fix each finding in the workflow's run: body (the line numbers are the"
    echo "        workflow's own), or silence a genuine false positive with an inline"
    echo "        '# shellcheck disable=SCxxxx' plus a justifying comment."
    exit 1
fi

echo "ACTION-RUN-SHELL: clean ($linted run: block(s) linted at -S warning across $subject Actions-shaped file(s) of $walked walked; $skipped_files file(s) skipped by the Actions-shape predicate, $plain plain-scalar run: value(s) skipped, $skipped_dialect block(s) skipped on a non-shell dialect)"
exit 0
