#!/usr/bin/env bash
# graph: couples=kit:smoke/*.sh,kit:gate-tests/*.test.sh,kit:bin/*.sh dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-assertion-strength — a guard over a call to a script whose declared `# exit:` header binds a verdict token to one non-zero code must not name that token in its failure message while comparing no status to that token's code
#
# usage: check-assertion-strength.sh [dir...]   scan the named smoke/gate-tests dir(s)
#        (no args) each kit's smoke/ and gate-tests/
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

WINDOW=8

# spec: gate-sdk/SPEC.md §check-assertion-strength — the token->code grammar: each uppercase token binds to the nearest preceding integer on the line; a token bound to several codes or to code 0 is not discriminable and is dropped, which is what keeps an honest `did not verdict OK` message off the finding list
exit_map() {
    awk '
    /^#[[:space:]]*exit:/ {
        sub(/^#[[:space:]]*exit:/, "")
        code = -1
        n = split($0, part, /[^A-Za-z0-9_-]+/)
        for (i = 1; i <= n; i++) {
            t = part[i]
            if (t == "") continue
            if (t ~ /^[0-9]+$/) { code = t + 0; continue }
            if (code < 0) continue
            if (t !~ /^[A-Z][A-Z0-9]*(-[A-Z0-9]+)*$/) continue
            if (length(t) < 2) continue
            if (t in code_of) { if (code_of[t] != code) dup[t] = 1 }
            else code_of[t] = code
        }
    }
    END { for (t in code_of) if (!(t in dup) && code_of[t] != 0) print t, code_of[t] }
    ' "$1"
}

fail=0
scanned=0
declaring=0
declare -a findings=()

scan_file() {
    local f="$1" kitroot="$2"
    local -a L=()
    mapfile -t L < "$f"
    local n=${#L[@]} i j

    local -a inv=()
    for (( i = 0; i < n; i++ )); do
        [[ "${L[i]}" =~ bin/([A-Za-z0-9._-]+\.sh) ]] || continue
        [[ -r "$kitroot/bin/${BASH_REMATCH[1]}" ]] || continue
        inv+=("$i")
    done
    [[ ${#inv[@]} -gt 0 ]] || return 0

    local k callee name map tok code wtext gtext line stop cbstart
    for (( k = 0; k < ${#inv[@]}; k++ )); do
        i="${inv[k]}"
        [[ "${L[i]}" =~ bin/([A-Za-z0-9._-]+\.sh) ]] || continue
        name="${BASH_REMATCH[1]}"
        callee="$kitroot/bin/$name"
        map="$(exit_map "$callee")"; st=$?
        fail_closed "$st" check-assertion-strength awk
        [[ -n "$map" ]] || continue
        declaring=$((declaring + 1))

        [[ "${L[i]}" == *'||'*|| "${L[i]}" == *'&&'* || "${L[i]}" =~ ^[[:space:]]*if[[:space:]] || "${L[i]}" == *'; then'* ]] || continue

        stop=$(( i + WINDOW ))
        (( stop > n - 1 )) && stop=$(( n - 1 ))
        (( k + 1 < ${#inv[@]} && inv[k+1] - 1 < stop )) && stop=$(( inv[k+1] - 1 ))

        cbstart="$i"
        while (( cbstart > 0 )) && [[ "${L[cbstart-1]}" =~ ^[[:space:]]*# ]]; do cbstart=$(( cbstart - 1 )); done
        gtext=""
        for (( j = cbstart; j <= stop; j++ )); do gtext+="${L[j]}"$'\n'; done
        [[ "$gtext" =~ ^\ *#.*assertion-strength-exempt: || "$gtext" == *'assertion-strength-exempt:'* ]] && continue

        wtext=""
        for (( j = i; j <= stop; j++ )); do
            line="${L[j]}"
            [[ "$line" == *echo* || "$line" == *printf* ]] && wtext+="$line"$'\n'
        done
        [[ -n "$wtext" ]] || continue

        while read -r tok code; do
            [[ -n "$tok" ]] || continue
            [[ "$wtext" =~ (^|[^A-Za-z0-9_-])"$tok"([^A-Za-z0-9_-]|$) ]] || continue
            [[ "$gtext" =~ (-eq|-ne|==|!=)[[:space:]]*\"?"$code"\"? ]] && continue
            [[ "$gtext" =~ (^|[^A-Za-z0-9_-])"$code"\) ]] && continue
            findings+=("$f:$(( i + 1 )): message names $tok (exit $code of $name) but the guard compares no status to $code")
            fail=1
        done <<<"$map"
    done
}

declare -a scan_dirs=()
if [[ $# -gt 0 ]]; then
    scan_dirs=("$@")
else
    while IFS= read -r k; do
        [[ -d "$k/smoke" ]] && scan_dirs+=("$k/smoke")
        [[ -d "$k/gate-tests" ]] && scan_dirs+=("$k/gate-tests")
    done < <(gate_kit_roots)
fi

shopt -s nullglob
for d in "${scan_dirs[@]}"; do
    [[ -d "$d" ]] || { echo "check-assertion-strength: not a directory: $d" >&2; exit 2; }
    kitroot="$(cd "$d/.." && pwd)"
    for f in "$d"/*.sh; do
        [[ -r "$f" ]] || { echo "check-assertion-strength: unreadable script: $f" >&2; exit 2; }
        scanned=$((scanned + 1))
        scan_file "$f" "$kitroot"
    done
done
shopt -u nullglob

if [[ "$fail" -ne 0 ]]; then
    echo "check-assertion-strength: guard(s) whose failure message is more specific than the"
    echo "guard behind it — the message names a verdict token the callee's declared '# exit:'"
    echo "header binds to one non-zero code, but the guard discriminates only zero from"
    echo "non-zero, so a different failure mode reports itself under the wrong name"
    echo "(gate-sdk/SPEC.md §check-assertion-strength):"
    for m in "${findings[@]}"; do echo "  $m"; done
    echo "  help: capture the status and compare it to the code the message claims —"
    echo "        cmd && rc=0 || rc=\$?   then   if [[ \"\$rc\" -ne <code> ]]; then …"
    echo "  (report the observed status in the message), OR reword the message to claim only"
    echo "  what truthiness establishes, OR add a '# assertion-strength-exempt: <reason>' line"
    echo "  on a guard that establishes the outcome by other means."
    exit 1
fi
echo "ASSERTION-STRENGTH: clean ($scanned script(s) scanned; $declaring call(s) to a script with a declared exit contract)"
exit 0
