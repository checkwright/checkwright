# SPEC amendment: scratch-bash-only

Closes the reach gap in the scratch-execution control. Pairs with
`scratch-execution-control-is-bash-only`, whose four prior declines the operator
reversed on 2026-08-23 and whose promotion this boundary pays.

**The entry's deliverable fork is this amendment's to rule, and it is ruled
below.** Three options were left open and untouched — widen the guard's match to
an interpreter set, widen the runner, or rule scratch execution bash-only and
have the guard say so. Delta 1 takes the third **using the first's mechanism**
and states why the second is refused outright.

## What changes

### (1) The policy: scratch execution is bash-only, and the guard says so

Executing a program body that lives under a scratch directory is **bash-only**.
`bin/scratch-run.sh` is the sanctioned form and its hardcoded interpreter stops
being an unexamined default and becomes the statement of the rule. {design-bearing}

**Why the third option, and why the second is not merely more expensive but
wrong.** Widening the runner means teaching it to execute an arbitrary
interpreter. The runner is reached through a committed allowlist grant —
`Bash(bash guard-kit/bin/scratch-run.sh *)` in this consumer's settings — and
that grant was bought by one specific compensating control: the runner echoes the
body it is about to execute, and it only ever hands that body to `bash`. Teaching
it a second interpreter converts a grant for *"run bash on a reviewed body"* into
a grant for *"run anything on a reviewed body"* **without any settings edit** —
that is, it widens a permission behind the boundary the operator owns
(TRAJECTORY.md §The closed rulings, 2026-08-22), using a code change no
permission gate reads. That is decisive on its own, and it is a security argument
rather than a cost one.

**The first option is incoherent alone.** Steering `python3 .tmp/x.py` to a
runner that would refuse it buys the session a second refusal, so option 1
presupposes option 2 and inherits its objection.

**The third option is also the only one a build session can land.** It needs no
settings edit, so it does not stall at the permission wall the way
`guard-grant-review`'s narrowing half did.

**What "bash-only" costs, stated rather than glossed.** It removes a capability:
a session that wants a Python scratch script must write a `.sh` that invokes
Python with the body inline, or do the work in a language the control covers.
That is the narrowing the entry's own third option described as "cheaper and
narrows what sessions may do", and it is taken deliberately: the measured
population of non-bash scratch runs is small (four `python3 .tmp/<name>.py` runs
across the whole series) while the control's silent hole applies to every one.

### (2) The steer becomes a generic rule with two arms

The consumer-only arm in `scripts/bash-guard.sh` matching `^bash[[:space:]]+\.tmp/`
is **generalized into guard-kit's numbered ruleset** and deleted from the
consumer copy. The new rule fires on a command whose leading binary is a script
interpreter and whose operand names a path under a `GUARD_KIT_SCRATCH_DIRS`
member, with two decisions. {design-bearing}

| arm | trigger | decision |
| --- | --- | --- |
| a | interpreter is `bash` or `sh` | block, naming `bin/scratch-run.sh` — today's steer, unchanged in effect |
| b | interpreter is a `GUARD_KIT_SCRIPT_INTERPRETERS` member | block, naming the bash-only rule and the same runner |

**Why generic rather than a wider consumer rule.** The rule's content is
guard-kit's own — its subject is the kit's scratch-dir knob and the kit's own
runner — so it carries no consumer vocabulary and crosses no provenance seam
(CLAUDE.md §The provenance seam). Making it generic is what buys it a decision
table: the consumer lane has no verification lane at all, and today all four of
this repo's consumer-only rules carry zero behavioral coverage. Delta 4 is the
coverage that arrives with the move.

**It also retires a second entry rather than colliding with it.**
`scratch-run-steer-rule` asks for exactly arm (a) and for a ruling on whether the
steer fires on the scratch dir alone or on any `bash <path>.sh`; arm (a) is that
steer, and the scratch dir alone is that ruling. That entry is filed to the gap
inbox at authoring as a wontfix candidate for the close drain rather than being
quietly absorbed here.

**Placement is at 22, ahead of fall-through logging.** Rule 22 is fall-through
and is terminal by definition, so the insertion renumbers exactly one rule. The
three-way correspondence a new rule must satisfy — the numbered roster in this
SPEC, the `guard_rule_*` function in `lib/guard.sh`, and the fixed dispatch order
in `guard_generic_rules` — is un-gated, which is the separate open entry
`guard-ruleset-registration-lockstep`; this amendment satisfies the
correspondence by hand and does not gate it.

### (3) The subject is the **path** form, and the inline forms are ruled out with grounds

The rule keys on the interpreter, as the entry's seventh measurement demanded,
and requires an operand under a scratch directory. A body carried **in the
command string** — `python3 -c '…'`, `python3 <<'PY' … PY`, `python3 - <<'PY'`
— does not fire. {design-bearing}

**The discriminator is body visibility, and it is the same one that bought the
runner its grant.** §scratch-run's whole argument is that a scratch path is
"rewritable by any session", so the body the operator approved is not the body
that runs; the runner's echo-at-execution is what restores the correspondence. A
`-c` or heredoc body **is** the command string — the permission prompt shows it,
the guard's own matcher reads it, and the friction log records it. There is
nothing for a compensating control to compensate for, so a rule firing there
would refuse a reviewable act in the name of reviewability.

**This narrows the entry's stated reach, and the narrowing is disclosed rather
than smuggled.** The entry's seventh measurement concluded that "stdin carries no
`.tmp/` path to match, so even the cheap third option misses it unless the rule
names the INTERPRETER rather than the path". The interpreter-naming half is
adopted in full. The implication that the stdin shape is therefore in the
subject is not, on the ground above — and on a second one the measurement series
does not separate out: the bulk of the counted stdin heredocs are `cat > <file>`
writes, which execute nothing at all and belong to the write-form question
`prompt-ranking-command-word-shape-blind` carries. A rule refusing those would be
refusing file authorship under a scratch-execution heading.

### (4) The interpreter roster is a knob, and the runner uses a shebang instead

`GUARD_KIT_SCRIPT_INTERPRETERS` — an array knob, config-via-env, this repo's
layout as its default — names the non-bash interpreters arm (b) covers.
{design-bearing}

**A roster is the shape guard-kit already uses for exactly this kind of set**
(`GUARD_KIT_RO_BINS`, `GUARD_KIT_APPEND_BINS`, `GUARD_KIT_SCRATCH_DIRS`), and
making it a knob is what answers the entry's "a roster that rots" objection: the
kit ships a default and the consumer owns the value, so a missing member is a
consumer's config edit rather than a kit release. The roster carries universal
tool names and no consumer vocabulary, so it is kit-shippable under the
provenance seam — unlike the `check-graph` class of content, which is
consumer-config precisely because its members are private.

**The runner does not read the roster, and reads the shebang instead.**
`bin/scratch-run.sh` already `cat`s the target before executing it, so it holds
the body; it refuses — exit 2, naming the bash-only rule — when line one is a
`#!` naming an interpreter that is not `bash` or `sh`. That is **exact** where a
roster is approximate, it is derivation-first (the file states its own
interpreter), and a target with no shebang is unaffected, which keeps every
`.sh` the runner handles today working. Two mechanisms, each exact for the input
it actually has: the guard has a command string and no file, the runner has a
file.

**The runner's `bash` line becomes a directive.** The hardcoded `bash "$TARGET"`
gains a `# spec:` binding to the bash-only rule, so the next reader meets the
policy at the line that implements it rather than inferring a default.

### (5) Behavioral coverage, which the moved rule gains and the consumer rule never had

`guard-tests/cases.tsv` gains a firing and a non-firing case per arm — the kit's
standing rule that every generic rule owes both. {mechanical}

The cases the arms are worth: `bash .tmp/x.sh` blocks (arm a) while
`bash tools/x.sh` falls through; `python3 .tmp/x.py` blocks (arm b) while
`python3 -c 'print(1)'` and `python3 - <<'PY'` fall through (delta 3's boundary,
which is the arm most likely to regress silently); and a roster member invoked on
a non-scratch path falls through. `bin/run-guard-tests.sh` drives
`templates/bash-guard.sh`, and the rule is generic, so the table reaches it
without a consumer-side lane.

`gate-tests/scratch-run.test.sh` gains the shebang refusal: a `.tmp` target whose
first line names a non-bash interpreter exits 2 and is not executed. {mechanical}

## The provenance seam

**Kit mechanism, and the one roster is ruled kit-shippable rather than assumed
so.** The rule, the runner's shebang refusal and the message are generic — their
subject is guard-kit's own scratch-dir knob and guard-kit's own runner.
`GUARD_KIT_SCRIPT_INTERPRETERS` carries universal interpreter binaries, which are
none of the classes CLAUDE.md §The provenance seam names (term lists, coupling
vocabularies, glossary bodies, wire-contract couplings, product constant sets),
and it sits beside three roster knobs the kit already ships with defaults. The
`check-graph` contrast is the discriminator worth stating: that roster is
consumer config **because its members are private**, and this one is not,
so the test is the content and never the shape.

**A consumer stays able to override it**, which is what keeps the rot objection
answered without publishing anything: the default is the kit's, the value is the
consumer's, through `GUARD_KIT_CONFIG_FILE` like every other roster knob.

**What deliberately does not become config: the policy.** Bash-only is a rule,
not a setting, and a knob that turned it off would restore the honour system the
rule replaces — the same reasoning delegation-kit's D1 gives for having no knob.
The valve is not vendoring the rule.

## Producers and consumers

**New rule — generic rule 22, scratch execution under a non-bash interpreter.**
- *Producer:* `guard_rule_script_interpreter` in `guard-kit/lib/guard.sh`,
  dispatched from `guard_generic_rules` in its fixed order, reached at every
  `PreToolUse(Bash)` firing through both `templates/bash-guard.sh` and the
  vendored `scripts/bash-guard.sh`. The enabling configuration is the hook
  registration that already exists in this consumer's settings — the same
  registration the three surviving consumer-only rules already run under — so no
  permission-class edit is reachable from this unit.
- *Consumer:* the harness, which converts `guard_block`'s exit 2 into a refusal
  and shows the message to the session; and the session itself, which receives
  the named alternative.

**New knob — `GUARD_KIT_SCRIPT_INTERPRETERS`.**
- *Producer:* `lib/guard.sh`'s defaulted-array idiom, alongside the three
  existing roster knobs, overridable through `GUARD_KIT_CONFIG_FILE`.
- *Named reader:* `guard_rule_script_interpreter`'s arm (b) trigger, at the rule
  dispatch transition. It has **exactly one** reader by design: `bin/scratch-run.sh`
  deliberately does not read it (delta 4), so the knob is not a second copy of a
  fact the file itself states.

**New refusal — the runner's non-bash shebang.**
- *Producer:* `bin/scratch-run.sh`, after it reads the target and before it
  executes, on the body it already holds.
- *Consumer:* the invoking session, by exit 2 and the message; and
  `gate-tests/scratch-run.test.sh`, which pins it.

**Retired surface — the consumer-only steer.** `scripts/bash-guard.sh` loses its
`^bash[[:space:]]+\.tmp/` arm and the block message with it. Its reader was
`check-template-copy-parity` assertion C, through the surface tokens the arm
declared; the removal reduces the copy's declared surface, which assertion C
does not read (it reads copy-side *additions*), and assertion B is unaffected
because the template never declared the arm. No `# copy-divergence:` marker is
orphaned, because the arm carried none — it is one of the four undeclared
consumer rules, which is the state `consumer-guard-rule-coverage` records and
this unit does not otherwise touch.

**No field, message or state is added that this list does not name a reader
for.** In particular no per-interpreter diagnostic is emitted: the block message
names the rule and the alternative, and which roster member matched is not a
value any reader wants.

## Existing sections updated

- guard-kit/SPEC.md §The generic ruleset — the numbered roster gains rule 22 and
  fall-through logging renumbers to 23; the new rule's subsection states its
  trigger, both arms, its declared inert classes, and delta 3's out-of-subject
  boundary with its grounds (deltas 2, 3 and 4).
- guard-kit/SPEC.md §scratch-run — the section that owns `bin/scratch-run.sh`
  currently argues only that nothing allowlists executing what lands in the
  scratch dir. It now also states the bash-only rule as policy, the shebang
  refusal as its runner-side enforcement, and the ruled-out
  widen-the-runner option with the permission-widening ground (deltas 1 and 4).
- guard-kit/SPEC.md §Layout and configuration — the knob roster gains
  `GUARD_KIT_SCRIPT_INTERPRETERS` with its default and its one reader (delta 4).
- guard-kit/SPEC.md §Consumer rules (the `##` placement contract at the section
  level, **not** the `###` content-shape subsection nested under §The generic
  ruleset — the two share a title and a `§` pointer resolves to the first, which
  is the live instance `spec-section-title-collision` records) — its statement
  that "guard-kit ships no consumer rule and names none" is unaffected, but the
  worked example of a consumer rule that this repo's steer supplied is now a kit
  rule, so any prose leaning on it moves (delta 2).
- `scripts/bash-guard.sh` — the steer arm and its three `# spec:` comment lines
  are deleted; the `cmd_unquoted` skeleton the three surviving arms share stays
  (delta 2).
- `guard-kit/lib/guard.sh` — the new `guard_rule_script_interpreter`, its
  registration in `guard_generic_rules`, and the roster knob's default
  (deltas 2 and 4).
- `guard-kit/bin/scratch-run.sh` — the shebang refusal, and the `# spec:`
  directive binding the hardcoded `bash` to the rule (deltas 1 and 4).
- `guard-kit/guard-tests/cases.tsv` and `guard-kit/gate-tests/scratch-run.test.sh`
  (delta 5).
- `guard-kit/README.md` — the guard's rule-count and scratch-execution lines, if
  either states a number or the old steer's reach (deltas 2 and 4).
<!-- update-target-exempt: a generated mirror is stale the moment any delta lands and is regenerated rather than authored -->
- `docs/guard-kit/SPEC.md` and `docs/guard-kit/README.md` — regenerated, never
  hand-edited.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. Specifically: no surface still describes the
      scratch steer as a consumer-only rule, and no rule number cited anywhere
      still points at the pre-renumber roster.
- [ ] **Probe, not assert** — the closing check is the one the entry itself was
      filed on: a `python3 .tmp/<script>.py` payload run against the live hook
      must now block where it exited 0 at the 2026-08-13 close, and
      `python3 -c` must still fall through.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
