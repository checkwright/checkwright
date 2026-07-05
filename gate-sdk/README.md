# gate-sdk

A self-testing lint framework for the surfaces conventional linters ignore:
markdown specs, glossaries, task queues, config projections, diagrams — any
text whose drift is mechanically decidable. Built for repos where coding
agents do the writing: a stateless agent session cannot be trusted to
*remember* conventions, so the conventions are enforced by machine-run gates
that block the commit instead.

A **gate** is a small shell script checking one invariant across one or more
governed surfaces. The kit ships the machinery that keeps a gate family
honest:

- `lib/gate.sh` — the one sourced helper: the `fail_closed` wrapper (a crashed
  parser must never read as "clean"), the fixture-tree prune adapters, and the
  registry/resolution helpers.
- `bin/run-gates.sh` — the aggregate battery: every gate in your `gates.list`,
  one shot, per-gate timings.
- `bin/run-gate-tests.sh` — the golden-fixture runner: every gate proves it
  accepts a `good/` case and rejects a `bad/` case with the right error text.
- `bin/run-consumer-smoke.sh` — the end-to-end check no fixture makes: builds a
  fresh scratch consumer, runs each vendored kit's `smoke/` installer, and
  asserts the battery is green under zero config (then red on each kit's crafted
  violation). Each kit ships a `smoke/` directory to join the party.
- `bin/gen-pre-commit.sh` + `bin/install-hooks.sh` — a pre-commit hook
  *generated* from per-gate `# graph:` coupling manifests; adding a gate to
  the hook is manifest-only, so hook membership cannot drift.
- `checks/` — seven meta-gates that hold the family to its own standard:
  ShellCheck self-lint, the output contract, the fail-closed contract, fixture
  coverage, SPEC↔code assertion coupling, exemption-list hygiene, and manifest
  / hook / graph-artifact freshness (`check-graph`).
- `templates/check-skeleton.sh` — the copy-paste skeleton a new gate starts
  from.

The design contracts, the manifest grammar, and each component's full contract
live in [SPEC.md](SPEC.md).

## Quick start

Vendor the kit into your repo at `gate-sdk/`, then:

```bash
mkdir -p scripts                     # your gates dir (GATE_SDK_GATES_DIR to relocate)
cat > scripts/gates.list <<'EOF'
# kit meta-gates (resolve from gate-sdk/checks/)
check-shellcheck
check-gate-output
check-gate-fail-closed
check-gate-fixture-coverage
check-gate-exemption-tasks
check-gate-assertions
check-graph
EOF

mkdir -p .workflow
bash gate-sdk/bin/gen-pre-commit.sh --write                       # generate the hook
bash gate-sdk/checks/check-graph.sh --emit > .workflow/CHECK-GRAPH.html
bash gate-sdk/bin/install-hooks.sh                                # opt in this clone

bash gate-sdk/bin/run-gates.sh                                    # the full battery
bash gate-sdk/bin/run-gate-tests.sh gate-sdk/gate-tests gate-sdk/checks  # the kit's own tests
```

Write your first gate by copying `gate-sdk/templates/check-skeleton.sh` to
`scripts/check-<area>.sh`, editing it, adding `check-<area>` to
`scripts/gates.list`, and shipping a `scripts/gate-tests/check-<area>/{good,bad}/`
fixture pair. The meta-gates will hold you to the rest.

## Requirements

bash 4+, git, GNU coreutils/findutils, GNU awk (`check-gate-assertions`),
[ShellCheck](https://www.shellcheck.net/) (`check-shellcheck`).

## License

Apache-2.0 — see the repository root.
