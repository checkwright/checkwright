# SPEC-example-gate — amendment fixture (bad)

An invalid `# graph:` manifest embedded in an amendment body — the recurring
failure that motivated assertion G: `dir=` and `valve=` carry legacy tokens
(`mono`/`no-fixture`) that were never legal, undetected until build re-typed
the manifest into a real gate.

## Definition of Done

- [ ] `check-example-gate.sh` carries its manifest
      (`# graph: couples=gate-tests,check-*.sh dir=mono valve=no-fixture tier=precommit`);
      registered in `gates.list`.
