# SPEC amendment: completeness-gates

The small missed-mechanism cluster from the platform second scan:
content-free structural gates the first extraction pass skipped, each
ruled here against §Minimal footprint — a real, cheap, mechanically
decidable, non-redundant drift axis on a surface this repo actually has
earns a slot; anything else stays on the platform. Admitted: six gates
across four kits. Ruled out: six candidates, rationale at the end (two of
them because drift-kit already ships the mechanism — the scan list
predated kit 7).

## What changes

spec-kit:

- **`check-spec-fence-balance`** (precommit) — every governed markdown
  file carries an even count of fence delimiters. The fence-skipping
  parsers across the kits (embedded-source, tag-lead-line, the queue
  scanners) all toggle a fence flag; an odd count desyncs the flag and the
  rest of the file fails *open*. This gate turns that silent hole into a
  red. Surface: the spec/manifest set `lib/spec.sh` already computes — no
  new knob.
- **`check-md-refs`** (precommit) — every internal markdown link in the
  governed doc set resolves: relative path to a tracked file, `#anchor`
  to a heading slug in the target. External URLs are out of scope
  (network is not a gate dependency). Untracked local-only files
  (`BRIEF.local.md`) are legitimate link *sources* but never required
  targets — the scan runs over tracked files only. Doc set: the same
  manifest set plus kit READMEs; `SPEC_KIT_MDREF_EXCLUDE` (glob list,
  default empty) valves a consumer's generated docs.

lifecycle-kit:

- **`check-stage-skill-coverage`** (precommit) — both directions between
  the configured stage set and the skills dir (default
  `.claude/commands/`): every stage has a `<stage>.md` skill file, and
  every skill file that invokes `enter-stage.sh` names a stage in the set
  (the invocation is the mechanical marker separating stage skills from
  ordinary skills, so a retired stage's orphan skill reddens without
  false-flagging `/agent-execution`). Knob for the dir:
  `LIFECYCLE_KIT_SKILLS_DIR`.

gate-sdk:

- **`check-hook-exec-bit`** (precommit) — every tracked file in the hooks
  dir carries the executable bit in the git *index* (mode 100755): git
  silently skips a non-executable hook, which disables the entire gate
  battery for every fresh clone — catastrophe-class, one `git ls-files -s`
  cheap. The index is the checked surface because it is what a clone
  receives; `install-hooks.sh`'s per-clone `chmod` cannot repair a wrong
  committed mode.
- **`check-root-tiering`** (precommit) — the repo root holds only an
  allowlisted orientation set (tracked top-level files and dirs ⊆
  `GATE_SDK_ROOT_ALLOWLIST`); workflow machinery stays under the
  configured workflow/gates dirs. Agent-authored repos accumulate root
  scratch by reflex; the allowlist makes a new root surface a deliberate
  config edit. The allowlist is consumer config (rule content); the kit
  default is the minimal orientation set (`README.md`, `LICENSE`, the
  configured queue file, `CLAUDE.md`, `.gitignore`).

queue-kit:

- **`check-queue-sections`** (precommit) — the queue file carries each
  required `##` section heading exactly once
  (`QUEUE_KIT_REQUIRED_SECTIONS`, default: the iteration header plus New
  Features / Technical Debt / Deferred / Done / Lessons Learned). Every
  section-scoped scanner (amendment-queue, task-names, conservation, the
  session-context index) locates work by these headings and finds nothing
  — fails open — when one is typo'd or dropped; this gate is the
  fail-closed floor under all of them.

Each gate ships the skeleton-derived four contracts, a `good/`+`bad/`
fixture pair, registration in this repo's `scripts/gates.list`, and a
`# graph:` couples line (using the `kit:` token where it spans kits, per
the kit-enum amendment — build order: kit-enum first).

## Ruled out (stays on the platform, or already shipped)

- **handbook-coverage** — presumes an always-loaded/on-demand doc split
  this repo does not have; no surface, no gate.
- **todo-refs** — zero `TODO(task:)` markers exist on governed surfaces
  here, and `check-comment-tier` already owns the shape and flags bare
  FIXME/HACK as non-roster comments; a slug-liveness gate over an empty
  marker set is footprint without a guarantee. Revisit only if markers
  proliferate.
- **backlog-aging** — already shipped as drift-kit's `kpi-deferred-age`
  (advisory, per the guard-kit precedent); the scan list predated kit 7.
- **gate-runtime-budget** — already shipped as drift-kit's
  `kpi-gate-runtime` over the `.tmp` timings measurement; refine that KPI
  only if it proves blind in practice.
- **required-section-presence (spec half)** — the queue half is admitted
  above as `check-queue-sections`; for specs and other manifests, heading
  liveness is already delivered from the referencing side
  (`check-spec-pointer` § resolution, `check-md-refs` anchors) without a
  per-file mandatory-heading roster to maintain.
- **slug-component-collision** — no tooling here resolves slugs in a way a
  directory name can confuse (queue slug readers anchor on the bullet
  shape, not free grep); no demonstrated axis, no gate. Revisit on
  incident.
- **script-names** — the naming half is enforced by resolution mechanics
  (a misnamed gate fails registry lookup loudly, fail-closed); the
  citation-coverage half is delivered by the `spec:` pointer convention,
  retained by this iteration's spec-pointer ruling. A second bidirectional
  gate would be redundant footprint.

## Producers and consumers

Each admitted gate is produced as a `checks/` member of its owning kit and
consumed by name through `scripts/gates.list` and the generated pre-commit
hook; fixture pairs are consumed by the kit's `gate-tests` runner. New
knobs (`SPEC_KIT_MDREF_EXCLUDE`, `LIFECYCLE_KIT_SKILLS_DIR`,
`GATE_SDK_ROOT_ALLOWLIST`, `QUEUE_KIT_REQUIRED_SECTIONS`) are read by
their kit's config loader with the defaults above; each has exactly one
reader — its gate.

## Existing sections updated

- Each owning kit's SPEC.md gains the gate's contract section
  (spec-kit ×2, lifecycle-kit ×1, gate-sdk ×2, queue-kit ×1) and lists the
  new knob under its §Layout and configuration.
- queue-kit/SPEC.md §check-queue-hygiene — cross-reference: hygiene owns
  line shape, sections gate owns heading presence.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain
      (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
