# SPEC-second-gate — amendment fixture (good, second file)

A second amendment in the same tree, so the walk assertion G runs is proved to
read every `SPEC-*.md` it finds rather than the first one only.

## Definition of Done

- [ ] `check-second-gate.sh` carries its manifest:

```sh
# graph: couples=docs/second.md,kit:templates/*.md trigger=docs/*.md,kit:templates/*.md dir=one valve=none tier=align-only
```
