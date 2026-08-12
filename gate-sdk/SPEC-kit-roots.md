# SPEC amendment: kit-roots-cohort

The next increment of `native-gate-port-remaining-corpus`: the **five-member
`gate_kit_roots` cohort** — `check-kit-registration`, `check-smoke-entry-guard`,
`check-test-hermetic`, `check-assertion-strength`, `check-template-registry-parity`
— ported to compiled subcommands, their shell originals deleted. Operator-ruled at
the `port-oracles-and-kit-roots` scope; the ruling's grounds are TRAJECTORY.md
§PRIORITY DIRECTIVE — the port track's sequence and are not restated here. The
porting procedure is gate-sdk/SPEC.md §Porting a gate to the binary substrate and
the payload rule is §Consumer payload; this amendment adds only what those do not
already say about these five.

**It depends on `SPEC-port-criteria.md`** for one clause — criterion 4's own
predicate — and says so rather than re-arguing it. The two may land in either
order in the same iteration; this cohort may not land while criterion 4 still
borrows assertion C's term, or its fifth member reads as a criterion-4 violation
in the very document that admits it.

## What changes

### (1) The shared derivation, and why these five are one cohort — **design-bearing**

The selection axis is §The first cohort's rule: *the largest set of
criteria-clearing gates sharing one corpus derivation*. All five derive their
corpus from `gate_kit_roots` and nothing else structural — a sweep of the kit
roots, then a fixed literal sub-path under each (`smoke/install.sh`,
`smoke/violation.sh`, `gate-tests/*.test.sh`, `smoke/*.sh`, `bin/*.sh`,
`templates/*.list`). That derivation is **already compiled**: `walk::kit_roots`
and `walk::kit_roots_rel` are bridged resolved values, landed with the canon-kit
cohort, so the axis that made the first cohort cheap is paid for before this one
starts. What each member adds on top is a per-file text test, which is the whole
of its rule.

The economy is therefore the first cohort's at five members instead of two: the
walk is ported once and proved five times, and the parity comparison is over one
corpus shape.

### (2) The criteria, per member, machine-checked where a machine could check — **design-bearing**

- **1 (registered)** — all five are in `scripts/gates.list`. Clear.
- **2 (fixture pair)** — all five carry `good/`+`bad/` under
  `gate-sdk/gate-tests/<name>/`. Clear, with one property worth recording because
  it is unusual and load-bearing: `check-kit-registration`'s `good/` case takes
  **no args** and runs against the repo's own `README.md` and live kit roots, so
  its parity run is a live-tree comparison by construction rather than by a second
  oracle. The other four steer onto synthetic kit trees through a positional root
  plus the case dir's `gate-sdk-config.sh`.
- **3 (`tier=precommit`)** — all five. A green `check-graph` after the port is
  end-to-end proof the manifest survived the substrate change.
- **4 (not substrate-sensitive)** — **four clear by name and one does not, and
  this is the finding that corrects the selection premise.** gate-sdk/SPEC.md
  §Meta-gate conservation names `check-kit-registration`,
  `check-smoke-entry-guard`, `check-test-hermetic` and `check-assertion-strength`
  as outside the derived set. It does **not** name `check-template-registry-parity`,
  which instead takes a disposition row of its own — and running assertion C's
  runtime derivation over the live registry reports it **substrate-sensitive**:
  its `couples=` carries `kit:*/*.sh`, which expands to `gate-sdk/*/*.sh` and
  covers `gate-sdk/checks/check-shellcheck.sh`. The verdict was read off the
  oracle rather than off the conservation table, which is what the taking session
  was told to do and what turned a pre-cleared cohort into a four-of-five one.

  **It ports anyway, and the ground is criterion 4's own predicate**
  (`SPEC-port-criteria.md` delta 1): the couple is a **reverse trigger**, and the
  corpus this gate scans as content is `<kit>/<name>/` for each
  `<kit>/templates/<name>.list` — live, `drift-kit/kpis/` and
  `gate-sdk/msg-patterns/`, neither holding a gate declaration. Its own
  conservation row already reads it that way. The port therefore changes nothing
  the gate reads, and the parity proof is not self-referential. Recorded at this
  length because every mechanical screen puts this member *out*, and the fact that
  admits it is one no reading of the conservation table alone supplies.
- **5 (vendored form stays runnable)** — per member, cleared by the ruled install
  model: a host with no artifact gets the member omitted-and-declared in its
  `gates.list`. **The cohort half is delta (5) below** and is not discharged by
  this line.
- **6 (self-contained corpus derivation)** — clear for all five, and the
  qualification is discharged in its strongest form for the shared part:
  `gate_kit_roots`/`_rel` cross the bridge as resolved values, so the duplication
  is *absent* rather than machine-held. One thing that looks like a criterion-6
  problem and is not: `gates_list_members` (`gate-sdk/lib/gate.sh`) is a
  comment-and-blank line filter over a `.list` file. That is a **content
  grammar**, not a corpus derivation — it selects lines inside a file the gate was
  already handed, never which files are scanned — so criterion 6 does not reach
  it and the Rust form implements it directly.
- **7 (no external program off the floor)** — clear for all five. Two invoke
  `git` (`check-kit-registration`'s `ls-files` over `<kit>/gate-tests/`,
  `check-template-registry-parity`'s over the sibling directory), which is the
  floor's one sanctioned exception; the other three spawn nothing. Every other
  external the shell forms use — `awk`, `grep`, `sort`, `comm`, `diff` — is an
  implementation of the rule, not a requirement of it, and disappears with the
  shell.

**No ERE engine is owed by this cohort, and it is stated so the nine-member engine
debt is not read as reaching here.** `check-assertion-strength` carries the only
pattern work in the five: three bash `[[ =~ ]]` tests and the `# exit:` token
grammar its `awk` implements. Every one of those patterns is a **kit literal** —
authored in the gate, invariant across consumers — not a consumer-configured POSIX
ERE the rule *interprets*. The distinction is the one
`cohort-held-members-port-prerequisites` draws for its nine members, and it lands
on the other side here: the port hand-writes three bounded matchers and one line
grammar, and inherits none of that entry's work.

### (3) The one blocker found, and its answer — **design-bearing**

`check-kit-registration` resolves its two documents as
`${1:-${GATE_SDK_REGISTRY_DOC:-README.md}}` and the same for
`GATE_SDK_RUNNER_DOC`. Both knobs exist **only inside the check**, with the
default inline; `gate-sdk/lib/gate.sh` defines neither. The config bridge's third
refusal is exactly this case — *a knob the owning kit's library does not define* is
exit 2, because serializing it as empty would hand the reader a fail-open dressed
as a default (§lib/gate.sh). A compiled `check-kit-registration` declaring either
knob therefore fail-closes on every invocation.

**The answer is to move the resolution, not to hold the member.** Both defaults
move into `gate-sdk/lib/gate.sh` as resolved library globals in the
`GATE_PRUNE_DIRS` shape the bridge names as its own precedent — one place the
value is computed, the crate holding no default to drift from, the positional
override unchanged for both substrates. The knob names the consumer writes
(`GATE_SDK_REGISTRY_DOC`, `GATE_SDK_RUNNER_DOC`) and their documented defaults are
untouched, so no consumer configuration changes and §Layout and configuration's
roster keeps its current entries.

This is the class of finding criterion 7's worked example exists to surface
*before* a port rather than during one, and it is the reason the cohort's first
build batch is the library move rather than the first gate.

### (4) The port itself — **mechanical**

Five Rust modules under `native/src/gates/`, five `.gate` descriptors replacing
five `.sh` files in `gate-sdk/checks/`, five registry entries in the crate
declaring each member's knobs and walk roots. Each module's rule is a direct
transcription; the mechanisms it draws on already exist:

- shallow per-kit listings (`templates/*.list`, `<dir>/*.sh`, `gate-tests/*.test.sh`)
  go through the crate's single sanctioned walk (`walk::glob_files`), so the
  `check-reads-couples` recorder still observes every root and unit test A stays
  complete;
- `git ls-files` goes through `proc::run`, the crate's one spawn site, so a child
  that exited non-zero yields no stdout;
- set differences (`comm -23` / `-13` over sorted unique name sets) become sorted
  `Vec` comparisons, and the one member that prints a corpus keeps set-identity
  with the sort order the crate's walk imposes — the same deliberate difference the
  canon-kit cohort recorded, asserted rather than normalised away.

**Parity is proved while both implementations exist**, on each fixture pair and on
the live tree, before any `.sh` is deleted — the only order in which parity can be
proved at all, since assertion A forbids a descriptor and a script coexisting in
one dir.

**Four of the five are registered in `gate-sdk/smoke/install.sh`** —
`check-smoke-entry-guard`, `check-test-hermetic`, `check-assertion-strength`,
`check-template-registry-parity` — so the port makes them *dispatch* in the
scratch consumer, not merely declare. That path is already built: the consumer
smoke's artifact arm builds the binary and packs it, which is what a registered
dispatch with no binary needs (§The first cohort). `check-kit-registration` is not
registered there and stays a declaration-only member in that tree.

### (5) The cohort's aggregate price, measured — **design-bearing**

The first customer of criterion 5's new cohort half (`SPEC-port-criteria.md`
delta 2). The measurement is `installer_smoke`'s value arm against the
post-cohort registry, and its verdict is recorded here at the port rather than
inferred.

**What the reasoning predicts, so the measurement has something to falsify**: all
five gates assert over **kit-authored** files — a vendored kit's own `smoke/`,
`gate-tests/`, `templates/` and registry rows — never over adopter-authored
content. The value arm plants its defect in an adopter's own README, so a
binary-less consumer losing these five should lose **no** class the arm measures,
and the cohort's aggregate price should be zero on that oracle. If the run
disagrees, the reasoning is wrong and the run wins.

**What the price is not.** `installer_smoke` is held `fail` on the baseline
against `port-criterion-aggregate-cost-blindness` half (2), which is deferred and
keeps that slug. That held row records the **markdown-link** hole this cohort
neither widens nor repairs; reading it as this cohort's price would be reading one
suite's verdict as one cohort's.

### (6) The queue entry is demoted, not moved to Done — **mechanical**

`native-gate-port-remaining-corpus` is the **whole corpus** and this cohort is one
increment of it, so the terminal move is the demotion canon-kit/SPEC.md §Merging
an amendment step 4 specifies: drop the `[spec:]` tag, return the entry to the
deferred section under `[design-pending]`, keep the `[roadmap:]` tag the public
projection reads. A Done move here would assert a finished port and silently drop
the item from `ROADMAP.md` while eighty-odd gates remain. Stated as a delta because
the contract has no gate behind either half.

### (7) One stale count corrected in passing — **mechanical**

§The canon-kit `spec_manifest_files` cohort closes by pointing at
`cohort-held-members-port-prerequisites` *"where the ERE engine is now owed by four
members rather than one"*. That entry was re-censused at this iteration's scope
over all 85 remaining shell gates and the figure is **nine members across three
kits**. Corrected in place rather than filed: the owner doc already carries the
right number, this is a restatement that went stale, and this amendment is the one
editing the neighbouring prose.

## Producers and consumers

Five new interfaces, one per member, plus one relocated configuration value. No
new message and no new field, so point 4 of the causal-completeness check binds on
the descriptors' declared fields rather than on a wire format.

- **Producer of each subcommand**: the crate's dispatch registry
  (`native/src/gates/mod.rs`), which a member cannot compile without an entry in.
  Its enabling config is the `.gate` descriptor plus the member's row in
  `scripts/gates.list`, both of which exist today for the shell forms and are
  edited in place — so the producer is reachable in the real configuration, not
  only under test.
- **Consumer of each subcommand**: `gate_command` (§lib/gate.sh), through the
  argv it emits, driven by `run-gates.sh`, the generated pre-commit hook
  (`gen-pre-commit.sh`), and `run-gate-tests.sh` for the fixture pairs. Three of
  the four also reach it in the scratch consumer through
  `run-consumer-smoke.sh`.
- **Every declared field has a named reader.** A descriptor's fields are the
  closed roster §The `.gate` descriptor fixes — `# graph:`, `# install:`,
  `# spec:` — and each is carried verbatim off the deleted `.sh`. Their readers
  are unchanged and named: `check-graph` and `gen-pre-commit.sh` read `# graph:`;
  `check-install-disposition` reads `# install:`; `check-spec-pointer` and
  `check-comment-tier` read `# spec:` through
  `canon-kit/lib/spec.sh`'s `spec_comment_surface`, whose `*.gate` and `*.rs` arms
  already landed. No field is added, so none is unread.
- **Producer/consumer of the relocated knob values** (delta 3): produced by
  `gate-sdk/lib/gate.sh` when a kit library is sourced, consumed by the shell
  `check-kit-registration` today and, after the port, by the bridge — which reads
  the resolved value, tab-joins it into `GATE_SDK_KNOB_<NAME>`, and hands it to
  the binary, whose `--knobs check-kit-registration` arm declares it off registry
  data the module cannot compile without.

**Each reader's red condition, because this delta NARROWS a corpus** — five files
leave `gate-sdk/checks/*.sh` — and a narrowing is exactly where a
clear-by-inspection is wrong (§The causal-completeness check, point 5). Enumerated,
not cleared:

- `check-gate-substrate-parity` **assertion A** reds on a dir carrying both
  `<name>.sh` and `<name>.gate`: the deletion and the descriptor land in the same
  commit, or the intermediate state is red. **Assertion B** reds both ways on a
  descriptor/`--list` mismatch, so the five modules and the five descriptors are
  one unit. **Assertion C** reds on a substrate-sensitive member with no
  disposition — `check-template-registry-parity` keeps its existing row and the
  other four stay outside the derived set, so no row is owed; and the
  §Meta-gate conservation sentence naming those four as untouched stays true
  verbatim. **Assertion E** reds on an implementation sibling under a kit root,
  which is why the modules go to `native/src/gates/` and never beside their
  descriptors.
- `check-gate-fixture-coverage` reds on a **registered member with no fixture
  pair** — a zero-count red, non-monotone under a narrowing, so it is checked
  rather than assumed: the five `gate-tests/<name>/` directories are **kept**, not
  deleted with the scripts, and the fixture runner's substrate-blind dispatch runs
  them against the binary.
- `check-gate-output` reds on a member whose declaration path lacks the
  `: clean` / `help:` strings. A `.gate` cannot hold them by the closed field
  roster, so coverage follows the rule to the module and the runtime assertion in
  `run-gate-tests.sh` — the resolution §check-gate-output already owns, arriving
  for five more members.
- `check-shellcheck`, `check-gate-fail-closed`, `check-exec-bit` each scan
  `check-*.sh` and get a **smaller** corpus; all three red on *finding a
  violation*, so a narrowing can only remove violations. Monotone; cleared by
  inspection, and named so the clearing is on the record.
- `check-readme-roster` reds on a gate missing from its kit README's roster in
  either direction. Its glob already unions `*.sh` with `*.gate`, so five renames
  are invisible to it — **provided the descriptors keep the same basenames**,
  which they do.
- `check-reads-couples` reds on a walk outside a member's declared couples, and on
  a binary member it consumes `--reads` rather than parsing shell. Each of the
  five declares its roots beside its dispatch entry, and crate unit test A runs
  each member over its own `good/`+`bad/` cases with recording on.
- `check-kit-registration` **is one of the five and reads the README it is being
  ported out of** — the one place this cohort could be self-referential. It is
  not: its corpus is the README plus `git ls-files <kit>/gate-tests/`, and the
  port deletes neither. Its `good/` case runs against the live tree, so the
  parity run proves it on the corpus it will actually have.

## Existing sections updated

- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** — a
  subsection recording what this cohort landed, in the shape §The canon-kit
  `spec_manifest_files` cohort has: the shared derivation, the criterion-4 finding
  on the fifth member, and the library move delta (3) bought. Coordinate with
  `SPEC-port-criteria.md` delta (3), which edits the same section's
  `check-roadmap-fresh` bullet.
- **gate-sdk/SPEC.md §The canon-kit `spec_manifest_files` cohort** — the stale ERE
  beneficiary count in its closing sentence (delta 7). Nothing else in that
  subsection changes.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — nothing
  is added; the sentence naming the four as not substrate-sensitive and the
  `check-template-registry-parity` row are **verified unchanged**, and this
  amendment records that verification as the delta so a merge does not quietly
  edit either.
- **gate-sdk/SPEC.md §lib/gate.sh** — the two relocated document knobs join the
  resolved-globals list (delta 3), and §Layout and configuration's existing
  `GATE_SDK_REGISTRY_DOC` / `GATE_SDK_RUNNER_DOC` entries are re-pointed at their
  new resolution site with their defaults unchanged.
- **gate-sdk/SPEC.md §check-kit-registration, §check-smoke-entry-guard,
  §check-test-hermetic, §check-assertion-strength,
  §check-template-registry-parity** — each gains the substrate line its ported
  siblings carry; no invariant and no calibration changes, because a port that
  changed a rule would have no parity to prove.
- **`scripts/gates.list`, `gate-sdk/README.md`'s roster, the generated pre-commit
  hook, `docs/enforcement.md`, `docs/footprint.md` and the graph artifact** — all
  are generated projections or rosters the port stales; each is regenerated by the
  command `docs/site-architecture.md` §Generated projections rosters for it, not
  hand-edited.

## Definition of Done

- [ ] **Causal completeness** — five subcommands with a named producer (the crate
      registry, enabled by descriptor plus `gates.list` row) and a named consumer
      (`gate_command`'s argv, via three execution sites); every descriptor field
      has a named reader; every reader's red condition under the narrowing is
      enumerated above and the three zero-count readers are checked rather than
      inspected.
- [ ] **Parity proved before deletion** — both substrates run over each fixture
      pair and the live tree while both exist, byte-identical or set-identical with
      the recorded sort difference.
- [ ] **Aggregate price measured** — delta (5)'s `installer_smoke` verdict recorded
      in the merged section, not inferred from the per-member reading.
- [ ] **Merged with no information lost** — each addition lands in its proper
      section; the merged §The first cohort reads as one narrative a reader who
      never saw this amendment can use.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather than
      at this commit, since sibling amendments are in flight for gate-sdk.
- [ ] **Entry demoted, not Done-moved** — delta (6).
- [ ] **Removals propagated** — grep every spec and doc for the five `.sh` paths;
      nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      through `lifecycle-kit/bin/file-gap.sh`.
