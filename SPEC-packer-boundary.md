# SPEC amendment: packer-boundary

**`scripts/pack-installer.sh` gains an owning section with a port disposition, and the 196 lines
behind it are cut to the binary as a non-gate arm.** The two halves are one unit because the
first is the second's precondition — no cut can be authored against a boundary that does not
exist — and they ride one iteration on the operator's purposive reading of the port-first run's
ground 1, which answers the ambiguity that the missing section *is* the defect and so the
"owning section's own sequencing sentence" test is unsatisfiable rather than merely unmet.

**They are separable at batch-cut, and the delta labels say so.** Deltas 1 and 2 stand alone and
leave the tree consistent; delta 3 is the cut and depends on them. A lead cutting batches may
take the boundary first and the cut second, which is the shape the sibling re-home-then-port
precedent took.

## What changes

### (1) The packer gains an owning section in `installer/README.md`, ruled on the provenance seam

`scripts/pack-installer.sh` carries `# spec: CLAUDE.md §Housekeeping`, an always-loaded manifest
that the standing ruling bars as a cut boundary, and no section of any SPEC or README claims the
file {design-bearing}. Two candidate homes were named, and they are decided on what each already
says about itself rather than on which is nearer.

**`gate-sdk/SPEC.md` §Consumer payload is the near candidate and is refused. The primary ground
is the provenance seam, not a filing preference.** gate-sdk is a **kit**, vendored into every
adopter's tree. A kit SPEC section that governs the disposition of a file living in this repo's
`scripts/` — a file no adopter receives, and whose existence no adopter can verify — inverts the
kit/consumer layering and publishes a rule about a private tool as kit mechanism. That is the
seam's own subject: a kit ships generic mechanism, and a consumer's release assembler is not
generic mechanism however precisely the section could describe it.

**The second ground is the section's own reach statement, and it is independent of the first.**
§Consumer payload opens by declaring exactly what it rules — what a gate on the binary substrate
*discloses* to the consumer it judges — and then bounds itself: *"Its reach is exactly that: it
rules what a gate ships."* A port disposition for a release assembler is not a disclosure rule,
and the four places the section already names the packer are rules about payload **content**
that happen to name their producer, never dispositions of the file. Already describing a file is
not owning it. Either ground alone refuses the placement; they are stated as two because the
first would still bind if the section's reach were written differently.

**The section lands in `installer/README.md`, and the decisive evidence is that this surface
already hosts exactly this shape.** §The consumer smoke carries the port disposition of
`installer/consumer-smoke/run-smoke.sh` — a repo-private maintainer tool that rides no payload
and that no adopter receives — and it establishes that tool's non-shipping status *by citing the
packer*: the packer assembles both transports out of the kit roots and never out of the smoke's
own directory. The two files are one family: a repo-private tool that assembles or exercises the
payload, governed from the surface that owns the payload's shape. The packer is the sibling of
the smoke, and its disposition belongs beside it.

**`installer/README.md` is also the right governance class.** It is repo-root-governed with no
owning kit, which is the class `scripts/pack-installer.sh` is in; the section therefore neither
widens a kit's subject nor publishes a private rule as kit mechanism.

**Ruled `lead, own-authority` 2026-09-06**, on the escalating session's recommendation and after
the lead independently re-read all three of its supporting citations in the tree rather than
accepting them, with the seam ground recorded as the primary one.

### (2) The section states the sequencing sentence, and the sentence is that nothing sequences it

The new section carries the disposition ground 1's test asks for {design-bearing}, and the
substantive finding is a negative one: **the packer is held behind nothing.**

- It carries **no bootstrap step**. §The install boundary's three dispositions apply to the
  steps of an install, and the packer runs no install — it assembles the artifact an install
  later consumes.
- It **ships to no adopter**. `pack_tracked` packs `installer/` and the kit roots; `scripts/` is
  neither, so the file is absent from both transports. This is what §The install boundary's
  behind-invoke hold does not reach, and it is why the packer's 196 lines sit in the reachable
  column while the ten `installer/` files beside them do not.
- Its callers are a **release path and a test harness**, both of which already build the binary
  before they reach it.

So the section's sequencing sentence is that the packer is a plain port obligation with no
precondition outstanding — the disposition the missing section would have carried all along, and
the sentence this entry's own *"no cut can take this file until a section exists"* was standing
in for.

**The section also fixes the pointer scatter, which is the defect's other half.** The file's nine
inner `# spec:` pointers name three surfaces: four genuinely point at payload-content rules
`gate-sdk/SPEC.md` §Consumer payload owns and **stay**; two point at `installer/README.md` §The
consumer smoke and are re-homed into the new section, which is the better owner for the packer's
own flag contract; and three point at the barred always-loaded manifest and are re-homed into the
new section, which is the whole of what gives them a legal target. The adjudication is per
pointer rather than wholesale, because a blanket re-home would move the four that are already
correctly placed.

### (3) The 196 lines are cut to the binary as a non-gate arm with two named callers

The packer ports as a **non-gate arm** and not a gate {design-bearing}: it returns a document
and an assembled artifact rather than a verdict a battery reads, so it owes no `.gate`
descriptor, no `gates.list` registration and no `good/`+`bad/` fixture pair. What it owes
instead is a **named caller**, and it has two real ones rather than a nominal one — the release
publish path, and `installer/consumer-smoke/run-smoke.sh`, which calls it at every one of its
pack call sites with `--root`.

**Its flag spelling is not a free choice.** A non-gate arm is a top-level `--`-prefixed flag
resolved before the registry lookup and absent from `--list`, and the front-end composes
`--emit-<name>` from its `--emit <name>` operand — so a member spelled anything else is
reachable by no shipped front-end. The arm is spelled to that grammar or it is callable only
against the binary directly, and staying outside `--list` is what keeps
`check-gate-substrate-parity` assertion B's descriptor-set equality true in both directions.

**The four-flag surface is preserved exactly**, `--help` included: the file's own `--help` arm is
the single tier for its flag roster, and §The consumer smoke rules in terms that no doc carries a
second copy to drift. A port that dropped or relocated that roster would create the second copy
the rule refuses.

**The refusal behaviours are the contract, not incidental.** The packer exits 2 on a dirty tree,
a non-toplevel `--root`, a missing tool, an unresolvable version, a roster target with no
artifact directory, a sidecar mismatch, an empty kit set, and a tracked symlink under a packed
root. Every one of those is a stated rule in a governed surface, and the cut preserves each exit
status and each cause message rather than re-deriving them — `pack_tracked`'s symlink pre-flight
in particular, whose whole point is that it writes nothing and names the cause before the
pipeline starts.

## Producers and consumers

**The new section.** *Producer* — this amendment, landing it in `installer/README.md`.
*Consumers* — three, and one is mechanical: **`check-spec-pointer`**, which resolves the
`# spec:` pointers deltas 1 and 2 re-home and reds on one that resolves to nothing; the port
oracle's per-file adjudication, which reads the section's sequencing sentence as ground 1's
test; and a session reading the file's header to learn what governs it.

**The ported arm.** *Producer* — `native/`'s multi-call binary, dispatching the new subcommand
before the registry lookup. *Consumers* — the two named callers above, at the transition where
each invokes the pack. **The enabling precondition is that the binary is reachable wherever the
arm runs, and it is checked rather than assumed:** both callers build the binary before reaching
the pack step today, so the arm's reachability is not a new obligation for either. **The one
place this interacts with a sibling unit of this iteration** is the macOS install-smoke leg — a
new host on which the smoke, and therefore the packer, runs. The smoke builds the binary from the
host it runs on before it packs, so the leg inherits the reachability rather than owing it; this
is stated because a new leg is exactly where an unstated precondition would first fail, and
because the two units land in one iteration.

**No new field, no new message, no new state file, no new knob.** `INSTALLER_PACK_TMP_DIR`,
`GATE_SDK_ROOT` and the target-roster knobs are read at their existing contracts and gain no
meaning here. `--version`, `--out`, `--artifacts` and `--root` keep their spellings, their
defaults and their refusal causes, so no caller changes.

**This delta set narrows two corpora — the shell tree by 196 lines, and the set of surfaces the
packer's `# spec:` pointers name, from three to two — so the causal-completeness check's point 5
binds and each reader's RED condition is enumerated rather than its subject.**

- **`check-spec-pointer`** reds on **finding** a `# spec:` pointer that resolves to nothing. The
  re-home in delta 2 both removes and adds pointers, so it is non-monotone in both directions and
  is run. It grades only that the target *resolves* and not that the target is a legal cut
  boundary — the hole `spec-pointer-boundary-legality` owns as a separately filed unit, which
  this amendment does not close and does not claim to.
- **`check-gate-substrate-parity`** holds the descriptor set equal to the `--list` roster
  (assertion B) and refuses an implementation source inside a vendoring kit root (assertion E).
  A new arm placed inside `--list` reds B; `scripts/` is not a kit root, so E is untouched. Run,
  because a new subcommand is exactly the shape that can breach B.
- **`check-gate-binary-fresh`** reds on the binary being stale against its source. The cut moves
  source into the crate, so it stales by construction and clears only by
  `bash gate-sdk/bin/build-native.sh` — the commit-time obligation the battery does not
  discharge.
- **`check-port-blockers`'s `--tree` arm** reads an owed count. This delta set **removes** 196
  lines from the owed column; the count is the completion predicate and moves monotonically down
  here, so it is read rather than reasoned about.
- **`check-install-claim`**'s red condition is a **zero count** over the primary-install-path
  claim. Delta 1 adds a section to `installer/README.md`, which is install-claim-bearing prose;
  the count can move in either direction and it is run.
- **`check-comment-tier`** reds on a comment that is not a directive. Delta 2 rewrites nine
  `# spec:` comments; each must remain a directive rather than becoming narration about where it
  now points.
- **`check-crate-arms`** runs the crate's lint and test arms through the battery, and the cut's
  new module is subject to both.
- **`check-md-refs`** reds on a reference resolving to nothing; delta 1 adds a section other
  surfaces will cite and delta 2 re-points three comments at it. Monotone under addition alone,
  not under the re-point, so it is run.

## Existing sections updated

- **`installer/README.md`, a new section** — the packer's owning boundary, its port disposition
  and its sequencing sentence (deltas 1 and 2).
- **`installer/README.md` §The consumer smoke** — it currently carries two of the packer's inner
  pointers and establishes the smoke's own non-shipping status by citing the packer; it cites the
  new section for the packer's contract rather than continuing to host part of it (delta 2).
- **`installer/README.md` §The install boundary** — read, and its reach confirmed as
  `installer/lib` and `installer/bin` by its own words, so the packer is outside it; recorded so
  the next reader does not look for the packer's disposition there (delta 2).
- **`gate-sdk/SPEC.md` §Consumer payload** — read, and deliberately **not** made the owner; its
  four packer-naming rules stay exactly where they are, and delta 1 records why the section's own
  reach statement is what refuses it (delta 1).
- **`scripts/pack-installer.sh`** — its header `# spec:` and the two other manifest-pointing
  inner comments re-homed; the file then cut (deltas 2 and 3).
- **`CLAUDE.md` §Housekeeping** — its one-line packer sentence points at the new owning section
  rather than being the packer's home (delta 2).
- **`native/`** — the new non-gate arm's module and its dispatch registration (delta 3).
- **`TASK-QUEUE.md` `pack-installer-no-owning-spec-section`** — the placement ruled and the cut
  landed; and **`spec-pointer-boundary-legality`**, which stays filed and is explicitly not
  closed by this unit (all deltas).

## Definition of Done

- [ ] **Causal completeness** — the new section names its mechanical reader; the ported arm names
      two real callers and the transition each reads it at, and the binary's reachability is
      confirmed on every host the arm runs on, the new macOS leg included.
- [ ] **Merged with no information lost** — every refusal cause, exit status and help line the
      shell form carried is preserved; the four-flag roster keeps its single tier.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the repo root
      (`ls SPEC-*.md`).
- [ ] **Removals propagated** — the owed column re-read with `--emit port-blockers --tree`; the
      binary rebuilt with `bash gate-sdk/bin/build-native.sh`; `check-spec-pointer` and
      `check-md-refs` green.
- [ ] **The unclosed sibling is left unclosed and said so** — `spec-pointer-boundary-legality`
      remains filed, because this unit gives one file a legal target and does not teach the gate
      to grade target legality.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed to the gap inbox.
