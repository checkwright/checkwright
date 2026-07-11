# SPEC amendment: reads-couples

## What changes

- `checks/check-reads-couples.sh` — the meta-gate that mechanizes the
  `couples=` authoring duty (gate-sdk/SPEC.md §The `# graph:` manifest):
  for every registered gate, every **statically resolvable recursive walk**
  in its source must have its current read set covered by the gate's
  expanded `couples=` globs. Output token `READS-COUPLES`; registered in
  the consumer's `gates.list`.
- The tractable class, stated as the invariant: a walk is a `gate_find` or
  `find` invocation whose first directory argument resolves statically —
  a literal repo-relative path, a `"$KIT"/<sub>` form (the gate's own kit
  dir), or a `"$REPO_ROOT"/<sub>` form. For each resolved root, the gate
  enumerates the **tracked** files under it (`git ls-files`, filtered by a
  `-name '<pattern>'` primary when one is extractable from the same
  invocation, else unfiltered) and asserts every one matches at least one
  expanded couple glob, under the manifest's own glob semantics (no `/`
  crossing; `kit:` expansion through `gate_expand_couples`). A walk whose
  root does not resolve statically is *skipped and counted* in the clean
  line — deciding what arbitrary bash reads is undecidable, so the gate
  claims only the resolvable class and says how much it skipped.
- Scope limits, ruled here so the gate's honesty line can state them:
  only tracked files need coverage (couples exist to fire the hook on
  commits of tracked paths; a walk over `.tmp/` or generated `.workflow/`
  state has no commit to couple to), and only *walks* are analyzed —
  single-file reads and `git ls-files` enumeration are out of scope (the
  motivating bug class is recursion under a shallow couple, per
  check-shim-restatement's stage-template subdirectory miss).
- The false-positive valve: a deliberate uncoupled walk carries
  `# reads-couples-exempt: <reason>` in the gate's source (the
  `comment-tier-exempt` marker precedent — local to the gate it excuses,
  auditable in place). The budget ruling: over-demand is absorbed by
  either adding the covering couple (the correct fix) or the marker with
  cause; the gate never weakens its glob semantics to pass a near-miss.
- Manifest: `tier=precommit` with `trigger=*`. The unconditional trigger
  is load-bearing, not laziness: the invariant breaks two ways — a gate
  edit changes a walk, or **the tree grows a subdirectory under a walked
  root** — and no couple glob can name a directory that does not exist
  yet. Its `couples=` names what it reads as content: every checks dir
  (`kit:checks/*.sh`, the consumer gates dir) plus `gates.list`; the
  tracked-file enumeration is `git ls-files` metadata, not a content
  read, so it needs no couple (the same ruling the scope limit above
  makes for analyzed gates).
- Ships per the four contracts with a `good/`+`bad/` fixture pair: good —
  a sandbox gate whose recursive walk is fully covered (including a
  subdir-matching sibling glob); bad — the check-shim-restatement shape,
  a recursive walk with a shallow one-level couple and a tracked file one
  level deeper. Landing regenerates the hook (`gen-pre-commit --write`),
  the graph artifact, and the enforcement map.

## Producers and consumers

- Producer: the generated pre-commit hook (unconditional block, via
  `trigger=*`) and the full battery run the gate; no new enabling config —
  every registered gate already carries the `# graph:` line and source the
  analysis reads.
- Consumer: the committing session reads the verdict; a red names the
  gate, the walk root, the uncovered tracked path(s), and the covering
  fix shape (add the sibling glob or the exemption marker), so the repair
  is mechanical.
- The exemption marker's reader: this gate alone (self-lint ignores it;
  `check-comment-tier` sees a directive). The skip counter's reader: the
  clean line's parenthetical — it is the honesty label for the
  undecidable remainder, not machine-read state.

## Existing sections updated

- gate-sdk/SPEC.md §The `# graph:` manifest — the authoring-rule sentence
  ("this coverage is the author's duty") now cites the mechanization:
  the duty holds for the unresolvable remainder; the resolvable class is
  enforced by §check-reads-couples.
- gate-sdk/SPEC.md gains §check-reads-couples (invariant + calibration,
  the meta-gate family's section shape).

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
