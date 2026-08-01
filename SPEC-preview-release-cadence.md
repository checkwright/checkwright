# SPEC amendment: preview-release-cadence

Reset release *signaling* for a pre-launch audience. The queue entry
(`preview-release-cadence`) already struck the checksummed-asset half as shipped;
what remains is three things a reader of the release history currently cannot
learn: **which channel they are on**, **why the tags come so fast**, and **what a
release means for them without reading the migration detail**.

The premise, re-verified at this spec against `refs/tags`: **21 tags across 18
calendar days**, `v0.1.0` (2026-07-14) through `v0.21.0` (2026-08-01), several
days carrying three or four. The entry's inherited "17 in 14 days" is stale in
the direction that strengthens it. No surface anywhere in the tree names a
release channel, a prerelease posture, or a cadence — searched across
`RELEASING.md`, `docs/install.md`, `README.md`, `.github/workflows/publish.yml`,
`installer/`. This amendment introduces that vocabulary; nothing contradicts it.

**Scope discipline — one mechanism here is already shipped and is not
redesigned.** `docs/install.md` §Versioning already specifies the batching
primitive: a close whose criteria were met but whose release was held back stamps
`deferred:vX.Y.Z`, and `check-release-bump` reads that floor when it derives a later
note's bump (its real extent is stated in Delta 2's honest limit, which is narrower
than the runbook prose and is corrected there rather than inherited). Batching
internal iterations into a slower external
cadence therefore needs **no new state and no new gate** — it needs a policy that
ever invokes the primitive. Delta 2 supplies exactly that and nothing more.

## What changes

### Delta 1 — a declared release channel, gated against the publish posture *{design-bearing}*

`docs/install.md` §Versioning gains a subsection **The release channel**, opening
with a one-line machine-readable declaration of fixed shape:

```
Release channel: **preview**
```

The two admissible values are `preview` and `stable`. The declaration is prose a
reader sees and a token a gate reads — the same one-line-declaration shape
queue-kit's `roadmap-summary:` already uses, and for the same reason: a bracketed
tag would imply a lead-line scanner, and this is read off a line of its own.

The channel is a statement about **audience and support expectations**, not a
second artifact stream. While it reads `preview`: the version line is 0.x, the
tag rhythm is an internal-iteration artifact rather than a stability signal, and
breaking changes ride minors under the pre-1.0 qualifier §Versioning already
carries. It flips to `stable` at `v1.0.0` — the same deliberate cut §Versioning
already calls "the first stability promise", now with a surface that says so
before the reader infers it from tag density.

**Mechanized on exactly one surface.** `.github/workflows/publish.yml`'s `release`
job creates the GitHub Release; while the channel is `preview` that creation
carries `--prerelease`, so every Release page states the posture without a human
remembering to. Nothing else in the publish path changes.

**Ruled out — an npm dist-tag split. Escalated at spec and ruled by the operator
2026-08-01; this is a settled decision, not an author's preference or an
unfinished half.** The obvious stronger signal is publishing to a `preview` (or
`next`) dist-tag rather than npm's default `latest`. It is rejected:
`docs/install.md` §Quick start documents a one-command install, and a
non-`latest` dist-tag makes that command resolve to nothing until the reader
learns to append `@preview`. That converts an honest signal into a
time-to-first-value regression on the precise surface this project's own weakness
list names first. The trade was put to the operator in exactly those terms — a
machine-readable channel signal bought with front-door friction — and declined.
Honest signaling that costs the front door is a bad trade at pre-launch; the
Release flag and the declaration carry the message at no install cost.

**Recorded so a later hardening pass does not "finish the job" by adding `--tag`
to the publish step.** Two things make that reading wrong rather than merely
unsanctioned. The channel is *fully* declared by Delta 1 as it stands — the
prose declaration and the `--prerelease` flag are the whole mechanism, held in
parity by a gate, so there is no dangling half a dist-tag would complete. And the
absence of `--tag` in `.github/workflows/publish.yml` is **load-bearing
configuration**, not an omission: it is what keeps the documented install
command working, so a pass that adds it silently breaks §Quick start.

**The one condition that reopens this**, stated so the decision is falsifiable
rather than permanent: a dist-tag split becomes the right call once the
documented front door no longer resolves through the default tag — i.e. once
`docs/install.md` §Quick start stops promising a bare one-command install, or
once a stable line exists to hold `latest` while preview moves off it. Neither
holds today. `scripts/pack-installer.sh`'s version regex already admits a
prerelease suffix, so nothing here forecloses the change; it is cheap to make
when its condition fires and wrong to make before.

**New gate — `check-release-channel-parity`** (this repo's `scripts/`,
repo-root-governed beside `check-release-bump`, whose surfaces are the same two
tiers). Invariant: the channel declared in `docs/install.md` §Versioning and the
prerelease posture of the Release-creating step in
`.github/workflows/publish.yml` agree — `preview` demands `--prerelease` on that
invocation, `stable` demands its absence. Fail-closed on a missing or duplicated
declaration line, an unrecognized channel value, or a `publish.yml` with no
recognizable Release-creating step: a gate that cannot find one of its two
surfaces refuses (exit 2) rather than passing. `tier=precommit` — both surfaces
are tracked files a commit can desynchronize. Ships with its `good/`+`bad/`
fixture pair per gate-sdk/SPEC.md's fixture-pair contract and registers in
`scripts/gates.list`.

This is the enforcement-first pairing for Delta 1: the declaration and the gate
that stops it drifting from the workflow land in one unit. Without it the
declaration is a comment — the exact failure mode the entry's sibling units keep
finding, where a stated posture and the machine that implements it diverge with
nothing watching.

### Delta 2 — an external cadence policy over the existing deferral primitive *{design-bearing}*

The close stage's release-disposition step reads the consumer's `release-policy`
slot (lifecycle-kit/templates/stages/close.md) and either releases or stamps an
explicit line. This repo's binding in `.claude/commands/close.md` today names
RELEASING.md's procedure and the disposition-evidence path, and does state a
**bump-derived** criterion — an all-None iteration stamps `none`. What it states
nowhere is a **cadence** criterion: nothing asks whether a qualifying iteration
should release *now*, so every iteration whose note earns a bump takes a tag, and
the tag history reads as churn.

**The primitive this delta invokes is not theoretical — it has already run,
including in the shape defer-by-default will make routine.** The disposition
history carries three `none` stamps and roughly ten `deferred:` stamps, among them
two consecutive-deferral stacks that discharged correctly:
`permission-posture-reconciliation` then `per-batch-tiering`, released together at
`v0.14.0`; and `front-door-readiness` then `pre-adoption-grammar-break`, released
together at `v0.18.0`. In both stacks the floor stayed at the *same* version rather
than rising, which is the semver-correct behavior — accumulated minors do not
compound into two minors. So this delta is not asking an untested mechanism to
carry new weight; it raises the frequency of a path already exercised, multi-
deferral form included.

The binding gains a cadence criterion. A close **defers by default** and tags only
when a release trigger fires:

- **Elapsed time** — the newest tag's creator date is ≥ 7 days old
  (`git for-each-ref --sort=-creatordate --count=1 refs/tags`). This is the
  weekly-class cadence the entry asks for, expressed as a floor rather than a
  schedule: it never forces a release, it only permits one.
- **A major** — the accumulated notes since the newest tag derive a major bump
  under §Versioning (a decommission). A decommission waiting behind a cadence
  floor is a deprecation promise coming due late; it releases immediately.
- **A security or supply-chain fix** in the batch. Reaching users late is the
  whole cost being avoided.
- **Explicit operator direction**, recorded in the disposition line's basis.

Otherwise the close stamps `<iteration> release deferred:vX.Y.Z — <basis>`, and
the accumulated declarations ride the next qualifying release. RELEASING.md's
step 1 already anticipates this exactly — *"Because the surface accumulates
across every iteration since the last tag, a release batching several iterations
inherits all of their declarations here"* — so the note-composition half needs no
change; batching is the shape it was already written for, and the two deferral
stacks above are it having been exercised. What changes is how often, not whether.

**The honest limit, stated rather than papered over.** No gate reds a release cut
too soon. The four triggers above are the cases where a fast release is *correct*,
so a timing gate would need an override valve covering all four and would end up
policing the valve rather than the cadence.

What *is* already gated is the half that can silently lose information — but its
coverage is narrower than the governed prose claims, and this delta states the real
extent rather than inheriting the claim. `check-release-bump` reads the outstanding
`deferred:` floor and, while one stands, refuses a **patch-only** bump: an
accumulated deferral cannot be discharged by a release that pretends nothing
accumulated. What it does **not** do is compare the new note's version numerically
against the floor, so it would not catch a deferred *major* discharged as a minor.
`docs/install.md` §Versioning asserts the stronger invariant — a later note *"may
not fall below that version"* — and the gate implements the weaker one. That
divergence predates this unit and closing it is a gate change rather than a policy
one, so it is filed rather than fixed here; it is named because defer-by-default
raises the number of outstanding deferrals and so raises exposure to it. Within
this delta's own trigger set the exposure is covered by policy rather than by the
gate: a major releases immediately and never waits behind the cadence floor.

Timing is policy under a mandatory disposition stamp — and that stamp is not
nothing, because lifecycle-kit already rules that silence is not a disposition.
This limit is recorded here so a later reader does not read the missing gate as an
oversight and mint the weak one.

### Delta 3 — a 30-second human section, ahead of the migration detail *{design-bearing}*

The release note gains a fourth fixed section, **`## In brief`**, placed first —
immediately after the opener, ahead of Tightened gates.

Grammar (owned by `docs/install.md` §The upgrade contract, beside the three it
already owns): three to five bullets, plain language, each answering *what you
get* or *whether you must act*. A bullet lead is a plain phrase, never a gate or
knob name — the machine-readable lead-token sections below it already carry those,
and the whole point of this section is to be readable without them. Unlike the
other three, `## In brief` has **no `None` form**: a release with nothing worth
saying to a human is a patch, and says that in one bullet.

**Why the existing Opener does not already satisfy this** — the question a reader
of RELEASING.md step 1 will reasonably ask, since the Opener carries a
"one- or two-sentence summary" slot. It does not, for two reasons. The slot is
one sentence inside reserved framing, so it reads as a lede rather than a
changelog; and in practice it summarizes the *engineering* (`v0.21.0`: "makes the
battery cost what it does rather than what its process spawns do") rather than
answering whether the reader must act. The Opener keeps its slot and its purpose
unchanged; `## In brief` is the scannable act/don't-act list beside it, which is
what "beside the migration detail" in the queue entry asks for.

**Enforcement — a widening, not a new gate.** `check-release-bump` already fails
closed when a fixed section is absent; the assertion widens from three sections
to four. Verified mechanically safe at this spec: that presence check binds the
**newest note only** (`scripts/check-release-bump.sh` — the three
`section_bullets "$newest_f" …` calls, never iterated over the full row set), so
the historical corpus is untouched and no retro-fabricated summaries are owed.
(`docs/posts/` holds 21 posts but 20 release notes: the undated announcement post
carries no front matter and no `release:` key, so it is not a note at all.) The section parses with the same
`gate-sdk/lib/declaration.sh` `decl_section_bullets` container the other three
use, so the note's four sections cannot diverge in how they are read.

### Delta 4 — the two runbook and chrome updates the above imply *{mechanical}*

`RELEASING.md` step 1's chrome skeleton gains the `## In brief` entry between the
Opener and the three variable sections, citing `docs/install.md` §The upgrade
contract for its grammar rather than restating it — the skeleton's established
pattern. `RELEASING.md` step 2 (bump derivation) states that `## In brief` feeds
no bump criterion, so a reader does not look for one. Purely transcription of
rulings Deltas 1–3 already fixed; no judgment left in it.

## Producers and consumers

**The channel declaration** (new interface, Delta 1).
*Producer:* a human editing `docs/install.md` §Versioning — the single line
`Release channel: **preview**`. Its enabling configuration is nothing: the line is
tracked content in a tracked page, present in every clone, so there is no
deployment on which the producer is unset. (`docs/install.md` is *not* on
`scripts/core-files.list`; tracked-ness alone carries the claim, and the stronger
pinning premise is not available and is not needed.)
*Consumers, both named:* (1) `check-release-channel-parity`, which reads the line
and the `publish.yml` Release step and reds on disagreement — the gate is the
declaration's reader, which is what keeps it from being a comment; (2) a human
reader of the rendered `docs/install.md` page, for whom the line is the answer to
"is this stable yet".
*Every field has a reader:* the declaration carries exactly one field, the channel
value, read by the gate at the parity comparison and by the reader at the
question. The value's two admissible spellings are both read — `preview` demands
the flag, `stable` demands its absence — so neither is a write-only state.

**The `--prerelease` flag** (changed interface, Delta 1).
*Producer:* `.github/workflows/publish.yml`'s `release` job, on the tag-push
trigger `v[0-9]*` — the only path that creates a Release, so the flag is emitted
on every release there is.
*Consumer:* GitHub's Releases page and API, which renders the prerelease badge and
excludes the release from "latest release" resolution. Also
`check-release-channel-parity`, statically.

**The cadence criterion** (new state, Delta 2).
*Producer:* the close stage's release-disposition step, at every iteration close,
reading `.claude/commands/close.md`'s `release-policy` slot. Its enabling config
is the slot itself, which lifecycle-kit already requires be filled and which this
repo already fills — the delta adds criteria to a slot that is emitted at every
close today, not a new slot that some closes might skip.
*Consumer:* `.workflow/release-disposition.txt`, which receives the
`deferred:vX.Y.Z` line, and `check-release-bump`, which reads that file for the
outstanding floor. Both already exist and already handle the `deferred:` form —
this delta produces a value they were built to consume and have not yet seen.
*No new field:* the `deferred:vX.Y.Z` disposition grammar is unchanged; only the
frequency with which it is written changes.

**The `## In brief` section** (new interface, Delta 3).
*Producer:* the close stage's note-authoring step, following RELEASING.md step 1's
chrome skeleton — the same producer as the three sections beside it, on the same
trigger.
*Consumers, both named:* `check-release-bump`'s presence assertion, which reds a
newest note lacking it; and the reader of the rendered post, for whom it is the
30-second read. *Explicitly not a consumer:* the bump derivation — Delta 4 states
this in the runbook so the section's presence is never mistaken for a fourth bump
input.
*Reader survey ran across the whole component set, not a subset.* The note's
existing three sections have three static readers —
`scripts/check-release-bump.sh`, `scripts/check-tightened-gates-grammar.sh`, and
gate-sdk's upgrade smoke — all reached through `gate-sdk/lib/declaration.sh`'s
`decl_section_bullets`. Of the three, only `check-release-bump` scans for section
*presence*; the other two key on the `Tightened gates` section by name and are
unaffected by a section added ahead of it. That is why Delta 3's enforcement is
one gate's widening rather than three.

## Existing sections updated

- **`docs/install.md` §Versioning** — gains the **The release channel**
  subsection (Delta 1). Also carries a **standing falsehood this delta must
  correct**: the section states *"The first tag rides the launch announcement."*
  Twenty-one tags have shipped and no launch announcement has happened, so the
  sentence is false as written. It is replaced by the channel declaration, which
  states the true relationship — the tags are preview-channel iteration
  artifacts and the announcement is a separate, later event. Claimed by Delta 1;
  it is the same false first-tag premise the sibling unit
  `knob-rename-compat-threshold` corrects on the lifecycle-kit side, and the two
  amendments must land the same reading of it.
- **`docs/install.md` §The upgrade contract** — gains the `## In brief` grammar
  beside the three sections it already owns (Delta 3), and states the
  no-`None`-form exception explicitly, since every other section it governs has
  one.
- **The count-bearing sentences that say "three"** (Delta 3). Adding a fourth fixed
  section puts every sentence that counts them in play, and **no gate parses this
  prose**, so a stale count survives a green battery — this entry exists because
  nothing else will catch it. A census at align found **eleven prose sentences
  across four files** (`RELEASING.md` ×3, `docs/install.md` ×6, `gate-sdk/SPEC.md`
  ×1, plus the two gates' message and spec-comment text), and the roster is
  deliberately *not* copied here: it would be a derived list maintained by hand,
  and the build stage re-runs the census against the tree it is editing.

  **What is stated here is the discriminator, because it is a design ruling and
  most of those sentences must NOT change.** A sentence counting *how many fixed
  sections the note has* becomes four. A sentence counting *the sections that bear
  declarations* — the bump inputs, the `None`-form roster, the residue-class folding
  — stays three, because `## In brief` bears none by Delta 3's own ruling that it
  feeds no bump criterion and has no `None` form. Only the newest-note **presence**
  assertion widens. A bare three→four sweep would therefore introduce more errors
  than it fixes; the count words are not interchangeable and the editor must read
  each sentence's referent.
- **`RELEASING.md` §The procedure, step 1** — chrome skeleton gains the
  `## In brief` entry in position (Delta 4).
- **`RELEASING.md` §The procedure, step 2** — states that `## In brief` is not a
  bump input (Delta 4).
- **`.claude/commands/close.md`, the `release-policy` binding** — gains the four
  release triggers and the defer-by-default rule (Delta 2).
- **`.github/workflows/publish.yml`, the `release` job** — the `gh release create`
  invocation gains `--prerelease` (Delta 1).
- **`scripts/gates.list`** — registers `check-release-channel-parity` (Delta 1).
  Registering a `tier=precommit` gate stales the generated pre-commit hook and the
  graph artifact; the full fan-out and its regen commands are
  `docs/site-architecture.md` §Generated projections, which the build stage runs
  rather than this amendment restating.
- **`gate-sdk/SPEC.md`, the `decl_section_bullets` caller list** (Delta 3) — one
  sentence describes `scripts/check-release-bump.sh` as *"counting bullets across
  all three of the note's fixed sections"*. Delta 3 makes the note carry four while
  that caller still counts three, so the sentence becomes false and is reworded to
  name the three it means. **This is the amendment's only kit-side edit, and it is
  named rather than absorbed** because §The seam below rules the seam explicitly and
  an unlisted kit surface would contradict it. It does not disturb that ruling:
  gate-sdk gains no mechanism, no knob, and no new name — the sentence documents a
  *consumer* gate's behavior, and the correction keeps a kit SPEC's description of
  the consumer accurate rather than pushing anything across the seam.
- **No other kit SPEC changes.** Every remaining surface above is
  repo-root-governed. The seam ruling is §The seam below.

## The seam

Ruled explicitly, per CLAUDE.md §The provenance seam.

**Nothing here becomes kit mechanism, and that is the finding rather than an
omission.** A channel name (`preview`), a cadence period (7 days), and a trigger
set are this project's release posture — the exact class of private rule content a
kit literal would publish as everyone's. lifecycle-kit already ships the correct
generic mechanism for all of it: the `release-policy` slot is a consumer-filled
binding, and the mandatory disposition stamp is the kit-side enforcement that the
slot was read. This amendment fills the slot. It adds no knob to any kit, and it
adds no `<KIT>_<KNOB>` env name, so the config-via-env convention has nothing to
bind here.

`check-release-channel-parity` lands in this repo's `scripts/`, not in a kit, for
the same reason `check-release-bump` and `check-tightened-gates-grammar` do: its
two surfaces are `docs/install.md` and `.github/workflows/publish.yml`, both of
which exist because *this* repo publishes an npm package from a GitHub workflow.
A kit-side generalization would need the workflow path, the job name, the
declaration file, and the section heading as four knobs to say something no
vendored consumer has asked for — a kit component with one consumer, which is the
shape gate-sdk/SPEC.md's consumer-first resolution exists to avoid needing.

If a second consumer ever wants it, the promotion path is the established one:
generalize the four surfaces to config, not to literals.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — the false *"The first tag rides the launch
      announcement"* sentence is gone from `docs/install.md`. The tree-wide sweep
      for restatements of the first-tag premise is **not repeated here**: it is
      `knob-rename-compat-threshold` Delta 4, which owns it as a delta and covers
      both instances. One sweep, one owner. This box is satisfied by that delta
      having run and by the two amendments landing the same reading.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
