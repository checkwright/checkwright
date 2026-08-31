# SPEC amendment: harness-integration arm

Ports the harness and git-hook template members `harness-template-port-residue`
holds — 12 files, 613 lines — off the shell substrate and into the gate binary,
as **harness-integration arms** of the non-gate class. The arm-kind fork is
closed (`ruled: harness-template-port-residue operator 2026-08-31 consult`;
TRAJECTORY.md §The closed rulings carries the ruling and the two refused
shapes). This amendment authors the contract that ruling was sequenced behind,
and its landing retires that TRAJECTORY paragraph.

**The member set is ruled too, and by a second authority on a second channel.**
`spec` put the arm's *class boundary* up rather than deciding it, because two of
the twelve members do not speak the hook protocol and one of those two — the
usage poller — would give the **installed** binary a path that reads an OAuth
credential file and reaches an external endpoint. That is a product-surface fact,
not a porting detail, and TRAJECTORY.md §The objectives rules nothing on the
installed binary's capability surface. The operator ruled **ship all twelve**
(`ruled: harness-template-port-residue operator 2026-08-31 lead-relay`): the
installed binary may carry that path. Everything else about the shape below is
the lead's own-authority calibration of the same date. The two are recorded apart
because neither is evidence for the other.

Home component: `gate-sdk/`, whose §The non-gate arm is the class this extends.

## What changes

### (1) The harness-integration arm, a named sub-class of the non-gate arm

gate-sdk/SPEC.md §The non-gate arm gains a **harness-integration arm**: a
bridged non-gate arm whose named caller is the **coding harness** rather than a
battery, a stage step, a regen command or a gate reaching it in process. {design-bearing}

It satisfies the class's three properties unchanged — `--`-prefixed and resolved
before the registry lookup, absent from `--list`; no descriptor, no
`gates.list` registration, no `good/`+`bad/` fixture pair; a named caller. What
it adds is that **its channels are the harness's, not the gate output
contract's**: stdin carries a harness payload, stdout carries whatever that
integration point reads, and the exit status means what the harness says it
means rather than what §Output contract says. That sentence is the whole of the
new property, and it is worth stating because every earlier member of the class
answers to a caller this project wrote.

**The family is forced, not chosen.** §The non-gate arm's forced-family test
settles it: `scripts/agent-dispatch-guard.sh:13-21` sources
`${DELEGATION_KIT_CONFIG_FILE:-<gates-dir>/delegation-config.sh}` and reads the
array `DELEGATION_KIT_READONLY_TYPES` out of it. A hardcoded top-level flag
receives no consumer override and would resolve platform defaults while silently
ignoring that roster — the difference between working and appearing to. Reading
that config inside the crate is the second-resolver refusal criterion 6 states.
So every member here is a **bridged-arm table** member, and its knobs reach it
through `gate_knob_env`.

**Three arms are minted, with their implementations, in this unit** — a spelling
written ahead of its implementation would be the reservation §The non-gate arm
refuses:

- `--hook <member>` — the **one dispatching hook arm** the ruling names. Six
  members (delta 2).
- `--statusline` — the harness's `statusLine` integration point. One member.
- `--usage-poll` — the usage snapshot refresher. One member.

**Why `--statusline` and `--usage-poll` are siblings of `--hook` rather than
members of it.** The ruled arm's contract is the *hook* protocol: a hook-JSON
envelope on stdout and the exit status as the harness's allow/block signal.
`statusline-usage.sh` writes an ANSI status line and the harness ignores its
exit status; `usage-poller.sh` has no harness event at all — its callers are
`DELEGATION_KIT_REFRESH_CMD` (`scripts/delegation-config.sh:15`) and the
`permissions.allow[]` grant a session invokes it through. Folding either into
`--hook` would make that arm's stated output contract "whatever the member's
integration point expects", which is not a contract. Each of the two satisfies
the class's three properties on its own and names its own caller, so each is a
member in its own right — the same reading by which `--run` sits in the table
beside the `--emit-` family without being one of them.

**The refused shapes stay refused and are cited, not re-argued**: one arm per
hook (eight arms sharing one protocol owe the class rule once, not eight times)
and hooks-stay-shell-as-a-class (a per-hook bash file is the PowerShell
duplicate the port exists to end). Both are TRAJECTORY.md §The closed rulings'.

**Two further shapes were refused at this amendment's own authoring, and the
second one names the fallback if the question ever returns** (lead, 2026-08-31,
own authority). *Folding the two non-hook members into `--hook`* is refused
because that arm's stated output contract would become "whatever the member's
integration point expects", which is not a contract. *Cutting to the six hook
members* is refused because it demotes the entry with 168 lines still owed, which
is the rate the port-only run exists to correct. The honest fallback is neither:
only `usage-poller` raises the capability question, `statusline-usage` being a
plain ANSI line with no credential and no network — so a returning question falls
back to **eleven members, 67 lines owed**, never to six.

### (2) The `--hook` arm's protocol and its six members

`--hook <member>` takes the **member name** as argv — not the hook *event*
name. {design-bearing}

The event name was weighed and refused: `.claude/settings.json` scopes hooks by
`matcher` inside an event, and this repo's `PreToolUse` carries three matcher
groups whose members are not interchangeable — `Bash` → `scripts/bash-guard.sh`
(declared `no-port`, permanently shell), `Agent` → the two dispatch guards,
`Write|Edit` → the workflow-state guard. An event-named arm would have to
re-derive the matcher from the payload, would fire on every tool call rather than
the matched ones, and could not express the **mixed substrate** the lead's
2026-08-31 ruling makes a fact: a shell member and a ported member sitting in the
same matcher group. The member name makes the port a one-for-one substitution of
the `command` value and leaves every matcher untouched.

**The channels.**

- **argv** — `<member>`, one token, from the closed roster below. An unknown
  member is exit 2 with the roster named, the `no_such_gate` shape.
- **stdin** — the harness payload, read whole and parsed with `serde_json`
  (already the crate's one dependency, `native/Cargo.toml:15`). An absent, empty
  or unparseable payload takes the member's own **degraded** path — the path each
  shell member already has for a missing `jq` — never a panic and never a block
  the member would not otherwise have issued.
- **stdout** — the hook-JSON envelope where the member emits one, serialized by
  `serde_json` rather than composed by `printf`. This **retires a live
  fragility**: `agent-dispatch-guard.sh:23` keeps its advisory literals free of
  any character JSON must escape *by convention alone*, because its degraded path
  hand-writes the envelope with no escaper. A member that emits no envelope
  writes nothing on stdout.
- **stderr** — the member's block or refusal text, which the harness shows.
- **exit status** — the harness's protocol, returned unchanged from the member:
  `0` allow or advise, `2` block or refuse.

**The roster, and each member's channels — the port is behavior-preserving
against this table, member by member.** Every row is read off the shell source
rather than off prose:

| member | event / matcher | stdin fields | stdout | exit |
|---|---|---|---|---|
| `agent-budget-guard` | `PreToolUse` / `Agent` | payload unused; verdict from `DELEGATION_KIT_VERDICT_BIN` | advisory envelope, or none | 2 block, else 0 |
| `agent-dispatch-guard` | `PreToolUse` / `Agent` | `.tool_input.subagent_type`, `.tool_input.isolation`, top-level `.agent_id` | advisory envelope (D3, degraded, nested) | 2 block (D1/D2), else 0 |
| `subagent-stop-liveness` | `SubagentStop` / none | `.hook_event_name`, `.session_id`, `keys_unsorted`, `.stop_hook_active` | none — a log line and stderr only | 2 refuse, else 0 |
| `escalation-guard` | `PreToolUse` / `SendMessage` | `.tool_input.to`, `.tool_input.message` | advisory envelope, or none | always 0 |
| `wakeup-guard` | `PreToolUse` / `ScheduleWakeup\|CronCreate` | whole payload, logged | none — stderr only | always 2 |
| `workflow-state-guard` | `PreToolUse` / `Write\|Edit` | `.tool_input.file_path` | advisory envelope on the degraded path | 2 block, else 0 |

Two members emit **no** envelope at all and speak the protocol through exit
status alone. That is inside the ruled contract rather than an exception to it:
"exit status as protocol" is the hook protocol's second channel, and a member
using only that channel is using the protocol.

**`escalation-guard` and `wakeup-guard` stay unwired in this repo**, as
guard-kit/SPEC.md §wakeup-guard (template) already rules; the port changes their
substrate and not their wiring, and `guard-kit/templates/settings-hooks.json` —
the only place `wakeup-guard`'s matcher is spelled in JSON anywhere in tree — is
an update target for that reason (delta 10).

**Two knob-pointed external commands stay external and stay spawned**, which is
the honest limit of this cut: `DELEGATION_KIT_VERDICT_BIN`
(`delegation-kit/bin/usage-verdict.sh`) and `DELEGATION_KIT_LIVENESS_CMD`
(`scripts/producer-liveness-reader.sh`) are members of **other** owed cohorts —
the kit-`bin/` class and the `scripts/` class — and the arm invokes each through
its knob exactly as the shell member does today, via `proc::run`. The
`timeout 10` bound `subagent-stop-liveness.sh:36-40` applies when `timeout(1)` is
present becomes an unconditional Rust-side wait bound, which removes one
optional program from the path rather than adding one.

### (3) The absent-binary behavior, which the port creates and must therefore settle

`exec_arm` (`gate-sdk/bin/run-gates.sh:57-71`) exits **2** when the binary is
absent or not executable. On a hook path that status is *block the tool call*, so
a tree that vendored the kits and has no binary for its platform — criterion 5's
omit-and-declare branch — would refuse every `Agent` dispatch and every
`Write`/`Edit`. {design-bearing}

Today this failure mode does not exist: the shell member is always present once
vendored. The port creates it, so the port owes the rule.

**The rule.** A harness-integration arm whose exit status is a **gate on a user
action** — `--hook` — fails **open** when the binary is absent: the front-end
writes the absent-binary diagnostic and the build remedy to stderr and exits `0`,
so the guard declines rather than wedging the session. `--statusline` fails open
the same way, writing nothing on stdout. `--usage-poll` keeps exit 2, its caller
being a refresh command or a session rather than a gate on a tool call.

**There is no tension with §Fail-closed contract to reconcile, and saying so
once is the point of this paragraph.** That contract attaches to a *gate* —
§The non-gate arm rules it onto "a thing that returns a verdict a battery reads"
— and a harness-integration arm returns a verdict to the harness, not to a
battery. A later reader meeting a non-gate arm that fails open should read a
class that was never governed by it, rather than filing a defect against a
contract that does not reach here.

**The ground is guard-kit's own, not an invention.** `guard-kit/lib/guard.sh`
already fails open on missing infrastructure — the `spec:` comment at
`guard-kit/lib/guard.sh:258` binds "the fail-open read rules 18 and 19 share, so
a missing jq or settings file emits nothing and every reader declines". A guard
that cannot run must decline, not brick. The alternative — fail closed, matching
the gate battery — is refused because its blast radius is every tool call in the
session and its trigger is an adopter's platform rather than an adopter's error.

### (4) `run-gates.sh` grows three operands

The front-end takes `--hook <member>`, `--statusline` and `--usage-poll`,
each dispatching through the existing `exec_arm`, plus the fail-open variant
delta 3 rules; its usage text gains the three lines. {mechanical}

No new shell file. `exec_arm` already resolves the bridged environment
(`gate_knob_env "$arm" "$@"`) and `exec env … "$bin" "$arm" "$@"`, and `exec`
preserves stdin, so the hook payload reaches the arm untouched and the front-end
writes nothing on stdout along that path. A dedicated `gate-sdk/bin/run-hook.sh`
was weighed and refused: it would add an owed shell member to the column this cut
exists to drain, and it would duplicate `exec_arm`.

The arm name is an **operand** here and the **whole flag** in the crate — unlike
`--emit <name>`, which the front-end composes into `--emit-<name>`. Composing
`--hook-<member>` would be one arm per hook, the refused shape, reached by the
front-end's grammar instead of by the crate's.

### (5) The declared-knob union moves off the `Arm` variant

`emit::knobs` (`native/src/emit/mod.rs:221-227`) answers `Arm::Emit` with the
member's own roster (through the sentinel path) and `Arm::Run` with the
**dispatch union** over the tree's gate registry. That is exact today only
because one member carries the `Arm::Run` variant. {design-bearing}

The three arms minted here are all `Arm::Run` — each prints on its own channels
and returns a status — and **none of them dispatches a gate**. Left as it stands,
each would declare the union of every registered gate's knobs, and
`gate_knob_env` would resolve the whole battery's configuration to run one hook.

§The non-gate arm already records the mechanism and this unit takes it: the union
becomes a **sentinel in the member's own declared roster** — the shape
`port_blockers::EVERY_REGISTERED_KNOB` already is — `--run` carries it, and
`knobs` stops keying on the variant. The two registry-scoped expansions
(`runner::registered_members`, `port_blockers::registered_members`) collapse to
one, which is the de-literalization the second member makes unavoidable.

**Verification, not assertion:** `--knobs --run` must print the same roster
before and after, argv for argv, including the `--gates-dir`-scoped form
`native/src/emit/mod.rs`'s existing test exercises.

`--knobs --hook <member>` answers **that member's** roster; `--knobs --hook` with
no member answers the union over the roster in delta 2. The per-member answer is
what makes the bridge resolve one guard's configuration rather than eight, and it
is reachable because `gate_knob_env "$arm" "$@"` already forwards the arm's argv
— the same property `--run` scopes its union by.

### (6) The crate's hook module and its member table

`native/src/hook/mod.rs` owns a `HOOKS` table keyed by member name, each row
carrying the member's function and its declared knob slice; one module per member
beside it. {design-bearing}

The table is the single roster the arm dispatches on, `--knobs --hook` reads and
the arm's refusal message prints — derived once, never transcribed. Each member's
declared knobs are exactly what its shell source reads:

- `agent-budget-guard` — `DELEGATION_KIT_VERDICT_BIN`
- `agent-dispatch-guard` — `DELEGATION_KIT_CONFIG_FILE`,
  `DELEGATION_KIT_READONLY_TYPES`
- `subagent-stop-liveness` — `DELEGATION_KIT_STOP_LOG`,
  `DELEGATION_KIT_LIVENESS_CMD`, `GATE_SDK_TMP_DIR`
- `escalation-guard` — none
- `wakeup-guard` — `GUARD_KIT_WAKEUP_LOG`
- `workflow-state-guard` — `GATE_SDK_WORKFLOW_DIR`

`GUARD_KIT_LIB` leaves the declared set for the four members that carry it: it
names the shell library the member sourced for its primitives, and a compiled
member sources nothing. This is an interface removal on a governed surface, so it
lands with its doc (delta 12) on §The non-gate arm's deleted-argument rule read
one axis over.

**`guard-kit/lib/guard.sh` is untouched and stays permanently shell**
(`guard-kit/lib/guard.sh:3` declares `# no-port:` on two grounds: it is the sole
resolver for the `GUARD_KIT_*` knobs, and it is the API a consumer's
`templates/bash-guard.sh` rules are written against). So the port necessarily
mints a **second producer of the hook envelope**, in Rust, beside that library's.
That is not the duplication criterion 6 refuses — the refused shape is a second
resolver of one *configuration*, and this is one *protocol* spoken on two
substrates — and it is exactly what the lead's 2026-08-31 ruling means by the
hook substrate staying **mixed by design**. Recorded rather than left to be
re-derived, because the next reader will meet two envelope writers and ask.

### (7) The `--statusline` and `--usage-poll` members

`native/src/hook/statusline.rs` renders the status line; `native/src/hook/poll.rs`
refreshes the usage snapshot. {design-bearing}

`--statusline` reads the harness's session payload on stdin
(`.model.display_name`, `.effort.level`, `.context_window.used_percentage`, and
the two `.rate_limits.*` pairs), reads `DELEGATION_KIT_USAGE_FILE`,
`DELEGATION_KIT_CRED_FILE` and `DELEGATION_KIT_ACCOUNT_CONFIG`, and writes one
ANSI-coloured line on stdout. **Not `DELEGATION_KIT_USAGE_HISTORY`**: an
earlier draft of this delta carried it on the strength of
delegation-kit/SPEC.md §The usage.txt contract's "the statusline calls it
(with `DELEGATION_KIT_USAGE_HISTORY` set)" sentence, and the align audit read
that against `delegation-kit/templates/statusline-usage.sh` rather than
trusting the prose — the shipped producer never calls `usage-verdict` at all,
so the knob has no reader there today. Declaring it on the arm would have been
exactly the causal-completeness defect this amendment's own DoD refuses: a
field with no named reader. Filed to `.workflow/gap-inbox.md` at the
2026-08-31 align audit rather than fixed here, because wiring the call is a
behavior change the port-only run does not carry. The gauge escapes are
self-contained by delegation-kit/SPEC.md §The statusline template's own rule
and stay so — no asset leaves the binary. The harness ignores the exit
status, so the member has none to speak.

`--usage-poll` reads `DELEGATION_KIT_USAGE_FILE`, `DELEGATION_KIT_CRED_FILE`,
`DELEGATION_KIT_ACCOUNT_CONFIG` and `DELEGATION_KIT_USAGE_ENDPOINT`, spawns
`curl` through `proc::run` exactly as the shell member does — **no HTTP client
enters the crate and no dependency is added** — and writes the snapshot. Exit 0
wrote a snapshot, exit 1 is fail-soft with the snapshot untouched.

**The property this delta puts on the record, and the ruling that admitted
it.** `--usage-poll` gives `checkwright-gates` a path that reads an OAuth
credential file and reaches an external endpoint. The capability is not new to
the *product* — it ships today as kit shell an adopter vendors — but it is new to
the *binary* an adopter installs, and a reader sizing what that binary is
entitled to do should meet the sentence here rather than derive it from a module
list. TRAJECTORY.md §The objectives rules nothing on the installed binary's
capability surface, which is why the question went to the operator rather than
being derived; the operator ruled the path admitted, 2026-08-31.

**And objective 3 reads *toward* this move rather than against it.** "Opacity is
a goal, not a side effect" — withholding an implementation's source favours
execution over analysis by the coding agents the mechanism exists to hold. A
credential-reading path moving out of a vendored, readable shell file and into
the binary is that objective working as stated. Recorded because the instinct on
meeting a credential read inside a shipped binary is to reach for a
capability-minimisation argument the objectives do not make.

### (8) The liveness log's field set is open, so attribution stays addable

`subagent-stop-liveness`'s log line is specified as an **open `key=value`
record**: a reader parses by key and never by position or arity, and the writer's
field set is one table in the member's module. {design-bearing}

This discharges the one constraint riding out of
`subagent-liveness-log-unattributed-refusal`, whose promotion the lead declined
this iteration (`ruled: … lead 2026-08-31 own-authority`): the arm's logging
contract **must not foreclose** adding per-session attribution — the agent id,
the agent type, the matched run key — later. Under an open record, adding those
three is a one-table edit that no existing reader has to change, and the entry's
own design fork (diagnostic for a reader vs. steer for the refused session) stays
open for the later unit to settle. The existing fields, `keys=` among them, are
carried unchanged; nothing here settles *what* is logged, only that the shape
does not lock.

### (9) The template/copy seam collapses into the config seam

Four members carry a `<gates-dir>/` copy — `agent-budget-guard`,
`agent-dispatch-guard`, `subagent-stop-liveness`, `workflow-state-guard` — and
both sides are **deleted** together, the copy rule §The harness-template port
disposition states once. {design-bearing}

After the port, a consumer's per-member surface is the kit's config file alone.
That is the correct end state and it costs one migration, because the corpus
carries **one live behavioral divergence and it is untagged**:
`delegation-kit/templates/subagent-stop-liveness.sh:7` defaults
`DELEGATION_KIT_LIVENESS_CMD` empty, while `scripts/subagent-stop-liveness.sh:7`
defaults it `scripts/producer-liveness-reader.sh`. Neither file carries
`# copy-divergence:` — the copy documents the fork in a plain `# spec:` comment —
so a port author scanning for the tag alone would delete the filled default and
leave a guard that logs a verdict it never probed.

**The default moves into `scripts/delegation-config.sh` in the same commit as the
delete**, never after. This is §The non-gate arm's own rule — a default the
deleted shell driver held inline moves into the owning kit's library in the same
cut — read one tier out: the value is a *consumer* path, so its home is the
consumer's config seam rather than `delegation-kit/lib/delegation.sh`, whose
shipped default stays empty because the path is this repo's vocabulary.

### (10) Every surface naming a deleted path moves in the same commit as the delete

Three **wiring** surfaces, and four more the align audit's own tree grep
found beyond authoring's count — the count is probed rather than assumed at
build time, and the probe is a literal-string grep for each of the twelve
basenames over the whole tree, not a re-read of this list.
{mechanical}

- `.claude/settings.json` — five hook `command` fields (lines 138, 158, 162, 171
  and the `statusLine.command` at 120) and **one** `permissions.allow[]` grant
  (line 61, naming `delegation-kit/templates/usage-poller.sh`). The grant is
  `check-settings-paths`' corpus and the five commands are read by no gate at
  all — the hole filed as `settings-hook-command-path-gate`, which this unit does
  not close and does not re-file. The grant moves under
  `native-gate-port-remaining-corpus`' settings-grant carve-out
  (`ruled: … 2026-08-29`): dead grant lines go in the same commit as the delete.
- `guard-kit/templates/settings-hooks.json` — the adopter-facing merge fragment,
  naming `scripts/wakeup-guard.sh` and `scripts/workflow-state-guard.sh`. Its
  `scripts/bash-guard.sh` block is untouched, which is where the mixed substrate
  becomes visible to an adopter.
- `scripts/delegation-config.sh:15` — `DELEGATION_KIT_REFRESH_CMD` names
  `bash delegation-kit/templates/usage-poller.sh`. This surface is named by
  neither the queue entry nor the class ruling; it is the poller's live caller in
  this repo and a stale value there silently stops the refresh.

delegation-kit ships **no** settings template — its wiring is prose in
`delegation-kit/README.md` §Install, which is therefore an update target where
guard-kit's and context-kit's JSON fragments are. The asymmetry is stated so the
build does not conclude from two JSON hits that it has found them all.
`installer/` names no member of the corpus.

**Four more, not wiring but every one load-bearing on its own reading, found
by the align audit's grep and not by this delta's own count.** None reopens
the member set or the arm kind; each is the same disposition as the three
above, read one level down from deployment config to the surfaces that
exercise or describe a deleted path directly.

- `delegation-kit/smoke/install.sh:49,71` — the delegation-kit consumer-smoke
  recipe **invokes** two deleted templates directly, `bash
  "$SMOKE_KIT_ROOT/templates/usage-poller.sh"` and `bash
  "$SMOKE_KIT_ROOT/templates/subagent-stop-liveness.sh"`, as the assertions
  exercising `--usage-poll`'s and `subagent-stop-liveness`'s behavior
  (gate-sdk/bin/run-consumer-smoke.sh, part of this repo's own validate
  battery). Deleting the templates without rewriting these two invocations to
  dispatch the arm — through `gate-sdk/bin/run-gates.sh`, the pattern the
  same file's line 84 already uses for `--emit graph` — silently drops smoke
  coverage for both members rather than porting it.
- `guard-kit/smoke/install.sh:9` — the guard-kit consumer-smoke recipe
  copies the deleted `templates/wakeup-guard.sh` into the scratch consumer's
  `scripts/`; the copied file is never itself exercised in this recipe, but
  an unguarded `cp` off a path the delete removes fails the recipe outright,
  before the `bash-guard.sh` assertion it exists to reach. Falls with this
  delta's own rewrite of `guard-kit/templates/settings-hooks.json` above: once
  that fragment's `wakeup-guard` command names the arm instead of a script,
  this copy line has nothing left to stage and is dropped with it.
- `delegation-kit/templates/delegation-config.sh:6` — a `shellcheck disable`
  comment reads "consumed by `templates/agent-dispatch-guard.sh` after
  sourcing"; that file is one of delta 9's four deletes. The comment is prose,
  not wiring, and no gate reads it, but a reader meeting it after the port
  would chase a path that no longer exists. Reworded to name the `--hook
  agent-dispatch-guard` arm as the consumer instead.
- `.claude/commands/lead.md:95-98` — the lead-model binding's optional-guard
  paragraph names `guard-kit/templates/wakeup-guard.sh` and
  `guard-kit/templates/escalation-guard.sh` as what a lead session wires;
  post-port, wiring means pointing the hook's `command` field at the binary,
  never copying a template, so the paragraph is reworded to name the arms.

### (11) The shell test estate retires with its subjects

Each ported member's cases become crate tests under the member's own module, and
every shell harness whose subject leaves is **verified** empty of other subjects
before it is deleted or narrowed. {design-bearing}

The estate, named so the build can verify rather than discover:
`delegation-kit/bin/run-budget-guard-tests.sh`,
`delegation-kit/bin/run-dispatch-guard-tests.sh`,
`delegation-kit/bin/run-usage-tests.sh`, `guard-kit/bin/run-guard-tests.sh`,
`delegation-kit/gate-tests/subagent-stop-liveness.test.sh`,
`lifecycle-kit/gate-tests/workflow-state-guard.test.sh`,
`scripts/gate-tests/subagent-stop-reader.test.sh`,
`delegation-kit/usage-tests/dispatch-guard-cases.tsv`,
`guard-kit/guard-tests/escalation-cases.tsv`.

`guard-kit/bin/run-guard-tests.sh` also drives `bash-guard.sh`, which stays
shell, so it **narrows** rather than retires. The two `.tsv` case tables are kit
test data rather than consumer config and stay on disk, read by the crate test
that replaces their shell driver — moving them into Rust literals would trade a
reviewable table for a recompile.

A harness deleted here leaves the owed column too, which is a **second-order
reduction this amendment claims nothing about**: the figure is the port oracle's
and the build reports it from `--emit port-blockers --tree` rather than
predicting it.

### (12) The prose and the generated projections follow

Every SPEC section describing a member's shell form is rewritten to describe its
arm, and every generated projection is regenerated from its own trigger. {mechanical}

The prose targets are enumerated under §Existing sections updated. The generated
set is `docs/` mirrors, `docs/enforcement.md`, `ROADMAP.md` where the entry's tags
move, and `scripts/git-hooks/pre-commit` — each with the trigger and regen command
`docs/site-architecture.md` §Generated projections already rosters. Release posts
under `docs/posts/` name members in prose and are **historical records that are
not edited**; the build verifies the docs gates stay green over them rather than
rewriting a dated claim.

**The audit stage is triggered by this amendment, and that is stated here so
the build session is not the one that learns it.** `check-stage-entry` assertion
C fires on a single amendment whose component set — its own directory union the
contract-surface tokens in its body that resolve to a roster directory — is two
or more. This file sits in `gate-sdk/` and its update targets name
`delegation-kit/SPEC.md`, `guard-kit/SPEC.md` and `lifecycle-kit/SPEC.md`, each a
roster directory. So the set is four, the trigger fires, and the `align` stamp is
demanded at **build**'s entry (lifecycle-kit/SPEC.md §check-stage-entry). Nothing
here waives it and nothing may: a waiver is written only on an explicit user
ruling and never self-issued by the entering session.

## Producers and consumers

**The arms (new interfaces).**

- **`--hook <member>`** — producer: `native/src/main.rs`'s bridged-arm dispatch,
  reached when `emit::lookup` resolves the flag; the enabling configuration is
  `.claude/settings.json`'s `command` fields, which this unit rewrites in the same
  commit (delta 10), so the producer is reachable in the only deployment that has
  it today. Consumer: the coding harness, which reads stdout as the hook-JSON
  envelope, stderr as the shown message and the exit status as allow/block.
- **`--statusline`** — producer: the same dispatch. Enabling configuration:
  `.claude/settings.json`'s `statusLine.command`. Consumer: the harness's status
  bar, reading stdout.
- **`--usage-poll`** — producer: the same dispatch. Enabling configuration:
  `scripts/delegation-config.sh`'s `DELEGATION_KIT_REFRESH_CMD` and the
  `permissions.allow[]` grant. Consumers: `delegation-kit/lib/delegation.sh`'s
  refresh path, and a session invoking it directly — a session caller precedented
  by `--emit-queue-index extent`.
- **The front-end operands** — producer: `gate-sdk/bin/run-gates.sh`'s argument
  grammar. Consumer: the three `command`/grant surfaces above, which name the
  operand verbatim.

**Every new field has a named reader.**

- `HOOKS`' **member-name key** — read by the arm's dispatch (at invocation), by
  `emit::knobs` through `--knobs --hook <member>` (at `gate_knob_env`, before the
  exec), and by the unknown-member refusal (which prints the roster). No fourth
  reader, and none is added.
- `HOOKS`' **per-member knob slice** — read by `--knobs --hook <member>` alone,
  at the front-end's bridge resolution. It is the only thing that keeps one
  guard's configuration from resolving the battery's.
- The **sentinel in `--run`'s roster** (delta 5) — read by `emit::knobs`'
  expansion at the same transition, replacing the variant test. Its red condition
  is not a new one: an under-declared knob is refused by the bridge at the child's
  own dispatch, unchanged.
- The **liveness log's key set** (delta 8) — read by close's drain and by any
  session diagnosing a refusal cluster. No field is added by this unit; the
  contract is that the *shape* admits one.

**Existing integration prose updated in this amendment** — enumerated below,
each against the delta that owns it.

**Readers whose corpus this narrows, by red condition rather than by subject**
(canon-kit/SPEC.md §The causal-completeness check, point 5 — twelve shell files
leave the tree):

- **`check-template-copy-parity`** — reds when a *paired* template and copy
  disagree on a declared surface. Deleting both sides of four pairs removes four
  pairs from a derivation that has **no floor and no count**: an unpaired template
  is silently skipped, not failed (gate-sdk/SPEC.md §check-template-copy-parity),
  and an empty corpus asserts nothing. Monotone; clearable by inspection. Its
  fixtures are a synthetic tree that happens to name `agent-budget-guard.sh` and
  are untouched by the real delete — verified at build, not assumed.
- **`check-settings-paths`** — reds when a literal `.sh` path in
  `permissions.allow[]` resolves to no file. **Not monotone under this narrowing**:
  the delete *adds* the violation, at `.claude/settings.json:61`. Delta 10 moves
  the grant in the same commit, which is the carve-out's whole point.
- **`check-shellcheck`**, **`check-comment-tier`**, **`check-path-dialect`** —
  each reds per offending line over a tracked-shell corpus. Monotone: removing
  files removes candidates.
- **`check-md-refs`** — reds on a markdown reference resolving to no file. Not
  monotone: the delete can *add* a dangling reference wherever a doc links a
  member's path. Delta 12 rewrites the prose targets; the build runs the gate
  rather than reasoning about which links were textual.
- **`--emit port-blockers --tree`** — its owed count is the port track's
  completion predicate and moves down; it is a measurement with no verdict, so it
  has no red condition to clear. The figure is the oracle's and is not restated
  here.
- **`check-crate-arms`** — reds when the crate's lint or test arm fails. The new
  modules and their tests widen its subject rather than narrow it.

## Existing sections updated

- gate-sdk/SPEC.md §The non-gate arm — the class gains the harness-integration
  sub-class, the three minted spellings, the argv-is-the-member-name rule, the
  absent-binary fail-open rule, and the sentinel replacing the variant test
  (deltas 1, 2, 3, 5).
- gate-sdk/SPEC.md §The harness-template port disposition — its "members the
  ground does not reach" paragraph names all twelve and sends their work to
  `harness-template-port-residue`; that sentence and its honest-limit paragraph
  are rewritten to record the disposition taken (deltas 1, 9).
- gate-sdk/SPEC.md §run-gates — the front-end's argument grammar and usage text
  gain three operands, and `exec_arm` gains the fail-open variant (deltas 3, 4).
- gate-sdk/SPEC.md §check-template-copy-parity — its parenthetical that "this repo
  wires two from the template path itself" names the two members that stop being
  wired that way (delta 9).
- TRAJECTORY.md §The closed rulings — the 2026-08-31 arm-kind paragraph states
  "the contract lands at gate-sdk/SPEC.md §The non-gate arm when
  `harness-template-port-residue` is specified; this paragraph then retires"
  (delta 1).
- delegation-kit/SPEC.md §The delegation model — the D1/D2/D3 guards' wiring and
  substrate (deltas 2, 6, 10).
- delegation-kit/SPEC.md §The turn-end liveness hook (template) — the member's
  substrate, its log-record shape, and the section title's "(template)" (deltas 2,
  6, 8).
- delegation-kit/SPEC.md §The statusline template — the member's substrate and its
  section title (delta 7).
- delegation-kit/SPEC.md §usage-verdict and §The usage.txt contract — the poller's
  substrate and the verdict binary staying shell behind its knob (deltas 2, 7).
- delegation-kit/SPEC.md §Layout and configuration — `GUARD_KIT_LIB` leaving the
  members' read set, `DELEGATION_KIT_LIVENESS_CMD`'s default moving to the
  consumer seam (deltas 6, 9).
- delegation-kit/SPEC.md §Testing — the harnesses that retire or narrow
  (delta 11).
- delegation-kit/README.md — the prose install steps that spell the wiring, this
  kit shipping no JSON fragment (delta 10).
- guard-kit/SPEC.md §wakeup-guard (template) and §escalation-guard (template) —
  each member's substrate, their unwired-here default unchanged (deltas 2, 10).
- guard-kit/SPEC.md §The guard framework (`lib/guard.sh`) — the library stays
  permanently shell and is now one of two producers of the hook envelope
  (delta 6).
- guard-kit/SPEC.md §Testing — `run-guard-tests.sh` narrowing to `bash-guard.sh`
  (delta 11).
- guard-kit/README.md — the optional-guard roster's wiring lines (delta 10).
- lifecycle-kit/SPEC.md §check-stage-entry — the paragraph beginning
  "`templates/workflow-state-guard.sh` closes the part of that window", including
  its `GUARD_KIT_LIB` indirection sentence and its inert-until-wired residual
  (deltas 2, 6, 10).
- lifecycle-kit/README.md — the workflow-state-guard install line (delta 10).
- README.md §This repo, governed — the per-kit fixture-runner roster, where a
  harness retires or narrows (delta 11).
- README.md's guard-kit row — its optional-wakeup-guard phrasing, checked against
  the member's new substrate and edited or cleared on the reading (delta 10).
- docs/site-architecture.md §Generated projections — the roster delta 12 regenerates
  against; it gains no member, and the build records that it was checked rather
  than leaving the silence to read as an omission (delta 12).
- TASK-QUEUE.md — `harness-template-port-residue` promotes with this amendment's
  ref, and `subagent-liveness-log-unattributed-refusal`'s carried constraint is
  discharged by delta 8 rather than by that entry moving (all deltas).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
