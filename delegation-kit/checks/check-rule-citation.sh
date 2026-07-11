#!/usr/bin/env bash
# graph: couples=delegation-kit/SPEC.md,delegation-kit/templates/agent-execution.md dir=one valve=none tier=precommit
# spec: delegation-kit/SPEC.md §One template, a resident pointer — every `the template's **<name>** rule` citation in SPEC §The delegation model resolves to a template bullet's bold lead-in (forward direction only)
#
# usage: check-rule-citation.sh [spec-file [template-file]]
#   paths resolve relative to cwd (= repo root in a battery run); fixtures pass
#   their own spec + template as the two positional arguments.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

SPEC_FILE="${1:-delegation-kit/SPEC.md}"
TEMPLATE_FILE="${2:-delegation-kit/templates/agent-execution.md}"
for f in "$SPEC_FILE" "$TEMPLATE_FILE"; do
    [[ -f "$f" ]] || { echo "check-rule-citation: not found: $f" >&2; exit 2; }  # exit 2: fail-closed
done

tmpl_out="$(awk '
    /^- \*\*/ {
        line=$0; sub(/^- /,"",line)
        rest=substr(line,3)
        p=index(rest,"**")
        if (p>0) {
            name=substr(rest,1,p-1)
            sub(/\.$/,"",name)
            gsub(/`[^`]*`/,"",name)
            print name
        }
    }' "$TEMPLATE_FILE")"; st=$?
fail_closed "$st" check-rule-citation awk

declare -A LEADIN=()
while IFS= read -r n; do [[ -n "$n" ]] && LEADIN["$n"]=1; done <<<"$tmpl_out"

cite_out="$(awk '
    /^## The delegation model[[:space:]]*$/ { insec=1; next }
    /^## / { insec=0 }
    insec { sec = sec " " $0 }
    END {
        gsub(/`[^`]*`/, "", sec)
        n=length(sec); i=1
        while (i<=n) {
            op=index(substr(sec,i),"**"); if (op==0) break
            op=i+op-1; after=op+2
            cl=index(substr(sec,after),"**"); if (cl==0) break
            cl=after+cl-1
            name=substr(sec,after,cl-after)
            tail=substr(sec,cl+2)
            if (tail ~ /^[[:space:]]+(rule|bullet)([^[:alnum:]]|$)/) {
                c=name; sub(/\.$/,"",c); print c
            }
            i=cl+2
        }
    }' "$SPEC_FILE")"; st=$?
fail_closed "$st" check-rule-citation awk

unresolved=()
cites=0
while IFS= read -r c; do
    [[ -n "$c" ]] || continue
    cites=$((cites + 1))
    [[ -n "${LEADIN[$c]:-}" ]] || unresolved+=("$c")
done <<<"$cite_out"

if [[ ${#unresolved[@]} -gt 0 ]]; then
    echo "check-rule-citation: SPEC §The delegation model cites a rule that does not resolve to a template lead-in in $TEMPLATE_FILE:"
    printf '  **%s**\n' "${unresolved[@]}"
    echo "  help: a citation names a template bullet's bold lead-in verbatim (minus its trailing period); fix the name or the renamed lead-in — delegation-kit/SPEC.md §One template, a resident pointer"
    exit 1
fi

echo "RULE-CITATION: clean ($cites citation(s) in SPEC §The delegation model each resolve to a template lead-in; ${#LEADIN[@]} lead-in(s) available)"
exit 0
