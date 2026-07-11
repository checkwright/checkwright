# good — kit references resolve; non-references stay out of scope

Live path segments (line- and slash-anchored) name a gate_kit_roots dir:
canon-kit/lib/spec.sh
docs/gate-sdk/index.md

Live-prefix knobs that resolve to a tracked kit knob:
CANON_KIT_CONFIG_FILE
GATE_SDK_ROOT
LIFECYCLE_KIT_CONFIG_FILE

Out of scope, never flagged — prose compounds and non-brand namespaces:
a per-kit note, check-kit-registration, and the SMOKE_KIT_ROOT harness var
(a KIT_ token whose stem names no gate_kit_roots dir).
