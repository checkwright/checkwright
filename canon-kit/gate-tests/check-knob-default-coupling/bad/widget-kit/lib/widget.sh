#!/usr/bin/env bash
# WIDGET_KIT_QUEUE_FILE's fallback drifted from the `TASK-QUEUE.md` the SPEC states.
: "${WIDGET_KIT_QUEUE_FILE:-QUEUE.md}"
# WIDGET_KIT_RUN_CAP disagrees with its own sibling site below (self-disagreement).
: "${WIDGET_KIT_RUN_CAP:-3}"
