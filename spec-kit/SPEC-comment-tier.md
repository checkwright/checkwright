# SPEC amendment: comment-tier

## What changes

One new gate, `spec-kit/checks/check-comment-tier.sh`: the content-home
model enforced on the comment surface. The tiering rule the kit already
states in prose — comments cite the owning section, never restate design —
gets its classifier: every comment in the governed sources must be one of

- a **machine directive** — a comment some tool parses (`graph:`,
  `shellcheck`, `contract:`),
- a **reason directive** — a positional justification or spec pointer
  (`spec:`, `exception-list:`, `no-fixture:`, `assertion`, `permanent:`,
  `TODO(task:`, `TODO(spec-ambiguity)`), which also blesses the
  immediately following comment run (the reason's own wording), or
- `comment-tier-exempt: <reason>` for the genuinely unavoidable;

anything else is flagged: design rationale is relocated to the owning
SPEC, restated-code prose is deleted. Attested exemplar of the failure
mode this repo already carries: friction-kit `lib/guard.sh`'s per-rule
design comments, restating rulings its SPEC §The generic ruleset owns.

**The scope ruling the queue entry deferred — the blessed set splits.**
The platform ruled its blessed set a coupling vocabulary and kept the
whole gate behind; that ruling conflated two halves. The platform's
*product* directives (`glossary:`, `diagram:`, `domain-enum`,
`term-chain:`, …) are rule content and stay. But the directives above are
the names Checkwright's own kits define — cross-kit mechanism vocabulary
this repo already writes into every gate header — and a classifier
parameterized over the directive roster is mechanism, the same shape as
`check-graph`'s vocab split. So the gate ships with the kit-mechanism
roster as its default, and a consumer extends it:

- `SPEC_KIT_COMMENT_MACHINE` / `SPEC_KIT_COMMENT_REASON` — extra directive
  prefixes, appended to the built-in roster (the platform's product set
  becomes its consumer config at migration time).
- `SPEC_KIT_COMMENT_SURFACE` — globs of governed files; default: shell
  sources under the gates dir and the kit roots
  (`SPEC_KIT_SCAN_KIT_ROOTS`), plus `${GATE_SDK_WORKFLOW_DIR:-.workflow}`
  `/*.txt` state files (where only `contract:`/`see` headers are blessed).
  The platform's slash-comment language support (`//`, `/* */`,
  doc-comments, heredoc skipping) ships — it is parsing mechanism — but
  the *default* surface is the shell-shaped one this repo needs; a Rust
  consumer widens the globs.
- Positional justifications (a comment on/beside an `unwrap`/`allow`-class
  construct) ship as mechanism with the Rust construct set as the built-in
  roster — inert on a shell-only surface, active when a consumer widens.
- **Not-yet-swept components** ride the existing `exception-list:`
  WHITELIST pattern (each entry drained by a filed task per
  `check-gate-exemption-tasks`) — this repo's initial roster is whatever
  kits build cannot sweep in one session, drained kit by kit.

Blessed exceptions per se — the second question the entry raised — thus
survive, but only as *directive vocabulary*, never as a free-form
allowlist of prose comments: a comment is either parseable by role or it
moves to the SPEC.

New names on governed surfaces: `check-comment-tier.sh` and the three
knobs above (feature litmus satisfied). Registered in this repo's
`gates.list`; fixture pair `good/`+`bad/` under
`spec-kit/gate-tests/check-comment-tier/`.

## Producers and consumers

- **Producer:** the pre-commit battery (`gates.list` registration) and
  `run-gates.sh`; the gate scans the working tree via `gate_find` like its
  siblings.
- **Consumers:** the committing agent receives the findings with the
  standard three-way help line (relocate to SPEC / delete / exempt with
  reason); `check-gate-exemption-tasks` consumes the WHITELIST entries;
  gate-sdk's meta-gates consume the gate itself (output, fail-closed,
  fixture-pair, self-lint contracts).
- **Fields:** knob values are read once at gate start from
  `lib/spec.sh` config resolution, same as every `SPEC_KIT_*` knob.

## Existing sections updated

- `spec-kit/SPEC.md §What stayed on the platform` — the
  `check-comment-tier` sentence is rewritten: the *product directive
  vocabulary* stays behind; the classifier, the kit-mechanism roster, and
  the surface/positional machinery ship here. "The SPEC↔code tiering rule
  ships as prose only" is retired.
- `spec-kit/SPEC.md §Content tiering — the star topology` — the comment
  surface joins the enforced surfaces, citing the new gate.
- `spec-kit/README.md` — gate count and listing updated (five → six).
- This repo's `scripts/gates.list` — registration.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls spec-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
