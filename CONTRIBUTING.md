# Contributing to Checkwright

Checkwright is a monorepo of gate kits that a coding agent and its supervisor
run against their own delivery work. Its contribution surface is built for the
failure mode of that era — a repo drowning in low-triage-value issues and PRs.
The defense is mechanical: **the fixture is the unit of contribution.** A report
a gate can verify costs near-zero to triage; everything else routes through the
inlets below.

## Where to file what

The issue forms are the whole issue surface — blank issues are off, and each
form asks for what a maintainer needs to act without a round trip. Open one from
the [issue chooser](https://github.com/checkwright/checkwright/issues/new/choose):

- **Gate defect** — a gate misses a violation or flags a valid tree. The
  fixture pair goes as a pull request instead, on the terms the next section
  sets.
- **Install failure** — a vendoring install refuses or breaks on a machine
  `checkwright doctor` reports clean. Run the doctor first: a below-contract
  toolchain is the likeliest cause and carries its own fix.
- **Documentation problem** — a page is wrong, incomplete, or unfollowable. A
  correction you can write is a pull request; the form is for the case where the
  text that should be there is what you came to learn.
- **Adoption report** — what happened after you vendored it into a real
  repository, including the gates you kept and the ones you unregistered. A
  removal is the sharpest signal this project receives, and there is no other
  inlet shaped for it.

Anything that wants a conversation before it wants a disposition — usage
questions, design proposals, anything open-ended — goes to
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

A filed issue is swept at an iteration boundary and given exactly one
disposition — promoted into the work queue or closed with cause. It is never
left linked-but-unqueued: the queue is the only place work waits.

## Pull requests

- **Build the gate binary before you commit.** Gates in this tree dispatch to a
  compiled subcommand, so `bash gate-sdk/bin/build-native.sh`
  is its own step and **`cargo test` does not discharge it** — the test harness is
  a different artifact from the binary the battery runs, so a full green test run
  leaves `check-gate-binary-fresh` red on a stale build
  ([gate-sdk/SPEC.md](gate-sdk/SPEC.md) §check-gate-binary-fresh). A fresh clone
  cannot commit until it has built once.
- **Battery-green in CI.** Run it locally first: `bash gate-sdk/bin/run-gates.sh`
  for the full battery, then the fixture runners the
  [README](README.md) lists — one of which builds and tests the `native/` crate,
  so a local run wants the whole toolchain roster in
  [docs/install.md](docs/install.md) §Requirements, `cargo` included. A red PR is
  not reviewed until it is green.
- **Fix the tree, never weaken the gate.** A PR that relaxes a gate to pass
  instead of fixing what it caught is the defect, not the fix — this is
  check-gate-tamper's doctrine ([delegation-kit/SPEC.md](delegation-kit/SPEC.md)
  §Verify after every agent commit). A gate change lands with the fixture that
  proves it and the reasoning in the PR body.
- **DCO sign-off on every commit.** Sign each commit (`git commit -s` adds the
  `Signed-off-by:` line); it certifies the
  [Developer Certificate of Origin](https://developercertificate.org/). There is
  no CLA and no bot — the sign-off is checked in review.
- **Every PR gets one disposition at a boundary.** A swept PR is merged
  (battery-green and in-convention), closed with cause, or reviewed with
  findings — where the findings that warrant design or follow-on work become
  queued entries. A PR is never left reviewed-but-actionless.

## Larger changes

The kits are governed by their own conventions before taste. Read the queue and
spec conventions first — [queue-kit](queue-kit/README.md) and
[canon-kit](canon-kit/README.md) — and open a
[Discussion](https://github.com/checkwright/checkwright/discussions) before
building, so the design is agreed before a PR exists.

## Support

Community support is best-effort: there is no response SLA, and stale-thread
automation may close inactive issues and PRs. Triage runs at iteration
boundaries rather than continuously — an issue or PR waits at most one iteration
for its disposition, so there is no need to ping between boundaries. A clean
fixture pair is the surest path to a merge — it asks the least of a reviewer.

Paid support, consultancy, and training are available — write to
<hello@checkwright.dev>.

## License

Contributions are under [Apache-2.0](LICENSE), the repo's license. The license
is not a contribution lever; provenance is carried by the DCO sign-off above.
