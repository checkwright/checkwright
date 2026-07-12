# shellcheck shell=bash
# Fixture config: make a sandbox 'widget-kit' the sole kit root, so gate_kit_roots
# resolves to this case dir (auto-sourced by lib/gate.sh at cwd) rather than the
# real repo. See gate-sdk/SPEC.md §Layout and configuration.
GATE_SDK_KIT_DIRS="widget-kit"
