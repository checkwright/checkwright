# SPEC amendment: dispatch-shape

Queue entries: **`fork-dispatch-prohibition`**, **`read-only-fanout-unenforceable`**,
**`subagent-parent-addressing`**. Owning component: **delegation-kit**
(`templates/agent-execution.md` and delegation-kit/SPEC.md §The delegation model).
The basename names the *mechanism*, not any one slug, and the ref resolves as a
bare basename tree-wide (canon-kit/SPEC.md §check-amendment-queue).

## Why three entries take one amendment

The three defects are one mechanism seen from three sides. Each is a rule about
the **shape of a dispatch call** that today is carried only by prose in
`templates/agent-execution.md`, and each fails the same way: the prose does not
reach the reader it binds. Splitting them would specify the same new guard three
times, and — worse — would put three build sessions on the same two files, which
this kit's own **Serialize on shared files; ≤`DELEGATION_KIT_FAN_WIDTH`-wide
otherwise** rule forbids. One amendment, one build unit, three queue entries
pointing at it; `check-amendment-queue` arm (c) requires every ref to resolve and
every amendment to pair with an entry, which many-to-one satisfies.

**This amendment changes three components' contracts** — delegation-kit's (the
protocol, the guard, the knob, the test lane), guard-kit's (one clause in §The
guard framework's posture taxonomy), and lifecycle-kit's (one sentence in
`templates/lead.md` shrinks to a citation). It therefore carries the audit-stage
trigger by itself, independently of how many amendments sit on disk
(lifecycle-kit/SPEC.md §check-stage-entry, assertion C, single-amendment arm).

## What this amendment inherits and may not re-litigate

- **TRAJECTORY.md's closed ruling of 2026-08-06**: the iteration takes token
  waste as its subject. These three units are the *reduction* half; batch 1's
  `stage-fanout-burn-unbilled` is the measurement half.
- **Operator direction, 2026-08-03** (`fork-dispatch-prohibition`): the
  prohibition is stated in `templates/agent-execution.md` — the surface every
  delegating session loads — **per dispatching context, each with its own
  grounds**, under one unifying ground; the lead template's sentence shrinks to a
  citation with its tier-economics grounds as one named instance. That fixes
  *where* and *in what shape*. It does not fix the enforcement mechanism, which
  is what this amendment rules.
- **The probe finding of 2026-08-06** (`subagent-parent-addressing`): upward
  addressing is *impossible*, not merely unstated; a child's sends to an invented
  dispatcher name and to its own type both failed, only `to: "main"` succeeded,
  and neither level knows its own identity or its parent's. Option (4) — "state
  the address a child uses" — is refused on that evidence and stays refused here.
- **`stage-fanout-burn-unbilled`'s refusal of the dispatcher-minted attribution
  key** (drift-kit/SPEC-fanout-attribution.md §The dispatcher-minted key, ruled
  out). Nothing here revives it as an *attribution* mechanism. The durable
  artifact this amendment prescribes is an escalation channel, not an
  attribution key, and it is chosen for the opposite reason: attribution needs a
  key that cannot be forgotten, while escalation needs a route that exists at all.

## The correction this amendment owes the queue

`fork-dispatch-prohibition` carried one remaining `[design-pending]` cost:

> the kit-shaped form must ship as a delegation-kit template with a consumer
> binding, not a repo-local script, else the doctrine ships without its
> enforcement.

**That is not an open design question. It is a pattern this repo has shipped
twice**, and the entry is corrected on promotion:

1. `templates/agent-budget-guard.sh` is a delegation-kit template consumer-copied
   into the gates dir and registered under `PreToolUse` matcher `Agent`. The
   copy diverges from the template by one comment line. Registration is the whole
   opt-in; unwired, the guard is inert (§Layout and configuration).
2. `guard-kit/templates/escalation-guard.sh` is the closer precedent: a
   *doctrine* guard on a non-`Bash` matcher, advisory, tested by its own second
   decision table, with its provenance seam explicitly ruled — the kit owns the
   grammar, the consumer owns the roster. This repo leaves it unwired by design
   (guard-kit/SPEC.md §escalation-guard), so it is precedent for the
   *template-plus-binding-mechanism* shape, not for a live wired instance; delta
   9 below follows `agent-budget-guard`'s wired precedent instead, so the
   blocker's "ships with its enforcement" half rests on citation 1 and delta 9,
   not on citation 2 alone.

So the blocker resolves by citation rather than by design, and the entry's own
`[design-pending]` framing was the last thing standing between it and a build.
The pinned-surface half was already answered at scope and re-verified here:
`scripts/settings-pins.conf` pins `.autoMemoryEnabled` and
`.env.CLAUDE_CODE_DISABLE_AUTO_MEMORY`, neither under `.hooks`, so registering a
second hook reds nothing.

## The chokepoint — what a `PreToolUse` hook can actually read

The three entries all propose enforcement at "a `PreToolUse` hook on the dispatch
matcher". That channel is live (`agent-budget-guard.sh` runs on it) and it
re-arms at every depth — the protocol template already states so. What was never
established is **what the hook can see**, and two of the three rules turn
entirely on that. It was established here, from the harness's published hook
contract rather than from memory:

A `PreToolUse` payload carries `session_id`, `prompt_id`, `transcript_path`,
`cwd`, `permission_mode`, `hook_event_name`, `tool_name`, `tool_input`, and
`tool_use_id`; and, **only when the hook fires inside a subagent**, `agent_id`
and `agent_type`. For the dispatch tool, `tool_input` carries the dispatch's own
parameters — `subagent_type`, `isolation`, `model`, `description`, `prompt`.

Three consequences, each load-bearing below:

- **The fork ban has an exact trigger**: `tool_input.subagent_type`.
- **The read-only claim has an exact trigger**: `tool_input.isolation`.
- **Depth has an exact, *documented* discriminator**: `agent_id` is present iff
  the dispatching session is itself a subagent, so its presence means the call
  about to be made creates a grandchild. This is a published field of the hook
  contract, not an inferred shape.

**Ruled out: deriving depth from the environment.** `CLAUDE_CODE_CHILD_SESSION`
is set in this session, and reading it would have been the obvious move. It is
wrong: lifecycle-kit/SPEC.md §bin/session-id.sh already records that this harness
sets that flag in top-level sessions too — "verified, not trusted" — and
`CLAUDE_CODE_SESSION_ID` carries the *root* uuid for a child, so a depth-1 and a
depth-2 dispatcher are indistinguishable by environment alone. The env route
would have shipped a rule that silently mis-fires on top-level sessions.

**Ruled out: deriving depth from the meta layer.** `transcript_path` is in the
payload, and batch 1 established that `<transcript-basename>.meta.json` carries
an exact `spawnDepth`. That route works and is deliberately not taken: it couples
a hook to an *uncontracted* harness artifact to answer a question one
*contracted* boolean already answers. Batch 1 accepts that coupling because it
needs the whole spawn forest and no contracted field supplies it; this amendment
needs one bit and gets it under contract. The two units' opposite choices are
consistent, and the axis is what the contract supplies, not appetite for risk.

## Why an oracle at all, when the rules are already written down

This is the amendment's central ruling, and delegation-kit's own SPEC already
argues it. §Operative residency closes on the finding that a rule restated where
its bound actor reads it is *reachable, not obeyed* — "Every restatement this
sanction licenses is a request delivered where it is read. Weigh it as one …
never a substitute for an oracle where one is buildable." §One template, a
resident pointer states the matching structural fact: a **dispatched** role fires
no trigger that loads this protocol, "which is how a posture nobody disputes
produced three incidents."

All three entries are instances of exactly that, and each is attested:

- a fork dispatched as a read-only audit completed the whole stage, commits
  included;
- four forks dispatched with prose-only "read-only, no edits" instructions, one
  of which committed unreviewed on the shared branch;
- fan-outs at two stages across two iterations messaged the lead instead of their
  dispatcher, which no prompt wording could have fixed.

§Operative residency also records that **no gate is owed** for the turn-ending
rule, on the honest ground that "no check can read a session's choice to end a
turn — the act leaves no tracked artifact." That reasoning is what makes the
dispatch case different and is why an oracle is owed *here*: a dispatch call is
not a tracked artifact either, but it is a **tool call**, and a tool call has a
chokepoint the harness fires a hook on. The generalization to state is the one
the three entries converge on independently:

> **Where a rule binds an act that leaves no tracked artifact, the enforcement
> question is whether the act passes a chokepoint — not whether it reaches the
> tree. A dispatch does. A turn-end does not.**

That sentence is the amendment's contribution to the doctrine, and it is what
lets the same reasoning be reused rather than re-derived at the next such rule.

## What changes

### 1. The fork prohibition, stated per dispatching context

**{design-bearing}**

`templates/agent-execution.md` gains a bullet whose bold lead-in is
**Never dispatch a fork to narrow a child**, stating the unifying ground first —
*a fork is wrong exactly when the dispatch's purpose is that the child does less
than the parent* — then the per-context instances, each with its own grounds:

- **a read-only audit or survey**, whose purpose is narrower *authority*: a fork
  inherits the parent's full context and toolset and disclaims nothing, so the
  narrowing exists only as a sentence in the prompt;
- **a tier-split oracle**, whose purpose is narrower *cost*: a fork inherits the
  dispatcher's model, which is exactly the split the posture exists to make
  (lifecycle-kit's lead template is the named instance);
- **a rule-injection dispatch**, whose purpose is a *different* brief: attested in
  `native-first-port-cohort`, where five close-stage forks each carried the line
  "stated here because your agent definition does not carry it" — fork was chosen
  to inject rules the roster types lack, so every one received a self-contained
  brief and still paid to re-materialize its parent's context.

The bullet states the boundary honestly: a fork stays correct where the child
does the **same job at the same authority** and only parallelism or isolation is
wanted. It also states why the guard below blocks anyway — see delta 5.

The measured cost rides as the rule's grounds, not as prose: ten of that
iteration's twenty-four dispatched agents were forks, 20.90 USD of roughly
159 USD priced burn.

### 2. The lead template's sentence shrinks to a citation

**{mechanical}**

`lifecycle-kit/templates/lead.md` currently states the rule locally: *"The oracle
must be a fresh dispatch, never a fork: a fork inherits the dispatcher's model,
which is exactly the tier split this posture exists to make."* It becomes a
citation of the template's **Never dispatch a fork to narrow a child** rule,
keeping only what is local to the lead posture — that the tier split is the
narrowing at stake here. This is content-tiering: one rule, one owner, and the
lead template names the instance rather than restating the rule.

### 3. A read-only claim is a dispatch-shape claim, not a sentence

**{design-bearing}**

`templates/agent-execution.md` gains a bullet whose bold lead-in is
**A read-only claim is made by isolation, not by sentence**: a dispatch whose
brief says "read-only, no edits" and whose *shape* grants write reach has made no
claim at all, because a subagent inherits its toolset from its type regardless of
instruction text, and every type available for audit-shaped work carries write
tools or at least a shell reaching `git`. The claim is made by
`isolation: worktree`, whose commits and index are the child's own.

**The existing prose must be rewritten, not supplemented — this is the
load-bearing correction of this delta.** The **Resume journal — agent writes,
scratch reset sweeps** bullet currently ends: *"Reserve the journal / worktree
isolation for agents that **mutate files**."* That clause is reasoned entirely
about a background agent's *journal write* failing, but it reads as *read-only
fan-outs need no isolation* — which is precisely the reading that licensed the
attested unreviewed commit. The rewrite separates the two purposes isolation
serves, which the current text conflates:

- **an own index**, for an agent that *will* commit — already stated in the
  **Serialize on shared files** bullet, unchanged;
- **write confinement**, for an agent *claimed* read-only — new, and the reason
  the claim is now a shape.

The journal caveat itself survives intact and is *not* overturned: for a
read-only fan-out the return value is still the contract, and the parent still
owes the durable landing of what it received. Only the sentence that bundled
worktree isolation into that caveat is rewritten.

*Why the cost objection does not land.* Requiring a worktree per read-only
fan-out looks like a checkout per dispatch. It is close to free, and it pays for
itself twice: the harness auto-cleans a worktree that is unchanged, so the
worktree survives **iff** the agent was not in fact read-only. The isolation is
simultaneously the confinement and the detector.

### 4. The upward-channel contract

**{design-bearing}**

`templates/agent-execution.md` gains a bullet whose bold lead-in is
**A child's only upward route is a durable artifact**, stating the probe's
finding as the contract's ground and the four dispositions the probe re-ranked:

1. **A durable artifact, for anything mid-run.** The dispatcher mints a path,
   names it **absolute into the main checkout** in the prompt, and the child
   writes as it goes. Already this repo's idiom, now the only upward route that
   exists, and stronger than a message would be — a file survives a child that
   dies before returning. A stop signal takes the same shape: a sentinel the
   child checks, cooperative by construction, so a wedged child stays out of
   reach and the contract says so.
2. **Return-value only, otherwise.** A fan-out that needs no mid-run channel
   needs no artifact; the return value is the contract, which is the caveat
   already in §Resume journal.
3. **The downward half is a dispatch-shape rule.** A dispatcher that wants
   control over its fan-out dispatches in the background and retains the handle;
   the probe corrected the wider reading that the channel is absent both ways —
   what failed was a dispatcher blocked on a foreground dispatch, holding neither
   the handle nor a turn.
4. **Stating the address a child uses is refused**, and the refusal is kept
   rather than deleted because both original filings reached for it first. There
   is no address: neither level knows its own identity or its parent's.

Two corollaries the probe found are stated with it, because each silently
defeats a workaround a reader would otherwise invent: a message reaching `main`
is attributed to the sender's **type**, so a fan-out wider than one is ambiguous
at the receiving end; and a child resumed by its dispatcher's message still
cannot reply, so being addressed creates no return path. A third cause is stated
as a live hazard: the message and stop tools are **deferred** for a dispatched
agent — callable only after a discovery call it must know to make — so a child
needing to escalate may not hold the channel at all.

**Provenance seam.** Addressing is harness vocabulary. The kit states the
*obligation* — a dispatcher that gives mid-run-channel-shaped work grants a
durable path — and leaves the handle and the path to consumer config. That is
also why disposition (4) could never have cleared the seam and why (1) and (3) do.

### 5. `templates/agent-dispatch-guard.sh` — the new guard

**{design-bearing}**

A second `PreToolUse` hook on matcher `Agent`, beside `agent-budget-guard.sh`,
composed from `guard-kit/lib/guard.sh` primitives. It reads the payload once and
runs three rules in order.

**Why a second guard rather than more rules in the budget guard.** Four reasons,
each independently sufficient: the budget guard blocks on a **transient**
condition and is deliberately knob-overridable, while these block on a
**permanent** doctrine violation that must not be per-dispatch overridable; the
budget guard reads no stdin at all; a consumer may want either enforcement
without the other, and hook registration is the only opt-in valve there is; and
their test lanes take different payload grammars. The harness fires every hook
registered on a matcher and any exit 2 blocks, so two guards compose without a
dispatcher.

| # | rule | trigger | decision |
| --- | --- | --- | --- |
| D1 | fork ban | `tool_input.subagent_type` is `fork` | block |
| D2 | isolation claim | `tool_input.subagent_type` ∈ `DELEGATION_KIT_READONLY_TYPES` and `tool_input.isolation` is not `worktree` | block |
| D3 | depth advisory | payload carries `agent_id` | advise |

Order is load-bearing only between D1/D2 and D3: a blocked dispatch needs no
advisory, so D3 runs last. D1 precedes D2 because the fork ban is unconditional
and its message is the more specific one for a dispatch violating both.

**D1 blocks unconditionally, and the reconciliation is stated rather than
elided.** Delta 1's doctrine names a case where a fork stays correct — same job,
same authority, parallelism or isolation wanted — and a hook cannot read intent.
Operator direction rules the block unconditional on the ground that no sanctioned
fork use exists in *this* doctrine, and that is honored here. The valve is
**unwiring the hook**, not a knob: a per-dispatch override is precisely the
honour system these three entries exist to end, and a knob would restore it under
a better name. The block message names the two lawful alternatives so the
corrective rides in the rejection, as guard-kit requires of every block message.

**D2 is inert until configured**, by construction: the default roster is empty
and the kit ships no agent-type names (delta 7). An inert D2 reports nothing —
it is a hook, not a gate, so there is no clean line to print — and the SPEC states
the inertness so a consumer does not infer coverage it has not configured.

**D3 advises rather than blocks**, and both halves are deliberate. It cannot
block: a nested dispatch is legitimate and common — a stage session's read-only
fan-out *is* a grandchild dispatch — so blocking would remove a capability rather
than confine one. It must not be silent: this is the one rule whose bound reader
provably never loads the protocol template, so the advisory is the *only* delivery
of it. The message states the constraint operatively — this child cannot reach
you mid-run; give it return-value-only work, or grant it a durable path in the
main checkout, named absolutely in its prompt — and cites its owner, which is
§Operative residency's (a)–(c) shape applied at a hook instead of a doc.

*Ruled out for D3: matching the prompt text.* A rule keyed on whether the prompt
"names a path" fires on the word and teaches dispatchers to game the wording,
which buys a green hook and no channel. The `agent_id` trigger is exact and
unforgeable; the message carries the judgment the trigger cannot.

### 6. Degradation — fail-open, but loud

**{design-bearing}**

guard-kit/SPEC.md §The guard framework currently names two postures: fail-open as
the default, and one sanctioned fail-closed shape — "a deny-guard whose hook
*matcher* already proves the tool identity (see wakeup-guard): there, a logging or
parse failure still denies."

This guard fits neither, and the gap is real rather than a technicality. Its
matcher proves the *tool*, as wakeup-guard's does — but wakeup-guard's rule *is*
the tool, so a parse failure that denies costs nothing. Here the rule needs a
**field**, so a parse failure that denied would wedge every dispatch in the
consumer whose primary token lever is delegation. A parse failure that passed
silently would instead ship the exact defect these entries name: an unenforceable
claim that reports success.

**Ruled: a third posture, and it is named in guard-kit's taxonomy** —
**fail-open-but-loud**, for a deny-guard whose matcher proves the tool but whose
rule turns on a payload field. On an absent `jq` or an unparseable payload the
guard allows the dispatch and emits a `guard_advise` naming what it could not
enforce. This is the same rule batch 1 applied to the meter — *a degraded or
absent measurement is visible, never silently folded* — reached independently on
the other side of the iteration, and the two are worth reading together.

| what is missing | behavior |
| --- | --- |
| `jq` absent | allow; one advisory naming the unenforced rules |
| payload unparseable, or `tool_input` absent | allow; same advisory |
| `subagent_type` absent from a parseable payload | D1 and D2 do not fire (nothing to match); D3 unaffected |
| `DELEGATION_KIT_READONLY_TYPES` unset | D2 inert; D1 and D3 unaffected |
| `guard-kit/lib/guard.sh` not vendored | the guard exits 0 before sourcing, as both shipped guards already do |

### 7. `DELEGATION_KIT_READONLY_TYPES` — the one new knob

**{mechanical}**

An array of agent-type names the consumer dispatches for read-only work;
default **empty**, in which case D2 is inert. It lives in
`templates/delegation-config.sh` with the kit's other arrays and joins the knob
roster in §Layout and configuration.

The default is empty because it must be: **every entry is the consumer's own
agent roster, never the kit's** (CLAUDE.md §The provenance seam). A kit literal
here would publish a consumer's agent vocabulary, and the same reasoning already
governs `GUARD_KIT_BREADTH_PROBES`, which ships no default probes for exactly
this reason.

**Deliberately not knobs**, each ruled out for a stated reason:

- *A fork-type-name knob, and an accepted-isolation-values knob.* `fork` and
  `worktree` are the **harness's** dispatch vocabulary, and a guard template
  carries its harness's tool vocabulary as literals — `wakeup-guard.sh` matches
  `ScheduleWakeup`/`CronCreate` literally, and the whole payload shape is
  harness-specific anyway. A consumer on a different harness rewrites the guard;
  it does not reconfigure it. Knobbing these would imply a portability the
  mechanism does not have — the same reasoning batch 1 used to refuse a
  meta-filename knob.
- *A warn-only or per-rule opt-out.* A doctrine that can be downgraded per
  session is the honour system all three entries exist to end. Registration is
  the valve, at the granularity of the whole guard, and that is the granularity
  a consumer should have to reason about.
- *Accepting `isolation: remote` as satisfying D2.* A remote agent plausibly
  cannot touch the local index, which would make it satisfy the rule's actual
  purpose — but that is unverified from here and remote availability is gated, so
  it is excluded with its reason recorded rather than admitted on plausibility.
  A consumer with remote availability that wants it accepted is asking for a
  doctrine change, not a config change.

### 8. The test lane

**{mechanical}**

A decision table in delegation-kit's own bespoke lane, following
`budget-guard-cases.tsv` exactly: `usage-tests/dispatch-guard-cases.tsv`, rows of
`<decision><TAB><subagent_type><TAB><isolation><TAB><nested><TAB><desc>`, driven
by `bin/run-dispatch-guard-tests.sh`, which builds the payload per row and
asserts the exit code and output class (`block`/`advise`/`fallthrough`).

It takes delegation-kit's lane rather than guard-kit's `guard-tests/` because
guard-kit's two tables are keyed on a command and on a to+message, neither of
which can express a dispatch's parameters — the same reason the budget guard has
its own runner. The runner strips ambient `DELEGATION_KIT_*` at every invocation
and exports a poison value, the discipline `run-budget-guard-tests.sh` already
uses so a consumer's live roster cannot leak into the fixture.

Every rule carries a firing and a non-firing case (the fixture-pair discipline,
transplanted), plus the degradation arms:

- D1: a `fork` dispatch blocks; a typed dispatch does not.
- D2: a rostered type without `isolation` blocks; the same type with
  `isolation: worktree` does not; an unrostered type without isolation does not;
  **and the empty-roster case, which must not block** — the assertion that proves
  D2 ships inert rather than ships broken.
- D3: a payload carrying `agent_id` advises; one without it falls through.
- Degradation: an unparseable payload **advises rather than blocks** — the
  assertion that makes delta 6's posture testable rather than asserted, and the
  one that must not be dropped for brevity.

The runner joins the per-kit fixture-runner battery in README.md §This repo,
governed, in the same commit as the test — a kit's fixture-runner line is read
from `git ls-files` by `check-kit-registration` and is never a follow-up.

### 9. This repo's consumer wiring

**{mechanical}**

Copy the template to `scripts/agent-dispatch-guard.sh`, register
`bash scripts/agent-dispatch-guard.sh` as a second `PreToolUse` entry under
matcher `Agent` in `.claude/settings.json`, and set
`DELEGATION_KIT_READONLY_TYPES` in `scripts/delegation-config.sh` to this repo's
read-only roster. Dogfooding is day-one; an unwired guard proves nothing, and
this repo is where all three defects were attested.

The roster names the types this repo dispatches for audit-shaped work. Note that
a type with no `Edit`/`Write` tools still reaches `git` through its shell, so
"has no write tools" is not a reason to leave a type off the roster — the entry's
own finding, and the reason D2 keys on the consumer's declared intent for a type
rather than on the type's toolset.

### 10. Citation alignment

**{mechanical}**

Three new bullets means three new bold lead-ins, and `check-rule-citation`
resolves every `the template's **<name>** rule` citation in SPEC §The delegation
model forward into the template. The paragraphs delta 11 adds to that section use
the citation grammar, so the lead-ins and the citations land in one commit or the
gate reds — verbatim minus the trailing period, and the gate is the oracle rather
than the eye.

## Producers and consumers

**New interface: the dispatch guard's hook decision.**

- *Producer* — `templates/agent-dispatch-guard.sh`, fired by the harness on every
  `Agent` tool call once the consumer registers it. Its enabling config is the
  registration; for D2 alone it is additionally `DELEGATION_KIT_READONLY_TYPES`,
  which delta 9 sets in this repo, so the deployed configuration that must set it
  does exist and D2 is not dead-except-in-tests. D1 and D3 need no config at all.
- *Consumer* — the harness's `PreToolUse` protocol: exit 2 with stderr denies the
  call and returns the message to the dispatching model;
  `hookSpecificOutput.additionalContext` feeds an advisory into that model's
  context. Both are `guard-kit/lib/guard.sh` primitives, unchanged.
- *Readers, each at a named transition:*
  - the **dispatching session**, at the dispatch call, which is the transition the
    whole design exists to reach — including a *dispatched* session dispatching
    its own fan-out, the reader §One template, a resident pointer records as
    reachable by no other trigger;
  - the **`/economics` narrative** and the **stage-economics trend log**, at
    close, where the fork tier's spend either falls or the guard is unwired —
    batch 1's fan-out row is what makes this amendment's effect measurable rather
    than asserted, and the two units meet exactly there;
  - the **supervisor**, reading a block in a dispatched agent's report.

**No new payload field is minted** — every field read is one the harness already
publishes, so the "every field has a named reader" obligation attaches to the one
new *config* field instead: `DELEGATION_KIT_READONLY_TYPES` is read by D2 at the
dispatch call and nowhere else, and if delta 9 did not set it the knob would be a
field with no reader and would have to be removed with D2.

**Nothing is produced on the lifecycle side.** No stamp, no cursor, no state
file. Delta 2 removes a sentence from a lifecycle template and adds a citation;
lifecycle-kit's contracts are otherwise untouched.

## Existing sections updated

Each named with the delta that owns it.

1. **`templates/agent-execution.md`** (deltas 1, 3, 4). Three new bullets, and the
   **Resume journal — agent writes, scratch reset sweeps** bullet's closing
   sentence rewritten per delta 3. The rewrite is the item most likely to be
   under-done: a merged template that still reads *reserve worktree isolation for
   agents that mutate files* has failed delta 3 whatever else it added.
2. **delegation-kit/SPEC.md §The delegation model** (deltas 1, 3, 4, 5, 6). The
   section's stated job is mechanism contracts plus "only the rationale that
   earns spec residency (a failure surface, a calibration history, a bound that
   is correctness rather than preference)". Three attested failure surfaces and
   the chokepoint reasoning qualify; the rule text itself stays in the template
   and is cited by name. **The chokepoint's payload-field roster lands here
   verbatim, not merely gestured at** — §The chokepoint — what a `PreToolUse`
   hook can actually read is where D3's whole basis lives (`agent_id`'s presence
   being a *documented* field rather than an inferred one), and losing that
   enumeration and its sourcing at merge would leave D3's trigger unexplained to
   a reader who never saw the amendment. Carry the honest limit with it: the fact
   is sourced from the harness's own published hook contract, fetched rather than
   asserted from memory, not from anything in this tree — the same footing as
   `CLAUDE_CODE_CHILD_SESSION`'s "verified, not trusted" treatment in
   lifecycle-kit/SPEC.md §bin/session-id.sh — so a future harness revision
   changing the payload shape is a drift this SPEC cannot self-detect; only
   re-reading the hook contract catches it.
3. **delegation-kit/SPEC.md §One template, a resident pointer** (delta 5). Its
   honest limit currently reads that "the guard enforces budget mechanically, not
   protocol literacy". After this change that is too narrow — a second guard
   enforces three protocol rules mechanically. The limit survives, tightened: the
   guards enforce the rules that pass a chokepoint, and protocol literacy beyond
   those stays unenforced.
4. **delegation-kit/SPEC.md §Operative residency** (delta 5). Its closing
   "**No gate is owed, and not for budget**" paragraph is *not* overturned — it
   is about the turn-ending rule, which leaves no chokepoint — but it now sits
   beside a case where an oracle *was* buildable, and the distinguishing
   sentence from §Why an oracle at all lands here so the two paragraphs do not
   read as each other's contradiction. This is the same treatment §The delegation
   model already gives the durability rule and the read-only-fan-out caveat,
   "because the rule and the caveat otherwise read as each other's contradiction
   and the next reader re-litigates them."
5. **delegation-kit/SPEC.md §Layout and configuration** (deltas 5, 7, 9). The
   layout block gains the guard template and the test lane;
   `DELEGATION_KIT_READONLY_TYPES` joins the knob roster; and the paragraph
   beginning "`agent-budget-guard.sh` is not a gate" gains its sibling — two
   hooks on one matcher, each registered independently.
6. **delegation-kit/SPEC.md §Testing** (delta 8). The new runner and table join
   the rostered lanes.
7. **guard-kit/SPEC.md §The guard framework** (delta 6). The fail-open paragraph
   names a third posture. Its closing cite-only paragraph — "delegation-kit's
   `agent-budget-guard.sh` is a second framework consumer … no guard-kit
   mechanism moves for it" — gains the dispatch guard as a third consumer, and
   must be reworded: guard-kit mechanism *does* move for this one, by exactly one
   clause, so leaving that sentence as written would make the SPEC false.
8. **lifecycle-kit/templates/lead.md** (delta 2). The never-a-fork sentence
   becomes a citation.
9. **README.md §This repo, governed** (delta 8). The new fixture-runner line.
10. **CLAUDE.md §Agent execution** — checked and **unchanged**. It carries the
    resident pointer only, and the pointer still resolves; adding the guard there
    would restate a rule its trigger already loads, which §Operative residency's
    anti-licence clause refuses.

## What reaches a sibling unit

Stated here because a later author working one unit will not read the others.

- **To `stage-fanout-burn-unbilled` (batch 1, already authored into its own
  amendment file, `drift-kit/SPEC-fanout-attribution.md` — not yet merged into
  drift-kit/SPEC.md, same as this one).** Nothing in this amendment changes the
  meter, the fan-out row, or the attribution key. The relationship runs the
  other way: batch 1's fan-out row is the instrument that measures whether this
  amendment worked, so the two are measurement and treatment of one iteration.
  **The row is an aggregate proxy, not a fork-labeled figure**: its `<stage>`
  column carries no dispatch-type dimension, so a fork's cost and a typed
  dispatch's cost under the same anchor sum into one number. If this amendment
  lands and the anchor's fan-out total does not fall in the next close's trend
  log, the guard is unwired or D2's roster is empty — check the registration
  before re-opening the doctrine; a fallen total is consistent with the guard
  working but, absent a type dimension, is not on its own proof of it.
- **To `cross-stage-census-duplication`** (this batch's second amendment,
  `lifecycle-kit/SPEC-survey-carryforward.md`). Delta 4's durable-artifact
  contract is the *escalation* channel — mid-run, dispatcher-minted, per-dispatch.
  The survey carry-forward artifact is a *hand-off* — written at one stage, read
  at a later one, discoverable by content rather than by session. They are
  deliberately **not** the same artifact, and §Resume journal's per-session-name
  discipline is why: conflating them would make a journal's name meaningless.
  The survey amendment cites delta 4 rather than extending it.
- **To `incident-recurrence-promotion-signal`** (deferred). The fork cascade
  recorded there is D1's population. Nothing here implements that entry's
  promotion mechanism.
- **To `falsifiability-self-revert-reminder`** (deferred). Its entry records
  itself as "Related to `fork-dispatch-prohibition` — both are cases where a
  dispatched agent's authority is narrower than what some in-context text invites
  it to do." That relation is real and this amendment does **not** close it: its
  hazard is an in-context *harness message*, which passes no dispatch chokepoint,
  so §Why an oracle at all's test classifies it with the turn-end case, not with
  these three. Recorded so that entry is not read as incidentally fixed.

## Definition of Done

- [ ] **Causal completeness** — every new interface has a named, reachable
      producer and a named consumer; the one new config field has a named reader
      at a named transition. *(No new payload field; the guard's producer,
      consumer, and three readers are named above.)*
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged specs read as
      coherent documents a reader who never saw the amendment can use alone. The
      ten targets in §Existing sections updated are the checklist.
- [ ] **The two prose corrections landed, not supplemented** — the
      `agent-execution.md` worktree-isolation clause (delta 3) and guard-kit's
      "no guard-kit mechanism moves for it" sentence (delta 6). Either left
      standing is a failed merge, because each would make its document false
      rather than merely incomplete.
- [ ] **Three queue entries move together** — `fork-dispatch-prohibition`,
      `read-only-fanout-unenforceable`, and `subagent-parent-addressing` all
      point at this file, so all three move to `## Done` in the merging commit;
      leaving one behind dangles a `[spec:]` ref at a deleted file.
- [ ] **Amendment deleted** — this file removed on merge. The
      *none-remain-for-the-component* half is discharged at the **iteration**,
      not at this commit (canon-kit/SPEC.md §Merging an amendment, step 3): this
      is delegation-kit's only amendment in flight, so it is satisfiable here,
      but the assertion is stated in its iteration-scoped form so it is not
      copied into a later sibling as an unsatisfiable one.
- [ ] **The guard is wired in this repo** (delta 9) — an unwired guard passes
      every fixture and enforces nothing, which is the exact failure shape these
      three entries describe. Verify by a real dispatch, not by reading settings.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      through `bin/file-gap.sh` (a build-time causal gap is resolved that
      session, not deferred).
