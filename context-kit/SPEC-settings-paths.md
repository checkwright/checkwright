# SPEC amendment: settings-paths

A new gate, **`check-settings-paths`**, landing in **context-kit**: every
committed permission allow-list entry naming a literal repo-relative `.sh` path
resolves in the working tree.

**The class, re-measured at this stage rather than carried from the filing.**
Three array entries named two absent paths in `.claude/settings.json`:
`canon-kit/checks/check-comment-tier.sh` (entries at `:58` and `:59`, the bare
and `*` forms) and `lifecycle-kit/checks/check-stage-evidence.sh` (`:46`). Both
were **native-port residue** — each kit's `checks/check-X.sh` was replaced by a
`checks/check-X.gate` descriptor and the grant was never repointed. Of 81 allow
entries, 54 named a `.sh` path token; 46 of those were literal single paths and
44 resolved. The two that did not accumulated **one cohort apart**, which is the
entry's central claim and it holds: this is one to two dead entries per cohort,
unbounded across the 63 members still unported.

**Corrected at align: the measured class no longer exists on this tree.** The
operator pruned all three entries and granted `gate-sdk/bin/port-blockers.sh` in
the same pass, in `630e77fa` ("chore(settings): prune dead gate paths, grant
port-blockers"), landed by the lead on the escalation this amendment's spec
session raised — see delta 5 below, corrected to match. The gate's invariant,
manifest and fixture pair are unchanged by this: the class it defends against is
a standing property of the port (one to two dead entries per still-unported
cohort), not a one-time count, and `good/`/`bad/` fixtures carry synthetic
`settings.json` snapshots rather than a slice of the live file (delta 3), so
this measurement's staleness touches only the narrative above, not the gate's
design.

## The placement ruling

**It lands in context-kit as a new gate — not as a second mode on
`check-settings-pins`, and not in guard-kit.** The queue entry left this open;
the governing specs settle it, so it is ruled here rather than escalated.

**Why not extend `check-settings-pins`.** That gate is single-key jq-path
equality against an operator-curated pins file (`scripts/settings-pins.conf`,
three pins today, none touching `permissions`). The new predicate iterates an
array and probes the filesystem — a different assertion shape sharing only the
file it opens. Fusing them gives one gate two unrelated invariants and one
output contract describing both, and a reader of a red then has to establish
which gate they hit. What genuinely transfers is inherited instead: the same
`tier=precommit`, the same `couples=.claude/settings.json`, the same
fail-closed-on-absent-`jq` posture, the same fixture-pair shape.

**Why not guard-kit, despite it owning the parsing.** guard-kit holds the
tree's three existing `.permissions.allow[]` readers
(`bin/compare-settings-allow.sh`, `lib/guard.sh`'s `_guard_allow_inners`,
`bin/scan-prompts.sh`), which is the one real argument for it. It loses to two
others. First, guard-kit **ships no gates at all** — it has no `checks/`
directory, and its allow-list tooling is advisory *by a standing ruling*
(guard-kit/SPEC.md §compare-settings-allow): the subject is "a gitignored
per-machine file CI cannot see" and the verdict is "operator-intent-dependent".
That ruling does not forbid this gate — this predicate's subject is the
**committed** file and its verdict is a **binary filesystem fact**, so neither
clause reaches it — but it does establish the kit's shape, and this gate would
invert it. Second, those three readers are **tooling** readers at triage tier;
co-location by parsing technique is the weakest of the available arguments, and
context-kit already owns the committed settings file *as a gate subject* twice
over (`check-settings-pins`, `check-memory-off`). Widest-true-tier placement
puts an enforcement fact about a committed governed file with the kit that
governs it.

## What changes

### 1. `context-kit/checks/check-settings-paths.sh`

A new gate asserting: every entry in the committed settings file's
`permissions.allow[]` that names a **literal** repo-relative `.sh` path resolves
to an existing file. **[design-bearing]**

Manifest: `# graph: couples=.claude/settings.json,kit:checks/*.sh,scripts/check-*.sh dir=one valve=none tier=precommit`.
The settings file is the subject; the check-script globs are the **reverse
trigger** — a cohort deleting a ported gate's `.sh` is exactly the edit that
creates a violation, so it must re-run the gate. Recorded as a reverse trigger
in the manifest's own terms, since gate-sdk/SPEC.md criterion 4 turns on that
distinction and a later port reading this couple as content would misclassify
the member.

Knobs, per config-via-env: `CONTEXT_KIT_SETTINGS_FILE` — **reused, not
introduced**, since `check-settings-pins` already owns it with
`.claude/settings.json` as this repo's default.

### 2. The extraction predicate, scoped against five measured false-positive shapes

The gate extracts a path candidate only from an entry matching a `Bash(`
invocation whose command token is a `.sh` path, and **skips any token containing
`*`**. **[design-bearing]** The shapes it must not misfire on are measured on
this tree, not imagined:

- **Glob entries (8)** — `*/checks/check-*.sh`, `*/bin/run-*-tests.sh`,
  `*/gate-tests/*.test.sh`, `*/gate-tests/run-*-tests.sh`, `*/smoke/install.sh`,
  `*/smoke/violation.sh`, `drift-kit/kpis/*.sh`, `scripts/check-*.sh`. These name
  a *pattern*, intentionally polymorphic across present and future files. A
  predicate that stripped the `*` and stat'd the remainder, or that read an
  unmatched glob as dead, would false-positive on every one. **Skipped by the
  `*`-containing rule**, which is why that rule is stated as part of the
  predicate rather than left to the implementation.
- **Prefixed invocation** — `Bash(env GATE_SDK_VERBOSE=1 bash gate-sdk/bin/run-gates.sh)`:
  the path is not the first token after `Bash(`. The extraction skips leading
  `env VAR=val` assignments before the interpreter.
- **Trailing flags** — `Bash(bash gate-sdk/bin/gen-pre-commit.sh --write)`:
  extraction stops at the `.sh` boundary and never swallows the flag.
- **Bare non-path commands** — `git add *`, `shellcheck *`, `chmod +x *`,
  `ls *`, and ~30 more name no repo-relative path at all.
- **Non-`.sh` path entries** — `: > .workflow/prompt-friction.log`, `rm .tmp/*`.
  **Deliberately out of scope**, and the ground is stated because it is not
  arbitrary: those paths are runtime-created and gitignored, so "absent" is their
  ordinary state and existence is not the right predicate for them. Only a `.sh`
  path is a tracked artifact whose absence means the grant is dead.

### 3. The fixture pair

`context-kit/gate-tests/check-settings-paths/{good,bad}/`, each carrying a
self-contained `settings.json` — not a slice of the live one — mirroring
`check-settings-pins`' existing pair shape. **[mechanical]** The `bad/` case
carries a dead literal path; the `good/` case carries **every one of the five
shapes above** alongside a resolving literal path, so the pair pins the
false-positive scoping and not merely the happy path. A `good/` fixture holding
only a resolving literal path would green a predicate that misfires on all eight
glob entries.

### 4. `jq` dependency and its criterion-7 debt, named rather than discovered

The gate parses JSON with `jq` and **fails closed** when it is absent, exactly
as `check-settings-pins.sh:61` does. **[design-bearing]**

`jq` is **not** on `GATE_SDK_PROGRAM_FLOOR` (`gate-sdk/lib/gate.sh:61-63`), so
this gate fails **port criterion 7** on the day it lands and owes designed-away
work at its port. That is recorded here, at authoring time, because criterion 7
is an ordering signal and not an eligibility screen: the alternative — hand-rolling
a JSON-array scan over floor programs to dodge the dependency — buys a fragile
parser to avoid a debt the sibling gate already carries on the same file. Taking
`jq` keeps one parsing story for the settings file across every reader in the
tree.

### 5. The prune is operator-owned, and it has already run for the entries this amendment measured

The three dead entries named above stayed in `.claude/settings.json` until an
**operator** removed them, and that removal has now happened. **[design-bearing
— and it is a sequencing constraint, not an implementation note.]**

The file is configuration; a session may propose a standing grant and may not
widen its own, and this entry's filing session recorded that the settings edit
"is operator-owned and stays that way". The gate itself reads the file and
writes nothing, so the gate is not operator-owned — but **the gate goes red on
a tree carrying the violations it was built for the moment it registers**.
Landing order is therefore: operator prunes, then the gate registers. This
constraint was escalated to the lead alongside this amendment rather than left
for build to discover at its first `run-gates`, and the lead landed the prune in
`630e77fa` before this iteration's align stage — **confirmed at align**: the
operator prune precedes registration, holds for the class this amendment
measured, and the build session lands the gate on an already-clean tree for that
class.

**Granted for the same operator pass**, not merely proposed:
`bash gate-sdk/bin/port-blockers.sh` was granted in `630e77fa`, as two literal
entries (the bare form and its `*`-suffixed twin, this repo's standing shape for
a grant with args) rather than a single glob — the same shape the neighbouring
`build-native.sh` grant already uses. It is a read-only kit tool the port track
calls at every scope, and this iteration's `SPEC-port-group.md` adds a second
arm to it, so the call count rises rather than falls; that is now discharged.

**Open at align, not resolved here — a second instance of this same class
arrives in this iteration, and the two amendments do not yet agree on its
order.** `gate-sdk/SPEC-eighth-cohort.md` deletes ported members' `.sh` files as
its whole point, and its own text names the consequence: "this cohort is
precisely the event that creates such entries... the new gate will red on them,
and the fix is an operator settings edit the build session may not make."
That amendment names the collision; this one does not, and neither states an
order. Unlike the class measured above, this instance is **not yet
resolved** — landing this gate before the eighth cohort risks the eighth
cohort's own commit going unfixably red; landing the eighth cohort first and
this gate after risks this gate registering red on entries only an operator can
clear, exactly the sequencing problem delta 5 already names for the prior
class. This is filed as an open cross-unit question for the lead rather than
ruled here (align/turn-end escalation), since it is a build-ordering decision
across two components, not a fact this session's oracles settle.

## Producers and consumers

**`check-settings-paths` (new gate).**

- **Producer** — the gate script, dispatched two ways, both already emitted and
  neither test-only: the generated pre-commit hook (via `tier=precommit`, so
  `gen-pre-commit.sh` emits it and `check-graph` holds the hook fresh) and the
  full battery `bash gate-sdk/bin/run-gates.sh` after registration in
  `scripts/gates.list`. Its enabling config is `CONTEXT_KIT_SETTINGS_FILE`,
  which already resolves to a default in this repo's layout, so there is no new
  knob that a deployed configuration could leave unset.
- **Consumer** — the committing developer and CI, by exit status and the stdout
  violation list, at the pre-commit and battery transitions.

**Field readers.**

| Emitted | Reader | Transition |
| --- | --- | --- |
| violating entry (verbatim allow string) + its resolved path | committing developer | fixing or escalating the grant at pre-commit |
| checked count | the same reader | confirming the predicate scoped to something rather than vacuously passing |
| exit status | `run-gates` / the pre-commit hook | battery verdict |

**Existing readers of the settings file, and each one's red condition.** This
delta adds a reader and prunes nothing, so causal-completeness point 5's
narrowing analysis has no subject **for this amendment**. It is enumerated
anyway, because the operator prune in delta 5 *is* a narrowing of the allow
array and is the change that ships beside it:

- `check-settings-pins` — reds on a pinned jq path whose value differs, or a
  malformed pin line. Its three pins are `.autoMemoryEnabled`,
  `.env.CLAUDE_CODE_DISABLE_AUTO_MEMORY`, `.worktree.baseRef`; none addresses
  `permissions`, so removing three `permissions.allow[]` elements cannot move
  its verdict. **Monotone in its violation set and clear by inspection.**
- `check-memory-off` — reads override keys in the local settings file, not the
  committed allow array. **Unaffected.**
- `guard-kit/lib/guard.sh` `_guard_allow_inners` and `bin/scan-prompts.sh` —
  advisory, never a commit gate; they report on grants that exist and cannot red.
- `guard-kit/bin/compare-settings-allow.sh` — **the one non-monotone reader, and
  it is the shape point 5 warns about.** It reports local entries *subsumed by a
  committed glob*, so it is a coverage-style reader over the committed set:
  removing committed entries can only ever *add* to its redundancy findings, not
  remove them. It is advisory and mutates nothing, so it cannot fail a battery —
  but it is named here rather than cleared by the reflex "a narrower corpus can
  only remove violations", which is false and is the first argument a narrowing
  reaches for.
- `check-enforcement-fresh` and `scripts/check-value-rollup-fresh.sh` — cite
  `.claude/settings.json` only in a `# graph:` freshness coupling and never
  parse it. **Unaffected.**

## Existing sections updated

- **context-kit/SPEC.md** — a new `### check-settings-paths` per-component
  section, owned by deltas 1–4: invariant, manifest, knob, output contract,
  fail-closed behavior, fixture pair, and the criterion-7 debt from delta 4.
- **context-kit/SPEC.md §check-settings-pins** — owned by delta 1. It is the
  sole documented reader of `CONTEXT_KIT_SETTINGS_FILE`; the knob now has two
  readers and the section stops being the knob's only home.
- **context-kit/SPEC.md §Layout and configuration** — owned by delta 1: the
  kit's gate roster gains a member.
- **guard-kit/SPEC.md §compare-settings-allow** — **owned by delta 1**
  (corrected at align: "The placement ruling" section names the reasoning, but
  the update target is delta 1's placement of the new gate in context-kit
  rather than guard-kit). Its advisory placement ruling reads as covering
  allow-list checking generally; it gains the boundary this amendment
  establishes — that ruling is about the gitignored overlay and operator-intent
  breadth, and a committed-file filesystem-fact predicate is gated elsewhere.
  Without this the next reader re-derives the same question, and this amendment
  is that re-derivation.
- **`scripts/gates.list`** — registration, owned by delta 1.
- **README.md and the enforcement map / graph artifact** — owned by delta 1: a
  new gate stales the generated projections rostered in
  docs/site-architecture.md §Generated projections, and each freshness gate
  prints its own regen command on red.

## Definition of Done

- [ ] **Causal completeness** — the gate has a reachable producer (hook +
      battery, no new knob) and a named consumer; every emitted field has a named
      reader; every existing reader of the settings file has its **red condition**
      enumerated against the operator prune, with the one non-monotone reader
      named rather than cleared by reflex.
- [ ] **Merged with no information lost** — the new section integrated into
      context-kit/SPEC.md; the guard-kit boundary landed in its own section.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      context-kit (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped for prose claiming allow-list checking
      is advisory-only tree-wide.
- [ ] **Gaps filed** — cross-component gaps found during the work filed as debt
      tasks.
- [ ] **Sequencing honored** — the operator prune precedes registration; the
      build session does not edit `.claude/settings.json`. Discharged for the
      class measured at filing (`630e77fa`); **open** for the class
      `SPEC-eighth-cohort.md` creates in this same iteration until the lead
      rules the build order between the two units (see delta 5).
