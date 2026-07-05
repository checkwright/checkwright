# SPEC amendment: consumer-smoke harness

Mechanizes the validate-stage scratch-consumer proof. Today the proof is
prose ritual (re-performed by hand each validate, no committed evidence), and
no suite exercises the *platform defaults* on a vendored-kit tree: kit
fixtures use contrived case dirs, and a consumer repo's battery runs under
that consumer's config overrides. The DoD-mode defect
(`spec-kit-vendored-spec-dod-scope`) shipped through exactly that gap.

## What changes

**New entry point `bin/run-consumer-smoke.sh`** — a bin/ tool beside
`run-gates.sh`/`run-gate-tests.sh`, never a registered gate (it builds a repo
and runs the full battery repeatedly; pre-commit-unfit by the runtime
budget). Usage:

```
run-consumer-smoke.sh [--keep] [kit-root...]
```

Kit roots default to `gate_kit_roots` resolution. The harness:

1. Builds a scratch consumer in a fresh temp dir (`git init`, one seed
   commit), vendoring each kit root by copy.
2. Runs each kit's `smoke/install.sh` (gate-sdk first, then argument order)
   in the scratch root — the executable form of that kit's README install
   steps: registering gates in `gates.list`, copying starter templates
   verbatim into place (which absorbs the old "run the starter template as a
   live surface" ritual clause).
3. Runs `bin/run-gates.sh` under **zero consumer config** and asserts the
   positive green token (`All N gates passed`) — the defaults-on-a-vendored-
   tree assertion no other suite makes.
4. Per kit with a `smoke/violation.sh`: runs it, re-runs the battery,
   asserts a non-zero exit AND a FAIL line naming the expected gate, then
   restores the tree (`git checkout . && git clean -fd`) before the next
   kit; asserts green once more after the last restore.
5. Removes the temp dir on exit (`--keep` retains it and prints the path;
   the temp-dir write has this named reclaim path).

Exit codes follow the gate convention: 0 all assertions hold, 1 an assertion
failed, 2 usage/environment. Output follows the gate output contract; the
positive success token is
`CONSUMER-SMOKE: clean (<n> kits installed, <m> violations fired)`.

**New per-kit contract: the `smoke/` directory.** Every vendored kit ships
one:

- `smoke/install.sh` (required) — run with cwd = scratch-consumer root, env
  `SMOKE_KIT_ROOT` = the vendored copy of the kit being installed. Must be
  idempotent-free of assumptions about other kits except gate-sdk (installed
  first). Exit non-zero aborts the harness with exit 2 (a broken installer
  is an environment failure, not a gate finding).
- `smoke/violation.sh` (optional) — same cwd/env contract; mutates the
  scratch tree to introduce exactly one violation and prints the expected
  gate name as its first stdout line (the harness's assertion reads it — the
  named reader). A kit without one contributes install coverage only; the
  harness prints a `no violation script` notice per such kit so the gap is
  visible in the evidence, not silent.

A kit root lacking `smoke/` entirely is an error (exit 2): once this lands,
shipping smoke material joins fixtures/README/SPEC in the kit-landing
checklist.

## Producers and consumers

- `smoke/` content — produced by the kit author at kit-landing time (the
  four existing kits gain theirs in this task); consumed by the harness
  steps 2 and 4.
- Expected-gate name (violation.sh stdout line 1) — produced by
  `violation.sh`; read by the harness's red-phase assertion (step 4).
- `SMOKE_KIT_ROOT` env — produced by the harness per script invocation; read
  by install/violation scripts to copy templates from their own kit.
- Harness verdict — produced by `run-consumer-smoke.sh`; consumed by the
  validate-stage ritual (the skill invokes it and gates on the success
  token) and, later, the CI story EXTRACTION.md's worklist names (the
  harness is the natural CI entry point; wiring CI is not in this task).
- `--keep` temp path — read by the operator when debugging a red run.

## Existing sections updated

- gate-sdk/SPEC.md gains a `## Consumer smoke` section (this delta merges
  there) and its kit-landing convention list grows the `smoke/` item.
- `.claude/commands/validate.md`: the scratch-consumer prose paragraph is
  replaced by invoking the harness and gating on its success token.
- lifecycle-kit's validate skill template: no change (the ritual body is a
  consumer placeholder there).
- CLAUDE.md's kit-conventions bullet list: fixtures+README+SPEC becomes
  fixtures+README+SPEC+smoke at merge time.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as
      one coherent document a reader who never saw the amendment can use
      alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] All four kits ship `smoke/install.sh`; each kit with a crafted
      violation ships `smoke/violation.sh`; the harness run is green on this
      tree's vendored kits under zero consumer config.
