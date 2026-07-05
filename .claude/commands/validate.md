The `validate` stage of a Checkwright iteration. Exit condition: the full
gate battery and every kit's fixture suite green, whole-tree.

**First step — stamp evidence.** Append `<iteration> validate <session-id>
<date>` to `.workflow/WORKFLOW-STATE.txt`; flip the TASK-QUEUE.md `[stage:]`
line to `validate` in the same commit. (`check-stage-entry` requires the
active queue drained before this flip.) Take `<session-id>` from
`bash lifecycle-kit/bin/session-id.sh` (it reads the id from the newest
transcript — never hand-pick it); `<date>` is `date +%F`.

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

When filing a finding, place it by kind: nameable deliverable + done-state ⇒
queue task (Deferred `[needs-spec]`; scope triages feature-vs-debt by the
new-names litmus, spec-kit/SPEC.md §The amendment lifecycle); an observation
about how the work should be done ⇒ Lessons Learned, dispositioned at close.
Undone work parked as a lesson evaporates; a process insight parked as a
task rots.
