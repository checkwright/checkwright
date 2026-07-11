# SPEC amendment: delegation-rules-parity

delegation-kit states the protocol twice: SPEC.md §The delegation model's
numbered rule list and templates/agent-execution.md's bullets carry the same
rules in different prose, with no owner ruling between them. Tier by
why/what: the template owns the rules (the loaded procedure); the SPEC keeps
rationale and mechanism contracts and drops the roster, citing rules by
stable name. Deleting the restatement outranks gating its parity
(enforcement-first's structural form); what remains mechanical is
name-citation liveness.

## What changes

- **The template is the rule owner.** Each bullet's bold lead-in is the
  rule's **stable name** (e.g. "Supervisor owns rulings; agents surface,
  never guess", "Background + notification, never poll"). Operational
  detail that today lives only in the SPEC's numbered list (e.g. the
  burn-projection guidance, the class-ladder derivation note) moves into
  the owning bullet where the template does not already carry it — the
  template must read complete alone, since it is what a session loads.
- **SPEC.md §The delegation model drops the numbered roster.** The section
  keeps: the supervisor/agent model paragraph, the budget-guard mechanism
  contract (producer/consumer/routing — mechanism, not procedure), and
  per-rule *rationale* that earns SPEC residency (failure surfaces,
  calibration history, why a bound is not configurable up) — each such
  paragraph citing its rule by the citation grammar below rather than
  restating the rule. Rationale that is mere paraphrase of the bullet is
  deleted, not relocated (blessing a restatement is itself the defect).
- **Citation grammar** — a SPEC reference to a template rule is written
  `the template's **<rule name>** rule`, where `<rule name>` matches a
  bullet's bold lead-in in `templates/agent-execution.md` verbatim (minus
  the trailing period). One line, one binding: the grammar is owned here
  (post-merge: §One template, a resident pointer).
- **check-rule-citation** (new gate, `checks/check-rule-citation.sh`) —
  invariant: every `**<text>**` span in SPEC.md §The delegation model that
  is followed by the word `rule`/`bullet` resolves to a bold bullet
  lead-in in the template. Forward direction only (a template bullet with
  no SPEC rationale is fine). Gate-economy defence for shipping it: the
  citation surface must exist (the SPEC cannot avoid referencing rules),
  a template lead-in rename silently dangles every citation, and the
  check is one grep pass with a natural fixture pair — the cheap-insurance
  case, not the removable-surface case. Ships with `good/`+`bad/`
  fixtures, registers in this repo's `gates.list`, `# graph:` couples
  `delegation-kit/SPEC.md,delegation-kit/templates/agent-execution.md`.

## Producers and consumers

- The citation grammar's producer is any SPEC edit referencing a rule; its
  consumers are the reader (follows the name into the template) and
  `check-rule-citation` (resolves each citation at its single scan
  transition). No persistent state.
- The gate's invocation producers are the standard three (run-gates.sh,
  generated hook via its `# graph:` manifest — regenerate hook + graph
  artifact in the same unit — and CI's derived fixture suites, which
  enrol the new fixture dir with no hand-list edit).
- The template bullets' consumer is unchanged: the consumer's binding shim
  (`.claude/commands/agent-execution.md`) loads the template; no shim or
  binding-slot change — the two slots stay as they are.

## Existing sections updated

- delegation-kit/SPEC.md §The delegation model — restructured as above.
- delegation-kit/SPEC.md §One template, a resident pointer — gains the
  citation grammar's one-line binding; its "single source for the
  protocol" sentence becomes literally true and is verified, not
  reworded.
- delegation-kit/SPEC.md §Testing — the new gate's fixture row.
- delegation-kit/README.md — only if it restates the rule roster (verify;
  cite instead).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
