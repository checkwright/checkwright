# SPEC amendment: mid-iteration gap channel + stage-cursor extraction

Two coupled halves of one seam ruling — mid-iteration work-state writes race
the stage session holding the shared git index, so (1) the fast-mutating stage
cursor leaves the queue file, and (2) mid-iteration gap *filing* gets a
committed channel of its own. Surfaced 2026-07-17 under live shared-index
pressure (the release-in-lifecycle lead session); premises re-verified against
the tree 2026-07-17 at promotion.

## What changes

### 1. The stage cursor derives from the evidence file (the flip dies)

- The queue header grammar narrows to `## Iteration: <name>` — the `[stage:]`
  field is retired. The cursor's single source becomes the **last data line**
  of the evidence file (`.workflow/WORKFLOW-STATE.txt`): its `<stage>` token
  already records every stage entry, so the header field was a second copy of
  a derivable fact, bought at the price of one queue write per stage entry
  (derivation-first; removing the duplication outranks gating it — the
  header/stamp agreement assertion existed only to hold the copy in sync).
- §The flip+stamp protocol renames to **The stamp protocol**: the arriving
  stage's skill *stamps* as its first step; nothing flips. The enforcement
  property is preserved — the entry commit still stages the evidence file, and
  every queue/state-coupled gate re-fires on it via the graph couples, so the
  prior stage's machine-expressible exit is still re-verified at entry.
- Mid-iteration, the queue file is written only for real work-state
  transitions: promotion + naming (scope), the Done move riding each
  amendment-merge commit (canon-kit's pairing already forces same-commit),
  and close's dispositions. Stage motion never touches it.

### 2. The committed gap inbox

- New governed surface: **`.workflow/gap-inbox.md`** (knob
  `LIFECYCLE_KIT_GAP_INBOX_FILE`, default
  `${GATE_SDK_WORKFLOW_DIR:-.workflow}/gap-inbox.md`) — a committed,
  append-only capture buffer for mid-iteration gap filing. Grammar: a
  `# contract:` prose header, then one `- <YYYY-MM-DD> — <gap prose>` bullet
  per gap. Committed, not gitignored: a per-clone buffer fragments the backlog
  across operators — the finding that ruled kfric out as the channel.
- New affordance: **`bin/file-gap.sh "<gap prose>"`** on the `kfric.sh`
  pattern — repo-root cd, config-via-env, appends one dated bullet, exit 2 on
  misuse (empty argument).
- Merge semantics: `merge=union` in the consumer's `.gitattributes`
  lifecycle-kit block (append-only bullets union cleanly; unlike the
  iteration-scoped surfaces, a gap filed on either side must survive the
  merge). `check-merge-attrs`'s roster gains the entry;
  `bin/install-lifecycle.sh` writes it.
- Boundary refusal: `enter-stage.sh`'s first-stage (iteration-boundary) entry
  refuses while the inbox holds bullets — the existing Lessons-section refusal
  pattern, same exit contract — so no gap outlives its iteration untriaged
  (gap disposition: costed and filed, never flagged-and-skipped).

### 3. The kfric seam (drift-kit)

kfric stays the narrow knowledge-friction sensor — a fact re-derived because
no doc owns it, logged to the gitignored per-iteration
`.workflow/knowledge-friction.log`. drift-kit/SPEC.md §The knowledge-friction
loop gains one seam sentence: *work-shaped* mid-iteration findings (a gap, a
task, a defect) are not knowledge friction and route to the consumer's gap
channel; overloading the kfric log as a backlog inbox dilutes the `kpi-knowledge-friction`
signal it exists to carry.

## Producers and consumers

- **Cursor (last stamp line).** Producer: `bin/enter-stage.sh` appending the
  stamp (unchanged writer, now sole). Consumers: a new `lib/stages.sh` helper
  `lifecycle_current_stage <state-file>` (the last-data-line read
  `enter-stage.sh` already performs inline today, hoisted so every reader
  shares one derivation) — used by `checks/check-stage-entry.sh` (derives the
  entered stage; today it parses the header) and `checks/check-stage-evidence.sh`
  (name-axis agreement only, below). `lifecycle_header_stage` is retired with
  no remaining reader.
- **Preflight hand-off.** `enter-stage.sh` today hands `check-stage-entry` a
  temp *queue* with the flipped header; post-change it hands a temp *evidence
  file* with the candidate stamp appended (queue passed through unchanged).
  Same refuse-before-write contract, including the `LIFECYCLE_KIT_ENTRY_PREFLIGHT`
  hook, whose commands receive the same `<queue> <state>` argv — the temp
  file swaps sides.
- **Gap bullet.** Producer: any mid-iteration session (lead or stage) via
  `bin/file-gap.sh` — enabling config is the knob default, live everywhere the
  kit is vendored. Consumers: the close skill's triage step
  (templates/skills/close.md gains the drain: every bullet dispositioned —
  promoted to a deferred `[needs-spec]` queue entry, fixed inline that
  session, or discarded with cause in the close commit message — and the
  inbox truncated to its header); the boundary refusal reads emptiness at
  the next scope entry as the backstop. Each bullet's two fields have named
  readers: the date feeds close's staleness judgment, the prose is the
  disposition body.
- **Residual `[stage:]` healing.** Producer: the first-stage boundary reset
  rewrites the header and drops a residual pre-upgrade `[stage:]` field.
  Consumer: the header parsers, which ignore a trailing bracketed field until
  then (a vendored consumer upgrades mid-iteration without a red).

## Existing sections updated (at merge)

- lifecycle-kit/SPEC.md §The state machine — the two governed surfaces: the
  header carries the slow axis (iteration name) only; the evidence file
  carries stamps *and is the cursor*; "every flip and stamp" wording.
- §The flip+stamp protocol → §The stamp protocol (rename + body).
- §Deviation transitions — "header/stamp agreement" citation in the split
  ritual re-words to the name-axis rule.
- §Layout and configuration — knob roster gains `LIFECYCLE_KIT_GAP_INBOX_FILE`;
  layout gains `bin/file-gap.sh` and the inbox file.
- §Multi-operator semantics — the inbox joins the surface roster with
  `merge=union` (contrast with iteration-scoped); queue-contention paragraph
  updated (stage motion no longer writes the queue).
- §bin/enter-stage.sh — non-first entries write only the evidence file; the
  preflight temp-file swap; the inbox boundary refusal; residual-field
  healing.
- §bin/install-lifecycle.sh, §check-merge-attrs — the union-merge roster row.
- §check-stage-entry, §check-stage-evidence — cursor source; the evidence
  gate's agreement assertion narrows to the name axis (the "current stage has
  a matching stamp" clause is vacuous once the last stamp *is* the stage).
- §templates/skills/ — the skills' first-step prose (stamp, not flip+stamp);
  close.md's drain step.
- §templates/lead.md — dispatch prose citing the flip.
- lifecycle-kit/README.md, docs/orchestration.md — flip wording, header shape.
- CLAUDE.md lifecycle block — in lockstep via `check-lifecycle-registration`.
- drift-kit/SPEC.md §The knowledge-friction loop — the kfric seam sentence.
- Gate fixtures: lifecycle-kit gate-tests for check-stage-entry /
  check-stage-evidence / check-lesson-disposition and `smoke/` scripts carry
  `[stage:]` headers — rewritten to the new grammar.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec, template, doc, and fixture
      for `[stage:]`, `flip+stamp`, and `lifecycle_header_stage`; nothing
      dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
