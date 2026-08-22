# shellcheck shell=bash
# spec: guard-kit/SPEC.md §Layout and configuration — the knob table (every knob, its default, when to set it) and how to install this config; set only what you override

# spec: guard-kit/SPEC.md §compare-settings-allow — declare your own probes; the kit ships none
# GUARD_KIT_BREADTH_PROBES=("Bash(rm -rf /)")

# spec: guard-kit/SPEC.md §compare-settings-allow — a breadth ruled intended, keyed on the exact local allow-rule string; the kit ships no declarations either
# declare -A GUARD_KIT_BREADTH_DECLARED=(["Bash(mytool *)"]="every subcommand is read-only")
