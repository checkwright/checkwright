# shellcheck shell=bash
# spec: queue-kit/SPEC.md §Layout and configuration — this repo's queue-kit consumer config: the one outbound lesson-harvest tag ([essay], routed by .claude/commands/close.md) and the living-page prose surfaces that make queue-membership claims; every other knob keeps the platform default
# shellcheck disable=SC2034  # consumed by queue-kit/lib/queue.sh after sourcing
QUEUE_KIT_LESSON_TAGS=(essay)
# shellcheck disable=SC2034  # consumed by queue-kit/lib/queue.sh after sourcing
QUEUE_KIT_PROSE_SURFACE_GLOBS=("docs/*.md")
