# SPEC amendment: emitter-tail

**The freshness family's last three emitters, applying a merged ruling rather than making one.**
gate-sdk/SPEC.md §The non-gate arm already answers *what a ported emitter is*, and
§SPEC-emitter-substrate applied that answer to the first three. This amendment applies it
unchanged to `scripts/gen-docs-mirror.sh` (127 lines), `drift-kit/bin/trajectory.sh` (242) and
`queue-kit/bin/roadmap.sh` (76), closing the six-member family 6/6 and taking 445 lines of shell
out of the tree. Nothing below re-opens the class contract; what is genuinely new here is the
seam holding on three consumer vocabularies, one shared adapter that carries a stated
never-disagree guarantee across the substrate boundary, and a corpus narrowing whose reader
verdict is not monotone.

**The cohort is the operator's, relayed through scope's 2026-08-18 cut on
`freshness-emitter-port-cohort`**, and it is not re-argued here.

## What changes

### 1. Three emitters become members of the crate's emitter table

`scripts/gen-docs-mirror.sh` becomes the `docs-mirror` member, `drift-kit/bin/trajectory.sh` the
`trajectory` member and `queue-kit/bin/roadmap.sh` the `roadmap` member of
`native/src/emit/mod.rs`'s `EMITTERS` table, each a `(name, EmitFn, knob-roster)` tuple whose
`--emit-<name>` spelling is derived by `arm_name` rather than written a second time; all three
resolve in `main` ahead of the registry lookup and stay absent from `--list`. **design-bearing**

**Table membership is forced rather than chosen, and the alternative is the failure mode that
looks like success.** §The non-gate arm rules that an arm receives no configuration and that only
an emitter-table member is bridged — the table's knob column is what `--knobs` prints and what the
battery runner's `--emit` front-end resolves. All three of these emitters read consumer knobs, so
a hardcoded top-level flag would resolve platform defaults and silently ignore every consumer
override. There is no calibration here between two workable shapes.

The knob rosters the table declares, read through `walk::knob_array` / `walk::knob_scalar`:

- `docs-mirror` — `DOCS_MIRROR_BLOB_BASE` (derived from `CANON_KIT_DOCS_BLOB_REF`).
- `trajectory` — `DRIFT_KIT_CONFIG_FILE`, `DRIFT_KIT_TRAJECTORY_SURFACES`, `DRIFT_KIT_GATES_FILE`,
  `DRIFT_KIT_STAGES`, `GATE_SDK_WORKFLOW_DIR`.
- `roadmap` — `QUEUE_KIT_QUEUE_FILE`, `QUEUE_KIT_HORIZONS`, `QUEUE_KIT_TRACKS`,
  `QUEUE_KIT_ROADMAP_FILE`, `QUEUE_KIT_ROADMAP_MARKER`.

`QUEUE_KIT_TRACKS` is on that roster although today's `roadmap.sh` only prints the track field
verbatim and never validates it: the arm's caller for assertion B is the ported
`check-roadmap-fresh`, which does validate it, and a knob the bridge does not carry is a knob the
gate cannot read (queue-kit/SPEC-roadmap-port.md owns that gate).

### 2. The seam holds on three consumer vocabularies, and the crate carries none of them

The arms ship the projection **grammar** and no lane, stage or surface name. **design-bearing**

- **`roadmap`.** `QUEUE_KIT_HORIZONS` and `QUEUE_KIT_TRACKS` are this consumer's editorial
  vocabulary (`scripts/queue-config.sh`), and queue-kit/SPEC.md §The tag algebra already rules why
  they are configured arrays rather than kit literals: *a kit literal spelling one project's
  horizons would ship its roadmap posture as everyone's*. The crate emits one `### <horizon>`
  heading per configured horizon **in the knob's own order** — the order is the array's, never a
  sort the crate imposes — the bullet shape ``- **`<slug>`** *(<track>)* — <summary>``, the
  empty-horizon placeholder, and the trailing blank line queue-kit/SPEC.md §bin/roadmap.sh rules
  load-bearing. Not one lane name enters `native/`.
- **`trajectory`.** `DRIFT_KIT_STAGES` is this consumer's lifecycle stage vocabulary and
  `DRIFT_KIT_TRAJECTORY_SURFACES` its evidence-file roster; both are the same class and take the
  same treatment.
- **`docs-mirror`.** The source set is derived from the tracked tree, not enumerated, so the only
  configured value is the blob base — a URL prefix, not a vocabulary.

The reading that licenses the port at all is the one §The settings cohort already ruled for the
JSON reader: **a general-purpose renderer is grammar, not vocabulary**, so it can no more carry a
project term than `ere.rs` could. What the seam forbids is a *consumer-shaped* literal, and the
knob column is what keeps every one of these on the caller's side of the bridge.

### 3. `queue_roadmap_entries` becomes one crate function, and the guarantee survives with it

`queue-kit/lib/queue.sh`'s awk adapter becomes `native/src/queue.rs`'s `roadmap_entries`, and the
shell form is deleted with its two callers. **design-bearing**

queue-kit/SPEC.md §lib/queue.sh deliberately excluded this adapter from the shell/Rust split *for
the opposite reason* from `queue_live_slugs`: its only two consumers are the emitter and the gate,
so while both sit on one shell adapter the two cannot disagree about what an entry claims. That
exclusion is **satisfied on the other substrate, not repealed** — after this port both consumers
share one crate function, and the guarantee holds by the identical argument. What would spend it
is porting exactly one of the two, which is why delta 11 exists.

**The record stops being a TSV line, and that is a deletion rather than a translation.** The
`<tag-count> <raw-field> <slug> <declaration-count> <summary>` tab-separated line existed because
a shell function's only return channel is stdout. In-crate the two callers take a typed record
carrying the same five fields, so the field-count and embedded-tab hazards of the wire form go
with it. The five fields and their meanings are queue-kit/SPEC.md §lib/queue.sh's and are not
restated here.

### 4. Two GNU-only invocations leave the tree, and the dividend this port must not claim

Criterion 7 clears all three members already — `realpath` and `date` are both on
`GATE_SDK_PROGRAM_FLOOR` — so **this port retires no program from the battery's dependency
floor**, and a criterion-7 dividend claim would be false. What it does retire is two **GNU-only
invocations**: `realpath -m --relative-to` in `gen-docs-mirror.sh` and `date -d` in
`trajectory.sh`, both named in the bash-portability costing §The decisions this substrate already
closed rejected. Relative-path arithmetic becomes crate path arithmetic; `date -d` converts git's
own `--date=short` output for a day-difference, so it becomes a civil-date-to-day-count helper
needing no dependency. That is a reach win against TRAJECTORY.md's objectives, and it is stated as
that rather than dressed as a floor reduction. **design-bearing**

`trajectory.sh`'s git reads (`rev-parse`, `cat-file -e`, `log -p -U0`, `log --format`, `show`) port
as `proc::run("git", …)`, the shape `fresh.rs` and `spec.rs` already use; git is the one sanctioned
floor exception and stays one.

### 5. The two already-native comparators stop spawning their emitters

`native/src/gates/docs_mirror_fresh.rs` and `trajectory_fresh.rs` call `emit::docs_mirror::emit` /
`emit::trajectory::emit` in-process, replacing the `fresh::emit` path that shells `bash` at the
emitter. **design-bearing**

This is where the family's `bash` hop is actually retired for those two members rather than
relocated — the distinction §The first cohort insists on when it records their banked win as
**zero** while their emitters stayed shell. Both move off zero here.

### 6. Both comparator descriptors acquire the coupling the port creates, one of them from behind

§The non-gate arm rules that a gate reaching an arm in-process owes its `.gate` descriptor every
crate module it reaches transitively, including a module shared by both sides of a compare, because
the generated hook's `staged_matches` trigger is derived from `couples=`. Today
`scripts/check-docs-mirror-fresh.gate` *triggers* on `scripts/gen-docs-mirror.sh` but does not
couple it, and `scripts/check-trajectory-fresh.gate` names `drift-kit/bin/trajectory.sh` in neither
field — so trajectory's descriptor is starting from a coupling it never had. Both descriptors gain
the emitter module paths in `couples=` and drop the now-dead script triggers. **design-bearing**

Omitting a module leaves the gate registered and green while the projection it holds goes stale at
commit time; only a full battery finds it.

### 7. Six permission grants strand, and the prune is the operator's rather than this unit's

`.claude/settings.json` carries a `Bash(bash <script>)` grant and its `Bash(bash <script> *)` twin
for each of the three emitters. `check-settings-paths` reds on a literal `.sh` command token that
does not resolve in the working tree, and context-kit/SPEC.md §check-settings-paths puts the
`*`-suffixed twin explicitly *in* scope on the ground that its path is as literal and as strandable
as the bare form's. So the deletion strands six grants and reddens that gate. **mechanical**

**No session edits the file.** The settings file is operator-owned configuration, and
context-kit/SPEC.md §check-settings-pins already fixes the landing order for exactly this class:
*the operator prunes, then the gate registers*. This delta is therefore a **build-time obligation
recorded with its owning gate**, not a sweep a batch performs on its own authority — build meets a
known red and routes the prune to the operator rather than discovering it as a surprise.

This is the amendment's non-monotone narrowing and it is called out under §Producers and consumers
point 5 rather than assumed away: deleting files **adds** violations here.

### 8. Every regen command, roster line and citation of the three scripts moves

Derived at build from `grep -rn --exclude-dir=.git 'gen-docs-mirror\.sh\|bin/trajectory\.sh\|bin/roadmap\.sh' .`
rather than from a roster maintained here, which would be a second copy to drift. The members that
are **not** obvious from that run and are named because a sweep can skip them:
`docs/site-architecture.md` §Generated projections carries all three regen commands (three
distinct sites), `.claude/commands/close.md` carries the trajectory regen in a stage step,
`scripts/drift-config.sh`'s shellcheck-disable comment names its consumer by path, and
`docs/` holds a mirrored copy of every kit SPEC that regenerates rather than being edited.
**mechanical**

### 9. Two kits lose a `bin/` tool, and the surfaces that ran it rewire onto the front-end

`drift-kit/smoke/install.sh` runs the *vendored* kit's `bin/trajectory.sh --emit`, and
`queue-kit/gate-tests/roadmap.test.sh` runs `bin/roadmap.sh` out of the kit under test; both move
onto the binary's arm through the battery runner's `--emit` front-end, which is the entry point
that supplies the bridged environment. **design-bearing**

**Criterion 5's consequence is recorded, not re-argued.** A kit that shipped a runnable shell tool
now ships one reachable only through the binary, so on a host the target roster carries no
artifact for, the tool is omitted rather than degraded — the same trade the first three members
took. The accumulating cost of that trade is `born-native-omission-accumulation`'s and is not
re-litigated here.

### 10. A transition-scoped parity run precedes each deletion

Byte-identical output between `bash <emitter> --emit` and the arm, over the live tree, while both
implementations exist — the discipline the delivered triple held, and the only oracle these three
have. **mechanical**

**`check-roadmap-fresh`'s fixture pair may not be cited as that proof.** §The first cohort records
that both its `good/` and `bad/` cases pass pre-baked files that steer assertion A off the live
emitter, so a ported pair goes green over an arm with no implementation at all. The parity run is
the proof; the pair is not.

### 11. `--emit-roadmap` and `check-roadmap-fresh`'s port land in one commit

`check-roadmap-fresh` is still shell and spawns `bash queue-kit/bin/roadmap.sh --emit`. Deleting
the script without the gate's port breaks the battery; porting the gate without the arm duplicates
`queue_roadmap_entries` across two substrates with nothing machine-held, which is precisely what
delta 3's guarantee forbids. **A batch cut that separates them yields a broken tree, not a
schedule.** The gate's half is `cohort-held-members-port-prerequisites`' increment, specified at
queue-kit/SPEC-roadmap-port.md, and this constraint is stated on both amendments so neither can be
read alone and get it wrong. **design-bearing**

The other two members carry no such pairing: both their comparators are already native.

### 12. The family closes 6/6 and the accounting moves with it

§The first cohort's per-member cost table, its *"the honest number for those three is therefore
zero"* paragraph and its *"across the whole family the count is one"* paragraph are all live counts
that this port moves. They are updated in place rather than appended to. **mechanical**

## Producers and consumers

**New interface: three emitter-table members.**

- *Producer* — `native/src/emit/mod.rs`'s `EMITTERS` table, reached by `emit::lookup` in `main`
  before the registry lookup. Enabling config: the knob rosters in delta 1, bridged by the battery
  runner's `--emit` front-end and by `gate_command` for the descriptor-declared comparators. This
  repo sets every one of them in `scripts/queue-config.sh`, `scripts/drift-config.sh` and the
  gate-sdk defaults, so no member is test-only.
- *Consumers*, per member, each with the transition where it is read:
  - `docs-mirror` — `check-docs-mirror-fresh` (in-process, at its freshness compare) and the
    `docs/site-architecture.md` regen command (a maintainer step after any kit SPEC or README edit).
  - `trajectory` — `check-trajectory-fresh` (in-process, at its freshness compare) and the close
    stage's regen step in `.claude/commands/close.md`.
  - `roadmap` — `check-roadmap-fresh` (in-process, at assertion A) and the `--write` regen path
    that splices `ROADMAP.md`'s marker block through `marker::write_block`.

**New interface: `native/src/queue.rs::roadmap_entries`.**

- *Producer* — called by the `roadmap` arm when it renders, and by `check-roadmap-fresh` when it
  scans the queue for assertions B and C.
- *Consumer and named reader per field* — the five fields are the shell adapter's and change
  meaning nowhere. Tag count and raw field are read **only** by the gate, at assertion B
  (field validity) and assertion C (marking parity); slug, track and summary are read **only** by
  the arm, at render. No field is added, so none is unread; the split is stated because a field
  read at one caller and not the other must not be populated at the other's expense.

**New interface: the civil-date helper and the relative-path helper (delta 4).**

- *Producer* — `native/src/emit/trajectory.rs` and `native/src/emit/docs_mirror.rs` respectively,
  at render. *Consumer* — the rendered projection; no other caller. Both are internal helpers with
  exactly one caller each at landing, which is stated so a later reader does not take them for
  general-purpose crate API.

**Point 5 — this change narrows a corpus, so each reader is named by its red condition rather than
its subject.** Three tracked shell scripts leave the tree, narrowing every shell-scanning corpus.

- `check-settings-paths` — reds on a literal `.sh` command token in `permissions.allow[]` that does
  **not resolve**. **Not monotone under this narrowing: the deletion adds six violations.** Delta 7
  records them as a build-time obligation routed to the operator, since no session edits that file.
- `check-docs-cmd` — assertion A reds on an invoked repo-relative `.sh` path inside a fence or
  inline backticks in the governed doc set that does **not resolve to a tracked file**. **Not
  monotone: the deletion strands every `bash <emitter>` line in the READMEs, the kit SPECs and
  `docs/site-architecture.md`.** gate-sdk/SPEC.md's own disposition table already names this the
  correct-not-vacuous red a port produces. Delta 8 covers it, and its oracle is the grep rather
  than a roster. `check-readme-roster` is **not** in this set, probed rather than assumed: its
  parity is over a kit's `checks/` basenames, not its `bin/` tools.
- `check-graph` — reds on a `couples=`/`trigger=` path that names nothing tracked. **Not monotone:
  the deletion strands `scripts/check-docs-mirror-fresh.gate`'s trigger.** Delta 6 covers it.
- `check-md-refs` / `check-spec-pointer` — red on a citation resolving to no file or no section.
  **Not monotone: the deletion strands every prose citation of the three paths.** Delta 8 covers
  them, on the same oracle.
- `check-shellcheck`, `check-comment-tier`, `check-gate-output`, `check-assertion-strength` — each
  reds on a **violation found** in the files it scans, with no count floor and no coverage floor.
  Monotone, therefore clearable by inspection: removing three files can only remove violations.
- `check-footprint-fresh` and `check-value-rollup-fresh` — red on a **stale byte-compare**, not on
  a count. The measured shell-line totals change, so both projections regenerate in the same
  commit; the verdict is monotone in neither direction and is cleared by regeneration, never by
  inspection.
- `check-install-claim` — the attested zero-count reader canon-kit/SPEC.md names, and therefore the
  one this narrowing must be probed against rather than reasoned about. gate-sdk/SPEC.md's
  meta-gate disposition table records it as a **reverse-trigger** member: it names `scripts/*.sh`
  in `couples=` only so a script change re-runs it, and the corpus it actually scans is the
  governed-doc set. Build re-runs it after the deletion rather than clearing it by inspection.

## Existing sections updated

- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** — the per-member cost
  table's three remaining rows, the zero-banked-win paragraph, the count-is-one paragraph, and the
  *"the emitters were filed, not adopted"* paragraph, which this port draws down to nothing.
  Owned by delta 12.
- **gate-sdk/SPEC.md §The non-gate arm** — its closing sentence, *"every remaining member of the
  freshness family acquires it the moment its emitter lands in the crate beside it"*, becomes a
  discharged statement rather than a standing one. Owned by delta 6.
- **queue-kit/SPEC.md §bin/roadmap.sh** — becomes the roadmap arm's section: the emit grammar,
  the marker splice and the honest lead-line limit are unchanged and move as they are; the
  `bash bin/roadmap.sh --emit|--write` interface becomes the arm and the front-end. Owned by
  deltas 1 and 2.
- **queue-kit/SPEC.md §lib/queue.sh** — the `queue_roadmap_entries` paragraph and its
  deliberate-exclusion reasoning, which delta 3 satisfies on the other substrate rather than
  repeals. Owned by delta 3.
- **drift-kit/SPEC.md §The published-evidence extractor** and its layout listing of
  `bin/trajectory.sh` — the extractor becomes an arm. Owned by deltas 1 and 8.
- **docs/site-architecture.md §Generated projections** — all three regen commands. Owned by
  delta 8.
- **drift-kit/README.md**, **queue-kit/README.md**, **README.md**'s queue-kit row — tool rosters
  naming the two `bin/` scripts and the roadmap projector. Owned by delta 8.
- **queue-kit/SPEC.md §check-roadmap-fresh** is **not** this amendment's to update:
  queue-kit/SPEC-roadmap-port.md owns it, and the boundary is stated so neither amendment leaves
  it orphaned at merge.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and
      a named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section (not appended); the merged spec reads as one coherent document a
      reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for the three script paths and the
      `queue_roadmap_entries` shell adapter; nothing dangles.
- [ ] **Parity proved before deletion** — each emitter's arm byte-matches `bash <emitter> --emit`
      over the live tree while both exist, and no member's fixture pair is cited as that proof.
- [ ] **The roadmap pair landed atomically** — `--emit-roadmap` and `check-roadmap-fresh`'s port in
      one commit (delta 11).
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
