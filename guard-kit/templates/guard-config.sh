# shellcheck shell=bash
# spec: guard-kit/SPEC.md §Layout and configuration — the knob table (every knob, its default, when to set it) and how to install this config; set only what you override
# no-port: gate-sdk/SPEC.md §The config-seam port disposition — the class ruling of 2026-08-30, reached by ground rather than by scope: this file is guard-kit's side of the adopter's config seam — the template `init` seeds into `<gates-dir>/guard-config.sh` under installer/README.md §What init seeds' derivation — so it IS the surface an adopter edits rather than kit mechanism reaching one, and porting it leaves nothing to edit. Its whole body is commented example lines for knobs whose values only the adopter can decide — GUARD_KIT_BREADTH_PROBES and GUARD_KIT_BREADTH_DECLARED, of which guard-kit/SPEC.md §compare-settings-allow ships neither, and GUARD_KIT_SEARCH_TOOLS, whose value is a property of the adopter's harness build — so the file's entire content is the adopter's to write. Structural, not a sizing judgment.

# spec: guard-kit/SPEC.md §compare-settings-allow — declare your own probes; the kit ships none
# GUARD_KIT_BREADTH_PROBES=("Bash(rm -rf /)")

# spec: guard-kit/SPEC.md §compare-settings-allow — a breadth ruled intended, keyed on the exact local allow-rule string; the kit ships no declarations either
# declare -A GUARD_KIT_BREADTH_DECLARED=(["Bash(mytool *)"]="every subcommand is read-only")

# spec: guard-kit/SPEC.md §Layout and configuration — the dedicated search tools your harness build carries (default Glob and Grep); drop a tool the build lacks, so rules 9 and 11 never steer at it
# GUARD_KIT_SEARCH_TOOLS=()
