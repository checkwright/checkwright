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
)
