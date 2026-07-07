# TASK-QUEUE.md — Checkwright work queue

## Iteration: context-kit  [stage: build]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

- **stage-entry-mechanization** [spec: lifecycle-kit/SPEC-enter-stage.md] —
  `enter-stage.sh <stage>`: the stamp+flip ritual mechanized with a
  check-stage-entry pre-flight, first-stage boundary reset included; skills
  and templates invoke it as their first step; gates stay the independent
  verifier.
- **comment-tier-gate** [spec: spec-kit/SPEC-comment-tier.md] — spec-kit's
  check-comment-tier: comments must be machine/reason directives or exempted;
  kit-mechanism directive roster as default, consumer vocabulary via
  SPEC_KIT_COMMENT_* knobs; sweep this repo's kits (WHITELIST-drained).
- **tag-lead-line-rename** [spec: queue-kit/SPEC-tag-lead-line.md] — rename
  check-blocked-by-lead-line to check-tag-lead-line: the gate governs every
  blocked-by/spec/needs-spec tag, the name claims only blocked-by; registry,
  SPEC section, fixture dir, and the generated hook artifacts move with it.

## Technical Debt

## Deferred

- **core-file-presence-gate** [needs-spec] — gate-sdk gate blocking deletion of
  a consumer repo's required core workflow files (moved from the platform
  queue 2026-07-07 — generic by construction; the first born-in-checkwright
  gate, proving the SDK supports new gates, not just extracted ones). Problem:
  with no required-file inventory and no `--diff-filter=D` deletion check, a
  lone `SPEC.md`, `TASK-QUEUE.md`, `WORKFLOW-STATE.txt`, or projection HTML
  can be `git rm`'d and committed; downstream gates catch it only
  incidentally (a hard-reference happens to dangle) and only at the next
  stage that runs them; the platform's bash-guard blocks `find … -delete`
  but not plain `rm`/`git rm`. Design owes: the inventory source (a committed
  per-repo manifest vs deriving the set from gate hard-references vs a glob
  of top-level `*.md` + `*.html` + the SPEC.md set), the tier (precommit, so
  it fires without a stage skill), and the intentional-removal valve (how a
  deliberately retired surface — e.g. a merged `SPEC-<feature>.md` — passes
  without weakening the gate). First consumer: the platform, which adopts it
  as `checkwright-kit-adoption` step 0 and whose cutover deletion sweep
  exercises the valve. Surfaced in the platform 2026-07-04.
- **drift-kit-extraction** [needs-spec] — drift-report skeleton with pluggable
  KPIs and lead/lag honesty labels (kit 7).
- **knowledge-friction-loop** [needs-spec] — the friction log / scan-prompts /
  close-triage loop surfaces *permission* friction only; two classes escape it:
  knowledge friction (re-deriving a fact no doc owns — e.g. the merge-closure
  queue step read off check-amendment-queue rather than spec-kit's merge
  procedure) and action friction (repeated low-value tool sequences). Cheap
  design mirrors the friction log: capture a re-derivation as a one-liner the
  moment it happens (a knowledge-friction.log) and triage it at close like
  scan-prompts — but the remediation is always a doc-owner edit (give the fact a
  home under the star topology), never a standing session-start instruction that
  the context-kit brevity meter is built to reject. Detection is the loop;
  elimination is a tiering edit. Aggregate view belongs in drift-kit (kit 7) as
  a "re-derivations/session" lagging KPI that trends down as holes fill. A full
  transcript LLM-scan is the heavy alternative; the writing-project reduction —
  keep only each party's messages, drop tool calls/results/reminders — shrinks
  the derived transcript substantially and makes a periodic scan affordable.
  Surfaced 2026-07-07.
- **identity-assertion-check** [needs-spec] — a consumer repo commits its
  expected git identity (committer email; remote host / SSH key identity) and
  a cheap FP-free check verifies local config matches, at setup and/or
  pre-push. Failure mode (hit twice on the platform, latest 2026-07-07): an
  agent commits or pushes under the wrong identity — misattribution is silent
  and unpurgeable without a SHA-breaking history rewrite, and the wrong-key
  symptom is a misleading "Repository not found"; multi-identity (work +
  personal GitHub) is common for the integrator/consultant audience. Scope
  fence: the *mapping* stays git's job (core.sshCommand, includeIf) — this is
  only the verification backstop for the fresh-clone gap, alongside
  install-hooks' apply-and-verify rung; likely friction-kit (guard) or the
  setup story. Surfaced in the platform 2026-07-07.
- **kit-terminology-renames** [needs-spec] — user ruling wanted on the two
  heavier renames: friction-kit to guard-kit (the kit's core is lib/guard.sh;
  "friction" is platform insider vocabulary) and delegation-kit usage-gate.sh
  to usage-verdict.sh (leave "gate" meaning one thing product-wide); cheapest
  while no external consumer exists.
- **ddd-positioning-docs** [needs-spec] — docs page plus example consumer
  config positioning Checkwright for DDD ubiquitous-language enforcement
  (vocabulary via the check-graph/graph-vocab pattern, comment-tier
  directives); mechanism kits stay DDD-neutral — the coupling lives in docs
  and examples only; natural landing slot is alongside drift-kit (kit 7).
- **scope-session-routing** [needs-spec] — iteration-ambiguity routing across
  sessions: a build/align session forwards a question to the still-live scope
  session and relays the reply back; design it atop the harness's native
  agent-to-agent messaging (SendMessage/subagents) rather than bespoke
  plumbing — the substrate is moving fast and bespoke plumbing would be
  obsoleted; likely a companion tool or repo, not a kit; triage post kit 7.
- **adoption-track** [needs-spec] — the outward-facing rung: docs site, demo
  walkthrough, announcement post, plugin-marketplace presence; this is the
  path that makes any absorption/standardization outcome possible; scope it
  as its own iteration post kit 7.
## Done

- allowlist-chain-steer-rule
- context-kit-extraction

## Lessons Learned
