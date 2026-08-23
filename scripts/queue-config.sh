# shellcheck shell=bash
# spec: queue-kit/SPEC.md §Layout and configuration — this repo's queue-kit consumer config: the one outbound lesson-harvest tag ([essay], routed by .claude/commands/close.md) and the living-page prose surfaces that make queue-membership claims; the icebox age floor, lowered from the kit's 30 days because this queue's inflow runs about a week per eviction review; every other knob keeps the platform default
# shellcheck disable=SC2034  # consumed by queue-kit/lib/queue.sh after sourcing
QUEUE_KIT_LESSON_TAGS=(essay)
# shellcheck disable=SC2034  # consumed by queue-kit/lib/queue.sh after sourcing
QUEUE_KIT_PROSE_SURFACE_GLOBS=("docs/*.md" "*.local.md")
# comment-tier-exempt: the roadmap posture is this repo's editorial vocabulary, never a kit literal — horizons in emitted section order, tracks as the per-item label; QUEUE_KIT_ROADMAP_MARKER keeps its kit default and is deliberately absent
# comment-tier-exempt: the `now` horizon is KEPT — ruled 2026-07-31, operator-confirmed, against the objection that one-iteration-at-a-time plus close-only pushes could let a unit be `now` locally and never publish as `now`. It has been non-empty on master three times, so the horizon is not structurally dead, and an empty `now` reports honestly that the current cycle is internal. Residual and accepted: a short roadmap-tagged iteration opening and closing inside one push window is never observed as `now`. Do not re-litigate without new evidence
# shellcheck disable=SC2034  # consumed by queue-kit/lib/queue.sh after sourcing
QUEUE_KIT_HORIZONS=(now next later)
# shellcheck disable=SC2034  # consumed by queue-kit/lib/queue.sh after sourcing
QUEUE_KIT_TRACKS=(adoption reliability ecosystem commercial)
# shellcheck disable=SC2034  # consumed by queue-kit/lib/queue.sh after sourcing
QUEUE_KIT_ROADMAP_FILE=ROADMAP.md
# shellcheck disable=SC2034  # consumed by queue-kit/lib/queue.sh after sourcing
QUEUE_KIT_ICEBOX_SECTION=Icebox
# shellcheck disable=SC2034  # consumed by queue-kit/lib/queue.sh after sourcing
QUEUE_KIT_ICEBOX_AGE_DAYS=7
