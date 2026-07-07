# SPEC amendment: core-file-presence

The first born-in-Checkwright gate (not extracted from the source platform):
`check-core-files`, blocking silent deletion or untracking of a consumer
repo's required core workflow files. Without it, a lone `TASK-QUEUE.md`,
`SPEC.md`, `WORKFLOW-STATE.txt`, or projection HTML can be `rm`'d or
`git rm`'d and committed; downstream gates catch it only incidentally (when
a hard-reference happens to dangle) and only at the next stage that runs
them.

## What changes

- **New gate `gate-sdk/checks/check-core-files.sh`** (skeleton-derived, the
  four contracts, `good/`+`bad/` fixture pair). Invariant: every path listed
  in the core-files manifest exists in the worktree **and** is tracked
  (`git ls-files --error-unmatch`). Red on a missing or untracked listed
  path — this catches plain `rm`, `git rm`, and a listed-but-never-added
  path alike, with no `--diff-filter` timing window.
- **New manifest convention `scripts/core-files.list`** — registry-style,
  one repo-relative path per line, `#` comments (the `gates.list` shape).
  Path knob: `GATE_SDK_CORE_FILES_FILE` (default
  `$GATE_SDK_GATES_DIR/core-files.list`). The manifest is **optional
  consumer config** (the `check-graph`/`graph-vocab.sh` pattern): absent
  manifest ⇒ the gate passes with a note; an empty or comment-only manifest
  also passes. A present-but-unreadable manifest is red (fail-closed).
- **The intentional-removal valve is the manifest itself**: retiring a
  surface deliberately (e.g. a merged `SPEC-<feature>.md` — though
  amendments, being short-lived, should not be listed) means deleting the
  manifest line in the same commit that deletes the file. The valve is
  diff-visible — a reviewer sees the manifest edit — and needs no exemption
  tag, so the gate is never weakened to pass, only re-scoped in the open.
- **Tier: precommit** — added to the gate's `# graph:` manifest so
  `gen-pre-commit.sh` picks it up; it fires without any stage skill.
- **Registered in this repo's `scripts/gates.list`**, with a
  `scripts/core-files.list` covering this repo's own core surfaces (the
  queue, README, CLAUDE.md, `gates.list` itself, the `.workflow/`
  projections, each kit's SPEC.md) — exact list is a build-time choice.

## Producers and consumers

- **Producer:** the generated pre-commit hook (and `run-gates.sh` full
  battery) — reachable in every consumer that registers the gate name in
  its `gates.list`; the enabling config is the committed manifest file.
- **Consumer:** the committing operator/agent, via the standard gate
  output contract (violation lines + non-zero exit blocking the commit).
- **Manifest fields:** each path line's reader is the gate's existence +
  tracked check; `#` comment lines have no reader by design (registry
  convention). No other component reads the manifest.
- **First external consumer:** the source platform, which adopts the gate
  as its kit-adoption step 0; its cutover deletion sweep exercises the
  valve (manifest lines removed alongside deliberately retired files).

## Existing sections updated

At merge into gate-sdk/SPEC.md:
- the per-gate contract section gains a `### check-core-files` entry
  (invariant + calibration as above);
- the configuration-knob table gains `GATE_SDK_CORE_FILES_FILE`;
- the consumer-layout section mentions the optional `core-files.list`
  beside `graph-vocab.sh` as the second optional-consumer-config instance.
No existing flow changes — the gate is additive.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
