# SPEC amendment: demo-walkthrough

## What changes

The demo is a script, not prose — `demo/run-demo.sh` (new repo-root
directory, no owning kit): an annotated end-to-end walkthrough that builds
a scratch consumer on gate-sdk's existing consumer-smoke harness
(`gate-sdk/bin/run-consumer-smoke.sh` mechanics), then narrates the
adoption arc:

1. vendor the kits into the scratch consumer, write `gates.list`,
   generate the pre-commit hook;
2. make a clean commit — the battery passes;
3. introduce a craftable violation (the smoke `violation.sh` pattern —
   e.g. a restated comment or a broken `spec:` pointer) — the gate blocks
   with its finding + help line;
4. fix, re-run, green.

Output is a self-narrating transcript on stdout (section banners between
acts); exit 0 only when every act behaved — a pass asserts the whole arc,
including that the violation act *was blocked*. `DEMO_TMP_DIR` (default:
the gate-sdk smoke harness's scratch location) is the only knob.

Anti-drift by registration, not review: the demo registers as a validate
suite — a `demo demo pass` row in `.workflow/validate-baseline.txt` and an
`EVIDENCE_KIT_RUN_demo` command in the evidence-kit consumer config
(`EVIDENCE_KIT_SUITES` gains `demo`) — so every validate stage runs the
walkthrough end-to-end and a bit-rotted demo is a red validate, not a
stale docs page.

The docs demo page (`docs/kits/` sibling, slot owned by SPEC-docs-site.md)
cites `demo/run-demo.sh` and describes the acts; it embeds no transcript
(cite-never-restate — a pasted transcript is the drift vector this design
exists to remove).

## Producers and consumers

- Producer: `evidence-kit/bin/run-validate.sh` executes the suite each
  validate stage; any session may run `bash demo/run-demo.sh` directly.
- Consumer: the validate session (baseline diff + evidence-manifest line —
  existing evidence-kit mechanics, no format change); adopters reading the
  transcript or the docs page.
- Inputs read: the vendored kit directories (read-only; the scratch
  consumer is built in `DEMO_TMP_DIR`). Writes nothing in-tree.
- `DEMO_TMP_DIR` reader: `demo/run-demo.sh` at scratch-consumer setup.

## Existing sections updated

- CLAUDE.md Housekeeping gains the `demo/` line (runnable walkthrough,
  registered as a validate suite) at merge.
- evidence-kit consumer config (`scripts/`): `EVIDENCE_KIT_SUITES` +=
  `demo`, `EVIDENCE_KIT_RUN_demo` set; `.workflow/validate-baseline.txt`
  gains the row — consumer wiring, no evidence-kit SPEC change.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
