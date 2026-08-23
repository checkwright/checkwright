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

### check-qux

Invariant: qux is held on two axes: (A) the first thing; (B) the second thing —
and its markers carry the `//` leader, the one every descriptor-declared
member's implementation module uses, so this contract proves the grammar reads a
code marker under either leader and under indentation.

### check-noun-filtered

Invariant: this one is settled on three grounds: (A) a labelled span is present;
(B) a second label follows it. The count-word's noun is not an enumeration noun,
so discovery excludes the heading — proved by greenness, since no gate code
resolves for this name and an unfiltered heading would red.

### check-paren-filtered

Invariant: this one holds on two axes (each described in the paragraph below):
(A) a labelled span is present; (B) a second label follows it. The first
parenthetical after the count-word is not a single-char label, so discovery
excludes the heading — again proved by greenness.

### check-single-filtered

Invariant: this one holds on two axes: (A) the only labelled item in the span.
Fewer than two distinct labels are enumerated, so discovery excludes the heading
— greenness proves the arity filter, not the resolver.
