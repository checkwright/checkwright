# SPEC amendment: policy-residue

The `consumer-policy-seam` census left eleven members owing a ruling under doctrine-kit/DOCTRINE.md's Policy-as-choice rule. The rule's discriminator:

- **A policy** is "a calibration another adopter could reasonably set differently while the gate's stated contract stays true".
- **Not a policy** are "a grammar the kit ships", "a contract invariant or an external limit", and "a value that changes no verdict".
- **`off`** belongs "wherever the gate's other assertions survive without that calibration". `off` for a calibration that is the gate's whole predicate "is unregistering the gate, which the registry already offers".
- **A structural bound**, "where one exists … beats a count".

The census's own rule is gate-sdk/SPEC.md §Calibration lessons: a matching window is a structural bound where the grammar has one, otherwise a knob, and never a bare count baked into a module.

**The ruling, per member.** Each fixed value takes a ground in the discriminator's terms, a knob, or a structural bound in place of a count. Each knob admits `off` or states that it is its gate's whole predicate. A ground written in those terms is also what lets a consumer's calibration sweep decline the member by citation instead of re-reading it, since no hand-kept roster of windows is permitted.

| Member | Class | Disposition |
|---|---|---|
| `check-shellcheck`'s `-S warning` | contract | already grounded as the Self-lint contract; no edit |
| `check-action-run-shell`'s `-S warning` | contract | ground restated (delta 1) |
| `check-prose-tells`' contrast and tricolon patterns | grammar | ground restated (delta 2) |
| `check-prose-tells`' all-caps run of 3 | policy | knob, `off` admitted (delta 2) |
| `check-provenance-seam`'s 7..40 hex run | external limit | ground restated (delta 3) |
| `check-queue-prose-precondition`'s past-tense list | policy | knob, `off` admitted (delta 4) |
| `check-prose-enum`'s 8- and 16-byte gaps | window | structural bound replaces the counts (delta 5) |
| `check-fence-run`'s `DEPTH_BACKSTOP=3` | contract | ground restated (delta 6) |
| `check-gate-assertions`' count words to nine | grammar | the shared cardinal grammar replaces the list (delta 7) |
| the msg-patterns template's `{0,6}` reach | policy, consumer-owned | ground restated (delta 8) |
| `QUEUE_KIT_WRAP_BUDGET` | whole predicate | ground stated, no `off` (delta 9) |
| `LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP` | policy | `off` admitted (delta 10) |
| `LIFECYCLE_KIT_SHIM_NGRAM` | whole predicate | ground stated, no `off` (delta 11) |

**Measured at authoring (2026-09-23)**, by the research dispatch and re-run here where marked:

- **The code sites.**
  - `native/src/gates/shellcheck.rs:74` and `action_run_shell.rs:639` pass `-S warning`.
  - `prose_tells.rs:292-303` is the run of three.
  - `provenance_seam.rs:408` is `(7..=40)`.
  - `queue_prose_precondition.rs:10-11` is `PAST_TENSE_RE_SRC`.
  - `prose_enum.rs:167-190` is `adjacent`, with `gap.len() <= 8` and `<= 16`.
  - `fence_run.rs:14` is `DEPTH_BACKSTOP`.
  - `gate_assertions.rs:11-12` and `:54-66` are the `two`…`nine` list.
  - `gate-sdk/templates/msg-patterns.list:29` is the `{0,6}` reach.
  - The three knob validators are `native/src/knobs/queue_kit.rs:71-80` and `lifecycle_kit.rs:164-174`.
- **Re-run here: `check-gate-assertions` moves no verdict when widened.** `native/target/release/checkwright-gates check-gate-assertions` prints `9 of 9 enumerated contract(s) coupled`. `git grep -n -i -E` for a count word from `ten` to `ninety` before `assertion(s)|axes|axis|checks` over `*/SPEC.md` finds one live line, `gate-sdk/SPEC.md:2394` ("Ten assertions" in §check-gate-substrate-parity). That is the section's *second* paragraph, since a lone descriptor line is its first, and discovery reads the first paragraph only (`gate_assertions.rs:137-140`).
- **`QUEUE_KIT_WRAP_BUDGET`'s gate is unregistered here.** `scripts/gates.list` carries `# unregistered: check-queue-wrap`, the `off` a consumer takes by the registry.
- **Inferred, cannot run before build:** dropping `check-prose-enum`'s byte caps moves no verdict on this tree — the caps are compiled constants, so the only run is the battery over a build without them, which delta 5 owes.

## What changes

### (1) `check-action-run-shell`'s severity is the Self-lint contract's {mechanical}

**Not yet applied.** In gate-sdk/SPEC.md §check-action-run-shell, "**Severity is `-S warning`**, the gate family's level, so one threshold governs all ShellCheck lint in the tree." becomes:

> **Severity is `-S warning`, and it is contract rather than calibration**: it is §Self-lint's level, the one every shell surface the family lints is certified at — kit-shipped workflow templates' `run:` blocks among them — so a per-gate level would certify the same shell at two levels.

`check-shellcheck` takes no edit. Its invariant already names "(the self-lint contract)".

### (2) `check-prose-tells`: the patterns are grammar, the run length is a knob {design-bearing}

**Not yet applied.**

- **The knob.** `native/src/knobs/canon_kit.rs` gains `Row::scalar("CANON_KIT_PROSE_TELL_ABBR_MIN_LEN", "3")`. The validator admits an integer of at least `2`, or `off`. At one letter every capital is an abbreviation.
- **The code.** `native/src/gates/prose_tells.rs` `flush_file` reads the minimum instead of the fixed three-byte lead: the first byte an uppercase letter, then uppercase letters or digits to the minimum length. At `off`, assertion D is skipped and the other assertions run.
- **The registry.** The member's row in `native/src/gates/mod.rs` declares the knob.
- **The test.** A crate unit case pins `2`, the default and `off`.

In canon-kit/SPEC.md §check-prose-tells:

- Assertion D's "an all-caps token of length ≥ 3" becomes "an all-caps token at least `CANON_KIT_PROSE_TELL_ABBR_MIN_LEN` long (default `3`; `off` skips this assertion alone)".
- "Exact detection regexes are implementation, owned by the gate source;" becomes "Exact detection regexes are implementation, owned by the gate source, and each is the grammar of the shape its assertion names — bounded by the sentence (contrast) or the clause (tricolon), never by a count, its calibration being that assertion's `*_MAX` knob;".

In canon-kit/SPEC.md §Layout and configuration, the `CANON_KIT_PROSE_TELL_*` threshold list gains:

> `CANON_KIT_PROSE_TELL_ABBR_MIN_LEN` — the shortest all-caps token the undefined-abbreviation tell reads, an integer of at least `2` or `off`, default `3`;

### (3) `check-provenance-seam`'s hex bounds are git's object-name format {mechanical}

**Not yet applied.** In canon-kit/SPEC.md §check-provenance-seam, the hex-reference bullet's "That is the shape of an abbreviated or full git object name and of a digest prefix;" becomes:

> The bounds are an external format rather than a calibration: 40 is a full SHA-1 object name and 7 git's default abbreviation, so the run is the shape of an abbreviated or full object name and of a digest prefix. A SHA-256 repository's 64-character names, and names a `core.abbrev` below 7 shortens, fall outside it — an honest limit of the format chosen, not a threshold to tune;

### (4) `check-queue-prose-precondition`'s past-tense strip is a knob {design-bearing}

**Not yet applied.**

- **The knob.** `native/src/knobs/queue_kit.rs` gains `Row::scalar("QUEUE_KIT_PRECONDITION_PAST_REGEX", <today's PAST_TENSE_RE_SRC>)`. The validator refuses empty, as it does for `QUEUE_KIT_PRECONDITION_REGEX`, and admits `off`.
- **The code.** `native/src/gates/queue_prose_precondition.rs` drops the `PAST_TENSE_RE_SRC` const and compiles the knob's value, exiting 2 when it does not compile. At `off` it skips the strip, and the trigger assertion runs as before.
- **The registry.** The member's row declares the knob.
- **The test.** A crate unit case pins the default and `off`.

In queue-kit/SPEC.md §Layout and configuration, after the `QUEUE_KIT_PRECONDITION_REGEX` bullet, add:

> - `QUEUE_KIT_PRECONDITION_PAST_REGEX` — the past-tense narration `check-queue-prose-precondition` strips before its trigger matches, default = the shipped phrase set; `off` strips nothing. A consumer who adds a forward verb to the trigger adds its past form here.

In queue-kit/SPEC.md §check-queue-prose-precondition's calibration, "past-tense narration stripped before matching" becomes "past-tense narration (`QUEUE_KIT_PRECONDITION_PAST_REGEX`) stripped before matching".

### (5) `check-prose-enum`'s adjacency is structural {design-bearing}

**Not yet applied.** In `native/src/gates/prose_enum.rs` `adjacent`, the `gap.len() <= 8` and `gap.len() <= 16` guards come off. A gap chains when it carries no letter and at least one delimiter, or no letter but a single `and` or `or`. Its unit tests at the bottom of the module move with it.

**This delta's act depends on the battery, so both acts are stated:**

- **The battery stays green over the change.** The caps are dropped, and the SPEC edit below lands.
- **`check-prose-enum` reds a finding the caps suppressed.** Read each finding. A true hand list is fixed in the same commit. Where a finding is not a list, the structural bound is insufficient: the caps stay, the member takes a knob instead of this delta's ruling, and that is escalated before it is built.

In canon-kit/SPEC.md §check-prose-enum, after "and a list always spells its separator.", add:

> The chaining test is structural rather than a count: a separator is a gap carrying no letter, holding a delimiter or a single `and`/`or`, whatever its length.

### (6) `check-fence-run`'s backstop is a termination bound {mechanical}

**Not yet applied.** In canon-kit/SPEC.md §check-fence-run's nesting-marker paragraph, "until a backstop depth of three ends a doc that names itself." becomes:

> until a backstop depth of three ends a doc that names itself — a termination bound and not a calibration: it sits one above the deepest nesting the kit's own fence chain reaches (a README fence running this gate's fixture suite, which runs its named docs), so it changes no verdict except a self-naming doc's, which it exists to end.

### (7) `check-gate-assertions` reads the shared cardinal grammar {mechanical}

**Not yet applied.** `native/src/gates/gate_assertions.rs` drops its `two`…`nine` alternation and `word_num`. It reads the count word through canon-kit's spelled-cardinal grammar, `spec::cardinal_word_value` in `native/src/spec.rs` (§check-manifest-count owns it), and requires a value of at least two. In gate-sdk/SPEC.md §check-gate-assertions, "adjacent to the count-word (`two`…`nine`)" becomes "adjacent to a count-word — a spelled cardinal of two or more, in canon-kit's cardinal grammar (canon-kit/SPEC.md §check-manifest-count), since the word is grammar and a hand list stopping at nine silently drops a tenth assertion".

### (8) The account-noun reach is the consumer's copy {mechanical}

**Not yet applied.** In gate-sdk/SPEC.md §check-commit-msg, after the paragraph opening "**That exclusion is what makes it tree-exact**", add:

> **The reach is a policy, and the consumer already holds it.** The clause bounds the match structurally; the code-point reach inside it is a calibration tuned for tree-exactness. It ships in the pattern template a consumer copies into its own gates dir and reads through `GATE_SDK_MSG_PATTERN_FILES`, so the consumer's copy is where it is recalibrated — the consumer-selectable form Policy-as-choice asks for, with no knob beside a file the consumer already owns.

### (9) `QUEUE_KIT_WRAP_BUDGET` is its gate's whole predicate {mechanical}

**Not yet applied.** In queue-kit/SPEC.md §Layout and configuration, the bullet becomes:

> - `QUEUE_KIT_WRAP_BUDGET` — default `100` (`check-queue-wrap` gate floor). It takes no `off`: the budget is that gate's one red condition, so `off` would be unregistering the gate, which the registry already offers.

### (10) `LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP` admits `off` {design-bearing}

**Not yet applied.**

- **The validator.** In `native/src/knobs/lifecycle_kit.rs`, the knob leaves the shared positive-integer loop for an arm of its own that also admits `off`.
- **The code.** `native/src/gates/audit_roster.rs` reads the cap as an option. At `off`, assertion B is skipped, and the help and clean lines say no cap is set.
- **The test.** `lifecycle-kit/gate-tests/check-audit-roster.test.sh` gains an `off` case: an over-long line, clean.

In lifecycle-kit/SPEC.md:

- **§Layout and configuration.** The bullet becomes:

  > - `LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP` — positive integer bytes bounding every roster line (§check-audit-roster), or `off`; default `1500`. A stated policy rather than a derived number, `QUEUE_KIT_ENTRY_CAP`'s posture: no prior byte cap exists to derive it from. It is sized so a `scope` line holds a class's standing readings as a paragraph, while a dozen-class roster at the cap on every line still reads whole in one pass. `off` drops assertion B and leaves A, C and D. The table validator refuses any other value at exit 2.

- **§check-audit-roster.** "(B) no line exceeds `LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP` bytes" becomes "(B) no line exceeds `LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP` bytes, skipped at `off`".
- **§The audit roster.** "Every line is bounded by `LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP`, so a whole-file read stays affordable at the review." becomes "Every line is bounded by `LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP` unless the consumer set it `off`, so a whole-file read stays affordable at the review."

### (11) `LIFECYCLE_KIT_SHIM_NGRAM` is its gate's whole predicate {mechanical}

**Not yet applied.** In lifecycle-kit/SPEC.md §Layout and configuration, the bullet becomes:

> - `LIFECYCLE_KIT_SHIM_NGRAM` — the shared-n-gram width `check-shim-restatement` trips at (positive integer; §check-shim-restatement); default `9`. It takes no `off`: the width is that gate's one red condition, so `off` would be unregistering the gate.

## Producers and consumers

- **`CANON_KIT_PROSE_TELL_ABBR_MIN_LEN`** (delta 2) and **`QUEUE_KIT_PRECONDITION_PAST_REGEX`** (delta 4).
  - Producers: each kit's table, whose defaults are today's values, so an upgrade moves no verdict. The doctrine's default rule reads "the value it shipped with".
  - Consumers: the one member each, which declares it.
  - Roster-holding readers: `check-knob-default-coupling` reds a knob whose owning SPEC states no default, and each bullet above states it. `check-knob-citation` reads the SPEC mentions. `--emit knob-roster` is derived. `templates/*.knobs` are one-line pointers. `check-kit-ref-liveness` resolves each name against its kit's table.
- **The `off` value** (delta 10). Reader: `check-audit-roster`, at its B arm.
- **Point 5.** Delta 5 removes two bounds from a chaining test. `check-prose-enum` reds per chained list, a monotone verdict, and its two acts above cover a widened red set. No other delta narrows a corpus.
- **Point 6.** The obliged corpus is the census's eleven members, and the table names each one's value.

## Existing sections updated

Roster from the census's code sites above, `grep -n "S warning\|length ≥ 3\|7 to 40\|past-tense narration\|backstop depth\|(\`two\`…\`nine\`)\|within a few characters\|WRAP_BUDGET\|AUDIT_ROSTER_LINE_CAP\|SHIM_NGRAM" */SPEC.md`, run 2026-09-23.

- `gate-sdk/SPEC.md` §check-action-run-shell (delta 1).
- `native/src/knobs/canon_kit.rs` and `native/src/gates/prose_tells.rs` (delta 2).
- `canon-kit/SPEC.md` §check-prose-tells and §Layout and configuration (delta 2).
- `canon-kit/SPEC.md` §check-provenance-seam (delta 3).
- `native/src/gates/queue_prose_precondition.rs` and `native/src/knobs/queue_kit.rs` (delta 4).
- `queue-kit/SPEC.md` §Layout and configuration and §check-queue-prose-precondition (deltas 4 and 9).
- `native/src/gates/prose_enum.rs` and `canon-kit/SPEC.md` §check-prose-enum (delta 5).
- `canon-kit/SPEC.md` §check-fence-run (delta 6).
- `native/src/gates/gate_assertions.rs` and `gate-sdk/SPEC.md` §check-gate-assertions (delta 7).
- `gate-sdk/SPEC.md` §check-commit-msg (delta 8).
- `native/src/knobs/lifecycle_kit.rs`, `native/src/gates/audit_roster.rs` and `lifecycle-kit/gate-tests/check-audit-roster.test.sh` (delta 10).
- `lifecycle-kit/SPEC.md` §Layout and configuration, §check-audit-roster and §The audit roster (deltas 10 and 11).
- `native/src/gates/mod.rs`, the two new knobs' declarations (deltas 2 and 4).
- `scripts/git-hooks/pre-commit` and the graph artifact, regenerated for the new declarations (deltas 2 and 4).
- `.workflow/release-declarations.md` (deltas 2, 4, 5, 7 and 10):
  - **Behavior changes:** two new knobs, `CANON_KIT_PROSE_TELL_ABBR_MIN_LEN` and `QUEUE_KIT_PRECONDITION_PAST_REGEX`, carrying today's values, and `off` for `LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP`.
  - **Tightened gates:** `check-prose-enum`'s chaining is no longer length-capped, and `check-gate-assertions` discovers a contract counted past nine.
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`, `docs/canon-kit/SPEC.md`, `docs/queue-kit/SPEC.md` and `docs/lifecycle-kit/SPEC.md`.

## Retired spellings

- `PAST_TENSE_RE_SRC` — the const the knob replaces (delta 4).

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for both knobs and the new `off`.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `policy-choice-census-residue` moves to Done in the merge commit, at a stage before the drain stage, which unblocks `repo-inherits-policy-defaults`.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` is green on the declared spelling.
- [ ] **Gaps filed.** The discovery gap measured above is filed to the gap inbox with this amendment's commit. It is a first paragraph holding only a descriptor line, which hides a section's enumerated contract from `check-gate-assertions`.
