# SPEC amendment: printed-followup

**`init` ends by telling an adopter to run two commands, and nothing anywhere checks that either
one resolves.** The strings are `printf` format literals in installer shell source, so no gate's
corpus reaches them in any form: `check-docs-cmd`'s subject is a fenced invoked repo-relative
`.sh` path in markdown, and the consumer smoke — which already holds `init`'s whole output in a
variable — reads exactly one line out of it. This amendment adds the assertion, and it takes the
operand from what `init` actually printed rather than from a second copy of the pair.

**What this amendment asserts, and what it deliberately does not.**

- **Asserted** — the three deltas below, each checkable on this host by one `installer_smoke` run.
- **Not asserted** — that the printed commands *succeed*. The assertion is that each resolves
  against the payload just installed; whether the battery it names then passes is the business of
  the arm that already asserts the battery, two steps earlier in the same function. Conflating the
  two would make one arm's red unreadable as either.

**The second copy is refused, and refusing it is the whole design.** The cheap form of this
assertion is to write the expected pair into the smoke and compare. That mints exactly the defect
the entry describes — a hand-kept copy of a string, in a second place, that a rename must be
remembered to move. The assertion instead **parses the pair out of `init`'s captured output**, so
a rename moves the printed string and the assertion follows it with no edit. The cost of that
choice is a grammar: `init`'s follow-up block becomes a shape the smoke can read, stated below and
gated by the smoke rather than by prose.

## What changes

### (1) `init`'s follow-up block gets a stated shape

`installer/lib/init.sh:426-428`'s `next:` block becomes a declared grammar rather than incidental
formatting: a `next:` line alone on its line, then one command per line, each indented and
carrying its explanation after a `#` {design-bearing}.

That is what the three lines already look like. What changes is that the shape becomes a
**contract with a named reader** instead of a layout choice, so a later edit that reflows the
block reds the smoke rather than silently un-covering the pair. The grammar is stated in
`installer/README.md` §init beside the narration of the block, in the terms a reader of the
script needs: the banner line is the block's start, the block ends at the first line that is not
an indented command, and everything from the first unquoted `#` on a command line is commentary.

**Why a printed grammar rather than a machine-readable side channel.** A `--print-followups` arm
or a JSON emission would be unambiguous and is refused: it mints a new adopter-facing interface
whose only consumer is this harness, on the one binary-selecting script objective 6 is shrinking,
and it leaves the *printed* pair — the thing an adopter actually copies — as uncovered as it is
today. Reading what the adopter reads is the only assertion that covers what the adopter does.

### (2) The consumer smoke asserts that each printed command resolves against the payload it just installed

`assert_install()` gains an arm, immediately after `:280`, that extracts the follow-up block from
the `out` it already holds and asserts each command's script path exists and is executable inside
the consumer, and that each flag the command names is one the target accepts {design-bearing}.

`run-smoke.sh:278-280` already captures the whole invocation, `2>&1`, into the local `out`, and
today greps one `^INIT:` line out of it. The operand is therefore already in hand and the arm
costs no second `init` run.

Three assertions per extracted command, in this order, because each makes the next meaningful:

- **The block is present and non-empty.** Zero commands extracted is a red, not a skip. This is
  the arm's fail-closed half and it is the one that matters most: an assertion over an empty set
  passes vacuously, and a reflowed banner would otherwise turn full coverage into silent zero
  coverage — the same failure shape `check-queue-sections` exists to close under its own scanners.
- **The script path resolves and is executable**, relative to the consumer directory the arm was
  handed. This is the assertion the entry names, and it is the one a rename breaks.
- **Each flag on the command is accepted by that script.** `--install-hooks` is the live case: the
  hooks cut renamed the opt-in behind it, and a path that still resolves while its flag does not
  is precisely the half a path check alone misses. The probe is the script's own refusal
  behavior — an unknown flag is expected to be refused — run in the consumer, against the payload
  under test, and never against this repo's own tree.

**Which `init` invocation the arm rides, and why it is not the obvious one.** It rides the first
`init --profile` call at `:278`, never the idempotent re-run at `:373`: the no-op branch at
`installer/lib/init.sh:405-412` prints no banner at all, so an arm placed on the re-run would
assert over an empty block on every profile and pass by vacuity — the exact hole the first
assertion above closes, arriving through the back door.

**Its verdict class.** A printed command that does not resolve is a statement about the payload
`init` just wrote, so it is `fail` at exit 1. A block the arm could not read at all — `init`
succeeded and printed nothing the grammar matches — is `blocked` at exit 2, this harness's own
precondition. That is the same line the manifest arm draws and this amendment adopts it rather
than inventing a second one.

### (3) The two surfaces that narrate the pair stop paraphrasing it

`installer/README.md` §init states the block's grammar and its assertion, and §The consumer smoke
adds the arm to its per-profile roster {mechanical}.

`installer/README.md:184-189` narrates the pair today as `run-gates.sh --install-hooks` and
`gate-sdk/bin/run-gates.sh`, while the script prints `bash gate-sdk/bin/run-gates.sh
--install-hooks` and `bash gate-sdk/bin/run-gates.sh`. **The prose and the print already
disagree**, which is the drift the entry predicts, arrived early and on the doc side rather than
the code side. Both narrations become descriptions of the block and its grammar — what the block
is for, where it starts and ends, and that the smoke reads it — rather than a second spelling of
the strings, so there is nothing left there to drift.

The per-profile arm roster at `:1363-1379` gains the new arm in its running order, between the
`doctor` assertion and the value arm.

## Producers and consumers

**New state: the follow-up block's grammar (delta 1).**

- **Producer.** `installer/lib/init.sh:426-428`, on the vendoring path only, unconditional there
  and reached by every `init` that writes a profile. It is already emitted on that path today —
  the delta constrains its shape, it does not add an emission — so there is no enabling
  configuration to set and no deployment where the producer is dark.
- **Consumer.** The new arm in `assert_install()` (delta 2), by reading the `out` variable
  `run-smoke.sh:278` already holds. Second consumer: the adopter, unchanged, who reads it on a
  terminal.
- **Every field has a named reader.** The block's fields are the banner line, one or more command
  lines, and each line's trailing commentary. The banner is read as the block's start delimiter;
  each command line's leading token is read as a path and its flags as flags; the commentary is
  read by the adopter and **deliberately by nothing else**, which is why the grammar names it as
  commentary rather than leaving the arm to guess where a command ends.

**New state: the extracted command set (delta 2).**

- **Producer.** The extraction step in `assert_install()`, from a variable already in scope.
- **Consumers, and each one's red condition rather than its subject.** The emptiness assertion
  reds on a **zero count** — which is a non-monotone reader, so it is named explicitly: narrowing
  what the grammar matches can *add* a violation here rather than only remove one, and that is
  the direction this arm is most likely to be broken from. The resolution assertion reds on a path
  that is absent or not executable. The flag assertion reds on a flag the target does not refuse
  as unknown. All three route through `fail`; a block that cannot be read at all routes through
  `blocked`.
- **The suite-side reader.** The arm runs inside `assert_install()`, so it rides that function's
  existing log header and the `installer_smoke` parser
  (`scripts/evidence-config.sh:22`, `--emit parse-smoke-log`) needs no change: the parser grades
  by arm header, and no new top-level header is minted. Had the arm been a new top-level one it
  would have owed a header the parser recognizes, which is why it is not.

**Existing integration prose describing the prior flow, updated rather than left to drift.**
`installer/README.md:184-189` describes `init` as printing "the two commands that finish the
setup" and spells them; §The consumer smoke's per-profile roster at `:1363-1379` enumerates what
each profile is put through and does not mention this pair at all. Delta 3 rewrites both. The
first is the one a reader checks first, and leaving it spelling a paraphrase beside a gated
grammar would leave two sources for one string on the very surface this unit exists to
de-duplicate.

**No knob is minted by any delta.** The corpus is this repo's own installer and its own
acceptance harness; `run-smoke.sh` is declared `no-port`, and `init.sh`'s change is a shape
constraint on an existing `printf`, not a new install step — the standing obligation from
TRAJECTORY.md §The closed rulings, *add no new shell-only install step*, is therefore not
approached.

**Coherence with the sibling amendment on the same two surfaces.**
`SPEC-verdict-witness.md` edits `run-smoke.sh:319-336` and `manifest_report()`, and §The consumer
smoke's sampling, refusal and truth-table passages. This one edits the block after `:280` and the
section's per-profile roster and §init. The regions are disjoint. The one rule they share — exit
1 is a finding about the consumer, exit 2 is a failure of the harness's own construction — is
stated in both because both need it and is owned by `installer/README.md:1463-1472`, which
neither restates.

## Existing sections updated

- `installer/README.md` §init — the narration at `:184-189`, which stops spelling the pair and
  states the block's grammar, its start and end, and that the smoke reads it (deltas 1 and 3).
- `installer/README.md` §The consumer smoke — the per-profile arm roster at `:1363-1379`, which
  gains the arm in its running order (deltas 2 and 3).
- `installer/lib/init.sh` — the follow-up block at `:426-428` and its `# spec:` coupling to the
  grammar's new home (delta 1).
- `installer/consumer-smoke/run-smoke.sh` — `assert_install()` at `:274` onward, the block
  immediately after `:280`, and the `# spec:` comment stating why the arm rides the first `init`
  and not the re-run (delta 2).
<!-- update-target-exempt: the gate's stated corpus is markdown fences and this unit deliberately covers a surface no markdown gate can reach -->
- `gate-sdk/SPEC.md` §check-docs-cmd — **deliberately untouched**. Its fence-scope limit is
  correct as it stands and this unit is not a re-filing of it; a printed format string in shell
  source is outside any markdown-fence corpus by construction, and widening that gate to reach one
  would be a different unit with a different oracle.
- `TASK-QUEUE.md` — `installer-printed-followup-commands-uncovered` promotes to New Features with
  this file's `[spec:]` ref, in the same commit as this file (all deltas).

## Definition of Done

- [ ] **Causal completeness** — the block grammar and the extracted command set each have a
      producer on an already-live path, a named consumer, and no field without a reader; the
      commentary field's non-reader is stated rather than left implied.
- [ ] **No second copy of the pair exists** — nothing in the smoke, and nothing in
      `installer/README.md`, spells either command as a literal to be kept in step; the assertion's
      operand is what `init` printed.
- [ ] **The vacuous pass is closed** — an empty or unreadable block reds, and the arm rides the
      `init` invocation that actually prints one.
- [ ] **Both halves of "resolves" are asserted** — the script path and the flags, since a renamed
      flag behind a surviving path is the live case.
- [ ] **The verdict classes match the file's existing line** — a payload finding at 1, a harness
      precondition at 2, neither invented here.
- [ ] **Battery and suite green** — `bash gate-sdk/bin/run-gates.sh` plus
      `bash gate-sdk/bin/build-native.sh`, neither discharging the other, and the
      `installer_smoke` suite clean on this host, where the new arm does run on every profile.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical
      section, not appended.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls installer/SPEC-*.md`), discharged at the iteration since a sibling amendment is in
      flight for the same component.
- [ ] **Removals propagated** — grepped for the retired paraphrase of the command pair; nothing
      dangles.
- [ ] **Gaps filed** — cross-component gaps found during the work filed to the gap inbox.
