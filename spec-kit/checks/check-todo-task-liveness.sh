#!/usr/bin/env bash
# graph: couples=scripts/*.sh,kit:*.sh,.workflow/*.txt,TASK-QUEUE.md dir=one valve=none tier=precommit
# spec: spec-kit/SPEC.md §check-todo-task-liveness — every TODO(task: <slug>) marker on a governed source resolves to a live queue task, stale-flagged on a done slug
#
# usage: check-todo-task-liveness.sh [scan-root [queue-file]]
#   Scans the governed comment surface under scan-root (default '.') for
#   TODO(task: <slug>) markers and resolves each slug against the queue
#   (default $SPEC_KIT_QUEUE_FILE): live task resolves, done task is stale,
#   absent slug is unresolved.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
QUEUE="${2:-$SPEC_KIT_QUEUE_FILE}"
[[ -d "$ROOT" ]] || { echo "check-todo-task-liveness: not a directory: $ROOT" >&2; exit 2; }
[[ -f "$QUEUE" ]] || { echo "check-todo-task-liveness: queue file not found: $QUEUE" >&2; exit 2; }

# spec: spec-kit/SPEC.md §check-todo-task-liveness — one queue pass splits the
#   slug namespace: a bold lead-in in an active/deferred section is live, a
#   bare-slug bullet outside those sections is the done shape (queue-kit format).
qout="$(awk -v activere="$SPEC_ACTIVE_RE" -v defre="$SPEC_DEFERRED_RE" -v sectre="$SPEC_SECTION_RE" '
    $0 ~ sectre { active = ($0 ~ activere || $0 ~ defre); next }
    active && $0 ~ /^[[:space:]]*-[[:space:]]+\*\*[a-z0-9][a-z0-9-]*\*\*/ {
        match($0, /\*\*[a-z0-9][a-z0-9-]*\*\*/)
        printf "live\t%s\n", substr($0, RSTART + 2, RLENGTH - 4); next
    }
    !active && $0 ~ /^[[:space:]]*-[[:space:]]+[a-z0-9][a-z0-9-]*[[:space:]]*$/ {
        line = $0; sub(/^[[:space:]]*-[[:space:]]+/, "", line); sub(/[[:space:]]*$/, "", line)
        printf "done\t%s\n", line
    }
' "$QUEUE")"; st=$?
fail_closed "$st" TODO-TASK-LIVENESS "queue awk"

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

stale=(); unresolved=(); scanned=0
for f in "${SURFACE[@]}"; do
    rel="${f#"$ROOT"/}"; rel="${rel#./}"
    scanned=$((scanned + 1))
    # spec: spec-kit/SPEC.md §check-todo-task-liveness — the marker needs a
    #   resolvable slug after the colon, so the bare roster literal a tool
    #   carries (check-comment-tier's own directive name) never self-matches.
    mout="$(awk '
        {
            s = $0
            while (match(s, /TODO\(task:[[:space:]]*[a-z0-9][a-z0-9-]*/)) {
                m = substr(s, RSTART, RLENGTH); sub(/TODO\(task:[[:space:]]*/, "", m)
                printf "%d\t%s\n", FNR, m
                s = substr(s, RSTART + RLENGTH)
            }
        }
    ' "$f")"; st=$?
    fail_closed "$st" TODO-TASK-LIVENESS "marker awk ($rel)"
    while IFS=$'\t' read -r ln slug; do
        [[ -n "$slug" ]] || continue
        [[ -n "${IS_LIVE[$slug]:-}" ]] && continue
        if [[ -n "${IS_DONE[$slug]:-}" ]]; then
            stale+=("$rel:$ln: TODO(task: $slug) — task '$slug' is done; drop the completed TODO")
        else
            unresolved+=("$rel:$ln: TODO(task: $slug) — no live task '$slug' in $QUEUE")
        fi
    done <<< "$mout"
done

if (( ${#stale[@]} + ${#unresolved[@]} > 0 )); then
    echo "TODO-TASK-LIVENESS: $(( ${#stale[@]} + ${#unresolved[@]} )) violation(s):"
    (( ${#stale[@]} ))      && printf '  %s\n' "${stale[@]}"
    (( ${#unresolved[@]} )) && printf '  %s\n' "${unresolved[@]}"
    echo "  help: a TODO(task: <slug>) binds a code site to a live queue task — point it at an active or deferred slug, or resolve the code and delete the marker once the task is done (a done or absent slug leaves the marker referencing nothing)."
    exit 1
fi
echo "TODO-TASK-LIVENESS: clean ($scanned governed source(s); every TODO(task: <slug>) marker resolves to a live queue task)"
exit 0
