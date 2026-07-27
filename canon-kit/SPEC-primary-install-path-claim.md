# SPEC amendment: primary-install-path-claim

Give the primary-install-path claim a machine-readable owner, then hold every
other governed prose surface to it. canon-kit's star topology — one owner per
fact, cite rather than restate — applied to a user-facing claim instead of an
internal one.

## The honest split, and what is deliberately not built

The general form of this class — *does this documented install command actually
resolve against the live registry?* — needs network egress at gate time and is
out of a hermetic battery's reach. It is not attempted, and the SPEC section
this amendment adds says so rather than leaving a reader to infer that the gate
covers more than it does. The command that broke was syntactically fine; only a
registry could have contradicted it.

What **is** buildable is the consistency half: two surfaces cannot name
different primary transports. That is decidable from the tree alone.

The gate lands **greenfield**. `docs/install.md` lines 26-33 already state the
tarball is primary, and `README.md`'s Quick start already leads with it — the
close that filed this entry fixed the live drift inline. So this pins a
currently-consistent claim rather than repairing one, and its value is entirely
recurrence: the claim drifted at two consecutive releases with nothing watching
it, and the surface it drifted on is the first thing a new reader runs.

**Both halves confirmed at align**, by walking every section the configured
regex selects rather than trusting the two cited lines. No section in the
scanned set leads with a non-primary transport, so the gate arrives green. And
the recurrence path is *in scope*, which is the half that makes the greenfield
verdict worth having: the drift that actually happened was `README.md` §Quick
start leading with `npx checkwright init`, and `## Quick start` is a
`##`-or-deeper section matching the configured regex. A gate that arrives green
on a path it could not have caught would be recurrence value in name only; this
one covers the path that fired.

## Seam ruling

Kit mechanism: the `install-primary:` declaration's spelling and grammar, the
singleton rule, the section-scoped leading-transport predicate, and the report.
Consumer config: the **transport vocabulary** — every transport id and the
pattern that recognizes it — plus the install-section heading regex and the path
valve. A kit literal spelling `npx` or `tarball` would publish one project's
distribution model as a kit fact, the leak the `check-graph` / `graph-vocab.sh`
pattern exists to avoid; the kit ships the emit grammar and nothing else.
Private rule content: none.

## What changes

### 1. The `<!-- install-primary: <transport-id> -->` declaration — *design-bearing*

A full-line HTML comment naming the transport that owns the primary-path claim.
It is a marker rather than a visible sentence because the reader-facing form of
this claim already exists as prose and must stay prose; the marker is the
machine-readable tier beside it, not a replacement for it. `docs/install.md`
already carries `<!-- toolchain:begin -->` markers under
`check-install-toolchain`, so the form is in-pattern for that file.

It lands in `docs/install.md` §Quick start — the page the release note and
`README.md` both already cite as the install authority, so the claim's owner and
its citations do not have to be re-argued.

The marker and the sentence at `docs/install.md`:28-29 ("The tarball is the
primary path because it removes a runtime dependency") are one claim in two
tiers. The SPEC section states that binding, so a future edit to the sentence
that forgets the marker reads as the contradiction it is.

### 2. Three config knobs — *mechanical*

On the `CANON_KIT_` shape (canon-kit/SPEC.md §Layout and configuration):

- `CANON_KIT_INSTALL_TRANSPORTS_CMD` — a consumer command emitting one
  `<transport-id>⇥<ERE>` line per transport, loaded through `lib/spec.sh` on the
  `CANON_KIT_ENUM_SETS_CMD` model: a command that fails or a line that does not
  parse is fail-closed (exit 2). Default empty ⇒ the gate is a clean skip.
- `CANON_KIT_INSTALL_SECTION_RE` — an ERE matched against `##`-or-deeper heading
  text to select the sections scanned. Default empty ⇒ clean skip.
- `CANON_KIT_INSTALL_CLAIM_EXCLUDE` — array of path globs dropped from the
  scanned set. Default empty.

### 3. `scripts/install-transports.sh` — *design-bearing*

This repo's consumer config: two ids, `tarball` and `npm`, each with the ERE
that recognizes it in prose. Design-bearing because calibrating the patterns
against the live tree *is* the false-positive work — each pattern must match
both the recipe form (`releases/download/...`, `npx checkwright`) and the prose
form (`release tarball`) without either pattern reaching into the other's
sentences. The patterns are checkwright-specific by construction, which is what
keeps `RELEASING.md`'s generic `npm install lodash` example silent.

An honest limit belongs in the SPEC section with the mechanism: the gate is only
as sharp as the consumer's patterns, and a loosely-written pattern wins matches
by accident. The kit contract asks for the emit grammar; pattern quality is the
consumer's own drift to own, exactly as `check-prose-enum` says of a
hand-listed set.

### 4. `check-install-claim` — *design-bearing*

A new canon-kit gate, `precommit` tier, over `check-md-refs`' governed doc set
(the manifest set minus `CANON_KIT_MDREF_EXCLUDE`) minus
`CANON_KIT_INSTALL_CLAIM_EXCLUDE`. Two assertions:

- **(A) Singleton owner.** Exactly one `install-primary:` declaration exists
  across the scanned set, and its id is a member of the configured transport
  set. Zero declarations is the defect this entry was filed for — nothing owning
  the claim — and two is the same defect wearing a different shape. An id
  outside the transport set is fail-closed.
- **(B) Leading transport.** Within each scanned document, for each `##`-or-deeper
  section whose heading matches `CANON_KIT_INSTALL_SECTION_RE`, the **earliest**
  line matching any transport pattern must match the declared primary. Later
  matches are never flagged. A section matching no transport pattern is silent.

Assertion B *is* the answer to the predicate question the queue entry left open
— how to distinguish leading from mentioning. Naming npm as a secondary path is
correct and must stay green, so the rule is positional, and the scope in which
"leading" is well-defined is the install section, not the file.

**Why the file is the wrong scope, verified rather than assumed:**
`docs/install.md`:18 names `npx checkwright init` in the page preamble, 120
lines above the tarball recipe, while framing npm as an installer rather than a
dependency channel. A whole-file first-match rule would red the owner page
itself on correct prose. Scoping to `##`-or-deeper sections leaves that preamble
under the H1 alone and out of scope, which is the calibration the assertion
turns on.

If one line matches two transport patterns, the section passes when any matching
id is the primary — a sentence naming both transports is not leading with the
secondary one.

**The third honest limit, which the section scope buys and the SPEC must
state.** The prose tier of the declared claim — the sentence at
`docs/install.md`:28-29 — sits in that same H1 preamble, ~104 lines above the
`## Quick start` heading delta 1 puts the marker under. So the one sentence the
marker is the machine tier *of* is itself outside assertion B's reach: rewriting
the preamble to call npm primary while leaving the marker on `tarball` stays
green. This is the price of the scoping, not an argument against it — the
alternative whole-file rule reds the owner page on correct prose today. The
binding delta 1 describes is therefore documentary, held by this SPEC section
and by a reader, not by the gate; saying so is what keeps a future reader from
over-trusting a green run. The recurrence path that actually fired is unaffected
(it was `README.md` §Quick start, in scope).

Ships with a `good/`+`bad/` fixture pair and a `# graph:` manifest coupling the
gate to the doc set, `scripts/*.sh`, and the consumer config, so a transport
rename re-fires it over the docs.

### 5. Consumer registrations — *mechanical*

`scripts/gates.list` gains `check-install-claim`;
`scripts/canon-config.sh` gains the three knob settings, with
`CANON_KIT_INSTALL_SECTION_RE` matching `Quick start` and `Install` and
`CANON_KIT_INSTALL_CLAIM_EXCLUDE` set to `docs/posts/*`.

**The posts valve is load-bearing, not housekeeping.**
`docs/posts/2026-07-26-checkwright-v0-16-0.md` §"If you are installing for the
first time" leads with `npx`, and it is *correct* — that release announced npm
as the activation path. A published post is an immutable record of what was true
at its release, and a gate forcing edits to it would contradict the immutability
`CANON_KIT_TEMPORAL_EXEMPT_PATHS` already grants the same directory for the same
reason.

### 6. Regeneration fan-out — *mechanical*

`bash gate-sdk/bin/gen-pre-commit.sh --write`,
`bash gate-sdk/checks/check-graph.sh --emit > docs/check-graph.html`,
`bash gate-sdk/bin/enforcement-map.sh --emit > docs/enforcement.md`,
`bash scripts/gen-docs-mirror.sh --write` (canon-kit's SPEC and README both
change), `bash context-kit/bin/footprint.sh --emit > docs/footprint.md` (two new
files), and `bash scripts/gen-value-rollup.sh` (per-kit counts derive from the
enforcement map).

## Producers and consumers

**The `install-primary:` declaration.**
*Producer:* the maintainer editing `docs/install.md` §Quick start. Its enabling
config is `CANON_KIT_INSTALL_TRANSPORTS_CMD` and `CANON_KIT_INSTALL_SECTION_RE`
in `scripts/canon-config.sh` — set by this repo in a live tracked configuration,
so the producer is reachable outside fixtures.
*Consumers:* `check-install-claim` assertion A, at the singleton scan (existence,
count, id membership); assertion B, at the per-section walk (the primary id it
compares every section's leading match against).
*Field reader:* the declaration carries exactly one field, `<transport-id>`, and
both assertions read it. There is no unread field, because there is no second
field — the deliberate minimum.

**The transport emit lines.**
*Producer:* `bash scripts/install-transports.sh`, invoked by the gate at
startup through `lib/spec.sh`'s loader.
*Consumer:* `check-install-claim` alone.
*Field readers:* `<transport-id>` is read at the assertion-A membership check
and again in assertion B's red output, which must name which transport the
section led with — a report saying only "wrong transport" would leave the reader
grepping. `<ERE>` is read at assertion B's per-line match. Both fields have a
named reader at a named transition.

**The section-heading regex and the path valve.**
*Producer:* `scripts/canon-config.sh`. *Consumers:* the gate's document finder
(valve) and its section walker (regex). Neither is read anywhere else, and both
default empty so an unconfigured consumer skips rather than scans on a kit
guess.

## Existing sections updated

- **canon-kit/SPEC.md §Layout and configuration** — the three knobs, their
  defaults, and the clean-skip semantics of the two empty ones. Owned by
  delta 2.
- **canon-kit/SPEC.md §lib/spec.sh** — the transport-emit loader beside the
  existing `spec_enum_sets` loader, sharing its fail-closed contract. Owned by
  delta 2.
- **canon-kit/SPEC.md §Per-component contracts** — a new `check-install-claim`
  section carrying both assertions, the leading-vs-mentioning ruling, the
  section-scope calibration and the preamble evidence behind it, and the three
  honest limits (no registry reachability; pattern quality is the consumer's;
  the claim's own prose tier sits in the unscanned preamble). Owned by deltas 3
  and 4.
- **canon-kit/SPEC.md §Content tiering — the star topology** — one sentence
  placing this gate as the star topology's first user-facing application, so a
  reader of that section finds the mechanism that now enforces it beyond
  internal facts. Owned by delta 4.
- **canon-kit/README.md** — the gate roster row, held by `check-readme-roster`.
  Owned by delta 4.
- **docs/install.md §Quick start** — gains the declaration marker; the
  primary-path sentence at lines 28-29 is now the prose tier of a checked claim
  and says so by citation rather than restating the rule. Owned by delta 1.
- **README.md §Quick start** — its "The primary path is the release tarball"
  lead is now a checked claim; no rewording is required, and the amendment
  records that the section is in the gate's scope so a future edit knows it is
  load-bearing. Owned by delta 4.
- **docs/site-architecture.md §Generated projections and their freshness
  gates** — no change: this gate generates nothing. Named here only because the
  install page's other gate (`check-install-toolchain`) is on that roster and a
  reader will look there first.

No update target is unclaimed: every section above names its owning delta.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
