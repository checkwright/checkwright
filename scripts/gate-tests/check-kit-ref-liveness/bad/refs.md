# bad — dangling kit references after a rename/retirement

A retired kit dir leaves a path segment naming no live root:
spec-kit/lib/spec.sh

A live-prefix knob that no tracked kit source declares:
CANON_KIT_BOGUS_KNOB

The bare kit prefix is not a family stem, or one composed spelling of it would
resolve every knob the kit could ever name:
GATE_SDK_BOGUS_KNOB
