# SPEC amendment: smoke-registration-accounting

Queue entry: **`queue-kit-starter-template-red`**.

Two coupled parts. queue-kit ships a starter template that fails the kit's own
gate, and the kit's smoke cannot see it because the smoke never registers that
gate. The second is why the first survived.

The framing that makes both one unit: **part 1 is a live violation of a
gate-sdk contract that already exists.** §Consumer smoke's *Starter-template
conformance* rule already says a kit that ships a starter template ships it
battery-clean, and already says the obligation is mechanical rather than ritual —
`smoke/install.sh` installs the template verbatim "so a template regression
against any kit reddens the harness instead of waiting for a hand-run validate
proof." That mechanism is defeated by an unregistered gate. Part 2 is what makes
the existing contract enforceable.

## What changes

### 1. The starter template is costed {mechanical}

`queue-kit/templates/TASK-QUEUE.md`'s lone deferred entry (`example-deferred`)
carries no `Cost while deferred` bold lead-in, so `check-queue-entry-budget`
assertion C reds on a verbatim copy under kit defaults. It gains the field,
which also makes the template teach the field — a starter queue that models a
costed filing is the better teaching artifact anyway.

With that line, queue-kit/SPEC.md §templates/'s claim that the template "ships
battery-clean when copied verbatim" becomes true as written; no prose changes.

**The reproduction trap, carried from the entry because it will otherwise cost a
build session.** The red reproduces only when the template path is passed
**explicitly**. `check-queue-entry-budget.sh` defaults its subject to
`${1:-$QUEUE_KIT_QUEUE_FILE}`, and this repo's `scripts/queue-config.sh` sets
neither `QUEUE_KIT_QUEUE_FILE` nor `GATE_SDK_QUEUE_FILE`, so on a bare
invocation `queue-kit/lib/queue.sh`'s own default resolves the subject to the
literal `TASK-QUEUE.md` — this repo's own, already-compliant queue file, not
the template. A build session that runs the gate with no positional argument
will conclude the defect is already fixed; passing
`queue-kit/templates/TASK-QUEUE.md` explicitly reproduces the red regardless of
cwd. Relatedly, `run-consumer-smoke.sh queue-kit` reports clean today — not a
contradiction but this entry's whole point, since the smoke never registers
the gate that reds.

### 2. The registration accounting {design-bearing}

**Contract, promoted and mechanized.** The predicate "a gate earns a
scratch-battery slot when it reads a surface the install writes" exists today as
site-kit-local commentary and binds nobody. It is promoted into §Consumer smoke
as the rule for every kit, and — the substance of this delta — it is
**evaluated** rather than transcribed.

The evaluation is the gate's own exit code, read against gate-sdk's **output
contract** (§Output contract) — the sole authority for what an exit code means:
**0** clean, **1** violation, **2** harness/usage error. Every shipped gate
signals on that contract and `check-gate-output` holds the family to it, so the
channel is already produced and already governed.

**One exit-2 reading is not sufficient, and the reason is in the contract
itself.** Exit 2 is usage/environment failure *generally* — a missing binary, a
malformed config, a non-repo cwd, an empty roster, **or** an absent subject
surface. Only the last is a justified omission; the rest are a broken gate or a
broken environment. A single probe in the scratch consumer cannot tell them
apart, and the exit-2 row is the one row granting a **permanent** exemption with
no written reason owed, now or ever — the one row nobody will ever review.
Reading it off an ambiguous signal turns the never-reviewed row into a
false-exemption channel: a gate exiting 2 because the scratch tree merely lacks a
dependency would be recorded as permanently, silently justified.

**So the exemption is corroborated by a second probe**, run in the invoking repo
— the tree the harness was launched from, where the kit's own surfaces exist. An
absent-surface gate exits 2 only in the scratch consumer. An
environmentally-broken one exits 2 in both, fails corroboration, and drops to the
declare-or-red rows. The corroboration is not belt-and-braces to be simplified
away later: it is the entire difference between "this surface is absent *here*"
and "this gate does not run *anywhere*", and without it the permanent-exemption
row cannot be trusted.

| probe (scratch consumer) | probe (invoking repo) | Verdict |
|---|---|---|
| exit 2 | exit 0 or 1 | **Justified omission**, self-declaring. The surface it reads is genuinely absent from the consumer. No written reason, now or ever. |
| exit 2 | exit 2 | **Not exempt.** The gate is broken or environment-dependent, not surface-absent. Declare or red. |
| exit 0 | — | Green here, and registering costs nothing — an unexplained omission. Red unless declared. |
| exit 1 | — | Green nowhere: the gate finds real violations in the scratch consumer and nothing runs it. **The hiding shape**, and this instance. Red unless declared. |

The second probe runs only for a gate that exited 2 in the scratch consumer; the
exit-0 and exit-1 rows are decided by the first probe alone and never reach it.

**The exemption is derived, not maintained.** That is the whole reason this
shape was chosen over a per-gate declared roster: the justification for the
large majority of omissions is computed from the tree every run and cannot go
stale, be forgotten, or be copied wrong. The hand-written reason is a
**residual valve for the exit-0/exit-1 minority**, not the mechanism. An
implementation that collects reasons for every unregistered gate and consults
the probe afterwards has inverted this delta and is wrong however green it runs.

**The declaration valve.** Where a kit author judges an exit-0 or exit-1
omission legitimate — a vacuous pass that is not real coverage is the honest
case site-kit's own reasoning describes — the kit's `smoke/install.sh` carries
`# smoke-unregistered: <gate-name> — <reason>` beside its registration block.
Kit-local, sitting where a reader looking at registration already is, and
readable off the vendored copy the harness has in hand.

**Where it runs, and why that is the runtime bound.** The accounting is one pass
in `bin/run-consumer-smoke.sh`, after the green-battery phase and before the
violation phase, over the **union** — every vendored kit's `checks/` basenames
against the scratch consumer's `scripts/gates.list`. The harness already vendors
all kits into one scratch tree and already has it built and green at that point,
so the accounting adds no install, no second tree, and no per-kit repetition.

Three properties bound the cost, and they are stated here so validate is not
surprised by its own KPI:

- **It never runs at pre-commit.** `run-consumer-smoke.sh` is a `bin/` tool,
  never a registered gate; it is the `consumer_smoke` validate suite. So the
  probe costs the pre-commit battery — the thing `kpi-gate-runtime` meters most
  closely — exactly nothing.
- **The probe set is self-limiting.** Every gate that probes green gets
  registered, which moves it out of the probe set and into the battery, where it
  was going to run anyway. The steady state is the fail-closed set plus the
  declared set, and it shrinks as the accounting is satisfied rather than growing.
- **The absolute cost is at most two gate invocations per unregistered gate** —
  one on a small scratch tree for every member of the probe set, plus the
  corroborating one on the invoking repo for the exit-2 subset only. Currently at
  most the 51 that are unregistered today, falling from there. Build measures the
  real wall-clock and reports it.

If build's measurement shows the added `consumer_smoke` wall-clock is material
anyway, the sanctioned mitigation is to report it, not to weaken the derivation
by sampling or by caching a verdict across runs — a cached exemption is a
maintained exemption wearing a derivation's clothes.

### 3. The measurement pass, and what build does with it {design-bearing}

The residual — how many unregistered gates land on exit 0 or exit 1 and so need
a human judgment — **is not measured**, and this amendment deliberately does not
guess it. Build measures the corroborated verdict distribution across all 51
**before writing a single declaration**.

**A large residual is a finding build reports, never scope it absorbs.** The
distribution is reported to the lead as a result in its own right, because it
answers a question the operator asked and the ruling turned on: if most
omissions need hand-written reasons, then this option and the full
declared-completeness option converge in practice, and that convergence is a fact
worth surfacing rather than a cost worth quietly paying. Build does not decide
on its own authority to write forty reasons because forty is what the
measurement returned.

The harness's clean line carries the three counts permanently — registered,
self-declared (exit 2), hand-declared — so the hand-declared number is visible
every validate run and its growth is legible without an audit. That number
staying small is what keeps the derivation honest, and printing it is the only
enforcement that survives the session that wrote it.

### 4. The registrations the accounting forces {mechanical}

Whatever the measurement returns, the kits' `smoke/install.sh` scripts gain the
registration lines for gates that probe green. queue-kit's is already known:
`check-queue-entry-budget` and `check-queue-sections` both read `TASK-QUEUE.md`,
which queue-kit's own install writes verbatim, and both are unregistered today.
`check-queue-slug-liveness` is the third omission and is expected to be the
self-declaring kind (its prose-surface glob defaults empty, so it has no subject
in a zero-config consumer) — expected, not assumed: the probes rule on it,
corroboration included.

Mechanical because the probe is the oracle — run it, register what it names,
stop when it is green. The judgment this delta does **not** contain is which
gates belong; that is exactly what delta 2 moved out of human hands.

**Ordering, which matters and is not free.** Delta 1 must land before delta 4's
queue-kit registrations can be green: registering `check-queue-entry-budget`
while the template is still uncosted turns the harness red, correctly. The two
parts of this entry are therefore sequenced, not parallel — and the fact that
the accounting *forces* delta 1 rather than merely permitting it is the evidence
that the accounting is aimed at the right defect.

### 5. Rejected alternatives, recorded so they are not re-litigated {design-bearing}

**Contract prose plus the queue-kit instance alone, with the accounting filed as
a costed gap.** Declined. It is the cheapest option by a wide margin and it
trades enforcement-first for a costed deferral on a defect class whose own queue
entry already states the objection: a hand-corrected roster re-arms immediately.
Registering two gates in one kit while fifty-one omissions stay silent leaves
the mechanism that hid this defect fully intact, and the next instance is found
by the next adopter rather than by the harness.

**Deriving each smoke's registration from the kit README's `gate-roster:`
block.** Declined, despite being the most elegant option on paper. The block
already holds full `checks/` parity under `check-readme-roster` and already
carries per-gate annotations naming the surface each gate needs, so a skip token
there would give one source for both the roster and its reasons. It is refused
on a boundary rather than on its merits: it changes what `smoke/install.sh`
*is*, from an executable install recipe into a derivation over a doc, and that
is adjacent to **`kit-owned-install-recipe`** — the open entry asking whether a
kit's install-time roster should live in a kit-owned `bin/install-<kit>.sh` at
all. Pre-empting the shape of install.sh from inside a different unit would
decide that entry's question sideways. It stays open, and this alternative stays
available to it.

**Registering every shipped gate unconditionally.** Declined on the ground
site-kit already argued for its own case: a gate whose subject no install writes
either passes vacuously or fail-closes, and neither outcome is coverage. Forcing
all 51 would trade a silent-omission defect for a scratch battery whose green is
less meaningful than the one it replaced.

## Producers and consumers

**The accounting phase (new state in an existing tool).**
Producer: `gate-sdk/bin/run-consumer-smoke.sh`, in the phase between the
green-battery assertion and the violation loop. Its enabling configuration is
not a knob — it runs on every invocation of the harness, and the harness is the
`consumer_smoke` suite this repo's `scripts/evidence-config.sh` already runs at
validate, so the producer is reachable in the real configuration rather than
only under test. It reads the vendored kits' `checks/` directories and the
scratch consumer's `scripts/gates.list`, both of which exist in `$SCRATCH` at
that point because `csmoke_vendor_and_install` has already run.
Consumer: the validate-stage ritual, which gates on the harness's success token,
and the operator reading the failure text. The harness verdict already has that
consumer; this delta adds findings to an existing channel rather than opening one.

**The three counts on the clean line (new fields).**
Producer: the accounting phase, once per run. Reader, named because a field
without one is removed: the operator at the validate transition, and — for the
hand-declared count specifically — the lead, as the number whose growth says the
derivation is decaying into a maintained roster. The counts are printed on the
existing `CONSUMER-SMOKE: clean (…)` line, which already carries an installed
count and a fired count and already has a reader.

**`# smoke-unregistered:` (new declaration surface).**
Producer: a kit author editing that kit's `smoke/install.sh`, and only for a
gate the probe placed on exit 0 or exit 1. Consumer: the accounting phase, which
reads the vendored copy of the script and matches the declared gate name against
the unregistered set. Both fields are read — the gate name to match, the reason
into the harness's report on any run that prints the accounting detail — so
neither is a write-only field. A declaration naming a gate that is registered,
or one that is not shipped by that kit, is itself a finding: a stale valve is
the failure mode a declaration surface has, and it is cheap to catch at the same
transition.

**The gate exit contract (existing interface, new reader).**
Producer: every shipped gate, unchanged. The authority is §Output contract's
three exit meanings — **not** `check-gate-fail-closed`, which is a static lint
over `awk`/`jq` command-substitution captures and asserts nothing whatever about
how a gate behaves when its subject surface is absent. No gate is modified and no
gate acquires an obligation it did not already carry. Consumer: the accounting
phase becomes a second reader of the exit code, beside `run-gates.sh` — twice for
an exit-2 gate, once for every other.

The exit code is already produced and already governed; what it is *not* is
already **disambiguated**, because exit 2 is the contract's general
usage/environment code rather than an absent-surface code. The delta supplies
that discrimination itself, from the corroborating probe. This preserves the
delta's premise rather than weakening it: the exemption stays **derived** from
the tree on every run, nothing becomes hand-maintained, and no gate, knob, or
fixture is added — the second probe is one extra invocation over a set that is
already self-limiting.

**Whole-component-set reader survey.** `smoke/install.sh` is read by
`run-consumer-smoke.sh`, `context-kit/smoke/agents-md.sh` and
`bin/upgrade-smoke.sh` (all three build the same baseline through
`csmoke_vendor_and_install`) and by `check-smoke-entry-guard`; the scratch
`scripts/gates.list` is read by `run-gates.sh` and the generated hook.
**The two sibling callers do not run the accounting** — they build the baseline
for a different assertion and adding a phase to their path would charge them for
a verdict they do not consume. Build re-runs this survey against the tree before
implementing, with no `2>/dev/null` on any path probe: a silenced stderr on a
mistyped path reads a live reader as absent, which is the same false-negative
shape this whole unit is about.

## Existing sections updated

- **gate-sdk/SPEC.md §Consumer smoke** — the `smoke/install.sh` bullet's "register
  its gates in `scripts/gates.list`" clause becomes the accounting contract: the
  promoted predicate, the two-probe exit-code table and why corroboration is what
  makes the permanent exemption sound, the declaration valve and its stale-
  declaration finding, and the statement that the exemption is derived (delta 2).
  The *Starter-template conformance* paragraph gains the sentence naming the
  accounting as the mechanism that makes its own "mechanical, not ritual" claim
  true — that paragraph is currently the contract this defect violated with
  nothing to catch it (deltas 1, 2).
- **gate-sdk/SPEC.md §Consumer smoke, Producers and consumers** — the accounting
  phase, the three counts, and the second reader of the gate exit contract
  (deltas 2, 3).
- **site-kit/SPEC.md** — the paragraph owning the "reads a surface the install
  writes" predicate cites gate-sdk's §Consumer smoke as the predicate's owner
  from here, keeping its local application and dropping the general statement.
  One owner per fact: the predicate now binds every kit, so a kit-local copy is
  the parallel source content tiering forbids (delta 2).
- **queue-kit/templates/TASK-QUEUE.md** — the cost field (delta 1).
- **queue-kit/smoke/install.sh**, and every other kit's, as the measurement
  directs (delta 4).
- **docs/site-architecture.md §Generated projections** — no new gate lands here,
  so no hook or graph regeneration is triggered by this amendment. Stated
  explicitly because the sibling amendments in this iteration *do* add gates, and
  a build session batching them should not regenerate on this one's account.

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
