# SPEC amendment: gitignored-append-grant

Builds the grant `session-mechanic-grants-uncommitted` was re-classed
feature-class for: the resume-journal append every dispatched session is
*mandated* to make (delegation-kit/SPEC.md §Resume journal — agent writes,
scratch reset sweeps) is granted by nothing committed, and the spelling ruled to
grant it — `Bash(cat >> .tmp/*)` — is **inert**, because a `Bash(...)` rule grants
the command and never the redirect **target**.

**The design is the operator's, ruled 2026-08-22 through the lead, and this
amendment calibrates rather than re-decides it:** a `PreToolUse` guard rule
running `git check-ignore` on every redirect target and granting only when all of
them are ignored, with `guard_rule_truncate_scratch` (guard-kit/lib/guard.sh:784,
generic rule 16) as the shipped precedent — that rule already does exactly this
for the `: >` truncation form. What the amendment adds is the part a one-sentence
ruling cannot carry: **which commands the grant may bless**, because a grant
blesses the command as well as its target.

**Why not `Edit(/.tmp/**)`, restated only so the merge does not re-reach for it.**
That rule would grant the target and is anchored so it provably cannot reach a
tracked path — but a redirect-target check is a **file-write** check and cannot
tell `>>` from `>`, so it grants the truncating form too. The append-only split
ruled 2026-08-20, so that a mistyped redirect cannot destroy a journal, is
inexpressible in **any** settings rule. Only a hook honours what was ruled.

## What changes

### (1) A new generic rule: auto-allow an append-only write to gitignored targets

`lib/guard.sh` gains `guard_rule_append_scratch`, dispatched **immediately after
`guard_rule_truncate_scratch`**, and `SPEC.md` §The generic ruleset gains it as
**rule 17**, renumbering the five rules below it. **Design-bearing.**

Granted silently — `guard_allow` — when **all four** hold, and falling through
untouched otherwise:

- **(a) Append-only.** Every redirect operator in the command is `>>` (an fd
  prefix allowed: `2>>`). A single truncating `>` **anywhere** refuses. This is
  the clause that carries the 2026-08-20 ruling and the one no settings rule can
  express.
- **(b) Every target gitignored.** Each redirect target satisfies
  `git check-ignore --quiet --`. Rule 16's exact predicate and its exact
  subprocess, gated behind the same rarity — the rule reaches `git` only once (a)
  and (c) have already matched.
- **(c) The leading command emits to stdout and nothing else.** The command is a
  **single statement** — `guard_split_compound` yields one segment — whose leading
  word is on `GUARD_KIT_APPEND_BINS` (delta 2).
- **(d) Conservative decline on anything unmodelled.** A backtick anywhere, or a
  redirect target that survives normalization carrying a quote, declines outright.
  Declares `sq dq hd`.

**Two inert targets are exempt from (a) and (b), and the carve-out is
precedented rather than invented:** `/dev/null` and an fd-dup (`&1`, `&2`) are
what rule 17 — rule 18 after the renumber — already treats as targets that are
not files. Neither is a file the append-only split protects and neither is a
path `git check-ignore` can answer about, so refusing on them would fragment the
grant on a shape the ruleset already blesses. Every **other** target takes both
tests.

**Why (c) is the delta's real design content.** A grant of `permissionDecision:
allow` blesses the whole call, so a rule keyed on the redirect alone would grant
`rm -rf .tmp/../.git >> .tmp/j.md` — the target is gitignored, the operator is
`>>`, and the command destroys the repository. Bounding the *emitter* is what
makes the grant safe, and bounding it to **stdout-only** emitters is what makes
the bound checkable: `cat`, `printf` and `echo` write nowhere the redirect does
not send them, so the redirect target is the whole of what the call can touch.
`tee` is deliberately **off** the roster despite being the obvious fourth member:
it takes a path **argument**, so `tee -a <tracked file>` writes a tracked file
with no redirect at all and (a) and (b) never see it.

**No expansion arm is needed and its absence is checked, not assumed.** Rule 6
blocks every `${…}` / `$(…)` / `<(…)` / `$NAME` and exits 2 before this rule runs,
and it declares `sq hdq` — so an **unquoted**-delimiter heredoc body stays live
for it and an expansion inside one is blocked before reaching here. Backticks are
outside rule 6's regex, which is why (d) carries its own arm, on the pattern rules
18 and 21 (post-renumber) already use.

### (2) `GUARD_KIT_APPEND_BINS` — the emitter roster

A new array knob, default `("cat" "printf" "echo")`, declared in `lib/guard.sh`
beside `GUARD_KIT_RO_BINS` under the same `declare -p … ||` guard and rostered in
§Layout and configuration. **Mechanical.**

A knob rather than a kit literal on the established convention: `GUARD_KIT_RO_BINS`
is the same shape for the same reason, and a consumer whose mandated write rides a
different emitter shadows the array instead of forking the rule. It carries **no
private rule content** — three POSIX utility names are shipped mechanism, not a
vocabulary — so the provenance seam is not crossed by making it a kit default.

**The widening hazard is inherited and stated**, because §The generic ruleset
already states it for `GUARD_KIT_RO_BINS`: a consumer that adds a
non-stdout-only command to this roster widens the grant past delta (1)(c)'s
safety argument. The roster's contract is *writes only to its stdout*, and that
sentence is what a consumer edits against.

### (3) The rule's own SPEC section, and what it is honest about

§The generic ruleset's new rule 17 entry states the four clauses, the two-target
carve-out, the emitter contract, and **one limit**: the grant is exactly as safe
as the roster's stdout-only property, which the rule asserts of the roster and
cannot verify of a member. **Design-bearing.**

That is the same honest bound rule 17 already carries for `GUARD_KIT_RO_BINS`
(*roster membership does not by itself prove an invocation read-only —
`sort -o` writes a file*), reached from the writing side rather than the reading
side, and it is stated rather than left for a later reader to discover.

### (4) The five rules below it renumber, and the one number-bearing citation moves

Rules 17-21 become 18-22. **Mechanical.**

Renumbering is precedented and already anticipated by the ruleset: rule 19's own
text records that its number moved when rules 12, 13, 14 and 15 were each
inserted ahead of it, and that DOCTRINE.md is therefore cited **by name and never
by number** for exactly this reason. The internal cross-references that *do* carry
numbers — rule 17's placement note, rule 18's *placed after the auto-allow rules
(16, 17)*, rule 19's *after (16, 17) and (18)*, rule 20's *on rule 18's
reasoning*, and §The guard framework's per-rule class list — are enumerated in
§Existing sections updated and move with them.

### (5) The decision table gains the pair, and every existing case is re-derived

`guard-tests/cases.tsv` gains a firing case and a non-firing case for the new
rule, and — the obligation that is easy to miss — **every existing case whose
command carries a heredoc or a redirect has its expected column re-derived.**
**Design-bearing.**

§Testing rules this explicitly and rules why: the table's red condition is **not
monotone**, because it fails on a verdict mismatch in *either* direction, and this
delta **narrows refusal**. A narrowing cannot be cleared by inspection. Nor can
`bin/scan-prompts.sh` substitute: `guard_block` exits 2 before
`guard_log_fallthrough` runs, so the scan is blind to every block, and a change
converting a fall-through into a grant reads there as a pure improvement.

The pairs, each written with §Testing's two substitutions (`@ROOT@` for the git
sandbox root, `@NL@` for a newline, without which a heredoc case is inexpressible
in a tab-separated cell):

| expected | case |
|---|---|
| `allow` | `cat >> @ROOT@/.tmp/journal.md <<'EOF'@NL@line@NL@EOF` — the mandated form |
| `fallthrough` | the same command with `>` for `>>` — clause (a), the ruled split |
| `fallthrough` | `cat >> @ROOT@/tracked.md <<'EOF'…` — clause (b) |
| `fallthrough` | `rm -rf @ROOT@/.tmp/x >> @ROOT@/.tmp/journal.md` — clause (c), the shape the grant would otherwise bless |
| `allow` | `printf 'x@NL@' >> @ROOT@/.tmp/journal.md 2>/dev/null` — the inert-target carve-out |

### (6) The wait-loop grant is a stated non-target, and the reason is a sequence

The entry's **third** member of the class — the mandated in-turn wait, a
`kill -0 "$pid"` loop that a `Bash(...)` glob cannot match because the mandated
form *is* a loop condition and so is decorated by construction — is **not**
granted here, and this delta records the refusal rather than leaving it as a
silence. **Design-bearing.**

Its 19 measured out-of-band calls are real and the entry's argument is sound: the
hook is the only route for that shape too. What blocks building it *now* is
`turn-end-chokepoint-and-wait-primitive`, promoted this same iteration, whose
measurement half asks **which wait primitive is reliable on this machine**. A
grant minted around the currently-sanctioned loop form would be a rule shaped to a
primitive that measurement may correct — *a guard on the wrong primitive inherits
its failure*, in that entry's own words. Sequence: measure, then grant. Building
both this iteration would either bake the ordering the sibling unit is testing, or
force the grant to be re-authored inside the same build.

## Producers and consumers

**`guard_rule_append_scratch`** (new rule function; new state: a
`permissionDecision: allow` verdict on a command class that previously reached the
permission prompt).
*Producer:* the consumer's `templates/bash-guard.sh` copy, at the generic-ruleset
dispatch block in `lib/guard.sh`, on every `PreToolUse(Bash)` firing — an
**already-enabled** path, since every rule 1-21 runs from it today and this repo's
`.claude/settings.json` already registers `scripts/bash-guard.sh` on that event.
No new hook registration, no new event, nothing to configure for the producer to
run.
*Consumer:* the **harness**, which reads the emitted `permissionDecision` JSON on
stdout and skips the out-of-band decision — the same consumer and the same
mechanism `guard_rule_truncate_scratch` already reaches, which is what makes that
rule the precedent rather than an analogy.
*Second consumer:* `bin/run-guard-tests.sh`, through the `cases.tsv` rows of
delta (5), at the transition where it asserts the exit code and output class.

**`GUARD_KIT_APPEND_BINS`** (new array knob).
*Producer:* the `declare -p … || GUARD_KIT_APPEND_BINS=(…)` guarded assignment at
the head of `lib/guard.sh`, which a consumer shadows by assigning before the
library is sourced — the exact mechanism `GUARD_KIT_RO_BINS` and
`GUARD_KIT_SCRATCH_DIRS` already use, so the enabling path is live for every
existing consumer with no edit.
*Consumer and named reader:* `guard_rule_append_scratch`'s clause (c), at the
single transition where it tests the leading word of the command's one segment.
It has **exactly one** reader and no other; a second reader would be the
widening delta (2) warns against.

**The grant's verdict** (new value on an existing channel — no new field).
*Named reader:* the harness, at the permission decision, and
`bin/run-guard-tests.sh`, at the decision-table assertion. No field is added to
the hook JSON — `guard_allow` already carries a reason string and this rule
supplies one.

**Red conditions, because delta (1) narrows a refusal set.** Two readers, and
neither is monotone, which is why both are named and neither is cleared by
inspection:

- **`bin/run-guard-tests.sh`** reds on **a verdict mismatch in either
  direction** — not on a violation count. A case that previously fell through and
  now grants reds until its expected column is re-derived. This is the reader
  delta (5) exists for and the reason its re-derivation sweep is a delta rather
  than a note.
- **`bin/scan-prompts.sh`** reds on nothing and is named so it is not mistaken for
  the oracle: it ranks the friction log, `guard_block` exits before the logger
  runs, and a narrowing therefore reads there as a strict improvement whether or
  not it is one.

**Existing integration prose describing the prior flow**, surveyed across the
whole component set rather than a hand-picked subset — `grep -rn` over every
tracked `SPEC.md`, `CLAUDE.md`, `DOCTRINE.md`, template and agent definition for
the rule numbers 17-21, for `GUARD_KIT_RO_BINS`, and for the journal-append
mechanic, with **stderr unsilenced** so a bad path reads as an error and never as
*no reader*. What it reaches is inventoried below.

## Existing sections updated

Each names the delta that owns it.

- **guard-kit/SPEC.md §The generic ruleset** — the new rule 17 lands between the
  truncation grant and the read-only pipeline grant, with its four clauses, its
  two-target carve-out and its honest limit (deltas 1, 3); rules 17-21 renumber to
  18-22 and every internal number-bearing cross-reference among them moves —
  rule 18's *placed after the auto-allow rules (16, 17)*, rule 19's *after (16, 17)
  and the decorated-allowlist rule (18)*, rule 20's *on rule 18's reasoning*, and
  rule 19's own renumber-history sentence, which gains this insertion (delta 4).
- **guard-kit/SPEC.md §Layout and configuration** — `GUARD_KIT_APPEND_BINS` joins
  the knob roster beside `GUARD_KIT_RO_BINS`, with its default and its
  stdout-only contract (delta 2).
- **guard-kit/SPEC.md §The guard framework (`lib/guard.sh`)** — the paragraph
  enumerating which rules read the **raw** command versus a skeleton names rules
  by number and is re-read at merge against the renumber; the new rule reads a
  skeleton and joins no such list, so the expected edit is the number shift alone
  (deltas 1, 4).
- **guard-kit/SPEC.md §Testing** — the decision table's new rows and the
  non-monotone re-derivation obligation this change triggers (delta 5).
- **guard-kit/lib/guard.sh** — the rule function, the knob's guarded default, and
  the dispatch line in the generic-ruleset block, whose **order is load-bearing**
  and whose position is fixed by delta (1): after `guard_rule_truncate_scratch`,
  before `guard_rule_ro_pipeline`. Placing it after the decorated-allowlist rule
  would be wrong for a stated reason — that rule blocks a bare allow entry
  decorated by a trailing redirect, which is one plausible spelling of the very
  command this rule grants (deltas 1, 2).
- **guard-kit/guard-tests/cases.tsv** — the five rows, plus the re-derived
  expected column of every existing heredoc-bearing or redirect-bearing case
  (delta 5).
<!-- update-target-exempt: the split is queue-kit/SPEC-entry-split.md's delta (4), owned there rather than restated as a delta here -->
- **TASK-QUEUE.md** — `session-mechanic-grants-uncommitted`'s unruled
  overlay-only-oracle half splits into its own deferred entry, under the split
  the lead authorized and the criterion `queue-kit/SPEC-entry-split.md` mints;
  the two entries carry that amendment's bidirectional citation.
- **`.claude/settings.json`** — **no edit**, and the clause exists so the merge
  does not add one. The previously ruled `Bash(cat >> .tmp/*)` entry is inert and
  was never committed; adding it now would ship a no-op and then measure no
  improvement, which is the finding this entry exists to keep from being bought
  twice. A settings edit is also operator-class under the 2026-08-22 ruling
  (TRAJECTORY.md §The closed rulings), and this delta's whole point is that the
  design does not need one (delta 1).
- **delegation-kit/SPEC.md §Resume journal — agent writes, scratch reset
  sweeps** — the mandating contract, re-read at merge and edited only if it
  asserts the append is ungranted; no edit is planned, and the clause is here so
  the check is not skipped (delta 1).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls guard-kit/SPEC-*.md`), discharged at the iteration rather
      than at the commit.
- [ ] **Removals propagated** — grepped every spec, template and agent definition
      for the rule numbers this change shifts; nothing cites a stale one.
- [ ] **Acceptance measured, not asserted** — the granted form costs no
      out-of-band decision, checked by driving the consumer's guard with the
      mandated command under a scratch `GUARD_KIT_LOG` (§Testing's convention, so
      the probe does not file itself as real friction) and reading the emitted
      decision.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
