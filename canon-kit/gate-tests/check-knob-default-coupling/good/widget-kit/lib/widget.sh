#!/usr/bin/env bash
# Every literal default agrees across its sites and with the owning SPEC.
: "${WIDGET_KIT_QUEUE_FILE:-TASK-QUEUE.md}"
: "${WIDGET_KIT_RUN_CAP:-3}"
[[ -v WIDGET_KIT_MODE ]] || WIDGET_KIT_MODE="exactly-one"
