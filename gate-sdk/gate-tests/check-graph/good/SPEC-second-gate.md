# SPEC-second-gate — amendment fixture (good, second file)

A second amendment in the same tree, so the walk assertion G runs is proved to
read every `SPEC-*.md` it finds rather than the first one only.

## Definition of Done

- [ ] `check-second-gate.sh` carries its manifest, and `check-third-gate.sh` roots
      its globs at a locator and at a scalar directory knob:

```sh
# graph: couples=docs/second.md,kit:templates/*.md trigger=docs/*.md,kit:templates/*.md dir=one valve=none tier=align-only
# graph: couples=knob:GATE_SDK_GATES_DIR/gates.list,knob:GATE_SDK_WORKFLOW_DIR/*.md dir=one valve=none tier=precommit
```
