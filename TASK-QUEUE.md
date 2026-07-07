# TASK-QUEUE.md — Checkwright work queue

## Iteration: hardening  [stage: close]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

## Technical Debt

## Deferred

- **drift-kit-extraction** [needs-spec] — drift-report skeleton with pluggable
  KPIs and lead/lag honesty labels (kit 7). Scope ruling 2026-07-07: the next
  iteration after `hardening`, bundled with knowledge-friction-loop (whose
  re-derivations/session KPI is drift-kit's first pluggable-KPI consumer).
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
  Surfaced 2026-07-07. Scope ruling 2026-07-07: bundled into the drift-kit
  iteration (see drift-kit-extraction).
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
  as its own iteration post kit 7. Evidence artifact: upstream Claude Code
  issue #75214 (project config can't lift the Task ask-first default),
  surfaced dogfooding this repo's delegation nudge 2026-07-07.
- **manifest-temporal-gate** [needs-spec] — spec-kit tiering-family gate: a
  cheap lexical tripwire flagging old-behavior narration ("previously",
  "formerly", "no longer", "used to", "renamed from") in tracked manifest
  prose (SPEC*.md, README, the always-loaded surface). History is derivable
  from git; a manifest states current behavior only, and a "formerly…" line is
  standing context cost documenting the old cost — it taxes every session that
  reads it. Mechanizes what close-brevity today leaves to manual judgment ("the
  semantic residue check-brevity cannot decide",
  context-kit/templates/close-brevity.md). Scope ruling owes the calibration:
  the cheap-and-FP-free bar is the crux — this repo's own extraction prose is
  deliberate provenance ("renamed from the platform's X"; every "What stayed on
  the platform" section), so the gate needs an exempt escape (the
  check-spec-embedded-source `-exempt:` pattern) and must be tuned against these
  SPECs as the FP corpus. Surfaced 2026-07-07.
## Done

- identity-assertion-check
- check-spec-pointer
- core-file-presence-gate
- kit-terminology-renames

## Lessons Learned
