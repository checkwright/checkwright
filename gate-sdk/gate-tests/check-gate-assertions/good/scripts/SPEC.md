# Synthetic SPEC for check-gate-assertions good-case fixture

## Per-gate contracts

### check-foo

Invariant: the foo surface stays consistent on two axes: (A) the first thing the
gate verifies about foo; (B) the second thing the gate verifies about foo.

### check-bar

Invariant: bar is internally coherent — a single-assertion contract with no
count-word and no label span, so it is out of scope (proves the discovery
filter excludes non-enumerated contracts).

### check-baz

Invariant: baz is checked on three checks: (1) the first; (2) the second; (3)
the third — the count-word and the label span agree, so baz is a covered
contract.
