# SPEC amendment: retired-path-citation

Queue entries: `cited-script-path-liveness-inline` and `stale-identifier-after-retirement`, the
(C) slice of `citation-liveness-family-convergence`. They ride unit set `static-config-seam`, built
first, under **operator direction, 2026-09-13, lead-relayed**.

## The question, and the premise measured

`check-docs-cmd` resolves a `.sh` path only in invocation position inside a fence. The same path
in an inline code span is never read, so a port that deletes a script leaves every inline
mention green. `stale-identifier-after-retirement` is the same failure on a wider subject: a
backticked path of any extension, left behind when a capability moves to a new holder.

Both entries name the valve as the design question, not the scan. A fence is the gate's only
exemption today ("a hypothetical example goes outside a fence"). Widening to inline spans takes
that valve away. The replacement has to admit prose that names a deleted path on purpose, as
history.

**Measured at this stage** against the governed doc set this repo configures (83 docs). The
probe took every inline code span outside a fence, kept the path-shaped tokens (two or more
segments, a final segment with an extension), and resolved each against the tracked tree. Of
the 155 `.sh` tokens that do not resolve against the doc's directory or the repo root:

- **43 resolve under another kit root.** These are cross-kit citations. `lib/gate.sh` written
  in canon-kit/SPEC.md means gate-sdk's library. A reader resolves them by kit context, so the
  arm has to as well.
- **7 were never tracked.** They are metasyntactic or consumer-side names: `bin/x.sh`, a scratch
  `zz-parity-probe.sh`, a payload-relative `package/bin/checkwright.sh`.
- **105 name a path this repository's history deleted.** Almost all of them are deliberate
  port-record history ("`bin/kfric.sh` was the one owed surface"). About half sit in
  `docs/posts/`.

Widening to every extension adds no new class. The extra non-resolvers are gitignored capture
streams (`.workflow/*.log`), consumer-side files (`.claude/settings.local.json`) and
metasyntactic names, and none of them was ever tracked.

**So the red condition that fits the class is *retired*, not *absent*.** Under that condition,
with kit-root resolution and outside `docs/posts/`, the probe leaves **60 tokens on 56 lines in
seven kit SPECs** (gate-sdk/SPEC.md 33 of them; 59 `.sh`, 1 `.tsv`). Each is either a real stale
citation or history owing a valve.

Four dispositions were weighed. One is taken.

- **Refused: red on every non-resolving inline path.** A path that was never tracked is not a
  stale identifier. It is a hypothetical, a consumer-side file or a gitignored capture stream.
  That class outnumbers the retired class outside `.sh`, and every member would owe a valve that
  records nothing true.
- **Refused: admit a retired path whose citing line postdates its deletion** (line blame against
  the deletion commit). The rule admits every line the deleting commit touched. A port commit is
  exactly the commit that rewrites the paragraphs citing what it deleted. So the heuristic would
  launder the stale citations at the one moment they matter.
- **Refused: historical mentions cite the commit rather than the path.** A kit SPEC states its
  rules undated, and provenance belongs in git history (CLAUDE.md §The provenance seam). A
  convention that fills kit SPECs with commit hashes puts provenance on the surface that must
  not carry it.
- **Taken: a retired inline path reds unless history is sanctioned at that site, and the
  sanction is the one `check-manifest-temporal` already owns.** A mention of a retired path is
  temporal narration, and this kit already rules where narration may stand. It does so with
  three valves: a per-site marker, a section list and a path list (§check-manifest-temporal).
  Minting a second valve family for the same fact would let the two disagree about which line
  is history.

## The seam

- **Kit mechanism:** the inline arm, its resolution order, the retired-set derivation, and the
  reuse of `check-manifest-temporal`'s valves.
- **Consumer config:** the existing `CANON_KIT_TEMPORAL_EXEMPT_SECTIONS` and
  `CANON_KIT_TEMPORAL_EXEMPT_PATHS` values, and each per-site marker's reason text. No knob is
  minted.
- **This repo's own:** the valve sweep over its kit SPECs, the audit-roster line, and the
  `scripts/canon-config.sh` comment for the posts path valve.
- **Private rule content:** none in reach.

## What changes

### (1) check-docs-cmd gains an inline retired-path arm {design-bearing}

canon-kit/SPEC.md §check-docs-cmd. Its invariant sentence and the gate's `# spec:` one-liners
(descriptor and module header) cover three assertions. **Not yet applied:**

> Invariant: every invoked repo-relative `.sh` path in a fence, every backticked or fenced
> kit-prefixed env knob, and every inline-span path citation in the governed doc set resolves
> against the tree, or names no path the tree has retired. Three assertions:

A third bullet follows (B). **Not yet applied:**

> - **(C) retired cited paths.** Outside a fence, a path-shaped token inside an inline code span
>   reds when it names a path the repository has **retired** and still tracks under none of its
>   resolutions. A token is path-shaped when it has two or more `/`-separated segments of
>   `[A-Za-z0-9._-]`, a final segment carrying an extension, and no `..`. The same quote and
>   punctuation trims as (A) apply. Its **resolutions** are the doc's directory, the repo root,
>   and each `gate_kit_roots` member: a kit-relative citation such as `lib/gate.sh`, written in
>   one kit's SPEC about another kit's file, resolves the way its reader resolves it. A token
>   **resolves** when any resolution is a tracked file or a directory holding one. The
>   **retired set** is every path deleted in the history this clone holds (`git log
>   --no-renames --diff-filter=D`, so a rename counts as a deletion of its old name), plus every
>   path the index deletes against `HEAD`. The second half makes the deleting commit red at
>   pre-commit, before its deletion is in history. A token that resolves nowhere and names no
>   retired path is admitted: a hypothetical, a consumer-side file or a gitignored capture
>   stream is not a stale identifier.
>
>   **History is admitted where `check-manifest-temporal` admits it, through its three valves
>   and no fourth:** a `manifest-temporal-exempt: <reason>` marker on the line or the one above,
>   a section named in `CANON_KIT_TEMPORAL_EXEMPT_SECTIONS`, or a file matching
>   `CANON_KIT_TEMPORAL_EXEMPT_PATHS`. Naming a retired path is narration about the past, and
>   that gate already rules where narration may stand. A second valve family would let the two
>   gates disagree about which line is history. The exemption goes no further than that: a
>   valved line's fenced invocations and knobs are still scanned.
>
>   **(C) reds on retirement and (A) on absence, and the difference is deliberate.** An
>   invocation of a path that was never tracked is a broken command. A citation of one is
>   usually a hypothetical. **A shallow clone under-reds and never invents:** its retired set is
>   whatever history the clone holds, so the arm reports fewer findings and no false ones.
>   gate-sdk's workflow template already fetches full depth for history-reading gates, and the
>   clean line names the shallow case rather than passing silently.

The paragraph after the bullets changes in two places. **Not yet applied:**

- *"Prose outside fences and backticks is never scanned; a hypothetical example path is written
  unfenced, …"* becomes: prose outside fences and code spans is never scanned for paths. A
  hypothetical *invocation* is written outside a fence. A hypothetical *citation* needs nothing,
  because a path never tracked is not retired.
- The coupling sentence gains `knob:CANON_KIT_TEMPORAL_EXEMPT_SECTIONS` and
  `knob:CANON_KIT_TEMPORAL_EXEMPT_PATHS`. A change to either knob re-fires the gate, since
  either can move a (C) verdict.

canon-kit/SPEC.md §check-manifest-temporal gains one sentence, after its three-valve list.
**Not yet applied:**

> These three valves also admit a retired-path citation under §check-docs-cmd assertion C, so a
> site's marker or path exemption clears both gates at once and their reach is stated once, by
> the section that owns the valves.

### (2) The compiled member implements the arm {design-bearing}

`native/src/gates/docs_cmd.rs`:

- `scan` emits a third token kind for inline-span path tokens outside fences, next to the
  existing inline knob scan (B).
- The retired set is built once per run: one `git log --no-renames --diff-filter=D --name-only
  --format=` and one `git diff --cached --no-renames --diff-filter=D --name-only`, both
  repo-root-anchored and graded by exit code the way `defined_knobs` already grades `git grep`.
  An erroring spawn is exit 2. The shallow state is read with `git rev-parse
  --is-shallow-repository`.
- The valve is read through the crate's existing temporal-exemption reading, not re-implemented.
  `check-manifest-temporal`'s marker window, section tracking and path match are factored so both
  members call one implementation (native/src/gates/manifest_temporal.rs). A second copy of
  "which line is history" is exactly the drift delta 1 argues against.
- The member's declared knobs gain `CANON_KIT_TEMPORAL_EXEMPT_SECTIONS` and
  `CANON_KIT_TEMPORAL_EXEMPT_PATHS`, and the descriptor's `couples=` gains both `knob:` tokens.
- Output. A finding reads `<doc>:<line>: cited path '<token>' was retired (no tracked file under
  any resolution)`. The help block gains one line naming the three valves and the remedy:
  re-point the citation at the capability's new holder, or mark the line as history. The clean
  line adds the cited-path count, and ` (shallow clone: retirements limited to fetched history)`
  when it applies.
- Fixture pair `canon-kit/gate-tests/check-docs-cmd/`. The bad case adds a doc citing a path
  this repository's history retired, in an inline span on an unvalved line. The good case adds:
  the same citation under a `manifest-temporal-exempt:` marker, a never-tracked hypothetical, and
  a kit-relative citation resolving under another kit root. The staged-deletion half is covered
  by a unit test over the set derivation, not by a fixture, because a fixture case cannot stage
  a deletion in the host index.

### (3) The tree is swept to green: history valved, stale citations re-pointed {mechanical}

Run the widened gate over this tree and triage every finding. The probe predicts 60 tokens on 56
lines in canon-kit's sibling SPECs, spread across delegation-kit, drift-kit, evidence-kit,
lifecycle-kit, context-kit, guard-kit and gate-sdk; **the gate's own output is the worklist, not
this count.** Each finding takes one of two dispositions:

- **Re-point** it when the prose cites the path as live. Name the capability's current holder.
  This is the `stale-identifier-after-retirement` class itself, and finding one is the arm
  paying for itself.
- **Valve** it when the prose narrates the retirement: one `manifest-temporal-exempt: <reason>`
  marker on the line or the one above, where one marker can cover two adjacent lines.

`docs/posts/*` is already in `CANON_KIT_TEMPORAL_EXEMPT_PATHS` here, so the posts' 49 tokens need
no edit. The `scripts/canon-config.sh` comment above that knob (line 41) currently says the path
valve "exempts them from temporal-narration governance while link and command resolution still
apply". It becomes: exempts them from temporal-narration governance and retired-path citation
while link and fenced-command resolution still apply.

Regenerate the kit SPEC mirrors under `docs/` and the pre-commit hook, whose baked argv for this
member gains the two knobs. Each freshness gate prints its own command.

### (4) The audit roster names the class for what the arm cannot reach {mechanical}

`.workflow/audit-roster.txt` gains one line after `capability-liveness-after-descope`. **Not yet
applied:**

```
stale-identifier-after-retirement — a path cited as live after it was deleted or renamed while its capability moved to a new holder, where check-docs-cmd assertion C cannot see it: TASK-QUEUE.md live entries, prose outside the governed doc set, and a path written outside a code span; un-gateable there because telling a live citation from narrated history is a session act on surfaces no per-site valve may burden (a queue entry is budget-capped). Sweep: every path the iteration deleted or renamed (git log --no-renames --diff-filter=D over the range), grepped as full path and as kit-relative tail. — due: an iteration that deletes or renames a tracked path — last: never
```

The line is distinct from `capability-liveness-after-descope`, which is due only on a revert,
unport or descope. A retirement that moves a capability intact, such as the 2026-09-13 renamed
declaration surface, fires this line and not that one.

### (5) The coverage row in gate-sdk stops describing the fenced-only limit {mechanical}

gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate, the table row for
`check-docs-cmd`, `check-install-claim`, `check-payload-claim` and `check-queue-slug-liveness`.
Its sentences on `check-docs-cmd` currently say the invoked-path scan runs only inside a fence
and that an unfenced mention gives a porting session no red. They become, **not yet applied:**
the gate reds on a doc that still fences an invocation of a deleted `.sh` path, and on an inline
citation of any retired path outside a history valve (canon-kit/SPEC.md §check-docs-cmd
assertion C). A mention written outside every code span stays unscanned, and that residue is the
audit roster's. The attestation sentence ("roughly fifty unfenced doc mentions … zero reds") is
history about the pre-widening gate. It stays only as the reason the arm exists, in the past
tense, under a `manifest-temporal-exempt:` marker if it keeps a path.

### (6) The release declaration {mechanical}

`.workflow/release-declarations.md` §Tightened gates gains, **not yet applied:**

```
- `check-docs-cmd` — gains assertion C: a path cited in an inline code span in the governed doc set reds when the repository has retired it (deleted in reachable history, or deleted in the index) and it resolves under neither the doc's directory, the repo root nor any kit root, unless check-manifest-temporal's valves mark the site as history. A doc citing a script your tree deleted now reds until the citation is re-pointed or the line carries `<!-- manifest-temporal-exempt: <reason> -->`.
```

## Producers and consumers

- **The retired set** (new derived state, per run). *Producer:* the member's two git reads at
  run time. No config enables them; `check-docs-cmd` is registered in `scripts/gates.list` and is
  `tier=precommit`, so the hook and the battery both run it. *Consumer:* the (C) verdict, in
  process, per token.
- **The (C) finding line** (new output row). *Producer:* the member. *Consumer:* the committing
  session through the output contract, and `check-gate-output`'s shape assertions, which read
  every member's output lines and need no edit. The line keeps the `<file>:<line>:` lead.
- **The clean line's cited-path count and shallow clause** (new fields). *Reader:* the operator
  or session reading a green run, and the good fixture's `expect.txt`, which matches the
  `DOCS-CMD: clean` prefix alone. Probed: `git grep -n "DOCS-CMD"` outside `docs/` finds only
  that fixture and the member itself, so no parser reads the counts. The shallow clause exists
  so a shallow run is not silently weaker.
- **The two knob declarations** on the member. *Producer:* canon-kit/lib/spec.sh, which already
  defines both (so the config bridge's does-not-define refusal cannot fire). *Consumer:* the
  member's valve reading. This repo sets `CANON_KIT_TEMPORAL_EXEMPT_PATHS`
  (`scripts/canon-config.sh`) and leaves `CANON_KIT_TEMPORAL_EXEMPT_SECTIONS` at this repo's
  configured `Out of scope`.
- **The shared temporal-valve reading** (moved interface). *Producers:* none new. *Consumers:*
  `check-manifest-temporal` (unchanged verdict) and `check-docs-cmd` (C). The factoring holds
  only if `check-manifest-temporal`'s own fixture pair stays byte-identical in verdict, and
  `check-crate-arms` runs that.
- **The audit-roster line.** *Producer:* delta 4. *Consumer:* the close binding's audit-roster
  review step (`.claude/commands/close.md`), which reads every line, judges due-ness and stamps
  `last:`.
- **Red conditions (point 5).** This unit *widens* a corpus, so point 5 does not bind. The
  readers were enumerated anyway, because the reuse moves one member's valves into a second
  member's corpus. `check-manifest-temporal` reds on a marker hit outside a valve; the factoring
  changes no input it reads, so its verdict is monotone and unchanged. `check-docs-cmd` (A) and
  (B) read the same doc set as before. `check-surface-ratchet` holds always-loaded line ceilings,
  and delta 3's markers land in kit SPECs, which are not always-loaded. The one reader whose red
  moves with the delta-3 sweep is `check-manifest-count` or any gate asserting an exact count
  over a kit SPEC. Build confirms none of them counts HTML comment lines by running the battery,
  not by inspection.

## Existing sections updated

- canon-kit/SPEC.md §check-docs-cmd: the invariant sentence, assertion C, and the unscanned-prose
  and coupling sentences (delta 1).
- canon-kit/checks/check-docs-cmd.gate: the `# spec:` one-liner and the `couples=` knob tokens
  (deltas 1 and 2).
- canon-kit/SPEC.md §check-manifest-temporal: one sentence saying its three valves also admit a
  retired-path citation under §check-docs-cmd assertion C, so its valves' reach is stated by
  their owner (delta 1).
- native/src/gates/docs_cmd.rs and native/src/gates/manifest_temporal.rs: the arm and the
  factored valve reading (delta 2).
- canon-kit/gate-tests/check-docs-cmd/: the fixture pair (delta 2).
- scripts/canon-config.sh: the posts path-valve comment (delta 3).
- The kit SPECs carrying findings, and their generated mirrors under docs/ (delta 3).
- scripts/git-hooks/pre-commit, regenerated (delta 3).
- .workflow/audit-roster.txt: the new class line (delta 4).
- gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate: the reverse-trigger row
  (delta 5).
- .workflow/release-declarations.md §Tightened gates (delta 6).

## Retired spellings

- None — no delta renames or removes a name. The fenced-only calibration sentence is rewritten
  in place and carries no identifier.

## Definition of Done

- [ ] **Causal completeness** — the retired set, the (C) row and the shared valve reading each
      have a named producer and consumer, and every new clean-line field has a named reader.
- [ ] **Instruction surfaces: instruction only** — the gate's help text carries the remedy, not
      the grounds.
- [ ] **Merged with no information lost** — assertion C integrated into §check-docs-cmd; the
      refused dispositions relocate into that section's prose.
- [ ] **Amendment deleted** — this file removed on merge; `ls canon-kit/SPEC-*.md` empty.
- [ ] **Removals propagated** — none declared.
- [ ] **Gaps filed** — any finding delta 3 cannot re-point from the specs alone goes to
      `--emit file-gap`.
