# SPEC amendment: overhead-cut

The port disposition of **`drift-kit/bin/overhead-meter.sh` (105 lines), the one owed file
declaring drift-kit/SPEC.md §The overhead meter**: it ports to the `--emit-overhead-meter`
bridged arm, and the session-key derivation §The overhead meter rules it must adopt is
**extracted into a shared crate module** rather than written a second time. A stated-contract
cut under the port-only run (TRAJECTORY.md §PRIORITY DIRECTIVE), hosted on
`drift-kit-bin-port-residue` and packaged by the lead as the first of this iteration's three.

**`overhead-meter-measures-the-lead` rides inside this cut** and is promoted with it, under
`native-gate-port-remaining-corpus`' ruling **(6)** — *an entry whose discharge is an owed file's
stated precondition rides inside that cut*. It is not a sibling unit and gets no amendment of its
own: its whole deliverable is delta 3 here. Its own 2026-09-05 operator ruling in consult says
the same thing in its own words — *the port carries the delegation-aware derivation in-crate and
both meters read it* — so this amendment implements that ruling and does not re-open it.

**The composer's precondition was run at this stage rather than inherited.**
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --group` trails *108 member(s) scanned, 0
group(s) formed, 0 undecidable, 108 already ported and excluded, 0 permanently shell and excluded,
0 temporarily held and excluded; 0 still owed, 0 takeable at this cut* — **no takeable group**,
which is the budget arm's stated precondition (gate-sdk/SPEC.md §The first cohort). The size arm
is permanently exhausted, so the budget arm composes. The selection ground is the **owed column**
of `--emit port-blockers --tree` — *92 file(s) scanned, 66 declared no-port, 0 temporarily held,
26 owed* — where this file reads `owed lines=105`.

**The seam is ruled and this amendment does not re-open it.** drift-kit/SPEC.md:632-641 rules that
the port *adopts the stage-economics derivation for both meters*: strip a leading `agent-`, scan
the subagent tier as well as the flat one, and resolve delegation-aware. TRAJECTORY.md records the
same ruling (2026-09-05, operator, consult) and the alternative it refused — porting each meter
with its own key, which would make the divergence permanent in a log whose only readers are
compiled. What the ruling states is the **destination**; what it does not state, and what this
amendment owes, is **where the one derivation lives**, what its interface is, and what the
delegation-aware resolution does to a log that already has rows in it. Those are deltas (2)
and (3).

**Cut ordering, stated here because build reads it and nothing else records it.** The sibling
`SPEC-economics-cut.md` **adopts** the module delta 2 extracts. So delta 2 must land before
that cut's own derivation delta, whether the two ride one batch or two. Authoring them as two
amendments that disagree about the derivation is the failure this ordering exists to prevent.

## What changes

### (1) The cut is the one owed file declaring §The overhead meter, and it does not discharge drift-kit

`bin/overhead-meter.sh` reads `owed lines=105` and is the **only** owed file whose `# spec:`
pointer binds `## The overhead meter` {mechanical}. Its reach and its section bound therefore
coincide, and no second section is rewritten by construction — with the one stated exception every
budget batch moves, gate-sdk/SPEC.md §The first cohort's *a budget batch records only findings*
rule (its own worked instance is `§run-gates`' stub cut, the precedent this cut's finding below
follows).

**It does not discharge drift-kit**, and the amendment says so because the kit's owed column is
short enough that a reader would assume otherwise. `bin/stage-economics.sh` (464) leaves it in the
sibling cut of this same iteration, not here. After both, drift-kit's owed column is empty and
§The knowledge-friction loop's residue paragraph — *this section's port-owed set is empty* — gains
its counterpart in this section and in the sibling's.

### (2) The shared session derivation is an extraction, not a mint, and it lands in a top-level crate module

This is the cut's central design ruling and the reason its delta set is not all-mechanical
{design-bearing}.

**The obstacle is stated first, because "carry the derivation in-crate once" reads as new work
until you look at what is already there.** `native/src/emit/session_id.rs` **already holds every
part of it**: `normalize` (`:29`, strip a leading `agent-` token then take eight characters),
`sessions_dir` (`:47`, the override else `<config-home>/projects/<cwd-slug>`), `pick` (`:62`, the
glob advance keeping bash's strictly-newer `-nt` semantics), and `derive` (`:81`, the
delegation-aware order — the env child-session id narrows the scan to
`<dir>/<harness-id>/subagents/*.jsonl`, verified rather than trusted, falling back to the two-tier
scan `<dir>/*.jsonl` then `<dir>/*/subagents/*.jsonl`). `Inputs` (`:13`) already takes the
sessions dir as a **field** rather than reading a knob name, which is exactly the seam a second
kit needs. So the ruling's *one derivation, carried in-crate once and read by both arms* is
satisfied by **promoting what exists**, and writing a second copy would be the divergence the
ruling refused, relocated from two shell scripts into two Rust modules.

**Four of those five items are private** (`normalize`, `slug`, `sessions_dir`, `pick`), and the
module has exactly two callers crate-wide — `native/src/emit/enter_stage.rs:432` and its own
`BRIDGED_ARMS` row at `native/src/emit/mod.rs:392-393`. So the extraction is a visibility and
placement change, not a rewrite.

**The ruling: the derivation moves to a top-level module, `native/src/sessions.rs`, and
`emit/session_id.rs` becomes a thin caller of it.** Top-level is where this crate already keeps a
kit-owned derivation that more than one arm reads — `toolfloor.rs`, `guard.rs`, `queue.rs`,
`stages.rs`, `evidence.rs` are all that shape — and `emit/` is where an *arm* lives. A module under
`emit/` that a non-`emit` reader imports would make the directory mean two things.

**The interface the extraction must expose, and why `derive` alone does not serve.**
`derive` returns a normalized **id**; the overhead meter needs the **transcript path** that id was
derived from, and the sibling cut needs a `session8 → path` resolver. So the module exposes three
public items, one private helper the sibling's lookup shares, and `derive` is re-expressed on top
of them:

- `pub fn normalize(id: &str) -> String` — the shared key, unchanged from `session_id.rs:29`.
- `pub fn sessions_dir(i: &Inputs) -> String` — unchanged from `:47`, still taking the dir as a
  field so each kit resolves its own knob and hands the answer in.
- `pub fn resolve(i: &Inputs) -> Result<String, String>` — the delegation-aware **path**
  resolution folding in `derive`'s current `:81-126`: its two early returns (an explicit
  `session_id`, or a bare `harness_id` with no child flag — `:82-87`, outside the narrower
  `:88-126` range a first read reaches for) return `basename`-free, since a bare id already has no
  `/` or `.jsonl` suffix and `normalize(basename(x)) == normalize(x)` for one; the delegation-aware
  scan (`:88-126`) is the rest. `resolve` returns the winning **path** in every branch rather than
  the normalized key, which is the one behavior change from `derive`'s body.
- `fn candidate_globs(dir: &str) -> [String; 2]` (private) — the two literal patterns `resolve`'s
  widened branch already walks, `{dir}/*.jsonl` and `{dir}/*/subagents/*.jsonl` (`:104-105` in the
  pre-extraction form), lifted out so a second caller can enumerate the same tier layout without
  restating the two strings. This is the seam the sibling cut's `find` needs and is named here
  because a reader auditing that cut needs to find it *in this one*: `find` answers "which
  transcript does this `session8` name" by walking the same two patterns with its own predicate,
  never its own copy of them, which is what makes the tier layout one glob rather than two.

`derive` becomes `normalize(basename(resolve(i)?))`, which is what it already computes at its own
tail; its behavior, its two refusal messages and its `emit` wrapper are unchanged, and
`--emit-session-id`'s contract (lifecycle-kit/SPEC.md §bin/session-id.sh) is untouched. A build
session verifies that by the module's existing `#[cfg(test)]` tests rather than by inspection.

**The contract owner does not move with the code, and that is the seam this delta holds.**
lifecycle-kit/SPEC.md §bin/session-id.sh keeps ownership of the derivation's *contract* — it
already owns the phrase "the same normalization" — and both drift sections **cite** it rather
than restating it. drift-kit keeps its **own knobs**: `DRIFT_KIT_SESSIONS_DIR` resolves the dir and
is handed to `Inputs.sessions_dir`. So drift-kit/SPEC.md §Layout and configuration's standing
sentence — *drift-kit re-derives with its own knob rather than importing a sibling kit's bin
contract* — **stays true and is not contradicted**, and the amendment says so because a reader
will reach for that sentence first. Two facts make it hold: the sharing happens *below* the config
layer, where a knob has already been resolved to a value; and post-port there is no bin contract
left to import, both tools being arms of one binary.

**Two alternatives are refused, each on its own ground.**

- **Copying the derivation into a drift-kit module** is refused as the ruling's own refused
  alternative reached from the other side. The 2026-09-05 ruling refused porting each *meter* with
  its own key; a third copy beside `session_id.rs` is the same defect with a different pair of
  holders, and gate-sdk/SPEC.md §lib/gate.sh's *exactly one place a value is computed* is the
  standing rule it breaks.
- **Leaving the items private and having drift call `derive`** is refused because `derive` returns
  the wrong thing. The meter would then re-glob the sessions dir to find the path behind the id it
  was handed, which is a second scan with a second chance to disagree — the divergence again.

**`session_id.rs`'s empty `KNOBS` is not disturbed.** Its own `# spec:` at `:3-5` rules the roster
must stay empty because neither name it reads is defined in lifecycle-kit's `lib/stages.sh`, so a
declared row would fail-close through the bridge on every invocation. The extraction moves no env
read into a declared roster: `sessions.rs` reads nothing from the environment at all — every input
arrives on `Inputs` — and each *arm* declares its own kit's knobs. Recorded because "the shared
module needs a roster" is the first thing a reader will assume.

### (3) The meter's resolution becomes delegation-aware, which discharges `overhead-meter-measures-the-lead`, and the trend series splits at the port date

Today the bare invocation picks the newest `*.jsonl` in the **flat tier alone**
(`overhead-meter.sh:44-53`), with no delegation branch. Under a live lead that resolves the
**lead's** transcript, so every close-stage row taken under the lead posture is the lead's own
governance/task split filed under a stage session's name. After delta 2 the meter calls
`sessions::resolve` and gets the session it is actually running in {design-bearing}.

**What the trend log does, stated because it is the consequence a reader will look for and the
queue entry rules it is not a repair.** `overhead-meter-measures-the-lead` records that each
logged row is a truthful measurement of a real session keyed by its own `session8`, so nothing is
deleted and nothing is re-measured. Two facts bound the change:

- **No existing row is orphaned and no key collides.** The old flat-tier glob could only ever
  resolve a transcript sitting directly under the sessions dir, so every logged key is a
  full-length id's first eight characters. The new resolution adds nested-tier transcripts, whose
  key is `normalize("agent-<hex>") = <hex>[0..8]` — a value the old reader never wrote. The
  populations are disjoint; the log simply starts carrying rows for a population it never carried.
- **The series is discontinuous at the port date, and the log gains no field for it.** A log field
  with no reader is a field removed (§The overhead meter's own rule), and this one would have
  none: `kpi-overhead` reads `pct`, `gate`, `total` and field one as the date
  (`native/src/emit/kpi/overhead.rs:36-40`), and the operator reads the same. The `date` field
  **already partitions the series**, so the port date recorded in §The overhead meter is what a
  reader splits on — derivation-first, and the cheaper of the two honest options.

**The residual limit is stated rather than closed.** `kpi-overhead` summarizes the trailing ten
lines (`kpi/overhead.rs:31`), so for roughly ten measured sessions after the cut its window spans
both populations and its average is a blend. Nothing in this cut fixes that and nothing should: the
window is the KPI's, the blend is finite and self-clearing, and a special case keyed on a date
would be a second thing to stale.

### (4) The arm family is forced, not chosen, and the meter joins the bridged table as `Arm::Emit`

{design-bearing} gate-sdk/SPEC.md §The non-gate arm's forced-family test decides this before any
preference does: *the family choice is forced for any tool that needs configuration at all*, since
a hardcoded top-level flag *resolves platform defaults and silently ignores every consumer
override*. The meter resolves `DRIFT_KIT_SESSIONS_DIR`, `DRIFT_KIT_METRIC_DIR` and
`DRIFT_KIT_OVERHEAD_LOG`. **Only the latter two are defined and defaulted in `lib/drift.sh` today,
and this cut must close that gap rather than assume it closed.** `DRIFT_KIT_SESSIONS_DIR` is
currently computed only inline, inside the standalone script's own `sessions_dir()` function
(`:31-40`); `lib/drift.sh` — read whole, 61 lines — carries no line for it. That costs the shell form
nothing, since a standalone script needs no bridge, but a compiled arm does:
`gate-sdk/lib/gate.sh:42`'s own `# spec:` states the rule this cut must satisfy — *a knob the owning
kit's library does not define is the bridge's undeclared-knob refusal, so a compiled member
declaring [it] would fail-close on every invocation* — and a bare invocation with no override set is
this meter's primary, intended use, so an unclosed gap fails the common case, not an edge one.
**Delta 4 therefore adds one line to `lib/drift.sh`**: `: "${DRIFT_KIT_SESSIONS_DIR:=}"`, the same
empty-default shape `DRIFT_KIT_ICEBOX_SECTION` already takes (`lib/drift.sh:54`) — declared so
`declare -p` finds it, computing nothing itself, because the computed fallback
(`<config-home>/projects/<cwd-slug>`) already lives in `sessions.rs::sessions_dir` (delta 2) and a
second copy in shell would be exactly the divergence this cut exists to avoid. So the arm is a
**bridged-arm table** member, spelled `--emit-overhead-meter` and reached through the front end as
`run-gates.sh --emit overhead-meter`.

**`Arm::Emit` rather than `Arm::Run`, on the variant's own stated test.** `native/src/emit/mod.rs`
rules the variant a return shape: `Arm::Emit` collapses every outcome to `{0, 2}` at
`native/src/main.rs:485-495`, while `Arm::Run` returns the member's own `i32` verbatim and is
required whenever a `1` is load-bearing. The meter is advisory by construction — **exit is always
0** and a missing transcript is a 0-exit notice — so no `1` exists to carry, and the arm returns
the report as its document while appending its line as a side effect. That is `--emit-kfric`'s
shape exactly (`native/src/emit/kfric.rs`), which is the sibling this member should be read
against.

**Its declared roster is the three knobs above and no more.** `DRIFT_KIT_TMP_DIR` is resolved by
the script at `:27` and **read by nothing in it** — a dead resolution the port drops rather than
declares, since a declared knob with no reader is the same defect as a log field with no reader.

**One argument survives the port and one arm does not exist to retire.** The optional
`[transcript.jsonl]` positional is the shape gate-sdk/SPEC.md §The non-gate arm calls an
*input-corpus positional* — it selects what the rule analyses, and no knob resolves it — so it
ports unchanged, the shape `check-gate-tamper`'s `--fixture` established. The meter carries **no**
`-h`/`--help` arm today, so nothing retires to the front end and the sentence §The knowledge-friction
loop had to write about `--emit kfric --help` has no counterpart here.

**The free-text shape refusal does not reach this member**, and that is worth one line because
three sibling members carry it. §The non-gate arm's refusal binds *free text reaching a capture
tool* — an argument written verbatim into a committed surface. This positional is a **path the
tool reads**, and nothing the caller passes is written to the log, so the hazard the refusal
addresses is absent.

### (5) The keyed log rewrite ports unchanged, against an in-crate append-only precedent that governs a different class

{design-bearing} `overhead-meter.sh:92-97` filters the log for lines carrying ` <session8> total=`
and rewrites it, then appends — the `grep -Fv` + `mv` shape. The crate has **no precedent for
it**: every in-crate log writer is append-only (`kfric.rs:19-26`, `file_survey.rs:135-139`,
`file_gap.rs`), and `file_survey.rs:132-134` states that as doctrine — *append-only within the
iteration, never edited in place, so a filing cannot lose a block a concurrent one landed*.

**The owner doc rules it and precedent does not.** §The overhead meter states in its own voice that
`session8` *is the dedup key the meter reads on append — re-measuring a session replaces its line
rather than double-counting it*, and §Testing asserts exactly that. Spec-over-precedent: the owner
doc is ground truth, and the append-only doctrine governs a different class — **capture** logs,
where concurrent filings by independent sessions must not be lost. A measurement log is
re-derivable by construction; a capture is not.

**The honest limit the port inherits, stated rather than fixed.** A read-filter-rewrite is lossy
under genuinely concurrent measurement: two meters finishing at once can lose one row. That is the
shell's behavior today and the port asserts nothing new, so closing it is not this cut's work. It
is bounded by the producer — §The overhead meter names the consumer's close-stage binding, one
session — and a build session that finds the bound false files it rather than widening the cut.

### (6) The classifier is a verbatim move, with one byte-semantics obligation the shell stated in a comment

{mechanical} The marker table (`overhead-meter.sh:65-74`) is five awk rules over each transcript
line, first match wins, unmatched bytes to task. It moves verbatim: same patterns, same order,
same whole-line classification. Two mechanical obligations ride with it.

`LC_ALL=C` is what makes awk's `length()` count **bytes**, which is the byte-proxy contract's
whole basis; in Rust the byte length is `line.len()` and the locale question does not arise, so the
shell's comment at `:64` retires into the module rather than being translated. And the percentage
rounding at `:79-80` is integer half-up (`(gov * 100 + total / 2) / total`); the crate's precedent
for reproducing awk's half-up is `kpi/overhead.rs:44,46`, pinned by that module's own test at
`:112-115`, and the port uses it rather than `f64::round`.

### (7) The smoke re-points onto the arm and the dead settings grant drops in the same commit

{mechanical} `drift-kit/smoke/install.sh` is the meter's behavioural oracle and stays on the shell
substrate permanently (§Testing). Its two direct invocations — `:197` and `:244` — re-point from
`bash "$SMOKE_KIT_ROOT/bin/overhead-meter.sh"` onto `bash "$DRIFT_ARM" --emit overhead-meter`,
`$DRIFT_ARM` being the front-end handle the file already defines at `:34` and already uses for
`--emit kfric` at `:594`. **Every assertion in those blocks survives unchanged** — the log-line
grammar, the task-line exclusion, `gate` a proper subset of `gov`, `pct` as the rounded governance
share, the replace-on-re-measure probe, and the writer/reader-divergence assertion spanning `:244`
(the writer, re-pointed here), `:248` (the reader, `kpi-overhead`) and `:251` (the divergence check
itself) that runs meter and KPI under one `DRIFT_KIT_METRIC_DIR` override.

`.claude/settings.json:18`, `"Bash(bash drift-kit/bin/overhead-meter.sh)"`, names a path this cut
deletes and drops **in the same commit as the delete**. That is inside
`native-gate-port-remaining-corpus` ruling (2)'s 2026-08-29 base — *removing a grant whose target a
RULED PORT CUT DELETES is OUTSIDE the 2026-08-22 bar — a pure narrowing* — and guard-kit/SPEC.md
:1323-1324 states the same rule from the kit side. **The 2026-09-05 widening is not relied on**
and needs no construal: ruled (i) by the lead on its own authority, 2026-09-05, on this stage's
escalation. No addition is owed — `.claude/settings.json:12` already grants
`Bash(bash gate-sdk/bin/run-gates.sh *)`, which covers every bridged arm.

## Producers and consumers

This cut introduces **no new knob, no new log and no new file on any governed surface**. It moves
an existing producer into an existing consumer, promotes a private crate derivation to a shared
one, and changes which transcript one resolver picks. The survey below is over the **whole
component set**: every tracked file was grepped for `overhead-meter`, for `session_id` and for
`DRIFT_KIT_OVERHEAD_LOG`, with stderr left open.

**The interface whose producer moves: the session derivation.**

- **Producer, before:** two independent resolvers — `overhead-meter.sh:31-53` (flat glob, newest
  wins, no delegation branch) and `native/src/emit/session_id.rs:47-126` (delegation-aware,
  two-tier).
- **Producer, after:** one, `native/src/sessions.rs`, holding `normalize`, `sessions_dir`, `resolve`
  and the private `candidate_globs` helper the sibling cut's lookup shares. Its enabling config is
  not new: `DRIFT_KIT_SESSIONS_DIR` and `LIFECYCLE_KIT_SESSIONS_DIR` both compute the same default
  value today, and neither default *value* changes — but `DRIFT_KIT_SESSIONS_DIR`'s default is
  currently computed inline in the standalone script rather than bridge-visible, which delta 4
  closes with one empty-default line in `lib/drift.sh`; `LIFECYCLE_KIT_SESSIONS_DIR` needs no such
  line, already being resolved through `session_id.rs`'s existing, unmodified arm.
- **Consumers, named:** `emit::session_id::derive` (unchanged behavior, re-expressed on `resolve`),
  the new `--emit-overhead-meter` arm, and — after the sibling cut — `--emit-stage-economics`.
  Each consumer resolves its **own** kit's sessions-dir knob and passes the value in on `Inputs`;
  no consumer reads another kit's knob.
- **What is removed:** the meter's flat-tier glob, and with it the mis-attribution
  `overhead-meter-measures-the-lead` records.

**The trend log's fields, each with its named reader at its named transition.** No field is added
and none is removed, so the obligation is to show that every existing one still has a reader after
the change:

| field | reader | transition |
| --- | --- | --- |
| `date` | `kpi-overhead` (`kpi/overhead.rs:40`) for the reading-age caveat; the operator, for the series split delta 3 names | at KPI render, and at any close-over-close read |
| `session8` | the meter's own dedup filter | at append, delta 5 |
| `total`, `gate` | `kpi-overhead` (`kpi/overhead.rs:36-38`) | at KPI render |
| `gov` | the meter's own stdout; no log reader | at measurement time |
| `pct` | `kpi-overhead` (`kpi/overhead.rs:36`) | at KPI render |

`gov` is the one field with no *log* reader, and it is **kept rather than removed** because it is
derivable-but-stated: it is `total - task` and the sum of the four categories, and the field-removal
rule reaches a field nothing reads at all. Recorded because the rule invites the question.

**Every caller of the meter, surveyed.**

| caller | site | what this cut owes it |
| --- | --- | --- |
| the close-stage binding | `.claude/commands/close.md` | re-point onto `--emit overhead-meter`; it is consumer config, not a lifecycle-kit change (§The overhead meter) |
| the `/economics` skill | `drift-kit/templates/economics.md` | it chains the two meters; re-point both, and cut 2 owes the sibling half |
| this kit's smoke | `drift-kit/smoke/install.sh:197,244` | delta 7 |
| the settings grant | `.claude/settings.json:18` | delta 7, removal |
| the kit's own usage doc | `drift-kit/README.md:75` | its `bash drift-kit/bin/overhead-meter.sh` example line re-points onto the arm form, the same edit class delta 7 already makes for the smoke |
| `kpi-overhead` | `native/src/emit/kpi/overhead.rs:55` | **nothing** — it reads `ctx.overhead_log`, resolved through the bridge; the writer moving substrate does not reach it |

**The one reader that is not a caller, and it must be checked rather than assumed.**
`native/src/emit/enter_stage.rs:432` calls `session_id::emit`. Delta (2) re-expresses `derive`
without changing it, so the call is unaffected — recorded because it is the only place outside the
module that depends on the derivation, and a refactor that changed `derive`'s answer would move the
stage stamp itself.

## Existing sections updated

- `drift-kit/SPEC.md` §The overhead meter, the **divergence paragraph** (`:626-641`) — its subject
  resolves (deltas 2, 3). The recorded divergence and the alternative the ruling refused are
  rewritten as the closed history they become rather than deleted: a later reader asking why two
  meters ever keyed differently needs the answer.
- `drift-kit/SPEC.md` §The overhead meter, the **resolution sentence** (`:603-604`) — *a bare
  invocation resolves the newest transcript under `DRIFT_KIT_SESSIONS_DIR`* is the sentence
  delta 3 falsifies, and it is corrected at its source rather than left to contradict the paragraph
  below it. It gains the port date the series splits on.
- `drift-kit/SPEC.md` §The overhead meter, the **producer sentence** (`:648-653`) — the close-stage
  binding now invokes the arm (deltas 4, 7).
- `drift-kit/SPEC.md` §The overhead meter — gains a **port-owed residue paragraph**, the shape
  §The knowledge-friction loop established at `:467-477` (all deltas). It states that the section's
  owed set is empty and that no later cut is sequenced against it.
- `drift-kit/SPEC.md` §Layout and configuration, the `DRIFT_KIT_SESSIONS_DIR` bullet (`:1223-1228`)
  — its parenthetical *drift-kit re-derives with its own knob rather than importing a sibling kit's
  bin contract* stays true and gains two clauses: the knob is still drift's, and the derivation
  below it is now shared in-crate (delta 2); and the default gains a second resolver, `lib/drift.sh`
  declaring it empty so the bridge can find it, the computed fallback staying in-crate (delta 4) —
  the same "two resolvers, both named" shape the adjacent `DRIFT_KIT_OVERHEAD_LOG` bullet already
  uses.
- `drift-kit/SPEC.md` §Layout and configuration, the `DRIFT_KIT_OVERHEAD_LOG` bullet (`:1229-1235`)
  — *two resolvers compute this default — the meter, which is a standalone tool, and `lib/drift.sh`*
  is falsified by the port: a bridged arm reads what `lib/drift.sh` resolved, so the pair collapses
  to one (deltas 4, 7). This is the same outcome §Layout records for `DRIFT_KIT_PRICE_TABLE` at
  `:1269-1273`, and the smoke's writer/reader assertion keeps its subject.
- `drift-kit/SPEC.md` §Testing, the overhead-meter fixture paragraph (`:1333-1344`) — the meter is
  driven through the arm (delta 7); the assertions are unchanged and the sentence says so.
- `drift-kit/SPEC.md` §The `/economics` skill — the chain runs two arms (deltas 4, 7).
- `lifecycle-kit/SPEC.md` §bin/session-id.sh — it keeps ownership of the derivation's contract and
  gains the one fact its readers now need: the derivation is shared in-crate and this section is
  where its contract is stated (delta 2). No rule of that section changes.
- `gate-sdk/SPEC.md` §The non-gate arm — the `--emit-` family roster gains `--emit-overhead-meter`
  with its dated attribution, the shape every prior member took (delta 4).
- `gate-sdk/SPEC.md` §The first cohort — under its own rule, *a budget batch adds a section only
  where it has a finding to record*, this cut adds one paragraph, in the shape its worked instance
  (`§run-gates`' stub cut) already took: delta 2's finding, that where a port is told to carry a
  derivation in-crate once, look for it in-crate first — the second copy this cut avoided was one
  an unexamined reading would have written (all deltas motivate it, delta 2 is its subject). This
  corrects the amendment's own earlier citation of "§The port disposition" for this finding: that
  heading (`gate-sdk/SPEC.md:7979`, nested under §Consumer smoke) is a differently-scoped,
  already-occupied section and is not this cut's to edit.

<!-- update-target-exempt: the file is deleted whole by this cut, so its own `# spec:` lines survive in no delta's scope -->
- `drift-kit/bin/overhead-meter.sh` — the file and its three `# spec:` pointers, removed with it.

## Definition of Done

- [ ] **Causal completeness** — the shared derivation has one named producer (`native/src/sessions.rs`)
      and three named consumers, each resolving its own kit's knob; every trend-log field has a
      named reader at a named transition; no field and no knob is added.
- [ ] **The oracle decides, not the roster** — `drift-kit/smoke/install.sh` green, the crate's lint
      and test arms green through `check-crate-arms`, `bash gate-sdk/bin/build-native.sh` run, and
      the tree re-grepped for `overhead-meter.sh` before the cut is called done.
- [ ] **`derive` is unchanged and proved so** — `emit/session_id.rs`'s existing `#[cfg(test)]` tests
      pass against the re-expressed form, and `enter_stage.rs:432`'s stamp is unaffected.
- [ ] **The bridge resolves `DRIFT_KIT_SESSIONS_DIR` rather than fail-closing on it** —
      `lib/drift.sh` gains the empty-default line delta 4 specifies, and a bare
      `run-gates.sh --emit overhead-meter` invocation with the knob unset is exercised (not just a
      run with it exported), since that is the invocation the gap would have broken.
- [ ] **The grant removal lands in the deleting commit** — `.claude/settings.json:18`, under ruling
      (2)'s 2026-08-29 base and guard-kit/SPEC.md:1323-1324, with no addition owed.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); §The overhead meter reads as one document to a reader who never saw
      this amendment, with the divergence record preserved as closed history.
- [ ] **Amendment deleted** — this file removed on merge; the none-remain assertion discharges at
      the iteration, since `SPEC-economics-cut.md` is a sibling in flight for this same component
      (canon-kit/SPEC.md §Merging an amendment step 3).
- [ ] **The terminal move is a demotion, not Done** — `drift-kit-bin-port-residue`'s deliverable is
      a corpus; it returns to the deferred section at the position its promoting commit's diff
      records. `overhead-meter-measures-the-lead` moves to `## Done`, its deliverable being finished.
- [ ] **Removals propagated** — every spec grepped for the names this change retires (the flat-tier
      glob, the standalone meter's own default resolver, the two-resolver claim); nothing dangles.
- [ ] **The oracle re-read, not the arithmetic trusted** —
      `bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree` re-run at the landing commit and
      its owed count recorded.
- [ ] **Gaps filed** — any cross-component gap found during the work filed to the committed gap
      inbox, and any fact re-derived off a non-owning surface stamped with `--emit kfric`.
