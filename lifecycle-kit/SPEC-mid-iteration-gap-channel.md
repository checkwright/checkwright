# SPEC amendment: mid-iteration gap channel

Mid-iteration work-state writes race the stage session holding the shared git
index: a gap surfaced mid-stage has no committed place to land except the queue
file that stage session is already contending on. So mid-iteration gap *filing*
gets a committed, append-only channel of its own. Surfaced 2026-07-17 under
live shared-index pressure (the release-in-lifecycle lead session); premises
re-verified against the tree 2026-07-17 at promotion.

The companion stage-cursor extraction — retiring the queue header's `[stage:]`
field so the evidence file's last stamp becomes the sole cursor — was demoted
to the deferred `stage-cursor-extraction` queue unit during the
concurrency-hardening align audit, when the cross-kit `[stage:]` readership
proved a missed scope premise (the field is an interface consumed by
evidence-kit, context-kit, delegation-kit, and queue-kit, not a
lifecycle-internal copy). Its design is preserved in this file's git history
for re-promotion; this amendment now covers the gap channel alone, which is
independent of that extraction.

## What changes

### 1. The committed gap inbox

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

### 2. The kfric seam (drift-kit)

kfric stays the narrow knowledge-friction sensor — a fact re-derived because
no doc owns it, logged to the gitignored per-iteration
`.workflow/knowledge-friction.log`. drift-kit/SPEC.md §The knowledge-friction
loop gains one seam sentence: *work-shaped* mid-iteration findings (a gap, a
task, a defect) are not knowledge friction and route to the consumer's gap
channel; overloading the kfric log as a backlog inbox dilutes the `kpi-knowledge-friction`
signal it exists to carry.

## Producers and consumers

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

## Existing sections updated (at merge)

- lifecycle-kit/SPEC.md §Layout and configuration — the knob roster gains
  `LIFECYCLE_KIT_GAP_INBOX_FILE`; the layout gains `bin/file-gap.sh` and the
  inbox file.
- §Multi-operator semantics — the inbox joins the surface roster with
  `merge=union` (contrast with the iteration-scoped supersede set, which uses
  the keep-ours `merge=iteration-scoped` driver): an iteration-scoped surface
  is superseded at the boundary, but a gap filed on either side of a concurrent
  merge must survive.
- §bin/enter-stage.sh — the first-stage (iteration-boundary) entry gains the
  inbox refusal, the same refuse-before-write contract as the Lessons-section
  refusal it sits beside.
- §bin/install-lifecycle.sh, §check-merge-attrs — the union-merge roster row
  (the installer emits it; check-merge-attrs verifies its presence and parity).
  Note: `merge=union` is git-native, so no per-clone driver registration is
  needed (unlike the keep-ours `merge.iteration-scoped.driver`).
- §templates/skills/ — close.md's drain step.
- drift-kit/SPEC.md §The knowledge-friction loop — the kfric seam sentence.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — the gap-channel half retires no wording (it is
      pure addition); the coupled `[stage:]`/`flip+stamp`/`lifecycle_header_stage`
      removals moved with the demoted `stage-cursor-extraction` unit and are that
      unit's obligation, not this one's.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
