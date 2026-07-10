# SPEC amendment: commit-subject-grammar

## What changes

New gate `checks/check-commit-subject.sh`, `tier=commit-msg`, riding the
generated commit-msg hook beside check-commit-msg (the leak guard keeps its
single job; subject shape is a sibling assertion with its own fixtures).

Invariant: the prospective message's subject line parses as
`<type>(<scope>)?!?: <summary>` with `<type>` drawn from the shared roster,
or matches a carve-out — `Merge `, `Revert `, `fixup! `, `squash! ` (git's
own generated forms and the autosquash grammar). Scope token: `[a-z0-9./-]+`.
A subject that does not parse is an unread write to a governed projection,
not a style nit: trajectory.sh's feat/debt column classifies commit
subjects, and the closed-row freeze leans on docs/chore filings sitting
outside that harvest — both properties held today by convention alone.

Roster: `gate_commit_types` in `lib/gate.sh` reading
`GATE_SDK_COMMIT_TYPES` (default
`feat fix refactor perf docs test build ci chore style`). The
one-vocabulary/two-readers tension is ruled *share the roster, keep the
mappings*: the roster's single home is lib/gate.sh; drift-kit's
kpi-task-split and trajectory.sh keep their own class mapping (feat vs
fix+refactor) — a classification over roster tokens, not a second roster —
and the grammar gate guarantees every subject carries a roster token, so a
mistyped prefix becomes a blocked commit instead of a silently drifted
evidence row.

Behavior at the edges, matching check-commit-msg: a no-argument run (the
whole-tree battery) is a clean skip — the message is not a tracked surface;
a missing message-file argument-with-value is fail-closed (exit 2). The
`# graph:` couples the roster's config home (the regeneration trigger), not
a tree path — the gate is emitted into the commit-msg hook.

## Producers and consumers

- Producer: the generated `commit-msg` hook (gen-pre-commit emits it from
  the `# graph:` manifest, the tier the plumbing already carries); enabling
  config is the consumer's existing install-hooks opt-in.
- Consumer: the committing operator via the output contract — the finding
  names the offending subject line and the roster, read once at the single
  match transition.
- `GATE_SDK_COMMIT_TYPES` is read by `gate_commit_types` at gate startup;
  each token is read in the type alternation of the subject regex.
- Downstream (unchanged code, now guaranteed input): drift-kit's
  kpi-task-split and trajectory.sh read the same subjects; the gate is the
  parse guarantee their classification silently assumed.

## Existing sections updated

- §check-commit-msg: one sentence delimiting the split — banned patterns
  here, subject shape in the sibling.
- §lib/gate.sh: the roster adapter joins the values-and-adapters list.
- drift-kit/SPEC.md §Bundled KPIs, kpi-task-split bullet: cites
  check-commit-subject as the parse guarantee behind the classification
  (one line; the mapping itself is unchanged).
- Fixtures: good/bad message-file pair (valid types, carve-outs; a mistyped
  prefix, a missing colon); `check-commit-subject.test.sh` covers the
  config path (a consumer-widened roster).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as
      one coherent document a reader who never saw the amendment can use
      alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
