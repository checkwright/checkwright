# SPEC amendment: economics-cut

The port disposition of **`drift-kit/bin/stage-economics.sh` (464 lines), the one owed file
declaring drift-kit/SPEC.md §The stage-economics meter**: it ports to the
`--emit-stage-economics` bridged arm, **adopting** the shared session derivation the sibling
`SPEC-overhead-cut.md` extracts rather than carrying one of its own. A stated-contract cut under
the port-only run (TRAJECTORY.md §PRIORITY DIRECTIVE), hosted on `drift-kit-bin-port-residue` and
packaged by the lead as the second of this iteration's three.

**The pair moves together by ruling, not by sizing.** drift-kit/SPEC.md:632-641 and TRAJECTORY.md
record the 2026-09-05 operator ruling in consult: both meters adopt this meter's derivation, one
copy in-crate, and the alternative refused was porting each meter with its own key. Splitting the
two across iterations would leave that divergence standing in-crate, which is what the ruling
exists to prevent. **The ordering is therefore load-bearing:** `SPEC-overhead-cut.md` delta 2
extracts `native/src/sessions.rs`, and this cut's delta 2 *reads* it. That extraction lands
first, whether the two ride one build batch or two.

**The composer's precondition was run at this stage rather than inherited.**
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --group` trails *108 member(s) scanned, 0
group(s) formed, … 0 still owed, 0 takeable at this cut* — no takeable group, the budget arm's
stated precondition (gate-sdk/SPEC.md §The first cohort). The selection ground is the **owed
column** of `--emit port-blockers --tree` — *92 file(s) scanned, 66 declared no-port, 0 temporarily
held, 26 owed* — where this file reads `owed lines=464`.

## The owed column understates this member, and the understatement is structural

The lead carried this forward on the host entry so spec would price it rather than rediscover it at
build, comparing it to the shape `run-gates.sh` had at 503. **Probed here rather than relayed**, by
a census of what the crate already holds. The shell buys five subsystems in a few lines each,
because `jq`, `awk`, `git` and `sort` supply them; the crate has **none of the five**, verified by
an exhaustive grep of `native/src/**/*.rs`:

| what the join needs | the shell's cost | what exists in-crate |
| --- | --- | --- |
| a JSONL transcript reader | one `jq` filter, `:71-76` | **nothing.** `json.rs` is a jq-*path* compiler over an already-parsed `Value` with no parse entry point and it never opens a file; `usage_trend.rs` parses a space-separated `key=value` text log, not JSONL; `footprint.rs` measures markdown bytes |
| assistant-turn usage extraction, last-usage-per-message-id dedup, per-model fold | one `awk` block, `:77-82` | **nothing.** `grep -rn 'input_tokens\|cache_read' native/src` returns zero hits crate-wide — no reader, fixture or otherwise, touches either field name today |
| the committed-history stamp harvest | `git log … -p -U0` piped to one `sed`, `:126-127` | **partly** — `emit/trajectory.rs` holds a *private inline* `Git` reader (`:26-45`) and `added_stamp` (`:107`), but the format is `COMMIT %H`, the filter demands a configured stage name, and it discards the session-id the join is keyed on |
| the price table and its arithmetic | one `while read` and one `awk` `BEGIN`, `:91-103` | **nothing.** No TSV parser; f64 parse/format precedent exists but no pricing |
| the `.meta.json` parent walk | one `jq` over a glob plus a bash walk, `:327-361` | **nothing.** No `parentAgentId` reader, no sibling-record resolution |

Beside those, eleven bash associative maps and four explicit `*_ORDER` vectors carry state the
shell needed because its maps are unordered; each is a real structure in the port. **This is a
sizing statement for the lead's batch cut, not a delta** — no line estimate is asserted here,
because an estimate is a number with no oracle behind it. What is asserted is the decomposition,
and each subsystem is its own delta below so a batch can be cut along it.

## What changes

### (1) The cut is the one owed file declaring §The stage-economics meter, and with the sibling it empties drift-kit's owed column

`bin/stage-economics.sh` reads `owed lines=464` and is the **only** owed file whose `# spec:`
pointer binds `## The stage-economics meter` {mechanical}. Its reach and its section bound
coincide, and no second section is rewritten by construction — with the one stated exception every
budget batch moves, gate-sdk/SPEC.md §The first cohort's *a budget batch records only findings*
rule (its own worked instance is `§run-gates`' stub cut, the precedent this cut's finding below
follows).

**Together with `SPEC-overhead-cut.md` it discharges the kit**, which is worth stating because it
changes what the section owes: after both, drift-kit holds no owed file, and this section gains the
residue paragraph §The knowledge-friction loop established at `drift-kit/SPEC.md:467-477` — the
owed set is empty and no later cut is sequenced against it. Neither cut can make that claim alone.

### (2) The meter adopts the shared derivation and writes no second copy

{design-bearing} The 2026-09-05 ruling names *this* meter's derivation as the one both adopt, and
the sibling cut extracts it to `native/src/sessions.rs` with three public items (`normalize`,
`sessions_dir`, `resolve`) and one private helper, `candidate_globs(dir: &str) -> [String; 2]`,
that `resolve`'s widened branch walks and this delta's own new function shares rather than
restates. This cut **reads all four** and deletes its own `normalize8` (`:45-48`), `sessions_dir`
(`:34-43`) and the two-tier scan inside `find_transcript` (`:52-64`).

**What this member needs beyond `resolve`, and why it is not a second derivation.** `resolve`
answers *which transcript is this session's*; the join asks the inverse, *which transcript does
this `session8` name*. That is a lookup over the same two-tier candidate set, and it is spelled as
a **fourth** function this delta adds beside the sibling's three — `pub fn find(i: &Inputs,
session8: &str) -> Option<String>` — walking `sessions::candidate_globs(&sessions_dir(i))`,
applying `normalize` to each candidate basename, filtering to matches, and keeping the newest,
exactly as `:57-61` does. It shares the candidate glob with `resolve` **by calling the sibling's
own private helper**, not by an unstated assumption about `resolve`'s internals — the property
that makes it one derivation and not two: **a change to the tier layout moves one glob,** because
both functions read it from the same source. `find` is this cut's own addition to the shared
module, landing *after* the sibling's delta 2 (which must exist first for `find` to have
`candidate_globs` to call), never a precondition of it.

**The raw-prefix trap is preserved and stated**, because it is the failure the shell's own comment
at `:50-51` was written against: this repo's stage sessions are subagent transcripts named
`agent-<hex>.jsonl` whose stamp is `<hex>` truncated to eight, so matching a raw filename prefix
against the `agent-` prefix selects nothing. The port normalizes each candidate, never the pattern.

### (3) The transcript usage reader is written new, and the port makes per-model row order deterministic

{design-bearing} `usage_by_model` (`:69-83`) is one `jq` filter and one two-pass `awk`. The port
writes it as: a line-at-a-time read of the JSONL, `serde_json::from_str` per line, a filter on
`.type == "assistant"` with a non-null `.message.usage`, then a map keyed on `.message.id`
**keeping the last record per id**, then a fold into four counters per `.message.model`. The
message-id dedup is not an optimisation and must not be dropped: §The stage-economics meter rules
that a streaming transcript repeats a message id across lines with input and cache constant and
output growing, so summing raw lines multi-counts.

**A line the parser cannot read is skipped, not fatal**, matching `jq -rc`'s behavior under the
shell's pipeline and the meter's advisory exit-0 contract. A transcript with no assistant-turn
usage keeps its existing named skip line (`:239`).

**The behavior change is stated rather than discovered at build.** The shell's aggregation ends in
`for (m in seen)`, and awk's array iteration order is **unspecified** — so today a session touching
two models emits its rows in an arbitrary order that can differ between runs. A Rust map iterated
in a defined order makes that deterministic. The port takes the determinism deliberately and
records it here because it is an observable difference a reader could otherwise read as an
accident; the log's dedup key is the `<iteration> <stage> <model>` triple, so no row's *identity*
depends on the order either way.

**The four explicit order vectors are not incidental and are preserved by name.** `SESS_ORDER`
(`:172`), `LEAD_ORDER` (`:173`), `ANCHOR_ORDER` (`:174`) and `FANOUT_ORDER` (`:319`) exist because
bash maps are unordered; each becomes an insertion-ordered structure carrying the same sequence.
A port that replaced them with a sorted map would change which anchor is recorded first and which
iteration takes an apportionment remainder.

### (4) The stamp harvest shares one git-history reader with the trajectory arm, and each keeps its own field parse

{design-bearing} `collect_stamps` (`:125-130`) is the history ∪ live read: `git log --reverse
--format='%H' -p -U0 -- <state-file>` filtered by a `sed -E` over added lines, unioned with the
live file's content. `emit/trajectory.rs` already performs the same read — `git -C <top> log
--reverse --format=COMMIT %H -p -U0 -- <state_file>` at `:141-149`, behind an existence probe at
`:138`, with `added_stamp` (`:107`) as its added-line filter.

**The ruling: the git invocation and its added-line stream get one owner; the field parse stays
per-reader.** The shared piece yields `(commit, added_line)` pairs for a path's whole committed
history, and a consumer that does not want the commit discards it. The coupling that must have one
owner is the **git output shape** — the `-p -U0` diff form and the `+`-prefix convention — because
a change there breaks both readers silently and in the same way; that is the second producer
gate-sdk/SPEC.md §The port-candidate criteria criterion 6 refuses. The **field selection** is not
shared: trajectory wants `(iteration, stage)` filtered against its configured `DRIFT_KIT_STAGES`
roster, and this meter wants `(iteration, stage, session8)` with no roster filter at all. Folding
those into one parser would give this meter a roster dependency it does not have and must not
acquire — a stamp whose stage is outside the roster still carries real spend.

**Two alternatives are refused.** *A second git invocation of its own* is the duplicate above.
*Reusing `added_stamp` as it stands* is refused on its own text: it requires a configured stage
name and returns two fields, so this meter would either inherit a roster filter or re-parse the
line it was handed.

**The degradation contract is preserved exactly.** `Git::read` yields `None` on a non-zero git exit,
which is the shell's `2>/dev/null`; an absent live state file is a notice and the run continues to
the history arm; the 0-exit *nothing to read* notice fires only when **both** sources yield no
stamps. And the diagnosis paragraph §The stage-economics meter carries — that a missing
`(iteration, stage)` row never means the boundary truncation lost it — survives untouched, because
the union survives untouched.

### (5) The price table is parsed in-crate and the pricing arithmetic must preserve its term order

{design-bearing} `:89-95` reads the TSV — skipping blanks, `#` comments and a `model` header row —
into four per-model maps, and `price_cell` (`:97-103`) prices with `awk 'BEGIN { printf "%.4f", i*pi
+ o*po + cr*pcr + cw*pcw }'`. The port parses the same file and prices in `f64`.

**The one instruction a build session cannot get from the shell by reading it.** Floating-point
addition is **not associative**, so the four terms must be summed in the shell's own left-to-right
order — `in`, `out`, `cache-read`, `cache-creation` — or a cost's last digit can differ from the
series already in the trend log. `{:.4}` and awk's `%.4f` agree on rounding, both being the
platform's double formatting, so the format is a direct translation; the term order is the part
that is not.

**Degradation is unchanged and each branch keeps its named reader.** An absent table, or one with
no row for a model the transcripts name, degrades that model's cost cell to `n/a`, the tokens still
report, and the run raises the incomplete-pricing caveat. The two dating headers are **not** read
here — `kpi-price-table-age` (`native/src/emit/kpi/price_table_age.rs:38,47,71`) reads them off
`ctx.price_table`, and this meter reads only rows, so an expired table still prices loudly rather
than silently. That separation is stated because a port that "reads the price table" invites
folding the header read into it.

### (6) The supervision and fan-out passes port whole, with the parent walk reading the harness's meta sibling in-crate

{design-bearing} Three derived structures move together, and none of them may be simplified,
because each is a ruled answer in §The stage-economics meter rather than an implementation choice:

- **The supervision row** (`:253-305`) — a nested-tier transcript's path names its supervising
  lead, the lead's own transcript resolves on the flat tier, its usage sums with the same reader,
  and a lead spanning iterations apportions by dispatch count with an integer split, remainder to
  the iteration holding the most dispatches, ties broken by iteration name. `split_tokens`
  (`:107-120`) is that split and its re-sum-to-the-whole property is the reason it exists.
  The `sort -k1,1nr -k2,2` at `:273` is the tie-break and ports as an explicit comparator —
  numeric descending on the count, then lexicographic ascending on the iteration — never as a
  default sort.
- **The fan-out row** (`:307-428`) — the `.meta.json` sibling read, `parentAgentId` resolved
  against `agent-<id>.jsonl` with the bare `<id>.jsonl` spelling accepted as a fallback, an absent
  `parentAgentId` meaning a direct child of the root session, the walk to the **nearest** anchor
  bounded by a visited set and by the transcript population, then apportion-first-fold-second into
  the row key.
- **The two collision rules and the attribution invariant** — a stamp naming
  `DRIFT_KIT_SUPERVISION_LABEL` suppresses supervision rows and, structurally, their fan-out; a
  stamp ending in `DRIFT_KIT_FANOUT_SUFFIX` suppresses fan-out rows; and a transcript already
  claimed by a row is never a candidate for another.

**`jq` leaves the meta read too**, so the `input_filename`-keyed batch read at `:340` becomes a
per-file `serde_json` parse. The path is then the loop variable rather than a field of the parsed
document — which removes the one place the shell depended on a `jq` invocation flag to tell it
which file a record came from.

**The degradation table in §The stage-economics meter is the port's own acceptance criterion** and
every row of it must still hold: the meta layer absent entirely, one record absent, a
`parentAgentId` naming an agent with no transcript, a cycle or over-long chain, and the price table
absent. Its `jq` row is the one exception and delta 7 owns it.

### (7) `jq` leaves the arm's spawned set, and one degradation branch retires with it

{design-bearing} §The stage-economics meter rules that *parsing the transcript needs `jq`; its
absence degrades to a token-less notice rather than a failure*, and the degradation table lists
`jq absent` as *already fatal to the whole join upstream*. In-crate, `serde_json` is the crate's
one dependency (`native/Cargo.toml:15`) and is linked, so **the branch cannot be entered**. It is
**deleted rather than kept as dead code**, on the same rule §run-gates applied to a documented flag
that does nothing: a documented degradation that can never fire is worse than none.

Two consequences follow and both are stated because a reader will look for them.
`drift-kit/smoke/install.sh:392` asserts the notice (`grep -q 'jq not found'`) and is deleted with
the branch — the one smoke assertion this cut removes rather than re-points. And the arm's spawned
program set shrinks from the shell's `{jq, awk, sed, sort, grep, mktemp, git, date}` to `{git,
date}`; gate-sdk/SPEC.md §The non-gate arm records spawned programs in prose, and `grep -rn
'proc::' native/src/emit/` is the derivation the build re-runs rather than transcribing this list.

### (8) The arm family is forced, the roster is the meter's own knobs, and no KPI context field is added

{design-bearing} gate-sdk/SPEC.md §The non-gate arm's forced-family test decides the family: the
meter resolves seven knobs — `DRIFT_KIT_METRIC_DIR`, `DRIFT_KIT_STAGE_ECONOMICS_LOG`,
`DRIFT_KIT_PRICE_TABLE`, `DRIFT_KIT_STATE_FILE`, `DRIFT_KIT_SESSIONS_DIR`,
`DRIFT_KIT_SUPERVISION_LABEL`, `DRIFT_KIT_FANOUT_SUFFIX` — so a hardcoded top-level flag would
resolve platform defaults and ignore every consumer override. **Only two of the seven,
`DRIFT_KIT_METRIC_DIR` and `DRIFT_KIT_PRICE_TABLE`, are actually defined and defaulted in
`lib/drift.sh` today** (`lib/drift.sh:51,58`, checked by reading the file whole — 61 lines). The
other five are currently defaulted only inline, inside `bin/stage-economics.sh` itself (`:28,
30-32`), the file this cut deletes — and `gate-sdk/lib/gate.sh:42`'s own `# spec:` rule is exact
about the consequence of shipping that as-is: *a knob the owning kit's library does not define is
the bridge's undeclared-knob refusal, so a compiled member declaring it would fail-close on every
invocation*. One of the five, `DRIFT_KIT_SESSIONS_DIR`, is the sibling cut's to close (its own
delta 4 adds the empty-default line, landing first per this amendment's own ordering rule) — this
cut does not re-add it. **The remaining four are this delta's own obligation**, closed with four
literal-default lines in `lib/drift.sh`, each mirroring the value the deleted script computes
today and matching the shape the adjacent `DRIFT_KIT_OVERHEAD_LOG` bullet already takes
(`lib/drift.sh:55`):

- `: "${DRIFT_KIT_STAGE_ECONOMICS_LOG:=$DRIFT_KIT_METRIC_DIR/stage-economics-log.txt}"`
- `: "${DRIFT_KIT_STATE_FILE:=${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt}"` — the same
  `${GATE_SDK_WORKFLOW_DIR:-.workflow}` composition `DRIFT_KIT_TRAJECTORY_SURFACES` already uses
  (`lib/drift.sh:38-39`).
- `: "${DRIFT_KIT_SUPERVISION_LABEL:=supervision}"`
- `: "${DRIFT_KIT_FANOUT_SUFFIX:=+fanout}"`

None of these is a new knob or a new default *value* — each name and value already governs the
shell tool's behavior today; what moves is only the default's resolution site, from the deleted
script into the config bridge's one sourced library, which is the same relocation §Layout already
records for `DRIFT_KIT_PRICE_TABLE`. It joins `BRIDGED_ARMS` as `--emit-stage-economics`, reached
as `run-gates.sh --emit stage-economics`, declaring exactly those seven.

**`Arm::Emit`, on the variant's own test.** The meter is advisory: exit is always 0, so no `1` is
load-bearing and the `{0, 2}` collapse at `native/src/main.rs:485-495` costs nothing. It returns the
report as its document and appends its rows as a side effect — `--emit-kfric`'s shape.

**No `Ctx` field is added, and that is a non-target rather than an omission.** `DRIFT_KIT_STAGE_ECONOMICS_LOG`
appears nowhere in `native/src` today, and a reader who notices that `drift_report::KNOBS` declares
the prefix family `DRIFT_KIT_*` may conclude a KPI context field is owed. None is: no bundled KPI
reads this log (the log's readers are the `/economics` narrative, the operator, and the deferred
`benchmark-ab-experiment` rung), and `kpi::Ctx` is what *built-in KPIs* read. A field with no reader
is a field removed, and this one would have none.

**The meter takes no argument today and gains none.** So §The non-gate arm's unportable-argument
test has nothing to bind on here, and the free-text shape refusal does not reach a member with no
positional.

### (9) The smoke re-points onto the arm and the two dead settings grants drop in the same commit

{mechanical} `drift-kit/smoke/install.sh` is this meter's behavioural oracle across six fixture
sets — the flat join, price-table-absent, the hermetic fake-history repo proving history ∪ live,
the two-stamp attribution fixture, the nested-tier supervision fixture and the three-level fan-out
fixture with its two degradation assertions. Its **five** direct invocations (`:366`, `:410`,
`:436`, `:464`, `:527`) re-point from `bash "$SMOKE_KIT_ROOT/bin/stage-economics.sh"` onto
`bash "$DRIFT_ARM" --emit stage-economics`, `$DRIFT_ARM` being the handle the file defines at
`:34`. `:388` and `:392` are assertion lines against the price-table-absent invocation's output,
not invocations themselves — `:392` is the jq-less assertion delta 7 deletes, named there rather
than double-counted here. **Every assertion survives unchanged except that one**, and §Testing
names each of them; a build session re-greps before editing rather than working from this list,
because the suite decides.

`.claude/settings.json:19` and `:20` — `"Bash(bash drift-kit/bin/stage-economics.sh)"` and its
wildcard form — name a path this cut deletes and drop **in the same commit as the delete**, inside
`native-gate-port-remaining-corpus` ruling (2)'s 2026-08-29 base and guard-kit/SPEC.md:1323-1324's
*a pure narrowing, forced by the change and landing with it*. The 2026-09-05 widening is not relied
on: ruled (i) by the lead on its own authority, 2026-09-05, on this stage's escalation. No addition
is owed, `.claude/settings.json:12` already granting every bridged arm.

## Producers and consumers

This cut introduces **no new knob, no new log, no new field and no new file on any governed
surface**. It moves one producer into the crate, deletes three duplicated derivations (the session
normalization, the sessions-dir default and the two-tier scan) in favour of the sibling cut's
shared module, and gives one shared owner to a git read that had two. The survey below is over the
**whole component set**: every tracked file was grepped for `stage-economics`, for
`DRIFT_KIT_STAGE_ECONOMICS_LOG` and for `DRIFT_KIT_PRICE_TABLE`, with stderr left open.

**The interfaces whose producers move.**

- **The session derivation.** Producer before: this script's own `normalize8`, `sessions_dir` and
  `find_transcript`. Producer after: `native/src/sessions.rs`, shared with `--emit-session-id` and
  `--emit-overhead-meter` (delta 2), gaining `find` as this cut's own addition to it. Consumer:
  this arm, resolving `DRIFT_KIT_SESSIONS_DIR` and handing the value in on `Inputs`. Enabling
  config: the default *value* is unchanged, but its bridge-visibility is the sibling cut's own
  delta 4 fix, landing first — this arm inherits it rather than re-adding it.
- **The committed-history read.** Producer before: two — this script's `git log … -p -U0` piped to
  `sed`, and `emit/trajectory.rs`'s private `Git` reader. Producer after: one shared reader
  yielding `(commit, added_line)` (delta 4). Consumers: `emit/trajectory.rs`, which keeps its
  `COMMIT`-boundary use and its roster-filtered `added_stamp`; and this arm, which discards the
  commit and parses three fields with no roster filter.
- **The transcript usage reader.** Producer before: `jq` plus `awk`, spawned. Producer after: an
  in-crate reader (delta 3). Consumers: this arm's stage pass, its supervision pass and its fan-out
  pass — three call sites, one reader, which is the property the shell already held at `:233`,
  `:279` and `:371` and the port must not lose.

**The trend log's fields, each with its named reader at its named transition.** No field is added
or removed; the obligation is to show each still has a reader:

| field | reader | transition |
| --- | --- | --- |
| `date` | the `/economics` narrative's reading-age caveat; the operator | at narrative render; at any close-over-close read |
| `<iteration> <stage> <model>` | the meter's own dedup filter | at append |
| `in`, `out`, `cw` | the `/economics` narrative; the deferred `benchmark-ab-experiment` rung | at narrative render |
| `cr` | the same, as the **headline** burn lever | the same |
| `cost` | the `/economics` narrative; the operator close-over-close | the same |

**Every caller of the meter, surveyed.**

| caller | site | what this cut owes it |
| --- | --- | --- |
| the `/economics` skill | `drift-kit/templates/economics.md` | re-point; it chains both meters, and the sibling cut owes the other half |
| this kit's smoke | `drift-kit/smoke/install.sh` ×7 | delta 9, plus delta 7's one deletion |
| the settings grants | `.claude/settings.json:19,20` | delta 9, removal |
| `kpi-price-table-age` | `native/src/emit/kpi/price_table_age.rs:38` | **nothing** — it reads the table's headers off `ctx.price_table`, never this meter's output (delta 5) |
| `emit/trajectory.rs` | `native/src/emit/trajectory.rs:141-149` | delta 4 — it gains a shared reader and keeps its own parse and its own roster |

**The readers that are not callers, checked rather than assumed.**
`lifecycle-kit/SPEC.md` §The state machine owns the stamp grammar and this meter is a **read-only
consumer** of it: no stamp is added, no cursor moves, no stage-skill template changes, and the
supervision and fan-out edges stay *derived*. That is unchanged by the port and is restated here
only because the port touches the reader. And `smoke/install.sh` counts trend-log lines as an
asserting reader of the log's grammar (§The stage-economics meter's blast-radius paragraph names
it); delta 9 keeps it pointed at the same grammar.

## Existing sections updated

- `drift-kit/SPEC.md` §The stage-economics meter, **input 2, the transcripts paragraph**
  (`:728-764`) — its `jq` sentence is falsified (delta 7) and its normalization and two-tier scan
  become the shared module's rather than this tool's (deltas 2, 3). The raw-prefix warning stays: it
  is why the normalization exists.
- `drift-kit/SPEC.md` §The stage-economics meter, **input 1, the stamps paragraph** (`:688-727`) —
  the history ∪ live read gains its shared owner and its per-reader field parse (delta 4). The
  union's reasoning, the no-depth-knob ruling and the standing-misdiagnosis paragraph are unchanged
  in substance and re-attributed.
- `drift-kit/SPEC.md` §The stage-economics meter, **the degradation paragraph** (`:786-795`) — the
  `jq` branch retires (delta 7); every other branch is unchanged and the paragraph says which.
- `drift-kit/SPEC.md` §The stage-economics meter, **the fan-out row's degradation table**
  (`:1032-1039`) — its `jq absent` row is removed (delta 7); the other five are the port's
  acceptance criteria (delta 6).
- `drift-kit/SPEC.md` §The stage-economics meter, **the trend-log paragraph** (`:819-849`) — no
  field changes; it gains the determinism delta 3 makes true of per-model row order, stated where
  a reader of the log will look.
- `drift-kit/SPEC.md` §The stage-economics meter — gains its **port-owed residue paragraph**, and
  with the sibling's it records that drift-kit's owed column is empty (delta 1).
- `drift-kit/SPEC.md` §Layout and configuration, the `DRIFT_KIT_SESSIONS_DIR` and
  `DRIFT_KIT_STATE_FILE` bullets (`:1223-1228`, `:1292-1297`) — both carry the *re-derives with
  its own knob rather than importing a sibling kit's bin contract* parenthetical, and both stay
  true: the knobs remain drift-kit's and the sharing is below the config layer (delta 2). The
  sibling amendment owns the `DRIFT_KIT_SESSIONS_DIR` bullet's edit (its delta 4 adds the
  bridge-visible empty default); this one names it so the two do not both rewrite it. The
  `DRIFT_KIT_STATE_FILE` bullet is this cut's own to edit: its stated default is not yet
  bridge-visible either, and delta 8 adds the literal-default line that makes it so, alongside the
  three other knobs delta 8 closes the same gap for.
- `drift-kit/SPEC.md` §Layout and configuration, the `DRIFT_KIT_PRICE_TABLE` bullet (`:1265-1273`)
  — *two sites compute this default, not three: `lib/drift.sh` for every bridge reader, and the
  standalone meter* collapses to one when the standalone meter stops being standalone (deltas 5, 8).
  The bullet already records the same collapse happening to the KPI's restatement and now completes
  it.
- `drift-kit/SPEC.md` §Layout and configuration, the `DRIFT_KIT_STAGE_ECONOMICS_LOG`,
  `DRIFT_KIT_SUPERVISION_LABEL` and `DRIFT_KIT_FANOUT_SUFFIX` bullets (`:1262-1264`, `:1274-1280`,
  `:1281-1287`) — none currently states where its default is computed; each gains the one clause
  the `DRIFT_KIT_OVERHEAD_LOG` bullet already carries (drift-kit/SPEC.md:1229-1235), naming
  `lib/drift.sh` as the resolver now that the standalone script it was computed inside is gone
  (delta 8).
- `drift-kit/SPEC.md` §Testing, the stage-economics fixture paragraphs (`:1344-1391`) — every
  fixture is driven through the arm (delta 9) and the jq-less assertion is deleted (delta 7); no
  other assertion changes and the section says so.
- `drift-kit/SPEC.md` §The `/economics` skill — the chain runs two arms (deltas 8, 9).
- `gate-sdk/SPEC.md` §The non-gate arm — the `--emit-` family roster gains `--emit-stage-economics`
  with its dated attribution, and the spawned-program prose records that this member spawns `git`
  and `date` and no interpreter (deltas 7, 8).
- `drift-kit/README.md` — its `bash drift-kit/bin/stage-economics.sh` usage example (`## Use`) and
  the two prose mentions of the script by path re-point onto the arm form, the same edit class
  delta 9 already makes for the smoke; missed by neither survey's grep in isolation but named here
  because the sibling cut's audit found the identical gap for its own README example.
- `gate-sdk/SPEC.md` §The first cohort — under its own rule, *a budget batch adds a section only
  where it has a finding to record*, this cut adds one paragraph beside the sibling's, in the shape
  its worked instance (`§run-gates`' stub cut) already took: an owed line count prices a shell
  file's *text*, not its *reach*, and the gap is widest where the shell bought whole subsystems
  from spawned interpreters — the census table above is the shape of that gap and the way to
  measure it before composing (all deltas motivate it). This corrects the amendment's own earlier
  citation of "§The port disposition" for this finding: that heading (`gate-sdk/SPEC.md:7979`,
  nested under §Consumer smoke) is a differently-scoped, already-occupied section and is not this
  cut's to edit.
- `lifecycle-kit/SPEC.md` §The state machine — **no rule changes**; named as a target because the
  port must be re-read against it to confirm the read-only consumption still holds, and a target
  named only to be confirmed is cheaper than one discovered at build (delta 4).

<!-- update-target-exempt: the file is deleted whole by this cut, so its fourteen `# spec:` lines survive in no delta's scope -->
- `drift-kit/bin/stage-economics.sh` — the file and its `# spec:` pointers, removed with it.

## Definition of Done

- [ ] **Causal completeness** — the session derivation, the git-history read and the usage reader
      each have one named producer and named consumers; every trend-log field has a named reader at
      a named transition; no knob, field or `Ctx` entry is added.
- [ ] **The oracle decides, not the roster** — `drift-kit/smoke/install.sh` green across all six
      stage-economics fixture sets, the crate's lint and test arms green through
      `check-crate-arms`, `bash gate-sdk/bin/build-native.sh` run, and the tree re-grepped for
      `stage-economics.sh` before the cut is called done.
- [ ] **The sibling landed first** — `native/src/sessions.rs` exists with `normalize`,
      `sessions_dir`, `resolve` and the private `candidate_globs` helper before this cut's delta 2
      is written against it. `find` is **not** part of that precondition — it is this cut's own
      delta 2 that adds `find` to the module, and no amendment other than this one ever creates it.
- [ ] **The bridge resolves the meter's five non-sibling knobs rather than fail-closing on them** —
      `lib/drift.sh` gains the four literal-default lines delta 8 specifies for
      `DRIFT_KIT_STAGE_ECONOMICS_LOG`, `DRIFT_KIT_STATE_FILE`, `DRIFT_KIT_SUPERVISION_LABEL` and
      `DRIFT_KIT_FANOUT_SUFFIX` (the fifth, `DRIFT_KIT_SESSIONS_DIR`, is the sibling's), and a bare
      `run-gates.sh --emit stage-economics` invocation with every one of the seven knobs unset is
      exercised, since that is the invocation the gap would have broken.
- [ ] **Pricing parity is verified, not assumed** — the four terms summed in the shell's order, and
      a fixture cost compared against the shell form's output before the shell form is deleted.
- [ ] **`trajectory.rs` is unchanged in behavior** — its own tests pass against the shared reader,
      including the one pinning that `+++ b/…` stays out of the added-line stream.
- [ ] **The grant removals land in the deleting commit** — `.claude/settings.json:19,20`, under
      ruling (2)'s 2026-08-29 base and guard-kit/SPEC.md:1323-1324, with no addition owed.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); §The stage-economics meter reads as one document to a reader who never
      saw this amendment, with the union's reasoning and every un-retired degradation preserved.
- [ ] **Amendment deleted** — this file removed on merge; the none-remain assertion discharges at
      the iteration, `SPEC-overhead-cut.md` being a sibling in flight for this component
      (canon-kit/SPEC.md §Merging an amendment step 3).
- [ ] **The terminal move is a demotion, not Done** — `drift-kit-bin-port-residue`'s deliverable is
      a corpus; it returns to the deferred section at the position its promoting commit's diff
      records, compressed in the same commit if the transcribed roster pushes it against
      queue-kit/SPEC.md §check-queue-entry-budget.
- [ ] **Removals propagated** — every spec grepped for the names this change retires (`jq` as a
      requirement of this join, the standalone meter's own default resolvers, the two-site
      price-table default); nothing dangles.
- [ ] **The oracle re-read, not the arithmetic trusted** —
      `bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree` re-run at the landing commit and
      its owed count recorded.
- [ ] **Gaps filed** — any cross-component gap found during the work filed to the committed gap
      inbox, and any fact re-derived off a non-owning surface stamped with `--emit kfric`.
