# SPEC-example-gate — amendment fixture (good)

Every shape assertion G must accept, one per branch it can clear: the four
required keys alone, the full seven-key form with every optional value legal, a
`kit:<glob>` couples token, a `*` trigger, two manifests on one line, a manifest
inside a non-`proto` fence, a `# graph:` span carrying no manifest key at all,
and the two spans the extractor must not read as manifests in the first place.

## Definition of Done

- [ ] `check-example-gate.sh` carries its manifest
      (`# graph: couples=TASK-QUEUE.md,*/SPEC.md dir=one valve=none tier=precommit`);
      registered in `gates.list`. The `*/SPEC.md` surface may be design-ahead —
      existence is not required, only glob syntax.
- [ ] `check-two-gate.sh` (`# graph: couples=docs/two.md dir=one valve=none tier=precommit`) and `check-three-gate.sh` (`# graph: couples=docs/three.md dir=bi valve=PROPOSED tier=align-only`) land together — two manifests on one line, and the second must be validated as well as the first.
- [ ] `check-full-gate.sh` carries every optional key at a legal value
      (`# graph: couples=kit:checks/*.sh,scripts/gates.list trigger=* dir=bi valve=PROPOSED tier=commit-msg mode=whole-tree gen=manual`) —
      the `kit:` prefix validates on its glob part, and a `*` trigger is exempt
      from token validation because it covers every surface.

A manifest inside a fenced block whose language is not `proto` is live design
text and is validated:

```sh
# graph: couples=scripts/gen.sh dir=one valve=none tier=precommit mode=staged
```

A `# graph:` span naming no key at all is a cross-reference, not a manifest, and
assertion G returns without a finding:

```sh
# graph: see check-example-gate.sh above for the manifest this gate will carry
```

A prose mention of the `# graph:` concept, with `couples=` named separately, is
not a manifest and must not be extracted at all.

```proto
// A `# graph: couples=bogus dir=mono` line inside a proto fence is illustrative
// wire context, never a manifest — assertion G must skip it.
message Example { string id = 1; }
```
