# SPEC amendment: isolation-base-ref — the base ref is configuration

`readonly-dispatch-isolation-unbuyable` was filed `[design-pending]` with four
authored design candidates, hand-mitigated in seven dispatches, and written into
shipped kit doctrine as measured fact. It is **not a design problem.** The
harness setting `worktree.baseRef` (enum `fresh` | `head`) selects the base an
isolated agent's worktree is cut from, and its own schema description states it
"Applies to `--worktree`, `EnterWorktree`, and agent isolation."

**Confirmed empirically, not inferred.** With `baseRef: "head"`, an isolated
agent dispatched from a checkout at `4ed57685` came up at `4ed57685` — the
dispatcher's HEAD — while `origin/master` stood at `b8f27d8e`, seven commits
behind. All seven prior attestations were taken under `fresh`, which is the
default. They record correctly-applied configured behavior, not a harness
defect, and this amendment writes up no defect.

**What does not collapse.** The entry has a second half, found at the same close
and caused by something else entirely: an isolated agent cannot read *untracked*
state. No value of `baseRef` reaches it, because an untracked file is in no
commit. Delta (3) is that half, and it is the half the shipped doctrine never
carried at all.

## What changes

### (1) The false universal in shipped, public kit prose — **design-bearing**

`delegation-kit/templates/agent-execution.md`, in the isolation-cost bullet,
asserts today:

> **(1) The worktree is cut at the remote tracking ref, not at HEAD** — measured
> rather than inferred: sweeps dispatched from several different local HEADs all
> came up at `origin/master` exactly. So the staleness *equals the unpushed
> backlog* [...]

Both sentences are true only under `baseRef: "fresh"`. They are stated as
universal, in a consumer-facing template a vendoring repo installs verbatim.
They are replaced by the configured form:

> **(1) The worktree's base is configuration, and the default is not HEAD.**
> `worktree.baseRef` selects it: `fresh` (the default) branches from
> `origin/<default-branch>`, `head` from the dispatcher's local HEAD. Under the
> default a child reads the *pre-change* tree and the staleness equals the
> unpushed backlog exactly, which a parent reads as `git log
> origin/<default-branch>..HEAD`; under `head` it does not. **Set it before you
> design around it** — and pin it, so the guarantee does not depend on whichever
> machine happens to dispatch.

The correction is written to be *portable*: it names the knob and both values
rather than this repo's choice, because a vendoring consumer inherits the
template, never our settings file. The measurement is not deleted — it is
re-attributed to the configuration it was taken under, which is what makes it
still usable by a consumer running the default.

### (2) The child-side rev discipline stays — **design-bearing**

`git rev-parse HEAD` to verify, `git show <rev>:<path>` to read, stop-and-say-so
when the target is unreadable at that rev: **kept, unchanged, and now with a
stated reason** rather than as a workaround for a defect that turned out not to
exist. Three grounds, and the first alone is sufficient:

- **The parent cannot read the child's effective configuration.** `baseRef`
  resolves on the dispatching machine's merged settings; nothing in the dispatch
  payload carries it and nothing returns it. A parent that assumes `head` is
  asserting an unverified premise about a third-party tool — the same move that
  produced this entry.
- **A vendoring consumer inherits the doctrine, not the pin.** The kit ships
  `agent-execution.md`; it cannot ship a consumer's `.claude/settings.json`. For
  every consumer that has not set the knob, the discipline is the whole
  protection.
- **It is nearly free and it is self-verifying.** `git rev-parse HEAD` costs one
  command and its output *is* the evidence; the discipline degrades from load-
  bearing to a cheap assertion when the pin holds, which is the right shape for
  defence in depth.

What is **retracted** is the framing that made it feel expensive — the doctrine
no longer says a parent "owes the doubt back" for a structural defect. It says
the parent states the rev and the child confirms it, which is one line in a
prompt and one command in the child.

### (3) The half no configuration reaches — **design-bearing**

The shipped template's cost bullet (2) says the worktree "lands inside the repo
and untracked", which is about the worktree directory. It says nothing about the
distinct and worse failure the queue entry recorded: **an isolated child cannot
read the parent's untracked or gitignored files at all**, because they exist in
no commit. Attested — a sweep sent to triage `.workflow/prompt-friction.log`
read an empty file and reported a 621-line corpus absent; `compare-settings-allow.sh`
saw no overlay because `.claude/settings.local.json` is uncommitted.

New prose in `agent-execution.md`, stated as its own numbered cost:

> **An isolated child sees only committed state.** Untracked and gitignored
> files are in no commit, so no base ref reaches them and naming a rev does not
> help. A sweep whose corpus includes an untracked surface returns a confident
> "nothing there" — indistinguishable in shape from a clean result, which makes
> it the worse of the two failures. **Such a sweep is not delegable to an
> isolated agent**: the parent reads that surface itself, or passes its content
> in the prompt. Note the bind this creates with the read-only rule above — a
> read-only claim is made by isolation, and isolation is what blinds the read —
> so a claimed-read-only sweep over an untracked corpus has no correct form and
> must not be dispatched.

This is where the queue entry's headline — *the shape that makes a read-only
claim is the shape that poisons the read* — is true. It was filed as true of the
whole class; it is true of exactly this case, and delta (1) is why the rest of
the class is not.

### (4) The repo pin — two lines that must land together — **mechanical**

Operator-directed, and **no new gate**: `check-settings-pins` is already the
general mechanism (context-kit/SPEC.md §check-settings-pins — "any settings key
is pinnable"), and it is the same gate that holds memory-off. A new gate would
duplicate shipped mechanism, which enforcement-first's own clause refuses.

- `.claude/settings.json` gains `"worktree": {"baseRef": "head"}` — confirmed
  absent today.
- `scripts/settings-pins.conf` gains `.worktree.baseRef = "head"`.

**They land in one commit and the ordering is not a style preference.** A pin
naming a key absent from the settings file is exit **2** — a desynced manifest,
not a soft miss — so a commit carrying the pin alone reds the battery closed.
The gate's `# graph:` manifest already couples the two paths (`couples=
.claude/settings.json,scripts/settings-pins.conf dir=one`), so the coupling is
declared, not discovered.

**Pinning at the repo level is the durable half.** Project settings are the
repo's own tracked config, so the guarantee stops depending on an operator's
global `~/.claude/settings.json` — which is untracked, per-machine, and outside
any gate's reach. The pin also inherits a second reader for free:
`check-memory-off`'s overlay condition reds when `.claude/settings.local.json`
sets **any** pinned key to a value other than its pin, so a local override back
to `fresh` is caught by a gate already registered.

**The honest residual, stated rather than assumed:** this closes the
project-and-overlay surface. Whether a project-level value overrides a
user-level one for this key is a precedence question about the harness's
settings merge; where it does not, the pin still documents intent and still
reds a local override, and the delta (2) discipline is what carries the
remainder. That residual is the reason delta (2) is a ruling and not a
courtesy.

### (5) The consumer-side recommendation, in the kit — **mechanical**

`delegation-kit/SPEC.md` §The delegation model gains a short configuration note:
a consumer dispatching isolated agents sets `worktree.baseRef` deliberately and
pins it if its harness config is gated, citing context-kit/SPEC.md
§check-settings-pins as the mechanism. It **names the knob, never a value** — a
consumer that wants a clean tree per dispatch is right to stay on `fresh`; what
the kit rules is that the value be chosen rather than inherited. No knob is added
to delegation-kit's own `<KIT>_<KNOB>` roster: this is harness configuration, not
kit configuration, and inventing a kit knob that shadows a vendor setting would
be a second source for one value.

### (6) Queue disposition — **mechanical**

`readonly-dispatch-isolation-unbuyable` moves to `## Done` at merge, dropping its
`[spec:]` tag. Both halves are dispositioned: the stale-base half by deltas
(1) and (4), the untracked half by delta (3)'s ruling — which is a *ruling* (this
class of sweep is not delegable), not unbuilt work, so it leaves nothing behind
to re-promote.

**One inbox bullet is superseded and close must not read it as live.** The
committed gap inbox carries a 2026-08-12 bullet re-filing this entry, whose
diagnosis ("the confirmed remote-tracking-ref mechanism attested a seventh
time") is correct as an observation and wrong as a cause. Close's drain should
disposition it against this amendment, not against the entry's pre-correction
body.

## Producers and consumers

**`worktree.baseRef` is not our state**, and every causal-completeness point
below is answered accordingly — the kit introduces no new event, no new message
and no new field. What it introduces is one pinned configuration value and three
pieces of prose.

**The pinned value** (delta 4). *Producer:* `.claude/settings.json`, a tracked
file in this repo, read by the harness at session start — so its "enabling
config" is the file itself and there is no set-nowhere-but-in-tests exposure;
the empirical confirmation above is a real dispatch, not a unit test.
*Consumers, both named, with red conditions:*

- `check-settings-pins` reads `scripts/settings-pins.conf` against
  `.claude/settings.json` at pre-commit. Its red conditions are **two and they
  differ**: a present-but-wrong value is exit 1 (the legible violation); a pin
  whose key is **absent** is exit **2**. The second is not monotone in anything
  — it reds on *finding none* — which is exactly why delta (4) states the
  one-commit rule as a contract rather than a convention.
- `check-memory-off` reads the same pins file against
  `.claude/settings.local.json`. Red condition: a pinned key present in the
  overlay with a non-pin value. Absent overlay, or overlay without the key, is
  clean — so adding a pin cannot red an existing clone that has no overlay.

No third reader exists: `guard-kit/bin/compare-settings-allow.sh` compares the
`permissions.allow` array only, so a new top-level `worktree` object is outside
its corpus. Probed rather than assumed.

**The corrected doctrine** (deltas 1-3). *Producer:*
`delegation-kit/templates/agent-execution.md`, loaded by the `/agent-execution`
trigger named in CLAUDE.md §Agent execution. *Consumers:* every dispatching
session in this repo, and every consumer vendoring the template. The one
mechanical reader is `check-shim-restatement`-class parity between the kit
template and any consumer copy — probed: this repo installs the template by
reference through the skill trigger and keeps no second copy, and the docs
mirror publishes `SPEC.md`/`README.md`, not `templates/`, so the correction has
exactly one home and no fan-out.

**The SPEC note** (delta 5). *Producer:* delegation-kit/SPEC.md prose.
*Consumer:* a consumer configuring its harness at vendoring time. No gate reads
it; `check-knob-roster`-class gates read the `<KIT>_<KNOB>` roster, which delta
(5) deliberately does not join — a note naming a vendor setting adds no kit knob,
and this is recorded so a later reader does not "fix" the omission.

**Nothing is narrowed.** No corpus, glob or file set shrinks in this amendment,
so causal-completeness point 5's narrowing rule has no subject; the red
conditions above are enumerated anyway because two of them are non-monotone and
one of those (the absent-key exit 2) is the entire reason delta (4) is written
as a single commit.

## Existing sections updated

- **delegation-kit/templates/agent-execution.md**, isolation-cost bullet (1) —
  replaced, owned by delta (1). This is the only surface in the tree carrying the
  false universal; grepped tree-wide for "remote tracking ref" / "unpushed
  backlog" / "cut at the remote", and the only other hits are the queue entry and
  the gap-inbox bullet, both dispositioned by delta (6).
- **delegation-kit/templates/agent-execution.md**, a new isolation-cost item —
  the untracked-read blindness, owned by delta (3). It sits beside the existing
  read-only-claim rule because delta (3)'s bind is between the two.
- **delegation-kit/SPEC.md §The delegation model** — the configuration note,
  owned by delta (5).
- **`.claude/settings.json`** and **`scripts/settings-pins.conf`** — owned by
  delta (4), one commit.
- **context-kit/SPEC.md §check-settings-pins** — nothing changes. Its
  "this consumer's first pins hold the auto-memory-disabling keys" sentence
  remains true of the *first* pins and the section already states the mechanism
  is general. Recorded as a target considered and not claimed, so build does not
  adopt it as an orphan.
- **CLAUDE.md §Agent execution** — nothing changes. It names the trigger, not the
  doctrine's content, which is the load-trigger residency rule working correctly.
- **TASK-QUEUE.md** — the entry's body is rewritten at promotion rather than at
  merge, because its recorded diagnosis is what a reader would otherwise act on.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The pin is proved, not declared** — the battery is green *after* both
      lines land, and red with the pin alone. A build that lands only the settings
      key has proved nothing about the pin.
