#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §The port-candidate criteria — criterion 7's roster, derived from the tree at each invocation rather than stated anywhere; a literal roster cannot be correct for every consumer because a renderer/command knob is consumer config
# spec: gate-sdk/SPEC.md §port-blockers — criterion 6's roster on the same tool and the same walk: the --group arm partitions the still-shell members by derived corpus derivation, so one tool is the derived roster for both criteria and neither is a maintained list
# usage: port-blockers.sh [--group]
#   default arm: each registered gate's external-program requirements beyond GATE_SDK_PROGRAM_FLOOR, as '<member><TAB><program><TAB><file:line>' rows plus a trailing scanned/undecidable count line — the criterion-7 input a porting session reads when it sequences a cohort; advisory, never parsed, and what it cannot decide prints '?' and is counted.
#   --group: the corpus-derivation partition over the still-shell members, groups largest first, each member carrying its criterion 2/3/7 columns and its expanded couples= — the criterion-6 input the session cutting the next cohort reads, advisory on the same terms and with an undecidable count of its own.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

pb_usage() {
    cat <<'EOF'
usage: port-blockers.sh [--group]

  (no argument)  criterion 7: every registered gate's external-program
                 requirements beyond GATE_SDK_PROGRAM_FLOOR, one
                 '<member><TAB><program><TAB><file:line>' row each.
  --group        criterion 6: the corpus-derivation partition over the
                 still-shell members, largest group first, each member
                 carrying its criterion 2/3/7 columns and expanded couples=.
  -h, --help     this text.

Both arms are advisory: nothing parses either output, and what cannot be
decided prints '?' and is counted rather than guessed.
EOF
}

# spec: gate-sdk/SPEC.md §port-blockers — the two argument behaviors the tool adopts from §The bin/-tool contract now that it has a mode; the tool takes no positionals, so a non-option word is unrecognized on the same footing as an unknown flag
MODE=default
case "${1-}" in
    "") ;;
    --group) MODE=group ;;
    -h | --help)
        pb_usage
        exit 0
        ;;
    *)
        pb_usage >&2
        exit 2
        ;;
esac
if [[ $# -gt 1 ]]; then
    pb_usage >&2
    exit 2
fi

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

# spec: gate-sdk/SPEC.md §port-blockers — criterion 2's column reads the fixture dirs check-gate-fixture-coverage resolves, in that order, so the report and the gate can never disagree about whether a member carries a pair
TESTS_DIRS=("${GATE_SDK_TESTS_DIR:-$GATES_DIR/gate-tests}")
while IFS= read -r _pb_root; do TESTS_DIRS+=("${_pb_root%/}/gate-tests"); done < <(gate_kit_roots_rel)
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

declare -A GROUP_ROWS=()
declare -A GROUP_COUNT=()
group_unkeyed=""
group_unkeyed_n=0
ported_excluded=0

record() {
    rows+="$1"$'\t'"$2"$'\t'"$3"$'\n'
    if [[ "$2" == "?" ]]; then
        member_undecidable=1
    else
        member_progs+="$2,"
    fi
    return 0
}

# spec: gate-sdk/SPEC.md §port-blockers — the content-glob factor is read from the declaration's non-comment lines only, which is what keeps the `# graph:` manifest out of the key: couples= is a printed cross-check and never a key factor
pb_glob_set() {
    local set
    set="$(grep -v '^[[:space:]]*#' "$1" | grep -Eo '\*\.[A-Za-z0-9]+' | sort -u | tr '\n' ',')"
    printf '%s\n' "${set%,}"
}

pb_criterion2() {
    local m="$1" decl="$2" t
    for t in "${TESTS_DIRS[@]}"; do
        [[ -d "$t/$m/good" && -d "$t/$m/bad" ]] && { printf 'pair\n'; return 0; }
    done
    grep -Eq '^# no-fixture:' "$decl" && { printf 'no-fixture\n'; return 0; }
    printf 'none\n'
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
    member_progs=""
    decl="$(gate_resolve "$member" "${CHECK_DIRS[@]}")" || {
        echo "port-blockers: $member is registered but resolves to no declaration path" >&2
        exit 2
    }
    # spec: gate-sdk/SPEC.md §The `# graph:` manifest — a .gate member's rule is a binary subcommand this tool cannot parse and no --needs flag answers yet, so it is counted undecidable rather than reported clean
    # spec: gate-sdk/SPEC.md §port-blockers — a ported member leaves the partition entirely rather than printing '?': the grouping exists to order the *remaining* corpus, so there is no open question to report, which is the deliberate divergence from the default arm's undecidable treatment
    if [[ "$decl" == *.gate ]]; then
        if [[ "$MODE" == group ]]; then
            ported_excluded=$((ported_excluded + 1))
            continue
        fi
        record "$member" "?" "$decl (binary substrate; no --needs)"
        undecidable=$((undecidable + 1))
        continue
    fi

    declare -A seen=()
    declare -A LOCAL_FUNCS=()
    declare -A libcalls=()
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
        # spec: gate-sdk/SPEC.md §port-blockers — the kit-library call factor is exactly this filter inverted: a name the default arm discards because it is not an external program is what the grouping key is made of, so no roster of "corpus primitives" is maintained anywhere
        [[ -n "${LOCAL_FUNCS[$prog]+x}" || -n "${FUNCS[$prog]+x}" ]] && {
            libcalls["$prog"]=1
            continue
        }
        [[ -n "${FLOOR[$prog]+x}" ]] && continue
        _pb_is_builtin "$prog" && continue
        [[ -n "${seen[$prog]+x}" ]] && continue
        seen["$prog"]=1
        record "$member" "$prog" "$evidence"
    done < <(awk "$PB_SCAN" "$decl")

    if [[ "$MODE" == group ]]; then
        libkey=""
        if [[ ${#libcalls[@]} -gt 0 ]]; then
            libkey="$(printf '%s\n' "${!libcalls[@]}" | sort -u | tr '\n' ',')"
            libkey="${libkey%,}"
        fi
        globkey="$(pb_glob_set "$decl")"
        # spec: gate-sdk/SPEC.md §port-blockers — a member empty in both factors is reported, never grouped, and never grouped with another empty-keyed member: sharing an absence of evidence is not sharing a derivation
        if [[ -z "$libkey" && -z "$globkey" ]]; then
            group_unkeyed+="  ?  $member"$'\t'"$decl"$'\n'
            group_unkeyed_n=$((group_unkeyed_n + 1))
        else
            if [[ "$member_undecidable" -eq 1 ]]; then
                c7="?"
            elif [[ -z "$member_progs" ]]; then
                c7="clean"
            else
                c7="${member_progs%,}"
            fi
            c2="$(pb_criterion2 "$member" "$decl")"
            c3="$(gate_manifest_field "$decl" tier)"
            gate_expand_couples_var couples_exp "$(gate_manifest_field "$decl" couples)"
            key="libs=${libkey:--} globs=${globkey:--}"
            GROUP_ROWS["$key"]+="$(printf '  %-36s c2=%-10s c3=%-9s c7=%s\n      couples=%s' \
                "$member" "$c2" "${c3:--}" "$c7" "${couples_exp:--}")"$'\n'
            GROUP_COUNT["$key"]=$((${GROUP_COUNT["$key"]:-0} + 1))
        fi
    fi

    unset seen LOCAL_FUNCS libcalls
    [[ "$member_undecidable" -eq 1 ]] && undecidable=$((undecidable + 1))
done < <(gates_list_members "$LIST")

if [[ "$MODE" == group ]]; then
    groups=0
    while IFS=$'\t' read -r n key; do
        [[ -n "$key" ]] || continue
        groups=$((groups + 1))
        printf 'group %d: %d member(s)\n  key: %s\n' "$groups" "$n" "$key"
        printf '%s' "${GROUP_ROWS["$key"]}"
        printf '\n'
    done < <(
        for key in "${!GROUP_ROWS[@]}"; do
            printf '%d\t%s\n' "${GROUP_COUNT["$key"]}" "$key"
        done | sort -t$'\t' -k1,1nr -k2,2
    )
    if [[ -n "$group_unkeyed" ]]; then
        printf 'undecidable (no kit-library call and no content glob this tool can see):\n'
        printf '%s\n' "${group_unkeyed%$'\n'}"
        printf '\n'
    fi
    printf 'port-blockers --group: %d member(s) scanned, %d group(s) formed, %d undecidable, %d already ported and excluded\n' \
        "$scanned" "$groups" "$group_unkeyed_n" "$ported_excluded"
    exit 0
fi

[[ -n "$rows" ]] && printf '%s' "$rows" | sort
printf 'port-blockers: %d member(s) scanned, %d with a requirement this report could not decide\n' \
    "$scanned" "$undecidable"
