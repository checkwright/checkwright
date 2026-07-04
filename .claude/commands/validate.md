The `validate` stage of a Checkwright iteration. Exit condition: the full
gate battery and every kit's fixture suite green, whole-tree.

**First step — stamp evidence.** Append `<iteration> validate <session-id>
<date>` to `.workflow/WORKFLOW-STATE.txt`; flip the TASK-QUEUE.md `[stage:]`
line to `validate` in the same commit. (`check-stage-entry` requires the
active queue drained before this flip.)

## Session ritual

```bash
bash gate-sdk/bin/run-gates.sh
bash gate-sdk/bin/run-gate-tests.sh gate-sdk/gate-tests gate-sdk/checks
bash gate-sdk/bin/run-gate-tests.sh lifecycle-kit/gate-tests lifecycle-kit/checks
```

Also exercise each landed kit as a consumer would: a fresh scratch repo,
vendor the kits, follow the kit README's install steps, confirm the gates
fire on a crafted violation (the fixture suites prove the gates; this proves
the install docs). Gate on the positive success tokens (`clean`, `All N
gates passed`), not the absence of failure text. A red that traces to a
deferred queue entry stays red behind its slug; a new red is fixed or filed
before validate completes. Report failures with their output.
