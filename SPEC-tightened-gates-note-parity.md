# SPEC amendment: tightened-gates-declaration-note-parity

*(Filed as `SPEC-tightened-gates-note-parity.md` — the full slug plus a `[spec:]`
pointer overruns `check-queue-wrap`'s lead-line budget by one column, the bound
queue-kit/SPEC.md §The tag algebra states as a true state of the queue. The
filename matches the gate this amendment introduces.)*

Close the honest limit RELEASING.md step 1 states about itself: *"nothing asserts
the composed section's set equals the surface it was composed from. You transcribe
one into the other and drain by hand at step 4, and review is what holds the
agreement."* This amendment supplies the assertion and deletes that limit.

Both sides are machine-readable and both parsers already ship in
`gate-sdk/lib/declaration.sh` — `decl_record_tokens` reads the bare-name-per-line
declaration surface, `decl_section_tokens` reads the note's backticked unbolded
bullet lead tokens. The assertion is a set comparison over two existing parsers.
**The whole design question is *when* it may run**, and the survey behind this
amendment settled it against the tree rather than against the entry's guess.

## The bind point, and the observation that decides it

The entry framed this as "the assertion only holds at the release commit". That
framing is what made the unit `[design-pending]`, and it is **too coarse**: the
release choreography is not one commit but two, and the surfaces are comparable
in the gap between them.

Verified against `git log`, the last three releases each ran the same two-commit
shape:

- `docs(release): author the vX.Y.Z release note` — composes the note's
  Tightened-gates bullets **from** the surface. The surface is untouched and still
  carries every accumulated name. **Both sides are non-empty and equal here, by
  construction.**
- `chore(release): stamp the vX.Y.Z release disposition and drain the
  declarations` — truncates the surface to its header line, stamps the
  disposition, and is the commit the tag points at (confirmed:
  `git rev-list -n1 v0.21.0` is that commit).

So there is a live window — every commit from the note's authoring until the tag
lands — in which the assertion is not merely holdable but *cheap*, and after which
it is meaningless because one side is deliberately empty.

**The predicate: a note whose declared version carries no tag yet is a note under
composition.** The gate arms on exactly that and disarms on its own, with no
state file, no marker, and nothing for a human to remember:

- **Armed** — the newest `docs/posts/` note declares `release: vX.Y.Z` and
  `git tag --points-at`/`git rev-parse` finds no `vX.Y.Z`. The note is being
  composed; assert parity.
- **Dormant** — every note's version is tagged. The surface has been drained by
  contract and the note is the sole record; there is nothing to compare, and
  comparing anyway would red permanently on every clone forever.

This is the same tag-resolution move `gate-sdk/bin/upgrade-smoke.sh` already makes
to pick its declaration source, so the gate reads release state the one way this
tree already reads it, rather than inventing a second reading.

## What changes

### Delta 1 — `check-tightened-gates-note-parity` *{design-bearing}*

A new gate in this repo's `scripts/`, registered in `scripts/gates.list`.

**Invariant.** While a release note is under composition — its declared version
has no tag — the token set of its `## Tightened gates` section equals the token
set of `.workflow/tightened-gates.txt`'s data lines. Set equality, **both
directions**: a name on the surface and missing from the note is a gate that
tightened and shipped undeclared, licensing an undeclared red that the upgrade
smoke will then wave through; a name in the note and missing from the surface
declares a gate that never tightened, which sends every consumer hunting a
reconcile that does not exist. The entry costed both, and one-directional
containment would catch only the first.

**Resolution and fail-closed behavior.** The gate refuses (exit 2) rather than
passing when it cannot establish its own preconditions: more than one untagged
note (the release choreography admits exactly one in flight, and two means a
state this gate cannot reason about); a note whose Tightened-gates section is
absent or unparseable; a declaration surface missing its required header line. A
gate that cannot find one of its two surfaces declines to certify — it never
reports clean. Where no note is untagged, the gate reports clean **and says it is
dormant**, so a reader of a green run can tell "checked and equal" from "nothing
to check" rather than reading dormancy as verification.

**Tier `precommit`.** Both surfaces are tracked files, the comparison is a
sub-millisecond set diff over two already-loaded parsers, and the commit that can
break parity is exactly the commit a pre-commit gate sees. Ships with its
`good/`+`bad/` fixture pair per gate-sdk/SPEC.md's fixture-pair contract; the
`bad/` fixture covers both directions of the inequality, since a pair covering
only the dropped-name direction would leave the added-name arm unproven.

**Parser reuse is the point, not an optimization.** The gate calls
`decl_section_tokens` and `decl_record_tokens` — the same two functions
`check-tightened-gates-grammar` and the upgrade smoke already ride. If parity were
asserted through a second, private parser, the gate could pass while the smoke
read a different token set from the same bytes, which is the exact class of defect
this unit exists to close, reintroduced one layer down.

### Delta 2 — RELEASING.md step 1 loses its honest limit *{mechanical}*

The paragraph beginning *"**Honest limit, and it is yours at exactly this
moment:**"* is deleted and replaced with a sentence naming the gate that now holds
the agreement and stating that the transcription is checked at commit time, not by
review. This is transcription of a ruling Delta 1 fixes; no judgment remains.

Deleting it rather than softening it is deliberate: a stated limit that has been
closed is worse than one never stated, because a reader who trusts the runbook
will keep performing the manual read-across it prescribes.

### Delta 3 — the two-commit choreography becomes stated contract *{design-bearing}*

The gate's arming predicate depends on the note being authored in a commit
*before* the drain. Today that ordering is convention — observed in every release,
enforced by nothing, and stated nowhere as a requirement. Delta 1 makes it
load-bearing, so it must stop being an accident.

`RELEASING.md` §The procedure states the ordering explicitly: the note-authoring
commit precedes the drain-and-stamp commit, and the two are not to be squashed
into one. Step 4 already carries the adjacent tag-ordering invariant (*"write the
drain-and-stamp commit, push master, watch the `gates` run for that SHA go green,
and only then tag that commit"*), so this lands beside a rule the runbook already
owns and in the same voice.

**The residual this does not close, stated rather than left for a reader to
discover.** If an author composes the note and drains the surface in one commit,
the gate never sees a comparable state and stays dormant — it does not red, it
simply has nothing to say. Enforcement of the *split* is the runbook's, not this
gate's, because a pre-commit gate cannot distinguish "note and drain in one
commit" from "note authored while the surface was already empty" without reading
history it does not have. The honest claim is therefore: **parity is gated over
the choreography the runbook prescribes, and the choreography itself is
prescribed prose.** That is strictly stronger than today, where neither is held,
and stating the seam is what stops the next reader over-reading the gate's
coverage — the failure mode this iteration's own queue records twice as reading
an oracle's satisfaction as the specification.

## Producers and consumers

**The parity assertion** (new interface, Delta 1). It introduces no new state,
no new file, and no new field — that is the design's principal claim, and it is
what makes the unit small.

*Producer:* `scripts/check-tightened-gates-note-parity.sh`, run by
`gate-sdk/bin/run-gates.sh` on every battery run and by the generated pre-commit
hook on every commit. Its enabling configuration is registration in
`scripts/gates.list`, which is this repo's only gate-enablement surface and is
present in every clone — there is no deployment on which a registered
`tier=precommit` gate goes unrun, since the hook is generated from the manifest
rather than maintained beside it.

*Consumers, both named:* (1) the committing agent or human, who receives the red
and the two-sided diff; (2) `gate-sdk/bin/run-gates.sh`'s battery result, which is
what the `gates` workflow watches to green before a tag is cut — so the assertion
also reaches the remote oracle on the release push, not only the local hook.

*Inputs and their readers.* The gate reads three existing surfaces and each read
is already someone else's contract:

- `.workflow/tightened-gates.txt` data lines — written by the build stage
  (`lifecycle-kit/templates/stages/build.md`, appending in the commit that lands
  or tightens a gate), contract owned by `gate-sdk/SPEC.md` §upgrade-smoke, read
  here through `decl_record_tokens`. Already read by the upgrade smoke's
  untagged-`TO` arm; this gate is its second reader, not its first.
- The newest `docs/posts/` note's `## Tightened gates` section — written by the
  close stage per RELEASING.md step 1, grammar owned by `docs/install.md` §The
  upgrade contract, read here through `decl_section_tokens`. Already read by
  `check-tightened-gates-grammar` and by the upgrade smoke's tagged arm.
- The note's `release:` front-matter version, resolved against `refs/tags` — the
  arming predicate. Already read by `check-release-bump` (for ordering) and by the
  upgrade smoke (for declaration-source selection).

*No new field exists to want a reader*, and that is checked rather than assumed:
the amendment adds no key to the front matter, no line form to the declaration
surface, and no section to the note. Every value it reads had a named reader
before this amendment and keeps it.

**The stated commit ordering** (Delta 3).
*Producer:* the close stage author following RELEASING.md §The procedure.
*Consumer:* `check-tightened-gates-note-parity`'s arming predicate, which is
satisfiable only across the split — named here so the runbook's ordering rule
carries its reason rather than reading as ceremony.

## Existing sections updated

- **`RELEASING.md` §The procedure, step 1** — the honest-limit paragraph is
  deleted and replaced by a pointer to the gate (Delta 2). This is the prose the
  unit was filed from, so leaving it would be the amendment contradicting its own
  deliverable.
- **`RELEASING.md` §The procedure** — gains the note-before-drain ordering
  statement (Delta 3), beside step 4's existing tag-ordering invariant.
- **`docs/install.md` §The upgrade contract** — the paragraph naming
  `check-tightened-gates-grammar` as what holds the Tightened-gates section over
  the corpus gains its sibling: the grammar gate holds each note's section
  *well-formed*, the parity gate holds a note under composition *equal to the
  surface*. Two gates, two claims, stated together so a reader does not read
  either as the other's coverage.
- **`gate-sdk/SPEC.md` §upgrade-smoke** — the section owns
  `.workflow/tightened-gates.txt`'s contract and describes the accumulate/compose/
  drain flow this amendment now constrains. It gains one sentence naming the
  consumer-side parity gate as a second reader of the surface. **Nothing else in
  gate-sdk changes** — see §The seam.
- **`scripts/gates.list`** — registers `check-tightened-gates-note-parity`
  (Delta 1). A `tier=precommit` registration stales the generated pre-commit hook
  and the graph artifact; the fan-out and regen commands are
  `docs/site-architecture.md` §Generated projections.

## The seam

Ruled explicitly, per CLAUDE.md §The provenance seam.

**The gate lands in this repo's `scripts/`, not in gate-sdk.** The survey behind
this amendment found no stated rule placing a parity gate between these two
surfaces, so the call is made here rather than inherited.

The reason is the note side. `docs/posts/` is *this* repo's release-note corpus,
its section names and bullet grammar are owned by `docs/install.md` — a page in
this repo's own public site — and the two-commit release choreography Delta 3
pins is `RELEASING.md`'s, which is repo-root-governed meta. A kit-shipped parity
gate would need the posts directory, the front-matter key, the section heading,
the surface path, and the release-commit shape as five knobs to say something no
vendored consumer has asked for. That is a kit component with one consumer, and
the tell is already on the tree: gate-sdk ships the *parsers* and declines to ship
the note-side gate, which is why `check-tightened-gates-grammar` and
`check-release-bump` both already sit in `scripts/`. This gate joins its two
siblings, and the placement is now stated rather than pattern-matched.

**What stays kit mechanism is exactly what already is.** `decl_record_tokens`,
`decl_section_tokens`, and the `DECL_TOKEN_RE` shape are gate-sdk's and are reused
unchanged — the amendment adds no parser, no knob, and no `<KIT>_<KNOB>` name to
any kit. gate-sdk's only edit is one sentence naming a second reader of a surface
it already specifies, which is documentation of a fact rather than new mechanism.

**No private rule content crosses.** The gate names no gate, no release, and no
version; it compares two token sets whose members are supplied entirely by the
consumer's own tree.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — RELEASING.md step 1's honest-limit paragraph is
      gone, and grepped tree-wide for any restatement of the "review is what holds
      the agreement" claim (`docs/install.md` and the close-stage template are the
      candidates).
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
