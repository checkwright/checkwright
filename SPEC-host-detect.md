# SPEC amendment: host-detect

**Nothing in this amendment is applied.** Every passage below is a proposal for the build stage to
land; where replacement wording is given it is marked **Not yet applied** at the passage itself.
`spec` authors, build lands, and a reader arriving mid-iteration must not read a quoted sentence as
one already in the tree.

## The entry names three surfaces and there are four

The entry's first defect is that `check-install-platforms` binds `docs/install.md` against
`native/targets.list` and nothing else, leaving *"the third surface in that triangle —
`target_of_host` in `installer/bin/checkwright.sh`"* held by nothing. Surveyed at this stage across
the whole component set rather than the two files named:

**`installer/bin/checkwright.ps1`'s `Get-HostTarget` (`:39-61`) is a fourth surface and its own
`# spec:` comment calls it *"the twin of `installer/bin/checkwright.sh`'s `target_of_host()`"*.** It
maps the same five host shapes to the same five triples, `linux/x64` and `linux/arm64` included —
so **both** defects the entry files exist in **both** bootstraps, and the musl fail-open is two
fail-opens.

That is not a widening of the entry's envelope; it is the entry's own rule applied to the complete
surface set, which the causal-completeness contract requires: *survey those readers across the whole
component set, never a hand-picked subset.* A three-way binding that left the twin out would
reproduce the entry's own defect one surface over.

## The parity ruling this amendment works with rather than against

installer/README.md §The install boundary records a 2026-08-26 ruling: *"The two bootstraps are
hand-kept, and parity is held by running, not by generation."* Its oracle is a per-bootstrap
install-smoke leg. Nothing here generates either half and nothing here retires that oracle.

What this amendment records is that the running oracle is **structurally blind to this particular
field**. `install-smoke-powershell` runs on `windows-latest` and only there, so `Get-HostTarget`'s
`linux/x64` and `linux/arm64` arms have never been executed by any run this repository has bought,
and no run it could buy today would execute them. A static binding over the triple sets is therefore
**complementary** to the ruling rather than a substitute for it: it covers exactly the region the
running oracle cannot reach, and the region the running oracle does reach is left to it.

## What changes

### (1) The binding becomes four-way and bidirectional, and both directions have a defect behind them

`check-install-platforms` gains the two detectors as read surfaces {design-bearing}.

The existing arms hold `docs/install.md`'s declaration block and `native/targets.list` in lockstep
both ways. The new arm holds each detector's **emitted triple set** equal to the declaration
block's **declared triple set** — equality, not containment, and each direction closes a distinct
failure:

- **A triple a detector emits that the block does not declare** is the entry's own attested case: a
  triple *"DETECTED BY THE INSTALLER WHILE ON NO ROSTER with nothing going red."* The adopter's
  outcome is a clean refusal rather than a wrong artifact — `select_artifact:64` greps the payload
  roster, which is the narrowing the entry itself records — so this is a coverage hole rather than a
  fail-open, and it is held as one.
- **A triple the block declares that no detector emits** is a support claim the installer can never
  honour: an adopter on that host is refused while the front door says the platform is served. No
  instance exists today, which is exactly why the direction has to be asserted rather than assumed —
  a claim with no live violation is the kind that acquires one silently.

**This arm reds on the first run against HEAD, and that is by design.** The detectors emit
`aarch64-unknown-linux-gnu` and the block does not declare it. Clearing it is SPEC-arm64-linux.md's
delta 2, which declares that triple `held`. **The two units land in one commit, or that one lands
first.** Landing this arm alone reds the battery, and the constraint is recorded here so the build
session inherits it rather than discovering it.

**The clean line prints each detector's triple count**, on the vacuous-pass ground the roster's
other counted arms already stand on: a source scan whose extraction quietly stops matching reports
an empty set as agreement, and a number on the clean line is what makes that visible without an
audit.

### (2) The detectors' shape is pinned, because a gate cannot read a grammar no surface states

installer/README.md §The gate binary gains the two extraction shapes {design-bearing}.

The triples are owned by the `case` and `switch` arms themselves — de-literalization forbids a
second copy in a roster comment beside them, which would be the drift this whole entry is about
reintroduced one line lower. So the gate reads the owner, and the owner's shape is pinned so that
reading is not a guess:

- **bash.** Inside `target_of_host`'s body, every mapped triple is the **sole single-quoted operand
  of a `printf`** and appears nowhere else in the function.
- **PowerShell.** Inside `Get-HostTarget`'s body, every mapped triple is the **sole single-quoted
  operand of a `return`**, and the empty `return ''` is the no-mapping arm rather than a triple.
  The `switch -Regex` patterns are also single-quoted, which is why the pin is on `return` and not
  on quoting.

**Fail-closed (exit 2), spelled out because this arm's failure mode is silence:** the named function
not found in either file; the function found and **zero** triples extracted; either file unreadable.
Each is a check-that-could-not-run, never a pass. An extraction that silently degrades to nothing is
the one way a lockstep assertion reports agreement it never tested.

### (3) The musl fail-open closes in both halves, and it closes at the selection step rather than in the detector

The one case where the platform contract's own promise is broken gets its refusal {design-bearing}.

`uname -s`/`uname -m` — and .NET's `OSPlatform`/`OSArchitecture` — cannot distinguish glibc from
musl, so an Alpine x86_64 host resolves to `x86_64-unknown-linux-gnu`, which **is** on the roster,
so `select_artifact`'s refusal never fires and the host is handed a binary that dies in the dynamic
linker. `native/targets.list` promises *"Both refuse; neither proceeds"* and this is the only case
violating it.

**The libc question is answered where the other refusals live, not inside the detector**, and the
placement is load-bearing rather than stylistic. The detector answers *which published artifact fits
this host's OS and architecture*; the libc gate answers *is this host's C library the one that
artifact was linked against*. Two questions, two places. Folding the second into the detector's
`case` arms would also destroy delta 2's extraction shape, so the split keeps the gate readable and
the questions honest at the same time.

**Three outcomes, and the third is the one worth arguing.** The discriminator runs only on a Linux
host and yields `musl`, `gnu`, or `unknown`:

- **`musl` — refuse by name.** Positive signal: a musl dynamic loader is present (`/lib/ld-musl-*`).
- **`gnu` — proceed.** Positive signal: glibc identifies itself, through `getconf GNU_LIBC_VERSION`
  or `ldd --version`, either sufficing. Both are glibc's own components, so the signal is the
  library answering rather than a distribution being recognised.
- **`unknown` — refuse by name.** Neither positive signal. This is the fail-closed direction and it
  is the entry's own ask — *"an unrecognised libc refusing by name like every other unsupported
  case"*. The exchange is stated plainly: a working glibc host with no `getconf` and no `ldd` is
  refused, and receives a message naming exactly which probe failed; the alternative is that every
  unidentifiable host receives an artifact that dies with a dynamic-linker error and no explanation.
  A refusal an adopter can read beats an exec failure they cannot.

**An override knob was considered and is refused.** `INSTALLER_ASSUME_LIBC`, or any spelling of it,
would be a fail-open valve added in the same delta that closes a fail-open, and its only reader
would be an adopter guessing at the question the probe just failed to answer. If the `unknown`
population is ever measured as non-trivial, the fix is a better probe, and this paragraph is where a
later reader finds that trigger already stated rather than re-deriving it.

**Both halves take it.** pwsh runs on Linux, so `Get-HostTarget`'s Linux arms are reachable and the
PowerShell half carries the same three-outcome discriminator against the same signals. Hand-kept, on
the 2026-08-26 ruling's terms.

### (4) The refusal names what the host was detected as

`select_artifact`'s unsupported-host refusal, and its PowerShell twin, name the detected host
{mechanical}.

Today the message is *"this host maps to no target this payload declares"* with a help line about
the roster being fixed at pack time — true, and it never says what the host *was*. An adopter
reading it cannot tell an unsupported architecture from an unsupported libc from a `uname` that
answered nothing, and neither can a maintainer reading their bug report. The refusal gains the
detected `<os>/<arch>` and, where the libc gate fired, the libc verdict that fired it. A refusal
message is a documented surface — §Fail-closed contract's own rule for the wrapper class, applied
to the install boundary's refusals.

### (5) The gate's manifest, its fixture pair, and the projection fan-out that follows

`scripts/check-install-platforms.gate`'s `# graph:` manifest gains the two bootstraps in `couples=`
{mechanical}.

The fixture harness needs no new shape: `install_platforms.rs` already reads both surfaces through
`fresh::positional` with a layout default, so the two detectors are two further positionals with
defaults `installer/bin/checkwright.sh` and `installer/bin/checkwright.ps1`, and each of
`scripts/gate-tests/check-install-platforms/{good,bad}` gains two fixture files. The `bad/` case
exercises the arm that reds at HEAD — a detected triple the block does not declare — so the fixture
is the defect itself rather than a synthetic one.

**The fan-out a `couples=` edit stales**, read off docs/site-architecture.md §Generated projections
rather than discovered one red at a time: the generated pre-commit hook
(`bash gate-sdk/bin/gen-pre-commit.sh --write`) and `docs/check-graph.html`
(`bash gate-sdk/bin/run-gates.sh --emit graph > docs/check-graph.html`). `docs/enforcement.md` and
`docs/value.md` do **not** fire — no `tier=` moves and no gate joins or leaves the registry.

## The oracle that does not exist, stated rather than left to be assumed green

Delta 3's discriminator has **no running oracle in either half**, and pretending otherwise would be
the false claim this repo's fail-closed posture exists to prevent.

- The bash half's discriminator is reachable by the four bash install-smoke legs, but every one of
  them runs on a glibc host, so they exercise the `gnu` rung and nothing else. No leg this
  repository has or could cheaply buy runs on musl.
- The PowerShell half's Linux arms are unreachable by any leg at all, per the blindness recorded
  above.

What the amendment therefore asks for is the honest layer and no more: delta 1's arm asserts that
each detector's Linux mapping is **reached through** the libc gate — a structural source assertion
whose limit is exactly stated, that it asserts the guard is wired and never that the guard is right
— and the `gnu` rung is exercised by every existing green leg. Buying the rest costs a musl
container leg and a Linux pwsh leg, which is a platform-matrix unit and not this one. Named here so
a later session reads a priced gap rather than an omission.

## Why this entry does NOT carry `[observed-by:]`, which is a judgment and not an oversight

Three of this batch's four entries carry the tag. This one does not, and the discrimination is the
point of having a tag at all. Its completion predicate is a **tree state**: the widened gate goes
green against a corrected declaration block, and the two refusals exist with their fixture cases.
Nothing about its completion waits on reading a remote run. The one thing a run *could* tell it —
whether a musl host is refused — is the very observation the section above records as unbuyable, and
tagging an entry with a producer that does not exist would make the tag's `<producer>` field
unreadable for its one named reader.

## Producers and consumers

**One new interface — the libc verdict — and one widened existing interface, the gate's read set.**
No new file, no new state file, no new emitted event. Surveyed across the whole component set: both
bootstraps, the gate module, the gate descriptor, the install page, both roster files, and the
consumer smoke, with no stderr suppressed on any path grep.

- **The libc verdict** (delta 3). *Producer:* each bootstrap's discriminator, at the selection step,
  on a Linux host only. **Enabling config: none, and the absence is delta 3's refused-knob
  paragraph.** *Consumers, each named with its transition:* the selection step itself, at the
  proceed-or-refuse branch (reads `musl`/`gnu`/`unknown`); and the refusal composer of delta 4, at
  the refusal, which reads the verdict to name it. *Every field has a named reader:* the verdict is
  one field with two readers at two named transitions, and no second field is minted — the probe's
  raw output was considered as a field and **refused**, because its only reader would be a human and
  the verdict already carries what the message needs.
- **The gate's widened read set** (deltas 1, 2, 5). *Producer:* the two bootstraps, as tracked source
  files; their triple sets are produced by hand-editing a `case` or `switch` arm. *Consumer:*
  `check-install-platforms`, at every battery run and at every commit touching a coupled surface.
  *Enabling config:* the two new positionals' layout defaults, emitted by the gate's own descriptor
  — so the gate is armed in the default layout and takes no consumer action to become live.
- **Existing integration prose describing the prior flow** is updated in four places, listed below;
  the one that matters most is `native/targets.list`'s *"Both refuse; neither proceeds"*, which
  today describes a promise with a known exception and afterwards describes one without.

**Producer/consumer edges inside batch A.** One, and it is hard: delta 1's arm **reds against HEAD**
until SPEC-arm64-linux.md's delta 2 declares `aarch64-unknown-linux-gnu` as `held`. One commit, or
that unit first. No edge to SPEC-run.md or SPEC-interp.md, which touch the crate's spawn path.
**No edge crosses to batch B.**

## Existing sections updated

- `docs/site-architecture.md` §Generated projections and their freshness gates, the
  `check-install-platforms` row — the binding is four-way and bidirectional, and the two detectors
  join the surfaces named (deltas 1 and 2).
- `installer/README.md` §The gate binary, step 2 — the two detectors' extraction shape pinned, and
  the note that the PowerShell half *"reads the platform and architecture off the runtime rather
  than shelling out to uname"* extended with the libc question that neither input answers
  (deltas 2 and 3).
- `installer/README.md` §The install boundary, step 3 — the selection step's three outcomes become
  four inputs to three outcomes: the libc gate is a second way to reach the unsupported-host
  refusal, and the section states which refusal an adopter meets and why (deltas 3 and 4).
- `installer/README.md` §The install boundary, the 2026-08-26 hand-kept-parity paragraph — one
  sentence recording that the running oracle is blind to the PowerShell half's Linux arms, so a
  later reader does not read the new static arm as displacing the ruling (delta 1). **Not yet
  applied**; the ruling, its date and its attribution are untouched.
- `native/targets.list`'s header, the sentence *"Both refuse; neither proceeds"* — it describes a
  promise this amendment makes true, and the header should say the musl exception was closed rather
  than leave a reader to assume it never existed (delta 3).
- `scripts/check-install-platforms.gate`'s `# graph:` manifest and its `# spec:` invariant line —
  the invariant sentence is the gate's one-line contract and today names two surfaces (deltas 1
  and 5).

## Retired spellings

- None — no delta of this amendment retires a spelling. Every function keeps its name:
  `target_of_host`, `Get-HostTarget`, `select_artifact`, `Select-Artifact`, `die`, `Die`. Delta 4
  rewrites a refusal's **text** rather than a name, and a refusal message is prose with no
  cross-tree spelling for a sweep to chase; delta 3 adds a discriminator whose name is new and
  replaces nothing.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and
      a named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical
      surface (not appended); the merged surfaces read as one coherent document a reader who never
      saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root
      (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired
      spellings` above, and `check-amendment-retired-spelling` runs each declaration against the
      whole tracked tree.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
- [ ] **The fixture pair exercises the arm that reds** — `bad/` carries a detector emitting a triple
      the declaration block omits, which is the defect at HEAD rather than a synthetic one, and
      `good/` carries the four-way agreement.
- [ ] **The fail-closed arms are exercised, not asserted about** — a case with the named function
      absent and a case with the function present and no triple extracted each reach exit 2.
- [ ] **The fan-out is paid in the same commit** — the generated pre-commit hook and
      `docs/check-graph.html` regenerate after the `couples=` edit, staged first and regenerated
      second on the ordering docs/site-architecture.md's closing paragraphs own.
