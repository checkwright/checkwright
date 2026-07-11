Execute the template at lifecycle-kit/templates/skills/validate.md, applying the bindings below.

## Bindings

**exit-condition** — the full gate battery and every kit's fixture suite green,
whole-tree.

**suites** — run the battery and the fixture suites:

```bash
bash gate-sdk/bin/run-gates.sh
bash gate-sdk/bin/run-gate-tests.sh gate-sdk/gate-tests gate-sdk/checks
bash gate-sdk/bin/run-gate-tests.sh lifecycle-kit/gate-tests lifecycle-kit/checks
```

Also exercise the kits as a consumer would: `bash
gate-sdk/bin/run-consumer-smoke.sh`, gating on its success token
(gate-sdk/SPEC.md §Consumer smoke). Record the evidence with the codified
spine: `bash evidence-kit/bin/run-validate.sh` runs each configured suite,
diffs the baseline slice, and appends one `verdict=clean` line per suite to
`.workflow/validate-evidence.txt`; commit that file (evidence-kit/SPEC.md
§check-evidence-manifest). Gate on the positive success tokens, not the absence
of failure text; a non-zero exit is a real new failure — fix or file it, never
edit the baseline to pass. A legitimate baseline edit (a new held-constant red
with its blocking slug, a recovered row promoted to pass) is written per
evidence-kit/SPEC.md §Baseline manifest; a finding filed into the queue
follows queue-kit/SPEC.md §The tag algebra.
