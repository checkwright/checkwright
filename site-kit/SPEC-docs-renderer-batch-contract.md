# SPEC amendment: docs-renderer-batch-contract

`check-docs-render-fidelity` is the largest single line in this tree's gate
battery, and almost none of that cost is rendering. §Layout and configuration
specifies `SITE_KIT_RENDERER` as a **stdin→stdout single-document** command, so
the gate starts one renderer process per tracked page. The shape is the cost.

**Measured, and restated here because the measurement files are disposable
scratch that dies at the next boundary reset.** On this tree, 68 tracked pages:

| what | measured |
| --- | --- |
| the whole battery (89 gates, the count on the day of measurement) | 40054 ms |
| `check-docs-render-fidelity` alone | 14373 ms |
| 68 pages, a fresh interpreter each (the gate's real shape) | 13875 ms |
| the same 68 pages through **one** interpreter | 450 ms |
| the gate's own per-page `awk` fan-out (4 spawns × 68) | 271 ms |

So ~97% of the gate's cost is interpreter restarts, and the gate is ~36% of the
battery. The probe that produced the last three rows is re-derivable in a dozen
lines: render each page with `ruby -e` in a loop, then pipe the page list into
one `ruby -e` that loops internally, and time both.

**The gate count is a snapshot, not a constant, and a sibling unit moves it.**
`nested-dispatch-ungoverned` ships `check-agent-tier-explicit` in this same
iteration, taking the battery to 90 gates. No gate holds this literal —
`check-manifest-count` scans canonical specs, `README.md` and `CLAUDE.md`, and
excludes amendments — so it is stated as a dated reading rather than a fact the
tree will keep true. The ratios above are what the argument rests on, and they
are unchanged by a gate that adds no render work.

The tax is paid per battery run *and* per docs commit — the gate is
`tier=precommit` coupling `docs/*.md`, so every documentation commit pays it
before it lands, and it grows linearly with the docs tree. Nothing rots; the
gate is correct, only slow.

**Scope boundary.** This amendment changes only the renderer contract — the half
that cannot be mechanical, because it changes a shipped, documented consumer
knob. The 271 ms `awk` fan-out is **not** claimed here: delta 7's projection
carries it forward unchanged, so this unit's measured case stays independently
checkable against the spawn-hoist unit's.

**Where the fan-out does belong, settled at align.** An earlier draft disclaimed
it to "the mechanical spawn-hoist debt entry" on the strength of that entry's
*exclusion* sentence, which actually excluded the docs renderer — leaving the
271 ms orphaned. `gate-battery-spawn-hoists` now **names this gate's four
per-page `awk` spawns in its measured worklist**, so the disclaimer points at a
worklist entry rather than at an inference.

**Ordering that follows from it.** That unit hoists spawns out of the render
loop delta 3 rewrites, so it lands **after** this one; otherwise it optimises a
loop about to be replaced. The two share
`site-kit/checks/check-docs-render-fidelity.sh` and no longer run in parallel.
Delta 7's projection is the handoff: this unit leaves ~450 ms of render plus an
unchanged ~271 ms of `awk`, and that residue is the spawn-hoist unit's opening
measurement.

## What changes

### Delta 1 — a second, optional knob rather than a changed contract {design-bearing}

`SITE_KIT_RENDERER_BATCH`: an **optional** array knob naming a command that
renders **N documents over one stream**. `SITE_KIT_RENDERER` keeps its
stdin→stdout single-document contract, unchanged and still the fallback. When
the batch knob is empty the gate runs exactly the loop it runs today.

**Why not replace the contract.** `SITE_KIT_RENDERER` is shipped and documented;
a consumer pointing it at a bespoke renderer (the version-pin recipe in
§check-docs-render-fidelity's *Parser-version fidelity* passage is the
sanctioned example) would break on upgrade, and every
such consumer would be obliged to implement a framing protocol to get back to
working. Additive is the only shape where no consumer breaks and the framing
obligation is opt-in.

**Why not a content-hash cache** — the alternative that changes no contract at
all, and the one to argue against rather than ignore. Three reasons, in order of
weight:

1. **A stale cache false-cleans.** The worst failure mode available to a gate is
   a silent pass. A cache key that misses any input to the verdict — the
   renderer build, the gate's own scan logic — greens pages that would now red,
   and correct keying requires hashing the renderer's resolved gem set, which is
   exactly the thing the *Parser-version fidelity* passage already says the kit
   will not resolve at gate time.
2. **It concedes the hermetic contract.** The oracle depends on running offline
   and deterministically; persistent cross-run state is the opposite property,
   and `check-test-hermetic` polices that direction in this tree already.
3. **It does not pay where the cost is.** The battery's full run is the
   expensive case and `.tmp/` is wiped at every scope boundary, so the first run
   of each iteration pays the full 14 s regardless.

The cache is refused, not deferred: it buys a smaller speedup at the cost of the
one property a gate may not trade.

### Delta 2 — the batch stream contract {design-bearing}

**NUL-terminated framing, both directions, count-preserving.**

- **In.** The gate writes each page's front-matter-stripped body to the
  command's stdin, each body followed by a `NUL` byte. `NUL` is a *terminator*,
  not a separator, so N documents produce exactly N `NUL`s and there is no
  trailing-empty ambiguity to resolve.
- **Out.** The command writes each rendered document to stdout in the **same
  order**, each followed by a `NUL`.
- **Count.** N documents in, N documents out. Any other count is a contract
  violation and the gate exits 2.

**Why `NUL` and not a sentinel line.** A sentinel (`<<<BOUNDARY>>>`) is forgeable
by document content, and a docs page about this very contract would forge it.
`NUL` is unforgeable here for a stronger reason than "markdown rarely contains
it": bash cannot hold a `NUL` in a variable, so the gate's own pipeline — which
reads each body through command substitution before framing it — has already
dropped any `NUL` in the source by the time framing happens. The delimiter is
outside the alphabet of the data by construction of the reader, not by
convention.

**Why not length-prefixed framing.** It is equally sound and survives command
substitution, which `NUL` does not (see delta 3). It loses on the obligation it
puts on the consumer: a renderer author must implement exact-byte reads in
whatever language their Pages stack uses, where `NUL` framing is a
split-on-a-byte in every one of them. Since the whole point of the additive
shape is to keep the consumer's obligation small, the simpler framing wins and
delta 3 absorbs the shell awkwardness on the kit's side, once.

### Delta 3 — the gate's read loop, and the trap in it {design-bearing}

The non-obvious implementation constraint, stated because a natural reading of
the contract silently breaks it: **command substitution strips `NUL` bytes**, so
`out="$(… | "${SITE_KIT_RENDERER_BATCH[@]}")"` destroys the framing and yields
one concatenated blob with no error. The batch output must never pass through
`$(…)`.

The shape that works, and which this tree already uses elsewhere for
`NUL`-delimited reads:

- a `while IFS= read -r -d '' html; do … done` loop,
- reading from a **process substitution** — `< <(printf '%s\0' "${bodies[@]}" |
  "${SITE_KIT_RENDERER_BATCH[@]}")`.

Process substitution rather than a pipe keeps the loop body in the current
shell, so the findings array it appends to survives. Writer and reader being
separate processes is also what makes the exchange deadlock-free: the gate never
blocks writing N bodies while the renderer blocks writing N documents.

**The count assertion is the fail-closed, not merely a framing check.** Process
substitution discards the renderer's exit status, so the batch path loses the
per-page `fail_closed "$rst"` the current loop has. A renderer that dies
mid-stream yields fewer documents than pages, so the count assertion detects
renderer death, truncation, and framing error alike — one check standing in for
the status the shell threw away. It must therefore exit 2 (fail-closed,
"cannot run my oracle"), never 1 (a finding about the docs).

Per-document scanning is unchanged: each document read off the stream is piped
through the existing `rendered_scan` awk program — a shell variable holding awk
source, not a function, so it is reused verbatim — and the source-side `awk`
passes are untouched.

### Delta 4 — the default, and the coupling that keeps a pin honest {design-bearing}

The kit fills a batch default for its own default stack, so this tree and every
consumer on the Pages default gets the speedup with **no configuration** — the
enabling config is the loader's, emitted on every run, not something a consumer
must remember to set. The default is the same kramdown-with-GFM-input
invocation the per-document default already uses, wrapped in a loop that reads
`NUL`-terminated documents from stdin and writes `NUL`-terminated HTML to
stdout. The literal lands in `lib/site.sh` beside the existing renderer default,
and §Layout and configuration states it there — one home for the value.

**The hazard this coupling exists to close.** Filling the batch default
unconditionally would silently defeat a deliberate override: a consumer who
points `SITE_KIT_RENDERER` at the *Parser-version fidelity* version-locked bundle
but does not know a second knob now exists would have their pinned oracle
replaced by the kit's *unpinned* batch default, and the gate would report clean
against a parser build the consumer explicitly rejected. That is a false clean
produced by an upgrade, which is the failure class this kit least tolerates.

**The rule:** the batch default is filled **only when `SITE_KIT_RENDERER` is
itself still at its kit default.** A consumer who overrode the per-document
renderer and set no batch knob gets the per-document path at today's cost and
today's semantics — slower, and correct. Opting in is one line: point
`SITE_KIT_RENDERER_BATCH` at the batch form of their own pinned renderer. The
version-pin recipe is updated to name both knobs, so the pin is not
half-applied by a reader following it.

**Where that recipe actually lives, verified at align rather than assumed.**
`Parser-version fidelity` is **not** a `site-kit/SPEC.md` section — it is a
bolded paragraph lead-in *inside* §check-docs-render-fidelity (the file's
sections are §The monitor boundary, §Layout and configuration,
§check-docs-cname-parity, §check-docs-render-fidelity, §lib/site.sh,
§templates/site-health.yml, §Out of scope). Every reference to it in this
amendment names the passage, not a section, and the edit lands inside
§check-docs-render-fidelity.

### Delta 5 — probing the batch oracle, and never downgrading silently {design-bearing}

The gate probes the renderer before scanning, and an unresolvable one exits 2
rather than false-cleaning. The batch path takes the same discipline with a
probe that also exercises the framing: two one-line documents in, **exactly
two** non-empty documents back.

Routing, which is where the design decision sits:

- **Batch knob empty** → per-document path, probed exactly as today. Unchanged.
- **Batch knob set and its probe passes** → batch path.
- **Batch knob set and its probe fails** → **exit 2.** Not a fallback.

A silent fallback to per-document would be defensible on speed alone and is
refused on oracle grounds: a set knob is a consumer's deliberate statement about
which parser is authoritative, and quietly running a different one is the same
false-clean class delta 4 closes from the other direction. A gate that cannot
run its configured oracle refuses.

### Delta 6 — the honest limit, stated rather than papered over {design-bearing}

The kit cannot verify that a consumer's batch renderer agrees document-for-
document with their per-document one; a divergent pair yields a divergent
oracle.

**Stated as a new limit, because the existing text does not already carry it.**
An earlier draft justified this as merely widening an obligation the SPEC
already placed on any consumer who overrides the renderer. Checked at align, it
does not: the only sentence on the subject is §Layout and configuration's "A
consumer whose Pages stack differs points this at its own renderer; an
unresolvable one fails the gate closed" — which states the mechanism and the
fail-closed behavior, not a correctness obligation on the consumer's choice. So
this delta **adds** the agreement limit rather than widening one, and it is
placed with §check-docs-render-fidelity's other stated limits.

What the kit does hold: its own two defaults agree, asserted by fixture
(delta 7), so the zero-config path is covered by construction.

### Delta 7 — fixtures and the landing checklist {mechanical}

The gate **accepts** a positional `[docs-dir] [config-file]` form
(`check-docs-render-fidelity.sh:15-16,20`), and that is the seam batch coverage
rides.

**Corrected at align — the fixtures do not use it today.** An earlier draft said
the pair "supplies its own renderer" through that form and priced this delta as
a config file. Read at the fixture: `good/args` and `bad/args` each carry only
`tree/docs`, the three companion `.test.sh` files invoke the gate with one
positional, no fixture `site-config.sh` exists, and the fixtures render through
the **real installed kramdown**. So this delta is the *first* use of the
config-file positional in this gate's fixtures — still mechanical, but it is new
fixture wiring plus a stub batch renderer, not a one-line config. Costing it
honestly here is the point; a build session that budgeted a config edit would be
surprised.

What the fixtures must add:

- a fixture config setting the batch knob, so the batch path is the one under
  test rather than an untaken branch;
- a **parity** assertion — the fixture corpus rendered both ways compares
  byte-identical, which is what makes the zero-config claim in delta 6 an
  assertion rather than a hope;
- a **count-mismatch** case: a batch renderer emitting the wrong number of
  documents exits 2, covering delta 3's fail-closed;
- a **probe-failure** case: the batch knob set to an unresolvable command exits
  2 rather than falling back, covering delta 5's routing.

Plus the ordinary landing set: the knob's literal in `lib/site.sh` paired with
its §Layout and configuration statement (the pairing
`check-knob-default-coupling` reads), and the generated projections regenerated
— each freshness gate names its own regen command on red, so the set is
recovered by running the battery rather than transcribed here.

**Expected result, as a projection and not a promise:** the gate's ~14.4 s
becomes the ~450 ms render plus its unchanged ~271 ms of `awk` plus loop
overhead. The battery run is the measurement; re-measure rather than trust this
number if it has aged.

## Producers and consumers

- **`SITE_KIT_RENDERER_BATCH`** — *Producer:* `site-kit/lib/site.sh`, which fills
  the kit default under delta 4's condition, or a consumer's own site-config
  file. The enabling configuration is genuinely emitted rather than test-only:
  the loader runs on every gate invocation in this tree and this tree does not
  override `SITE_KIT_RENDERER` (its site-config sets only the alias list), so the
  default arms the batch path in the live battery. *Consumer:*
  `check-docs-render-fidelity.sh`, at two named transitions — the pre-scan probe
  (delta 5) and the render loop (delta 3).
- **The framed input stream** — *Producer:* the gate, writing N
  `NUL`-terminated bodies to the batch command's stdin after the existing
  front-matter strip. *Consumer:* the batch renderer process, which splits on
  `NUL`.
- **The framed output stream** — *Producer:* the batch renderer, writing N
  `NUL`-terminated HTML documents in input order. *Consumer:* the gate's
  `read -r -d ''` loop, which hands each document to the existing
  `rendered_scan` at the per-page scan transition — the same reader the
  per-document path has today.
- **The document count** — the one new value the contract carries, and it has a
  named reader: the gate compares documents-read against pages-enumerated after
  the loop and exits 2 on any difference (delta 3). It is read at exactly that
  transition and nowhere else. No other field is added to either stream; a
  framing that carried a page name or an index would have no reader, since order
  already pairs each document with its page, so neither is added.
- **The `exit 2` verdicts** — *Producer:* the probe arm and the count assertion.
  *Consumer:* `run-gates.sh` and the generated pre-commit hook, which treat 2 as
  a hard refusal distinct from a finding — the existing convention, not a new
  one.

**Seam.** The framing convention is generic mechanism: `NUL`-delimited records
are a universal stream convention, carrying no consumer vocabulary and no
project rule content. The knob is the config seam, per config-via-env — a
consumer's renderer identity and their Pages stack stay theirs, and the kit
ships only a default for its own default stack. Nothing here obliges a consumer
to publish anything about their toolchain, and a consumer who ignores the knob
entirely is unaffected.

## Existing sections updated

- **site-kit/SPEC.md §Layout and configuration** — the `SITE_KIT_RENDERER_BATCH`
  entry beside `SITE_KIT_RENDERER`, stating the knob's default literal and the
  delta 4 fill condition, and marking the existing entry as the per-document
  fallback rather than the sole renderer contract (deltas 1, 4).
- **site-kit/SPEC.md §check-docs-render-fidelity** — the oracle paragraph, which
  currently describes one probe and a per-page render, gains the batch stream
  contract, the two-arm probe routing, and the count-assertion-as-fail-closed
  (deltas 2, 3, 5). Its honest-limit passage gains delta 6's renderer-agreement
  limit, placed with the other stated limits rather than appended.
- **site-kit/SPEC.md §check-docs-render-fidelity, its *Parser-version fidelity*
  passage** (a paragraph lead-in in that section, not a section of its own) —
  the version-pin recipe names both knobs, so a consumer following it pins the
  oracle the gate will actually run (delta 4).
- **site-kit/SPEC.md §lib/site.sh** — the loader's "fills every knob's default"
  sentence, which is no longer unconditionally true of every knob: the batch
  default is conditional on the per-document knob (delta 4).
- **`site-kit/lib/site.sh`** and **`site-kit/checks/check-docs-render-fidelity.sh`**
  — the default and the render path (deltas 3, 4).
- **The docs-site mirror of site-kit/SPEC.md** and the other generated
  projections, regenerated (delta 7).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls site-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
