# spec-kit — spec discipline for agent-authored components

One canonical spec per component, deltas as short-lived amendment files, and
a star topology across the prose surfaces: every fact has exactly one owning
surface, and every other surface cites it — never restates it. The problem
the kit solves: when coding agents author the specs, design rationale gets
re-derived under build pressure unless it is captured at scope, and a
parallel copy of any gated fact is an un-gateable second source that drifts
silently. The remedy is a lifecycle (amendments are written at scope,
merged and deleted at build) and gates over the copy-shaped failure modes.

Extracted from the governance meta-layer of a private production platform.
The kit carries the lifecycle, the checklists, and five gates; a consumer's
surface names, banned headings, and scan languages are config with the
platform's values as defaults. Requires [gate-sdk](../gate-sdk/) (the gates
follow its four contracts and resolve through its registry); the queue
lifecycle gate reads the tag syntax [queue-kit](../queue-kit/) defines.

## The spec model

Every component has exactly one canonical spec file (default `SPEC.md`) —
complete, current, the single source of truth. A spec has three jobs, and
"copying the code into prose" is not one of them:

1. **Owns semantics** — invariants, ordering, error behavior; the prose no
   other artifact carries.
2. **Names contracts** — every interface is named, and each name is tied to
   the implementation by a gate; a named-but-ungated contract is the rot
   vector.
3. **Never copies a structural definition** — reference the code, do not
   copy it. Anything readable from the code (exact types, field layouts,
   config values) is implementation, not spec content; an ungated verbatim
   copy drifts silently the moment the code changes
   (`check-spec-embedded-source`).

A **SPEC amendment** (default glob `SPEC-*.md`, in the owning component's
directory) is the delta artifact for a designed-but-unimplemented change:
it describes only what is added or changed, is named after the feature
(`SPEC-sqlite.md`, never `SPEC-PHASE3-SQLITE.md`), and is a transition
artifact — merged into the canonical spec and deleted when the work
completes; an amendment never outlives its implementation. The one
sanctioned copy exemption: an amendment may embed a wire-contract delta
(e.g. a fenced proto block) until merge, because it is the design home for
a contract that does not exist yet; the canonical spec cites the contract
file, never re-embeds it. A ruling with no owning component (governance,
workflow, gates) is a **root-level amendment**: same lifecycle, lives at
the repo root, merges into the consumer's rulebook instead of a component
spec.

## The amendment lifecycle

Spec-writing is a scope-stage activity — build sessions never author specs.
A feature task is therefore in exactly one of two states, marked with the
queue tags whose syntax queue-kit defines:

- **Design-pending** — no amendment yet; the entry sits in the deferred
  section tagged `[needs-spec]`, excluded from selection. Every deferred
  entry carries the tag: all deferred work is design-pending by definition.
- **Spec-ready** — the amendment exists; the entry sits in the feature
  section tagged `[spec: <ref>]`, eligible for selection.

The **bidirectional rule**: every feature entry in the active queue carries
a `[spec:]` ref that resolves to a file on disk, and every amendment on
disk has a queue entry pointing at it. Writing the amendment *is* promoting
the deferred entry — without the pairing, design rationale and ruled-out
alternatives get re-derived under build pressure. Technical debt needs no
amendment (it fixes behavior to an existing spec); a debt task that needs a
design *ruling* is design work — it goes to the deferred section
`[needs-spec]` until scope rules on it. Enforced by
`check-amendment-queue`.

### The causal-completeness check

Before an amendment is ready, every new state, event, and interface it
introduces passes four points:

1. **Producer named and reachable** — what code path, call, or timer
   triggers it; a named producer whose enabling config no deployed
   configuration actually sets is dead everywhere but unit tests.
2. **Consumer named** — what component receives it, by what mechanism
   (which stream, which call, which poll).
3. **Existing integration sections updated** — any affected spec section
   describing the prior flow is updated in the amendment itself.
4. **Every field has a named reader** — for each field on a new message,
   name the consumer that reads it and the transition where it is read; a
   field with no reader is removed, and a field read at one transition is
   not populated at others.

A cross-component causal gap that surfaces during build is not a deferred
TODO: stop, resolve it that session, update the spec before resuming.
lifecycle-kit's scope/build skill templates carry these hooks; spec-kit
supplies the checklist they invoke and the gate behind the promotion rule.

### Merging an amendment (on task completion)

1. Read the canonical spec fully, then the amendment.
2. Integrate — do not append: each addition lands in its proper section,
   and the merged spec reads as one coherent document a reader who never
   saw the amendment can use alone.
3. Delete the amendment file; verify none remain for the component.
4. Propagate removals (grep every spec for names the change retired), file
   discovered gaps as debt tasks, and commit the merge with the work.

The shipped amendment template ends in a Definition-of-Done checklist that
includes causal completeness, merged-with-no-information-lost, the
file-deleted assertions, and gap filing.

## Content tiering — the star topology

Every governed prose surface owns exactly one content tier and *points to*
— never restates — a fact owned by another surface. Which surfaces exist
and what each owns is the consumer's tier contract (the platform's is its
own instance and stays there); the kit's rules are the topology itself:

- **One owner per fact.** A parallel copy of a gated fact is the defect —
  a second source no gate reads, drifting silently. The fix keeps each
  surface's tier and replaces the foreign-tier slab with a pointer.
- **Definitions have one home.** A canonical bold-lead-in definition of a
  glossary term belongs only to the glossary; another surface may *use*
  the term, explain *why* it exists, or carry its local mechanism — never
  the definition (`check-surface-duplication`, with per-site valves for a
  surface that legitimately introduces a concept).
- **Quantitative literals are code-owned.** A count or enumerated set
  transcribed into prose is a parallel copy; cite the owning source
  instead. A literal stays verbatim only when load-bearing, and then only
  gate-coupled.
- **Honest mechanizability.** Only the structural sub-rules gate (banned
  headings, fence density, duplicate definitions, verbatim copies); the
  core judgment — is this sentence a definition or a narration, a why or a
  mechanism — is FP-prone and stays a review tripwire, explicitly not a
  blocking gate.

## Layout and configuration

The kit is vendored beside gate-sdk (conventionally at `spec-kit/`); its
five gates are registered in the consumer's `gates.list` by name — each
where its surface exists (a consumer with no glossary does not register
`check-surface-duplication`) — and resolve through gate-sdk's multi-kit
path.

Config follows queue-kit's pattern: copy `templates/spec-config.sh` into
the gates dir as `spec-config.sh` (or point `SPEC_KIT_CONFIG_FILE`
elsewhere) and override any knob; defaults fill what the consumer left
unset, and the loader exits 2 on a malformed config. Knobs:

- `SPEC_KIT_SPEC_NAME` — canonical spec filename, default `SPEC.md`.
- `SPEC_KIT_AMENDMENT_GLOB` — default `SPEC-*.md`.
- `SPEC_KIT_QUEUE_FILE` — default `${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}`.
- `SPEC_KIT_FEATURE_SECTIONS` — array, default `("New Features")`: active
  sections whose entries require `[spec:]`.
- `SPEC_KIT_ACTIVE_SECTIONS` — array, default
  `("New Features" "Technical Debt")`: sections where `[needs-spec]` is a
  violation.
- `SPEC_KIT_DEFERRED_SECTION` — default `Deferred`: the section whose
  every entry requires `[needs-spec]`.
- `SPEC_KIT_DOD_HEADING` — default `Definition of Done`;
  `SPEC_KIT_DOD_MODE` — `exactly-one` (platform default) or `at-most-one`
  (a reference-spec corpus like this repo's kits carries no DoD).
- `SPEC_KIT_BANNED_HEADINGS` — array, default
  `("Directory Structure" "Public API" "Cargo.toml Dependencies")`;
  `SPEC_KIT_DERIVABLE_DENSITY` — default `60` (percent fenced);
  `SPEC_KIT_DERIVABLE_POINTER_REGEX` — the index-pointer marker that exempts
  a shed section, default `pub-index|proto-index` (consumer index tooling).
- `SPEC_KIT_EMBED_THRESHOLD` / `SPEC_KIT_EMBED_MINLINES` — defaults `0.70`
  / `8`; `SPEC_KIT_EMBED_LANGS` — the scanned fence-language → source
  mapping, one `kind|fence-alias,…|file-glob,…` entry per language family,
  default = the platform list; `SPEC_KIT_EMBED_ILLUSTRATIVE` — fences
  illustrative-by-default, default `(json)`; `SPEC_KIT_EMBED_WIRE_KIND` — the
  one fence an amendment may embed as a not-yet-merged contract delta,
  default `proto`.
- `SPEC_KIT_GLOSSARY_FILE` — default `GLOSSARY.md`;
  `SPEC_KIT_DUP_SURFACES` — array of surfaces scanned for foreign
  definitions, default `(VISION.md)` plus every component spec.

Cross-kit note: the section knobs carry the same defaults as queue-kit's;
the knobs are independent (either kit runs without the other), so a
consumer renaming its sections sets both. Valve and marker spellings
(`[needs-spec]`, `[spec:]`, `vision-introduces` / `spec-introduces`,
`spec-embedded-source-exempt: <reason>`) are mechanism, not config.

## Per-component contracts

### lib/spec.sh

The sourced config loader plus shared adapters: section-regex builders for
the queue-facing gate (queue-kit's rule — both sides of a section boundary
must parse identically), and the canonical-spec/amendment finders the
spec-scanning gates share (a `templates/` skeleton is a copyable stub, not
governed content, so the finders skip it — the same rationale as the
gate-tests prune). Values and adapters only, never gate structure.

### check-amendment-queue

Invariant: the bidirectional rule holds — (a) no feature-section entry
without `[spec:]` and no `[needs-spec]` anywhere in the active sections
(entries or prose; a prose mention masks the tag's absence); (b) every
deferred entry carries `[needs-spec]`, and a deferred entry already
carrying `[spec:]` must be promoted; (c) every amendment on disk pairs
with a `[spec:]` queue entry and every `[spec:]` ref resolves to a file.

Calibration: a ref is a bare amendment basename (searched tree-wide) or a
repo-relative path (resolved directly — the generalization that lets a
consumer point a task at any design artifact, e.g. this repo's kit-SPEC
drafts). Sub-bullets and prose notes are outside the entry grammar;
`precommit` tier.

### check-spec-dod-singleton

Invariant: no canonical spec carries the configured Definition-of-Done
heading more than once (a duplicate checklist is the two-sources defect on
the completion contract); under `exactly-one` mode a spec with none is
also flagged.

Calibration: heading match is level-insensitive within `##`–`####`;
`at-most-one` exists because a reference-spec corpus legitimately has no
DoD. `align-only` tier.

### check-spec-derivable-section

Invariant: no canonical-spec section under a banned heading (the
configured code-derivable set) whose body exceeds the density budget in
fenced lines — such a section is a code dump that drifts; it sheds to a
one-line index pointer (which is exempt).

Calibration: density counts non-blank lines; the heading set and budget
are config because what is derivable depends on the consumer's index
tooling. `align-only` tier.

### check-spec-embedded-source

Invariant: no fenced block in a canonical spec verbatim-copies a tracked
source file above the overlap threshold — cite the path instead. Overlap
detection needs no author opt-in; the amendment wire-delta exemption and
the per-site `spec-embedded-source-exempt: <reason>` marker are the two
valves.

Calibration: blocks shorter than `SPEC_KIT_EMBED_MINLINES` are ignored;
languages in `SPEC_KIT_EMBED_ILLUSTRATIVE` are skipped by default; the
threshold is calibrated against real specs, not synthetic fixtures.
`precommit` tier.

### check-surface-duplication

Invariant: no unvalved bold-lead-in definition of a glossary term on a
configured non-glossary surface — the canonical definition form belongs
only to the glossary; elsewhere it is a second definition that drifts.
Valves `vision-introduces:` / `spec-introduces:` (same line or the line
above) make a legitimate narrative or local introduction a reviewable
decision in the diff.

Calibration: a bold lead-in naming a component (a directory owning a
canonical spec) is never flagged — a component's definition lives in its
own spec, so there is nothing to restate; gate-test fixtures are excluded;
exits 2 when the glossary file is absent (register the gate only where the
topology exists). `align-only` tier.

### templates/

`spec-config.sh` — the consumer config template documenting every knob.
`SPEC-amendment.md` — the amendment skeleton: delta sections plus the
Definition-of-Done checklist (causal completeness, no information lost on
merge, amendment deleted, none remaining, gaps filed as debt).

## What stayed on the platform

The platform's tier contract — which surfaces exist and what each owns —
is its own instance; spec-kit ships the topology rules, not the table. The
glossary/vision structural gates (`check-glossary-tiering`,
`check-glossary-entry-types`, `check-vision-tiering`,
`check-retired-term-coupling`) encode that instance's surface roles,
entry taxonomy, and ubiquitous-language couplings — rule content; their
generic cross-surface axis ships here as `check-surface-duplication`.
`check-comment-tier`'s blessed directive set is a coupling vocabulary
(rule content), so the comment surface stays behind; the SPEC↔code
tiering rule ships as prose only. `check-root-tiering` is a pure
consumer-filename allowlist with no mechanism residue. Global-constant
literal gates (the pagination-literal pattern) are per-constant rule
content. Diagram/spec annotation couplings are a later kit's scope.
