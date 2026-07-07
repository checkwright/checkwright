# shellcheck shell=bash
# drift-kit consumer config. Copy into your gates dir (default scripts/), or
# point DRIFT_KIT_CONFIG_FILE elsewhere, and uncomment any knob you want to
# override. bin/drift-report.sh sources this first, then fills every unset knob
# with the platform-value default shown here, and exports the resolved set to
# the KPI plugins.

# The KPI registry (one plugin name per line, # comments — the gates.list grammar).
# DRIFT_KIT_KPIS_FILE="${GATE_SDK_GATES_DIR:-scripts}/kpis.list"

# Extra resolution roots searched before the vendored kits' kpis/ dirs. A
# consumer shadows a bundled KPI by dropping a same-named file in one of these.
# DRIFT_KIT_KPI_DIRS=("${GATE_SDK_GATES_DIR:-scripts}")

# Surfaces the bundled KPIs read.
# DRIFT_KIT_QUEUE_FILE="${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}"
# DRIFT_KIT_KNOWLEDGE_LOG="${GATE_SDK_WORKFLOW_DIR:-.workflow}/knowledge-friction.log"
# DRIFT_KIT_TIMINGS_FILE="${GATE_SDK_TMP_DIR:-.tmp}/gate-timings.txt"

# Plugin scratch root (plugins never write outside it).
# DRIFT_KIT_TMP_DIR="${GATE_SDK_TMP_DIR:-.tmp}"

# Queue section headings the task-split and deferred-age KPIs scan.
# DRIFT_KIT_DONE_SECTION="Done"
# DRIFT_KIT_DEFERRED_SECTION="Deferred"
