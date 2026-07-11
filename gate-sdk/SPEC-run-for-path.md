# SPEC amendment: run-for-path

## What changes

- `bin/run-gates.sh --for <path> [<path>...]` — path-scoped selection, the
  agent-callable half of the oracle-first rule: resolve the registry exactly
  as today, then run only the gates whose effective trigger (`trigger=` else
  `couples=`, expanded through `gate_expand_couples`) glob-matches at least
  one given repo-relative path. Registry order and per-gate output are
  unchanged; bare `run-gates.sh` keeps its current behavior.
- One matcher, stated as an invariant: the flag's match semantics are
  *defined* as identical to the generated hook's staged-path matching — both
  read the same `# graph:` manifest fields through the same shared reader
  (gate-sdk/SPEC.md §check-graph names that single-reader rule), and the
  glob-match step lives once in `lib/gate.sh` where both the selector and
  the hook generator consume it. A divergence between what the hook would
  run for a staged path and what `--for` runs for the same path is a bug
  against this section.
- No-match behavior: print an explicit "no registered gate couples to
  \<path\>" note and exit 0 — an ungoverned path is a fact, not a failure;
  the selector is a bin tool, never a registered gate. Its own plumbing
  stays fail-closed: an unreadable registry or manifest exits non-zero.

## Producers and consumers

- Producer: a mid-edit session (or a delegated agent's gate-driven
  worklist) invoking `run-gates.sh --for <edited-path>`. No new enabling
  config — every registered gate already carries the `# graph:` manifest
  the selection reads.
- Consumer: the selected gates run as in a full battery pass; their
  output — verdict line plus help — is the feedback channel. The loop this
  buys is edit → run coupled gates → read help, strictly cheaper than
  reading gate source to predict a verdict.

## Existing sections updated

- gate-sdk/SPEC.md §run-gates: usage line and the `--for` selection
  contract.
- gate-sdk/SPEC.md §The `# graph:` manifest: the manifest now feeds three
  readers — gen-pre-commit, check-graph, and `--for` selection — through
  the one shared expansion.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
