# SPEC amendment: the turn-end liveness probe

Closes `subagent-stop-liveness-hook-wiring`. delegation-kit/SPEC.md §Operative
residency rules that a turn-end passes **no chokepoint** — *"A dispatch does. A
turn-end does not"* — and the enforcement of the waiting rule was relocated to
guard-kit rule 14 on exactly that ground. A `SubagentStop` hook is a chokepoint
at the turn-end, so it is the one candidate that would falsify the axis, and the
entry's load-bearing unknown is whether the harness already **defers** the stop
while a background child is live: if it does, the whole class dissolves.

The operator authorized the **logging-only** variant on 2026-08-20 — a hook that
records what it would have blocked and blocks nothing. **A blocking hook is a
second authorization and is not implied by it**, and nothing in this amendment
implies one.

## What this amendment is and is not

It is a **probe with a durable artifact**, not an enforcement mechanism. It mints
a template, a consumer copy, two knobs, a log and a settings registration, which
is why it is feature-class; but the thing it delivers is an *answer*, and the
entry is entitled to return that a capped refusal buys nothing.

The event is **`SubagentStop`, never `Stop`.** A dispatched stage session is a
subagent, so its turn end fires `SubagentStop`; every attested firing of the
waiting rule was a dispatched session. `Stop` is deliberately **not** registered:
registering it would widen the subject past the ruling, and the main-session turn
end has no attested firing to justify it.

## What changes

### (1) `delegation-kit/templates/subagent-stop-liveness.sh` — the hook

**Design-bearing.** A `SubagentStop` hook that, on every firing:

1. reads its payload from stdin;
2. asks the liveness reader whether any launch record under the scratch dir names
   a live producer;
3. appends **one line** to the log; and
4. **exits 0 unconditionally**, emitting no hook JSON at all.

**Emitting nothing is the design, not an omission, and it is what makes this
variant buildable today.** guard-kit's framework hardcodes
`hookEventName:"PreToolUse"` in every one of its three emitters
(`guard_advise`, `guard_allow`, `guard_rewrite`), so a hook that had to *speak*
at `SubagentStop` would need a primitive guard-kit does not have and whose event
scope its SPEC never states. A logging-only hook speaks nothing, needs no
emitter, and therefore **does not source guard-kit's lib** — which also keeps
delegation-kit from acquiring a dependency on guard-kit being vendored. The
blocking variant would need that primitive, and that cost is stacked on the
second authorization rather than paid here.

**It never wedges a turn.** Every failure path — an unreadable payload, an absent
liveness reader, an unwritable log — exits 0 with the failure recorded in the
line where it can be, and silently where it cannot. A probe that could hang or
refuse a turn end would be the blocking variant by accident.

### (2) The liveness reading reuses `check-producer-liveness`, and copies no grammar

**Design-bearing**, because the obvious alternative is the one delegation-kit
already refused. §The delegation model rules both record affordances declined,
on the ground that *"the generic reading affordance that would genuinely earn its
place already exists and is `check-producer-liveness`"*. A hook re-implementing
the `pid=<n> run=<key>` parse and the PID predicate would be a **third** copy of
a grammar evidence-kit owns, contradicting that ruling for no gain.

So the hook **invokes the reader** in its set mode over the scratch dir —
`check-producer-liveness <dir>` quantifies the per-record verdict over `*.run`
and already resolves *exit 2 wins over red wins over green* — and records its
exit status and first output line verbatim. Nothing about how a PID's liveness is
decided is seconded here; that stays evidence-kit/SPEC.md §The producer-liveness
lock's, exactly as rule 14's reach does.

**The consequence is a stated prerequisite rather than a hidden assumption**: a
consumer that wires this hook without evidence-kit vendored gets
`liveness=unavailable` on every line and a probe that answers nothing. That is
honest degradation, it is stated in the template's own header and in the SPEC
section, and it is preferable to a silent third parse that would work everywhere
and drift from its owner.

### (3) Two knobs

**Design-bearing** (the names and the defaults), **mechanical** to wire.

- `DELEGATION_KIT_STOP_LOG` — the probe's log. Default
  `${GATE_SDK_WORKFLOW_DIR:-.workflow}/subagent-stop-liveness.log`, the same
  deferral guard-kit's two logs already take.
- `DELEGATION_KIT_LIVENESS_CMD` — the liveness reader the hook invokes. Default
  `evidence-kit/checks/check-producer-liveness.sh`. A knob rather than a literal
  because a consumer's evidence-kit may sit elsewhere, and because an empty value
  is the supported way to run the hook with no reader at all.

**No scratch-dir knob is minted.** The record's home is `${GATE_SDK_TMP_DIR:-.tmp}`,
the cross-kit deferral three kits already take for exactly this directory. A
`DELEGATION_KIT_RUN_DIR` would be a fourth name for one path.

### (4) The log line, and every field has a reader

**Design-bearing** — the field set is the whole instrument.

One line per firing, appended, space-delimited `key=value` after a leading
timestamp:

```
<UTC ISO-8601>  event=<hook_event_name|->  session=<session_id|->  live=<yes|no>  verdict=<green|red|corrupt|unavailable>  records=<n>  keys=<comma-separated top-level payload keys>
```

- **timestamp** — read by the close-stage reader to order firings and to tell a
  fresh line from a stale one.
- **`event`** — the payload's own `hook_event_name`. Read at the first firing to
  confirm the event fires **at all** for a dispatched session and that it is
  spelled `SubagentStop`. This is the cheapest of the unknowns and it is worth a
  field because the entry's whole correction turned on the event's identity.
- **`session`** — the payload's `session_id` if present, `-` otherwise. Read when
  correlating a firing with the stage stamp that names the same session id, which
  is how a firing is attributed to a stage session rather than to an anonymous
  subagent.
- **`live`** — `yes` when the reader reported a live producer. **This is the
  decisive field**; see the honest limit below.
- **`verdict`**, **`records`** — the reader's exit class and the number of `*.run`
  records it saw. Read together: `records=0` makes a `live=no` uninformative,
  while `records=2 live=no` says records existed and their producers had exited.
- **`keys`** — the payload's top-level key set, nothing more. Read **once**, at
  the first firing, to settle what a `SubagentStop` payload carries without
  asserting anything about it in advance. Values are not logged: the entry has
  already settled that the payload names no background task, PID or shell id, so
  the key set is the whole of what remains unknown, and logging values would put
  transcript paths and prompt ids into a file for no reader.

No field is populated at a transition where it is not read, and no field is
carried that no line above names a reader for.

### (5) The consumer copy and its registration

**Mechanical**, but the registration is the part no agent session may author on
its own say-so, and it is here on the 2026-08-20 authorization.

- `scripts/subagent-stop-liveness.sh` — the copy, following the shape every hook
  in this tree already has: a `# spec:` line binding it to the owning SPEC
  section, and any divergence from the template marked `copy-divergence:`. No
  divergence is expected.
- `.claude/settings.json` gains a `SubagentStop` key registering
  `bash scripts/subagent-stop-liveness.sh`. `SubagentStop` takes **no matcher**,
  so the entry carries a `hooks` array alone — unlike every `PreToolUse` entry in
  the file, and unlike them it is not tool-scoped.
- `.gitignore` gains the log path, beside the two guard-kit logs it sits with.

**No settings gate observes any of this**, and it is recorded rather than assumed:
`check-settings-pins` asserts only the three pinned paths in
`scripts/settings-pins.conf` (none under `hooks`), `check-settings-paths`'s
subject is `permissions.allow[]` alone and never `hooks[].hooks[].command`, and
`check-memory-off` scans the memory surface. So adding the registration reds
nothing — and neither would a registration naming a script that does not exist.
That gap is filed (below), not left implied.

### (6) The close-surface declaration

**Mechanical.** The log is capture-tier — gitignored, advisory, drained by a named
reclaim path — which gate-sdk/SPEC.md §The workflow directory makes definitional
rather than optional, and `check-workflow-tiering` refuses a member that is
neither tracked nor ignored. It declares itself on the close-surface roster in
the section that owns it, in the established grammar:

```
close-surface: .workflow/subagent-stop-liveness.log advisory reclaim=: > .workflow/subagent-stop-liveness.log
```

`advisory` rather than `forced`, on the same reasoning guard-kit's friction log
takes: nothing refuses a close that skips it, and a visible skip is the honest
mode for a probe.

### (7) A new SPEC section owns all of it

**Design-bearing.** delegation-kit/SPEC.md gains **§The turn-end liveness probe
(template)**, sibling in shape to guard-kit's §escalation-guard (template): an
opt-in template, inert absent the consumer's registration, whose wiring no gate
observes and which a session therefore decides by reading the section rather than
by predicting a verdict.

The section owns: the hook's behavior and its unconditional exit 0; the two
knobs; the log grammar and each field's reader; the close-surface line; the
`check-producer-liveness` reuse and its prerequisite; the **question the probe
exists to answer**; and the honest limit below. It is the section the queue entry's
`[spec:]` ref resolves against at merge.

## The honest limit — the probe is asymmetric, and the build must not read it otherwise

**A `live=yes` line proves the harness does not defer the stop.** One firing with
a live producer settles the load-bearing unknown in that direction, and the class
stands.

**No accumulation of `live=no` lines proves that it does.** `live=no` is equally
consistent with *the harness deferred the stop*, *the session waited correctly*,
and *the session recorded nothing* — and the third is precisely the residue the
sibling unit `launch-chokepoint-liveness-record-write` exists for. Passive
accumulation therefore cannot return the finding that would dissolve the class,
and a build that waits for it will wait forever and then report the wrong thing.

**So the build's first act after wiring is one deliberate firing, not a wait.** A
dispatched session backgrounds a producer that runs long, writes its `<key>.run`
record, and then ends its turn. Two outcomes and both are results: the hook fires
immediately with `live=yes` → the harness does **not** defer, §Operative
residency's axis is confirmed at the turn-end too, and the blocking variant is the
only lever left; the hook does not fire until the child exits → the harness
**does** defer, the class dissolves, and this template's remaining value is the
evidence for retiring it. The deliberate firing is the probe; the log is the
instrument that makes it and every later accidental firing legible.

**A capped refusal buying nothing is a result, not a failure** — recorded here
because the entry is entitled to return it and because the logging-only variant
may make that the answer without a single block being written.

## The seam

**Kit mechanism:** the template, the log grammar, the two knobs, and the SPEC
section. Each is generic — a hook that asks a liveness question and writes a line
— and none names a project, a command vocabulary, or a term list.

**Consumer config:** both knob values, and the wiring itself. The log path and the
liveness reader's path are `DELEGATION_KIT_*` knobs with this repo's layout as
their defaults, per the config-via-env convention; the `.claude/settings.json`
registration is the consumer's own act, which is precisely why it needed the
operator and not an agent session.

**Private rule content: none, and there is none to keep out.** The hook reads a
PID and a run key. It logs no command, no prompt, no transcript path and no
payload values beyond a session id and an event name — delta (4) refuses the
values deliberately — so nothing consumer-specific can reach a kit file or a
committed log line by construction.

## Producers and consumers

**The log line.**
*Producer:* `scripts/subagent-stop-liveness.sh`, invoked by the harness at every
`SubagentStop` firing. Its enabling config is the `.claude/settings.json`
registration in delta (5) — a real deployed configuration in this repo, not a
test-only one, which is the point of the authorization: an unregistered template
produces nothing, and the entry has already spent one session proving that.
*Consumer:* the close stage, via the close-surface roster declared in delta (6),
which is what makes the log a surface a close must dispose of rather than a file
that accumulates unread. The **finding** it carries routes to
`turn-end-chokepoint-and-wait-primitive`, the entry holding this unit's other open
mechanism question and the threshold recurrence it feeds.
*Every field's reader at a named transition:* enumerated field by field in delta
(4). No field is added without one.

**The liveness verdict.**
*Producer:* `evidence-kit/checks/check-producer-liveness.sh` in set mode, resolved
through `DELEGATION_KIT_LIVENESS_CMD` and invoked by the hook at each firing.
*Consumer:* the hook, at the line-building transition, where its exit status
becomes `verdict` and `live`.
*Red condition:* the reader's own — exit 1 on a live producer, exit 2 on a record
it cannot parse, 0 otherwise. The **hook inverts none of it and propagates none of
it**: the hook's own exit is unconditionally 0, so a red reader produces a logged
red and a passing turn. This is the one place where a status is deliberately
dropped, and it is dropped because the variant is logging-only.

**The `SubagentStop` registration.**
*Producer:* the operator's authorization, landed as the settings entry in delta
(5). *Consumer:* the harness, which dispatches the hook.
*Red condition:* **none anywhere**, and that is delta (5)'s recorded finding
rather than an oversight — no gate in the tree reads `hooks[]`.

**Existing integration prose describing the prior flow** is inventoried below.
The load-bearing one is §Operative residency's chokepoint sentence, which is a
*claim about the world* this probe is built to test; it is updated to say so
rather than left reading as settled.

**No corpus is narrowed by this amendment**, so causal-completeness point 5's
narrowing clause is inert: nothing is pruned, no glob tightens, and the one
reader with a non-monotone verdict in the vicinity — guard-kit's decision table —
is untouched, because this hook adds no guard rule and changes no guard decision.

## Existing sections updated

Each names the delta that owns it.

- **delegation-kit/SPEC.md §The turn-end liveness probe (template)** — new; owns
  deltas 1–4, 6 and the honest limit (delta 7).
- **delegation-kit/SPEC.md §Operative residency** — the chokepoint sentence *"A
  dispatch does. A turn-end does not"* and the paragraph relocating enforcement to
  rule 14 are the prior-flow prose. They are **not** reversed — the relocation
  stands and rule 14 is unchanged — but the claim gains its live test: a
  `SubagentStop` hook is a turn-end chokepoint, the probe is registered, and what
  the axis rests on is now measurable rather than argued (deltas 1, 7).
  The wording must not overclaim in either direction before the probe returns.
- **delegation-kit/SPEC.md §Layout and configuration** — the knob roster gains
  `DELEGATION_KIT_STOP_LOG` and `DELEGATION_KIT_LIVENESS_CMD` with their defaults
  stated, and the layout block gains `templates/subagent-stop-liveness.sh`
  (deltas 1, 3). Both defaults are `${…:-tail}` deferrals or a literal path;
  `check-knob-citation` requires the citation and `check-knob-default-coupling`
  couples the literal, so the SPEC statement must be the exact literal the
  template carries.
- **delegation-kit/README.md** — re-read for the template roster and the kit's
  one-line description of what it ships; updated if it enumerates templates
  (delta 1).
- **delegation-kit/smoke/install.sh** — the consumer smoke copies this kit's
  templates into the scratch consumer; a new template belongs in that copy, and
  gate-sdk/SPEC.md §Consumer smoke's kit-landing checklist is what makes it owed
  (delta 1).
- **`.claude/settings.json`**, **`scripts/subagent-stop-liveness.sh`**,
  **`.gitignore`** — the consumer-side landing (delta 5).
- **evidence-kit/SPEC.md §check-producer-liveness** — **unchanged**. Set mode
  already takes a directory and already resolves the aggregate verdict; this
  amendment adds a caller, not a capability. Recorded so the merge does not go
  looking for an edit (delta 2).
- **guard-kit/SPEC.md** — **unchanged**. The hook sources no guard-kit primitive
  and adds no rule; the framework's `PreToolUse`-only emitters are cited as the
  reason for that, not amended (delta 1). Recorded because "a hook, therefore
  guard-kit" is the wrong inference a merge would otherwise reach for.
- **The `docs/` kit-SPEC mirror** — `docs/delegation-kit/SPEC.md` is a generated
  projection and stales on any edit above; the regen command and its freshness
  gate are rostered at docs/site-architecture.md §Generated projections and run in
  the merge commit (all deltas).

## What the build owes beyond the deltas

- **The deliberate firing** described under the honest limit, run once and its
  outcome recorded on `turn-end-chokepoint-and-wait-primitive`. Wiring without it
  buys an instrument and no reading.
- **The hook-path gap is already filed**, 2026-08-22 at this stage, in the
  committed gap inbox: nothing checks that a `hooks[].hooks[].command` path exists
  or is runnable. A renamed or deleted hook script reds nowhere and fails silently
  at run time, and this amendment adds the second registration that would be
  affected. It is the same gap the sibling amendment
  `guard-kit/SPEC-launch-chokepoint.md` names, filed once for both; the build owes
  nothing further on it.
- **No `guard-tests/` row and no fixture pair are owed** — the hook is not a
  guard-kit rule and not a gate. What it owes is the bespoke-unit-test lane if
  anything: a test asserting the unconditional exit 0 and the line grammar on a
  crafted payload, homed at `delegation-kit/gate-tests/*.test.sh` so
  `check-test-hermetic` reaches it.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`), discharged at the iteration
      rather than at the commit.
- [ ] **Removals propagated** — grepped every spec and template for the claim
      that the turn-end passes no chokepoint at all; nothing dangles and nothing
      overclaims ahead of the probe.
- [ ] **Gaps filed** — the hook-path gap above is filed once across the two
      sibling amendments; cross-component gaps discovered during the work are
      resolved that session, not deferred.
