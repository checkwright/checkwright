# SPEC amendment: site-count

Pairs `reviewed-site-count`. Some calls are safe only in a reviewed form, and a pattern cannot tell the reviewed form from the unreviewed one: a CLI call that must carry a pinned account, say, when the machine's active account can change under it. What a gate can hold is the count: a person reviewed N sites in a file, and a file whose count moves is reviewed again. This amendment adds check-site-count over a consumer's site patterns and a reviewed-count baseline, and the test-code classifier it and gate-sdk/SPEC-since-base.md share.

**What the tree does today**, each bullet with the probe that settled it:

- check-action-pinning reads `uses:` refs in workflow YAML (gate-sdk/SPEC.md §check-action-pinning); the adopter's ask is not about `uses:` at all (its answer on the entry).
- No gate counts pattern sites per file against a baseline, and no shared reader tells test code from shipped code (`git grep -n -i 'test code\|is_test' -- native/src/walk.rs native/src/gates` finds no classifier).

## What changes

### (1) check-site-count {design-bearing} {user-facing: the adopter input on `reviewed-site-count`}

**Not yet applied**: §Layout and configuration's rule rosters gain `GATE_SDK_SITE_RULES` (default `<gates-dir>/site-rules.list`, **optional**). A new section, §check-site-count:

> `checks/check-site-count.gate` (`precommit`, binary-dispatched). Invariant: every file's count of a configured site equals the count a person reviewed. **The rules file** takes §check-script-preflight's block grammar with one kind, `rule <ID> site-count`, and keys `site <ERE>` (required, repeatable), `baseline <path>` (required, a count baseline, §The count baseline) and `paths <pathspec>…` (default the whole tracked tree). Its corpus is `git ls-files` under the pathspecs and the shared prune dirs, binary files and test code (delta 2) skipped. A file's count is the number of non-overlapping matches of every `site` ERE over its lines.
>
> **The comparison is exact**: a file whose count differs from its row reds `<path>: [<ID>] <found>/<reviewed>`, a file with no row counting as 0 reviewed, so a new site, a removed site and a stale row all red. With no rules file, or one holding no block, it prints `not applicable: no site rule configured`; it declares `# install: on-surface` and `# armed-by: content GATE_SDK_SITE_RULES`. A malformed rules file is exit 2 and an unparseable baseline exit 3, each naming the line; a named baseline that is absent is exit 2.
>
> The pair: `bad/` holds a file over its row, one under it, and one with sites and no row; `good/` holds equal counts, a site inside a test file, and a binary file. The test holds the absent baseline, a baseline line with no tab, and a duplicate row.

- **Why counts and not a site-level waiver.** A comment beside a call is copied with the call; a count lives apart from the code, so a new site cannot arrive pre-approved.
- **Why one generic kind.** The adopter's CLI-account rule is a `site` ERE over an argv literal and a shell command start, consumer config like any vocabulary (§The provenance seam).
- **Honest limit.** A count is blind to which site changed: replacing a reviewed site with an unreviewed one in the same file keeps the count and passes.

### (2) The test-code classifier {design-bearing} {user-facing: the adopter input on `reviewed-site-count` and `stale-test-literals` — one definition of test code}

**Not yet applied**: §Layout and configuration's rule rosters gain `GATE_SDK_TEST_GLOBS` (default `tests/ fixtures/ tests.rs *_tests.rs tests_*.rs test_* *_test.*`). A new section, §The test-code classifier, after §The count baseline:

> A path is **test code** when one of its components matches a glob ending in `/`, or its basename matches any other glob. In a `.rs` file, the lines from the first `#[cfg(test)]` attribute on are test code too, whatever the path. `walk::test_code` answers both, and every content gate that skips or selects test code reads it.

- **Why a knob with a default.** Test layout is a language and project convention; the default names the common ones and a consumer narrows or empties it (the policy-as-choice rule).

## Producers and consumers

- **check-site-count.** Producer: the battery, armed where `GATE_SDK_SITE_RULES` holds a block. Deployed: the orchestrator adopter, over its CLI-account sites *(inferred: its configuration is out of tree, lead-relayed)*. This repo binds no rule: reviewing each of its own CLI call sites is a calibration this unit does not make, so the gate is registered and not applicable here, and `doctor` names it disarmed. Consumers: the committing session, and the SARIF log through the path-only placement (gate-sdk/SPEC-rule-class.md delta 2).
- **The test-code classifier.** Producer: `walk::test_code` over `GATE_SDK_TEST_GLOBS`. Consumers: check-site-count (delta 1) and the stale-literals arm (gate-sdk/SPEC-since-base.md delta 2).
- **Roster readers of the new member:** `gates::REGISTRY`, `check-readme-roster` (gate-sdk/README.md's roster block), `check-install-disposition`, `check-gate-substrate-parity`, `scripts/gates.list`, and the knob rows in `native/src/knobs/gate_sdk.rs`.

## Existing sections updated

- gate-sdk/SPEC.md §Layout and configuration, rule rosters — the two knobs (deltas 1 and 2).
- gate-sdk/SPEC.md §check-site-count, new (delta 1).
- gate-sdk/SPEC.md §The test-code classifier, new (delta 2).
- `native/src/gates/site_count.rs`, `checks/check-site-count.gate`, its pair and test, `gates::REGISTRY`, gate-sdk/README.md's roster block and `scripts/gates.list` (delta 1); `native/src/walk.rs` (delta 2); `native/src/knobs/gate_sdk.rs` (deltas 1 and 2).
- `docs/gate-sdk/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- None — a new gate, two knobs and a classifier; nothing existing is renamed or removed.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
