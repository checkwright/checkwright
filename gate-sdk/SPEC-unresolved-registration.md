# SPEC amendment: unresolved-registration

**A `gates.list` name that resolves nowhere becomes one red, not six.** The runner already fails such a member at its registry line, as `FAIL: <name> (unresolved)` (§run-gates). Five siblings re-report the same line, each in its own words, so an adopter upgrading across a withheld or renamed gate reads six unrelated-looking reds for one stale line, and the release owes six Tightened-gates bullets. The runner's result becomes the one owner, and the siblings skip an unresolved member and count the skip.

**Measured at authoring** (reading the code at the paired entry's promotion, plus the entry's own re-verification that a probe name appended to a scratch gates dir reds all five siblings):

- The runner fails an unresolved member in `native/src/runner.rs` `dispatch_one` (`:610-624`). The pre-commit selection keeps it: `select` with a tier pushes an unresolved member ahead of the tier filter (`:366-374`), and the hook dispatches it through the same function. `commit-msg` leaves it to the pre-commit run (§git-hook).
- The five sibling sites, all reading the registry through their own resolve call: `check-gate-output` (`native/src/gates/gate_output.rs:127-134`, exit 1), `check-gate-fixture-coverage` (`gate_fixture_coverage.rs:78-85`, exit 1), `check-gate-substrate-parity` assertion A (`gate_substrate_parity.rs:634-641`, exit 1; its assertions C and I already skip the member), `check-graph` assertion A (`graph.rs:527-537`, exit 1) and `check-kit-enum` (`kit_enum.rs:96-105`, exit 2 at the first one).
- Other registry readers already skip an unresolved member with no finding: the graph emitter's `projected_members` (`native/src/emit/graph.rs:166-169`), `gen-pre-commit`'s manifest read (`emit/git_hooks.rs:48-69`), and `doctor` (`installer/doctor.rs:154-158`), whose comment already rests on the battery reporting it.
- No committed fixture of the five exercises an unresolved member. The one test is bespoke: `gate-sdk/gate-tests/check-graph-tree.test.sh:124` (`want_red a-unresolved`). `gate-sdk/gate-tests/run-arm-contract.test.sh:49` holds the runner's `(unresolved)` line.
- `.workflow/release-declarations.md` carries the cost for the one live instance: `check-rule-citation`'s own Tightened-gates bullet (line 6) and one per sibling (lines 7-11).

## What changes

### (1) The five siblings skip an unresolved member and count it

Each sibling site above stops reporting an unresolved member and counts it instead {design-bearing}. The member takes no further part in the gate's assertions, as `check-gate-substrate-parity`'s assertions C and I already treat it. The clean line gains `<N> unresolved member(s) skipped, reported by the runner` when N > 0, since the parenthetical states what was checked (§Output contract) and a skipped member was not. A red line carries the same count.

- `check-kit-enum` stops exiting 2 at the first unresolved member. Its other refusals are unchanged.
- `check-gate-fixture-coverage` keeps its fixture-dir-first order: a member with a fixture dir is never resolved, and is not counted.

**Why the runner is the owner rather than one of the five.** Every door that runs the registry, the battery (`--run`), the generated pre-commit hook and CI's battery, dispatches each member through `dispatch_one`, so the runner's result reaches every one of them with no gate of its own. `--for` refuses the whole selection at exit 2 on the same member (§run-gates), so no door runs the siblings with the stale line unreported. A sibling cannot own it: a sibling is one registry member, and a consumer may leave it unregistered.

**Fixtures.** No pair gains a red case. The four `good/` cases carrying a registry (`check-gate-output`, `check-kit-enum`, and `check-gate-fixture-coverage` and `check-gate-substrate-parity` under `scripts/`) each gain one name no case file declares, and each `expect.txt` pins the skip-count line. `check-graph`'s pair carries no registry, so its case is the bespoke `check-graph-tree.test.sh` `a-unresolved`, which turns from `want_red` into a clean case asserting the count.

### (2) §Layout and configuration states the one owner and what it removes

gate-sdk/SPEC.md {mechanical}. **Not yet applied.** In §Layout and configuration, the `gates.list` bullet's resolution paragraph gains, after "…while execution resolves separately through `gate_command` (§lib/gate.sh).":

> A listed name that resolves nowhere is one red, the runner's `(unresolved)` result at its registry line (§run-gates). Every other reader of the registry skips it, the gates among them counting the skip on their clean line, so a release withholding or renaming a gate declares one Tightened-gates bullet, the gate's own (installer/SPEC.md §The upgrade contract).

In §run-gates, "A member that resolves nowhere is a failure, not a skip." becomes "A member that resolves nowhere is a failure, not a skip, and this result is its one report (§Layout and configuration)."

In §check-kit-enum, "Fail-closed: an unreadable manifest, an unresolvable registered gate, or a non-repo cwd is red, not a skip." becomes "Fail-closed: an unreadable manifest or a non-repo cwd is red, not a skip. A registered name that resolves nowhere is skipped and counted, the runner's result being its report (§Layout and configuration)."

§check-gate-output, §check-gate-fixture-coverage, §check-gate-substrate-parity (assertion A) and §check-graph (assertion A) each gain one sentence at the registry read: "A registered name that resolves nowhere is skipped and counted (§Layout and configuration)."

### (3) The release declaration surface drops the five sibling bullets

`.workflow/release-declarations.md` {mechanical}. The five sibling Tightened-gates bullets for `check-rule-citation` (lines 7-11 at authoring) are deleted: once delta 1 lands, a vendored tree upgrading meets `check-rule-citation`'s `(unresolved)` red alone, which line 6 already declares. A Behavior-changes bullet is added:

> - **unresolved registration** — a `gates.list` line naming a gate that resolves nowhere is reported once, by the runner, as `FAIL: <name> (unresolved)`; `check-gate-output`, `check-gate-fixture-coverage`, `check-gate-substrate-parity`, `check-graph` and `check-kit-enum` skip it and count the skip. Nothing to do beyond dropping the stale line.

**Both acts, by the release the unit ships in.** Should a release be tagged between the `check-rule-citation` withholding and this unit's landing, the five bullets have shipped in that release's note and are history: the surface then gains only the Behavior-changes bullet. At authoring, no published note names `check-rule-citation`.

### (4) The site mirror follows

`docs/gate-sdk/SPEC.md` is regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commit that lands delta 2 {mechanical}.

## Producers and consumers

- **The skip count.** Producer: each of the five sibling gates, at its registry read, when `registry::resolve` (or the gate's private copy, `gate_output.rs:21-31`) returns none. Consumer: the committing session reading the clean or red line (§Output contract), and `run-gates`' SARIF placement, which places no result for a clean line. No field beyond the count.
- **The one owner.** Producer: `runner::dispatch_one`, unchanged. Consumers: the battery's summary line, the SARIF log's registry-line result (§run-gates), `--emit parse-gates-log` through the `(unresolved)` tail, and the upgrade smoke's allowed-red containment, which reads the red member's name, the withheld gate's own (§upgrade-smoke).
- **A reader whose verdict narrows.** Each sibling's verdict over the registry narrows by the unresolved member (causal-completeness point 5). Red conditions after the change: `check-gate-output` reds a resolved member missing its success or `help:` line; `check-gate-fixture-coverage` a resolved member with no pair and no `# no-fixture:`; `check-gate-substrate-parity` A a resolved member with an ambiguous declaration; `check-graph` A a resolved member's malformed manifest; `check-kit-enum` a resolved member's hand-list. None reds on finding none, asserts an exact member count, or holds a coverage floor over the registry, so dropping an unresolved member only removes findings, and the removed finding is the runner's.
- **The narrow door left unreported.** `run-gates.sh --only <sibling>` runs the sibling alone, and its clean line then carries the skip count naming the runner rather than a red. That is the targeted run's contract (§Layout and configuration, *A targeted run*), and the full battery reports the line.

## Existing sections updated

Roster produced by reading the five sibling modules and `runner.rs` at the sites listed above, `grep -n 'unresolv\|resolves nowhere\|resolves in none' gate-sdk/SPEC.md`, and `grep -rn 'resolves in none\|unresolved' gate-sdk/gate-tests/`.

- `gate-sdk/SPEC.md` — §Layout and configuration, §run-gates, §check-kit-enum, §check-gate-output, §check-gate-fixture-coverage, §check-gate-substrate-parity, §check-graph (delta 2).
- `native/src/gates/gate_output.rs`, `native/src/gates/gate_fixture_coverage.rs`, `native/src/gates/gate_substrate_parity.rs`, `native/src/gates/graph.rs`, `native/src/gates/kit_enum.rs` — the skip and count (delta 1).
- `gate-sdk/gate-tests/check-gate-output/`, `gate-sdk/gate-tests/check-gate-fixture-coverage/`, `gate-sdk/gate-tests/check-gate-substrate-parity/`, `gate-sdk/gate-tests/check-kit-enum/` — each `good/` case's unresolved name and skip-count line (delta 1).
- `gate-sdk/gate-tests/check-graph-tree.test.sh` — the `a-unresolved` case (delta 1).
- `.workflow/release-declarations.md` — the five bullets and the Behavior-changes bullet (delta 3).
- `docs/gate-sdk/SPEC.md` — the regenerated mirror (delta 4).

## Retired spellings

- None — the change narrows five verdicts and mints a clean-line count; it retires no name.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
