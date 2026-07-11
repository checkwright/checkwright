The `validate` stage of a Checkwright iteration. Exit condition: the full
gate battery and every kit's fixture suite green, whole-tree.

**First step — stamp evidence.** Run
`bash lifecycle-kit/bin/enter-stage.sh validate`: it appends `<iteration>
validate <session-id> <date>` to `.workflow/WORKFLOW-STATE.txt` and flips the
TASK-QUEUE.md `[stage:]` line to `validate`, committed together. It reads
`<session-id>` from `bin/session-id.sh` itself (never hand-picked), uses
`date +%F`, and refuses (writing nothing) if `check-stage-entry` is red —
which for `validate` requires the active queue drained before this flip.

## Session ritual

```bash
bash gate-sdk/bin/run-gates.sh
bash gate-sdk/bin/run-gate-tests.sh gate-sdk/gate-tests gate-sdk/checks
bash gate-sdk/bin/run-gate-tests.sh lifecycle-kit/gate-tests lifecycle-kit/checks
```

Also exercise the kits as a consumer would, mechanized: run
`bash gate-sdk/bin/run-consumer-smoke.sh` and gate on its success token
(`CONSUMER-SMOKE: clean (<n> kits installed, <m> violations fired)`). It
builds a scratch consumer, installs each kit under **zero config**, asserts
the full battery is green on the vendored tree, and fires each kit's crafted
violation to confirm the right gate reddens (gate-sdk/SPEC.md §Consumer
smoke) — the defaults-on-a-vendored-tree proof the fixture suites cannot make,
and the executable form of the old copy-the-install-docs ritual. Gate on the
positive success tokens (`clean`, `All N gates passed`), not the absence of
failure text. A red that traces to a deferred queue entry stays red behind its
slug; a new red is fixed or filed before validate completes. Report failures
with their output.

Record the validate evidence with the codified spine:
`bash evidence-kit/bin/run-validate.sh` runs each configured suite, diffs the
baseline slice, and appends one `verdict=clean` line per suite to
`.workflow/validate-evidence.txt`; commit that file — the close-entry manifest
requires it (evidence-kit/SPEC.md §check-evidence-manifest). A non-zero exit is
a real new failure: fix or file it, never edit the baseline to pass.

When filing a finding, place it by kind: nameable deliverable + done-state ⇒
queue task (Deferred `[needs-spec]`; scope triages feature-vs-debt by the
new-names litmus, canon-kit/SPEC.md §The amendment lifecycle); an observation
about how the work should be done ⇒ Lessons Learned, dispositioned at close.
Undone work parked as a lesson evaporates; a process insight parked as a
task rots.
