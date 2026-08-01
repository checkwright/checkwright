# SPEC amendment: battery-roster

Queue entry: **`battery-roster-hand-copy`**.

The repo's runner doc hand-copies a subset of the suite roster
`EVIDENCE_KIT_SUITES` already owns, and nothing reds the divergence. This
amendment gives the roster one owner and one gate, on the same
parity-over-a-human-read-register shape gate-sdk/SPEC.md §check-readme-roster
already chose for the kit-README gate rosters.

## What changes

**1. `check-battery-roster` — a new evidence-kit gate. {design-bearing}**

Invariant: the configured runner doc's battery-roster block holds name-set
parity with `EVIDENCE_KIT_SUITES`, both directions.

Marker vocabulary follows gate-sdk/SPEC.md §check-readme-roster verbatim in
shape: the doc wraps its register in `<!-- battery-roster:begin -->` /
`<!-- battery-roster:end -->` markers, which may carry leading indentation
(the roster scan trims surrounding whitespace before matching). Inside the
markers a **roster line** is a line whose content begins with `bash `; a
trailing `#` annotation clause is prose the gate never reads. Outside the
markers nothing is scanned, so the same command appearing elsewhere in the doc
for a different rhetorical job neither satisfies nor violates the gate.

A suite's **documented invocation** is `EVIDENCE_KIT_RUN_<suite>` normalized by
stripping a leading `env` and its `VAR=value` assignments — the `gates` suite
runs under `GATE_SDK_VERBOSE=1` to emit the per-gate tails its parser reads,
which is a validate-harness concern and not something a contributor types. What
remains is compared as an exact string against the roster line's command
(annotation clause excluded).

Two assertions over the suite set versus the roster set:

- **(A) Every suite is documented** — a member of `EVIDENCE_KIT_SUITES` whose
  normalized invocation matches no roster line is red. This is the assertion
  the seven-suite omission fails today.
- **(B) Every roster line resolves to a suite** — a roster line whose command
  matches no suite's normalized invocation is red, so a retired suite cannot
  leave a stale line telling a contributor to run a command validate no longer
  runs.

Each finding names the suite (A) or the command (B), the doc, and the line.

Fail-closed, following gate-sdk/SPEC.md §check-kit-registration exactly: a
configured doc that does not exist, a doc carrying no marker block, an empty
suite roster, or a non-repo cwd is a misconfiguration (exit 2), never a false
clean. **There is no empty-knob valve** — a consumer keeping no runner doc opts
out by not registering the gate in its `gates.list`, the opt-out shape that
kit's registry gate already establishes.

**2. `EVIDENCE_KIT_RUNNER_DOC` — a new knob. {design-bearing}**

Default `README.md`, resolved relative to the git toplevel; an explicit
positional argument (`check-battery-roster.sh [runner-doc]`) overrides, the
positional-override shape gate-sdk's sibling meta-gates use so a hermetic
fixture tree can be pointed at. The name mirrors `GATE_SDK_RUNNER_DOC`
deliberately: the two knobs name the same physical doc in this repo for two
different assertions, and a reader who has met one should not have to learn a
second vocabulary for the other.

**3. This repo's README battery block — completed and marked. {mechanical}**

`README.md` §This repo, governed gains the marker pair and the eight missing
lines: `delegation-kit/bin/run-usage-tests.sh`,
`delegation-kit/bin/run-budget-guard-tests.sh`,
`delegation-kit/bin/run-trend-tests.sh`, `context-kit/bin/run-index-tests.sh`,
`gate-sdk/bin/run-consumer-smoke.sh`, `gate-sdk/bin/upgrade-smoke.sh`,
`context-kit/smoke/agents-md.sh`, and `demo/run-demo.sh`. The delta is
mechanical because assertion (A) is the oracle: run the gate, add what it
names, stop when it is green — no judgment about *which* suites belong, since
`EVIDENCE_KIT_SUITES` already answers that.

The `demo` line is in the set. It reads as a duplicate of the Quick-start
invocation near the top of the README and is not one: that occurrence is the
adoption showcase, this one is the suite's roster row, and the marker block is
what distinguishes them — which is the reason assertion (A) scans the block and
not the whole doc.

**4. The block's framing prose — corrected. {design-bearing}**

The block is introduced today as what to run "before committing". Completing it
under that framing would make the README instruct a contributor to run the
whole validate battery — the demo, the consumer smoke, the upgrade smoke, the
installer smoke — before every commit, which is neither what this repo does nor
what CLAUDE.md §This repo is governed by its own kits says (the full gate
battery plus the touched kit's fixture suite).

So the block is reframed as what it actually is: **the register of the repo's
runnable verification suites — the set validate runs in full**, with the
per-commit selection rule stated above it and owned by CLAUDE.md, cited rather
than restated. The completeness the gate enforces is completeness of the
*register*, not of any one session's obligation.

**5. The overlap with `check-kit-registration` assertion B — stated, not
resolved. {design-bearing}**

gate-sdk/SPEC.md §check-kit-registration assertion B already requires every kit
root with tracked `gate-tests/` files to have a line in the runner doc naming
`<kit>/gate-tests`. Because `gate_fixture_suites` derives exactly those roots
into `EVIDENCE_KIT_SUITES`, assertion (A) here is a **superset** of that arm for
a consumer running both kits.

The overlap is kept rather than removed, and the reason is a dependency
direction: gate-sdk may not read evidence-kit's configuration. B must keep
firing for a consumer that vendors gate-sdk and no evidence-kit, so retiring it
would trade a redundancy for a coverage hole in the more common adoption shape.
Both SPEC sections say so, each naming the other, so the next reader who
notices the redundancy finds the reason instead of re-deriving it. Two gates
reporting one omission is a duplicate finding, not a contradiction — they name
different sets (a kit root, a suite) in their output.

## Producers and consumers

**`check-battery-roster` (new gate).**
Producer: the generated pre-commit hook and `gate-sdk/bin/run-gates.sh`, via
registration by name in `scripts/gates.list`. Its `# graph:` manifest couples
it to the runner doc, to `scripts/evidence-config.sh`, and to `gate-sdk/lib/gate.sh`
(whose `gate_fixture_suites` the config's loop calls), so an edit to any of the
three re-fires it. `check-graph` is the reader of that manifest, and the
generated hook is the artifact it stales — the regen trigger and command are
rostered in docs/site-architecture.md §Generated projections like every other
generated projection, and a new gate's fan-out is that section's own roster.
Consumer: the committing operator, through gate-sdk's output contract (name,
file, line, finding), read once at the pre-commit transition.

**`EVIDENCE_KIT_RUNNER_DOC` (new knob).**
Producer: `scripts/evidence-config.sh` may set it; unset, the gate's own
default `README.md` applies, which is this repo's actual value — so the repo
sets nothing and the default is the emitted configuration rather than a
test-only path. Consumer: `check-battery-roster.sh` at startup, its one reader.
It is a gate-local knob, not a `lib/evidence.sh` knob: nothing in the validate
run path reads it, so the loader does not need to.

**`EVIDENCE_KIT_SUITES` / `EVIDENCE_KIT_RUN_<suite>` (existing, new reader).**
Producer: unchanged — `scripts/evidence-config.sh`, part derived through
`gate_fixture_suites` and part hand-listed. Consumer: `bin/run-validate.sh`
today; `check-battery-roster.sh` becomes a **second** reader, and reads them by
sourcing the config through evidence-kit's existing loader rather than by
parsing the file, so a suite added by the derivation loop is visible to the gate
with no second parse to keep in step.

**The marker block (new surface in `README.md`).**
Producer: a maintainer editing the README. Consumer: `check-battery-roster.sh`,
which reads the begin marker, the roster lines, and the end marker. No field is
introduced with no reader: the annotation clause after `#` is explicitly
declared unread prose, and is kept only because the existing block's lines
carry it and a gate that forced their deletion would lose the human-read
labelling the register exists for.

**Whole-component-set reader survey.** The suite roster's readers across the
tree are `evidence-kit/bin/run-validate.sh`, `evidence-kit/bin/diff-baseline.sh`,
and `evidence-kit/lib/evidence.sh`'s loader; the runner doc's readers are
`gate-sdk/checks/check-kit-registration.sh` (assertion B) and, after this
amendment, `evidence-kit/checks/check-battery-roster.sh`. Build re-runs that
survey against the tree before writing the gate rather than trusting this list,
and does it without silencing stderr on any path probe — a `2>/dev/null` on a
mistyped path reads a live reader as absent.

## Existing sections updated

- **evidence-kit/SPEC.md §Layout and configuration** — `EVIDENCE_KIT_RUNNER_DOC`
  joins the knob roster with its default (delta 2). The section's knob list is
  the citation target `check-knob-citation` resolves against, so the knob does
  not exist until it is listed there.
- **evidence-kit/SPEC.md §Per-component contracts** — a new
  `### check-battery-roster` section holding the invariant, the marker
  vocabulary, both assertions, the normalization rule, and the fail-closed
  posture (delta 1), plus the overlap paragraph naming
  `check-kit-registration` assertion B (delta 5).
- **evidence-kit/SPEC.md §Producers and consumers** — the suite roster gains its
  second reader (delta 1).
- **gate-sdk/SPEC.md §check-kit-registration** — assertion B's paragraph names
  the evidence-kit gate that supersets it and states why it is kept: gate-sdk
  may not read evidence-kit config, so B is the arm that survives a
  gate-sdk-only adoption (delta 5). This is the reciprocal half of the overlap
  ruling; without it the redundancy reads as an oversight from the gate-sdk
  side.
- **README.md §This repo, governed** — the marker pair, the eight lines, and the
  reframed introduction (deltas 3, 4).
- **docs/site-architecture.md §Generated projections** — the new gate's fan-out
  (the generated pre-commit hook, the graph artifact, the enforcement map, the
  footprint rollup, the docs mirror) follows that section's standing roster for
  a new gate; it is named here so build treats it as claimed work rather than an
  orphan a batch adopts on its own authority.
- **evidence-kit/README.md** — the gate joins the kit's register-the-gates block
  between its `gate-roster:` markers, which `check-readme-roster` asserts
  against the kit's shipped `checks/` basenames; a gate landed without that line
  is red.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls evidence-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
