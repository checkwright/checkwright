# SPEC amendment: brevity-reach

Queue entry: `always-loaded-bullets-restate-owned-spec`, leading unit set
`resident-tier-restatement` under **operator direction, 2026-09-13, lead-relayed**. Scope ruled
the seam the same day: the brevity gate's reach is kit mechanism; which sections are governed, and
their budget, stay consumer config in `scripts/context-config.sh`; no section name enters a kit
literal; no private rule content is in reach.

## The question, after scope's premise correction

The entry asked what a resident bullet owes beyond its pointer. Scope found the doctrine already
answers it — the *Always-loaded shape* rule: one line, the convention plus a pointer, the mechanism
behind it — and that what is unruled is **enforcement reach**. `check-brevity` reads one section
(`CONTEXT_KIT_BREVITY_SECTION`) and, inside it, only bullets opening `- **`. This repo points it at
the gate-sdk conventions block, whose bullets are one line each, so the gate has never had a finding
to issue here; every restating bullet the entry measured sits in `## Housekeeping`, where most
bullets do not even open in bold. A fifth restating bullet landed the day before this scope, which
is the class recurring on an ungated surface.

The entry's second ground is why a close could not take the cuts: a cut dropping a load-bearing
clause surfaces as a behaviour change in some later session, with no gate between. That is answered
here the only way it can be — **every dropped clause is traced to a named owner that states it,
before the cut is proposed** (delta 4), and a clause with no other owner stays resident or
relocates first.

Measured at authoring, over the two sections this repo will govern, with the widened grammar and
the stock budget of four: **eight bullets red** — `.tmp/` (6 lines), local-only files (7),
`native/` (8), `TRAJECTORY.md` (9), permission settings (5), knowledge-friction capture (5), gap
capture (7), survey capture (6). Every other bullet passes; the conventions block's four are one
line each.

## What changes

### (1) The governed section becomes a set {design-bearing}

`CONTEXT_KIT_BREVITY_SECTION` (a scalar) is retired for **`CONTEXT_KIT_BREVITY_SECTIONS`**, an
array of headings in the governed file, default `("## Shared conventions")` — the retired knob's
default as a one-element set, so a zero-config consumer's verdict is unchanged.

- **Every element must resolve.** A heading matching nothing in the governed file exits 2 and names
  that heading, exactly the single-section rule today (§The brevity gate): a renamed section must
  not disarm the gate while it reports the rest clean. Resolution, not emptiness, stays the
  fail-closed condition — a resolved section holding no bullets is clean.
- **An empty set exits 2.** A registered gate governing nothing is the vacuous pass the fail-closed
  contract refuses; a consumer that wants no brevity check unregisters the gate. This is the one
  place the set differs from `CONTEXT_KIT_RATCHET_PATHS`, whose empty default is meaningful ("no
  load-triggered surface"); an empty brevity set has no such reading.
- **A repeated element is scanned once**, so a duplicated heading in config cannot double a
  finding or the clean line's bullet count.
- **One budget for the set.** `CONTEXT_KIT_BREVITY_BUDGET` stays a scalar. A per-section budget was
  weighed and refused: no governed section here wants a different value, so the field would have
  no reader (causal-completeness point 4). A consumer that attests one reopens it.
- **The findings and the clean line name the section** each bullet was read in, since a finding
  line that names only the bullet no longer locates it once several sections are governed.
- **The retired knob refuses.** `lib/context.sh` exits 2 when `CONTEXT_KIT_BREVITY_SECTION` is set,
  naming its replacement. The refusal lives in the library for the reason
  `CONTEXT_KIT_SETTINGS_FILE`'s does (§Layout and configuration): once values cross the config
  bridge a compiled reader sees only declared knobs, and the retired one is no longer declared, so
  the library is the last place its being set is visible. Without it, a consumer config setting the
  old name is silently ignored, and a file that happens to carry `## Shared conventions` is governed
  on the wrong section at exit 0. The precedent is gate-sdk's retired `GATE_SDK_GRAPH_THEME`
  (gate-sdk/SPEC.md §check-graph), kept named so a consumer grepping the kit for the old knob lands
  on its replacement.

On the wire nothing new is needed: a scalar and a one-element array already cross the config bridge
as the same tab-joined value (gate-sdk/SPEC.md §lib/gate.sh), and the member reads the set through
the bridge's array read.

### (2) Every top-level bullet in a governed section is measured {design-bearing}

The item predicate widens from a bullet opening `- **` to **every top-level `- ` list item** of a
governed section. A bullet's extent, its span measure, the pointer conjunct and the exempt marker
are unchanged. A finding names the bullet by its line number and its lead — the bold run where the
bullet opens with one, else its lead line's opening text — because a `- ` bullet has no name to
print.

**This widens the corpus for every consumer**, so it can only add findings, never remove one: an
adopter whose governed section carries a non-bold, over-budget, pointer-carrying bullet reds on the
re-vendor that ships this delta. That is the intended catch rather than a regression, and it is
a **tightened gate**: the build appends `check-brevity` to `.workflow/tightened-gates.txt`
(gate-sdk/SPEC.md §upgrade-smoke), so the release note carries it. The retired knob's refusal
(delta 1) tightens the same gate on the config axis and rides the same line.

The predicate is the member's own argument to the shared section-walking primitive, which
`check-doctrine-registration` also calls with its own predicate; the primitive is untouched, so
that gate's walk cannot move (gate-sdk/SPEC.md §The first budget batch records the shared
primitive).

**Two honest limits, stated rather than discovered.**

- **The unit is the physical line.** A bullet joined onto one long line passes any budget. That
  unit is shared by the always-loaded meter and the surface ratchet, so the whole context apparatus
  measures the same quantity and a joined line lowers all three together; a width-aware span would
  diverge from both. Delta 4's replacement texts hold the file's prevailing wrap so the cuts do not
  buy their budget that way.
- **The pointer conjunct reads `§` by default**, so a bullet pointing at a document by path alone
  (`installer/`'s, at five lines) passes on it. `CONTEXT_KIT_BREVITY_POINTER_RE` is the consumer's
  lever; this repo does not widen it, because a path pattern also matches a bullet whose *subject*
  is a file, which is most of `## Housekeeping`.

Prose outside any bullet — the paragraph sections of an agent file — stays outside the gate's
grammar; that residue is filed as a costed gap at this spec rather than widened into here.

### (3) This repo governs the conventions block and Housekeeping {mechanical}

`scripts/context-config.sh` sets

```bash
CONTEXT_KIT_BREVITY_SECTIONS=("## Conventions established in gate-sdk (keep every kit consistent)" "## Housekeeping")
```

and removes its `CONTEXT_KIT_BREVITY_SECTION` line in the same commit, which delta 1's refusal
forces. The generated pre-commit hook bakes bridged knob values into its argv, so it is regenerated
in that commit; its freshness gate prints the command.

`## Delivery doctrine` is not governed: it is generated one line per rule and held byte-for-byte by
`check-doctrine-registration`, so a brevity finding there is unreachable. The three paragraph
sections carry no bullets.

### (4) The eight red bullets are cut to budget, every dropped clause traced to its owner {design-bearing}

Each replacement below is **Not yet applied**, holds at most four lines at the file's prevailing
wrap (no new line wider than the widest line the bullet already had), and keeps every command
byte-identical. A clause is dropped only where the named owner **states** it — checked by reading
the owner at authoring, not by the pointer resolving. Clauses with no other owner are kept, and the
one exception relocates first.

**`.tmp/` / `.metric/` / `.workflow/`** (6 → 4)

```markdown
- `.tmp/` is gitignored disposable scratch the scope boundary wipes; `.metric/` is
  gitignored persistent, account-bearing measurement, **never committed**;
  `.workflow/` holds tracked projections beside gitignored capture
  (gate-sdk/SPEC.md §The workflow directory).
```

- Dropped *"gate timings, resume journals, `<key>.run` liveness records"* — gate-sdk/SPEC.md §run-gates
  owns the timings file, delegation-kit/SPEC.md §Resume journal — agent writes, scratch reset
  sweeps the journals, evidence-kit/SPEC.md §The producer-liveness lock the record grammar.
- Dropped *"(keep-list: `scripts/lifecycle-config.sh`)"* — lifecycle-kit/SPEC.md §Layout and
  configuration, `LIFECYCLE_KIT_BOUNDARY_PRESERVE`.
- **Kept, sole owner:** `.metric/`'s never-committed rule and `.tmp/`'s disposability. No kit SPEC
  states either; `scripts/bash-guard.sh` and `.claude/commands/scope.md` cite this bullet for them.

**Local-only files** (7 → 4)

```markdown
- Local-only and gitignored: `BRIEF.local.md` (private brief), `ENV.local.md`
  (machine profile, context-kit/SPEC.md §bin/env-probe) and `OPS.local.md`. Consult
  `OPS.local.md` before any domain, repo-settings, release or push work, and run its
  account step before **any GitHub write** — per write, never per session.
```

- Dropped *"(DNS, repo settings, the release account and push transport)"* — the consult clause
  that follows names the same four scopes; the bullet stated them twice.
- Dropped *"plus gotchas"* — context-kit/SPEC.md §bin/env-probe owns the profile and its
  hand-authored gotchas seam.
- **Kept, sole owner:** the consult-before rule and the any-write account step.
  `RELEASING.md` states an account step scoped to release writes only, so the wider rule has no
  other home.
- **Relocated first:** the ground *"a write needing no permission succeeds silently under the wrong
  account"* has no tracked owner. It moves into `OPS.local.md`'s wrong-account paragraph, the local
  owner of the account step, in the same build — a local-only write, since that file is gitignored.

**`native/`** (8 → 4)

```markdown
- `native/` is the gate binary's Rust crate, **not a kit** (no `checks/`, no `smoke/`).
  The commit-time obligation is the battery **plus** `bash gate-sdk/bin/build-native.sh`;
  neither discharges the other, and an editor diagnostic discharges neither
  (gate-sdk/SPEC.md §Porting a gate to the binary substrate).
```

- Dropped *"one multi-call binary, a subcommand per gate plus the non-gate arms"* —
  gate-sdk/SPEC.md §The non-gate arm.
- Dropped *"which runs the crate's lint and test arms through `check-crate-arms`"* —
  gate-sdk/SPEC.md §check-crate-arms.
- Dropped *"the predicate that makes a root directory one"* — gate-sdk/SPEC.md §Meta-gate
  conservation for the binary substrate. **The predicate's two terms stay**, because the
  `installer/` bullet reads *"not a kit by the predicate under `native/` above"*.
- Dropped *"`check-gate-binary-fresh` holds the binary's currency"* — gate-sdk/SPEC.md
  §check-gate-binary-fresh, whose own text also carries *neither obligation discharges the other*.
- Dropped the pointer's label *"Dispatch, descriptors, port sequencing and the toolchain floor"*;
  the pointer stays.
- **Added:** *"and an editor diagnostic discharges neither"* — the resident clause
  `doctrine-kit/SPEC-ambient-diagnostic.md` delta 2 owns, carried here so the bullet is rewritten
  once.

**`TRAJECTORY.md`** (9 → 4)

```markdown
- [`TRAJECTORY.md`](TRAJECTORY.md) is the **override ledger**: a ruling is closed —
  escalate it to `/consult`, never reverse, annotate or re-verify it. **Only the
  operator rules, only through `/consult`**; what an operator says in a lead or stage
  session is a direction (lifecycle-kit/SPEC.md §The steering vocabulary).
```

- Dropped *"hand-authored"* and *"the objectives of the running pivot and the operator's rulings,
  each naming the instruction it overrides and its discharge, and leaving the file on discharge"* —
  lifecycle-kit/SPEC.md §The steering vocabulary defines objective and ruling in those terms, and
  `TRAJECTORY.md`'s own header states its discharge rule.
- Dropped *"revisable at scope or spec, and lands on the work surface it concerns as
  `operator direction, <date>`"* — the same section's definition of a direction, and
  queue-kit/SPEC.md states the stamp's spelling for queue entries.
- Dropped the label *"The terms (objective, ruling, direction, decision, grant):"*.
- **Kept byte-for-byte in meaning:** closed ruling, `/consult` escalation, the three refused acts,
  operator-only rulings through `/consult`, and that a lead or stage session's operator words are a
  direction. Nothing here widens or narrows who rules.

**Permission settings** (5 → 4)

```markdown
- **A permission-settings edit is applied on the operator's behalf, never by hand**
  (`operator direction, 2026-09-13`): a high-impact edit waits for explicit confirmation,
  a low-impact one is applied and reported, and a delegated session only prepares the
  diff (guard-kit/SPEC.md §compare-settings-allow).
```

- Dropped *"and hands it up"* and the label *"Impact classes and the refused laundering
  alternative:"* — guard-kit/SPEC.md §compare-settings-allow states both. The direction stamp stays:
  it is the provenance of a direction on the surface it concerns.

**Knowledge-friction capture** (5 → 4)

```markdown
- **Knowledge-friction capture (any session):** re-deriving a fact no doc owns (off an
  implementation, a gate's source, a commit, or a prior/sibling deliverable)? stamp it in
  the moment with `bash gate-sdk/bin/run-gates.sh --emit kfric "<fact>" "<surface>"` —
  deferred capture is no capture (drift-kit/SPEC.md §The knowledge-friction loop).
```

- Dropped *"close triages it"* — drift-kit/SPEC.md §The knowledge-friction loop, the close triage
  step; a capturing session does not act on it.
- **Kept deliberately:** the recognition examples, *any session* and *deferred capture is no
  capture*. The deferred entry `kfric-obligation-residency` records that this line is the capture
  obligation's only resident statement and that capture is already under-exercised, so cutting the
  trigger's recognition cues is the behaviour change the entry warns a close about.

**Gap capture** (7 → 4)

```markdown
- **Gap capture (any mid-iteration session):** a gap, task or defect goes to
  `bash gate-sdk/bin/run-gates.sh --emit file-gap "<gap>"`, never a queue edit
  (lifecycle-kit/SPEC.md §The committed gap inbox) — unless the operator directs a
  direct entry, staged and committed in one motion under the shared-index rule above.
```

- Dropped *"routes to the committed gap inbox"*, *"contending on a stage session's surface"* and
  *"close drains it"* — lifecycle-kit/SPEC.md §The committed gap inbox states the arm's destination,
  the contention ground and the close drain.
- Dropped the label *"The sanctioned exception:"*.
- **Kept, sole owner:** the operator-directed direct-entry exception. The kit section does not
  carry it; `.claude/commands/consult.md` and a queue entry cite this bullet for it.

**Survey capture** (6 → 4)

```markdown
- **Survey capture (any stage session):** read the survey record and run its witness
  before buying a survey; land one a later stage will want before acting on it, with
  `bash gate-sdk/bin/run-gates.sh --emit file-survey "<question>" "<corpus>" "<oracle>" "<edges>" "<finding>"`
  (lifecycle-kit/SPEC.md §The survey record).
```

- Reordered, and *"a census, a cohort sweep, a roster over a corpus"* dropped — lifecycle-kit/SPEC.md
  §The survey record defines the survey class. The command keeps its own line, as today.

Net: the file loses twenty-one lines. Every citation into `## Housekeeping` found at authoring
still lands on a kept clause: `scripts/bash-guard.sh` (`.tmp/` and `.metric/`),
`.claude/commands/scope.md` (the same), `.claude/commands/consult.md` (the direct-entry exception),
`scripts/canon-config.sh` (`reserve/`, not cut) and `scripts/core-files.list` (`installer/`, not
cut).

## Producers and consumers

- **`CONTEXT_KIT_BREVITY_SECTIONS`** (delta 1).
  - *Producer:* `lib/context.sh` defaults it; a consumer's `context-config.sh` overrides it; this
    repo sets it (delta 3). The config bridge carries it because the member declares it in its knob
    roster — the declaration swap is what makes the value reach the binary at all.
  - *Consumer:* `check-brevity`, through the bridge's array read; `smoke/violation.sh`, which
    inserts its violation under the first governed heading.
  - *Every element has a reader:* each heading is resolved and walked. No per-element budget field
    exists (delta 1 refused it for want of a reader).
- **The retired-knob refusal** (delta 1). *Producer:* `lib/context.sh`'s validation block.
  *Consumer:* any consumer or session whose config still sets the old name, at the next gate or
  tool run — exit 2 naming the replacement.
- **The widened item predicate and the section-bearing finding line** (delta 2). *Producer:*
  `check-brevity`. *Consumer:* the committing session through the output contract, on the generated
  hook, `run-gates.sh` and CI. The line number is read to open the bullet; the section is read to
  locate it; the lead is read to recognize it.
- **The cuts** (delta 4). *Producer:* the build. *Consumer:* every session, at load.
- **Red conditions** (causal-completeness point 5). Delta 4 **narrows** `CLAUDE.md`, so its readers
  are enumerated by what makes each red, not by subject:
  - `check-brevity` reds on an over-budget pointer bullet — delta 4 exists to take it from red to
    clean; exit 2 on an unmatched or empty section set.
  - `check-surface-ratchet` reds on a file **above** its ceiling; a shrink cannot red it, and
    close's `--update-baseline` lowers the row.
  - `check-doctrine-registration` reds on a missing link or a digest mismatch — the digest block is
    untouched.
  - `check-lifecycle-registration` reds on a stale marker block — untouched.
  - `check-footprint-fresh` reds on a stale `docs/footprint.md`; it measures only kit marker blocks
    in `CLAUDE.md`, which no delta touches.
  - `check-value-rollup-fresh` couples `CLAUDE.md`; whether its projection reads the file's size is
    the gate's to answer, so the build runs it rather than clearing it here.
  - `check-md-refs` and `check-spec-pointer` red on an unresolvable link or `§` — every kept pointer
    already resolves, and no new one is added.
  - `check-docs-cmd` reds on a documented command that does not resolve — every command is kept
    byte-identical.
  - `check-manifest-count` reds on a bare cardinal; `check-manifest-temporal` on a narration
    marker; `check-install-claim`, `check-payload-claim`, `check-tracking-claim`, `check-prose-enum`
    and `check-knob-citation` on claims or names in manifest prose. **These are the zero-count and
    exact-match shapes point 5 warns about**: a claim gate that reds on *finding none* of a
    declaration can flip green to red when the only instance leaves. The cuts remove examples and
    labels, not declarations, but the build runs each rather than inspecting — the monotone
    argument is exactly the one point 5 refuses.
  - Delta 2 **widens** `check-brevity`'s own corpus and cannot remove a finding; delta 1's empty-set
    refusal is a new red, reachable only by a consumer configuring an empty set.

## Existing sections updated

- `context-kit/SPEC.md` §The brevity gate (deltas 1 and 2) — rewritten for a section set, the
  widened predicate, the finding line, both honest limits and the empty-set refusal.
- `context-kit/SPEC.md` §The consumer footprint, the floor-holder paragraph (deltas 1 and 3) —
  *"`check-brevity` bounds the one bulleted section its knob designates … this repo points it at
  the conventions block, not the digest"* becomes the governed sections and this repo's two.
- `context-kit/SPEC.md` §Layout and configuration (delta 1) — the knob row, the layout tree's
  `check-brevity` comments, and the retired knob named beside its replacement.
- `context-kit/SPEC.md` §Testing (deltas 1 and 2) — `check-brevity.test.sh`'s held axes gain the
  unmatched element of a multi-section set, the empty set, the retired knob and a non-bold bullet.
- `context-kit/lib/context.sh` (delta 1) — the default and the refusal.
- `context-kit/checks/check-brevity.gate` (deltas 1 and 2) — its `# spec:` summary line.
- `context-kit/gate-tests/check-brevity/good/` and `bad/` (delta 2) — `bad/` gains a non-bold
  over-budget pointer bullet, `good/` a non-bold one within budget.
- `context-kit/gate-tests/check-brevity.test.sh` (deltas 1 and 2) — the cases above.
- `context-kit/smoke/violation.sh` (delta 1) — reads the set's first element.
- `context-kit/README.md` (delta 1) — its knob summary line.
- `native/src/gates/brevity.rs` (deltas 1 and 2) — the set, the predicate, the finding line.
- `native/src/gates/mod.rs` (delta 1) — the member's declared knob roster.
- `scripts/context-config.sh` (delta 3).
- `.workflow/tightened-gates.txt` (deltas 1 and 2) — `check-brevity` appended by the build.
- `scripts/git-hooks/pre-commit` (delta 3) — generated; regenerate, never hand-edit.
- `CLAUDE.md` §Housekeeping (delta 4) — the eight bullets. **Not yet applied.**
- `OPS.local.md` (delta 4) — the relocated ground; local-only, untracked, so no gate reads this
  bullet's discharge. **Not yet applied.**
- `docs/context-kit/SPEC.md` and `docs/context-kit/README.md` (all deltas) — the generated mirror;
  `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

**Deliberate non-updates.** `doctrine-kit/DOCTRINE.md` rule 5 already rules the shape and names this
gate as its enforcement; the rule's text stays. `context-kit/templates/close-brevity.md` step 3
still describes the gate truly. `docs/install.md` names only `CONTEXT_KIT_BREVITY_FILE`.

## Retired spellings

- `CONTEXT_KIT_BREVITY_SECTION` — the scalar section knob, replaced by the section set and refused
  when set (delta 1). The survivor scan matches substrings, so every site of the replacing
  `…_SECTIONS` spelling is a hit too; the roster above names every surface either spelling lands
  on, `context-kit/lib/context.sh` keeping the old name deliberately in its refusal.

## Definition of Done

- [ ] **Causal completeness** — the set, the refusal and the finding line each have a named
      producer and consumer; no per-section budget field exists.
- [ ] **The gate bites where it now reaches** — before delta 4 lands, a battery run with delta 3's
      config reds the eight bullets named above and no others; after it, `check-brevity` is clean.
      Read from a real run, not the diff.
- [ ] **No clause lost** — every bullet's dropped clause is found stated at the owner this amendment
      names, re-read at build; the `OPS.local.md` relocation lands before the Local-only cut.
- [ ] **No line joined to fit** — no rewritten bullet carries a line wider than that bullet's
      widest line before the cut.
- [ ] **Commands byte-identical** — `git diff` over `CLAUDE.md` touches no backticked command.
- [ ] **The ambient-diagnostic clause is present** in the `native/` bullet after both this unit and
      `doctrine-kit/SPEC-ambient-diagnostic.md` land.
- [ ] **Fixture coverage** — the pair and `check-brevity.test.sh` exercise the non-bold bullet, an
      unmatched set element, the empty set and the retired knob.
- [ ] **Full battery green** (`bash gate-sdk/bin/run-gates.sh`),
      `bash gate-sdk/bin/build-native.sh`, and the context-kit fixture suite.
- [ ] **Merged with no information lost** — each addition integrated into its canonical section.
- [ ] **Amendment deleted** — none remain for the component (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — the paragraph-section residue and the consumer-footprint roster's missing
      lifecycle-kit row are filed at spec; a gap found in build is resolved that session.
