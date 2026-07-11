#!/usr/bin/env bash
# graph: couples=.claude/commands/*.md,lifecycle-kit/templates/skills/*.md,lifecycle-kit/templates/lead.md,delegation-kit/templates/agent-execution.md dir=one valve=none tier=precommit
# spec: lifecycle-kit/SPEC.md §check-skill-binding — every binding-shim skill names an existing template and binds exactly that template's slot set
#
# usage: check-skill-binding.sh [skills-dir]
#   Defaults to LIFECYCLE_KIT_SKILLS_DIR (.claude/commands). Template paths in a
#   binding directive resolve relative to the current directory.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

DIR="${1:-$LIFECYCLE_KIT_SKILLS_DIR}"
[[ -d "$DIR" ]] || { echo "check-skill-binding: skills dir not found: $DIR" >&2; exit 2; }

findings=()
shims=0

shopt -s nullglob
for f in "$DIR"/*.md; do
    tmpl="$(sed -nE 's/^Execute the template at (.+), applying the bindings below\.$/\1/p' "$f" | head -1)"
    [[ -n "$tmpl" ]] || continue
    shims=$((shims + 1))

    if [[ ! -f "$tmpl" ]]; then
        findings+=("$(basename "$f"): binding directive names template '$tmpl' — no such file")
        continue
    fi

    mapfile -t slots < <(grep -oE '\*<[a-z][a-z0-9-]*:' "$tmpl" | sed -E 's/^\*<//; s/:$//' | sort -u)
    mapfile -t binds < <(awk '/^## Bindings/ { f=1; next } f' "$f" \
        | sed -nE 's/^\*\*([a-z][a-z0-9-]*)\*\* —.*/\1/p' | sort -u)

    for s in "${slots[@]+"${slots[@]}"}"; do
        printf '%s\n' "${binds[@]+"${binds[@]}"}" | grep -qxF "$s" \
            || findings+=("$(basename "$f"): template slot '$s' has no binding")
    done
    for b in "${binds[@]+"${binds[@]}"}"; do
        printf '%s\n' "${slots[@]+"${slots[@]}"}" | grep -qxF "$b" \
            || findings+=("$(basename "$f"): binding '$b' names no slot in $tmpl")
    done
done
shopt -u nullglob

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-skill-binding: a binding shim must name an existing template and bind"
    echo "exactly that template's slot set — an unbound slot or an orphan binding is drift:"
    for m in "${findings[@]}"; do echo "  $m"; done
    echo "  help: fix the binding directive's template path, or align the shim's ## Bindings"
    echo "        entries with the template's *<slot-name: ...>* set (one entry per slot)."
    exit 1
fi

echo "SKILL-BINDING: clean ($shims binding-shim(s); each names an existing template and binds its exact slot set)"
exit 0
