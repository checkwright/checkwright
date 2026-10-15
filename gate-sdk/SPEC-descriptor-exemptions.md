# SPEC amendment: descriptor-exemptions

A gate's exemption list is read by two meta-gates. `check-gate-tamper` refuses an added element that excuses a file staged beside it, and `check-gate-exemption-tasks` holds each element's disposition to a live task. Both parse the list as a shell `# exception-list:` array literal, so a gate whose declaration is a `.gate` descriptor can carry no list either one reads. A compiled gate's module, or a PowerShell gate's script, could hold one only in its own syntax, which neither meta-gate parses.

gate-sdk/SPEC-gate-substrates.md rules the question this entry waited on: **a meta-gate reads the declaration and never learns a substrate** (§The gate-authoring surface, delta 1 there). An exemption list is read without a build, so it is manifest-class and lives in the descriptor. This amendment gives the descriptor that field. It mints no Rust-source reader, which gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate's `check-gate-tamper` row currently says the first ported holder owes.

No tracked declaration carries an exemption list today: `git grep -n -E '^\s*#\s*exception-list:'` over the tree returns fixtures, bespoke tests and one unit-test literal. So no verdict on this tree moves, and the field's first holder is a later gate's.

## What changes

### (1) The `# exception:` descriptor field {design-bearing} {user-facing: gate-tamper-exemption-reader-substrate's opening, "a ported gate's Rust module can carry no exemption the gate is able to read", settled by the SDK ruling its body waited on}

A descriptor declares one exemption element per line:

```text
# exception: <element> # until: <slug>
# exception: <element> # permanent: <reason>
```

`<element>` is one token with no whitespace: a path or glob the gate does not judge, or any other token the gate's own rule reads. The trailing disposition is the shell array's per-element annotation, spelled identically, so one disposition reader serves both grammars. The field takes no tag line: each line names itself.

- **Its readers.** `registry::exemptions(text)` in `native/src/registry.rs` parses a descriptor's text into `(element, disposition)` pairs, where the disposition is until, permanent or none. It is the one parser of the field, and three readers call it. **`check-gate-tamper`** calls it on the staged and the `HEAD` blob of every staged descriptor in its gate-file set, beside its shell-array read, and assertion B applies unchanged to an added path-shaped element. **`check-gate-exemption-tasks`** calls it on every in-scope `.gate` declaration and holds each element's disposition as it holds an array element's. **The declaring gate** reads its own elements through it over its resolved declaration, so a compiled gate's rule and the two meta-gates read one list. An exec member's script reads its descriptor's `# exception:` lines itself, one line each.
- **Where it may sit.** Any `.gate` descriptor, kit or consumer. `check-gate-exemption-tasks`' scope rule applies unchanged: a vendored kit's descriptor is out of scope outside the authoring tree, and counted.
- **What it does not change.** A `.sh` declaration keeps its `# exception-list:` array, read by each meta-gate's existing shell parse. That the two shell parses diverge is filed as its own gap and is not this unit's.

Replacement for the field roster in gate-sdk/SPEC.md §The `# graph:` manifest: one bullet added after `# no-fixture:`. **Not yet applied.** Where gate-sdk/SPEC-gate-substrates.md delta 3 has landed first, the bullet joins the roster that delta wrote, after its `# exec:` bullet. Where this lands first, it joins the current roster, and that delta's replacement roster carries it.

> - `# exception:`, optional, any number — `check-gate-tamper`, `check-gate-exemption-tasks` and the declaring gate itself, all through `registry::exemptions` (§check-gate-exemption-tasks).

`native/src/gates/comment_tier.rs`' `SHELL_COLON` gains `exception:`, since a descriptor line `check-comment-tier` does not bless reds wherever its surface reaches `*.gate`.

### (2) `check-gate-exemption-tasks` holds a descriptor's elements {design-bearing} {user-facing: gate-tamper-exemption-reader-substrate's opening, as delta 1}

Replacement for §check-gate-exemption-tasks' first bullet. **Not yet applied.**

> - Every exemption element in a gate declaration carries exactly one of two disposition annotations: an element of an `# exception-list:`-tagged array in a `check-*.sh` gate, or an `# exception:` line in a `.gate` descriptor (§The `# graph:` manifest). `# until: <slug>` is temporary and must resolve to a live task in the queue file's New Features / Technical Debt / Deferred sections — *live* meaning the slug of an **entry heading** within that span, one per entry, never every slug-shaped token in it. `# permanent: <reason>` is structural out-of-scope. An element with neither, a `# until:` slug that is Done-only or missing, or array elements sharing the array's opening `=(` line are violations.

Replacement for §check-gate-exemption-tasks' paragraph **The header-field arm enters the walk independently**, its first two sentences. **Not yet applied.**

> **The header-field arm and the descriptor arm enter the walk independently of the `# exception-list:` marker.** The array arm opens on that marker, so a declaration carrying only a header field or only `# exception:` lines would be skipped by it, and each of those arms is its own entry into the walk.

The gate's clean line gains one count, `<n> descriptor exemption(s)`, after its array count, so a descriptor arm that found nothing reads as such rather than as an absent arm.

Replacement for §check-gate-exemption-tasks' paragraph **Clean-line contract.**, its first two sentences. **Not yet applied.**

> **Clean-line contract.** The line reports the exemption-array count, the `# exception:` descriptor-element count, the `# port-until:` header-field count, the **out-of-scope kit-shipped declaration count** and the derived live-slug count. All five are §run-gates' vacuous-pass tripwire applied to this gate's sets, and they read in opposite directions.

The sentences after those two (an empty set means an arm ranged over nothing, an absurdly large slug set is the fail-open) stand, each now holding for the descriptor count as for the array count.

### (3) `check-gate-tamper` reads a descriptor's elements {design-bearing} {user-facing: gate-tamper-exemption-reader-substrate's opening, as delta 1}

Replacement for delegation-kit/SPEC.md §Verify after every agent commit, assertion B's first sentence. **Not yet applied.**

> **B — no self-serving exemption.** A newly added path or glob element in any gate's exemption list, an `# exception-list:` array in a script or an `# exception:` line in a descriptor, must not match a file staged in the same commit: an exemption never excuses the change it lands with.

Replacement for the last three sentences of gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate's `check-gate-tamper` row, from **Known limit:** to its end. **Not yet applied.**

> A ported member's exemption list lives in its descriptor as `# exception:` lines, which this gate reads through `registry::exemptions` beside its shell-array read (§check-gate-exemption-tasks). No reader parses a substrate's source, so a ported holder owes no reader of its own.

### (4) The crate cases and fixtures {mechanical}

- **`registry.rs`**: `exemptions` parses an until line, a permanent line and a line with no disposition; ignores a line whose element is empty; and returns nothing for a descriptor carrying none.
- **`gate_tamper.rs`**: a staged descriptor whose blob gains an `# exception:` element is read as an addition, against the `HEAD` blob's set.
- **`delegation-kit/gate-tests/check-gate-tamper.test.sh`** gains a case: a descriptor gaining a path-shaped `# exception:` element that matches a co-staged file reds assertion B, and the same element matching nothing staged is silent.
- **`gate-sdk/gate-tests/check-gate-exemption-tasks/`**: `good/scripts/check-descriptor.gate` gains one `# until: fix-sample-surface` element and one `# permanent:` element. `bad/scripts/check-descriptor.gate` gains one element whose `# until:` names `retired-task` and one with no disposition. Each `expect.txt` gains the lines those findings print and the new count.

### (5) The release declaration {mechanical}

`.workflow/release-declarations.md` §Gate-authoring changes gains one bullet: **a descriptor's exemptions** — a `.gate` descriptor may carry `# exception: <element> # until: <slug>` or `… # permanent: <reason>` lines, held by `check-gate-exemption-tasks` and `check-gate-tamper` as a script's `# exception-list:` array is. Nothing to do; a script's array is unchanged.

## Producers and consumers

- **An `# exception:` line.** Producer: the author of a descriptor; no tracked descriptor produces one today, so the live producer is the next gate needing an exemption, and the fixtures and cases of delta 4 meanwhile. Consumers: `registry::exemptions`, called by the three readers in delta 1. Each field has its reader: `<element>` is read by tamper's assertion B where it is path-shaped and by the declaring gate's rule, and the disposition by `check-gate-exemption-tasks`.
- **Roster-holding readers of the new field name**, by `grep -rn '"exception-list:"\|SHELL_COLON' native/src`: `comment_tier.rs`' `SHELL_COLON` (delta 1). The descriptor field roster in gate-sdk/SPEC.md §The `# graph:` manifest is prose, updated by delta 1.
- **Narrowed corpora.** None. Both meta-gates gain an arm, and each arm adds findings without removing any (point 5 is not engaged).
- **Knobs.** None is minted.
- **The seam.** The field, its grammar and its readers are kit mechanism; every element and disposition is the descriptor author's content.

## Existing sections updated

- `gate-sdk/SPEC.md` §The `# graph:` manifest — the field roster's `# exception:` bullet (delta 1).
- `gate-sdk/SPEC.md` §check-gate-exemption-tasks — the first bullet, the header-and-descriptor arm paragraph and the Clean-line contract paragraph (delta 2).
- `gate-sdk/SPEC.md` §Meta-gate conservation for the binary substrate — the `check-gate-tamper` row (delta 3).
- `delegation-kit/SPEC.md` §Verify after every agent commit — assertion B (delta 3).
- `native/src/registry.rs` — `exemptions` (delta 1).
- `native/src/gates/comment_tier.rs` — `SHELL_COLON` (delta 1).
- `native/src/gates/gate_exemption_tasks.rs` — the descriptor arm and the count (delta 2).
- `native/src/gates/gate_tamper.rs` — the descriptor read (delta 3).
- `gate-sdk/gate-tests/check-gate-exemption-tasks/` and `delegation-kit/gate-tests/check-gate-tamper.test.sh` — the cases (delta 4).
- `.workflow/release-declarations.md` — the §Gate-authoring changes bullet (delta 5).
- `docs/gate-sdk/SPEC.md` and `docs/delegation-kit/SPEC.md` — generated mirrors, stale once a delta lands (all deltas).

## Retired spellings

- None — the amendment adds a descriptor field and retires no spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry moved** — `gate-tamper-exemption-reader-substrate` moves to Done with `--queue done` in the merging commit, ahead of the drain stage.
