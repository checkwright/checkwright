# SPEC-example-gate — amendment fixture (good)

A well-formed `# graph:` manifest embedded in an amendment body: all four
required keys present, enum values legal, couples tokens syntactically valid
globs (the `*/SPEC.md` surface may be design-ahead — existence is not required).

## Definition of Done

- [ ] `check-example-gate.sh` carries its manifest
      (`# graph: couples=TASK-QUEUE.md,*/SPEC.md dir=one valve=none tier=precommit`);
      registered in `gates.list`.

A prose mention of the `# graph:` concept, with `couples=` named separately, is
not a manifest and must not be validated.

```proto
// A `# graph: couples=bogus dir=mono` line inside a proto fence is illustrative
// wire context, never a manifest — assertion G must skip it.
message Example { string id = 1; }
```
