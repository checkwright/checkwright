# SPEC amendment: front-end-residency

Rules the residue the door sweep left behind, recorded at gate-sdk/SPEC.md
§run-gates: whether the fail-open
front-end set can retire, and — the half two other queue entries are blocked on
— **which spelling an adopter-facing door may carry**. Sited in `gate-sdk/`
because both answers land in that SPEC; its sibling
`guard-kit/SPEC-door-reach.md` takes the same discriminator from the gate's side.

## What changes

### (1) The residency is ruled permanent, on three grounds, and the open question closes

gate-sdk/SPEC.md §run-gates ends its harness-shim paragraph by deferring
"whether a harness could instead be wired to a *binary* that fail-opens on its
own absence". That question is answered here and the deferral is deleted.
**{design-bearing}**

**The premise that decides it:** a binary cannot fail open on its own absence,
because the absent program does not run. Only three things can produce a verdict
when the binary is missing — a script that runs in its place, the harness itself,
or the harness's own handling of a command that does not resolve. All three are
weighed below and none retires the stubs.

**Ground one — the locator, already on record and merely re-read here.** Both
stubs' `# no-port:` cause is that the front end *locates the binary it executes*,
over `GATE_SDK_NATIVE_BIN`'s pre-binary precedence. A harness hook is a literal
`command` string in a settings file; a literal resolves no precedence, so a
consumer who sets the knob has a settings value that names the wrong path. This
ground alone refuses every shape that replaces the stub with a direct wiring.

**Ground two — the diagnostic.** The candidate that survives ground one is
wiring the harness at the binary and letting the harness's own handling of an
unresolvable command be the fail-open. It is reachable: the harness's documented
hook contract treats exit 2 as a block and **every other non-zero status,
`127` included, as a non-blocking error after which the tool call proceeds**, and
a `statusLine` command's non-zero status only blanks the line. So the harness
already declines rather than wedging. It is refused anyway, because the same
contract makes that decline **silent** — an absent hook command disables a
policy hook with a `hook error` notice on the first run alone — where the stub's
decline is loud and carries the build remedy naming
`bash gate-sdk/bin/build-native.sh`. Trading a loud decline for a silently
disabled guard is the failure guard-kit's own fail-open discipline exists to
avoid (guard-kit/SPEC.md §The guard framework (`lib/guard.sh`): a guard that
cannot run must decline, not brick — and must say so). *Evidence class, stated
rather than overclaimed:* the harness contract above is a **read of the vendor's
published hook and statusline documentation**, not a measurement on this box; a
live measurement of the absent-command path was attempted this session and
refused by the sandbox. The ruling does not rest on it — grounds one and three
each refuse the candidate alone, and a harness that blocked instead would refuse
it harder.

**Ground three — the pre-build caller, which no harness change reaches.** The
second caller the harness-shim paragraph already names is a clone with no built
binary. That caller has no harness event, no installed artifact and no settings
file in play, so it is untouched by every candidate above and is what makes the
residency permanent rather than merely unfinished.

**Both named candidate shapes are refused with their grounds, so a later session
finds a judgment rather than an unexplored option.** *A wrapper the installer
places beside the artifact* is the same dual-implemented script surface
relocated: it shrinks the interpreter surface by nothing (a bash half and a
PowerShell half still), it adds a name the uninstall reversal must reverse
(installer/SPEC.md §uninstall), and it cannot reach ground three, there being no
install in a pre-build clone. *A harness feature* is not this project's surface
to build, and ground three makes it insufficient even if it landed.

### (2) The adopter-facing door rule names its two admitted caller classes

§The adopter constraints' interpreter bullet states the rule and no exception,
while §run-gates records two callers that keep a script door. The exception is
stated where the constraint is, as **caller classes** rather than as a file list,
because a file list is what goes stale the next time a surface is added.
**{design-bearing}**

The two classes:

- **A harness-configuration value carrying a fail-open arm** — a settings file's
  `command`, and the prose that publishes that value for an adopter to paste.
  This is the class `check-door-binding`'s arm-keyed exemption already
  implements, and the exemption is **unchanged**: measured this session over the
  tracked tree, fail-open doors sit in `guard-kit/README.md`,
  `delegation-kit/README.md` and `lifecycle-kit/README.md` install steps as well
  as in settings files and `guard-kit/templates/settings-hooks.json`, and those
  README lines *are* the publication of the recommended value, so narrowing the
  exemption to configuration files would red the publication itself. Probe:
  `git grep -nE 'run-gates\.(sh|ps1)[^`"]*(--hook|--statusline)'`.
- **A pre-build door** — a command a reader runs in a tree that has no built
  binary, where naming the binary would name a path that does not exist. Minted
  here, because §run-gates names this caller and no surface names the class, so
  nothing could be said to hold a door to it.

Everything outside those two classes names the binary through
`GATE_SDK_NATIVE_BIN`, which is the door sweep's rule restated as a predicate
over callers instead of as a completed sweep over files.

### (3) A pre-build door carries its precondition at the site

A pre-build door is admitted, not excused: the class exists because the binary is
absent, so the door owes the reader the step that makes it work. The obligation
lands at the site, on the **Not yet applied** rule's reasoning — a precondition
recorded anywhere but where the command is typed is a precondition the typing
reader never sees. **{design-bearing}**

**Measured, and it is why this delta exists rather than a re-point.**
`native/target/` is gitignored and no binary is tracked, so a fresh clone has
none. Run with the binary absent —
`GATE_SDK_NATIVE_BIN=native/target/release/absent-binary bash gate-sdk/bin/run-gates.sh --run-demo`
— the front end exits **2** with
`run-gates: --run-demo dispatches to the native binary, but ./native/target/release/absent-binary is absent or not executable — it could not run. Build it: bash gate-sdk/bin/build-native.sh`.
So this repo's two headline try-it doors — `README.md`:25 and
`docs/index.md`:27, both spelling `bash gate-sdk/bin/run-gates.sh --run-demo` —
present as "one command" a command that cannot run in the tree the reader just
cloned. The defect is therefore **not** that the line hands a visitor `bash`; it
is that the line is a pre-build door with its precondition unstated, and
re-pointing it at the binary would make it worse by naming a path that does not
exist.

**The satisfying form is one of two, and the choice is the executing unit's**
(the paired debt entry
`readme-front-door-is-adopter-facing-and-outside-every-sweep`, whose deliverable
this is): either the door states its precondition adjacently — the build step,
and that `bash` is the clone-path floor and not the install floor — or the
headline hands the reader the **install** path instead, which carries a
PowerShell half (installer/SPEC.md §The install boundary) and so does not put
bash in front of a Windows evaluator. This delta rules the obligation; it does
not choose between the two, because which one the front door should carry is an
editorial call on the landing copy rather than a contract.

**What this does not claim.** §The adopter constraints' non-technical-adopter
bullet binds *install* steps, and a clone-and-run evaluation path is not one, so
no landed constraint is violated today. What is missing is that a reader cannot
tell the two paths apart at the site — which is the whole content of this delta.

## Producers and consumers

**Delta 1 — a ruling, so its producer is the text and its consumer is the next
reader of it.** It introduces no state, event or interface. Its one obligation is
negative: the deferral sentence in §run-gates is deleted, so no later session
re-opens a closed question. Roster-holding readers of §run-gates: none — the
section is cited by pointer, and `check-md-refs` holds those citations, which the
merge leaves resolving because the section name does not change.

**Delta 2 — the minted name is the class `pre-build door`, and its readers are
named.** Producer: an author writing or reviewing a door on any governed
surface; the enabling configuration is the one already deployed, since the rule
is read at authoring and review time rather than executed.

- **Consumer, mechanical:** `check-door-binding` (guard-kit/SPEC.md
  §check-door-binding), whose arm-keyed exemption implements the *first* class
  and whose new assertion C — the sibling amendment
  `guard-kit/SPEC-door-reach.md` — is what reads a door outside the kit roots at
  all. The second class is **deliberately not mechanized**: a gate cannot decide
  whether a reader's tree has a built binary, so the class is held by review and
  by delta 3's site obligation. Stated as a non-target so a later reader does not
  file the silence as a missing arm.
- **Consumer, human:** the author of a door on `README.md`, `docs/`, a kit
  README or a settings template.
- **Roster-holding readers of the surface the name lands on.** §The adopter
  constraints is a bulleted roster of constraints, and the interpreter bullet is
  the row the exception attaches to; the roster's own reader is prose. Probed for
  a mechanical roster over that section —
  `git grep -n "The adopter constraints" -- native/src scripts` — and what it
  returns is a single `§` citation in a comment in `native/src/runner.rs`, not a
  roster, so no gate roster needs a row; the section name does not change, so
  that citation keeps resolving. The *knob* `GATE_SDK_NATIVE_BIN`, which
  the rule names as the sanctioned spelling, already carries its
  §Layout and configuration row and its `check-knob-citation` obligation; naming
  it in prose adds no row.

**Delta 3 — the obligation, its producer and its red condition.** Producer: the
authoring session that writes a pre-build door. Consumer: the reader who types
it. **No corpus is narrowed by any delta here**, so causal-completeness point 5
is vacuous for this amendment — the deltas add a class and an obligation and
prune no file, glob or declaration. Point 6 binds delta 3, whose corpus *is*
enumerable at authoring time, so each member's satisfying value is named: the
corpus is exactly the two pre-build doors measured in delta 3, `README.md`:25
and `docs/index.md`:27, and each member's satisfying value is one of delta 3's
two stated forms. No third member exists — probe:
`git grep -nE "(bash|sh|-File) +[^ ]*run-gates\.(sh|ps1)" -- README.md docs/index.md`
returns those two lines and, in `README.md`, the eighteen lines of the
`<!-- battery-roster:begin -->` block, which are the contributor battery register
held in name-set parity with `EVIDENCE_KIT_SUITES` by `check-battery-roster`
(evidence-kit/SPEC.md §check-battery-roster) and are not pre-build doors: a
contributor running the validate battery has built the binary by construction.

## Existing sections updated

- **gate-sdk/SPEC.md §run-gates** — the harness-shim paragraph's closing
  sentence, which defers the harness-wired-binary question, is **replaced** by
  the ruling and its three grounds; the paragraph's surviving text (the caller
  set narrowing to two, and that narrowing it to zero is unreachable) is kept and
  re-phrased around the ruling rather than restated beneath it. (delta 1)
- **gate-sdk/SPEC.md §The adopter constraints** — the script-interpreter bullet
  gains its two admitted caller classes, re-phrased into the bullet rather than
  appended after it; the bullet's existing pointer to §The port-candidate
  criteria is kept. (deltas 2 and 3)
- **gate-sdk/SPEC.md §Layout and configuration** — no row is added or changed.
  Listed because a reader checking whether a delta naming `GATE_SDK_NATIVE_BIN`
  owes a knob row should find the answer rather than re-derive it: the knob
  exists, the rule only cites it. (delta 2)
- **`TASK-QUEUE.md`, the entry
  `readme-front-door-is-adopter-facing-and-outside-every-sweep`** — its body's
  block prose records that it is blocked on this unit's ruling; the ruling having
  landed, that prose is re-phrased to carry the ruled remedy (delta 3's two
  satisfying forms) instead of the block. Its scope also grows by one measured
  site, `docs/index.md`:27, which the entry does not name. **Not yet applied —
  and the growth is a queue/envelope question, escalated to the lead rather than
  decided here.** (delta 3)

### Replacement text

**gate-sdk/SPEC.md §run-gates — the closing sentence of the harness-shim
paragraph. Not yet applied.** Replace

> Whether a harness could instead be wired to a *binary* that fail-opens on its
> own absence is a separate design question this does not open.

with

> **A harness cannot be wired to a binary that fail-opens on its own absence,
> and the residency is permanent.** An absent program does not run, so the
> fail-open can only come from a script standing in for it, from the harness, or
> from the harness's handling of a command that does not resolve. The first is
> what the stubs are. The third is reachable — the harness's documented contract
> proceeds on any non-zero status but 2 — and is refused, because it converts a
> loud decline carrying the build remedy into a policy hook silently disabled
> after one notice. Both are moot against the stub's own `# no-port:` cause: the
> front end *locates* the binary, and a settings file's literal `command`
> resolves no precedence, so a consumer who sets `GATE_SDK_NATIVE_BIN` would
> have a wiring that names the wrong path. And none of the three reaches the
> second caller — a clone with no built binary has no harness event at all.

**gate-sdk/SPEC.md §The adopter constraints — the script-interpreter bullet.
Not yet applied.** Re-phrase the bullet to carry its exception:

> - **The script-interpreter surface shrinks to the unavoidable, and the
>   unavoidable is dual-implemented** — a POSIX shell for Linux and macOS,
>   PowerShell for Windows — where a script stands between an adopter and the
>   install or the battery (installer/SPEC.md §The install boundary's two
>   bootstrap halves; §The port-candidate criteria for what may stay shell and
>   on what ground). A script that guards a shell the harness already runs is
>   written in that shell on every host, because a session with a call to guard
>   already has the shell the guard needs (guard-kit/SPEC.md §The hook on native
>   Windows). **Two caller classes keep a script door permanently, and they are
>   classes rather than a file list because a list goes stale at the next
>   surface.** A **harness-configuration value carrying a fail-open arm** keeps
>   one by construction — its status is read exactly when the binary that would
>   answer is absent (§run-gates) — and so does the prose publishing that value
>   for an adopter to paste. A **pre-build door** keeps one because naming the
>   binary would name a path that does not exist yet; such a door **carries its
>   precondition at the site**, since a reader who cannot see that a build comes
>   first reads a clone-path floor as the install floor. Every other door names
>   the binary through `GATE_SDK_NATIVE_BIN`.

## Retired spellings

- None — no delta of this amendment retires a spelling. Delta 1 deletes one
  sentence and delta 2 re-phrases one bullet; both replace prose with prose over
  names that already exist (`GATE_SDK_NATIVE_BIN`, `--hook`, `--statusline`, the
  two stub basenames), and the one name the amendment *mints* — the class
  `pre-build door` — displaces no earlier spelling, there having been none for
  it.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation. Points 5 and 6 are discharged in §Producers and consumers
      above, 5 as vacuous and 6 by enumeration.
- [ ] **Instruction surfaces: instruction only** — no template, agent definition
      or shim is touched by any delta, so the rule is vacuous here; recorded
      rather than dropped so the check reads as run.
- [ ] **Merged with no information lost** — each replacement re-phrases the text
      it refines; §run-gates keeps its caller-set narrowing and §The adopter
      constraints keeps its two pointers.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), and the sibling
      `guard-kit/SPEC-door-reach.md` is repointed at the canonical sections
      these deltas merge into, per canon-kit/SPEC.md §check-amendment-queue arm
      (e).
- [ ] **Removals propagated** — declared negative above, and
      `check-amendment-retired-spelling` re-runs that declaration.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed to
      the gap inbox; the one already known is the queue/envelope growth in
      §Existing sections updated's last bullet, which is the lead's to rule.
