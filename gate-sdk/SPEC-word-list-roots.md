# SPEC amendment: word-list-roots

**A rooted token may name a word-list knob, one root per word.** `check-shellcheck` lints every direct `*.sh` of each directory `GATE_SDK_LINT_EXTRA_DIRS` names, but its hook trigger cannot follow: the knob is a `.words()` row, and a rooted token `knob:<NAME>/<glob>` refuses one because "a root is one directory" (§The `# graph:` manifest). So an edit in a knob-added directory is linted by the full battery and CI, never at commit, which §check-shellcheck records as an honest limit and names the fix for: a manifest token that expands a word-list knob. The rooted token takes that job with no new prefix: over a `.words()` row it expands to one rooted pattern per word.

**Measured at authoring.**

- `GATE_SDK_LINT_EXTRA_DIRS` is `Row::scalar(…, "").empty_takes_default().words()` (`native/src/knobs/gate_sdk.rs:161`), split by `walk::knob_words` in `native/src/gates/shellcheck.rs:40`, and declared in the member's knob list (`native/src/gates/mod.rs:2198`), so the token is admissible for this member.
- `gate-sdk/checks/check-shellcheck.gate` couples `knob:GATE_SDK_GATES_DIR/*.sh,kit:*.sh`.
- The refusal lives in `knobs::root_refusal` (`native/src/knobs/mod.rs:171-185`), read by `registry::rooted_path` (`native/src/registry.rs:703-733`) for couples and `# projection:` alike, and by `check-graph` at `native/src/gates/graph.rs:358` and `:594`. The crate unit test at `native/src/registry.rs:1202-1221` asserts `knob:GATE_SDK_KIT_DIRS/x` refuses.
- `check-graph`'s pair names no rooted word-list token: `bad/` reds a *bare* `knob:GATE_SDK_KIT_DIRS` (`bad/expect.txt:14`), which this change keeps.

## What changes

### (1) The rooted token expands a word-list knob to one pattern per word

In `native/src/registry.rs`, `rooted_path` resolves a `.words()` row through `walk::knob_words` and returns one path per word, and its two callers, `rooted_pattern` for couples and `knob_paths` for `# projection:`, take every path {design-bearing}. Each word is a root on the scalar root's terms: a trailing `/` trimmed, a leading `./` dropped, `.` read as no prefix, and a word carrying a comma refused. A word list resolving empty expands to **no** pattern, where an empty scalar root still refuses: an empty scalar would re-root the glob at the repository root, while an empty list names no root at all, so no trigger is lost and none is invented.

`knobs::root_refusal` stops refusing a `.words()` row; an indexed or packed row still refuses, since its elements are not a whitespace list of directories. The bare token keeps refusing a `.words()` row, because a word list's words need not be repository patterns (`GATE_SDK_PAYLOAD_WITHHOLD`'s words are kit-root-relative members).

The unit test at `native/src/registry.rs:1202-1221` drops `knob:GATE_SDK_KIT_DIRS/x` from its refusal list and gains a word-list case: two words expand to two covering patterns, and an empty list to none. The manifest in `gate-sdk/gate-tests/check-graph/good/SPEC-second-gate.md` gains `knob:GATE_SDK_LINT_EXTRA_DIRS/*.sh`, holding that `check-graph` admits it.

### (2) check-shellcheck couples the knob-added directories

`gate-sdk/checks/check-shellcheck.gate`'s `couples=` gains `knob:GATE_SDK_LINT_EXTRA_DIRS/*.sh` {mechanical}, so a staged `*.sh` under a knob-added directory fires the gate at commit. The generated hook needs no regeneration for a trigger change, since it reads the manifests at commit time, but `CHECK-GRAPH.html` does (`check-graph` assertion E prints the command).

### (3) §The `# graph:` manifest, §check-graph and §check-shellcheck state the word-list root

gate-sdk/SPEC.md {mechanical}. **Not yet applied.**

In §The `# graph:` manifest, the rooted paragraph's sentence "An indexed, packed or `.words()` row is refused there, since a root is one directory." becomes:

> An indexed or packed row is refused there. A `.words()` row roots the glob at each of its words, one pattern per word, and a list resolving empty expands to none, since it names no root to lose.

In §check-graph, "Its knob must hold one directory, so a name no static kit declares and an indexed, packed or `.words()` row are findings in both places" becomes "Its knob must hold a directory or a word list of them, so a name no static kit declares and an indexed or packed row are findings in both places".

In §check-shellcheck, the paragraph opening "**The knob widens what the gate scans, not when the hook fires it.**" is replaced by:

> **The knob widens when the hook fires it too.** The gate's `# graph:` couples `knob:GATE_SDK_GATES_DIR/*.sh,kit:*.sh,knob:GATE_SDK_LINT_EXTRA_DIRS/*.sh`, the last a rooted token over a word list, one trigger per named directory (§The `# graph:` manifest). The kit family is covered through the `case`-pattern matcher, where `*` spans `/`, so `<kit>/*.sh` matches every script in every subdirectory of that kit.

### (4) The site mirror and the graph artifact follow

`docs/gate-sdk/SPEC.md` is regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`, and the coupling-graph artifact with the command `check-graph` assertion E prints, in the commit landing deltas 2 and 3 {mechanical}.

## Producers and consumers

- **The widened expansion.** Producer: `registry::rooted_path`, on any couples or `# projection:` token rooting a `.words()` row. Enabling configuration: the check-shellcheck descriptor after delta 2, in every tree that registers the gate (this one does: `scripts/gates.list:6`); the knob's value is empty by default, so the token is inert there until a consumer names a directory. Consumers are every reader of `registry::expand_couples` and `registry::knob_paths`: the hook selector and `run-gates --for` (triggers), `check-graph` (manifest loop, admissibility and assertion E's artifact), the graph emitter, `check-gate-substrate-parity` assertion G, `check-reads-couples` (whose coverage of the gate's knob-added walk roots now resolves), `port-blockers` (prints the field), `check-projection-roster` and `--projection-witness` (no member declares a word-list projection today, so their output is unchanged).
- **A reader whose red condition moves.** `check-graph` stops reddening a rooted `.words()` token; it still reds the bare form and an indexed or packed root. `check-reads-couples` can only gain coverage. No reader reds on finding none over this token's expansion; an empty word list yields no pattern, and the member's other couples keep its trigger set non-empty.
- **No new state or field.**

## Existing sections updated

Roster produced by `grep -rn 'root_refusal\|rooted_path\|rooted_pattern\|a root is one directory' native/src gate-sdk/SPEC.md` and `grep -rn 'knob:[A-Z_]*/' gate-sdk/gate-tests/check-graph*` at authoring.

- `native/src/registry.rs` — `rooted_path`, `rooted_pattern`, `knob_paths` and the unit test (delta 1).
- `native/src/knobs/mod.rs` — `root_refusal` and its comment (delta 1).
- `native/src/gates/graph.rs` — the refusal message at `:364` that names the one-directory rule (delta 1).
- `gate-sdk/gate-tests/check-graph/good/SPEC-second-gate.md` — the admitted token (delta 1).
- `gate-sdk/checks/check-shellcheck.gate` — the couple (delta 2).
- `gate-sdk/SPEC.md` — §The `# graph:` manifest, §check-graph, §check-shellcheck (delta 3).
- `docs/gate-sdk/SPEC.md` and the coupling-graph artifact at `GATE_SDK_GRAPH_ARTIFACT` — regenerated (delta 4).

## Retired spellings

- None — the token keeps its spelling and admits a further row shape; no name is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
