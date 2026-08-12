# SPEC amendment: bin-argv — argument shape, and the rename verb

Two recurrence riders share one surface: the argument-and-verb grammar of the
kits' `bin/` tools. `capture-affordance-help-flag` is a tool accepting an
argument it should have refused; `scope-rename-guard-deadlock` is a tool
*missing* a verb, so its contract has no unblocked path. They are batched
because the third firing of the second was **caused by** the first: the
2026-08-12 scope session ran `bash lifecycle-kit/bin/enter-stage.sh --help`
looking for a rename mode, and got `'--help' is not a lifecycle stage` instead
of the usage text that would have told it. One tool's missing help handler is
what sent that session three guards deep.

## What changes

### (1) The defect class, stated once where a fifth tool's author finds it — **design-bearing**

`gate-sdk/SPEC.md` gains a `bin/`-tool authoring rule under the cross-kit
conventions it already owns (it governs `*/bin/*.sh` tree-wide at
§check-exec-bit, and CLAUDE.md §Conventions established in gate-sdk names it the
home of a convention every kit keeps). The rule:

> **A `bin/` tool whose positional arguments are free text validates their
> shape, not only their arity.** Free text is an uninterpreted caller-supplied
> string; an argument drawn from a known set is not free text, because
> validating membership already validates shape. Three behaviors:
>
> - `-h` / `--help` as the first argument prints the tool's usage on **stdout**
>   and exits **0**. Usage on a successful help request is output, not a
>   diagnostic.
> - A positional argument beginning with `-` that is not a recognized option is
>   a **refusal** — usage on stderr, exit 2.
> - `--` ends option processing, so every remaining argument is taken as free
>   text however it is spelled.
>
> An arity check is not a shape check. A tool taking exactly one free-text
> argument accepts `--help` as that argument, and a tool taking two accepts a
> flag in either slot; arity makes the single-argument case worst and the
> others merely quieter, never safe. A tool that *captures* — that appends its
> argument to a durable surface — turns the defect into a written record that
> reads like a real filing, so the rule binds hardest there, but it binds on
> every free-text tool: the help half is discoverability, and delta (3) below
> exists because discoverability failed.

The `--` escape is not decoration: without it the refusal makes a legitimate
filing unfileable, and this very entry's prose ("a `--list` argument was
captured") is an instance. `bash lifecycle-kit/bin/file-gap.sh -- "--list is
captured at exit 0"` is the sanctioned spelling.

**Ruled out: a new gate.** `check-exec-bit`'s corpus is the whole `*/bin/*.sh`
set, and a gate over it could assert only the weak static shape (does the file
contain a `--help` branch), which passes a tool that prints usage and still
captures `--list`. The predicate that matters is behavioral, and the precedent
for behavioral coverage of a `bin/` tool is already ruled and shipped:
lifecycle-kit/SPEC.md §bin/enter-stage.sh rules `--simulate` "advisory tooling,
not a gate: no fixture pair is owed; it is exercised end-to-end in
`smoke/install.sh`". Delta (5) follows that precedent. A new shell gate would
also run against this repo's standing port directive (TRAJECTORY.md), and
enforcement-first's own clause — removing the duplication outranks gating it —
points at one stated rule plus behavioral coverage rather than a static scanner.

### (2) The member roster, including the two the queue entry did not name — **mechanical**

**The scoping this census ran under was wrong, and that is worth stating before
the count it produced.** The table below was built from the survey record's
2026-08-12 `spec` census, whose own corpus line reads "every kit's `bin/`
directory across all **seven** vendored kits, plus `scripts/`." The repo carries
ten `*/bin/` directories; nine of them belong to kits — `installer/` is the
tenth and is not one, by the same no-`checks/`-no-`smoke/` predicate CLAUDE.md
states for `native/` and repeats for `installer/`. Seven undercounts the
kit-owning set by two. **Any other census carrying this survey's seven-kit
scoping forward is suspect on the same ground** and should be re-run against the
full nine before being trusted.

The miss this undercount produced is not academic: align's audit found a sixth
member the census table below does not carry, `gate-sdk/bin/run-gate-tests.sh`
— free-text `TESTS_DIR="${1:-…}"` and `GATE_DIRS=("${@:2}")`, gated by `$# -gt 1`
alone, no `-h`/`--help` anywhere in the file. **The deliverable stays at five** —
operator-ruled scope-gated intake's default files a mid-iteration finding as a
costed Deferred entry rather than starting it, and widening this amendment's
roster is exactly that starting. The sixth member is filed, not folded in:
`.workflow/survey-record.md`'s 2026-08-12 `align` block and the matching
`.workflow/gap-inbox.md` bullet carry it for a future cohort.

The five below were re-verified at the read site during align, independent of
the seven-kit census that first produced them, and hold:

| tool | today | class |
| --- | --- | --- |
| `lifecycle-kit/bin/file-gap.sh` | `$# -ne 1` | captures; three attested firings |
| `drift-kit/bin/kfric.sh` | `$# -ne 2` | captures; a lone flag fails arity, a flag in either slot does not |
| `lifecycle-kit/bin/file-survey.sh` | `$# -ne 4` | captures; same accidental safety as `kfric.sh` |
| `lifecycle-kit/bin/cite-survey.sh` | `$# -ne 1` | reads; `--help` returns "no block heading contains: --help" |
| `lifecycle-kit/bin/enter-stage.sh` | membership-validated | exempt from the refusal half, **not** from the help half |

Two of these are census findings rather than firings. `cite-survey.sh` carries
`file-gap.sh`'s exact `$# -ne 1` shape and has simply not been run with a flag
yet; its blast radius is smaller (it writes nothing) but its help behavior is
the same misleading error. `enter-stage.sh` needs no refusal — an unknown
argument already fails `lifecycle_stage_known` — but it is the one member whose
missing help handler has a *measured* cost, in this amendment's own opening
paragraph. Each member gains the preamble **before** its arity check, reusing
the `usage()` every one of them already defines.

`file-survey.sh` and `kfric.sh` take more than one argument, so their refusal
scans every positional, not only the first.

### (3) `enter-stage.sh --rename <name>` — the mechanized iteration rename — **design-bearing**

The naming step in lifecycle-kit/templates/stages/scope.md is a **two-surface**
write: the queue header's `—` placeholder and column 1 of the `scope` stamp in
`.workflow/WORKFLOW-STATE.txt`, which `check-stage-evidence` requires to agree.
The second surface is guarded (`workflow-state-guard` blocks Write/Edit;
`bash-guard` blocks `sed -i` and steers back to Edit), so every scope session
has completed a documented contract by routing around a guard — three
consecutive times, each re-inventing the workaround.

The tool gains a mode:

```
enter-stage.sh [--simulate] --rename <name>
```

- **It writes both surfaces in one motion.** The queue header becomes
  `## Iteration: <name>` and column 1 of **every** data line in the state file
  becomes `<name>`. Rewriting every line rather than only the last is correct by
  the boundary invariant, not by convenience: the first-stage entry truncates the
  state file, so every data line belongs to the current iteration by
  construction — which is exactly what `check-stage-evidence` asserts. It also
  heals a half-landed hand-rename, where one surface moved and the other did not.
- **The stage cursor is untouched, and this is not stage motion.** No stamp is
  appended and no stage token is written, so `stage motion never writes the
  queue` is not weakened: a rename is not a transition. The precedent for the
  tool writing the queue header at all is the boundary reset, which already does.
- **The columns-2-4 witness.** Before the write, the tool asserts that the state
  file's fields 2 through 4 — stage, session id, date — are byte-identical
  before and after the rewrite. This is the *content* predicate the queue entry
  ruled "the harder guard to write correctly", applied by the **writer** instead
  of the guard, where it is cheap and exact: the writer knows which field it
  meant to touch, and the guard would have to infer it from a diff.
- **Pre-flight, same contract as the stamp path.** The candidate header and
  candidate state file are built as temporaries, `check-stage-evidence.sh
  <tmpqueue> <tmpstate>` runs against them, and a non-zero exit refuses with the
  gate's output relayed and **nothing written** — the identical posture
  `check-stage-entry` already gets. `--simulate --rename <name>` relays what
  would change, prefixed `enter-stage (simulate):`, and writes nothing.
- **Refusals** (exit 2, nothing written): `<name>` empty; `<name>` not matching
  the queue slug grammar `[a-z0-9][a-z0-9-]*` — whitespace is the corrupting
  case, since column 1 is whitespace-delimited and a two-word name silently
  shifts every field, but the full slug grammar is what every iteration name in
  this repo's history already satisfies and what commit subjects and the
  release-disposition record consume; `<name>` equal to the unnamed placeholder,
  which only the boundary reset may write.
- **Idempotent**: when the header already reads `<name>` and every stamp's
  column 1 already is `<name>`, it reports and exits 0 without writing, matching
  the stamp path's idempotence.
- **The report** names both written files and tells the caller to commit them
  together — the coupling scope.md currently states as prose ("they ride in one
  commit") becomes a property of the writer.

**Ruled out: a separate `rename-iteration.sh` tool.** It would add a second
sanctioned writer of the state file, and one-writer is the property
`workflow-state-guard`'s own block message asserts and the whole reason the
guard exists. The mode rides the writer.

**Ruled out: exempting the guard on a content predicate.** The queue entry
offered this as option two and called it the harder one. It is harder for a
reason that does not go away: a `PreToolUse` hook sees the *proposed* content,
so proving "no stage token moved" means reconstructing the pre-edit file and
diffing field-wise inside a hook — the same computation delta (3) does, in the
place with less information and no ability to refuse cleanly. Option one is
taken.

### (4) The guard message names the mode — **mechanical**

`lifecycle-kit/templates/workflow-state-guard.sh` and its consumer copy
`scripts/workflow-state-guard.sh` (held in lockstep by `check-shim-restatement`)
gain the rename spelling in the block text, beside the stamp spelling already
there. This is the discoverability half: the guard is the surface a blocked
session is looking at, and today it names only the writer for the operation the
session is *not* performing.

### (5) Coverage — **mechanical**

Behavioral, following the `--simulate` precedent rather than adding a gate.

- `lifecycle-kit/smoke/install.sh` gains, per member: `--help` exits 0 with
  usage on stdout; an unrecognized leading-`-` positional exits 2; and for the
  two capture members, **the durable surface is byte-unchanged after both** —
  the assertion that actually distinguishes this fix from a cosmetic one.
- `drift-kit/smoke/install.sh` gains the same three for `kfric.sh`.
- `lifecycle-kit/gate-tests/` gains the `--rename` cases the smoke cannot carry
  hermetically: both surfaces rewritten, fields 2-4 proved unchanged, the
  half-landed heal, each refusal (empty, non-slug, whitespace-bearing,
  placeholder), the idempotent no-op, and `--simulate --rename` writing nothing.

### (6) Queue disposition — **mechanical**

Both entries move to `## Done` at merge, dropping their `[spec:]` tags.
`capture-affordance-help-flag`'s deliverable is discharged in full including
its "decision on whether the convention becomes a gate-sdk authoring rule",
which delta (1) rules yes-with-a-narrowed-predicate.

## Producers and consumers

**The argv preamble** (deltas 1-2). *Producer:* each member tool's own argument
parse, running before its arity check — a code path every invocation reaches, no
enabling config, so there is no "set nowhere but in tests" exposure. *Consumer:*
the invoking session, via exit code and stream. Two behaviors, two streams, and
the split is the contract: exit 0 writes usage to **stdout** because a
successful help request's output is the answer; exit 2 writes it to **stderr**
because there the usage is a diagnostic accompanying a refusal. No new state and
no new field is introduced — the preamble adds exits, not data — so points 4 and
5 of the causal-completeness check have no subject here.

**The class rule** (delta 1). *Producer:* gate-sdk/SPEC.md prose. *Consumer:*
the author of the next `bin/` tool, and the two kit SPECs that cite it rather
than restate it (below). No gate reads it; that is delta (1)'s explicit ruling,
and the enforcement that does read it is delta (5)'s behavioral coverage.

**`--rename`** (delta 3). *Producer:* a `scope` session performing the naming
step in lifecycle-kit/templates/stages/scope.md — the one caller, and the
template is updated in delta "Existing sections updated" below so the producer's
instruction actually names the mode. *Consumers, all three named:*

- `check-stage-evidence` reads the header/stamp agreement the mode writes. Its
  **red condition** is a stamp whose column 1 differs from the header's name —
  which is precisely what a half-landed rename produces, so this consumer is the
  mode's own pre-flight rather than a downstream reader that might drift.
- `lifecycle_header_iter` (lib/stages.sh) reads the header on every later
  `enter-stage` invocation to compute `stamp_iter`, so a rename that moved only
  the state file would make every subsequent stamp disagree with itself.
- `LIFECYCLE_KIT_BOUNDARY_REQUIRE`'s check reads the closing iteration's name at
  the next boundary and matches it against the first token of a disposition line
  in `.workflow/release-disposition.txt`. Its **red condition** is *no line
  found* — a non-monotone, reds-on-finding-none verdict — so a name that
  changed after close stamped its disposition line reds the next boundary. The
  mode does not narrow any corpus, so point 5 does not otherwise bind here; the
  reader is named because the rename is the one operation that can invalidate a
  stamp already written, and the refusal on the unnamed placeholder is what keeps
  a rename from being used to un-name a dispositioned iteration.

**No new field** is added to the stamp line, the header, or any record. The
rename mutates an existing field in place; every reader of that field is
enumerated above.

**The guard message** (delta 4). *Producer:* the two hook copies. *Consumer:* a
blocked agent session, via `additionalContext`. `check-shim-restatement` is the
reader that holds the two copies in lockstep, and its red condition is a
divergence between them — monotone in the divergence set, so the paired edit
clears it by inspection.

## Existing sections updated

- **gate-sdk/SPEC.md** — the new `bin/`-tool argument-shape rule, owned by
  delta (1). Placed with the cross-kit conventions rather than in the gate model:
  it governs tools, and `bin/` tools are explicitly not gates.
- **lifecycle-kit/SPEC.md §bin/enter-stage.sh** — the `--rename` mode, its
  refusals, its witness, its pre-flight and its `--simulate` composition, owned by
  delta (3); plus the help-handler note owned by delta (2). This section already
  describes the tool's argument grammar and its `--simulate` mode, so the mode
  lands beside them rather than appended.
- **lifecycle-kit/SPEC.md §The committed gap inbox** and **§The survey record** —
  the argv contract for `file-gap.sh`, `file-survey.sh` and `cite-survey.sh`,
  owned by delta (2), stated as a citation to gate-sdk's rule, never a restatement.
- **drift-kit/SPEC.md §The knowledge-friction loop** — the same one-line citation
  for `kfric.sh`, owned by delta (2).
- **lifecycle-kit/templates/stages/scope.md** — the naming step names the mode
  instead of describing a two-surface hand-edit, owned by delta (3). Without this
  the producer's own instruction still points at the deadlocked path.
- **guard-kit/SPEC.md §The guard framework** — nothing changes; recorded here as
  a target *considered and not claimed*, because the guard's framework contract is
  untouched and only lifecycle-kit's instance's message text moves (delta 4).
- **CLAUDE.md §Housekeeping** — nothing changes (delta 2). The capture-affordance
  bullets name the tools and their arguments, not their flag handling, and remain
  true.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The two surfaces are proved, not assumed** — the `--rename` coverage
      asserts fields 2-4 byte-identical and both files written, and the capture
      members' coverage asserts the durable surface byte-unchanged after a
      refused flag. A test that only checks exit codes would pass the bug.
