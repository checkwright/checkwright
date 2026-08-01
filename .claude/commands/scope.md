Execute the template at lifecycle-kit/templates/stages/scope.md, applying the bindings below.

## Bindings

**exit-condition** — the design is written down (this session's plan or a SPEC
draft) and the seam is ruled: what ships as mechanism, what stays private rule
content, what becomes consumer config.

**evidence-reset** — nothing to reset by hand here:
`LIFECYCLE_KIT_BOUNDARY_TRUNCATE` (`scripts/lifecycle-config.sh`) lists
`.workflow/validate-evidence.txt`, so `enter-stage.sh scope` truncates it —
together with the lesson-evidence file — in the same stamp commit. Wipe
`.tmp/`'s files at this boundary — **except** the session-role marker
(`.tmp/session-role`), which a live dispatching lead's session-context hook
still reads: its lifetime is the lead session's, not the iteration's
(context-kit/SPEC.md §The session-context hook), so deleting it here would
silently drop the lead's role suppression mid-session. The rest is purely
disposable scratch (CLAUDE.md §Housekeeping — persistent trends live in
`.metric/`), and a resume journal from a closed iteration is dead by definition.

**ritual** — read `BRIEF.local.md` (local-only brief); run the GitHub
boundary sweep (below); decide the unit's
layout, config surface, and worklist; name the iteration after the unit. Hold
the provenance seam per CLAUDE.md §The provenance seam (never cross it) and
config-via-env per CLAUDE.md §Conventions established in gate-sdk. The amendment
lifecycle the template's triage step invokes is canon-kit/SPEC.md §The
amendment lifecycle; the queue-entry grammar the promotion step writes
against is queue-kit/SPEC.md §The tag algebra, and `[spec:]` ref
resolution is canon-kit/SPEC.md §check-amendment-queue.

**The GitHub boundary sweep** — after the brief read, before promotion triage,
sweep the public repo's intake. `TASK-QUEUE.md` stays the sole owner of work
state and nothing lives triaged-but-unqueued across the three lanes below —
with one sanctioned exception, stated here because it is what makes the claim
true rather than merely asserted: an **unpublished advisory**, whose record is
its own thread until publication (Advisories, below). Cap: the top five
items per lane per boundary — a per-item analysis heavier than a read is
delegated under the usage verdict (CLAUDE.md §Agent execution). Overflow beyond
five stays on the tracker for the next boundary (the tracker's own open-item
list is the carry — no shadow copy). The `gh` calls run only inside this
interactive sweep; no pre-commit or session-context hook makes a network call.

- **Issues** — `gh issue list --state open`; take the top five, oldest first.
  Each gets exactly one disposition: *promoted* — a queue entry in this file's
  grammar, its body citing the issue on the existing provenance sentence
  (`Surfaced <date> by GitHub issue #N` — no new tag, queue-kit/SPEC.md §The
  tag algebra untouched) — or *closed with cause*, a comment naming the reason.
  No linked-and-skipped middle state (gap disposition).
- **PRs** — `gh pr list --state open`; take the top five. A PR is reviewed work
  under CONTRIBUTING.md's fixture-is-the-unit rule, never a task. Each gets one
  of three dispositions: *merged* (battery-green, in-convention), *closed with
  cause*, or *reviewed with findings* — the findings warranting design or
  follow-on work become queue entries citing `PR #N`, posted as the review. The
  third is a disposition, not a skip: the PR carries an actionable review after.
- **Advisories** — `gh api 'repos/{owner}/{repo}/security-advisories?state=triage'`;
  take the top five, oldest first. `triage` is GitHub's documented state for an
  advisory a reporter submitted through private vulnerability reporting and which
  awaits maintainer action; accepting one moves it to `draft`, so `triage` is
  exactly the undispositioned set (REST *List repository security advisories*,
  whose `state` enum is `triage`/`draft`/`published`/`closed`). **Read that
  literal from here, never from memory:** the endpoint does not validate the
  parameter — an unknown state returns `[]` at exit 0, indistinguishable from a
  clean lane — so a misremembered literal is a silent fail-open the sweep cannot
  feel.

  This lane's items are secret, which is the whole of how it differs from its
  siblings: its dispositions are stated **on the advisory thread**, and until the
  advisory is published the public tree says nothing. Not a redacted entry, not a
  placeholder slug, not a count — a public entry naming a fix's shape against an
  unpublished advisory tells a reader which surface to look at and when, so
  partial disclosure is refused rather than formatted. Four dispositions, each
  terminal:
  *fix under embargo* — tracked on the thread; its queue entry lands **at
  publication**, on the existing provenance sentence (`Surfaced <date> by security
  advisory <id>`, no new tag, as above);
  *advisory-only* — assessed, no code change owed, published with the assessment,
  nothing enters the queue;
  *declined with cause* — the thread carries the reason and the advisory is
  closed (the Issues lane's *closed with cause* under embargo);
  *not a vulnerability, but real work* — an ordinary defect leaves the lane for
  ordinary intake immediately, so the embargo rule can never park ordinary work
  out of sight.
  No linked-and-skipped middle state (gap disposition), as above.

**handoff** — lay out the branches per docs/orchestration.md §Running an
iteration under a lead. Dispatched by a live split-posture lead, there is no
handoff: finish the stage, report back, and stay resumable — the lead will
route intent questions here for the rest of the iteration. Run by hand, the
choices are the unified shape (`/compact` this session, then `/lead` in it) or
no lead at all, the operator consulting this session when a stage asks. That
page is the sequence's single source (owned by
lifecycle-kit/SPEC.md §templates/lead.md) — send the reader there, do not
transcribe its steps.
