# SPEC amendment: third-batch

**The third budget batch: `check-memory-off` (89 shell lines) and `check-root-tiering` (67).**
`bash gate-sdk/bin/port-blockers.sh --group` read every group a singleton at the 2026-08-18 cut,
which is the budget arm's stated precondition, so the increment is a hand-composed batch rather
than a cohort. Per the property gate-sdk/SPEC.md §The first budget batch fixed, **the batch is not
one unit of work and is not merged, recorded or argued as one**: each member takes its own
descriptor, its own registry entry, its own parity run and its own deletion, and either is
droppable without touching the other or the iteration's freshness spine.

**What this amendment owes, and what it deliberately does not.** Under the arm's
record-only-findings rule the gate-sdk section this lands carries the batch's *findings* and the
verdict its cut owed — never a member roster, which is derivable from the tree and whose count
`scripts/measured-claims.sh` emits. The findings are the criterion-2 correction (delta 2), the
discharge and the redundant arm it dissolves (deltas 3 and 4), the two semantic divergences the
JSON re-use exposes (delta 5), and the jq boundary (delta 6).

## What changes

### 1. Both members become descriptors and crate modules

`context-kit/checks/check-memory-off.sh` is deleted and replaced by
`context-kit/checks/check-memory-off.gate` plus `native/src/gates/memory_off.rs`;
`gate-sdk/checks/check-root-tiering.sh` by `gate-sdk/checks/check-root-tiering.gate` plus
`native/src/gates/root_tiering.rs`. Both `scripts/gates.list` registrations are unchanged — each
member was always registered, and only the declaration spelling moves, which is what keeps
criterion 1 satisfied through the port. **design-bearing**

### 2. `check-memory-off`'s hold is criterion 2, not criterion 7, and the record is corrected

**The premise this member was selected on is wrong, and correcting it is the batch's first
finding.** The 2026-08-18 cut recorded on `freshness-emitter-port-cohort` admits
`check-memory-off` on a criterion-7 argument: its `jq` use is a path query the already-ported
`check-settings-pins` performs on the same grammar, so the design criterion 7 demands is paid.
That argument is true and it answers a criterion this member was never held on. **mechanical**

Two passages of the governing spec hold it on **criterion 2** instead:

- §The first cohort, and the rule that selects the next — *"`check-memory-off` fails criterion 2
  decisively: its `--fixture <dir>` arm is a different code path from its live arm, and the live
  arm's corpus is not in the tree at all — it is the harness memory directory under `HOME`. A gate
  whose fixture arm bypasses the derivation being ported has no parity oracle for the part that
  matters."* Recorded second there, as the weaker ground: it also word-splits an override into
  multiple globs and reads four knobs.
- §The settings cohort, and the crate's first dependency — *"held on criterion 2, named and not
  ported… What it owes is criterion 2's **constructed-scenario** discharge, and that is the cohort
  taking it. **Its blocker is the oracle, not the dependency**, so this cohort's reader does not
  unblock it."*

That last clause is decisive: the spec has already adjudicated that the dependency axis is not
this member's blocker, so retiring `jq` cannot retire the hold. Spec-over-precedent makes
gate-sdk/SPEC.md ground truth over a queue entry's framing, so **the entry is corrected rather
than left carrying a superseded rationale** — what pays for the member is the criterion-2
discharge below; the jq re-use is the real-but-secondary dividend of delta 6.

Recorded because a member admitted on the wrong criterion reads, to the next selector, as evidence
that the criterion was satisfied.

### 3. The `--fixture <dir>` arm is deleted, which dissolves the hold rather than working around it

**The second code path criterion 2 names is redundant, and nothing in the record had noticed.**
The shell gate's live arm resolves three paths from three knobs — `CONTEXT_KIT_MEMORY_DIRS` for the
scanned dirs, `CONTEXT_KIT_SETTINGS_PINS` for the pins manifest, and the local settings file
derived from `CONTEXT_KIT_SETTINGS_FILE` by swapping `.json` for `.local.json`. Its `--fixture
<dir>` arm resolves the same three from `<dir>/memory`, `<dir>/settings-pins.conf` and
`<dir>/settings.local.json`. **Every path the fixture arm redirects, a knob already redirects**, so
the arm buys no reach — it buys a shorter spelling, and pays for it with the divergent code path
that is the whole of the criterion-2 finding. The ported member has **one arm**, knob-driven.
**design-bearing**

This is an interface removal and is flagged as one rather than folded into an implementation note:
`--fixture` is a documented flag on a governed surface, and its deletion is why this unit is a
feature.

What follows from the collapse, and it is the point of taking it:

- `context-kit/gate-tests/check-memory-off/{good,bad}/` re-point from `--fixture` onto the three
  knobs through their `args`/environment. The pair keeps the memory-dir axis it already fixes and
  **stops being vacuous**: it now drives the same code path the live battery drives, which is the
  property criterion 2 says it lacked.
- `context-kit/gate-tests/check-memory-off.test.sh` — the behavioral test holding the
  local-override axis the pair cannot express — re-points the same way and is kept. It is named
  explicitly because a port that reads it as a shell artifact of the deleted script would delete
  the only coverage of red condition 2.
- `check-gate-fixture-coverage` reds on a registry member carrying neither a pair nor a
  `# no-fixture:` reason. The pair is kept, so it stays green; a port that had dropped the pair
  along with the arm would have found that out at the battery.

### 4. The constructed scenario covers the one derivation no fixture can reach

Collapsing the arms leaves exactly one thing untested by any in-tree case: the **default**
derivation, `context_memory_dir_default`, which reads the harness layout under `HOME` and is the
corpus §The first cohort says *"is not in the tree at all"*. That is criterion 2's own
constructed-scenario shape, and the discharge is bought once, at port time: stand the state up in
a throwaway tree, run both implementations over it, compare bytes and exit codes.
**design-bearing**

The scenario is a throwaway `HOME` with the harness project layout beneath it, and the matrix both
substrates run over:

- memory dir **absent** (the CI-neutral fail-open case, whose clean line states the caveat);
- memory dir holding **only `.gitkeep`** (clean — the dir-preserving file is not content);
- memory dir holding **a regular file** (red condition 1);
- local settings **absent** (clean);
- local settings setting a pinned key **to its pin** (clean);
- local settings setting a pinned key **off-pin** (red condition 2);
- local settings setting a pinned key to JSON **`null`** (delta 5b's divergence, pinned by a case
  rather than by prose);
- an override resolving to **multiple globs**, word-split — the "weaker ground" the spec recorded
  second, which no single-dir case reaches.

**Honesty about the mechanism's fit.** Criterion 2 writes the constructed scenario for a
`# no-fixture:` member, whose state has no static representation at all. This member *has* a pair;
what it lacked was a pair that reached the ported derivation. Delta 3 fixes that half in the tree,
and the scenario covers only the residue — the default path. The extension is stated rather than
assumed, so a later member does not cite this as licence to replace a feasible pair with a
scenario.

**And one arm cannot be compared, which is recorded rather than glossed.** The shell fails closed
(exit 2) when a local settings file is present and `jq` is not. The ported member reads JSON
itself, so the branch has no counterpart: it is **retired**, not **proved equal**, and the parity
run records it as an expected divergence rather than reporting a clean sweep it did not have.

### 5. Two semantic divergences the JSON re-use exposes, each pinned as a delta

The re-use is real — `native/src/json.rs`'s `Path::compile` accepts `.field`, `."quoted key"`,
`.a["k"]`, `.a[N]` and `.`, covering every construct the `<path> = <expected JSON>` grammar of
`scripts/settings-pins.conf` permits and every pin this gate reads. It is not, however, a
substitution: two behaviors differ, and both are named here so the parity run's expected
disagreements are known before it runs rather than explained after. **design-bearing**

**(a) Comparison becomes structural, deliberately.** The shell compares `jq -c`'s compact output
against the pins right-hand side by **raw string equality**, so `1` and `1.0`, and a right-hand
side carrying internal whitespace, compare unequal. `values_equal` compares **structurally**. The
ported member takes the structural semantics: the manifest declares *expected JSON*, not an
expected byte form, and `check-settings-pins` already applies structural equality to that same
manifest — so keeping the shell's string equality would leave two gates disagreeing about one
grammar. A change, recorded as one.

**(b) Null handling is opposite, and `settings_pins.rs`'s branch must not be re-used verbatim.**
`check-memory-off` **silently skips** a null actual: a null means the local file sets no override
for that key, which is this gate's ordinary clean case. `settings_pins.rs` treats
`actual.is_null()` as a fail-closed **absent pin** — correct for the tracked file it reads, and a
**correctness regression** here, since it would red every tree whose local settings simply omit a
pinned key. The ported member re-uses `Path::compile`/`eval` and `values_equal` and supplies its
own null disposition. This is the one place where "re-use rather than re-implementation" is false
in detail, and delta 4's scenario carries a case for it.

### 6. The jq dividend, and the boundary no reader may take it past

`port-blockers.sh`'s criterion-7 report over 104 scanned members names `check-memory-off` the
battery's **only** remaining `jq` consumer — the other external-program rows are `shellcheck`
(twice), `cargo`, `ruby` and `paste`. So the port subtracts `jq` from **the gate battery's**
dependency floor, against TRAJECTORY.md objective 1. **mechanical**

**It retires `jq` from nothing else, and the phrasing is held to that.** `installer/lib/` shells to
it on the shipped install path and refuses naming the program where it is absent
(installer/README.md §Requirements); guard-kit, the delegation-kit templates, drift-kit and
`scripts/` carry their own uses. *"The batch retires jq"* is false in every direction but the
battery's. §The settings cohort's honest-claim paragraph is the model, and this one is written to
its bar rather than to a looser one.

### 7. `check-root-tiering` ports with no finding, and its accounting says so

A batch member that produces no finding still owes its reckoning, or the batch's silence reads as
coverage. Its derivation is the `git ls-files` first-segment set of a scan root tested against an
allowlist; positional `$1` is the allowlist path (default `GATE_SDK_ROOT_ALLOWLIST`, itself
defaulting to `<gates-dir>/root-allowlist.list`), positional `$2` the scan root (default `.`); a
non-repo cwd or an unreadable allowlist is fail-closed (exit 2); an absent allowlist falls back to
the built-in orientation set, and entries match as exact names or globs. **design-bearing**

Every primitive is one the crate already carries — `proc::run("git", …)` for the tracked set,
`walk::knob_scalar` for the knobs, the registry-style list reader for the allowlist, and the glob
matcher criterion 6's answer already required. Its hermetic fixture pair carries its own
`allow.list`, `root/`, `args` and `expect.txt` and drives both positionals, so unlike its
batch-mate's it is a genuine parity oracle and the port needs no scenario beside it.

**Seam note.** The built-in fallback set is generic orientation — `README.md`, `LICENSE`,
`.gitignore`, the `SPEC-*.md` amendment glob — plus two configured knobs (`GATE_SDK_QUEUE_FILE`,
`GATE_SDK_AGENT_FILE`), so nothing consumer-shaped crosses into the crate. The allowlist proper
stays optional consumer config on the `graph-vocab.sh` pattern: a file the gate reads, never a
literal it carries.

### 8. Descriptors, and the substrate-sensitive set re-derived at the cut

Each member's `# graph:` manifest and `# install:` line carry across unchanged —
`couples=scripts/settings-pins.conf dir=one valve=none tier=precommit trigger=*` for
`check-memory-off`, `couples=scripts/root-allowlist.list dir=one valve=none tier=precommit` for
`check-root-tiering`, both `on-surface` — and each gains the crate modules its gate reaches
transitively. Knob declarations: `CONTEXT_KIT_MEMORY_DIRS`, `CONTEXT_KIT_SETTINGS_FILE` and
`CONTEXT_KIT_SETTINGS_PINS` on the first; `GATE_SDK_ROOT_ALLOWLIST`, `GATE_SDK_QUEUE_FILE` and
`GATE_SDK_AGENT_FILE` on the second. **design-bearing**

`CONTEXT_KIT_MEMORY_DIRS` is the one that needs new wiring: it is a word-split multi-glob list with
no native reader today, and it takes the `walk::knob_array` shape the crate already uses.

**Assertion C is re-run fresh after both descriptors land**, on the rule §The first budget batch
established — a port moves a declaration path from `<kit>/checks/<name>.sh` to `<name>.gate`, which
can move *other* members into or out of the derived substrate-sensitive set, so the reading is
never inherited from an earlier cut. On the pre-port tree neither member's `couples=` reaches a
gate declaration path, so §Meta-gate conservation for the binary substrate is expected to gain no
row; the re-run is what makes that a verdict rather than an assumption.

### 9. Six stranded permission grants are the operator's prune, recorded here as a build obligation

The iteration's freshness spine deletes three emitter scripts, stranding a `Bash(bash <script>)`
grant and its `Bash(bash <script> *)` twin for each in `.claude/settings.json`, which
`check-settings-paths` reds on. **No session edits that file** —
context-kit/SPEC.md §check-settings-pins fixes the order for this class: *the operator prunes, then
the gate registers*. Owned by gate-sdk/SPEC-emitter-tail.md delta 7 and named here only because
this batch's members are the other half of the same build. **mechanical**

Probed rather than assumed: neither of this batch's own deletions strands a grant —
`.claude/settings.json` carries no entry naming `check-memory-off.sh` or `check-root-tiering.sh`.

### 10. The batch's terminal move is a demotion

`native-gate-port-remaining-corpus`'s deliverable is the whole corpus and a budget batch is one
increment of it, so the terminal move is a **demotion** back to the deferred section under
`[design-pending]`, not a Done move — the branch canon-kit/SPEC.md §Merging an amendment states
explicitly, and the move this entry has taken once per increment since 2026-08-09. **mechanical**

The second cost that branch names binds here in particular: the entry carries
`[roadmap: now/reliability]`, and a done entry is a bare slug, so a Done move would silently drop
the port from the **public** roadmap projection while the corpus is still outstanding.

### 11. The gate-sdk section lands with the batch, not after it

§The third budget batch is authored in the same iteration its members land, because a batch finding
that outlives its session is a finding nobody writes down — the failure the first two batch
sections exist to have already prevented. **design-bearing**

## Producers and consumers

**New interface: two binary subcommands.**

- *Producer* — `gates::REGISTRY` dispatch on each subcommand name, reached from the generated
  pre-commit hook and from `run-gates.sh`. Enabling config: the `GATE_SDK_KNOB_*` bridge
  `gate_command` emits for a `.gate`-declared member, over the six knobs delta 8's descriptors
  declare. This repo sets all six — `scripts/settings-pins.conf` and the context-kit defaults for
  the first, `scripts/root-allowlist.list` and the gate-sdk defaults for the second — so neither
  producer's config is test-only.
- *Consumer* — the battery. `check-root-tiering` at every pre-commit staging
  `scripts/root-allowlist.list`, plus the whole-tree `run-gates.sh` run as the backstop for a
  pure-addition commit outside the trigger's staged view (its manifest already relies on that
  backstop). `check-memory-off` at every pre-commit, on `trigger=*`, because the surfaces it guards
  never stage.

**Interface removed: `--fixture <dir>` (delta 3).** Its readers, enumerated so the removal is not
discovered by a red: `context-kit/gate-tests/check-memory-off/{good,bad}/args` and
`context-kit/gate-tests/check-memory-off.test.sh`, which are the only two callers in the tree and
both re-point onto the knobs in the same commit. No documentation outside
context-kit/SPEC.md §check-memory-off and the script's own usage line names the flag.

**New interface: the constructed-scenario parity harness (delta 4).**

- *Producer* — the porting session, once, while both implementations exist. *Consumer* — the
  session's own verdict, recorded in the commit and in the gate-sdk section delta 11 lands.
- **Its standing limit is criterion 2's own and is not softened:** it proves the two agreed *then*,
  and nothing machine-held keeps them agreeing after — which is exactly why the shell original is
  deleted rather than left running beside the port.

**New state: `check-memory-off`'s null disposition (delta 5b).** *Producer* —
`native/src/gates/memory_off.rs`, at each pin evaluation. *Consumer* — the gate's own verdict; a
null actual yields **skip**, never a violation and never a fail-closed exit. *Named reader of the
distinction* — delta 4's scenario case, which is what makes it checkable rather than asserted.

**Point 5 — this change narrows a corpus, so each reader is named by its red condition.** Two
tracked shell scripts leave the tree.

- `check-settings-paths` — reds on a literal `.sh` command token in `permissions.allow[]` that does
  not resolve. **Not monotone under a deletion.** Probed: no grant names either script, so this
  batch strands none; the six that do strand are delta 9's.
- `check-gate-fixture-coverage` — reds on a registry member with neither a fixture pair nor a
  `# no-fixture:` reason. **A coverage floor, therefore not monotone.** Both pairs are kept
  (deltas 3 and 7), and delta 3 says so explicitly because the pair is the thing a reader would
  expect to be deleted along with the arm it drove.
- `check-gate-substrate-parity` — assertion B reds when the `.gate` descriptor set and the roster
  `--list` prints are unequal. **Not monotone in either direction:** a descriptor without its
  registry module reds, and a module without its descriptor reds. This is what makes delta 1 atomic
  per member whether or not the session treats it as such.
- `check-graph` — reds on a manifest path naming nothing tracked, and on a committed hook that does
  not match its generator's emission. **Not monotone:** both declaration paths move, so the hook is
  regenerated in the same commit.
- `check-gate-binary-fresh` — reds when the committed binary's `--source-stamp` does not match the
  crate's hashed source. **Not monotone:** red from the crate edit until
  `bash gate-sdk/bin/build-native.sh` runs, which a green `cargo test` does not discharge.
- `check-docs-cmd` — assertion A reds on an invoked repo-relative `.sh` path in the governed doc set
  that does not resolve. **Not monotone:** any doc fencing either check script strands. The oracle
  is the grep at build, not a roster kept here.
- `check-readme-roster` — reds on name-set parity failure between a kit README's gate-roster block
  and the kit's `checks/` basenames, in both directions and over **both** `*.sh` and `*.gate`
  spellings. Monotone here by construction: basenames are unchanged and only the extension moves,
  which is the case that gate reads both spellings to cover. Cleared by inspection.
- `check-shellcheck`, `check-comment-tier`, `check-gate-output`, `check-gate-fail-closed`,
  `check-assertion-strength` — each reds on a **violation found**, with no count floor and no
  coverage floor. Monotone; removing two scripts can only remove violations.
- `check-install-claim` — canon-kit/SPEC.md's attested zero-count reader, and therefore the one
  probed rather than reasoned about. gate-sdk/SPEC.md's meta-gate disposition table records it as a
  **reverse-trigger** member: it names `scripts/*.sh` in `couples=` only so a script change re-runs
  it, and the corpus it scans is the governed-doc set. Re-run at build rather than cleared by
  inspection.

## Existing sections updated

- **gate-sdk/SPEC.md — a new §The third budget batch**, sibling to §The first budget batch and §The
  second budget batch and written to their rule: the findings and the cut's verdict, never a member
  roster. It carries deltas 2, 4, 5 and 6 — the criterion-2 correction, the discharge with its one
  uncomparable arm, the two semantic divergences, and the jq boundary. Owned by delta 11.
- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** — its exclusions
  paragraph, whose `check-memory-off` sentence is now a **spent** exclusion. The record of why it
  stood stays; what changes is that the discharge it named arrived. Owned by deltas 2 and 4.
- **gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency** — the *"held on
  criterion 2, named and not ported"* paragraph, and the honest-jq-claim paragraph, whose
  *"retired from the battery but for those two members"* becomes one member,
  `check-installer-no-deps`. Owned by deltas 2 and 6.
- **gate-sdk/SPEC.md §The port-candidate criteria, criterion 2** — the constructed-scenario
  paragraph gains its second worked instance, and the two are usefully different: the first is a
  member with *no* static representation, this one a member whose pair existed but did not reach
  the ported derivation. The delta-3 collapse belongs in that contrast, not only in the batch
  section. Owned by deltas 3 and 4.
- **gate-sdk/SPEC.md §check-root-tiering** — the declaration line becomes
  `checks/check-root-tiering.gate` (hermetic, `precommit`, binary-dispatched). Owned by delta 7.
- **context-kit/SPEC.md §check-memory-off** — the declaration line, the deleted `--fixture` arm and
  its usage, the retired *"fails closed … a local settings file with no `jq`"* sentence, and the
  structural-comparison and null-skip semantics, which have never been written down and are what
  delta 5 discovered by reading the shell. Owned by deltas 1, 3 and 5.
- **context-kit/SPEC.md §check-settings-pins** — its criterion-7-debt paragraph names *"`jq`
  remains required by `check-memory-off` (held on criterion 2) and `check-installer-no-deps`"*;
  after this batch only the latter. Owned by delta 6.
- **context-kit/SPEC.md's layout listing** — the `checks/` line for this member and the
  `gate-tests/` entries, which keep their pair and their `.test.sh`. Owned by deltas 1 and 3.
- **context-kit/README.md** and **gate-sdk/README.md** gate rosters — each member's annotation,
  whose declaration extension moves. Owned by delta 1.
- **TASK-QUEUE.md `freshness-emitter-port-cohort`** — the criterion-7 adjudication block, corrected
  to the criterion-2 ground with the jq re-use demoted to the dividend it is. Owned by delta 2.
- **TASK-QUEUE.md `native-gate-port-remaining-corpus`** — the increment line, at the demotion.
  Owned by delta 10.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and
      a named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section (not appended); the merged spec reads as one coherent document a
      reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for the two check-script paths and for
      `--fixture`; nothing dangles.
- [ ] **The criterion-2 discharge is real** — the pair and the `.test.sh` re-pointed onto the knobs
      (delta 3), the throwaway-`HOME` scenario run over the full matrix with both implementations
      live (delta 4), and the jq-absent arm recorded as retired rather than as proved equal.
- [ ] **The two divergences carry cases, not prose** — structural comparison and the null skip each
      pinned by a scenario case (delta 5).
- [ ] **The false premise is corrected in the queue entry**, not only in the spec (delta 2).
- [ ] **The binary rebuilt** — `bash gate-sdk/bin/build-native.sh` run in the same commit; a green
      `cargo test` does not discharge it. Assertion C re-run after both descriptors land (delta 8).
- [ ] **The entry demoted, not moved to Done** (delta 10).
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
