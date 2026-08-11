#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §The port-candidate criteria — criterion 7's roster, derived from the tree at each invocation rather than stated anywhere; a literal roster cannot be correct for every consumer because a renderer/command knob is consumer config
# usage: port-blockers.sh
#   each registered gate's external-program requirements beyond GATE_SDK_PROGRAM_FLOOR, as '<member><TAB><program><TAB><file:line>' rows plus a trailing scanned/undecidable count line — the criterion-7 input a porting session reads when it sequences a cohort.
#   advisory by construction: never joins gates.list and nothing parses the output; a requirement it cannot resolve prints '?' and is counted, never guessed.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

GATES_DIR="$(gate_sdk_gates_dir)"
LIST="$GATES_DIR/gates.list"
[[ -f "$LIST" ]] || {
    echo "port-blockers: registry not found: $LIST" >&2
    exit 2
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — the resolution dirs gate_check_dirs yields, taken through gate_kit_roots_rel so every evidence path this report prints is repo-relative: the report is read beside a diff and cited in a session note, where an absolute clone path resolves for nobody
CHECK_DIRS=("$GATES_DIR")
while IFS= read -r _pb_root; do CHECK_DIRS+=("${_pb_root%/}/checks"); done < <(gate_kit_roots_rel)
unset _pb_root

# spec: gate-sdk/SPEC.md §The port-candidate criteria — a name defined as a shell function by the gate itself or by any kit library it can source is not an external program; the set is derived from the tree, so a kit that adds a helper never has to be listed here
declare -A FUNCS=()
_pb_collect_funcs() {
    local f name
    for f in "$@"; do
        [[ -f "$f" ]] || continue
        while IFS= read -r name; do
            [[ -n "$name" ]] && FUNCS["$name"]=1
        done < <(grep -Eo '^[[:space:]]*(function[[:space:]]+)?[A-Za-z_][A-Za-z0-9_]*[[:space:]]*\(\)' "$f" |
            sed -E 's/^[[:space:]]*(function[[:space:]]+)?//; s/[[:space:]]*\(\)$//')
    done
}
while IFS= read -r _pb_kit; do
    _pb_collect_funcs "$_pb_kit"/lib/*.sh
done < <(gate_kit_roots)
unset _pb_kit

# spec: gate-sdk/SPEC.md §The port-candidate criteria — keyword/builtin status is asked of the interpreter rather than held as a list, so the classification is a property of bash and not a roster this tool maintains
declare -A BUILTIN=()
_pb_is_builtin() {
    local w="$1" t
    if [[ -z "${BUILTIN[$w]+set}" ]]; then
        t="$(type -t "$w" 2>/dev/null)"
        case "$t" in
            keyword | builtin) BUILTIN["$w"]=1 ;;
            *) BUILTIN["$w"]=0 ;;
        esac
    fi
    [[ "${BUILTIN[$w]}" == 1 ]]
}

declare -A FLOOR=()
for _pb_p in "${GATE_SDK_PROGRAM_FLOOR[@]}"; do FLOOR["$_pb_p"]=1; done
unset _pb_p

PB_SCAN='
BEGIN { cmdpos = 1; sq = 0; dq = 0; hd = ""; sp = 0; inword = 0; w = ""; wcmd = 0; clvl = 0 }

function emit(kind, word) { printf "%s\t%s\t%s\t%d\n", kind, word, FILENAME, wline }
function addc(c) { if (!inword) { inword = 1; w = ""; wcmd = cmdpos; wline = FNR }; w = w c }
function endword() {
    if (!inword) return
    inword = 0
    if (w == "]]") { dbrack = 0; cmdpos = 0; w = ""; return }
    if (dbrack) { w = ""; return }
    if (w == "esac") { if (clvl > 0) clvl--; cmdpos = 1; w = ""; return }
    if (wcmd) { probe = 0; classify(w) }
    else if (w == "in" && clvl > 0 && cst[clvl] == 1) { cst[clvl] = 2; cmdpos = 0 }
    else if (probe) probeword(w)
    w = ""
}
function probeword(t) {
    if (t == "-v" || t == "-V" || t == "-p" || t == "-P") { probe = 2; return }
    if (probe == 2) { if (t ~ /^[A-Za-z_][A-Za-z0-9_.+-]*$/) emit("guard", t); probe = 0 }
}
function push(newdq) {
    sp++; stk[sp] = dq; wsv[sp] = w; isv[sp] = inword; csv[sp] = wcmd; lsv[sp] = wline
    dq = newdq; w = ""; inword = 0
}
function pop() {
    if (sp > 0) { dq = stk[sp]; w = wsv[sp]; inword = isv[sp]; wcmd = csv[sp]; wline = lsv[sp]; sp-- }
    else dq = 0
}
function incase() { return (dbrack || (clvl > 0 && cst[clvl] == 2)) }
function skipbrace(l, k, m,   d) {
    d = 1
    while (k <= m) {
        if (substr(l, k, 1) == "{") d++
        else if (substr(l, k, 1) == "}") { d--; if (d == 0) return k + 1 }
        k++
    }
    return k
}

function classify(t,   kn) {
    if (t ~ /^[A-Za-z_][A-Za-z0-9_]*(\[[^]]*\])?\+?=/) { cmdpos = 1; return }
    if (t == "esac") { if (clvl > 0) clvl--; cmdpos = 1; return }
    if (t == "case") { clvl++; cst[clvl] = 1; cmdpos = 0; return }
    if (t == "in" && clvl > 0 && cst[clvl] == 1) { cst[clvl] = 2; cmdpos = 0; return }
    if (t == "if" || t == "then" || t == "else" || t == "elif" || t == "do" || t == "while" ||
        t == "until" || t == "!" || t == "time" || t == "fi" || t == "done" ||
        t == "{" || t == "}") { cmdpos = 1; return }
    if (t == "[[") { dbrack = 1; cmdpos = 0; return }
    if (t == "for" || t == "select" || t == "in" || t == "function" || t == "[") { cmdpos = 0; return }
    if (t ~ /^"?\$\{?[A-Za-z_][A-Za-z0-9_]*(\[[@*]\])?\}?"?$/) {
        kn = t
        gsub(/[$"{}]/, "", kn)
        gsub(/\[[@*]\]/, "", kn)
        emit("expansion", kn)
        cmdpos = 0
        return
    }
    if (t == "command") { cmdpos = 0; probe = 1; return }
    if (t ~ /^[A-Za-z_][A-Za-z0-9_.+-]*$/) { emit("cmd", t); cmdpos = 0; return }
    cmdpos = 0
}
function skiparith(l, k, m,   d) {
    d = 1
    while (k <= m) {
        if (substr(l, k, 1) == "(") d++
        else if (substr(l, k, 1) == ")") { d--; if (d == 0) return k + 1 }
        k++
    }
    return k
}
function skiparray(l, k, m,   d) {
    d = 1
    while (k <= m) {
        if (substr(l, k, 1) == "\047") { k++; while (k <= m && substr(l, k, 1) != "\047") k++ }
        else if (substr(l, k, 1) == "\"") { k++; while (k <= m && substr(l, k, 1) != "\"") k++ }
        else if (substr(l, k, 1) == "(") d++
        else if (substr(l, k, 1) == ")") { d--; if (d == 0) return k + 1 }
        k++
    }
    return k
}

{
    line = $0
    n = length(line)

    if (hd != "") {
        t = line
        if (hdstrip) sub(/^[ \t]+/, "", t)
        if (t == hd) { hd = ""; cmdpos = 1 }
        next
    }

    pend = ""
    i = 1
    while (i <= n) {
        c = substr(line, i, 1)

        if (sq) { addc(c); if (c == "\047") sq = 0; i++; continue }

        if (dq) {
            if (c == "\\") { addc(substr(line, i, 2)); i += 2; continue }
            if (c == "$" && substr(line, i + 1, 2) == "((") { i = skiparith(line, i + 3, n); continue }
            if (c == "$" && substr(line, i + 1, 1) == "{") { j = skipbrace(line, i + 2, n); addc(substr(line, i, j - i)); i = j; continue }
            if (c == "$" && substr(line, i + 1, 1) == "(") { addc("$("); push(0); cmdpos = 1; i += 2; continue }
            if (c == "`") { addc(c); push(0); cmdpos = 1; i++; continue }
            addc(c)
            if (c == "\"") dq = 0
            i++
            continue
        }

        if (c == " " || c == "\t") { endword(); i++; continue }
        if (c == "\\") {
            if (i == n) { cont = 1; i++; continue }
            addc(substr(line, i, 2)); i += 2; continue
        }
        if (c == "#" && !inword) break
        if (c == "\047") { addc(c); sq = 1; i++; continue }
        if (c == "\"") { addc(c); dq = 1; i++; continue }

        if (c == "<" && substr(line, i + 1, 1) == "<" && substr(line, i + 2, 1) != "<") {
            endword()
            j = i + 2
            hdstrip = 0
            if (substr(line, j, 1) == "-") { hdstrip = 1; j++ }
            while (substr(line, j, 1) == " ") j++
            q = substr(line, j, 1)
            d = ""
            if (q == "\047" || q == "\"") {
                j++
                while (j <= n && substr(line, j, 1) != q) { d = d substr(line, j, 1); j++ }
                j++
            } else {
                while (j <= n && substr(line, j, 1) ~ /[A-Za-z0-9_]/) { d = d substr(line, j, 1); j++ }
            }
            pend = d
            i = j
            cmdpos = 0
            continue
        }

        if (c == "$" && substr(line, i + 1, 2) == "((") { i = skiparith(line, i + 3, n); continue }
        if (c == "$" && substr(line, i + 1, 1) == "{") { j = skipbrace(line, i + 2, n); addc(substr(line, i, j - i)); i = j; continue }
        if (c == "$" && substr(line, i + 1, 1) == "(") { addc("$("); push(0); cmdpos = 1; i += 2; continue }
        if (c == "<" && substr(line, i + 1, 1) == "(") { endword(); push(0); cmdpos = 1; i += 2; continue }
        if (c == "`") { addc(c); push(0); cmdpos = 1; i++; continue }
        if (c == "(" && substr(line, i + 1, 1) == "(" && !inword) { i = skiparith(line, i + 2, n); cmdpos = 0; continue }
        if (c == "(" && inword && w ~ /[=]$/) { endword(); i = skiparray(line, i + 1, n); cmdpos = 1; continue }

        if (c == ";" && substr(line, i + 1, 1) == ";") {
            endword()
            if (clvl > 0 && cst[clvl] == 3) cst[clvl] = 2
            cmdpos = incase() ? 0 : 1
            i += 2
            continue
        }
        if (c == ";" || c == "&" || c == "|") { endword(); cmdpos = incase() ? 0 : 1; i++; continue }
        if (c == "{" || c == "(") { endword(); if (c == "(") push(0); cmdpos = 1; i++; continue }
        if (c == ")") {
            endword()
            if (incase()) { cst[clvl] = 3; cmdpos = 1; i++; continue }
            pop(); cmdpos = 0; i++; continue
        }
        if (c == "}") { endword(); cmdpos = 1; i++; continue }
        if (c == ">" || c == "<") { endword(); cmdpos = 0; i++; continue }

        addc(c)
        i++
    }
    endword()
    if (pend != "") hd = pend
    if (!sq && !dq && !cont) cmdpos = incase() ? 0 : 1
    cont = 0
}
'

scanned=0
undecidable=0
rows=""

record() {
    rows+="$1"$'\t'"$2"$'\t'"$3"$'\n'
    [[ "$2" == "?" ]] && member_undecidable=1
    return 0
}

# spec: gate-sdk/SPEC.md §The port-candidate criteria — a knob's value is resolved through lib/gate.sh's own bridge resolver, the one place a knob default is read, so this report can never disagree with what a dispatched binary is handed
knob_program() {
    local knob="$1" val
    [[ "$knob" == *[a-z]* ]] && return 1
    val="$(_gate_knob_value "$knob" port-blockers 2>/dev/null)" || return 1
    val="${val%%$'\t'*}"
    [[ -n "$val" ]] || return 1
    printf '%s\n' "$val"
}

while IFS= read -r member; do
    [[ -n "$member" ]] || continue
    scanned=$((scanned + 1))
    member_undecidable=0
    decl="$(gate_resolve "$member" "${CHECK_DIRS[@]}")" || {
        echo "port-blockers: $member is registered but resolves to no declaration path" >&2
        exit 2
    }
    # spec: gate-sdk/SPEC.md §The `# graph:` manifest — a .gate member's rule is a binary subcommand this tool cannot parse and no --needs flag answers yet, so it is counted undecidable rather than reported clean
    if [[ "$decl" == *.gate ]]; then
        record "$member" "?" "$decl (binary substrate; no --needs)"
        undecidable=$((undecidable + 1))
        continue
    fi

    declare -A seen=()
    declare -A LOCAL_FUNCS=()
    while IFS= read -r _pb_fn; do [[ -n "$_pb_fn" ]] && LOCAL_FUNCS["$_pb_fn"]=1; done < <(
        grep -Eo '^[[:space:]]*(function[[:space:]]+)?[A-Za-z_][A-Za-z0-9_]*[[:space:]]*\(\)' "$decl" |
            sed -E 's/^[[:space:]]*(function[[:space:]]+)?//; s/[[:space:]]*\(\)$//'
    )
    # spec: gate-sdk/SPEC.md §The port-candidate criteria — `declare -F <name>` is this tree's convention for dispatching an optional shell hook, the negative counterpart of the `command -v` guard: a name probed that way is a function, however the tool would otherwise classify it
    while IFS= read -r _pb_fn; do [[ -n "$_pb_fn" ]] && LOCAL_FUNCS["$_pb_fn"]=1; done < <(
        grep -Eo 'declare[[:space:]]+-F[[:space:]]+[A-Za-z_][A-Za-z0-9_]*' "$decl" |
            sed -E 's/.*[[:space:]]//'
    )
    while IFS=$'\t' read -r kind word file lineno; do
        [[ -n "$kind" ]] || continue
        prog="$word"
        evidence="$file:$lineno"
        case "$kind" in
            expansion)
                prog="$(knob_program "$word")" || {
                    [[ -n "${seen[?$word]+x}" ]] && continue
                    seen["?$word"]=1
                    record "$member" "?" "$evidence (command-position \$$word, default unresolvable)"
                    continue
                }
                evidence="$evidence (\$$word)"
                ;;
        esac
        [[ -n "${LOCAL_FUNCS[$prog]+x}" || -n "${FUNCS[$prog]+x}" ]] && continue
        [[ -n "${FLOOR[$prog]+x}" ]] && continue
        _pb_is_builtin "$prog" && continue
        [[ -n "${seen[$prog]+x}" ]] && continue
        seen["$prog"]=1
        record "$member" "$prog" "$evidence"
    done < <(awk "$PB_SCAN" "$decl")
    unset seen LOCAL_FUNCS
    [[ "$member_undecidable" -eq 1 ]] && undecidable=$((undecidable + 1))
done < <(gates_list_members "$LIST")

[[ -n "$rows" ]] && printf '%s' "$rows" | sort
printf 'port-blockers: %d member(s) scanned, %d with a requirement this report could not decide\n' \
    "$scanned" "$undecidable"
