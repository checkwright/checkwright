# SPEC amendment: enum-sets-cut

The port disposition of **`scripts/enum-sets.sh` (50 lines)** — the one owed file behind
`canon-kit/SPEC.md §check-prose-enum` — off the shell substrate as a bridged `Arm::Emit`
member of the gate binary. Cut B of `parser-and-enum-adapter-cuts-with-graph-hotfix`,
under the port-only run (TRAJECTORY.md §PRIORITY DIRECTIVE); the unit set and this cut's
host were ruled by the **lead on 2026-09-03 on its own authority**.

**Measured at this HEAD rather than carried from the scope survey**: the port oracle's
`--tree` arm reads 107 files scanned, 64 declared `no-port`, 0 temporarily held,
**43 owed**, with `scripts/enum-sets.sh owed lines=50` among them.

**The section is a singleton, so the 2026-09-03 outer-bound ruling is not exercised
here.** `§check-prose-enum` declares exactly one owed file and the cut takes it; the
stated-contract composer's *the owed files behind one specification section* is satisfied
in its unqualified form, and nothing about a proper subset arises. Stated because the
sibling cut riding this iteration is measured against the same ruling and a reader moving
between the two amendments should not have to work out which shape each is.

**The member is not a gate**: it emits a document on stdout and returns a status, so no
`gates.list` row, no `.gate` descriptor and no `good/`+`bad/` fixture pair. The gate it
feeds — `check-prose-enum` — keeps its own pair unchanged, because the cut moves the
gate's *input producer* and not the gate.

## What changes

### (1) `--emit-enum-sets` is a bridged `Arm::Emit` member declaring **two** knobs, and they belong to two different kits

The member's contract is a document — one `<set-name>`⇥`<member>` line per member on
stdout — and every one of its failures is already exit 2, which is `Arm::Emit`'s collapse
exactly {design-bearing}, so the variant is `Emit` and the spelling `--emit-enum-sets`,
reached through the generic `--emit <name>` composer rather than a front-end branch of its
own. The declared roster is **`GATE_KIT_ROOTS_REL`** and **`QUEUE_KIT_LESSON_TAGS`** and
nothing else.

**A two-kit roster is lawful and precedented, and saying so is worth a sentence because
the bridge's does-not-define refusal makes it look risky.** `gate_knob_env_set`
partitions a declared set by `_gate_knob_owning_kit` — derived from the knob's own
`<KIT>_` prefix — and resolves each kit's slice inside that kit's own sourced subshell
(gate-sdk/SPEC.md §lib/gate.sh). `--emit-enforcement-map` already declares knobs owned by
five kits. `QUEUE_KIT_LESSON_TAGS` has its default in `queue-kit/lib/queue.sh`, so it is
declarable; `GATE_KIT_ROOTS_REL` is the roster `--emit-close-surfaces` already declares
for the same purpose.

### (2) The seam is untouched: the knob's **value** re-points, and ruling (1) narrows nothing

`CANON_KIT_ENUM_SETS_CMD` keeps its contract exactly — *a consumer command emitting the
governed sets, one `<set-name>`⇥`<member>` line per member, default empty ⇒ clean skip* —
and this repo's value moves from `bash scripts/enum-sets.sh` to
`bash gate-sdk/bin/run-gates.sh --emit enum-sets` {design-bearing}. That is
`native-gate-port-remaining-corpus`' 2026-09-03 ruling (4) in its own words: *"Porting one
moves its mechanism into the binary and re-points the value; the knob still takes any
consumer command, so no extension point narrows"* — the shape `DRIFT_KIT_KPI_DIRS` took
when drift-kit's bundled KPIs went in-crate. What the payload gains is a **bundled**
emitter an adopter can name without authoring a script; what it does not gain is any claim
on where a consumer's sets come from.

### (3) The gate keeps receiving the sets as **data**, and an in-process call is refused

`check-prose-enum` must **not** call the new emitter in process {design-bearing}. Its
sets cross the config bridge as two parallel resolved arrays — `CANON_KIT_ENUM_SET_NAMES`
and `CANON_KIT_ENUM_SET_MEMBERS`, produced by `spec_enum_sets` running the configured
command inside canon-kit's own sourced subshell — and gate-sdk/SPEC.md's port table
already records that this *"is what keeps the compiled form from spawning the emitter it
reads"*. Once the bundled emitter is in the same binary the in-process shortcut looks
free; it is not, because it would resolve the *bundled* producer for a consumer who
configured a different one, which is precisely the extension point ruling (1) protects.
The property that made the shortcut tempting — one binary holding both — is the reason to
write the refusal down rather than leave it to be re-derived.

### (4) The provenance-seam question resolves **positively**, and this is the cut's licence

Nothing in `scripts/enum-sets.sh` is consumer content {design-bearing}. The two tag sets
derive from queue-kit's own class table plus the `QUEUE_KIT_LESSON_TAGS` knob; the two
roster families derive from `gate_kit_roots_rel` and the tracked tree. Every member is a
function of a kit's own surfaces, so a bundled emitter publishes no vocabulary — it
publishes a **derivation**, and any adopter running it gets *their* tags and *their* kits.
That is exactly CLAUDE.md §The provenance seam's own test read in the favourable
direction, and gate-sdk/SPEC.md §Porting a gate to the binary substrate already recorded
the 2026-08-25 reading of this very file — *being a knob's value is half (i) of the
exemplar's cause, and the ruling declares on (b) alone*. The one consumer literal in the
file's orbit, `QUEUE_KIT_LESSON_TAGS=(essay)`, stays in `scripts/queue-config.sh` and
crosses as a knob; it is never baked.

### (5) The class-table read becomes a **direct reference**, and its fail-closed anchor retires with its cause

The shell read the tag vocabulary by parsing `native/src/gates/tag_lead_line.rs` as text —
`awk`-extracting the `CLASSES` block, stripping quotes, then stripping a trailing `]` or
`:` from each token {design-bearing}. In the crate the emitter references
`gates::tag_lead_line::CLASSES` directly. Three consequences travel together and none is
optional:

**The `_nclass == 1` guard retires, and it retires because its cause is gone.** That check
refused to run when the file held anything other than exactly one `CLASSES` table, and its
own message says why — *"anchor the read on the class table rather than on position"*. A
compiled reference cannot be ambiguous about which table it names, so the guard is not
weakened but made unstatable. Recorded as a retirement with its ground, because deleting a
fail-closed check inside a port is otherwise the exact move this project refuses.

**The terminator strip collapses onto the gate's own.** `sed -E 's/[]:]$//'` and the
gate's `&c[c.len() - 1..]` are two spellings of one fact about the table's grammar; after
the cut there is one holder, in the module that owns the table. The **read-from-the-owner
property is preserved and is the whole point** — the comment at `tag_lead_line.rs:7`
saying this table has a second reader keeps its subject and changes only the mechanism.

**The gate's own corpus rule is unchanged.** `check-prose-enum`'s `# graph:` manifest
already couples the kit sources that feed it and gate-sdk/SPEC.md's port table already
records that this gate's corpus follows the rule to `tag_lead_line.rs`. The cut adds no
coupling: the emitter is reached through the bridge as a spawned command, which is
invisible to a `# graph:` manifest in the compiled form exactly as it was in the shell one.

### (6) The two roster families keep their inputs, their tracked-only rule and both fail-closed arms

The `<kit>-lib` and `<kit>-gate-test` families are re-implemented against
`GATE_KIT_ROOTS_REL` and a **tracked** file listing {design-bearing}. Tracked-only is
contract rather than an implementation accident: `prose-uniqueness-claim-unchecked` cites
this derivation by name for the property that an *untracked* new `gate-tests/*.test.sh`
sibling does not enrol, so a walk of the filesystem would silently widen a set another
entry reasons about. The listing therefore comes from `git`, which is on
`GATE_SDK_PROGRAM_FLOOR` as the one sanctioned exception, so criterion 7 is untouched and
the crate's existing `proc::run("git", …)` sites are the precedent.

Both fail-closed arms are preserved with their asymmetry intact, because the asymmetry is
the design: **no kit roots at all**, and **a `lib/` tracking no top-level `*.sh`**, are
contradictions of the shape the derivation reads and exit 2; a `gate-tests/` holding only
`good/`+`bad/` fixture directories emits **nothing** and is a normal kit. A port that
levelled the two would either brick an ordinary adopter or convert a broken derivation
into a silently empty set, and §check-prose-enum states both readings today.

### (7) The re-point nests one bridge invocation inside another, and the cost is stated rather than discovered

`spec_enum_sets` runs the configured command **while canon-kit's library is being sourced
by the config bridge**, so after the re-point that command is itself
`bin/run-gates.sh --emit enum-sets`, which sources `lib/gate.sh` again and resolves the
arm's own two knobs before exec'ing the binary {design-bearing}. **There is no cycle** —
the arm declares no canon-kit knob, so the nested resolution cannot re-enter the one that
started it — and stating that is half the delta, because a reader meeting a bridge call
inside a bridge call will reasonably suspect one. What is real is the **cost**: one extra
`bash` process, one extra `lib/gate.sh` source and one extra binary exec on every
resolution of this gate's two knobs. §check-graph's own measurement is the precedent for
taking such a figure seriously, and the build measures it rather than asserting it is
small; the disposition if it is not is a finding to file, never a licence to take
delta (3)'s refused shortcut.

### (8) The generated hook is the port's **byte oracle**, and it is cheaper than any test written for the purpose

The resolved sets are baked verbatim into the tracked `scripts/git-hooks/pre-commit` as
`GATE_SDK_KNOB_CANON_KIT_ENUM_SET_NAMES` and `…_MEMBERS` {mechanical}. So a faithful port
regenerates that hook **byte-identical**, and `check-graph`'s hook-parity assertion is the
comparator — no golden is minted for the purpose and none should be, because the tracked
hook already is one. `docs/site-architecture.md` §Generated projections already names this
staling path, which is why the assertion is available rather than invented here. Any
difference at all — a member, an order, a set name — is a port defect until argued
otherwise, and *"the emitter now sorts differently"* is not an argument.

### (9) The prose citations re-point, and **no gate forces them** — which is why they are a delta rather than a chore

Five governed surfaces name this file and each re-points to the arm {mechanical}:
`canon-kit/SPEC.md` §Layout and configuration's `CANON_KIT_ENUM_SETS_CMD` bullet and
§check-prose-enum's consumer-config paragraph, `queue-kit/SPEC.md`'s lesson-tag
cross-reference, `gate-sdk/SPEC.md`'s port table row for `check-prose-enum`, and
`docs/site-architecture.md` §Generated projections' staling clause. The `docs/` mirrors of
each follow as a regenerated projection.

**The hazard is that a stale citation ships green.** `check-docs-cmd` assertion A resolves
invoked `.sh` paths **inside a fence only** — its inline-backtick arm scans for *knobs*,
not paths — and every one of these citations is inline backticks. So nothing reds if a
re-point is missed, and the sibling cut carries the same exposure over its own citations.
`prose-filename-citation-liveness` owns the general gap and is not re-filed. The
consequence for this cut is a Definition-of-Done item rather than a gate, and the
enumeration above exists so the sweep is a check-list rather than a recollection.
Assertion B is cleared **by inspection and by naming the holder**: `CANON_KIT_ENUM_SETS_CMD`
occurs in `canon-kit/lib/spec.sh`, which is inside that assertion's kit-roots corpus —
`scripts/canon-config.sh` is **not**, because `scripts/` is no kit root, so the config
file's own mention would not have satisfied it.

### (10) Exactly **zero** permission grants move, and the count was probed

`.claude/settings.json` carries no entry naming `scripts/enum-sets.sh` {mechanical}. It
never needed one: the emitter runs inside the config bridge rather than from a session's
shell. So the 2026-08-29 settings-grant carve-out has nothing to do here, and no grant is
added either, `Bash(bash gate-sdk/bin/run-gates.sh *)` being committed already. Recorded
because the carve-out demands the count be probed rather than assumed, and the answer
being zero is still an answer.

### (11) The owed count moves **43 → 42** on this cut alone, and the composer entry demotes

`scripts/enum-sets.sh` leaves the owed column by **deletion**, not by declaration
{mechanical}: no `# no-port:` and no `# port-until:` is written anywhere by this cut, so
nothing is subtracted from the 2026-08-28 completion predicate. The sibling parser cut
takes two more files off the same total independently; neither amendment may claim the
other's, and the iteration's arithmetic is 43 → 40 only when both land.
`native-gate-port-remaining-corpus` **demotes** at build rather than reaching `## Done`,
its own text saying so, and returns to the position this promotion took it from.

## Producers and consumers

The cut introduces **no new state, event or field**. It introduces one new **interface** —
the `--emit-enum-sets` arm — and relocates one existing one, the emitter, from a consumer
script into the binary. The emitted grammar, the set names, the member spellings, the two
fail-closed arms and both exit statuses are unchanged, so the checklist runs over the
relocation and over the deletion.

- **Producer.** `--emit-enum-sets`, dispatched in `main` ahead of the registry lookup and
  absent from `--list`, reached through `bin/run-gates.sh --emit enum-sets`. Its enabling
  configuration is delta (1)'s two bridged knobs, both of which a deployed configuration
  actually sets: `GATE_KIT_ROOTS_REL` is resolved unconditionally by `gate-sdk/lib/gate.sh`
  and `QUEUE_KIT_LESSON_TAGS` is defaulted unconditionally by `queue-kit/lib/queue.sh` and
  set by this repo's `scripts/queue-config.sh`. Point 1 of the causal-completeness check is
  satisfied by the libraries rather than by a consumer's diligence.
- **Consumer, and there is exactly one.** `spec_enum_sets` in `canon-kit/lib/spec.sh`,
  which runs the configured command at library-source time under `_spec_resolving`,
  validates every line against the two-field grammar and fills
  `CANON_KIT_ENUM_SET_NAMES` / `CANON_KIT_ENUM_SET_MEMBERS`. Those two arrays are read by
  `check-prose-enum` at its scan transition — the set name into the report, the member into
  the matcher — which §check-prose-enum already states and which this cut does not change.
- **A second consumer exists and is a *file*, not a process**: the generated pre-commit
  hook, which bakes the resolved arrays verbatim. Named because delta (8) makes it the
  port's oracle, and because a reader would otherwise not count a tracked artifact as a
  consumer at all.
- **No new field.** Both fields of each emitted line keep the readers they have.

**Every reader's RED condition, because this delta narrows a corpus.**
canon-kit/SPEC.md §The causal-completeness check point 5 binds — a reader is clearable by
inspection only where its verdict is monotone in the violation set, and reds-on-finding-none,
exact-count and coverage-floor shapes are not.

- **`check-prose-enum` itself is the non-monotone reader and the one that matters.** Its
  verdict is a **set difference**: a paragraph hand-listing two or more members of a
  declared set must name them all. So the sets *growing* reds prose that was clean and the
  sets *shrinking* silently stops asking about a member — neither direction is safe by
  inspection, and delta (8)'s byte comparison is what stands in for an inspection that
  cannot be done. A command that fails or a line that does not parse is **exit 2**, so an
  arm that cannot run is loud rather than empty; that is the fail-closed property the port
  must preserve exactly, and the one an accidental `|| true` would silently convert into a
  clean skip.
- **`check-graph`** — reds when the committed hook diverges from the generator's `--emit`
  output. An **equality**, therefore not monotone, and it fires on this cut by
  construction because the baked argv contains the sets. This is signal: it is what forces
  the regeneration, and its being green afterwards is delta (8)'s whole assertion.
- **`check-docs-cmd` assertion A** — reds on a **fenced** invoked `.sh` path that does not
  resolve. Its red condition is *finding an unresolvable path*, monotone, and it is
  **cleared vacuously here** because none of this file's citations is fenced. That vacuity
  is the hazard delta (9) records, not a discharge.
- **`check-docs-cmd` assertion B** — a **zero-count** reader over each kit-prefixed knob
  name, so it is one of the three non-monotone shapes by name. Cleared by inspection only
  because the in-corpus holder is named: `CANON_KIT_ENUM_SETS_CMD` in
  `canon-kit/lib/spec.sh`. `scripts/` is not a kit root and contributes nothing to it.
- **`check-settings-paths`** — reds on an allow-list entry naming a path that no longer
  resolves. Cleared because delta (10) probed the count and it is zero.
- **`check-shellcheck`, `check-comment-tier`, `check-exec-bit`** — monotone in the
  removing direction; deleting a file removes findings, and the crate module's `// spec:`
  headers carry the deleted script's comment bindings rather than dropping them.
- **`check-gate-substrate-parity` assertion B** — an **equality** between the `.gate`
  descriptor set and `--list`'s roster, which a non-gate arm stays outside by construction;
  that is what §The non-gate arm's first property buys.
- **`check-gate-fixture-coverage`** — reds on a registered gate with no `good/`+`bad/`
  pair. Untouched: no gate is added or removed.
- **The port oracle `--emit port-blockers`** — reports rather than reds; `--tree` owed
  moves 43 → 42 on this cut alone.
- The build re-runs the full battery rather than resting on this enumeration, which is the
  enumeration's purpose: it says where to look when one goes red.

## Existing sections updated

- `canon-kit/SPEC.md §check-prose-enum` — the consumer-config paragraph names the arm
  rather than the script; the class-table read is a direct reference with the retired
  `_nclass` anchor and its ground recorded; the two roster families keep their tracked-only
  rule and their asymmetric fail-closed arms, restated for the compiled holder; and the
  refusal of an in-process call is written down where the gate is specified rather than
  where the arm is (deltas 3, 4, 5 and 6).
- `canon-kit/SPEC.md §Layout and configuration` — the `CANON_KIT_ENUM_SETS_CMD` bullet's
  *this repo sets* clause names the arm; the knob's own contract is unchanged, and saying
  so is what stops a reader taking the re-point for a narrowing (delta 2).
- `canon-kit/SPEC.md §lib/spec.sh` — `spec_enum_sets` is unchanged in contract and gains a
  named nesting property: the command it runs may itself be a bridged arm, which is a
  bridge invocation inside the bridge and provably acyclic for this member (delta 7).
- `gate-sdk/SPEC.md §The non-gate arm` — the `--emit-` family roster gains
  `--emit-enum-sets` as a 2026-09-03 member, and the paragraph on declared rosters gains
  this member as a worked instance of a **two-kit** roster resolved by the partitioning
  bridge (delta 1).
- `gate-sdk/SPEC.md §Porting a gate to the binary substrate` — the port table's
  `check-prose-enum` row is corrected: its derivation no longer *reads the module as text*
  but references the table, and the sentence about the derivation crossing the bridge as
  data gains the reason it must keep doing so (deltas 3 and 5).
- `queue-kit/SPEC.md` §The Lessons Learned channel — the cross-reference naming this
  repo's emitter re-points at the arm; the `QUEUE_KIT_LESSON_TAGS` contract is untouched
  and the knob stays the consumer's (deltas 2 and 4).
- `native/src/gates/tag_lead_line.rs` — the `CLASSES` table's second-reader comment keeps
  its subject and changes its mechanism, from a text parse to a module reference (delta 5).
- `scripts/canon-config.sh` — one knob value re-pointed, in the commit that deletes its
  target (delta 2).
- `docs/site-architecture.md` §Generated projections — the hook-staling clause names the
  arm rather than the script; the clause's substance is unchanged and is what makes
  delta (8)'s oracle available (deltas 8 and 9).
- `TASK-QUEUE.md`, the `native-gate-port-remaining-corpus` entry — gains this amendment's
  `[spec:]` ref on its lead line, which the arithmetic below prices, and **demotes** at
  build rather than reaching `## Done` (delta 11).

<!-- update-target-exempt: the composer entry takes no body write from a cut by its own 2026-08-28 ruling; each closed cut's record lands in the contract section the cut selected, which for this cut is canon-kit/SPEC.md §check-prose-enum above -->
- `TASK-QUEUE.md`, `native-gate-port-remaining-corpus`'s body — deliberately unwritten.

<!-- update-target-exempt: cited by delta 9 as the owner of the general gap that no gate resolves a backticked dead path; this cut neither closes it nor changes it, and re-filing the finding against it would be the duplicate its own text exists to prevent -->
- `TASK-QUEUE.md`, `prose-filename-citation-liveness` — deliberately untouched.

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
- [ ] **The two emitters agree line-for-line before the delete** — the shell form and the
      arm run on this tree and their output diffed empty, in the session that deletes the
      script, because after the delete one holder cannot be compared with itself.
- [ ] **The regenerated pre-commit hook is byte-identical**, and the diff is read rather
      than the gate's verdict alone — a green `check-graph` after a regeneration proves the
      hook matches the *current* emitter, not that the emitter is unchanged.
- [ ] **Both fail-closed arms exercised on the compiled holder** — no kit roots, and a
      `lib/` tracking no top-level `*.sh`, each exit 2; and a `gate-tests/` holding only
      fixture directories emits nothing and does **not**.
- [ ] **The tracked-only rule proved**, with an untracked `gate-tests/*.test.sh` sibling
      absent from the emitted set, since that property is one another queue entry reasons
      about.
- [ ] **The nested-bridge cost measured**, not asserted: this gate's knob-resolution time
      before and after, on the same tree.
- [ ] **The five prose citations swept by the list in delta (9)**, because no gate forces
      any of them and a missed one ships green.
- [ ] **The `--tree` owed count re-read to confirm 43 → 42 on this cut**, with no
      `# no-port:` and no `# port-until:` written anywhere by it.
