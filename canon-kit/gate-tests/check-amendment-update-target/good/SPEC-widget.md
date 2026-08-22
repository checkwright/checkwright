# SPEC amendment: widget

Pairs with the queue entry this fixture does not carry — the pairing is
`check-amendment-queue`'s axis, not this gate's.

## What changes

### (1) The first delta

Body prose.

### (2) The second delta, whose body embeds a fenced block

The template sanctions an embedded wire-contract delta until merge, so a fence
must read as neither a delta heading nor an update target:

```
### (9) not a heading
- **not/a/target.md** — and no citation anywhere in it
```

### (3) The third delta

Body prose.

## Producers and consumers

Out of this gate's reach; present so the fixture is a whole amendment.

## Existing sections updated

Each names the delta that owns it.

- **component/SPEC.md §One** — a citation that wraps across the newline the
  entry is written to (delta
  1).
- **component/SPEC.md §Two** — a list citation (deltas 2, 3) and a possessive
  one, since delta 2's argument is what this target carries.
- **component/README.md** — a list joined by the word rather than the comma
  (deltas 1 and 3).
<!-- update-target-exempt: the valve, exercised — this target is owned by no delta on purpose -->
- **component/CHANGELOG.md** — deliberately owned by no delta, and tagged.
- **The generated mirror** — stale the moment any delta lands (all deltas).

## Definition of Done

- [ ] **Causal completeness** — the fixture asserts nothing here.
