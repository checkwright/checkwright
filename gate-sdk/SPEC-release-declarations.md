# SPEC amendment: release-declarations

Queue entries: `behavior-change-surface` and `deferred-release-declaration-accumulation`, paired.
They lead unit set `carried-record-reliability` under **operator direction, 2026-09-13,
lead-relayed**. Both ask where a declaration that must survive a deferral accumulates.

## The question, and why one surface answers both entries

A release note has three declaration-bearing sections: Tightened gates, Renamed knobs and
Behavior changes (docs/install.md §The upgrade contract). Only the first has an accumulating
surface, `.workflow/tightened-gates.txt`. Build writes it from knowledge, and the tag drains it.
The other two have none. RELEASING.md §The procedure step 2 says a deferral's outstanding criteria
"are carried into the next qualifying note", and nothing carries them. What has carried them so
far is the deferral line's basis prose in `.workflow/release-disposition.txt`, a file the next
iteration boundary truncates. `check-release-bump` reads that file's history, but only for the
deferred *version*. The criteria behind the version survive only in git history.

Measured at this stage: 55 distinct `deferred:v0.26.0` disposition lines were written after the
v0.25.0 tag. The latest carried a knob rename in its basis clause, and the boundary truncated it
at this iteration's scope. The next qualifying note would have to reconstruct Renamed knobs and
Behavior changes from 55 basis clauses and whatever a queue entry happened to hold.

`behavior-change-surface` left its design question open: a second `.workflow/` file, or a
widening of the existing one. Its two witnesses both land on *name plus prose versus bare
names*. One was a changed surface that is not a gate (a `bin/` tool's refusal), which the bare
gate-name grammar cannot hold even in principle. The other was a tightened gate whose one-line
consumer remedy had no route from build to the session composing the note.
`deferred-release-declaration-accumulation` names three alternatives:

- **Refused: a second accumulating file for the two prose sections.** That leaves two surfaces to
  drain at one tag, and the remedy witness stays unanswered, because a tightened gate's remedy is
  still a bare name on the first surface. The entry's objection, "two of them free prose no gate
  can hold to the note", is not what refuses it. Delta 3 shows a gate can hold them.
- **Refused: the deferral line's basis as the declared carrier, with a gate on it.** Close writes
  it, so it is written from reconstruction rather than knowledge, which is the property
  `behavior-change-surface` exists to restore. It also lives in a boundary-truncated file.
- **Taken: one surface carrying all three sections, in the release note's own grammar.** Build
  appends the bullet it would want in the note, remedy included. Close composes by transcription.
  One compiled holder parses the surface and the note, so the untagged and tagged smoke arms read
  the same grammar. The parity gate holds all three sections equal at the one comparable moment,
  and a single drain at the tag empties it.

## The seam

- **Kit mechanism:** the surface's grammar and drain contract (gate-sdk §upgrade-smoke), the
  holder's per-section token rules (`native/src/declaration.rs`), the smoke's untagged read, and
  the build template's declaration instruction.
- **Consumer config:** none new. The path derives from the existing `GATE_SDK_WORKFLOW_DIR`, and
  a consumer's release procedure stays behind the close template's `release-policy` slot.
- **This repo's own policy, not kit:** `RELEASING.md`, the note grammar in `docs/install.md`, and
  `check-release-declaration-parity`, a `scripts/` gate no consumer vendors.
- **Private rule content:** none in reach. The surface's bullets name public kit surfaces.

## What changes

### (1) The release declaration surface replaces the tightened-gates declaration surface {design-bearing}

`gate-sdk/SPEC.md` §upgrade-smoke, the paragraph opening **The tightened-gates declaration
surface**, is replaced. **Not yet applied:**

> **The release declaration surface**, `<workflow-dir>/release-declarations.md`, is a tracked
> checked projection (§The workflow directory). Its path derives from `GATE_SDK_WORKFLOW_DIR` and
> it has no knob of its own, since a knob naming this file would add a way to configure the
> assertion away without adding a way to satisfy it honestly. Its first line is the `# contract:`
> header. Below that, it carries up to three `## ` sections named exactly as the note's
> declaration-bearing sections (**Tightened gates**, **Renamed knobs**, **Behavior changes**).
> Each holds bullets in the grammar docs/install.md §The upgrade contract gives that section in a
> note. A section that is absent, or present with no bullet, is the empty set. The surface has no
> `None` body, so a drained surface is its header line alone and "absent" is never a state a
> reader must interpret. Order carries no meaning. A section is written in note order when it is
> first created.
>
> The surface holds **exactly what the note will declare, written in the note's grammar**, and
> that choice replaces the bare-name record this section used to rule. The old ruling refused a
> rationale column as "a field the smoke never reads and the note's bullet prose already owns".
> Two measured failures undid that ground. A red that a vendored tree meets on upgrade can need a
> one-line remedy only the landing session knows, and the bare-name record gave it no route to
> the composing session. And a changed surface that is not a gate had no line it could hold at
> all. Prose that the smoke does not read is still read, by the close that transcribes it into the
> note. The smoke reads only the Tightened-gates section's lead tokens, through the same holder
> the tagged arm uses.
>
> Only surfaces that ship **inside a kit** are declared, because those are the only ones a
> consumer's vendored tree meets. A gate, tool, template or knob living solely in the consumer's
> own tree cannot reach a vendored tree and belongs to no release's declaration. Being tracked is
> load-bearing: the smoke reads the file out of a `git archive` of TO, which carries tracked
> content only.

The paragraph opening **Its producer is the build stage** becomes, **Not yet applied:**

> Its **producer** is the session that lands the change, in the same commit. That is the build
> stage, for every section (lifecycle-kit/templates/stages/build.md), and any other session whose
> commit ships such a change, such as a close fixing a drained bullet inline, on the same
> obligation. The landing session is the only one that knows what it changed at the moment it
> changes it, so the declaration is written from knowledge rather than reconstructed from commits
> or from a red. A gate that lands **new** or gets stricter takes a Tightened-gates bullet, and an
> assertion that discovers its allowed-red set from the gate it was meant to check is its own
> trigger. A knob renamed or removed takes a Renamed-knobs bullet. Any other change to what the
> kits do takes a Behavior-changes bullet. Where a consumer must act, the bullet's prose carries
> the remedy.

The paragraph beginning **A gate whose *input* moved is appended on the same ground** keeps its
first three sentences. Its last sentence, which says this surface "taking bare names only, cannot
hold it", becomes: *"The bullet's prose carries the one-line remedy such a red needs."*

The paragraph opening **It accumulates** keeps its ground and widens its subject from Tightened
gates to all three declaration-bearing sections. Its closing sentences become: *"RELEASING.md §The
procedure composes the note's three declaration-bearing sections from it at step 1 and drains
it — truncating to the header, never clearing the file — at the tag in step 4. An iteration
closing on `release none` or a deferral carries every section forward, so a deferral's disposition
line carries the earned level and no criteria."*

The section's **declaration resolves on two arms** paragraph changes its untagged arm to *"reads
the Tightened-gates section of `<workflow-dir>/release-declarations.md` out of TO's tree"*. The
**Producers and consumers** closing paragraph is rewritten over the new surface. That rewrite
covers the producer clause above, the three readers (the smoke's resolve step, close's
compose-and-drain, `check-release-declaration-parity`), and the upgrade skill reading the note,
which is unchanged. **Not yet applied.**

### (2) The declaration holder reads one grammar on both surfaces, and the record arm retires {design-bearing}

`native/src/declaration.rs`:

- **`record_tokens` is deleted.** Its two callers, the smoke's untagged arm and the parity gate,
  move to the markdown arm. The holder's bounded public surface loses that entry point.
- **Per-section lead tokens.** `section_tokens` takes the section's token rule rather than
  assuming the gate-name predicate. *Tightened gates* keeps today's rule unchanged: a backticked,
  unbolded bare gate name directly after the marker. *Renamed knobs*: the backticked, unbolded
  span directly after the marker, which is the old name, with any non-empty content. A knob name
  carries underscores and a tag carries brackets, and the gate-name predicate refuses both.
  *Behavior changes*: the bolded span directly after the marker, verbatim and trimmed, with any
  non-empty content. The trichotomy and its refusal of a non-`None` section yielding no token hold
  for every section.
- **The surface verdict.** A reader of the declaration surface maps `Absent` and a heading with no
  bullets to the empty set, and treats `Unparsed` as a finding, exactly as it does on a note.
  `ExplicitNone` cannot arise on a conforming surface. A reader meeting one treats it as the empty
  set rather than refusing, since "None" means the same thing on both surfaces.

`native/src/emit/upgrade_smoke.rs`: the untagged arm reads `release-declarations.md` through
`section_tokens(text, "Tightened gates")` under the surface verdict above. A missing file at an
untagged TO keeps today's *no declaration anywhere* finding.

`gate-sdk/SPEC.md` §lib/declaration.sh is rewritten to match. **Not yet applied:** one container
arm over three section token rules; the record-arm bullet removed; the caller-relations paragraph
naming the parity gate's three-section comparison; and the public-surface count moved from three
entry points to two. Its heading stays, for the reason its first paragraph gives.

### (3) The parity gate holds all three sections and is renamed to what it now holds {design-bearing}

`check-tightened-gates-note-parity` becomes **`check-release-declaration-parity`**. It is this
repo's `scripts/` member, not kit-shipped, so the rename reaches no consumer's `gates.list`. The
rename is a **lead decision, 2026-09-13**: this repo's own gate, inside this amendment's envelope.
Its rule, **Not yet applied:**

- The corpus and arming are unchanged. It arms on exactly one note whose declared version carries
  no tag, is dormant on none, and refuses (exit 2) on more than one.
- The declaration-file argument defaults to `.workflow/release-declarations.md`. The hardcoded
  `.workflow/` default stays the open config-bridge question §The declaration cohort already
  records. This amendment inherits it and does not answer it.
- For each of the three sections, the note's token set and the surface's token set are compared
  as sets in both directions. **Red (exit 1)** on any difference, in any section, reported per
  section. The Tightened-gates grounds for both directions stand as written. For the other two
  sections, a token on the surface and missing from the note is a declared change the note
  dropped. A token in the note and missing from the surface is a change close reconstructed
  instead of the landing session declaring it: the composing session appends it to the surface in
  the composing commit, then transcribes.
- **Exit 2** on a note section that is `Absent` or `Unparsed`, on a surface section that is
  `Unparsed`, and on a surface whose first line is not a `#` header, as today.

Touched with it: `native/src/gates/tightened_gates_note_parity.rs` is renamed to
`release_declaration_parity.rs`; the `native/src/gates/mod.rs` registration; the descriptor
`scripts/check-tightened-gates-note-parity.gate` is renamed to
`scripts/check-release-declaration-parity.gate`, whose `couples=` names
`.workflow/release-declarations.md`; `scripts/gates.list`;
`scripts/gate-tests/check-tightened-gates-note-parity/` is renamed, with its `good/` pair
carrying all three sections and its `bad/` pair a mismatch in each; the pinned exit-2 harness
`scripts/gate-tests/check-tightened-gates-note-parity.test.sh`, renamed alongside; the generated
pre-commit hook, the enforcement map and the coupling graph, each regenerated by its own command;
and `.workflow/gate-timing-baseline.txt`'s row. `gate-sdk/SPEC.md` §The declaration cohort names
the member by its new name.

### (4) The note grammar pins the two prose sections' lead tokens {design-bearing}

`docs/install.md` §The upgrade contract, **Not yet applied:**

- **Renamed knobs** gains: *"The lead token is the old name, backticked and unbolded, directly
  after the bullet marker (``- `OLD_NAME` → `NEW_NAME` ``). The arrow, the new name and any prose
  follow it."*
- **Behavior changes** keeps its bolded lead and its statement that the Tightened-gates rule does
  not reach it, and gains: *"The lead token is the bolded span directly after the bullet marker,
  read verbatim, so a surface or phrase inside the bold is the token and whatever follows the
  closing `**` is prose."*
- The paragraph opening **A sibling gate makes a separate claim** names
  `check-release-declaration-parity`, and states that the parity holds all three sections, with
  the per-section grounds from delta 3.
- **A declaration precedes its release** retargets to *"a working tree's release declaration
  surface"*, and says the surface carries every section, so a behavior change or a rename is owed
  from the moment it lands, like a tightened gate.
- The **In brief** paragraph's sentence naming the predicate *"`check-tightened-gates-note-parity`'s"*
  names `check-release-declaration-parity`.
- The upgrade smoke paragraph's *"`TO`'s tightened-gates declaration"* reads *"`TO`'s Tightened-gates
  declaration"*, and its parenthetical names the release declaration surface.

These pins bind the **note under composition** only. No gate reads a published note's Renamed
knobs or Behavior changes tokens, since `check-tightened-gates-grammar` scans Tightened gates
alone. Historical notes spelled the leads several ways and are not retro-fitted.

`docs/install.md` §Versioning's second-input paragraph, the sentence *"The next qualifying note
carries them in its declaration-bearing sections"*, gains: *"— composed from the release
declaration surface, which carried them across every deferral."* **Not yet applied.**

### (5) The release runbook composes three sections from one surface {mechanical}

`RELEASING.md` §The procedure, **Not yet applied:**

- Step 1, the three-variable-sections bullet. **"Tightened gates is composed, not recalled"**
  becomes *"All three are composed, not recalled"*. The bullets come from
  `.workflow/release-declarations.md`, one note bullet per surface bullet, and each section's lead
  tokens are carried unchanged. The prose may be edited for the reader. An empty section means a
  stated "None." A change the composing session finds undeclared is appended to the surface
  first. The **Held by a gate** sentence names `check-release-declaration-parity` over all three
  sections.
- Step 2: *"the outstanding criteria are carried into the next qualifying note's three sections"*
  becomes *"the outstanding criteria stay on the release declaration surface, which the next
  qualifying note is composed from"*. The deferral line's basis states the level and why the
  release was held. It carries no criteria.
- Step 4: the heading's *"drain the tightened-gates declaration surface"*, the parity-gate
  sentences, and *"The tag is also what discharges `.workflow/tightened-gates.txt`"* retarget to
  the release declaration surface and the renamed gate. The drain is still a truncation to the
  header line.

### (6) The lifecycle surfaces follow {mechanical}

- `lifecycle-kit/templates/stages/build.md`, the paragraph **Declare a gate you land or tighten**,
  becomes, **Not yet applied:**

  > **Declare what a vendoring consumer will meet, in the unit that lands it.** A unit that lands
  > or tightens a gate, renames or removes a knob, or changes what a kit script, template or
  > default does appends one bullet to the matching section of the release declaration surface,
  > `<workflow-dir>/release-declarations.md` (gate-sdk/SPEC.md §upgrade-smoke), in the same commit,
  > written as the release note's bullet: the lead token that section takes, then what moved and
  > what the consumer must do. The surface accumulates across the iterations batched into one
  > release; the release step composes the note from it and drains it at the tag, so nothing here
  > loads the release runbook. Only what ships inside a kit is declared.

- `lifecycle-kit/SPEC.md` §templates/stages/, the release-disposition paragraph's sentences *"The
  criteria themselves stay in the basis prose and are not structured fields: the release note's
  upgrade-contract sections already own them … the line carries the *level*, the note owns the
  *criteria*"* become: *"The criteria are not fields on the line. Until a release ships they
  accumulate on the release declaration surface (gate-sdk/SPEC.md §upgrade-smoke), and the note
  composed from it owns them after. The line carries the *level*."* **Not yet applied.**
- `lifecycle-kit/SPEC.md` §Layout and configuration, the knob-rename compat precedent's
  *"a rename owes a tightened-gates/release-note declaration"* becomes *"a rename owes a Renamed
  knobs declaration on the release declaration surface, and so in the note"*. **Not yet applied.**
- `.claude/commands/close.md` **release-policy**: the sentence *"let the accumulated declarations
  ride the next qualifying release. RELEASING.md step 1 already composes a batched note from every
  declaration accumulated since the last tag"* was false for two sections. It becomes true as
  written once delta 1 lands, so no edit is owed. It is listed under Existing sections updated as
  a reviewed target.

### (7) The workflow directory's rules name the new member {mechanical}

`gate-sdk/SPEC.md` §The workflow directory, **Not yet applied:**

- The draining-member sentence names `release-declarations.md` at the tag in place of
  `tightened-gates.txt`.
- The extension rule's `.md` bullet becomes *"a **prose surface a human reads and dispositions**,
  machine-read only for emptiness, a bullet count, or a bullet's lead token under a named
  section"*. The release declaration surface is transcribed by a human into a note and read by
  the holder for lead tokens. A `.txt` line grammar would give it a second grammar beside the
  note's. The audit roster's workflow-surface-extension review, due at this addition, is
  discharged by this bullet.

### (8) The surface is seeded from what the deferrals carried, and the old file retires in the same commit {design-bearing}

The one commit that lands deltas 1 to 3 also creates `.workflow/release-declarations.md` and
deletes `.workflow/tightened-gates.txt`. Neither gate can see a consistent tree otherwise. The
seed:

- **Tightened gates**: one bullet per name in `.workflow/tightened-gates.txt` at that commit, each
  a bare backticked token. The intent prose is added where the landing commit's body states it,
  and is otherwise left to composition, as it is today.
- **Renamed knobs** and **Behavior changes**: reconstructed once, from two sources. First, every
  disposition line added to `.workflow/release-disposition.txt` since the newest release tag
  (`git log -p` over that file from the tag). Each line's basis clause is read for renames and
  behavior changes. Second, the queue entry `deferred-release-declaration-accumulation`'s carried
  bullets, each checked against every published note since its filing. A bullet that no published
  note declares is carried. Every reconstructed bullet names the iteration it came from in its
  prose, so the composing session can tell a seed from a declaration.
- **Sibling declarations**: any bullet an amendment in this unit set obliges on the surface,
  if its change landed before this commit. `lifecycle-kit/SPEC-claim-standing.md` delta 7 is one
  such bullet. A sibling landing after this commit appends its own.

This delta is the last reconstruction the surface pays for. The commit message states the count
per section and the iterations read. Where a basis clause is too thin to reconstruct from, the
message says so rather than guessing, and the gap goes to the inbox.

The retired file's history mentions in the published `docs/posts/` notes stay as history.

### (9) Precedent mentions follow {mechanical}

- `.workflow/audit-roster.txt` names `tightened-gates.txt` as a precedent member of the record
  tier. Retarget or drop that mention in the same commit as delta 8.
- `canon-kit/SPEC.md` §check-amendment-update-target's historical incident ("dropped a
  tightened-gates declaration across all three of its build batches") names the declaration class
  rather than the file, and is left as history.

## Producers and consumers

- **`<workflow-dir>/release-declarations.md` (new surface).** *Producer:* the session landing a
  kit-shipped change, appending in the landing commit. That is build under its template (delta 6),
  and after the seed of delta 8, any session shipping such a change under the §upgrade-smoke
  producer clause. The only config it needs is `GATE_SDK_WORKFLOW_DIR`, which defaults to this
  repo's layout. *Consumers:* (a) `--upgrade-smoke`'s untagged arm, at its resolve step, which
  reads the Tightened-gates lead tokens as the allowed-red set; (b) close's release step
  (RELEASING.md step 1), which composes all three note sections by transcription, and step 4,
  which drains it; (c) `check-release-declaration-parity` at pre-commit, during the composition
  window.
- **Each section's bullets.** *Lead token:* read by the parity gate in every section, and by the
  smoke in Tightened gates only. *Prose:* read by the composing close, which transcribes it into
  the note. No field on the surface lacks a reader.
- **The Renamed knobs and Behavior changes token rules (new holder interface).** *Producer:*
  `native/src/declaration.rs`. *Consumer:* the parity gate. `check-release-bump` keeps its
  container-only bullet count and does not take the token rules. `check-tightened-gates-grammar`
  keeps the Tightened-gates rule over published notes.
- **Red conditions (point 5).** Delta 2 changes the smoke's untagged-arm corpus from a record file
  to a section of a markdown file, carrying the same token set after delta 8's seed. The smoke's
  finding is *red set not a subset of the declared set*. That verdict is monotone in the declared
  set, so it holds only if the seed carries every current name, which delta 8 requires. The
  no-declaration finding reds when a phase-B red set meets a missing file. The seed creates the
  file, and the drain truncates to the header rather than deleting it. The parity gate's red condition is *set difference
  in either direction*, which is not monotone. Its armed window is only the composition window,
  and no note is under composition at this commit (every published note is tagged), so the gate
  is dormant when delta 3 lands. It first arms at the next release, against a surface seeded by
  delta 8. `check-workflow-tiering` reds on a tracked member without a ruled header, so the seed
  carries the `# contract: gate-sdk/SPEC.md §upgrade-smoke — …` header on its first line.
  `check-release-bump` is unaffected, since it reads notes and the disposition file only.

## Existing sections updated

- `gate-sdk/SPEC.md` — §upgrade-smoke's surface, producer, input-moved, accumulates, two-arm
  resolve and producers-and-consumers paragraphs (delta 1); §lib/declaration.sh (delta 2); §The
  declaration cohort's member name (delta 3); §The workflow directory's draining member and
  extension rule (delta 7).
- `native/src/declaration.rs` — the record arm deleted and the per-section token rules added
  (delta 2).
- `native/src/emit/upgrade_smoke.rs` — the untagged arm (delta 2).
- `native/src/gates/tightened_gates_note_parity.rs` — renamed and widened (delta 3).
- `native/src/gates/mod.rs` — the registration (delta 3).
- `scripts/check-tightened-gates-note-parity.gate` — renamed, `couples=` retargeted (delta 3).
- `scripts/gates.list` — the registered name (delta 3).
- `scripts/gate-tests/check-tightened-gates-note-parity.test.sh` — renamed with the gate (delta 3).
- `scripts/git-hooks/pre-commit` — regenerated (delta 3).
- `docs/enforcement.md` — regenerated (delta 3).
- `docs/check-graph.html` — regenerated (delta 3).
- `.workflow/gate-timing-baseline.txt` — the renamed row (delta 3).
- `docs/install.md` — §The upgrade contract's lead-token pins, parity, In brief, smoke and
  precedes-its-release sentences, and §Versioning's second-input sentence (delta 4).
- `RELEASING.md` — §The procedure steps 1, 2 and 4 (delta 5).
- `lifecycle-kit/templates/stages/build.md` — the declaration paragraph (delta 6).
- `lifecycle-kit/SPEC.md` — §templates/stages/'s release-disposition criteria sentences and §Layout
  and configuration's knob-rename declaration sentence (delta 6).
- `.claude/commands/close.md` — reviewed, no edit owed (delta 6).
- `.workflow/tightened-gates.txt` — deleted, its names seeded (delta 8).
- `.workflow/release-declarations.md` — created and seeded (delta 8).
- `.workflow/audit-roster.txt` — the precedent mention (delta 9).
- `canon-kit/SPEC.md` — the historical incident, reviewed and left (delta 9).
<!-- update-target-exempt: a published release note is history; it names the file as it was when that release shipped -->
- `docs/posts/2026-07-31-checkwright-v0-19-0.md` — names the retired file in its own release's prose.
<!-- update-target-exempt: generated mirror, regenerated by its own freshness gate when the kit SPEC it mirrors changes -->
- `docs/gate-sdk/SPEC.md` — the site mirror of gate-sdk's SPEC.
<!-- update-target-exempt: generated mirror, regenerated by its own freshness gate when the kit SPEC it mirrors changes -->
- `docs/lifecycle-kit/SPEC.md` — the site mirror of lifecycle-kit's SPEC.

## Retired spellings

- `tightened-gates.txt` — the declaration surface's file name, retired for
  `release-declarations.md` (deltas 1, 7 and 8).
- `check-tightened-gates-note-parity` — the parity gate's name, retired for
  `check-release-declaration-parity` (delta 3).
- `record_tokens` — the declaration holder's record arm, deleted (delta 2).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them
      (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls <component>/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in
      `## Retired spellings` above, and `check-amendment-retired-spelling` runs
      each declaration against the whole tracked tree, not against the specs
      alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Self-declared** — this unit's own consumer-facing changes are on the surface it lands:
      the build template's new instruction and the surface rename, under Behavior changes. The
      parity gate is this repo's own and is not declared.
- [ ] **Smoke oracle** — `bash gate-sdk/bin/run-gates.sh --upgrade-smoke` passes at an untagged
      TO after delta 8, reading the seeded surface.
