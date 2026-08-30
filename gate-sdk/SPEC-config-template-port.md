# SPEC amendment: config-template-port

The port disposition of the **config-seam class** — every kit's
`templates/<kit>-config.sh` and the `<gates-dir>/<kit>-config.sh` copy `init`
seeds from it. Cut 1 of 3 of the `port-declaration-cohort-and-windows-leg`
declaration cohort, sequenced first because it is the only one of the three
carrying a sibling declaration on its own ground.

**The deliverable is the class ruling and the declarations it licenses.** Here
the ruling comes back uniform, and the uniformity is itself the finding the
entry called for: the ground is either right for the whole class or right for
none of it, and it is right for the whole class.

## What changes

### (1) A class ruling, `gate-sdk/SPEC.md §Layout and configuration` gains *The config-seam port disposition*

The config seam is permanently shell because it **is** the seam rather than
mechanism reaching it, and porting a seam deletes the thing there is to edit
{design-bearing}. The class and its ground are already stated, in a section this
ruling cites rather than restates: `installer/README.md §What init seeds` rules
that "the config seam is derived, never listed: a kit's consumer config is
whatever `templates/*-config.sh` it ships, and the destination is always your
gates directory under the file's own name", and that these are "precisely the
files whose whole purpose is to be edited by you". A class defined by a
derivation and grounded in a stated purpose needs no roster here and gets none.

**The ruling has a sibling declaration on exactly this ground, and that is what
sequences this cut first.** `drift-kit/templates/drift-config.sh` declares
`# no-port:` on it verbatim — "it **is** the adopter's config seam rather than
kit mechanism reaching it, so porting it deletes the seam: there would be
nothing left for a consumer to edit" — landed at the drift-kit cut. This ruling
is that declaration's ground read across its own class, which is what the entry
means by *right for all of them or right for none*.

**Two mechanisms in this tree already treat the class as an edit surface, and
they are cited as evidence rather than as argument.** `installer/lib/init.sh`
compares a placed file's on-disk hash against the hash it was written with
before rewriting it, so an adopter's edit is detected and preserved on a re-run
rather than clobbered — the seam is a file the installer expects to have been
changed. And §check-template-copy-parity excludes `*-config.sh` from the
template↔copy parity assertion **by name suffix**, on the stated ground that "a
config template is a starting point the consumer customizes, so equality would
be the defect". A gate that refuses to hold this class to its template is a gate
already saying the class is the consumer's.

**The class is the whole population on both sides of the seam**, template and
seeded copy alike, because they are one artifact at two points of one
derivation: the copy is what the template exists to become, and a ruling that
took only one side would leave the other re-arguing the same ground at the next
cut. `gate-sdk`'s own `<gates-dir>/gate-sdk-config.sh` is already declared and
is the shape the rest of the copies take.

**This does not reverse the 2026-08-24 vocabulary ruling, and the dates are why
the two never met.** That ruling generalised
`scripts/measured-claims.sh`'s cause **on its private-vocabulary half alone**,
and it enumerated four of this class's copies as left owed *deliberately*,
because they hold layout rather than vocabulary — a verdict this ruling agrees
with and does not touch. The edit-seam ground is a different ground, and it did
not exist on 2026-08-24: its first statement anywhere is the drift-kit
declaration above, five days later. The two grounds are cumulative, not
competing, and a file may be reached by either. The caution that ruling states
for itself — that over-declaring wrongly excuses a file from the port and
mis-sizes a governed completion predicate with nothing red to catch it, where
under-declaring is visible and cheap — is adopted here rather than set aside: it
is why this cut declares only files whose whole documented purpose is to be
edited, and why its sibling cuts leave every member a thin ground reaches.

**What reopens it**, on §Consumer smoke *The port disposition*'s terms: the
ground dissolves for a kit whose config template stops being seeded into the
consumer's gates dir — the derivation `installer/README.md §What init seeds`
owns — because the file is then kit mechanism reaching a seam rather than the
seam. A kit that ships no config template is simply not a member; nothing here
obliges one to exist.

**The honest limit.** This ruling says nothing about the *knob defaults* a
config template's owning kit library holds. Those live one directory over, they
are a different question, and they are the sibling cut's — stated here because
the two look alike from a distance and a reader meeting one first should not
carry its answer to the other.

### (2) Eleven `# no-port:` header declarations

Every member of the class that does not already carry one gains a single
`# no-port: <cause>` header line naming the ruling that makes it permanent
{mechanical}: the seven kit `templates/<kit>-config.sh` files, and the four
`<gates-dir>/` copies not already declared under the 2026-08-24 vocabulary
ruling. Each cause points at delta (1)'s subsection, on the pattern the smoke
cut set for a class spanning kits — a member outside gate-sdk cites the arguing
section and says it is reached by ground rather than by scope. No file gains any
other field, none carries `# port-until:`, and no `.gate` descriptor is touched.

### (3) The reached surfaces record the disposition in one sentence each

The ruling reaches by ground, so each owning kit's `§Layout and configuration`
records its own template's disposition beside the knob roster that template
seeds, and `installer/README.md §What init seeds` records that the class it
derives is permanently shell {design-bearing}. Each is one sentence naming the
disposition and pointing at delta (1), never a second copy of the argument —
the restatement `check-surface-duplication` and the content-tiering rule both
exist against.

## Producers and consumers

The only new state is a **port-disposition declaration** on eleven tracked shell
files. No new field, tag, knob or interface, and no change to any knob's value.

- **Producer** — the build session's declaring commit writes one `# no-port:`
  header line per member. Its enabling path is the file being tracked and
  nothing else: §The `# graph:` manifest rules the fields' domain to be any
  tracked script with "no registration step, which is the whole reason the field
  can reach a corpus that owns no descriptor".
- **Consumer 1 — §port-blockers' `--tree` arm**, which reads the header block
  and reclassifies each row `owed` → `no-port`. This is the reader TRAJECTORY.md's
  completion predicate is stated over.
- **Consumer 2 — §check-gate-substrate-parity assertion G**, once the sibling
  amendment `SPEC-port-declaration-shape.md` widens its corpus to the tracked
  shell tree. Until that lands no gate reads any of these eleven causes, which is
  the enforcement-first case for pairing the two and the reason this amendment
  names the dependency rather than assuming it.
- **Consumer 3 — §check-comment-tier**, whose built-in directive roster already
  carries both spellings over the whole governed tree, so a declaration line
  reads as a directive rather than a restatement. No widening needed.
- **Consumer 4 — canon-kit's measured-claim oracle**, transitively:
  `scripts/measured-claims.sh` reads the `--tree` trailer's owed count into the
  `tree-shell-owed` key and the generated hooks bake that resolved value, so a
  declaration stales them. A named reader at a named transition, and the reason
  the regeneration below is an update target rather than a courtesy.

**No field is added without a reader.** A `# no-port:` payload is free text by
§The `# graph:` manifest's own ruling, read by consumer 2 for non-emptiness and
by a human for the ruling it names. No cause here carries a slug or a vocabulary
token — which is what keeps a cut about the provenance seam from crossing it.

**A caller-side check the class invites and this cut does not need.** Every kit
library defaults its `<KIT>_CONFIG_FILE` to `<gates-dir>/<kit>-config.sh` and
sources the file only if it exists, so a member's declaration changes no
resolution path and no absent-file behavior. Verified at each library's own
default rather than assumed from one, because the class's whole subject is the
file those defaults name.

**This delta widens no corpus and narrows none**, so §The causal-completeness
check point 5's red-condition enumeration does not bind: every reader sees more
declarations and fewer owed rows, and no reader over this corpus reds on
*finding none*, asserts an exact count, or holds a coverage floor. The one count
that moves, `tree-shell-owed`, is cited by no governed sentence today.

## Existing sections updated

- `gate-sdk/SPEC.md §Layout and configuration` — gains the *The config-seam port
  disposition* subsection (delta 1).
- `gate-sdk/SPEC.md §check-template-copy-parity` — its `*-config.sh` exclusion
  sentence gains the note that the excluded class is this cut's corpus and the
  in-scope class is the sibling cut's, so a reader arriving at either meets the
  partition (delta 1).
- `installer/README.md §What init seeds` — records that the class its derivation
  defines is permanently shell, beside the derivation itself (deltas 1 and 3).
- Each owning kit's `§Layout and configuration` — `canon-kit/SPEC.md`,
  `context-kit/SPEC.md`, `delegation-kit/SPEC.md`, `evidence-kit/SPEC.md`,
  `guard-kit/SPEC.md`, `lifecycle-kit/SPEC.md`, `queue-kit/SPEC.md` — each
  records its own template's disposition in one sentence (delta 3).
- `drift-kit/SPEC.md §Layout and configuration` — records that its already-landed
  declaration is now the class's precedent rather than a lone case, so a reader
  meeting it first is sent to the ruling (delta 1).
- The generated projections this cut stales: the on-site SPEC mirror, and the
  generated `pre-commit`/`commit-msg` hooks together with
  `docs/check-graph.html`, which `docs/site-architecture.md` §Generated
  projections names as staling when "a script header gaining a `# no-port:`
  cause moves the `tree-shell-owed` key" — verified against the committed hook,
  which bakes the owed count today (all deltas).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather
      than at the commit, this iteration carrying sibling amendments.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The oracle moved by the roster, not by the trailer** — every member of
      the class reports `no-port` on `--tree` and no file outside it changed
      disposition, read as a per-file diff.
- [ ] **The regeneration fan-out is discharged in the landing commit** — the
      generated hooks and the graph artifact, stale through the
      `tree-shell-owed` key.
