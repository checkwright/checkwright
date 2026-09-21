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

The remaining four are fenced, so the fenced extraction path reds as well as the
inline one; the third names a knob whose row is a word list, and the fourth roots
a glob at a knob no kit declares and at an indexed row:

```sh
# graph: couples= dir=one valve=none tier=postcommit gen=auto
# graph: couples=docs/[a].md trigger=docs/(x).md dir=one valve=none tier=precommit
# graph: couples=docs/*.md,knob:GATE_SDK_KIT_DIRS dir=one valve=none tier=precommit
# graph: couples=knob:GATE_SDK_NO_SUCH_DIR/*.md,knob:GATE_SDK_PROGRAM_FLOOR/*.md dir=one valve=none tier=precommit
```
