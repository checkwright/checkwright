# SPEC amendment: port-group

`bin/port-blockers.sh` gains a `--group` arm: the corpus partition over the
registry that §The first cohort, and the rule that selects the next has always
demanded and no session has ever been able to buy.

**The reframing this amendment accepts, and the correction it makes.** The queue
entry offered three candidates. Candidates (1) *derive the grouping from the
manifests plus each gate's corpus primitive call* and (2) *extend
`bin/port-blockers.sh` with a `--group` arm* are **not exclusive**: (2) is the
housing, (1) is the key, and this amendment takes both. Candidate (3) — accept
the residue as small enough to rule per-cohort and retire the rule's
whole-corpus premise — is **rejected**: 63 of 103 registered members remain, the
two largest blocks have never been grouped, and the entry's own cost field is
that the ordering rule's "largest" stays unverifiable while it is.

**Both candidate keys over-select, and neither under-selects.** This is the
correction. `couples=` over-selects, and the canonical spec already says so
rather than leaving it to be probed: criterion 4 records that the expanded
`couples=` derivation is "deliberately **trigger**-shaped" and "wide on
purpose". Measured on this tree, `check-shellcheck` couples
`scripts/*.sh,kit:*.sh` and `check-gate-output` couples
`scripts/gates.list,scripts/*.sh,kit:*.sh` over genuinely different corpora. But
a bare **primitive-call** key over-selects too, and by more: `gate_kit_roots`
has ten gate callers spanning canon-kit, gate-sdk and `scripts/`
(`check-knob-default-coupling`, `check-gate-assertions`,
`check-gate-binary-fresh`, `check-gate-fixture-coverage`, `check-gate-output`,
`check-gate-substrate-parity`, `check-install-disposition`, `check-kit-enum`,
`check-readme-roster`, `check-shellcheck`), which share no corpus whatever.
`check-shellcheck` is the worked case in both directions: it calls
`gate_kit_roots` (`gate-sdk/checks/check-shellcheck.sh:22`), so the claim that
it calls no shared corpus primitive is false — what is true is that it uses the
call as a **root source** and then composes four fixed subdirectory names and a
`*.sh` glob on top of it, while `bin/port-blockers.sh` composes the same call
with one subdirectory. Same primitive, different corpus.

**So the key is neither factor alone; it is set-equality over both.** A single
shared call is not evidence of a shared derivation, and the tool never treats it
as one.

**Why this arm belongs on this tool and not in a new one.**
`bin/port-blockers.sh` is already **criterion 7's roster, derived at each
invocation rather than written down**, for the reason §port-blockers states: a
literal roster cannot be correct for every consumer. Criterion 6 — *its corpus
derivation is self-contained* — is a question of exactly the same kind about
exactly the same corpus, answered today only by hand, per member, at cohort-cut
time. `--group` makes one tool the derived roster for both criteria. It also
inherits the machinery: the arm needs a command-position tokenizer over every
registered gate's declaration, and this tool is the only thing in the tree that
has one.

## What changes

### 1. The `--group` arm

`bash gate-sdk/bin/port-blockers.sh --group` walks the same registry through the
same `gates_list_members` / `gate_resolve` path as the default arm and emits a
**corpus-derivation partition over the still-shell members**, groups ordered by
size descending. **[design-bearing]**

The two arms are exclusive: `--group` replaces the criterion-7 report rather
than appending to it. Both arms remain **advisory by construction** — nothing
machine-parses either output, so `--group` acquires no `# graph:` manifest, no
fixture pair and no freshness gate, for the reason §port-blockers already rules
against one: a freshness gate would compare the derivation against a stored
expectation, which is the maintained roster re-entering by the back door.

### 2. The grouping key: set-equality over two derived factors

The key for a member is the pair **(kit-library call set, content-glob set)**,
each sorted and de-duplicated; two members group together when both sets are
equal. **[design-bearing]**

- **Kit-library call set** — the command-position words the existing `PB_SCAN`
  tokenizer emits that are names in `FUNCS` or `LOCAL_FUNCS`. This arm therefore
  **inverts the default arm's filter**: `bin/port-blockers.sh:297` discards
  exactly these names because a kit-library function is not an external program,
  and they are what the grouping key is made of. No new parsing and no new
  roster — in particular, **no maintained list of "corpus primitives"**. The tool
  does not classify which library calls yield a corpus; it compares whole sets,
  which is what makes the ten-caller `gate_kit_roots` over-selection
  measured above collapse.
- **Content-glob set** — the literal glob tokens (`*.sh`, `*.md`, `*.rs`,
  `*.gate`, `*.list`, …) appearing in the declaration. This is the factor that
  separates members composing the same root source differently, which is the
  `check-shellcheck` case.

### 3. `couples=` is a printed column, never a key factor

Each group prints the expanded `couples=` of its members beside it, read through
the existing `gate_manifest_field` + `gate_expand_couples_var` helpers — the
shared readers, so this report cannot disagree with what `check-graph` and the
generated hook see. **[design-bearing]**

It is a **cross-check, not a key**: fusing a deliberately trigger-shaped field
into a content key is the over-selection criterion 4 names, and burying it
inside a single fused key would hide the disagreement the reader most needs to
see. Where members share a key but diverge in `couples=`, the divergence is a
finding for the cohort session to adjudicate, not something the tool resolves.

### 4. Already-ported members are excluded and counted, not reported undecidable

A member resolving to a `.gate` descriptor is **already ported** and leaves the
partition entirely, counted in the trailing line. **[design-bearing]** This is a
deliberate divergence from the default arm, where a `.gate` member prints `?`
because its external-program requirement genuinely cannot be asked. Here there
is no open question: the grouping exists to order the *remaining* corpus, and a
ported member is not in it.

### 5. Undecidable is reported, never guessed

A still-shell member whose key is **empty in both factors** — no kit-library
call and no content glob the tool can see — prints `?` with its declaration path
and is counted in the trailing line. **[design-bearing]** It is never placed in
a group, and empty-keyed members are never grouped *with each other*: sharing an
absence of evidence is not sharing a derivation. This adopts §port-blockers'
standing rule verbatim, and it is what makes the arm honest about the thing two
failed read-only sweeps could not deliver — the output is a **decidable
partition plus a counted remainder**, never a complete partition claimed as one.

### 6. Cheap criterion columns, so the selection rule is applicable end-to-end

The selection rule wants "the largest set of **criteria-clearing** gates sharing
one corpus derivation", so each member's row carries the three criteria that are
mechanically derivable: **criterion 2** (a fixture pair exists, or the member
carries `# no-fixture:`), **criterion 3** (`tier=` from the manifest), and
**criterion 7** (the default arm's own verdict for that member).
**[design-bearing]**

Criterion 1 is true by construction — the walk is the registry. Criteria 4, 5
and 6 are **not** emitted: each needs judgment the tool cannot take (a
self-referential parity oracle, an aggregate binary-less residual, whether a
duplication is machine-held), and emitting a guess at them would invite a cohort
to be cut on the tool's authority instead of the session's.

### 7. Trailing count line

One line closing the report: members scanned, groups formed, members
undecidable, members already ported and excluded. **[mechanical]** The default
arm's trailing line is unchanged.

### 8. The `bin/`-tool contract, now that the tool has a mode

`bin/port-blockers.sh` takes no positional arguments and gains none, so §The
`bin/`-tool contract's free-text rule does not bind it. Two of its three
behaviors are adopted anyway: `-h`/`--help` prints usage on **stdout** at exit
**0**, and an unrecognized leading-`-` argument is a **refusal** — usage on
stderr, exit 2. **[mechanical]** `--` is not adopted: it ends option processing
in favor of free-text positionals, and this tool has none to end it in favor of.

The ground is the cost that section already measures — a session that "ran a
stage writer with `--help`, got `'--help' is not a lifecycle stage` in place of
usage, and went three guards deep working around a contract the usage text would
have told it did not exist." A tool with one undiscoverable mode is that cost
waiting to be paid; a tool with none was not.

## Producers and consumers

**The `--group` report (new interface).**

- **Producer** — `bash gate-sdk/bin/port-blockers.sh --group`, invoked by hand
  from the repo root by the session cutting a port cohort. Its enabling
  configuration is the one the default arm already resolves and is emitted
  everywhere the tool runs: `gate_sdk_gates_dir` for the registry and
  `gate_kit_roots` for the resolve dirs. No new knob is introduced, so there is
  no new default to be unset anywhere.
- **Consumer** — a **human** session, at exactly one transition: cutting the
  next port cohort under §The first cohort, and the rule that selects the next.
  The consumer is named and singular, and it is why the output stays advisory:
  nothing parses it, so it has no machine consumer to keep a contract with.
  `native-gate-port-remaining-corpus` is the queue entry standing at that
  transition, and its own amendment (`SPEC-eighth-cohort.md`) names this report
  as the input its selection step runs.

**Field readers, one per emitted field.**

| Field | Reader | Transition read at |
| --- | --- | --- |
| group key (kit-lib call set, content-glob set) | cohort-cutting session | verifying two members share a derivation before pairing them |
| group size ordering | cohort-cutting session | applying "largest set" |
| `couples=` column | cohort-cutting session | adjudicating a key/manifest disagreement |
| criterion 2 / 3 / 7 columns | cohort-cutting session | applying "criteria-clearing" |
| `?` rows | cohort-cutting session | bounding the error in "largest" |
| trailing counts | cohort-cutting session | sizing the remaining corpus |

No field is emitted without a reader above; the criteria-4/5/6 columns were
considered and **removed** under that rule, since their only honest reader would
have had to disregard them.

**This delta narrows no corpus.** It adds an arm to a tool and prunes no file,
no glob and no declaration, so causal-completeness point 5's red-condition
enumeration has no subject here. The one behavior that *removes* rows —
excluding already-ported `.gate` members from the partition (delta 4) — removes
them from a **new** report that no reader has yet, not from the default arm,
whose output is byte-unchanged by this amendment.

## Existing sections updated

- **gate-sdk/SPEC.md §port-blockers** — owned by deltas 1–8. The section
  currently describes a single-arm tool ("Run `bash gate-sdk/bin/port-blockers.sh`
  from the repo root"). It gains the `--group` arm, its key, the exclusion of
  ported members, the criterion columns and the argument behaviors. Its standing
  rulings are **extended, not restated**: "Undecidable is reported, never
  guessed" now covers the empty-key case, and the no-freshness-gate ruling is
  stated once for the tool rather than per arm.
- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  owned by deltas 1, 2 and 6. The rule's sentence "The next cohort is the largest
  set of criteria-clearing gates sharing one corpus derivation" acquires the
  instrument that makes it applicable. The paragraph beginning "**A primitive's
  *remaining* consumers are derived, never recorded here**" already states the
  derive-don't-record doctrine and the one-command derivation for a *single*
  primitive's leftovers; it is updated to name `--group` as the whole-corpus
  form of the same doctrine, so the two are not read as rival instruments.
- **gate-sdk/SPEC.md §The port-candidate criteria, criterion 6** — owned by the
  amendment's framing paragraph. Criterion 6 asks whether a member's corpus
  derivation is self-contained and is answered per member by hand; it gains a
  pointer to `--group` as the derived roster, matching criterion 7's existing
  pointer to the default arm.

## Definition of Done

- [ ] **Causal completeness** — the `--group` report has a named, reachable
      producer (the tool, no new knob) and a named consumer (the cohort-cutting
      session); every emitted field has a named reader at a named transition.
- [ ] **Merged with no information lost** — deltas integrated into
      §port-blockers, §The first cohort, and criterion 6; the merged spec reads
      as one document describing a two-arm tool.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      gate-sdk (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather
      than at the commit while `SPEC-eighth-cohort.md` is in flight.
- [ ] **Removals propagated** — grepped every spec for the claim that
      `port-blockers.sh` takes no arguments.
- [ ] **Gaps filed** — cross-component gaps found during the work filed as debt
      tasks.
