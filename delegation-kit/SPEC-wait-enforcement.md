# SPEC amendment: wait-enforcement

The waiting rule has fired ten times across six iterations, through every prose
carrier the project has, including the agent definition itself. This settles the
question the 2026-08-06 operator ruling deliberately left open — **given that
prose alone does not hold, what does** — and it is `waiting-rule-fourth-firing-post-fix`'s
promoted half, promoted 2026-08-17 by operator ruling on its third threshold
recurrence.

**The answer is a reframing before it is a mechanism, and the reframing is the
design.** §Operative residency's own ruling is that no gate can read a session's
choice to end a turn, because the act passes no chokepoint. That ruling is
**upheld, not overturned** — and the sentence it ships beside is what this
amendment executes: *"'no tracked artifact' is why neither rule gets a gate over
the tree, and it is not by itself a reason to stop looking for an oracle."* The
turn-end is unreachable. **The harm the turn-end causes is not**: it arrives as
an ordinary tool call, at a chokepoint this repo already fires a hook on, and it
is what the tenth firing actually cost evidence for.

**The tenth firing is the worked case and it is why this shape was chosen.** That
session backgrounded `run-validate.sh`, backgrounded a `kill -0` loop on it,
ended its turn — and then **committed a gap filing while the producer was still
running**, dirtying the worktree the `installer_smoke` pack step checks. The run
reported `verdict=new-failures` on a false ground and had to be discarded and
re-run. The turn-end cost turns; the **mutation under a live producer** cost a
whole suite's evidence, and that mutation is a `Bash` call a `PreToolUse` guard
sees.

**What is deliberately not taken, so a build session does not reopen it.** The
`SubagentStop` hook is the chokepoint that would reach the turn-end itself, and
it stays `subagent-stop-liveness-hook-wiring`'s: wiring one is a
`.claude/settings.json` change no agent message authorizes, and that entry's
refusal is settled rather than re-litigated here. **Nothing in this amendment
touches `settings.json`** — every surface it uses is already wired. That is the
property that makes it buildable this iteration and it is stated up front,
because the queue entry's own framing names a settings probe as the unit's first
obligation and this design routes around that probe rather than waiting on it.

## What changes

### 1. The launch-time liveness record gains a **naming convention**

A session backgrounding a shell child writes its record as
`<scratch-dir>/<key>.run`, carrying the `pid=<n> run=<key>` line it already
carries — **design-bearing**, because what the convention buys is a **derivable
set** where today there is only a path a reader must be told.

The grammar is untouched: it stays evidence-kit's, for the reason §The delegation
model already gives — *"an inverted grammar would have bought a second
implementation of a decided thing"*. What was missing is not the record's shape
but its **discoverability**. Today the record's path is known only to the session
that wrote it and to whoever it tells, so the second reader §The delegation model
names — *"whoever arrives next ... where the launcher is gone"* — can read a
record only if it happens to guess the name. A suffix makes the set a glob, and
every delta below reads that set rather than a path.

**`.run` rather than `.lock`, and the distinction is load-bearing.** A lock is
claimed and released by one owner and its absence means *free*;
`EVIDENCE_KIT_LOCK_FILE` is one and keeps its name. A launch record is a
**statement of fact left behind** — §The delegation model's own words for it are
that it answers *"is it still running* **to whoever arrives next**", a question a
dead session's successor can still ask — and its absence means *nothing was
recorded*, never *nothing is running*. Two meanings, two suffixes.

### 2. `check-producer-liveness` gains a **set mode**

Pointed at a directory, the gate reads every `*.run` record in it and reports the
live ones — **design-bearing**.

Its per-record verdict, its exit contract and its PID predicate are **unchanged
and are not re-decided**: green when no record names a live PID, red naming every
blocking run key, exit 2 on a record it cannot parse. What the mode adds is the
**quantifier**, and the aggregation rule is the only new decision: exit 2 wins
over red wins over green, so one corrupt record cannot be averaged away by nine
clean ones. A directory holding no record at all is **green**, which is the same
verdict the absent-lock case already takes.

The existing single-path mode stays exactly as it is — `EVIDENCE_KIT_LOCK_FILE`
has one path and one writer, and forcing it through a directory would be the
generalization that breaks the case that works.

### 3. The entry preflight is pointed at the record **set**, at **every** stage

`LIFECYCLE_KIT_ENTRY_PREFLIGHT` gains one `check-producer-liveness` entry per
stage, pointed at the scratch directory rather than at `.tmp/run-validate.lock` —
**design-bearing**, and this delta is a **consumer-config change in this repo**,
not a kit-default change.

**The present wiring's hole is measured rather than asserted.** The knob names
the liveness gate twice, `validate=` and `close=`, and both point at exactly
`.tmp/run-validate.lock`. So the eighth firing — validate ending its turn on a
live `run-validate.sh` — was in fact reachable by the close entry, and the
**ninth was not**: close ended its turn on a live `gh run watch`, which has no
lock and whose launch record no preflight entry names. The gate was right and its
aim was one path wide.

**Why every stage rather than the two that carry it.** The two were chosen when
the subject was one producer with one lock; the subject is now any recorded
producer, and any stage can leave one. The cost is one gate invocation per stage
entry against a directory that is usually empty.

**What this delta is honest about: it detects, and it detects late.** An orphan is
found at the *next* entry, so the turns between the firing and that entry are
still spent — and if the firing is the iteration's last stage, no entry follows
it. Detection is the backstop §Operative residency already names; delta 4 is the
half that prevents.

### 4. guard-kit's generic ruleset gains **rule 14 — tracked-tree mutation under a live producer**

A `git` command that writes the index, the worktree or a ref is **blocked** while
a `*.run` record in the consumer's scratch directories names a live PID —
**design-bearing**, and this is the delta that answers the unit's question.

**The act set is bounded and named rather than gestured at**: `add`, `commit`,
`rm`, `mv`, `restore`, `checkout`, `switch`, `reset`, `stash`, `merge`, `rebase`,
`cherry-pick`, `revert`, `apply`, `am`, `clean`. Read-only git — `status`,
`log`, `diff`, `show`, `rev-parse`, `ls-files` — passes, and so does every
non-`git` command. **`Write`/`Edit` are deliberately outside the rule**: a
mutating session's mandated journal write lands through them, the scratch
directory is gitignored, and a rule that blocked them would refuse the very
mechanic §Resume journal requires while a wait is in progress.

**Why the block is right even though it over-reaches, stated rather than
softened.** A read-only producer — `gh run watch` was the ninth firing's — takes
no harm from a commit, and the record cannot say which kind it is. This ruleset's
established direction is to bias toward passing; that direction is **not** taken
here, and the ground is that the biased-toward-passing reading is what has failed
ten times. The corrective is cheap and names both exits: **wait for the producer
on its own artifact, or delete the record if the producer is done** — and
deleting a record whose producer has exited is not a workaround, it is the
statement of fact becoming false and being retracted.

**Conservative in this ruleset's other established directions**, which are kept:
an expansion or substitution anywhere in the command declines outright (rule 6
already blocks those shapes); a record that does not parse declines rather than
blocks, because a guard is not the place a corruption verdict is taken and
delta 2's consumer already exits 2 on one; an unrecognized `git` subcommand
declines. Each biases toward passing rather than toward a false block, which is
the calibration the previous paragraph departs from **only** on the read-only
producer.

**Placed with rules 12 and 13, before every auto-allow rule — so it inserts at
14 and the present 14–19 shift to 15–20.** It is the third member of the
wait-discipline family and inherits their placement argument (`git` is not on
the default `GUARD_KIT_RO_BINS` roster, but a consumer that widened it would
otherwise have the read-only-pipeline rule silently grant the mutation). **Two
collisions make the placement necessary rather than tidy**, and both are with
rules that today sit *after* the auto-allows: the decorated-allowlist rule would
block a chained `git commit` first and hand back *run it bare* — a corrective
that is wrong under a live producer and that the session can follow; and the
git-history-rewrite rule **advises** on `git commit --amend`, so a rewrite under
a live producer would proceed with a re-verification steer instead of stopping.

### 4a. The renumbering, and why it is a named cost rather than a surprise

Inserting at 14 renumbers six rules — **mechanical**, and bounded by a count
taken rather than estimated: **12** `rule N` cross-references inside
guard-kit/SPEC.md and **5** in `lib/guard.sh`'s `spec:` directives name a rule
at 14 or above and move with it; the `docs/` mirror is a generated projection and
is **regenerated, never hand-edited**.

**The rule numbers are a SPEC-side ordinal only.** `lib/guard.sh` dispatches
named functions (`guard_rule_pgrep_self_match`, `guard_rule_git_rewrite`), so no
implementation identifier renumbers — the new rule lands as one more named
function inserted into the ordered run.

**The move is precedented and the ruleset says so itself.** The
git-history-rewrite rule's own text records that *"this ruleset renumbers on its
own account too — this rule's own number moved when rules 12 and 13 were
inserted ahead of it"*, and gives the reason a doctrine rule is cited by name
there rather than by number. That is this insertion's precedent, and it is why
the numbering is not worked around by appending at the end: list order **is**
execution order in that section, and an appended rule whose prose claimed an
earlier position would put the two into disagreement.

### 5. The generic ruleset's admitting clause takes its **second** widening, with a stated ground

§The generic ruleset admits *harness or shell-substrate behavior, never any
project's toolchain* — **design-bearing**, because rule 14 is neither of the two
admitted classes as written and slipping it in unremarked is what would rot the
clause.

**The precedent is the clause's own history**, which the section records rather
than hides: *"The shell-substrate half of that clause was added for rule 12, and
it admits nothing a project owns."* This widening takes the same form. The
admitted class becomes **behavior over an artifact whose grammar a kit owns** —
here evidence-kit's `pid=<n> run=<key>` record and the OS's process table. Rule
17 interprets no project vocabulary at all: it reads a PID and asks whether it is
alive. A project's toolchain stays out on the same words as before, and the test
the widening must pass is the one rule 12's passed — **it admits nothing a
project owns**.

### 6. §Operative residency records the answer, and records what it does not claim

The section states today that no gate is owed and that the enforcement-design
question is untouched. Both sentences move — **design-bearing**.

What lands: the enforcement question is **answered by relocation**. The act is
unreachable and stays unreachable; the **harm** passes a chokepoint and is
blocked there. The chokepoint ruling itself is confirmed rather than weakened —
rule 14 is a `PreToolUse` rule, which is precisely the class §The delegation
model says the axis admits.

What lands with it, because a claim without its bound is the failure this unit
exists to end: **the enforcement holds only for a session that recorded**. §The
delegation model already rules that the record's write is structurally
uncheckable — *"a session that skips the rule writes nothing, so there is no
absence a check could have been told to expect"* — so a session that backgrounds
without recording is invisible to rule 14 and to delta 3 alike, exactly as it is
today. This unit narrows the failure to that residue rather than closing it, and
delta 7 is what owns the residue.

### 7. Enforcing the record's **write** at the launch chokepoint is designed, refused here, and filed

The remaining residue has a candidate — a rule firing on the backgrounding call
itself and refusing one that writes no record — and it is **not taken**, on a
fact this session could not establish — **design-bearing**, because a build
session will reach for it and must find a ruling rather than a blank.

**Two backgrounding forms, and the guard's reach differs between them.** A shell
`&` is in the command text, which every rule in this ruleset already reads, so
that arm is buildable today. The harness's `run_in_background` is a **tool
parameter**, not command text — and whether it appears in the `PreToolUse`
payload's `tool_input` is **unverified in this tree**: probed, and no guard here
reads it. What *is* established is the general reach —
`scripts/agent-dispatch-guard.sh` reads `tool_input.subagent_type` and
`tool_input.isolation`, so non-headline `tool_input` fields are reachable by
`jq` path — which makes this an empirical question about one field rather than
about the mechanism.

**Building only the `&` arm is refused as worse than not building it.** Every
attested firing used the harness form; a rule covering only the shell form would
block the spelling nobody uses and pass the one that fires, which is a rule that
reads as coverage and asserts nothing.

**Filed costed** against `turn-end-chokepoint-and-wait-primitive`, which holds
this unit's other open mechanism question, with the probe named: one backgrounded
`Bash` call through a guard that records its payload. **That probe is not
`subagent-stop-liveness-hook-wiring`'s** — it needs no `settings.json` change,
because the `PreToolUse(Bash)` matcher is already wired — and a build session must
not refuse it as the operator-gated one.

### 8. The rule's carriers are re-swept by its own wording, not by a roster

Rule 17's corrective states the wait clause, so the carrier set grows — and the
sweep is by **grepping the clause's phrasing across the tree**, never by
consulting §Operative residency's list — **mechanical**.

That method is the section's own ruling, earned the last time this fired: *"a
roster is a record of the carriers someone noticed, never a proof of the set."*
The known carriers at authoring are the two `.claude/agents/` definitions and
guard-kit rule 12's block message; the sweep is what decides the actual set.

### 9. Fixtures, and the battery before commit

`check-producer-liveness`'s pair gains the set-mode verdicts — an empty
directory, a directory whose records are all dead, one holding a live record, and
one holding an unparseable record beside a clean one (the aggregation rule of
delta 2). Rule 17 takes guard-kit's own rule-test coverage in the shape rules 12
and 13 carry. Then `bash gate-sdk/bin/build-native.sh` and
`bash gate-sdk/bin/run-gates.sh`, neither discharging the other —
**mechanical**.

## Producers and consumers

### The `<key>.run` launch record

- **Producer:** the session backgrounding a shell child, at launch, by the shell
  builtin the templates already spell — unchanged except for the filename.
  **Enabling config actually emitted:** the scratch directory is
  `GUARD_KIT_SCRATCH_DIRS` for rule 14's reader and this repo's `.tmp/` for
  delta 3's, both set in this repo's committed config today; the record needs no
  knob of its own. Not test-only — the record is written on every dispatch that
  backgrounds a producer, which is every validate session.
- **Consumers, named with the mechanism, surveyed across the whole component
  set:**
  - `check-producer-liveness` set mode (delta 2), by directory read, at the
    stage-entry preflight (delta 3).
  - guard-kit rule 14 (delta 4), by directory read, at every `PreToolUse(Bash)`
    firing whose command is a mutating `git`.
  - the **launching session itself**, in-turn, as its wait condition — the first
    of the two readers §The delegation model names, unchanged by this unit.
  - `enter-stage.sh`'s boundary reset, which sweeps the scratch directory at the
    iteration boundary against `scripts/lifecycle-config.sh`'s keep-list. A
    `*.run` record is **not** added to that keep-list: a record surviving an
    iteration boundary names a producer from a previous iteration, which is
    precisely the stale statement the sweep exists to remove.
  - **`check-scratch-citation` is surveyed and is not a consumer**: its subject
    is a finding cited from scratch into a permanent surface, and a liveness
    record is neither.
- **Every field has a named reader.** The record has exactly two, both
  pre-existing: `pid=` is read by `ek_pid_alive` at both consumers above, and
  `run=` is read by `check-producer-liveness`'s finding line and by rule 14's
  corrective, each naming the blocking run so a reader can tell *wait for that
  run* from *reclaim a record whose owner is gone*. **The filename's `<key>` is
  not a third field** — it is the same `run=` value, and delta 1 makes it the
  filename rather than adding data.

### `check-producer-liveness` set mode

- **Producer:** `evidence-kit/checks/check-producer-liveness.sh`, invoked with a
  directory argument. Reached by exactly one enabling configuration:
  `LIFECYCLE_KIT_ENTRY_PREFLIGHT`, which delta 3 sets in this repo's committed
  consumer config — not a default, and stated as such.
- **Consumer:** `lifecycle-kit/bin/enter-stage.sh`, which runs each matching
  preflight entry and **refuses the entry on a non-zero exit, writing nothing**.
  That refusal path is the mode's whole point and it already exists.
- **Red condition, named rather than its subject:** red when any record in the
  directory names a live PID; exit 2 when any record fails to parse, which wins
  over red; green on an empty directory and on all-dead records.

### guard-kit rule 14

- **Producer:** `lib/guard.sh`, invoked from the consumer's template guard —
  `scripts/bash-guard.sh` in this repo, already registered under the
  `PreToolUse` `Bash` matcher in `.claude/settings.json`. **No hook registration
  is added**, which is what keeps this unit clear of the operator gate.
- **Consumer:** the calling session, through `guard_block`'s stderr text at
  exit 2 — the refusal contract this ruleset already ships.
- **Red condition, named:** a command whose segment leads with `git` and whose
  subcommand is in delta 4's named write set, **and** a `*.run` record under a
  `GUARD_KIT_SCRATCH_DIRS` member names a live PID. Absent either conjunct it
  does not fire; on an unparseable record, an expansion, or an unrecognized
  subcommand it **declines**.

### Nothing here narrows a corpus, and one delta *widens* a gate's reach

Causal-completeness point 5 binds on a narrowing. Delta 2 widens
`check-producer-liveness` and delta 3 widens where it is pointed; delta 4 adds a
rule. The one reader whose verdict is non-monotone under a *widening* is the
preflight itself: pointing it at a set means a stage entry can now be **refused**
where it previously passed, which is the mechanism working and is why delta 1's
naming convention and delta 3's wiring must land together — a record dropped in
the scratch directory under an old name is invisible, and one under the new name
with a dead PID is green, so neither ordering strands a session.

## Existing sections updated

- **delegation-kit/SPEC.md §The delegation model** — delta 1's naming convention,
  in the paragraph that already rules the record's grammar and its two readers;
  and delta 7's refusal, recorded where the write-side's structural
  uncheckability is already ruled.
- **delegation-kit/SPEC.md §Operative residency** — delta 6. Its *"No gate is
  owed, and not for budget"* paragraph and its closing *"the enforcement-design
  question ... is untouched"* clause both move; the chokepoint ruling above them
  is confirmed unchanged.
- **delegation-kit/templates/agent-execution.md** — the wait rule's own carrier,
  which states the record's placement; delta 1's filename lands there, and delta 8
  is the sweep that finds the rest.
- **evidence-kit/SPEC.md §check-producer-liveness** — delta 2's set mode and its
  aggregation rule, beside the argument mode it generalizes; its
  *"Its subject is a record, not this lock"* paragraph is where the set belongs.
- **guard-kit/SPEC.md §The generic ruleset** — delta 4's rule 14 and delta 5's
  widening of the admitting clause, which is the section's opening prose.
- **`scripts/lifecycle-config.sh`** — delta 3's preflight entries, with the
  `spec:` line on that array updated: it currently states the liveness gate is
  wired *"at the entry and nowhere else: `close=` is the filed case, `validate=`
  stops a second validate batch"*, which delta 3 supersedes.
- **CLAUDE.md §Housekeeping** — the `.tmp/` bullet names the scratch directory's
  contents; a `*.run` record is a new class there and the keep-list ruling of the
  producers section is what it must not contradict.
- **`turn-end-chokepoint-and-wait-primitive`** — delta 7's filed residue, costed
  against the entry that already holds this unit's other open mechanism question.

## Definition of Done

- [ ] **Causal completeness** — the record's two consumers plus its two
      surveyed non-consumers are named with mechanisms; both record fields have a
      named reader at a named transition; the set mode's and rule 14's red
      conditions are named rather than their subjects; the filename is shown to
      be an existing field rather than a new one.
- [ ] **Merged with no information lost** — each delta lands in the canonical
      section named above; delta 6 **replaces** two claims in §Operative residency
      rather than appending a correction beside them, and delta 5 rewrites the
      admitting clause rather than annotating it.
- [ ] **Amendment deleted** — this file removed on merge; `ls delegation-kit/SPEC-*.md`
      clean.
- [ ] **Removals propagated** — the `.tmp/run-validate.lock` preflight spelling is
      grepped tree-wide before it is replaced, and every prose carrier stating the
      record's placement is found by **grepping the clause's wording**, not by
      reading a roster (delta 8). Stderr not silenced on either grep.
- [ ] **Gaps filed** — delta 7's launch-chokepoint rule filed costed through
      `bash lifecycle-kit/bin/file-gap.sh`, with its probe named and distinguished
      from the operator-gated one; any build-time causal gap resolved that session.
- [ ] **No `settings.json` write** — every surface this unit uses is already
      wired, and a build session that finds itself editing the permission or hook
      surface has left the amendment.
- [ ] **Terminal move** — `waiting-rule-fourth-firing-post-fix`'s promoted half is
      complete in one unit, so its entry moves to `## Done`. The routing clause it
      carries — *a third threshold recurrence routes to the operator* — is **live
      and unspent**, ruled so at the promotion relay, and travels with the entry
      rather than being retired by it.
