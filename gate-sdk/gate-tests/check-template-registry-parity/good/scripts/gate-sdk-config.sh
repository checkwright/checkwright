# shellcheck shell=bash
# Fixture config: make the sandbox kits the kit roots, so gate_kit_roots
# resolves to this case dir (auto-sourced by lib/gate.sh at cwd) rather than
# the real repo. See gate-sdk/SPEC.md §Layout and configuration.
GATE_SDK_KIT_DIRS="alpha-kit beta-kit"
