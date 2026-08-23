# Synthetic SPEC for check-gate-assertions bad-case fixture

## Per-gate contracts

### check-foo

Invariant: the foo surface stays consistent on two axes: (A) the first thing the
gate verifies about foo; (B) the second thing the gate verifies about foo.

### check-baz

Invariant: baz is checked on three checks: (1) the first; (2) the second; (3)
the third.

### check-extra

Invariant: extra is held on two axes: (A) the first thing; (B) the second thing
— and the code grew a third marker the contract never gained, which is the
drift an internal count-vs-span check cannot see.

### check-count

Invariant: count is checked on three checks: (1) the first; (2) the second — the
count-word and the label span disagree, so the contract is internally
inconsistent whatever the code carries.

### check-nowhere

Invariant: nowhere is held on two assertions: (A) the first thing; (B) the
second thing — but no gate code resolves for the name this heading carries.

### check-zero

Invariant: zero is held on two axes: (A) the first thing; (B) the second thing —
and its code carries no marker at all, which is the retrofit obligation.
