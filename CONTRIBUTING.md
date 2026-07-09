# Contributing to Checkwright

Checkwright is a monorepo of gate kits that a coding agent and its supervisor
run against their own delivery work. Its contribution surface is built for the
failure mode of that era — a repo drowning in low-triage-value issues and PRs.
The defense is mechanical: **the fixture is the unit of contribution.** A report
a gate can verify costs near-zero to triage; anything else routes to
[Discussions](https://github.com/checkwright/checkwright/discussions), where
volume is harmless.

## Report a gate defect as a failing fixture pair

Every gate ships a `good/`+`bad/` fixture pair (see
[gate-sdk/SPEC.md](gate-sdk/SPEC.md) §Fixture-pair discipline). A defect is a
missing case:

- **A gate misses a violation** → add the `bad/` case it wrongly passes.
- **A gate flags a valid tree** → add the `good/` case it wrongly fails.

Submit that case as a pull request. The CI backstop
([gate-sdk/SPEC.md](gate-sdk/SPEC.md) §templates/gates-workflow.yml) runs the
full battery and every fixture runner over it, so the pair *is* the
reproduction — no prose repro steps, no maintainer setup. File a gate-defect
issue only when you cannot craft the fixture, and it must still name the gate,
the exact gate output, and the expected-versus-actual verdict.

## Pull requests

- **Battery-green in CI.** Run it locally first: `bash gate-sdk/bin/run-gates.sh`
  for the full battery, then the fixture runners the
  [README](README.md) lists. A red PR is not reviewed until it is green.
- **Fix the tree, never weaken the gate.** A PR that relaxes a gate to pass
  instead of fixing what it caught is the defect, not the fix — this is
  check-gate-tamper's doctrine ([delegation-kit/SPEC.md](delegation-kit/SPEC.md)
  §Validate after every agent commit). A gate change lands with the fixture that
  proves it and the reasoning in the PR body.
- **DCO sign-off on every commit.** Sign each commit (`git commit -s` adds the
  `Signed-off-by:` line); it certifies the
  [Developer Certificate of Origin](https://developercertificate.org/). There is
  no CLA and no bot — the sign-off is checked in review.

## Larger changes

The kits are governed by their own conventions before taste. Read the queue and
spec conventions first — [queue-kit](queue-kit/README.md) and
[spec-kit](spec-kit/README.md) — and open a
[Discussion](https://github.com/checkwright/checkwright/discussions) before
building, so the design is agreed before a PR exists.

## Support

Community support is best-effort: there is no response SLA, and stale-thread
automation may close inactive issues and PRs. A clean fixture pair is the
surest path to a merge — it asks the least of a reviewer.

Paid support, consultancy, and training are available — write to
<hello@checkwright.dev>.

## License

Contributions are under [Apache-2.0](LICENSE), the repo's license. The license
is not a contribution lever; provenance is carried by the DCO sign-off above.
