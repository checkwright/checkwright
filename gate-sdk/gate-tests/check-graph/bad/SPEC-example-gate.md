# SPEC-example-gate — amendment fixture (bad)

Every finding assertion G can raise, one manifest per group — the recurring
failure that motivated it being the legacy tokens in the second: `dir=` and
`valve=` carried `mono`/`no-fixture`, values that were never legal, undetected
until build re-typed the manifest into a real gate.

## Definition of Done

- [ ] `check-bare-gate.sh` carries no required key and one token that is not a
      key at all (`# graph: mode=partial bogus=1`).
- [ ] `check-example-gate.sh` carries the legacy tokens
      (`# graph: couples=gate-tests,check-*.sh dir=mono valve=no-fixture tier=precommit`);
      registered in `gates.list`.

The remaining two are fenced, so the fenced extraction path reds as well as the
inline one:

```sh
# graph: couples= dir=one valve=none tier=postcommit gen=auto
# graph: couples=docs/[a].md trigger=docs/(x).md dir=one valve=none tier=precommit
```
