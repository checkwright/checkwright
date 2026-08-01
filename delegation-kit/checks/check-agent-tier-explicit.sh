#!/usr/bin/env bash
# graph: couples=.claude/agents/*.md dir=one valve=none tier=precommit
# spec: delegation-kit/SPEC.md §check-agent-tier-explicit — every agent definition under the scanned directory declares a `model:` field in its frontmatter (an explicit `inherit` passes; only omission reds)
#
# usage: check-agent-tier-explicit.sh [agent-dir]
#   defaults DELEGATION_KIT_AGENT_DIR; a fixture passes its own directory.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/delegation.sh
source "$KIT/lib/delegation.sh"

DIR="${1:-$DELEGATION_KIT_AGENT_DIR}"; DIR="${DIR%/}"

if [[ ! -d "$DIR" ]]; then
    echo "AGENT-TIER-EXPLICIT: clean (0 agent definition(s) under $DIR; no agent-definition directory)"
    exit 0
fi

listing="$(find "$DIR" -type f -name '*.md' | LC_ALL=C sort)"; st=$?
fail_closed "$st" AGENT-TIER-EXPLICIT find

files=()
while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    gate_path_pruned "$path" && continue
    files+=("$path")
done <<< "$listing"

if [[ ${#files[@]} -eq 0 ]]; then
    echo "AGENT-TIER-EXPLICIT: clean (0 agent definition(s) under $DIR; nothing to check)"
    exit 0
fi

bare="$(awk '
    FNR == 1 { if (fname != "" && !seen) print fname; fname = FILENAME; seen = 0; open = 0; shut = 0; ln = 0 }
    { ln++ }
    ln == 1 { if ($0 == "---") open = 1; else shut = 1; next }
    shut || !open { next }
    $0 == "---" { shut = 1; next }
    $0 ~ /^model:[[:space:]]*[^[:space:]]/ { seen = 1 }
    END { if (fname != "" && !seen) print fname }
' "${files[@]}")"; st=$?
fail_closed "$st" AGENT-TIER-EXPLICIT awk

if [[ -n "$bare" ]]; then
    echo "check-agent-tier-explicit: agent definition(s) whose frontmatter omits the model: field:"
    while IFS= read -r f; do
        [[ -n "$f" ]] && echo "  $f: no 'model:' field in frontmatter"
    done <<< "$bare"
    echo "  help: state the tier in the definition's frontmatter — an omitted model: is not a neutral"
    echo "        default but the literal 'inherit' (the dispatcher's tier), so declare it even when"
    echo "        the answer is to inherit; 'model: inherit' passes."
    exit 1
fi

echo "AGENT-TIER-EXPLICIT: clean (${#files[@]} agent definition(s) under $DIR, each declaring an explicit model:)"
exit 0
