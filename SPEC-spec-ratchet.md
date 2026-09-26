# SPEC amendment: spec-ratchet

Undated, marker-free narrative of how a rule was reached passes every prose gate, and no component SPEC is under a size ratchet, so between brevity passes a SPEC grows by accretion that no commit has to own. This amendment puts every component SPEC in this repo under context-kit's surface ratchet. A SPEC that grows then reds in the commit that grows it, until that commit re-stamps its ceiling, and the close-stage brevity pass keeps the judgment of whether the growth was worth it.

**The ruling: the ratchet ships; narration tells do not.** The entry offered both. The ratchet needs no calibration: it bounds size, not wording, so it has no false positive, and it moves the growth signal from close's walk to the growing commit. A tell set has to be calibrated, and the calibration below finds too little to gate.

**Refused: narration tells in `CANON_KIT_TEMPORAL_MARKERS_EXTRA`.** The candidates were run over the manifest corpus, and each one either catches rule text or catches one or two sentences. `turned out` also matches a rule about a survey that turned out wrong (lifecycle-kit/SPEC.md §The survey record). `had been` also matches a conditional (`whether an adopter's edit had been seen`). `it was followed` also matches a `feat` landing commit that "was followed by a later `chore`". The phrases with no false positive, `the hard way` and `the porting session`, each match one or two sentences, far fewer than the undated narration the brevity entries are cutting. The phrases that match at scale, `the port` and `landed`, are mostly contract vocabulary. A marker bans a phrasing for good, so a set this sparse would restrict how rules may be written and catch almost none of the narration. Narration stays the brevity pass's semantic residue (context-kit/SPEC.md §The close-stage brevity pass).

**Measured at authoring:**

- **The members.** `git ls-files -- '*/SPEC.md' ':(exclude)*/gate-tests/*' ':(exclude)docs/*/SPEC.md'` lists canon-kit, context-kit, delegation-kit, doctrine-kit, drift-kit, evidence-kit, gate-sdk, guard-kit, installer, lifecycle-kit, queue-kit and site-kit. `grep -c SPEC .workflow/surface-ceiling.txt` finds only `canon-kit/templates/SPEC-amendment.md`, so none of these has a row.
- **The knob's readers.** `grep -rn RATCHET_PATHS native/src` finds the ratchet's governed set (`native/src/emit/always_loaded.rs`, `governed`), the knob table and the reads-couples declaration, and nothing else. The generated pre-commit hook expands the knob's values into `check-surface-ratchet`'s trigger list, and `docs/check-graph.html` draws each value as a node.
- **The tells.** Hit counts over `git ls-files -- '*/SPEC.md' 'README.md' '*/README.md' 'CLAUDE.md' ':(exclude)*/gate-tests/*' ':(exclude)docs/*'`, case-folded: `the hard way` 1, `the porting session` 2, `turned out` 3, `had been` 3, `it was followed` 2, `the port` 319, `landed` 106.

## What changes

### (1) This repo governs its SPECs under the ratchet {mechanical}

**Not yet applied.** In `scripts/context-config.knobs`, after the file's last `CONTEXT_KIT_RATCHET_PATHS` line, add:

```
# comment-tier-exempt: every component SPEC, so its growth between brevity passes shows in the growing commit; the docs/ mirrors and gate-tests/ fixtures are excluded above
CONTEXT_KIT_RATCHET_PATHS[] = */SPEC.md
```

In the same commit, run `bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling` so `.workflow/surface-ceiling.txt` gains one row per member listed above, each at that SPEC's size at the landing commit. Where a sibling unit in the batch has grown a SPEC before this lands, the row is stamped at the grown size, so the growth is on record as re-stamped in this commit.

### (2) The ratchet's contract names the SPEC case {mechanical}

**Not yet applied.** In context-kit/SPEC.md §The surface ratchet, the **Why these files, per file** bullet's "A consumer may govern any authored surface whose growth it wants shown in the growing commit — a public page whose reader tier must not regrow is the second case." becomes:

> A consumer may govern any authored surface whose growth it wants shown in the growing commit: a public page whose reader tier must not regrow, or a SPEC that grows by accretion between brevity passes, whose growth the walk alone meets only at close.

### (3) The prose gate's honest limit names where narration goes {mechanical}

**Not yet applied.** In canon-kit/SPEC.md §check-prose-bounds, **Honest limits**, "Undated narrative of how a rule was reached passes all three assertions; the dated half is §check-provenance-seam's." becomes:

> Undated narrative of how a rule was reached passes all three assertions. The dated half is §check-provenance-seam's, and the rest is the close-stage brevity pass's, which a consumer's surface ratchet over its specs brings forward to the growing commit (context-kit/SPEC.md §The surface ratchet).

### (4) The generated projections follow the knob {mechanical}

**Not yet applied.** Regenerate the pre-commit hook, whose `check-surface-ratchet` trigger list gains `*/SPEC.md`, and `docs/check-graph.html`, and the SPEC mirrors under `docs/` for deltas 2 and 3, each with the command its freshness gate prints on red (docs/site-architecture.md §Generated projections and their freshness gates).

## Producers and consumers

- **The `*/SPEC.md` pathspec.**
  - Producer: this repo's `scripts/context-config.knobs`, which every battery run resolves.
  - Consumer: `check-surface-ratchet`, through `governed` in `native/src/emit/always_loaded.rs`, which lists the tracked matches with `git ls-files`. It reds a governed file above its row, or one with no row. The hook generator and `check-graph` read the knob's values as coupling.
- **The new ceiling rows.**
  - Producer: `--emit always-loaded --ceiling` in the growing commit, and `--update-baseline` at close (context-kit/SPEC.md §The surface ratchet, **Writers**).
  - Consumer: `check-surface-ratchet`. Each row's one field, the size, is the bound it tests against.
- **Obligation on later sessions.** A commit that grows a SPEC re-stamps the ceiling in that commit. The red prints the command, so no stage template changes. That includes an amendment merge in a build batch and a brevity pass that trades one section's cut for another's growth.

## Existing sections updated

Roster probe: `git grep -n "RATCHET_PATHS\|Undated narrative\|reader tier must not regrow"` over the tracked tree.

- `scripts/context-config.knobs` — the SPEC pathspec (delta 1).
- `.workflow/surface-ceiling.txt` — one row per member SPEC (delta 1).
- `context-kit/SPEC.md` — §The surface ratchet, **Why these files, per file** (delta 2).
- `canon-kit/SPEC.md` — §check-prose-bounds, **Honest limits** (delta 3).
- `scripts/git-hooks/pre-commit` — the ratchet's trigger list (delta 4).
- `docs/check-graph.html` — the graph's ratchet pathspec node (delta 4).
- `docs/context-kit/SPEC.md` and `docs/canon-kit/SPEC.md` — the generated mirrors (deltas 2, 3 and 4).

## Retired spellings

- None — no delta retires a name; delta 2 rewrites a sentence without removing a term.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; no root-level amendment of this unit remains (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The ratchet holds every member** — `bash gate-sdk/bin/run-gates.sh --only check-surface-ratchet` clean, and `.workflow/surface-ceiling.txt` carries a row for each file the members probe above lists; the entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
