#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §Testing — decision-table runner for the Agent dispatch-shape guard (D1 fork ban, D2 read-only isolation claim, D3 nested-dispatch advisory)
#
#   usage: run-dispatch-guard-tests.sh [cases.tsv]
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
GUARD="$KIT/templates/agent-dispatch-guard.sh"
LIB="$KIT/../guard-kit/lib/guard.sh"
CASES="${1:-$KIT/usage-tests/dispatch-guard-cases.tsv}"

for f in "$GUARD" "$LIB" "$CASES"; do
    [[ -f "$f" ]] || { echo "run-dispatch-guard-tests: missing $f" >&2; exit 2; }
done
command -v jq >/dev/null 2>&1 || { echo "run-dispatch-guard-tests: jq not found on PATH" >&2; exit 2; }

# spec: delegation-kit/SPEC.md §Testing — strip ambient DELEGATION_KIT_* at every guard invocation, the discipline run-budget-guard-tests.sh already uses, so a consumer's live roster or config-file pointer cannot leak into the fixture
DK_UNSET=()
while IFS= read -r name; do DK_UNSET+=(-u "$name"); done < <(env | grep -o '^DELEGATION_KIT_[A-Za-z0-9_]*')

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT
ROSTER_CFG="$SANDBOX/roster-config.sh"
ERR="$SANDBOX/err"
printf '%s\n' 'DELEGATION_KIT_READONLY_TYPES=(ro-type)' > "$ROSTER_CFG"

classify() {
    local rc="$1" out="$2"
    if [[ "$rc" -eq 2 ]]; then echo block; return; fi
    if [[ "$rc" -ne 0 ]]; then echo "exit$rc"; return; fi
    if grep -q '"additionalContext"' <<<"$out"; then echo advise; return; fi
    if [[ -z "$out" ]]; then echo fallthrough; return; fi
    echo unknown
}

# spec: delegation-kit/SPEC.md §Testing — builds a real hook payload (tool_input plus, for a nested case, the top-level agent_id/agent_type the harness only sets inside a subagent), so the guard's own jq extraction is exercised rather than a hand-rolled shortcut
build_payload() {
    local type="$1" isolation="$2" nested="$3" nb='false'
    [[ "$nested" == yes ]] && nb='true'
    jq -nc --arg t "$type" --arg iso "$isolation" --argjson nested "$nb" '
        {tool_name: "Agent",
         tool_input: (
            (if $t != "" then {subagent_type: $t} else {} end)
            + (if $iso != "" then {isolation: $iso} else {} end)
         )}
        + (if $nested then {agent_id: "agent_child", agent_type: "stage-session"} else {} end)
    '
}

fails=0
ran=0

while IFS=$'\t' read -r want rawtype isolation nested desc; do
    [[ -z "${want// }" ]] && continue
    [[ "$want" == \#* ]] && continue

    type="$rawtype"
    cfg_file="$ROSTER_CFG"
    case "$rawtype" in
        noroster:*) type="${rawtype#noroster:}"; cfg_file="" ;;
    esac

    iso="$isolation"; [[ "$iso" == "-" ]] && iso=""
    nst="$nested"; [[ "$nst" != "yes" ]] && nst="no"

    if [[ "$rawtype" == "UNPARSEABLE" ]]; then
        payload='this is not { json'
    else
        payload="$(build_payload "$type" "$iso" "$nst")"
    fi

    envargs=(GUARD_KIT_LIB="$LIB")
    [[ -n "$cfg_file" ]] && envargs+=(DELEGATION_KIT_CONFIG_FILE="$cfg_file")

    out="$( cd "$SANDBOX" && printf '%s' "$payload" \
        | env "${DK_UNSET[@]}" "${envargs[@]}" bash "$GUARD" 2>"$ERR" )"
    rc=$?
    err="$(cat "$ERR" 2>/dev/null)"
    got="$(classify "$rc" "$out")"
    ran=$((ran + 1))

    if [[ "$got" != "$want" ]]; then
        echo "  FAIL [$desc]: want '$want', got '$got' (rc=$rc) -- out=$out err=$err"
        fails=$((fails + 1))
    fi
done < "$CASES"

if [[ "$ran" -eq 0 ]]; then
    echo "run-dispatch-guard-tests: no cases parsed from $CASES" >&2
    exit 2
fi
if [[ "$fails" -gt 0 ]]; then
    echo "run-dispatch-guard-tests: $fails/$ran case(s) failed"
    exit 1
fi
echo "run-dispatch-guard-tests: ok ($ran cases across D1/D2/D3 and the degradation posture)"
exit 0
