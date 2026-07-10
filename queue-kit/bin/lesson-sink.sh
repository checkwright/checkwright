#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §bin/lesson-sink.sh — resolve a harvest tag's sink and stream the lesson body into it (a tool, not a gate; no # graph: manifest)
#
# usage: lesson-sink.sh <tag>
#   reads the lesson body on stdin; runs the QUEUE_KIT_LESSON_SINKS command for
#   <tag>, or appends to <workflow-dir>/<tag>-harvest.md when none is configured
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

tag=""
while (($#)); do
    case "$1" in
        -h|--help) sed -n '4,6p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; exit 0 ;;
        -*) echo "lesson-sink: unknown option: $1" >&2; exit 2 ;;
        *)  tag="$1"; shift ;;
    esac
done
[[ -n "$tag" ]] || { echo "lesson-sink: needs a <tag>" >&2; exit 2; }

if [[ -v QUEUE_KIT_LESSON_SINKS[$tag] ]]; then
    # spec: queue-kit/SPEC.md §bin/lesson-sink.sh — a configured sink that fails is a red close step, never a silent fallback to staging
    exec bash -c "${QUEUE_KIT_LESSON_SINKS[$tag]}"
fi

exec cat >> "${GATE_SDK_WORKFLOW_DIR:-.workflow}/${tag}-harvest.md"
