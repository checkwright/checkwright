# shellcheck shell=bash
# spec: guard-kit/SPEC.md §Layout and configuration — this repo overrides one guard-kit knob; templates/guard-config.sh lists them and lib/guard.sh holds the defaults

# spec: guard-kit/SPEC.md §compare-settings-allow — witnesses that a local-overlay glob is too broad; probes, not a roster, so no completeness is claimed
# shellcheck disable=SC2034  # consumed by guard-kit/lib/guard.sh after sourcing
GUARD_KIT_BREADTH_PROBES=(
    "Bash(git reset --hard)"
    "Bash(git clean -fd)"
    "Bash(git push --force)"
    "Bash(git rm -r src)"
    "Bash(git checkout -- TASK-QUEUE.md)"
    "Bash(git stash clear)"
    "Bash(gh repo delete owner/repo --yes)"
    "Bash(gh api -X DELETE /repos/owner/repo)"
    "Bash(gh release delete v0.24.0 --yes)"
    "Bash(gh secret delete TOKEN)"
    "Bash(git worktree remove --force .)"
)

# spec: guard-kit/SPEC.md §compare-settings-allow — breadth ruled intended; exact-string keyed, so re-spelling or narrowing the glob returns it to the narrowing candidates
# shellcheck disable=SC2034  # consumed by guard-kit/lib/guard.sh after sourcing
declare -A GUARD_KIT_BREADTH_DECLARED=(
    ["Bash(git worktree *)"]="worktree isolation is mandated rather than optional here — agent-dispatch-guard refuses a read-only-type dispatch that does not carry it — and the harness both creates and auto-cleans those trees, so the force-removal this breadth reaches destroys only disposable child state"
)
