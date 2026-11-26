# SPEC amendment: content-gate-calibrations

Pairs `content-gate-calibrations`. Four content gates bake a calibration a consumer cannot bind. Two of them have a structural bound, which replaces the constant. A migration number's width is read off the directory itself, and the shortest string the binary scan reads is derived from the deny terms. The other two become knobs whose defaults are today's values: the pipefail reader's waiver-reason floor, and the stale-literal arm's counting floor. No default moves a verdict on a tree that passes today, apart from the two structural changes, each named with its reach below.

**What the tree does today**, each bullet with the probe that settled it:

- check-migration-guard reads a name as four digits, an underscore, a name and `.sql`, and any other file in the directory is exit 3. So a tree of five-digit or timestamp names cannot arm the gate (`grep -n 'const DIGITS\|fn number' -A8 native/src/gates/migration_guard.rs`). The record prints each number padded to four (`fn padded`). With four digits fixed, a file's name order and its number order agree.
- check-binary-strings reads a string as a run of at least `MIN_RUN`, 4, printable bytes, for term hits and shape hits alike (`grep -n 'MIN_RUN' native/src/gates/binary_strings.rs`). A deny term of two or three characters that sits alone in a short run is never read. The pair's `good/` holds that case as a pass: a three-byte term `qux` whose only occurrence is a run too short to be a string (`gate-sdk/gate-tests/check-binary-strings/good/terms.list`).
- check-pipefail-readers passes `FLOOR`, 1, to the early-exit detector as its waiver-reason floor (`grep -n 'FLOOR' native/src/gates/pipefail_readers.rs`). check-script-preflight's sibling floor is its rules file's `waiver-min` (gate-sdk/SPEC.md §check-script-preflight).
- `--emit stale-literals` counts a fragment at 8 or more characters holding at least 4 ASCII letters. `literals::counts` hard-codes both numbers, and `fragments` is its only caller (`grep -n 'fn counts' -A3 native/src/literals.rs`; `grep -rn 'literals::fragments' native/src`).
- The four calibrations' spellings: `git grep -n -i 'four digits\|NNNN\|4 or more printable\|reason floor\|8 or more characters' -- ':!docs/llms-full.txt' ':!TASK-QUEUE.md'` finds gate-sdk/SPEC.md's four sections, their generated mirror, the dated v0.39.0 post (history, left as it reads), the two comments and the fault text in `migration_guard.rs`, and the comment over `FLOOR`. The comments over `MIN_RUN` (`strings(1)`) and `counts` (`8 characters, 4 ASCII letters`) spell theirs in other words.

## What changes

### (1) A migration number's width is the directory's own {design-bearing} {user-facing: a migration directory whose names share any one width arms the gate, where only four digits did (the entry's deliverable, a numeric read of the leading decimal run, set record 2026-11-26)}

A migration file is named `<digits>_<name>.sql`, `<digits>` one or more ASCII digits and its number their decimal value. **Every file in the directory, at the merge base and in the working tree, carries one width.** That keeps name order and number order the same, so a runner that applies files in name order applies them in the order the gate judged. A free-width numeric read would lose that property: it passes `9_a.sql` before `10_b.sql`, which a name-order runner applies in the other order.

**Not yet applied** — gate-sdk/SPEC.md §check-migration-guard, the sentence "Every file there is named `NNNN_<name>.sql`, four digits; any other file is unparseable input at exit 3, naming it." becomes:

> Every file there is named `<digits>_<name>.sql`, its number the decimal value of `<digits>`. All of them, at the merge base and in the working tree, share the width of the first by name, so their name order is their number order. A file of another shape or width, or a number too large for 64 bits, is unparseable input at exit 3, naming it.

A record prints a number as its file spells it. With one width per directory that is the padded form, so the ORDER record's text is unchanged for a four-digit tree.

`native/src/gates/migration_guard.rs`: `DIGITS` and `padded` go. `number` reads the leading decimal run, and `files` checks the shared width across both sets, naming the first file off it. The width check spans the merge base's and the working tree's names together, so a branch adding a five-digit file to a four-digit history is exit 3, as it is today.

**Both acts, by whether `migration-guard-lint-parity` lands in this batch.** Its DUP and CLASH records print `<n>` as the file spells it, which reads the same with this delta landed or not. Its CLASH arm reads another ref's names through `number` and compares numbers by value. If it lands first, CLASH reads four-digit names until this delta widens `number`. If this delta lands first, CLASH takes `number` as this delta leaves it. Either way the shared-width check reads this directory's two sets alone and never another ref's names, and a name there that `number` refuses is skipped, as that amendment states.

- **Why read the width rather than knob it.** A width knob is a second statement of what the directory's names already show, and a consumer forgetting it gets exit 3 on every commit. The derivation has no such failure, and a directory with no files yet takes the width of its first addition.

### (2) The binary scan reads strings as short as the shortest deny term {design-bearing} {user-facing: a deny term of one to three characters is caught in a run that short, so a consumer listing one can red where it passed (the entry's deliverable, scanning down to the shortest configured deny term, set record 2026-11-26)}

The string floor is 4, or the length of the shortest live deny term that can hit if that is shorter. A term holding a byte outside the token class never hits, so it does not lower the floor. With no term shorter than 4, every verdict stands.

**Not yet applied** — gate-sdk/SPEC.md §check-binary-strings, the sentence "**A string is a run of 4 or more printable ASCII bytes** (0x20 to 0x7E, and tab), the `strings(1)` default." becomes:

> **A string is a run of printable ASCII bytes** (0x20 to 0x7E, and tab) **at least as long as the floor**: the `strings(1)` default of 4, or the shortest deny term able to hit (§The deny-term list), if that is shorter. A term shorter than 4 reaches only a string at least its own length, and the floor goes no lower, so the scan reads no run a term could not fill.

**Not yet applied** — the same section's honest limit "a string shorter than 4 bytes or held in a code section is not read" becomes "a string shorter than the floor, or held in a code section, is not read". The pair sentence's `good/` clause "a three-byte term whose only occurrence is a run too short to be a string" becomes "a run shorter than the floor that a shape would match, beside a shortest term of three", and `bad/` gains "a three-byte term alone in a three-byte run".

`native/src/gates/binary_strings.rs`: `MIN_RUN` stays as the floor's ceiling, `inner` derives the floor from the term list, and `strings` takes it as an argument.

**Both acts, by whether `content-gate-reading-limits` lands in this batch.** That debt fixes or states the sectionless-ELF read in the same section. Its sentence is the ELF one, and this delta's are the string sentence and the honest-limit clause, so whichever lands second keeps the other's text as it finds it.

- **Why derive the floor rather than knob it.** A floor knob beside a term list would be two configurations that must agree: a consumer adding a two-character term would also have to know to lower the floor, which is the silent pass this delta exists to close. Shapes share the floor because one string definition serves both hit kinds. A shape shorter than 4 is the consumer's own choice to match short text.

### (3) The pipefail reader's waiver floor is a knob {mechanical} {user-facing: a new knob, `GATE_SDK_PIPEFAIL_WAIVER_MIN`, defaulting to today's 1 (the entry's deliverable, a consumer knob defaulting to today's value, set record 2026-11-26)}

**Not yet applied** — gate-sdk/SPEC.md §check-pipefail-readers, the sentence "It runs the detector with the waiver token `pipefail-ok` and a reason floor of 1." becomes:

> It runs the detector with the waiver token `pipefail-ok` and the reason floor `GATE_SDK_PIPEFAIL_WAIVER_MIN`, so a consumer holds a waiver's reason to the length §check-script-preflight's `waiver-min` holds its own to.

**Not yet applied** — gate-sdk/SPEC.md §Layout and configuration, a bullet after `GATE_SDK_PIPEFAIL_BASELINE`'s:

> - `GATE_SDK_PIPEFAIL_WAIVER_MIN` (default `1`): the shortest trimmed reason a `pipefail-ok` waiver takes, read by that member and its fix arm. Any value but a positive integer refuses at exit 2.

`native/src/gates/pipefail_readers.rs`: `FLOOR` gives way to the knob, read in `config`, which the fix arm shares, and passed to the detector and to the record's text. A row joins `native/src/knobs/gate_sdk.rs`, its value-shape check beside `GATE_SDK_PIPEFAIL_ALL`'s, and the knob name joins the member's knob list in `native/src/gates/mod.rs` and `pipefail_fix::KNOBS`.

### (4) The stale-literal counting floor is two knobs {mechanical} {user-facing: two new knobs, `GATE_SDK_STALE_LITERAL_CHARS` and `GATE_SDK_STALE_LITERAL_LETTERS`, defaulting to today's 8 and 4 (the entry's deliverable, a consumer knob defaulting to today's value, set record 2026-11-26)}

**Not yet applied** — gate-sdk/SPEC.md §The since-base corpus, the literal bullet's clause "and a fragment counts at 8 or more characters holding at least 4 ASCII letters" becomes "and a fragment counts at `GATE_SDK_STALE_LITERAL_CHARS` or more characters holding at least `GATE_SDK_STALE_LITERAL_LETTERS` ASCII letters". The floor keeps a short or numeric fragment from matching every test that happens to hold it, and a consumer whose messages are short lowers it.

**Not yet applied** — gate-sdk/SPEC.md §Layout and configuration, two bullets after `GATE_SDK_TEST_GLOBS`':

> - `GATE_SDK_STALE_LITERAL_CHARS` (default `8`): the fewest characters a fragment takes to count for `--emit stale-literals` (§The since-base corpus). Any value but a positive integer refuses at exit 2.
> - `GATE_SDK_STALE_LITERAL_LETTERS` (default `4`): the fewest ASCII letters such a fragment holds. Any value but a non-negative integer, or one above `GATE_SDK_STALE_LITERAL_CHARS`, refuses at exit 2, since no fragment could count.

`native/src/literals.rs`: `counts` takes the two floors, and `fragments` passes them through from its caller. `native/src/emit/stale_literals.rs` reads both knobs and adds them to `KNOBS`. Two rows join `native/src/knobs/gate_sdk.rs` with their value-shape checks.

- **Why knobs for these two, where the entry called them borderline.** Each number decides whether a gate verdict or a listed line turns on a given site. They are calibrations by the policy-as-choice rule, and neither has a structural bound to derive. A floor of 1 reads like a structural "a reason is required". But its sibling gate already makes that floor the consumer's, and the grounds that section gives, a one-word reason tells its reader nothing, apply unchanged here.

### (5) The fixtures and the tests {mechanical}

- **check-migration-guard.** `a_migration_name_carries_its_number` takes any width: `42_add.sql` and `20261126120000_x.sql` parse, and the strays keep the other shapes. A unit test holds the shared width: a directory of one width passes, and a file off it is the fault naming it. `check-migration-guard.test.sh` gains a sandbox case of a five-digit history with an addition, clean, and a four-digit history with a five-digit addition at 3.
- **check-binary-strings.** `a_run_shorter_than_four_bytes_is_no_string` becomes a test of the derived floor: 4 with no short term, 3 with a three-character term, and a term outside the token class lowering nothing. `good/` and `bad/` are reshaped as delta 2 states, with the knob files unchanged.
- **check-pipefail-readers.** `check-pipefail-readers.test.sh` gains a floor of 10 that reds a five-character reason the default passes, and a malformed value at 2.
- **stale-literals.** A unit test in `native/src/literals.rs` lowers both floors and counts a fragment the defaults do not. The existing `counts` assertion keeps the defaults.
- **The knob table.** Each new row's value-shape check gets a case in the knob validator's tests, on the model of `GATE_SDK_PIPEFAIL_ALL`'s.

## Producers and consumers

- **The directory's width.** Producer: `files` over the merge base's and the working tree's names. Consumers: `number`'s caller and the records, which print a number as its file spells it. The `--base-dir` arm feeds the stand-in's names through the same check.
- **The string floor.** Producer: `inner`, from the term list after it is read. Consumers: `strings`, for every scanned range, ELF or whole-file. The record's grammar is unchanged.
- **`GATE_SDK_PIPEFAIL_WAIVER_MIN`.** Producer: the knob table and the consumer's knob file. Consumers: check-pipefail-readers' detector call and its short-waiver record, and `--emit pipefail-fix`, which chooses hits as the gate does. The knob validator reads its shape.
- **`GATE_SDK_STALE_LITERAL_CHARS` and `_LETTERS`.** Producer: the knob table. Consumers: `--emit stale-literals` through `fragments` and `counts`. The validator reads both, together for the letters-above-chars refusal.
- **Point 2, roster-holding readers.** §Layout and configuration's knob roster, held to the table by `check-knob-default-coupling` and `check-knob-citation`. `native/src/knobs/gate_sdk.rs`'s rows. The member's knob list in `native/src/gates/mod.rs`, and the two arms' `KNOBS`. `docs/gate-sdk/SPEC.md` is the generated mirror. Each is an update target below.
- **Point 5, narrowing and widening.** Delta 1 widens the corpus the gate judges. A uniform-width directory that was exit 3 is now judged, and the readers of that exit are the stray case in `check-migration-guard.test.sh`, which keeps `README.md` at 3, and `a_migration_name_carries_its_number`, rewritten by delta 5. A mixed-width directory stays exit 3. Delta 2 widens what the binary scan reads only where a term is shorter than 4. Its reader is the `good/` side, whose `qux` case would red, and delta 2 moves it to `bad/`. Deltas 3 and 4 move nothing at their defaults.
- **Point 6.** All four calibrations the entry names are weighed: two structural, two knobs.

## Existing sections updated

Roster probe: `git grep -n -i 'four digits\|NNNN\|4 or more printable\|strings(1)\|reason floor\|8 or more characters' -- ':!docs/llms-full.txt' ':!TASK-QUEUE.md'`, `git grep -n 'DIGITS\|MIN_RUN\|FLOOR\|fn counts' -- native/src/gates/migration_guard.rs native/src/gates/binary_strings.rs native/src/gates/pipefail_readers.rs native/src/literals.rs` and `git grep -l 'GATE_SDK_PIPEFAIL_ALL\|GATE_SDK_TEST_GLOBS'`, over the tracked tree.

- `gate-sdk/SPEC.md` — §check-migration-guard's naming sentence (delta 1), §check-binary-strings' string sentence, honest limit and pair sentence (delta 2), §check-pipefail-readers' detector sentence (delta 3), §The since-base corpus's literal bullet (delta 4), and §Layout and configuration's three new knob bullets (deltas 3 and 4).
- `native/src/gates/migration_guard.rs` — `number`, `files`, the records, the unit tests (deltas 1 and 5).
- `native/src/gates/binary_strings.rs` — the floor and `strings`, the unit test (deltas 2 and 5).
- `native/src/gates/pipefail_readers.rs` — the knob read (delta 3).
- `native/src/literals.rs` and `native/src/emit/stale_literals.rs` — the floors and `KNOBS` (deltas 4 and 5).
- `native/src/emit/pipefail_fix.rs` — `KNOBS` (delta 3).
- `native/src/knobs/gate_sdk.rs` — three rows and their checks (deltas 3, 4 and 5).
- `native/src/gates/mod.rs` — check-pipefail-readers' knob list (delta 3).
- `gate-sdk/gate-tests/check-binary-strings/good/` and `bad/` — the reshaped floor cases (deltas 2 and 5).
- `gate-sdk/gate-tests/check-migration-guard.test.sh` and `check-pipefail-readers.test.sh` — the new cases (delta 5).
- `.workflow/release-declarations.md` — *New and tightened gates* gains delta 2's short-term reach, *Behavior changes* delta 1's width widening, and *Knob changes* the three knobs (deltas 1 to 4).
- `docs/gate-sdk/SPEC.md` — generated mirror (all deltas).
- `docs/llms-full.txt` and `docs/requirements.md`'s install-size block — generated, each regenerated by the command its freshness gate prints on red (all deltas).

## Retired spellings

- `four digits` — §check-migration-guard's naming sentence and the code comments over `number` and its unit test, retired by delta 1.
- `DIGITS` — the constant in `native/src/gates/migration_guard.rs`, retired by delta 1.
<!-- retired-spelling-exempt: the dated v0.39.0 post is release history and keeps the spelling it shipped with -->
- `NNNN_<name>.sql` — the naming form in §check-migration-guard and in the fault text of `native/src/gates/migration_guard.rs`, retired by delta 1.
- `a reason floor of 1` — §check-pipefail-readers' sentence, retired by delta 3.
- `a_run_shorter_than_four_bytes_is_no_string` — the unit test's name, retired by delta 5.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the directory's width, the string floor and the three knobs.
- [ ] **Instruction surfaces: instruction only** — the knob bullets state the value and its refusal, and the grounds stay in the owning sections.
- [ ] **Merged with no information lost** — each passage is re-phrased in place, not appended to.
- [ ] **Amendment deleted** — this file removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` holds.
- [ ] **Run once** — check-migration-guard over a sandbox history of five-digit names with an addition, clean, and check-binary-strings over a file holding a two-character term alone in a two-byte run with that term listed, red.
- [ ] **Release declared** — `.workflow/release-declarations.md` carries the three knobs, the width widening and the short-term tightening.
- [ ] **Entry moved** — `content-gate-calibrations` moves to Done before the drain stage, in the commit that lands the merge.
- [ ] **Gaps filed** — any cross-component gap discovered during the work filed as a debt task.
