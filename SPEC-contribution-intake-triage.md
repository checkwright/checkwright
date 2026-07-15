# SPEC amendment: contribution-intake-triage

Repo-root governance ruling (no owning kit): the public repo's GitHub issues
and PRs become an owned intake surface. Merge targets: `CONTRIBUTING.md`,
`.claude/commands/scope.md` (the scope-skill ritual binding), the `.github/`
issue/PR templates. This is this repo's process, not kit mechanism —
extraction into a kit is demand-gated on a second consumer asking for it.

## What changes

- **Intake, never a second queue.** `TASK-QUEUE.md` stays the sole owner of
  work state. The tracker holds only what has not yet been swept; nothing
  lives there in a triaged-but-unqueued state.
- **The boundary sweep (issues).** The scope ritual gains one step, after the
  brief read and before promotion triage: enumerate open issues
  (`gh issue list --state open`), take the top **five, oldest first**, and
  give each exactly one disposition — *promoted* (a queue entry in this
  file's grammar, its body citing `issue #N`) or *closed with cause* (a
  comment naming the reason). No linked-and-skipped middle state (gap
  disposition). The overflow beyond five is left on the tracker for the next
  boundary — the tracker itself is the carry, no shadow list.
- **The PR lane.** The same sweep enumerates open PRs (`gh pr list
  --state open`). A PR is reviewed work under CONTRIBUTING.md's
  fixture-is-the-unit rule, never a task. Each swept PR gets one of three
  dispositions: *merged* (battery-green, in-convention), *closed with
  cause*, or *reviewed with findings* — the findings that warrant design or
  follow-on work become queue entries citing `PR #N`, posted as the review.
  The third disposition is a disposition, not a skip: the PR carries an
  actionable review after it.
- **Budget cap.** The sweep is capped at the top five items per lane per
  boundary; per-item analysis heavier than a read is delegated under the
  usage verdict (CLAUDE.md §Agent execution — the `/agent-execution`
  protocol and its budget guard). A flood of intake defers to later
  boundaries; it cannot burn the scope session's window on its own analysis.
- **No new queue grammar.** The issue/PR citation rides the existing
  entry-body provenance sentence (`Surfaced <date> by GitHub issue #N`) — no
  new tag, no queue-kit change. queue-kit/SPEC.md §The tag algebra is
  untouched.
- **Enforcement point.** The ritual checklist only: the binding is
  load-triggered at every `/scope`, so the step cannot be forgotten by a
  session that runs the skill. Pre-commit stays offline and the
  session-context hook gains no network call — `gh` runs only inside the
  interactive sweep.
- **The seam rides the templates.** The gate-defect issue form and the PR
  template each gain one caution line: a report or fixture describes generic
  mechanism, never the reporter's private rule content (term lists, coupling
  vocabularies, product constants) — mirror of CLAUDE.md §The provenance
  seam, pointed at contributors.

## Producers and consumers

- **The sweep step** — producer: the scope session executing the ritual
  binding (the binding edit is the enabling config, live at every `/scope`
  invocation; no other trigger exists and none is needed — intake waits at
  most one iteration). Consumers: `TASK-QUEUE.md` (promotions land in queue
  grammar) and the GitHub tracker (closures with cause, PR reviews).
- **The cap (five per lane)** — reader: the scope session, which stops
  enumerating; the overflow's reader is the next boundary's sweep, via the
  tracker's own open-item list (no second copy).
- **The template caution lines** — reader: the contributor filling the form;
  no machine reads them.
- **The CONTRIBUTING cadence sentence** — reader: the contributor deciding
  whether to wait or ping; it sets the expectation that triage happens at
  iteration boundaries under the existing no-SLA support posture.

## Existing sections updated

- `CONTRIBUTING.md` §Report a gate defect and §Pull requests gain the
  disposition contract (what happens to a filed issue / opened PR, and
  when); §Support gains the cadence sentence. The merge keeps
  CONTRIBUTING.md's content-tier: contributor-facing expectation, not
  maintainer mechanics.
- `.claude/commands/scope.md` **ritual** binding gains the sweep step with
  the two `gh` one-liners, the cap, and the disposition table — the
  maintainer-side mechanics live here (load-trigger residency: resident
  exactly when a scope session runs).
- `.github/ISSUE_TEMPLATE/gate-defect.yml` + `.github/pull_request_template.md`
  gain the seam caution line each.
- Verify at build: docs/orchestration.md and lifecycle-kit's scope template
  describe the scope stage — confirm neither enumerates ritual steps in a
  way this addition contradicts (expected: bindings are consumer content,
  no change needed).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper home (CONTRIBUTING.md sections, the ritual binding, the
      templates); each surface reads as one coherent document alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
