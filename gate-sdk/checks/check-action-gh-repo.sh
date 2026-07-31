#!/usr/bin/env bash
# graph: couples=.github/workflows/*.yml,.github/workflows/*.yaml,.github/ISSUE_TEMPLATE/*.yml,docs/_config.yml,kit:templates/*.yml,kit:templates/*.yaml dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-action-gh-repo — a job whose run: bodies invoke gh establishes a repository context: a checkout before the first call, GH_REPO in scope, or --repo on every call
#
# usage: check-action-gh-repo.sh [scan-root]
#   scan-root: the walked tree (default '.'). Requires GNU awk (3-arg match).
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

SCANROOT="${1:-.}"
[[ -d "$SCANROOT" ]] || { echo "check-action-gh-repo: scan root not found: $SCANROOT" >&2; exit 2; }

listing="$(gate_find "$SCANROOT" -type f \( -name '*.yml' -o -name '*.yaml' \))"; st=$?
fail_closed "$st" ACTION-GH-REPO gate_find
mapfile -t files < <(printf '%s' "$listing")

if [[ ${#files[@]} -eq 0 ]]; then
    echo "ACTION-GH-REPO: clean (no YAML under $SCANROOT — 0 job(s) to check)"
    exit 0
fi

WALK='
function ind(s,   n) { n = match(s, /[^ ]/); return (n == 0) ? -1 : n - 1 }

function keyof(s,   t) {
    t = s
    sub(/^[ ]*/, "", t)
    if (t !~ /^[^ :]+[ ]*:/) return ""
    sub(/[ ]*:.*$/, "", t)
    return t
}

# spec: gate-sdk/SPEC.md §check-action-gh-repo — the trigger and the --repo arm share one detector, so the arm is universally quantified over the detected set rather than satisfied by a witness
function scanlogical(s, ln,   t, n, i, c, prefix, w, ok, ext, j, cut, hasrepo) {
    t = s
    sub(/^[ \t]+/, "", t)
    if (substr(t, 1, 1) == "#") return
    n = length(s)
    for (i = 1; i < n; i++) {
        if (substr(s, i, 2) != "gh") continue
        c = substr(s, i + 2, 1)
        if (c ~ /[A-Za-z0-9_.-]/) continue
        prefix = substr(s, 1, i - 1)
        sub(/[ \t]+$/, "", prefix)
        ok = 0
        if (prefix == "") ok = 1
        else {
            c = substr(prefix, length(prefix))
            if (index("|&;(`!{", c) > 0) ok = 1
            else {
                w = prefix
                sub(/^.*[ \t]/, "", w)
                if (w == "then" || w == "else" || w == "do" || w == "elif") ok = 1
            }
        }
        if (!ok) continue
        ext = substr(s, i + 2)
        cut = length(ext)
        for (j = 1; j <= length(ext); j++) {
            c = substr(ext, j, 1)
            if (c == ";" || c == "|" || c == "&") { cut = j - 1; break }
        }
        ext = substr(ext, 1, cut)
        hasrepo = (ext ~ /(^|[ \t])--repo([ \t=]|$)/) ? 1 : 0
        printf "G\t%d\t%d\n", ln, hasrepo
    }
}

# spec: gate-sdk/SPEC.md §check-action-gh-repo — backslash continuations are joined before matching, so a call split across lines is one unit and its --repo is found wherever on the call it sits
function endrun(   i, s, acc, accln) {
    inrun = 0
    acc = ""; accln = 0
    for (i = 0; i < rn; i++) {
        s = rbuf[i]
        if (acc == "") accln = rln[i]
        if (s ~ /\\$/) { sub(/\\$/, "", s); acc = acc s " "; continue }
        scanlogical(acc s, accln)
        acc = ""
    }
    if (acc != "") scanlogical(acc, accln)
    rn = 0
}

# spec: gate-sdk/SPEC.md §check-action-gh-repo — the marker binds by its own indentation: at or left of the job-id column it precedes a job, at the step dash column it precedes a step, inside a step it binds that step
function marker(line, ln,   c, r) {
    if (line !~ /gh-repo-exempt/) return
    r = ""
    if (line ~ /gh-repo-exempt[ \t]*:/) {
        r = line
        sub(/^.*gh-repo-exempt[ \t]*:[ \t]*/, "", r)
        sub(/[ \t]+$/, "", r)
    }
    if (r == "") { printf "XBAD\t%d\n", ln; return }
    if (!injobs) return
    c = ind(line)
    if (jobcol >= 0 && c <= jobcol) { pendjob = r; return }
    if (curjob == "") { pendjob = r; return }
    if (insteps) {
        if (stepdashcol < 0 || c <= stepdashcol) { pendstep = r; return }
        if (stepcol >= 0) { printf "SX\t%s\n", r; return }
        pendstep = r
        return
    }
    printf "JX\t%s\n", r
}

function stepkey(rest, col, ln,   k, v, u) {
    k = keyof(rest)
    if (k == "") return
    v = rest
    sub(/^[^:]*:[ \t]*/, "", v)
    sub(/[ \t]+$/, "", v)
    if (k == "uses") {
        gsub(/^["\047]+|["\047]+$/, "", v)
        sub(/[ \t].*$/, "", v)
        u = v
        sub(/@.*$/, "", u)
        if (u == "actions/checkout") printf "C\t%d\n", ln
        return
    }
    if (k == "env") { envscope = "step"; envcol = col; return }
    if (k != "run") return
    # spec: gate-sdk/SPEC.md §check-action-gh-repo — a folded body is scanned line-wise like a literal one, and a plain scalar as a single logical line: both over-detect, the stated safe direction
    if (v ~ /^[|>][-+]?[0-9]*$/) { inrun = 1; runkeycol = col; rbi = -1; rn = 0; return }
    if (v != "") scanlogical(v, ln)
}

BEGIN { jobcol = -1; jobkeycol = -1; stepcol = -1; stepdashcol = -1; envscope = ""; curjob = ""; pendjob = ""; pendstep = ""; inrun = 0; injobs = 0; insteps = 0 }

{
    line = $0
    sub(/\r$/, "", line)

    if (inrun) {
        if (line ~ /^[ \t]*$/) { rbuf[rn] = ""; rln[rn] = FNR; rn++; next }
        c = ind(line)
        if (c > runkeycol) {
            if (rbi < 0) rbi = c
            rbuf[rn] = substr(line, rbi + 1); rln[rn] = FNR; rn++
            next
        }
        endrun()
    }

    if (line ~ /^[ \t]*$/) next
    if (line ~ /^[ ]*#/) { marker(line, FNR); next }

    c = ind(line)

    if (envscope != "") {
        if (c > envcol) {
            if (line ~ /^[ ]*GH_REPO[ ]*:/) print (envscope == "workflow" ? "W" : (envscope == "job" ? "JE" : "SE"))
            next
        }
        envscope = ""
    }

    if (c == 0) {
        topkey = keyof(line)
        injobs = (topkey == "jobs")
        curjob = ""; stepcol = -1; insteps = 0
        if (topkey == "env") { envscope = "workflow"; envcol = 0 }
        next
    }

    if (!injobs) next
    if (jobcol < 0) jobcol = c

    if (c == jobcol) {
        curjob = keyof(line)
        if (curjob == "") next
        printf "J\t%s\t%d\n", curjob, FNR
        if (pendjob != "") { printf "JX\t%s\n", pendjob }
        pendjob = ""; pendstep = ""
        jobkeycol = -1; stepcol = -1; stepdashcol = -1; insteps = 0
        next
    }
    if (curjob == "") next
    if (jobkeycol < 0) jobkeycol = c

    if (c == jobkeycol) {
        stepcol = -1
        k = keyof(line)
        insteps = (k == "steps")
        if (k == "env") { envscope = "job"; envcol = c }
        next
    }

    if (!insteps) next

    # spec: gate-sdk/SPEC.md §check-action-gh-repo — a new step is a dash at the step-list column alone, so a nested list under with:/strategy: is not read as a step
    if (match(line, /^[ ]*-[ ]+/)) {
        if (stepdashcol < 0) stepdashcol = c
        if (c == stepdashcol) {
            stepcol = RLENGTH
            printf "S\t%d\n", FNR
            if (pendstep != "") { printf "SX\t%s\n", pendstep }
            pendstep = ""
            stepkey(substr(line, stepcol + 1), stepcol, FNR)
            next
        }
    }
    if (stepcol >= 0 && c == stepcol) stepkey(substr(line, c + 1), c, FNR)
}

END { if (inrun) endrun() }
'

walked=0; subject=0; composite=0; outside=0
armed=0; inert=0; exempt=0; calls=0
findings=(); bare=()

curfile=""; wenv=0
job=""; jobline=0; jexempt=0; jenv=0
checkouts=(); iln=(); irepo=(); ienv=()
senv=0; sexempt=0; sln=(); srepo=()
have_job=0; have_step=0

finish_step() {
    (( have_step )) || return 0
    have_step=0
    local i
    if (( sexempt == 0 && ${#sln[@]} > 0 )); then
        for i in "${!sln[@]}"; do
            iln+=("${sln[$i]}"); irepo+=("${srepo[$i]}")
            if (( wenv || jenv || senv )); then ienv+=(1); else ienv+=(0); fi
        done
    fi
    senv=0; sexempt=0; sln=(); srepo=()
    return 0
}

# spec: gate-sdk/SPEC.md §check-action-gh-repo — the three arms are disjoined per job and each is universally quantified over the job's detected set
finish_job() {
    (( have_job )) || return 0
    finish_step
    have_job=0
    if (( jexempt )); then exempt=$((exempt + 1)); return 0; fi
    if (( ${#iln[@]} == 0 )); then inert=$((inert + 1)); return 0; fi
    armed=$((armed + 1)); calls=$((calls + ${#iln[@]}))
    local first="${iln[0]}" i allenv=1 allrepo=1
    for i in "${iln[@]}"; do (( i < first )) && first="$i"; done
    if (( ${#checkouts[@]} > 0 )); then
        for i in "${checkouts[@]}"; do (( i < first )) && return 0; done
    fi
    for i in "${ienv[@]}"; do (( i )) || allenv=0; done
    for i in "${irepo[@]}"; do (( i )) || allrepo=0; done
    (( allenv || allrepo )) && return 0
    findings+=("$curfile:$jobline: job '$job' first invokes gh at line $first with no repository context")
    return 0
}

for f in "${files[@]}"; do
    [[ -n "$f" ]] || continue
    walked=$((walked + 1))
    # spec: gate-sdk/SPEC.md §check-action-gh-repo — the Actions-shape predicate is split in two: the gate's unit is a job under jobs:, and a runs:-shaped composite action inherits its caller's repository context
    if grep -qE '^jobs:' "$f"; then
        subject=$((subject + 1))
    elif grep -qE '^runs:' "$f"; then
        composite=$((composite + 1))
        continue
    else
        outside=$((outside + 1))
        continue
    fi

    stream="$(awk "$WALK" "$f")"; st=$?
    fail_closed "$st" ACTION-GH-REPO "awk walk($f)"

    curfile="$f"
    wenv=0
    grep -qx 'W' <<< "$stream" && wenv=1

    while IFS=$'\t' read -r kind a b; do
        case "$kind" in
            J)  finish_job
                job="$a"; jobline="$b"; jexempt=0; jenv=0
                checkouts=(); iln=(); irepo=(); ienv=(); have_job=1 ;;
            JE) jenv=1 ;;
            JX) jexempt=1 ;;
            S)  finish_step; have_step=1 ;;
            SE) senv=1 ;;
            SX) sexempt=1 ;;
            C)  checkouts+=("$a") ;;
            G)  sln+=("$a"); srepo+=("$b"); have_step=1 ;;
            XBAD) bare+=("$f:$a: a gh-repo-exempt marker with no reason") ;;
        esac
    done <<< "$stream"
    finish_job
done

red=0

if [[ ${#findings[@]} -gt 0 ]]; then
    red=1
    echo "check-action-gh-repo: a job invokes gh with no way to resolve a target"
    echo "repository, so every call in it dies before its first request — on a tag,"
    echo "where nothing else in the battery runs:"
    printf '  %s\n' "${findings[@]}"
    echo "  help: add an actions/checkout step before the first gh call, or set"
    echo "        GH_REPO: \${{ github.repository }} on the workflow, the job, or the"
    echo "        invoking step's env:, or pass --repo on every gh call in the job."
    echo "        A job standing outside all three takes '# gh-repo-exempt: <reason>'."
fi

if [[ ${#bare[@]} -gt 0 ]]; then
    red=1
    echo "check-action-gh-repo: a gh-repo-exempt marker carries no reason, so it records"
    echo "that an arm was stood outside of without saying which one or why:"
    printf '  %s\n' "${bare[@]}"
    echo "  help: write the marker as '# gh-repo-exempt: <reason>' naming the arm the"
    echo "        job stands outside of, or delete it and satisfy an arm."
fi

[[ "$red" -eq 1 ]] && exit 1

echo "ACTION-GH-REPO: clean ($armed job(s) invoking gh across $subject Actions-shaped file(s) of $walked walked, all resolving a repository; $calls invocation(s) detected, $inert job(s) invoking none, $exempt exempt, $composite composite-action file(s) and $outside non-Actions file(s) skipped)"
exit 0
