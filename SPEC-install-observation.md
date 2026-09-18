# SPEC amendment: install-observation

The tree can attest what its own gates do; it can attest nothing about what a
gate did to someone who did not write it. This amendment lands the surface that
records that — a private, per-install observation record and a public aggregate
projection over it — so the first externally observed install is recorded in a
shape the claim can later be read off, instead of being remembered.

It pairs two queue entries because the record has **one chance to fix its
fields**. The gate-quality history it must carry cannot be collected
retrospectively: an install whose reds went unrecorded is an install whose
gate-quality evidence never existed. Designing those fields after the first
install would be designing them too late.

**Owner: drift-kit.** Ruled here rather than assumed, because the two entries
were filed against two other surfaces. Four facts decide it, each read off the
tree: `.metric/` is `DRIFT_KIT_METRIC_DIR`, whose stated contract — append-only,
survives a scratch wipe, gitignored, account-bearing — is this record's contract
verbatim; drift-kit already owns **both halves** this channel joins, two private
append-only logs under that dir (the overhead log and the stage-economics log)
and one publicly committed, freshness-gated projection (the published-evidence
extractor behind `docs/evidence-data.md`), so what is new here is the join and
not either half; the gate-quality half keys on the gate roster, which drift-kit
already reads as `DRIFT_KIT_GATES_FILE` and which evidence-kit never reads; and
evidence-kit's whole spine is the validate run, so an observation of a person
outside this tree attests nothing it holds. drift-kit carries no `checks/`
directory and registers no gate, which is why delta 4's gate is consumer-owned.

**The installer takes no delta**, and that is a finding rather than an omission.
Nothing installer-side is a prerequisite for observing an install — the host-floor
facts a preview would meet are outputs of the observation, not inputs to it — and
the record's installer-shaped fields are free text by shape, because a kit literal
spelling one consumer's profile or floor vocabulary is the provenance seam
(gate-sdk/SPEC.md §The provenance seam).

## What changes

### (1) The install-observation record

drift-kit gains a fourth measurement channel, **the install-observation record**:
an append-only private log of installs observed outside the authoring tree, with
a field set fixed before the first observation rather than after it
{design-bearing}. New section `drift-kit/SPEC.md` §The install-observation
record, replacement text below.

**Not yet applied** — proposal text for build to land.

> ## The install-observation record
>
> A gate's fixture pair proves it does what its author specified. It cannot
> distinguish a gate that is right from a gate whose author agrees with it,
> because in the authoring tree the party that writes a red and the party that
> dispositions it are the same party. The distinguishing evidence is a red hit by
> someone who did not write it, and that evidence has one property the kit's
> other channels do not: it is **unrecoverable**. A red nobody wrote down was
> never observed. So the channel's field set is fixed before the first
> observation, not grown to fit the observations that arrive.
>
> The record is one file, `DRIFT_KIT_INSTALL_RECORD`, resolving under
> `DRIFT_KIT_METRIC_DIR` and inheriting that dir's retention and privacy
> contract in full (§Layout and configuration): append-only, surviving a scratch
> wipe, gitignored, never committed. Here the privacy half is the load-bearing
> one — the record keys to people, so the file is the private half of this
> channel and §The install-evidence projection is the only thing about it that is
> ever published.
>
> One line per observation event, whitespace-separated positional fields, the
> date first and the event kind second:
>
> ```
> <date> install <id> <profile> <floor> <ttfg>
> <date> red     <id> <gate> <verdict> <disposition> <behaviour>
> <date> checkin <id> <day> <kit> <retained>
> ```
>
> Fields split into two classes, and the split is the seam rather than a
> convenience. A field the projection aggregates into a published count takes a
> **closed value set**, because an open value there produces a column no
> projection can render honestly:
>
> - `<verdict>` — `true-positive` | `false-positive` | `unclear`. Whether the red
>   named a real defect.
> - `<disposition>` — `fixed` | `worked-around` | `bypassed` | `abandoned`. What
>   the person hit by it did.
> - `<behaviour>` — `changed` | `unchanged`. Whether the red changed what the
>   tree does, which is what separates a gate that taught something from one that
>   was merely satisfied.
> - `<day>` — `7` | `30`. The check-in horizon.
> - `<retained>` — `yes` | `no`.
> - `<ttfg>` — a non-negative integer, minutes from the first install command to
>   the first green gate run, or `-` where no green was reached. `-` is a value,
>   not a gap: an install that never went green is the observation this channel
>   most needs and the one a missing line would erase.
>
> Every other field is **free text validated for shape alone** — `<id>`,
> `<profile>`, `<floor>`, `<gate>`, `<kit>` — because each names a *consumer's*
> vocabulary. A kit literal enumerating one consumer's profile names, host floors
> or gate roster would publish that consumer's vocabulary and be false for every
> other (§the provenance seam gate-sdk owns). The roster a `<gate>` value would
> be checked against is the consumer's own `DRIFT_KIT_GATES_FILE`, and it is read
> by the projection as a *classifier*, never as a validator: an adopter's red may
> name a gate this roster does not carry, and refusing it would discard the
> observation over a naming disagreement.
>
> `<id>` is an opaque install key the observer mints. It is the record's privacy
> hinge: it joins an install's rows to each other without carrying anything about
> who ran it, so the private file can be read by a human and the public
> projection can aggregate over it without either needing a name.
>
> **Three key rules, because assuming one would be wrong for two of the kinds.**
> The sibling meters replace a measured line; this channel mostly does not.
>
> - `red` is **pure append**. Two identical red lines are two reds. A red event
>   has no natural key — the same person can hit the same gate twice, and
>   collapsing that would erase the recurrence the channel exists to measure.
> - `install` **replaces on `<id>`**. One install has one first-green time; a
>   re-measure corrects it rather than adding a second install.
> - `checkin` **replaces on the `<id> <day> <kit>` triple**, the shape
>   §The stage-economics meter already uses for the same reason: a re-run of a
>   check-in must not double-count retention.
>
> **What is not stored, because it is derived.** *First useful red* is the first
> `red` line for an install whose verdict is `true-positive`; *per-gate
> true/false-positive history* is a group-by over `red` lines. Storing either
> would be maintaining a fact the record already determines, and the two would
> then be able to disagree.

### (2) The `file-install` capture arm

A capture affordance on the `kfric` precedent, so an observation is recorded in
the moment rather than reconstructed afterwards {design-bearing}. New arm
`--emit file-install`, specified in §The install-observation record's interface
paragraph, replacement text below; and one new row in `bin/run-gates.sh`'s help
and in drift-kit/README.md §Use.

**Not yet applied** — proposal text for build to land.

> **Interface.** `bash gate-sdk/bin/run-gates.sh --emit file-install [--] <kind>
> <field>...` appends one line, stamping the date from `date +%F` and creating
> the record's parent dir if missing. The three arities are the three grammars
> above, less the date:
>
> ```
> --emit file-install install <id> <profile> <floor> <ttfg>
> --emit file-install red     <id> <gate> <verdict> <disposition> <behaviour>
> --emit file-install checkin <id> <day> <kit> <retained>
> ```
>
> It refuses with a usage message at **exit 2**, writing nothing, on an unknown
> kind, on the wrong arity for the kind, on an empty positional, on a closed-set
> field whose value is outside its set, and on a free-text positional failing the
> shape scan (gate-sdk/SPEC.md §The bin/-tool contract) — every positional
> scanned, not the first, since several slots make arity safe in none. `--`
> ends option processing. `--help` is a **refusal at exit 2**, not usage: a
> non-gate arm's usage lives in `bin/run-gates.sh`'s own help and in this kit's
> README, the spelling `--emit kfric` already carries.
>
> Its declared knob roster is the one row `DRIFT_KIT_INSTALL_RECORD`.
>
> The refusals are the protocol. An observer who runs the arm is told what the
> record wants — that is why the closed sets are enforced at capture rather than
> at projection, and it is the whole of the observation protocol this channel
> needs: a separate protocol doc would be a second home for a contract the arm
> already states, and an always-loaded instruction bullet would convert a bounded
> campaign into a permanent per-session tax (§The knowledge-friction loop's
> standing-instruction rule binds capture here too).
>
> **Named caller and transition**, which gate-sdk/SPEC.md §The non-gate arm
> requires of every arm: the caller is a human observer at an observed install or
> a check-in, and the transition where the output is read is the projection's
> next emission.

The arm also gains smoke coverage in `drift-kit/smoke/install.sh`: the line
grammar for each of the three kinds, the three key rules (a re-filed `install`
and a re-filed `checkin` each replace rather than double, a re-filed `red`
appends), and one refusal per refusal class.

Landing it is one row in the crate's single emit dispatch table,
`native/src/emit/mod.rs`, mapping `--emit-file-install` to its
`Arm::Emit(..)`/`KNOBS` pair — and the shared positional, anchoring and append
helpers the sibling capture arms already import rather than a second
implementation of them.

### (3) The `install-evidence` projection

The public half: an aggregate-only projection carrying the external gate-quality
claim, byte-stable over an unchanged record so a consumer freshness gate can
hold it {design-bearing}. New section `drift-kit/SPEC.md` §The install-evidence
projection, replacement text below.

**Not yet applied** — proposal text for build to land.

> ## The install-evidence projection
>
> `bash gate-sdk/bin/run-gates.sh --emit install-evidence` writes a markdown
> table set to stdout — the shape a consumer's committed projection pins; bare
> invocation prepends a human-oriented header. It degrades per block to an
> `n/a (<reason>)` cell and exits 0, this kit's fail-visible discipline,
> registering no gate of its own.
>
> It is a **pure function of the record**, and byte-stable over an unchanged one:
> it reads no now-relative field, so nothing ages between emissions, and every
> group-by renders in a fixed order — gate and kit names sorted, `<day>`
> ascending — so two emissions over one record cannot differ by iteration order
> alone. That property is what a byte-comparing freshness gate rests on; without
> it the gate would flap rather than assert.
>
> **Every published figure carries its denominator.** The population this
> measures is small by construction, and a rate published without its n is the
> dishonest form of exactly the claim this channel exists to make honest. Blocks:
>
> - **Installs** — observed; how many reached a first green; the min, median and
>   max time-to-first-green over those that did.
> - **Reds hit by a non-author** — the total, split by `<verdict>`.
> - **Per gate** — one row per gate a `red` line names: reds, false positives,
>   reds that changed behaviour. A gate name absent from `DRIFT_KIT_GATES_FILE`
>   is rendered in its own `unrostered` count rather than dropped, so an adopter
>   running a roster this consumer does not carry still contributes.
> - **Dispositions** — the `<disposition>` split over the same total.
> - **Retention** — per `<day>`, per `<kit>`.
> - **First useful red** — installs reaching one, and the median days from the
>   install line to it.
>
> **Three fields never reach the projection**, and the rule is stated as a rule
> rather than left to each block: `<id>`, `<profile>` and `<floor>`. `<id>` keys
> to a person. The other two describe an adopter's machine and choices, which at
> the population sizes this channel is for are near-identifying in combination
> with a date. What is published is the fields whose vocabulary is the
> *publisher's* own — `<gate>`, `<kit>` — plus counts and statistics derived from
> the rest. `<profile>` and `<floor>` are recorded and unpublished deliberately:
> their reader is the observer, at the write-up transition, the same
> human-reader-at-a-named-transition shape queue-kit's `[observed-by:]` value
> already takes.
>
> No row keyed to a single install is ever emitted.
>
> **Named caller and transition**: the caller is the consumer's regeneration step
> before committing its projection, and the gate that byte-compares it at the
> commit — the shape §The published-evidence extractor already carries, and what
> gate-sdk/SPEC.md §The non-gate arm requires of an arm rather than leaving it
> dead weight.

The arm also gains smoke coverage in `drift-kit/smoke/install.sh`: emission over
a synthetic record fixture asserting each block's denominator, the sorted
group-by order, the byte-identity of two emissions over one record, the
`unrostered` classification against a throwaway gates file, and the
record-absent degradation.

Landing it is a second row in `native/src/emit/mod.rs`'s dispatch table, mapping
`--emit-install-evidence` to its `Arm::Emit(..)`/`KNOBS` pair.

### (4) Consumer wiring — the committed page and its freshness gate

This repo's own activation of the channel, kept on the consumer side of the seam
exactly as `docs/evidence-data.md` is {design-bearing}.

- The emission is committed at `docs/install-evidence.md`, with no framing page
  around it, so every number a reader meets there is emitted and none is
  hand-copied — the rule `docs/evidence-data.md` already follows.
- A consumer gate `check-install-evidence-fresh`, declared by
  `scripts/check-install-evidence-fresh.gate` and dispatching to the compiled
  binary, re-emits and byte-compares. Its `# graph:` manifest couples
  `docs/install-evidence.md` to the record path. A `good/`+`bad/` fixture pair
  exercises the byte-compare hermetically by supplying a synthetic emission as a
  second argument, the shape `check-trajectory-fresh` uses for the same reason —
  the real source is not in the tree.
- **The gate is inert — clean on a counted zero, and says so — when the record
  does not resolve to a file.** The record is gitignored, so in CI, in a fresh
  clone and in every adopter's tree there is nothing to re-emit from, and an
  armed gate there would red on every commit. Counted inertness is the shape
  `check-action-pinning` already carries for a tree holding none of its subject,
  and it keeps this gate armed on the one machine that holds the record, where a
  hand-edited or stale page is caught at commit, and silent everywhere else.
  **The honest limit, stated because it is real:** this gate's coverage is one
  machine's, not the battery's, so it is a guard against the authoring session's
  own drift and not an attestation to anyone else.
- Registration in `scripts/gates.list`, and a row in
  `docs/site-architecture.md` §Generated projections and their freshness gates
  carrying the projection's trigger and its regen command, modelled on the
  trajectory projection's row.
- **The new-gate fan-out that page already rosters applies in full**, and it is
  the half of this delta that is invisible from the diff: the on-site SPEC
  mirror, `docs/enforcement.md` (the gate joins the class registry),
  `docs/value.md`'s rollup block derived from it, `docs/check-graph.html`,
  `docs/install.md`'s `ported-gate-members` measured claim (this gate is born
  native, as every new gate is), and — where the gate registers at hook tier —
  the generated hooks. The owning-kit `smoke/install.sh` expected-gate roster is
  **not** in this unit's wake: the gate is consumer-owned, declared under
  `scripts/`, and drift-kit carries no `checks/` directory and registers no gate,
  which is itself the check that the ownership ruling above was applied rather
  than assumed. The staging order that page fixes binds here, because this unit
  adds files: stage first, regenerate second.

**The page is committed from the first build, reading `observed: 0`, rather than
held back until an install exists** (operator direction, 2026-09-18,
lead-relayed — a **direction**, revisable at a later scope or spec, and not a
ruling). Two grounds carried it: a freshness gate whose target does not exist
asserts nothing and a projections roster naming a missing file is false, so
holding the page back costs the enforcement this delta is for; and a published
zero is a candid statement of the same gap this tree already carries unstated
elsewhere, in a project whose posture is that claims are proven rather than
asserted.

**The path not taken**, recorded so build reads a settled question rather than an
open one: land the arms and the gate now and commit the page at the first
observation, trading that enforcement for not publishing a zero pre-launch. It is
revisable on the direction's own terms, but it is **not build's to re-open** —
taking it would change this bullet and the roster row's timing, and nothing else
in this amendment.

**This paragraph is amendment-scoped and does not merge.** The direction's date
and channel are provenance: they belong to this file and to the landing commit,
never to `drift-kit/SPEC.md`, whose consumer-wiring paragraph this delta's
content otherwise joins. What merges is the wiring above; what dies with this
file is the stamp (gate-sdk/SPEC.md §The provenance seam, the publisher-provenance
class).

### (5) Knob rows

Three rows in `drift-kit/SPEC.md` §Layout and configuration's static table, in
the kit's established derived-default spelling {mechanical}.

**Not yet applied** — proposal text for build to land.

> - `DRIFT_KIT_INSTALL_RECORD` — the install-observation record
>   (§The install-observation record); default `.metric/install-observations.log`,
>   derived as `${DRIFT_KIT_METRIC_DIR}/install-observations.log` so a set metric
>   dir moves it (gitignored; the capture arm `mkdir -p`s the dirname). The table
>   is the one resolver of this default, and the capture arm and the projection
>   both read it there.

The projection reads `DRIFT_KIT_GATES_FILE`, an existing row, which gains a
second reader; its row's prose names the trajectory extractor alone today and is
updated to name both.

## Producers and consumers

**The record** (new state, delta 1).
*Producer:* the `--emit file-install` arm (delta 2), run by an observer at an
observed install and at each check-in. Its enabling config is
`DRIFT_KIT_INSTALL_RECORD`, whose default resolves with no consumer action, so
the producer is live in any tree that vendors drift-kit rather than test-only.
*Consumer:* the `--emit install-evidence` arm (delta 3), by file read; and a
human, reading the private file directly at the write-up transition.
*Roster-holding readers of the surface the record's path lands on:* the static
knob table in `drift-kit/SPEC.md` §Layout and configuration (delta 5), whose
rendered form is what `bash gate-sdk/bin/run-gates.sh --emit knob-roster` prints,
so a knob the table omits is a knob the roster cannot name; and `.gitignore`,
which already carries `.metric/`, so the record is covered by an existing line
and no new ignore is minted — verified with `git check-ignore -v .metric/`.

**The `file-install` arm** (new interface, delta 2).
*Producer:* a human observer; invoked through `bash gate-sdk/bin/run-gates.sh`.
*Consumer:* the record file.
*Roster-holding readers of the arm-name surface:* gate-sdk/SPEC.md §The non-gate
arm, `bin/run-gates.sh`'s own help text, the crate's single emit dispatch table
in `native/src/emit/mod.rs`, and drift-kit/README.md §Use. Each is an update
target below. The declared knob roster is one row, the shape
§The knowledge-friction loop fixes for a capture arm. The arm is absent from
`--list` by construction, so the gate-roster parity assertion does not reach it.

**The `install-evidence` arm** (new interface, delta 3).
*Producer:* any caller; and this repo's regeneration step before committing the
page.
*Consumer:* `docs/install-evidence.md`, and `check-install-evidence-fresh`
byte-comparing against it.
*Roster-holding readers:* the same four arm-name rosters as delta 2, plus
`docs/site-architecture.md` §Generated projections and their freshness gates,
which reds a generated projection missing from it.

**The freshness gate** (new obligation, delta 4).
*Producer:* the pre-commit hook and the full battery, through
`scripts/gates.list`.
*Consumer:* the committing session.
*Roster-holding readers of a gate name:* `scripts/gates.list`; the generated
`scripts/git-hooks/pre-commit`, regenerated from the gate's `# graph:` manifest;
and gate-sdk's meta-gates, which red a registered gate with no fixture pair, no
`--reads`/`couples` parity, or no self-lint clean.

**Every field has a named reader, at a named transition.**

| field | reader | transition |
| --- | --- | --- |
| `<date>` | the projection's first-useful-red and retention blocks | emission |
| `<id>` | the projection, as a join key only; never published | emission |
| `<profile>` | the observer | write-up |
| `<floor>` | the observer | write-up |
| `<ttfg>` | the projection's Installs block | emission |
| `<gate>` | the projection's Per-gate block, classified against `DRIFT_KIT_GATES_FILE` | emission |
| `<verdict>` | the projection's Reds and Per-gate blocks | emission |
| `<disposition>` | the projection's Dispositions block | emission |
| `<behaviour>` | the projection's Per-gate block | emission |
| `<day>` | the projection's Retention block | emission |
| `<kit>` | the projection's Retention block | emission |
| `<retained>` | the projection's Retention block | emission |

`<profile>` and `<floor>` have a *human* reader at a *private* transition, which
is a reader by this checklist's own standard — queue-kit's `[observed-by:]`
value is the precedent for a field whose one reader is a human at a named
transition. They are kept because the host-floor and per-profile questions are
the ones the first observations exist to answer; they are unpublished because at
this population size they are near-identifying.

**No delta narrows a corpus**, so point 5 does not bind: every delta adds a
surface, and the one gate this amendment registers is new, so it has no prior
violation set to shrink.

**Point 6 — each member's satisfying value.** The one enumerable corpus this
amendment obliges is the closed-set field roster in delta 1, and each member's
satisfying set is enumerated at the field: `<verdict>`, `<disposition>`,
`<behaviour>`, `<day>`, `<retained>`, `<ttfg>`. Every other field is obliged only
to a shape, so no member is left without a satisfying value. The gate roster is
deliberately *not* such a corpus: the projection classifies against it rather
than obliging membership, precisely because an adopter's gate may not be in it.

## Existing sections updated

Roster produced by: `grep -n "emit \|^## " drift-kit/README.md`;
`grep -n "^## \|^### " drift-kit/SPEC.md`; `git check-ignore -v .metric/`;
reading `docs/site-architecture.md` §Generated projections and their freshness
gates in full, whose own **new-gate fan-out** bullet supplied every generated
surface below; and a delegated read-only sweep over the arm rosters, the
projections roster and `scripts/gates.list`, which located the crate's single
emit dispatch table and confirmed the tree carries no prior per-install
observation mechanism.

- `drift-kit/SPEC.md` §Layout and configuration — the new knob row and the
  second reader on `DRIFT_KIT_GATES_FILE`, whose row names the trajectory
  extractor alone today (delta 5).
- `drift-kit/SPEC.md` §Testing — the smoke assertions for both arms (deltas
  2 and 3).
- `drift-kit/README.md` §Use — the two new arm lines and their one-line pointers
  into the owning SPEC sections, the shape the four existing arms take (deltas
  2 and 3).
- `gate-sdk/SPEC.md` §The non-gate arm — the two new arms. Its prose roster is
  the surface a mistyped arm is sent to, which is what makes it governed rather
  than documentation, so an arm absent from it makes that refusal's own citation
  false (deltas 2 and 3).
- `gate-sdk/bin/run-gates.sh` — the help text's arm listing (deltas 2 and 3).
- `native/src/emit/mod.rs` — the crate's single `--emit-<name>` dispatch table:
  one row per new arm (deltas 2 and 3).
- `scripts/gates.list` — registration of `check-install-evidence-fresh`
  (delta 4).
- `docs/site-architecture.md` §Generated projections and their freshness gates —
  the row for `docs/install-evidence.md` (delta 4).
- `docs/enforcement.md` — regenerated with
  `bash gate-sdk/bin/run-gates.sh --emit enforcement-map > docs/enforcement.md`;
  the new gate joins the class registry (delta 4).
- `docs/value.md`'s rollup block — regenerated with
  `bash gate-sdk/bin/run-gates.sh --emit value-rollup --write`, derived from the
  enforcement map above (delta 4).
- `docs/check-graph.html` and the generated hooks — regenerated with
  `bash gate-sdk/bin/run-gates.sh --emit git-hooks --write` then
  `bash gate-sdk/bin/run-gates.sh --emit graph > docs/check-graph.html`, staged
  before regenerating because both derive through `git ls-files` (delta 4).
- `docs/install.md`'s `ported-gate-members` measured claim — the new gate is born
  native, so the count moves (delta 4).
- `docs/drift-kit/SPEC.md`, `docs/drift-kit/README.md` and `docs/gate-sdk/SPEC.md`
  — the on-site SPEC mirror, regenerated with
  `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` and byte-gated by
  `check-docs-mirror-fresh` (all deltas).
<!-- update-target-exempt: the record file is gitignored and untracked, so it is not a surface any commit updates; it is listed because a reader looking for it in the roster should find the reason it is absent -->
- `.metric/install-observations.log` — created by the capture arm at first use,
  never committed.

## Retired spellings

- None — every delta adds a surface; no name is renamed or removed.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them.
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      components it touched.
- [ ] **Removals propagated** — the negative declaration above re-run against the
      tracked tree by `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      through the gap inbox.
- [ ] **Both paired entries are DEMOTED, not moved to Done.** Each delivers one
      increment of a corpus whose remainder is unbuilt: `design-partner-preview`
      keeps its operator-hours and calendar half, and
      `external-gate-quality-evidence` keeps the published evidence itself, which
      only accrues once observations exist. Drop each `[spec:]` tag, restore each
      `[cost:]` and `[surface:]` from this amendment's promoting commit, and
      return each entry to the position that diff shows it was promoted from
      (canon-kit/SPEC.md §Merging an amendment, step 4). **Measured at promotion,
      so the demotion need not re-derive it:** the deferred `design-partner-preview`
      ran 49 lines against a `QUEUE_KIT_ENTRY_LINE_CAP` of 50, and the promotion
      compressed it to 29; `external-gate-quality-evidence` went 39 to 23. Both
      demote with headroom, so neither needs a compression pass — but the count is
      re-checked rather than assumed, since a build that grows either body spends
      that headroom.
- [ ] **`[surface:]` restored to the ruled owner, not the filed one.**
      `design-partner-preview` and `external-gate-quality-evidence` were filed
      against `installer` and `evidence-kit`; this amendment rules the mechanism
      drift-kit's. Restore both as `[surface: drift-kit]`.
