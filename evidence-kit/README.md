# evidence-kit

A held-constant test baseline and a committed per-run evidence manifest for the
validate stage: a stage stamp proves a stage was *invoked*, evidence-kit proves
it produced its green result. The manifest is a versioned, hashable wire
contract (`# contract: evidence-manifest v1`) an external verifier can consume,
so the kit is adoptable with or without an iteration lifecycle.

The gates: `check-evidence-baseline` (baseline grammar, blocking-slug liveness,
scenario coverage) and `check-evidence-manifest` (manifest grammar and, where
lifecycle drives the tree, close-entry green block + validate-stamp coupling).
The `bin/` tools that drive it: `run-validate.sh` (the codified spine that runs
the suites and records evidence) and `diff-baseline.sh` (the situational runtime
diff). See [SPEC.md](SPEC.md) for the full contracts.

## Install

Vendor the kit beside [gate-sdk](../gate-sdk/) (required), then:

1. Register the gates — add to your `gates.list`:

   <!-- gate-roster:begin -->
   ```
   check-evidence-baseline
   check-evidence-manifest
   ```
   <!-- gate-roster:end -->

   Regenerate the hook + graph artifacts: `bash gate-sdk/bin/gen-pre-commit.sh --write`.

2. Seed the two surfaces — `.workflow/validate-baseline.txt`:

       # contract: evidence-kit/SPEC.md §Baseline manifest — held-constant validate baseline: <suite> <scenario> <status> [<slug>]

   and `.workflow/validate-evidence.txt`:

       # contract: evidence-manifest v1

   (override the paths with `EVIDENCE_KIT_BASELINE_FILE` /
   `EVIDENCE_KIT_MANIFEST_FILE`).

3. Configure the suites — copy `templates/evidence-config.sh` into your gates
   dir as `evidence-config.sh`, naming `EVIDENCE_KIT_SUITES`, an
   `EVIDENCE_KIT_RUN_<suite>` command per suite,
   and the `EVIDENCE_KIT_PARSER` adapter (`exit-code` for a whole-suite pass/fail,
   `libtest` for per-test result logs, or your own log-parsing command).

4. Record evidence at validate — run `bash evidence-kit/bin/run-validate.sh`; it
   runs each suite, diffs the baseline, and appends one evidence line per suite.

5. Optional lifecycle integration — set `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` to the
   evidence manifest so a new iteration starts from the contract header, and the
   manifest gate's close-entry and stamp-coupling assertions arm automatically.

## Test

```bash
bash gate-sdk/bin/run-gate-tests.sh evidence-kit/gate-tests evidence-kit/checks
```
