#!/usr/bin/env bash
# graph: couples=scripts/*.sh,kit:*.sh,.workflow/*.txt,TASK-QUEUE.md dir=one valve=none tier=precommit
# spec: canon-kit/SPEC.md §check-deprecation-task — every deprecation marker on a governed source binds task: <slug> to a live queue task
#
# usage: check-deprecation-task.sh [scan-root [queue-file]]
#   Resolves each CANON_KIT_DEPRECATION_MARKERS marker's task: <slug> binding
#   against the queue (default $CANON_KIT_QUEUE_FILE); an empty roster is a skip.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
QUEUE="${2:-$CANON_KIT_QUEUE_FILE}"
[[ -d "$ROOT" ]] || { echo "check-deprecation-task: not a directory: $ROOT" >&2; exit 2; }

if [[ ${#CANON_KIT_DEPRECATION_MARKERS[@]} -eq 0 ]]; then
    echo "DEPRECATION-TASK: clean (no CANON_KIT_DEPRECATION_MARKERS configured; the deprecation-marker vocabulary is consumer config — nothing to resolve)"
    exit 0
fi

[[ -f "$QUEUE" ]] || { echo "check-deprecation-task: queue file not found: $QUEUE" >&2; exit 2; }

marker_re="$(printf '%s|' "${CANON_KIT_DEPRECATION_MARKERS[@]}")"; marker_re="${marker_re%|}"

# spec: canon-kit/SPEC.md §check-deprecation-task — the queue-resolution pass is
#   the shared lib adapter, the binding grammar check-todo-task-liveness's twin.
qout="$(spec_queue_slugs "$QUEUE")"; st=$?
fail_closed "$st" DEPRECATION-TASK "queue awk"

declare -A IS_LIVE=() IS_DONE=()
while IFS=$'\t' read -r kind slug; do
    [[ -n "$slug" ]] || continue
    case "$kind" in
        live) IS_LIVE["$slug"]=1 ;;
        done) IS_DONE["$slug"]=1 ;;
    esac
done <<< "$qout"

declare -a SURFACE=()
while IFS= read -r f; do
    [[ -n "$f" ]] && SURFACE+=("$f")
done < <(spec_comment_surface "$ROOT")

unbound=(); stale=(); unresolved=(); scanned=0
for f in "${SURFACE[@]}"; do
    rel="${f#"$ROOT"/}"; rel="${rel#./}"
    scanned=$((scanned + 1))
    mout="$(awk -v markerre="$marker_re" '
        $0 ~ markerre {
            m = $0; match(m, markerre); marker = substr(m, RSTART, RLENGTH)
            if (match($0, /task:[[:space:]]*[a-z0-9][a-z0-9-]*/)) {
                slug = substr($0, RSTART, RLENGTH); sub(/task:[[:space:]]*/, "", slug)
                printf "%d\tbound\t%s\t%s\n", FNR, slug, marker
            } else {
                printf "%d\tunbound\t\t%s\n", FNR, marker
            }
        }
    ' "$f")"; st=$?
    fail_closed "$st" DEPRECATION-TASK "marker awk ($rel)"
    while IFS=$'\t' read -r ln kind slug marker; do
        [[ -n "$ln" ]] || continue
        case "$kind" in
            unbound) unbound+=("$rel:$ln: deprecation marker '$marker' — no 'task: <slug>' binding on the line") ;;
            bound)
                [[ -n "${IS_LIVE[$slug]:-}" ]] && continue
                if [[ -n "${IS_DONE[$slug]:-}" ]]; then
                    stale+=("$rel:$ln: deprecation marker '$marker' → task: $slug is done; decommission the surface or bind a live task")
                else
                    unresolved+=("$rel:$ln: deprecation marker '$marker' → no live task '$slug' in $QUEUE")
                fi
                ;;
        esac
    done <<< "$mout"
done

if (( ${#unbound[@]} + ${#stale[@]} + ${#unresolved[@]} > 0 )); then
    echo "DEPRECATION-TASK: $(( ${#unbound[@]} + ${#stale[@]} + ${#unresolved[@]} )) violation(s):"
    (( ${#unbound[@]} ))    && printf '  %s\n' "${unbound[@]}"
    (( ${#stale[@]} ))      && printf '  %s\n' "${stale[@]}"
    (( ${#unresolved[@]} )) && printf '  %s\n' "${unresolved[@]}"
    echo "  help: a deprecation marker binds its surface to a live decommission task — add 'task: <slug>' on the marker line pointing at an active or deferred queue task, or decommission the surface and drop the marker once that task is done (an unbound marker, a done slug, or an absent slug all leave the deprecation tracking nothing)."
    exit 1
fi
echo "DEPRECATION-TASK: clean ($scanned governed source(s); every deprecation marker binds task: <slug> to a live queue task)"
exit 0
