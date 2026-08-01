# SPEC amendment: local-overlay-git-blanket-grant

The local permission overlay carries `Bash(git *)` — one glob granting every git
subcommand without a prompt. The destructive ones ride it: `git reset --hard`,
`git clean -fd`, `git push --force`, `git rm -r`. `scripts/bash-guard.sh`'s
project rules cover exactly two cases (`git commit --no-verify`, `git clean
-x/-X`), so the rest of that set is auto-allowed by the blanket rather than by
any judgment about it.

**Why it survived.** `compare-settings-allow.sh` reports the entry
non-redundant and therefore never flags it — correctly, since the committed set
holds no glob that covers it. The tool answers *"is this entry redundant?"*; the
question this entry needs is *"is this entry too broad?"*, and nothing asks it.
That missing question is the gap, not the entry.

## The diagnosis the entry did not have

The entry names the deliverable a *breadth criterion* and its cost line calls it
"one gate class". The survey behind this amendment found two facts that reshape
that:

**guard-kit registers no gates, deliberately and by stated category.**
`guard-kit/README.md`: *"Unlike the other kits, guard-kit registers **no
gates**: its surfaces are hooks and advisory `bin/` tools, so nothing joins
`gates.list`."* There is no `guard-kit/checks/` directory. Minting one here
would be a category change, not a feature: it would ripple into `gates.list`,
the generated pre-commit hook, `check-kit-registration`, `check-docs-kit-parity`,
`check-readme-roster`'s marker-block requirement, the enforcement map, the
footprint and the value rollup — a very large blast radius for a criterion whose
subject is one gitignored file on one machine.

**And a gate is the wrong instrument on the merits, independent of the kit's
category.** A breadth finding is *"this glob would auto-allow a command you
called destructive"*. Whether that is **wrong** depends on what the operator
meant: `Bash(git *)` in a throwaway sandbox is fine. A blocking gate over that
judgment is a red the operator must valve, and a valved gate is the
false-positive failure `gate-sdk/SPEC.md` §When a gate earns its place warns
about — high-FP checks wait for an attested miss, and no unprompted destructive
command has ever fired here. The subject is also gitignored and per-machine, so
a gate over it is CI-neutral by construction (`check-memory-off`'s own clean
line states that an absent surface proves nothing about another clone) — which
means it would be advisory in effect while carrying a gate's cost.

So the criterion lands where the redundancy criterion already lives, as its
sibling on the same tool, with the same reader.

## What changes

### Delta 1 — `compare-settings-allow` gains a breadth report beside its redundancy report *{design-bearing}*

The tool answers a second question over the same two files: **which local allow
entries are too broad**, reported as a distinct set from the redundant one. It
stays read-only and stays advisory — it reports candidates, the operator
disposes.

The mechanism reuses `guard_allow_match` unchanged, with the arguments in the
other order. Today the tool asks `guard_allow_match <local-entry>
<committed-glob>` — "is this local entry already covered?". The breadth question
is `guard_allow_match <probe> <local-glob>` — "would this local glob auto-allow
this command?". Same primitive, same `:*`-to-`*` normalization, **no new
matching code**. Verified by execution during authoring: against `Bash(git *)`,
the probes `Bash(git reset --hard)`, `Bash(git clean -fd)`, `Bash(git push
--force)` and `Bash(git rm -r src)` all match.

The report names the glob and the probe that witnesses its breadth, so the
finding is legible without the operator re-deriving why. `--count` gains the
same treatment the redundancy count has.

### Delta 2 — the probe set is optional consumer config, and it is probes rather than a roster *{design-bearing}*

The entry's open question: where the destructive set is owned without becoming a
maintained roster.

**It is never owned by the kit.** guard-kit is tool-agnostic; a git verb list in
`lib/guard.sh` would ship one project's vocabulary as the kit's, which the
provenance seam forbids outright. The set is declared by the consumer via a new
knob following the house shape, `GUARD_KIT_BREADTH_PROBES` — an array of
permission-rule strings, **default empty**, in which case the breadth report is
simply absent and the tool behaves exactly as today. That is the `graph-vocab.sh`
pattern gate-sdk already establishes for rule content: optional, declared by the
consumer, absent means off.

**The reframing that answers "without becoming a maintained roster": these are
probes, not a roster.** The array is not a list of destructive commands that
must be kept complete. Each entry is a **witness** — a single string whose
auto-allowance would be bad, offered as evidence that a glob is too broad. A
missing probe costs one witness; it can never produce a false green on a
completeness claim, because **no completeness is claimed**. Nothing in guard-kit
says "these are the destructive commands", and nothing may come to say it: a
report worded as coverage would be exactly the false-confidence proxy §When a
gate earns its place refuses.

Entries are full permission-rule strings rather than bare commands, matching the
settings vocabulary the tool already reads on both sides. This keeps the kit free
of any wrapping convention and lets a consumer probe non-`Bash` rules with the
same mechanism.

### Delta 3 — the close-stage triage step reads the second report *{mechanical}*

`guard-kit/SPEC.md` §The close-stage triage step already directs the close
session to run `compare-settings-allow` and prune the listed local entries. It
gains the breadth set as a second disposition alongside the prune: narrow the
glob, or record that the breadth is intended. No new invocation point and no new
schedule — the consumer that reads this output already exists and already runs
at a fixed point.

### Delta 4 — the instance: the blanket is replaced, and the split is ruled *{design-bearing}*

`Bash(git *)` is deleted from the local overlay. The write-side verbs the
workflow actually uses land in the **committed** allowlist rather than being
re-granted locally, and destructive git is left to prompt.

**Why committed rather than local.** A narrowing that lives only in a gitignored
file is another local-only fact nothing can review, diff, or inherit — the same
invisibility that let the blanket survive. The committed set already carries
write-side verbs beside the read-only ones (`git add`, `git commit -m`, `git rm
-q`), so this is continuous with existing policy rather than a new posture. The
read-only verbs promoted at the last close are what made the narrowing
affordable; this finishes that move.

**`git push` is ruled to keep prompting**, and the ruling is recorded because it
looks like an omission otherwise. CLAUDE.md §This repo is governed by its own
kits budgets one to two pushes per iteration and treats a master push as a
deliberate act verified against the remote oracle. At that frequency the prompt
costs almost nothing and buys exactly the deliberateness the doctrine asks for.
Auto-allowing it would remove the one confirmation that makes the act
deliberate.

Verified by execution during authoring: against the narrowed set, `git reset
--hard`, `git clean -fd`, `git push --force origin master` and `git rm -r` all
prompt, while `git checkout -- <path>` and `git stash pop` are allowed. The
narrowing is demonstrated before a line of it is written.

**Honest limit, stated rather than discovered at validate.** The overlay half of
this delta edits a gitignored file. It leaves no tracked artifact, so no gate
holds it and validate cannot verify it — the breadth report is what makes the
state re-checkable on the next close, and that is the whole of the mechanical
residue. The committed half is tracked, reviewable, and diffable, which is the
second reason the split falls that way.

## Producers and consumers

This amendment adds one **knob** and one **report** to an existing advisory
tool. It adds no gate, no state file, no hook, and no message.

**`GUARD_KIT_BREADTH_PROBES`** (new knob, Delta 2).
*Producer:* the consumer's `guard-config.sh`, the file guard-kit's §Layout and
configuration already specifies as the override point for every `GUARD_KIT_*`
knob. Its enabling configuration is a file this repo carries and the tool
already sources, so the producer is not a test-only path. **In this repo the
delta must actually write the array** — a knob whose only setter is the kit's
own default is the dead-producer shape, and Delta 4's narrowing is what gives it
a non-empty value here.
*Consumer:* `compare-settings-allow.sh`, at the point it has both allow lists in
hand, iterating probes against the local entries.
*Default-empty behavior:* the breadth section is omitted entirely. Absence is
clean and silent, matching the tool's existing "no local file — nothing to
compare" path rather than inventing a new disposition.

**The breadth report** (new output, Delta 1).
*Producer:* `compare-settings-allow.sh` on each invocation with a non-empty probe
set.
*Consumer:* the close-stage triage step (Delta 3) — a reader that exists today
and already consumes this tool's other report at the same moment. This is the
check this unit most needed to pass: a second report with no scheduled reader
would be output nobody sees, which is the field-with-no-reader defect in
tooling's clothes.
*Every field has a named reader:* the report emits two fields per finding — the
local glob and the witnessing probe. Both are read by the close session at the
disposition transition: the glob is what gets narrowed, the probe is why. No
third field (a severity, a count per glob) is emitted, because neither would
change the disposition.

**Enforcement, ruled explicitly.** This unit ships **no gate**, and that is the
diagnosis's conclusion rather than an omission. The enforcement-first doctrine's
own ordering is what the design follows: the surface is per-machine and
gitignored, the verdict is operator-intent-dependent, and the outer tier that
would make a gate a guarantee (CI) cannot see the file at all. What replaces the
gate is a scheduled reader — the close-stage step — which is the same mechanism
the redundancy criterion has relied on since it shipped.

**Testing.** The criterion is not a generic guard rule, so it earns no row in
`guard-tests/cases.tsv` — that decision table drives the template guard's hook
decisions, and this is `bin/` tooling. It belongs to the bespoke `gate-tests/`
lane the kit's other two `bin/` tools already use
(`gate-tests/scan-prompts.test.sh`, `gate-tests/scratch-run.test.sh`), covering a
firing probe, a non-firing probe, and the empty-knob silence.

## Existing sections updated

- **`guard-kit/SPEC.md` §compare-settings-allow** — the breadth criterion added
  beside the redundancy one, stated as a second question over the same two files
  and the same `guard_allow_match` core (Delta 1); the probes-not-a-roster
  framing and its no-completeness-claim limit recorded (Delta 2).
- **`guard-kit/SPEC.md` §Layout and configuration** — `GUARD_KIT_BREADTH_PROBES`
  joins the knob roster with its default-empty semantics (Delta 2).
- **`guard-kit/SPEC.md` §The close-stage triage step** — the breadth set added
  as a second disposition beside the prune (Delta 3).
- **`guard-kit/SPEC.md` §Testing** — the new `gate-tests/` case named, covering
  a firing probe and a non-firing one (Delta 1) and the empty-knob silence
  (Delta 2); the reasoning is the **Testing** paragraph under §Producers and
  consumers, not a paragraph inside either delta.
- **`guard-kit/bin/compare-settings-allow.sh`** — the breadth report and its
  `--count` treatment (Delta 1), reading the knob (Delta 2).
- **`guard-kit/templates/guard-config.sh`** — the knob's skeleton entry (Delta 2).
- **The consumer's `guard-config.sh`** — the probe array written non-empty
  (Delta 2's knob, given its value by Delta 4) — and **`.claude/settings.json`**,
  the narrowed committed verbs (Delta 4). The local overlay edit is untracked by
  construction.
- **`guard-kit/lib/guard.sh`** — **not edited.** `guard_allow_match` is reused
  as-is; listed so build does not add a breadth variant of a primitive that
  already answers the question with its arguments swapped.
- **`guard-kit/README.md`** — **not edited on its no-gates sentence**, which
  stays true: this unit adds a `bin/` report, not a gate. Listed because the
  queue entry's "one gate class" wording invites the opposite edit.
- **No `gates.list` registration, no `checks/` directory, no pre-commit hook
  change.**

## The seam

Ruled explicitly, per CLAUDE.md §The provenance seam.

**The kit ships the question; the consumer ships the vocabulary.** guard-kit
gains the ability to ask "is this local glob too broad?" and the machinery to
answer it against a declared probe set. It gains **no git verbs, no destructive
vocabulary, and no default probes**. Every string naming a command lives in the
consumer's `guard-config.sh`. A kit literal listing `git reset --hard` would
publish one project's tool vocabulary as the kit's — the same rule that makes
`GRAPH_VOCAB` consumer config rather than a gate-sdk literal.

**Default-empty is the seam's enforcement, not just a convenience.** With no
probes the feature is inert, so a consumer who vendors guard-kit inherits the
mechanism and none of this project's judgments about which commands are
dangerous.

**Nothing moves up from the consumer either.** `scripts/bash-guard.sh`'s two
existing project rules stay where they are and are not generalized into the
probe set: they are *hook* rules that block a command at execution, while probes
are *audit* strings that characterize a permission glob. Merging them would
conflate two mechanisms with different readers and different failure modes.

## Definition of Done

- [ ] **Causal completeness** — `GUARD_KIT_BREADTH_PROBES` has a named producer
      (`guard-config.sh`, written non-empty in this repo by Delta 4) and a named
      consumer (`compare-settings-allow.sh`); the report has a named consumer
      (the close-stage triage step); both emitted fields have a named reader at
      the disposition transition.
- [ ] **Merged with no information lost** — the breadth criterion lands inside
      §compare-settings-allow as that section's second question, not as an
      appended section; the knob lands in the existing roster; the close-step
      edit reads as one step with two dispositions.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      guard-kit (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — `Bash(git *)` is gone from the local overlay and
      no surface still describes the overlay as holding a blanket git grant;
      guard-kit's no-gates sentence is confirmed still true.
- [ ] **Projections regenerated** — the SPEC edit stales the docs mirror, the
      footprint and the value rollup; the new `bin/`/`gate-tests/` files stale
      the footprint. Each gate names its own regen command on red.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed via
      `bash lifecycle-kit/bin/file-gap.sh`. (The gate-versus-tool placement is
      settled in the diagnosis and Delta 1, not filed.)
