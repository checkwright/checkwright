# SPEC amendment: preflight-cut

The port disposition of **`scripts/gate-exec.sh` (29 lines), the one owed file this repo
holds behind evidence-kit/SPEC.md §check-evidence-manifest**: it leaves the tree and the
nine `LIFECYCLE_KIT_ENTRY_PREFLIGHT` entries re-point onto `run-gates.sh --only <gate>`,
the form the operator ruled on 2026-09-05 in consult. A stated-contract cut under the
port-only run (TRAJECTORY.md §PRIORITY DIRECTIVE), selected by
`native-gate-port-remaining-corpus`'s composer and packaged by the lead.

**Selection ground, host and packaging: `run-gates-stub-cut`'s**, not restated. The oracle
readings are that entry's and are not re-transcribed here, on §The first cohort's rule that
a budget batch records findings rather than rosters.

**This cut was escalated at spec and its blocking question is answered, which is why the
amendment opens with the answer rather than the work.** The re-point as the ruling spells
it could not be authored directly: `run-gates.sh --only` forwards **no argv to the gate**
in either substrate, while every one of the nine entries carries an argument. The lead
ruled 2026-09-05, on its own authority, that the missing channel is built where the
operator put the caller — `run-gates.sh --only <gate> -- <args…>`, single-member-or-refuse
— and that channel is **`run-gates-stub-cut` delta (2)**, not this amendment's. This cut
consumes it. That split is deliberate: the grammar belongs to the section that owns the
front end, and a second statement of it here would be the drift the split exists to
prevent.

**Its dependency on that sibling is hard, one-directional, and is this cut's one ordering
constraint.** `preflight-front-end-cut` is **not landable before** `run-gates-stub-cut`'s
argv channel exists. Landing it first does not degrade the pre-flight roster — it
**refuses every stage entry in the repo**, which is the most expensive failure available
in this tree. Stated here in the amendment so a build session holds it without deriving
it, and stated on both queue entries.

## What changes

### (1) The cut is one owed file, and the section it declares already carries its disposition

`scripts/gate-exec.sh` reads `owed lines=29` and its `# spec:` (`:2-4`) binds
evidence-kit/SPEC.md §check-evidence-manifest, which already states the outcome at
`:843-850`: *"the battery front-end's `--only <gate>` form is the pre-flight front end,
and a consumer-side resolver script leaves the tree with the surface it drives"*
{mechanical}. The amendment therefore lands the work, not the ruling.

**What that paragraph is silent about is the whole of this cut's difficulty**, and the
silence is corrected by delta (2) rather than glossed: it says every entry re-points and
says nothing at all about the **argument** each entry carries.

### (2) Every entry takes the `--` form, including the one whose knob default would have covered it

This is the cut's central correction and the reason its delta set is not all-mechanical
{design-bearing}. The naive re-point — `<front-end> --only <gate>`, argument dropped —
fails, and it fails for **two independent reasons**, only the second of which was visible
from the entry roster.

- **The argument is load-bearing on eight entries.** Six pass a bare `.tmp` for set mode
  over `*.run`; two pass `.tmp/run-validate.lock` for single-path mode. With no positional
  the gate falls back to `EVIDENCE_KIT_LOCK_FILE`
  (`native/src/gates/producer_liveness.rs:146-149`), whose default *is* the lock file, and
  no knob exists for the set-mode directory (`native/src/gates/mod.rs:1700-1706` declares
  only that one). So six entries would silently stop scanning `*.run`.
- **And the appended argv breaks all nine.**
  `native/src/emit/enter_stage.rs:1260-1263` pushes `<queue>` and `<state>` onto every
  entry's argv — its own `# spec:` at `:527-529` states it — and `--only` consumes every
  remaining token as a member name (`native/src/runner.rs:69-72`). So
  `--only check-evidence-manifest` arrives as
  `--only check-evidence-manifest <queue> <state>` and refuses with *"'<queue>' is not
  registered"*. **The one entry whose knob default is byte-identical to its positional is
  broken too.**

**So the `--` separator is mandatory on every entry rather than an option on eight.** The
re-pointed forms, with the three corpora unchanged:

```
close=gate-sdk/bin/run-gates.sh --only check-evidence-manifest -- .workflow/validate-evidence.txt
<stage>=gate-sdk/bin/run-gates.sh --only check-producer-liveness -- .tmp
<stage>=gate-sdk/bin/run-gates.sh --only check-producer-liveness -- .tmp/run-validate.lock
```

**Nine entries stay nine.** No entry is collapsed, merged or dropped, no knob is minted,
and no gate's behavior changes — which is the property that distinguishes this from the
alternative refused below.

**The trailing `<queue> <state>` lands inside the forwarded argv and that is correct
rather than tolerated.** It is exactly where it lands today: `gate-exec.sh` appends its
own trailing args to the resolved argv (`:29`), so both gates already receive the pair.
`check-producer-liveness` ignores every argument past the first by its own stated contract
(`producer_liveness.rs:143-145`), and `check-evidence-manifest` reads argument 0 through
`knob_or` (`evidence_manifest.rs:59`). No reader changes.

### (3) The widening of `check-producer-liveness` is refused, and the ground is a section this cut may not amend

Recorded as a delta rather than dropped, because the next reader meeting the same gap
will reach for the same answer {design-bearing}. The tempting fix is to make the gate's
argument-free invocation read **both** corpora — mint a run-dir knob beside the lock
knob — so no entry needs an argument at all.

**It contradicts the owning section in terms.** §check-producer-liveness rules at
`:1045-1049` that *"The two modes are told apart by the argument being a directory, not by
a flag: the caller already knows which it holds"*, and a no-positional invocation reading
both is precisely a caller that no longer tells them apart. `:1051-1056` rules the
`.run`/`.lock` split *"load-bearing rather than cosmetic"* — a lock's absence means
*free*, a launch record's absence means *nothing was recorded*, never *nothing is running*
— and `:1206-1212` rules that *"the lock-pointed entries stay beside the set entries
rather than being replaced by them"*. What the workaround treats as an accident is a
stated design with its grounds written down.

**And it is out of bound, which is the composer question rather than a preference.** That
is §check-producer-liveness, a different section from this cut's stated-contract bound; a
port cut may not reshape a gate's asserted behavior so the front end need not carry a
capability it is deleting. **It would also not have worked**: no knob addresses the
appended `<queue> <state>` of delta (2), so the naive re-point refuses either way.

### (4) The nine entries and the file's `# spec:` are rewritten together

`scripts/lifecycle-config.sh:14` carries a `# spec:` whose text *is* the old routing —
*"every entry names a gate and reaches it through scripts/gate-exec.sh, never a declaration
path"* — and `:16-24` are the entries {mechanical}. Both change in one edit, because the
comment is a contract citation bound to the lines beneath it and leaving it while
re-pointing them is the restatement drift CLAUDE.md's comment doctrine bars.

**The constraint that comment states survives verbatim and is why the form works.**
`--enter-stage` execs the configured argv with **no interpreter word**
(lifecycle-kit/SPEC.md §bin/enter-stage.sh, `enter_stage.rs:1252-1254`), so the entry's
first token must ride its own exec bit. `gate-sdk/bin/run-gates.sh` is `100755` with
`#!/usr/bin/env bash`, so it is directly exec-able and the entry names it with no `bash`
word — the same shape `scripts/gate-exec.sh` had. The knob is **still** not taught to
resolve a name; the entry names a front end and a gate, exactly as before.

**The entry grammar carries the separator without a change.** Each `<command>` is split on
whitespace with no quoting (`enter_stage.rs:537`), so a bare `--` is one token like any
other. The pre-existing limit is unchanged and unreached: an argument containing a space
still cannot be expressed, and all three corpora are space-free.

### (5) The two `.claude/settings.json` grants drop in the landing commit, and no grant is added

`.claude/settings.json:23-24` name `scripts/gate-exec.sh` in the bare and argument forms
{mechanical}. Both are deleted **in the same commit as the file**, by
`native-gate-port-remaining-corpus`'s ruling (2) as widened by the operator on 2026-09-05:
removing a grant whose target a ruled cut deletes is a pure narrowing, outside the
2026-08-22 bar, and build edits the file itself with no confirmation and no out-of-band
step. That widening is **scoped to decommissioning bash gates** and is not a licence over
that file; nothing else in it is touched.

**No grant is added, which is worth stating because a re-point looks like it needs one.**
`.claude/settings.json:12` already grants `Bash(bash gate-sdk/bin/run-gates.sh *)`, which
covers `--only <gate> -- <args…>` for a session invocation — probed, per ruling (2)'s own
instruction to probe the count rather than assume it. The pre-flight path needs no grant
at all: `--enter-stage` spawns the entry directly rather than through the Bash tool.

### (6) The resolution-failure obligation transfers rather than lapsing, and it was verified rather than assumed

§check-evidence-manifest's `:852-867` puts an obligation on **the front end**: call
`gate_command` through a command substitution, keep its status, name the
resolves-in-no-check-dir case, and propagate any other non-zero status **without adding a
second sentence** {design-bearing}. `gate-exec.sh:15-28` is where that obligation is
currently discharged, and it leaves the tree.

**`:845-847` asserts the replacement already carries it, and this cut checked rather than
relayed the claim.** `native/src/runner.rs:266-275` reports *"listed in … but resolves in
none of:"* for a member resolving nowhere, and `:286`, `:309` and `:322` report *"dispatch
harness error, exit 2"* for a member that resolved and could not be run. The two
`gate_command` signals are kept apart in the crate exactly as the obligation requires. The
paragraph moves from naming this repo's script as the discharge to naming the arm.

**One difference is real and is not load-bearing at this caller, which is stated so a
reader does not go looking for a regression.** `gate-exec.sh` exits **2** on a resolution
failure; the `--only` run exits with the aggregate's non-zero status. The pre-flight caller
reads `out.ok` — zero versus non-zero — and nothing else (`enter_stage.rs:541-544`), so
both refuse the entry identically. No other caller of these entries exists.

## Producers and consumers

This cut introduces **no new state, no new field, no new knob and no new file**. It
deletes one adapter and re-spells nine configuration values onto a channel a sibling cut
builds. The survey is over every reader of the deleted script and of the knob whose values
change, run by grepping every tracked file for `gate-exec` with stderr left open.

**The interface whose producer moves: the pre-flight entry's resolved argv.**

- **Producer, before:** `scripts/lifecycle-config.sh:16-24` names `scripts/gate-exec.sh`;
  that script resolves the gate name through `gate_command` over `gate_check_dirs` and
  `exec`s the resolved argv with its own trailing arguments appended (`:14-29`).
- **Producer, after:** the same knob, naming `gate-sdk/bin/run-gates.sh --only <gate> --
  <args…>`; the front end resolves the same name through the same registry, and
  `run-gates-stub-cut` delta (2)'s channel carries the arguments to the selected member.
- **Consumer:** `run_preflight_command` (`native/src/emit/enter_stage.rs:1255-1265`),
  which spawns argv[0] with the rest plus `<queue> <state>` and reads the status.
- **Enabling config, actually emitted:** `scripts/lifecycle-config.sh` is this repo's own
  seeded config copy and is sourced by the bridge on every run, so the re-pointed values
  are live for every stage entry the moment they land. The kit-side default for
  `LIFECYCLE_KIT_ENTRY_PREFLIGHT` is empty (lifecycle-kit/SPEC.md `:656`) and stays empty:
  nothing here becomes kit content.

**Every reader of the deleted script, surveyed, with its disposition:**

| reader | site | disposition |
| --- | --- | --- |
| the nine entries and their `# spec:` | `scripts/lifecycle-config.sh:14,16-24` | re-pointed, delta 4 |
| the two permission grants | `.claude/settings.json:23-24` | deleted in the landing commit, delta 5 |
| the front end's own binding | `evidence-kit/SPEC.md:827-841` | re-worded, below |
| the port disposition | `evidence-kit/SPEC.md:843-850` | gains the argument sentence, below |
| the resolution-failure obligation | `evidence-kit/SPEC.md:852-867` | re-attributed, delta 6 |
| a bare cross-reference | `evidence-kit/SPEC.md:1014` | re-worded, below |
| the lapsed-premise discussion | `delegation-kit/SPEC.md:836` | present tense, re-worded below |
| a precedent citation | `delegation-kit/SPEC.md:1551` | historical, left as written |
| consumer-front-end cross-references | `gate-sdk/SPEC.md:463,642,1009,2183-2184` | re-worded where present tense |
| ruling narrative | `TASK-QUEUE.md` (ten sites) | left as written — a queue entry narrating what was true when it was filed is the historical record, not a stale citation |
| generated mirrors | `docs/gate-sdk/SPEC.md`, `docs/evidence-kit/SPEC.md` | regenerated, never hand-edited |

**Named reader for the one thing that could look like a new field.** The `--` token is not
a new interface of this kit: it is `run-gates-stub-cut` delta (2)'s grammar, read by the
crate's `--only` parser at one transition — selection. This cut is a caller of it and
declares no reader of its own.

## Existing sections updated

- `evidence-kit/SPEC.md` §check-evidence-manifest, the paragraph beginning **"A pre-flight
  caller names this gate, never its declaration path"** (`:827-841`) — it names
  `scripts/gate-exec.sh` as the discharge in the present tense (deltas 1, 4). The
  no-interpreter-word constraint and the wired-for-the-whole-roster rule stay verbatim:
  they are what make the new form legal, not what the new form replaces.
- `evidence-kit/SPEC.md` §check-evidence-manifest, the **port-disposition** paragraph
  (`:843-850`) — the assertion that *"every pre-flight entry re-points to `<front-end>
  --only <gate>`"* is silent on the argument, which after the lead's ruling is
  load-bearing. It gains the sentence that the entry's argument rides the re-pointed form
  through the `--` separator, and cites gate-sdk/SPEC.md §run-gates for the grammar rather
  than restating it (delta 2).
- `evidence-kit/SPEC.md` §check-evidence-manifest, the **resolution-failure obligation**
  (`:852-867`) — re-attributed from this repo's script to the arm that now discharges it,
  with the rule itself unchanged (delta 6).
- `evidence-kit/SPEC.md:1014` — the bare cross-reference naming the deleted script
  (delta 1).
- `delegation-kit/SPEC.md:836` — *"(this repo's `scripts/gate-exec.sh`)"*, inside §The
  turn-end liveness hook's lapsed-premise discussion (delta 1). The sibling
  `liveness-reader-cut` rewrites the paragraphs around it, so the two cuts touch one
  section and a build session lands them in one edit rather than two.
- `gate-sdk/SPEC.md:463,642,1009,2183-2184` — the consumer-front-end cross-references
  (delta 1). Most describe the front end's *role* and read correctly once the file is
  gone; the pass confirms none asserts a present-tense fact about the deleted script.
- `gate-sdk/SPEC.md` §run-gates — cited, **not** amended here: the `--only` argv grammar is
  `run-gates-stub-cut` delta (2)'s and is stated once (delta 2).
- `scripts/lifecycle-config.sh:14` — the `# spec:` whose text is the old routing (delta 4).

<!-- update-target-exempt: the deleted file's own header and spec pointer are removed with the file, so no delta can claim them as surviving targets -->
- `scripts/gate-exec.sh` — deleted whole.

<!-- update-target-exempt: generated projections regenerate mechanically from their sources and are never hand-edited; the freshness gate is the reader -->
- `docs/gate-sdk/SPEC.md`, `docs/evidence-kit/SPEC.md`, `docs/delegation-kit/SPEC.md` —
  re-run `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

## Definition of Done

- [ ] **The sibling's channel lands first** — `run-gates-stub-cut` delta (2)'s `--only …
      --` forwarding exists and refuses on a multi-member selection **before** any entry is
      re-pointed. Landing this cut first refuses every stage entry in the repo.
- [ ] **Causal completeness** — the re-pointed argv has a named producer
      (`scripts/lifecycle-config.sh`) and a named consumer (`run_preflight_command`); no
      new field is added; every reader of the deleted script is dispositioned above.
- [ ] **The entries are exercised, not just edited** — a stage entry is taken with the
      re-pointed roster live, and both corpora are proved reachable: a `*.run` record under
      `.tmp` refuses, and a held `run-validate.lock` refuses, each at its own entry.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section (not appended); §check-evidence-manifest reads as one document
      to a reader who never saw this amendment.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls evidence-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every tracked file grepped for `gate-exec` with stderr
      open; nothing dangles outside the queue's own historical record, and the docs mirrors
      are regenerated rather than hand-edited.
- [ ] **The oracle re-read, not the arithmetic trusted** —
      `bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree` re-run at the landing
      commit and its owed count recorded.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed to the
      committed gap inbox.
