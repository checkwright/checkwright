# SPEC amendment: disposed-findings

**A finding the gap drain discards gets a durable record, and the capture arm and the filing owner lookup match a new bullet against it.** Today a discard's only record is the drain's commit message. The live-slug advisory and the owner lookup (lifecycle-kit/SPEC.md §The committed gap inbox) search live queue entries alone, so a finding discarded once is never matched when it is filed again. It pairs `disposed-findings-register`.

**The witness, read at authoring.** The ubuntu-latest runner notice was filed to the gap inbox four times (commits `08663d34`, `d9c65019`, `bbad7889`, `7c91b291`, dated 2026-09-27, 09-29, 10-01 and 10-02) and discarded at each next drain on one ground, native/runners.list's pin-or-ride rule. All four bullets spell `ubuntu-latest` (`git show <the four> -- .workflow/gap-inbox.md | grep '^+- ' | grep -i ubuntu`), so one record keyed on that term matches every filing.

**What it records, and what it does not.** Only →discard leaves no other trace. →fix lands a change in its commit, →forward a consult-inbox bullet, and →promote a live entry the slug advisory already matches. The record is written by the session that discards, at the drain, never at capture, so capture stays the cheap two-field append the inbox keeps it.

## What changes

### (1) The disposed-findings record {design-bearing} {user-facing: the operator's direction of 2026-10-02 on the entry, lead-relayed: a finding's disposition recorded at its first processing, matched at capture or filing}

A committed file, `LIFECYCLE_KIT_DISPOSED_FILE`, default `${GATE_SDK_WORKFLOW_DIR:-.workflow}/disposed-findings.txt`. It is a checked projection of the workflow directory (gate-sdk/SPEC.md §The workflow directory), a `.txt` by that section's extension rule, since the arm's matcher and the gate parse it field-wise. Its first line is

```
# contract: lifecycle-kit/SPEC.md §The disposed-findings record — - <YYYY-MM-DD> — `<term>`[, `<term>`]… — <finding and ground>
```

and every later non-blank line is one **record**: `- <YYYY-MM-DD> — <terms> — <prose>`. The date is the discarded bullet's own. `<terms>` is one or more backticked spans joined by `, `, each non-empty and free of backticks: the words a later filing of the same finding would spell, chosen by the discarding session. `<prose>` states the finding and the ground it was discarded on, non-empty.

- **Written at the drain, by hand, in the drain's commit.** A →discard at close's gap drain or at the first stage's carried-bullet intake appends one record, unless the bullet matched a record whose ground still holds, which it cites instead. The record lands in the commit that truncates the inbox, so the judgment and its grounds sit in one diff. The first record seeds the header above, byte for byte.
- **A record leaves only when its ground falls.** A drain whose re-verification shows a matched record's ground no longer holds dispositions the bullet as any other, by fix, forward or promote, and deletes the record in the same commit. No age expiry: a ground that still holds is still the answer.
- **Normal merge semantics.** Two branches each appending a record conflict at the file's end and are resolved by keeping both. It is not a `union` surface, since a record is also deleted, and the union set's `check-merge-attrs` forward edge would red every adopter that registered that gate until its `--install-lifecycle` re-ran.
- **Not a close surface.** Close writes it and reads it as a lookup corpus. It is never an inbound triage surface with a backlog to drain, so it carries no `close-surface:` declaration.
- **It outlives iterations.** The boundary truncates nothing in it.

**Not yet applied.**

### (2) The capture arm matches a bullet against the record {design-bearing} {user-facing: the operator's direction of 2026-10-02 on the entry, lead-relayed, "capture or filing matching a new bullet against it"}

`--emit file-gap` reads the record after appending, and for each record any of whose terms occurs in the bullet's prose raises one stderr advisory, beside the live-slug advisory:

> the prose matches the finding disposed `<date>` (`<term>`): `<prose>`. If this bullet re-files it, say in the prose what has changed since; the drain discards a re-filing whose ground still holds by citing that record.

A term matches case-insensitively and **word-bounded** on `[a-z0-9-]`, the live-slug matcher's boundary, so a term inside a longer hyphenated token raises nothing. Every matching record is named, in file order, since two records are two findings. The advisory writes nothing and refuses nothing: the matcher prompts rather than decides, on the live-slug advisory's own ground. An absent record is no advisory. A record line outside the grammar is skipped by the matcher and left to delta 4's gate. The arm's declared roster gains `LIFECYCLE_KIT_DISPOSED_FILE`, six rows of the kit's static table. **Not yet applied.**

### (3) The drain and the owner lookup read the record {design-bearing}

lifecycle-kit/SPEC.md §The committed gap inbox and the two drain steps change as below. **Not yet applied.**

- **§The committed gap inbox**, the paragraph opening **Filing looks for an owner first.**: its second sentence, *the session searches the queue file, the icebox included,* becomes *the session searches the queue file, the icebox included, and the disposed-findings record (§The disposed-findings record)*, and the list gains a fourth reading: *a record whose ground still holds is a re-filing of a discarded finding, discarded again citing the record's date, with no second record.*
- **§The committed gap inbox**, the affordance paragraph: the arm's declared roster becomes *six rows of the kit's static table*, gaining `LIFECYCLE_KIT_DISPOSED_FILE`. The shape paragraph after it reads *the four advisories below ride **stderr***, the disposed-finding advisory joining the three.
- **§The committed gap inbox** gains a subsection, `### The disposed-findings record`, after the paragraph on the drain's ordered dispositions, so the record's header and the pointers below resolve to it. It states delta 1's surface, grammar, writer, deletion rule, merge semantics and close-surface status, and delta 2's advisory, in the merge's re-phrased form, and the drain's four dispositions name the record at →discard. Its **Producers and consumers** paragraph gains the record: producer, the session running a drain's →discard; consumers, `--emit file-gap`'s advisory, every filing's owner lookup, and `check-disposed-findings`.
- **§Layout and configuration** gains the knob bullet: *`LIFECYCLE_KIT_DISPOSED_FILE` — the committed record of discarded gap findings (§The committed gap inbox); default `${GATE_SDK_WORKFLOW_DIR:-.workflow}/disposed-findings.txt`, written by hand at a drain's →discard, read by the `--emit-file-gap` arm's advisory and asserted by `check-disposed-findings`.*
- **`templates/stages/close.md` step 2**, the →discard clause, becomes: *→discard (state why in the close commit message — the bullet's own prose is the disposition body — and append its record to `LIFECYCLE_KIT_DISPOSED_FILE`, or cite the record it matched whose ground still holds).* The **Re-verify before dispositioning.** passage gains: *A bullet matching a disposed-findings record re-verifies that record's ground; a ground that fell takes the bullet through the ordinary dispositions and deletes the record in this commit.*
- **`templates/stages/scope.md`**'s second step, its *discarded with cause in the commit message* clause, gains *, its record appended to the disposed-findings record or the matching record cited (lifecycle-kit/SPEC.md §The committed gap inbox)*.
- **canon-kit/SPEC.md §Layout and configuration**, the list of surfaces `check-kit-ref-liveness` valves, gains *the disposed-findings record (`LIFECYCLE_KIT_DISPOSED_FILE`, read by its knob), valved as a frozen record: a discarded finding may name a knob or path that was never minted, and a record outlives the iteration that wrote it*, and `native/src/gates/kit_ref_liveness.rs` reads the knob beside the survey record's. The gate scans every tracked file outside its valves, so without this a record of a discarded proposal reds the tree.

### (4) `check-disposed-findings` holds the record's grammar {design-bearing} {user-facing: the operator's direction of 2026-10-02 on the entry, lead-relayed: its contract and a fixture pair; its `on-surface` disposition is the operator's direction of 2026-10-02 in the lead session, lead-relayed (not a ruling)}

A native gate, `lifecycle-kit/checks/check-disposed-findings.gate`, implemented in `native/src/gates/disposed_findings.rs` and owned by lifecycle-kit in `gates::REGISTRY`, declaring `LIFECYCLE_KIT_DISPOSED_FILE`. Its graph manifest couples `knob:LIFECYCLE_KIT_DISPOSED_FILE` at `tier=precommit`.

**Its descriptor carries `install: on-surface`** (gate-sdk/SPEC.md §The install disposition). The record is a surface only a stage session writes, the analogue of the stage attestation that disposition names, so the gate arms when the adopter registers it rather than at install. lifecycle-kit's stance that no gate of its registers at install (installer/SPEC.md §What init seeds) holds unchanged.

Invariant: every non-blank line below the `# contract:` header is a record in delta 1's grammar: a valid date, at least one backticked term with no empty span, and non-empty prose. The matcher skips a malformed record silently, so a malformed record would turn a discard's protection off with nothing printed, and this gate is what tells. The header's presence and form are `check-workflow-tiering`'s.

- **Output**: `DISPOSED-FINDINGS: clean (<n> record(s))`, and per finding the line and what it lacks, with a `help:` naming the grammar.
- **Fail-closed**: exit 2 on an unreadable file or an explicitly named missing one. An absent record at the configured path is clean, since a consumer that never discarded a finding is in a legal state.
- **Bare drives the configured record; an explicit file argument drives it hermetically**, the §check-gap-inbox-neutrality precedent.
- **Fixture pair** `lifecycle-kit/gate-tests/check-disposed-findings/{good,bad}`: the good case two records, one with two terms; the bad case a record with no term, one with an empty span, one with no prose and one with an unparseable date. The module's unit tests pin the absent-record clean line the fail-closed bullet states, which only the bare form reaches, since a named missing file is exit 2.
- **Self-lint**: registration in this repo's `scripts/gates.list`, beside `check-gap-inbox-neutrality`.

lifecycle-kit/SPEC.md gains a `### check-disposed-findings` section in the per-component contracts, stating the above. **Not yet applied.**

### (5) The record opens with the witness {mechanical}

The build seeds `.workflow/disposed-findings.txt` with its header and one record for the runner notice: the date of its first filing, the term `` `ubuntu-latest` ``, and the finding and its ground as the four drains stated them, native/runners.list's pin-or-ride rule. The arm's advisory then fires on a fifth filing, and the gate reads a real file in this repo's battery. **Not yet applied.**

### (6) The affordance's own test pins the advisory {mechanical}

`lifecycle-kit/gate-tests/file-gap-recurrence.test.sh` gains a case: a record whose term occurs in a filed bullet's prose raises the advisory naming the record's date and term on stderr and leaves stdout the bullet alone; a term inside a longer hyphenated token raises nothing; a malformed record raises nothing. The arm module's unit tests pin the word-bounded, case-insensitive match. **Not yet applied.**

## Producers and consumers

- **A record.** Producer: the session running close's gap drain or the first stage's carried-bullet intake, at a →discard, with no enabling config: the knob's default makes the path live wherever lifecycle-kit is vendored. Consumers: the `--emit-file-gap` arm, reading the file after its append (delta 2); every filing's owner lookup, a session grep (delta 3); `check-disposed-findings` (delta 4). Each field's reader: the date, the advisory text and the drain's citation; the terms, the matcher and the lookup's grep; the prose, the advisory text and the drain's re-verification of the ground.
- **The advisory.** Producer: `--emit file-gap`, on a term match. Consumer: the filer, who writes the re-filing claim into the bullet's prose, which the drain reads.
- **A record's deletion.** Producer: a drain whose re-verification shows the ground fell. Consumer: the next capture, which no longer matches.
- **Roster-holding readers of the minted names.** `LIFECYCLE_KIT_DISPOSED_FILE` joins lifecycle-kit's static knob table (`native/src/knobs/lifecycle_kit.rs`) and the file-gap arm's declared roster, `KNOBS` in `native/src/emit/file_gap.rs`, which the arm-table row in `native/src/emit/mod.rs` names by reference and so leaves unchanged. `check-disposed-findings` joins `gates::REGISTRY`, this repo's `scripts/gates.list`, lifecycle-kit/README.md's gate roster and the generated enforcement map; as an `on-surface` member it joins no registry `init` writes. `LIFECYCLE_KIT_DISPOSED_FILE` joins `check-kit-ref-liveness`'s valve. The new tracked `.workflow/` member is read by `check-workflow-tiering`, which its header satisfies, and by the close-surface derivation's tracked-file pass, which reads only gitignored members and declarations, so the file adds no row.
- **Red conditions.** `check-disposed-findings` reds on a malformed record, never on finding none, and an absent record is clean, so an adopter registering it before its first discard reads clean. No existing reader's corpus narrows: `check-kit-ref-liveness`'s valve takes out only the new record, which it never read.

## Existing sections updated

Probe for the reader roster: `git grep -n "LIFECYCLE_KIT_GAP_INBOX_FILE\|Filing looks for an owner\|discarded with cause"` over the tracked tree less `docs/posts`, and the file-gap arm's callers in `native/src`; at align, `git grep -n "LIFECYCLE_KIT_SURVEY_RECORD_FILE\|check-survey-record"` for a sibling record's fan-out and `git grep -n "lifecycle-kit registers\|No lifecycle-kit gate"` for the disposition's.

- `lifecycle-kit/SPEC.md` §The committed gap inbox — the owner lookup, the new subsection, the drain dispositions and the producers-and-consumers paragraph (deltas 1, 2 and 3).
- `lifecycle-kit/SPEC.md` §Layout and configuration — the knob bullet (delta 1).
- `lifecycle-kit/SPEC.md` §check-disposed-findings — new (delta 4).
- `lifecycle-kit/templates/stages/close.md` — step 2's →discard clause and its re-verification passage (delta 3).
- `lifecycle-kit/templates/stages/scope.md` — the second step's discard clause (delta 3).
- `lifecycle-kit/README.md` — the gate roster gains `check-disposed-findings` (delta 4).
- `canon-kit/SPEC.md` §Layout and configuration — the valve list (delta 3).
- `native/src/gates/kit_ref_liveness.rs` — the record's valve (delta 3).
- `lifecycle-kit/checks/check-disposed-findings.gate` and `lifecycle-kit/gate-tests/check-disposed-findings/` — new (delta 4).
- `lifecycle-kit/gate-tests/file-gap-recurrence.test.sh` — the advisory case (delta 6).
- `native/src/knobs/lifecycle_kit.rs` — the knob row (delta 1).
- `native/src/emit/file_gap.rs` — the advisory, the arm's declared roster and its header comment's knob count (delta 2).
- `native/src/gates/disposed_findings.rs` and `native/src/gates/mod.rs` — the gate and its registry row (delta 4).
- `scripts/gates.list` — the registration (delta 4).
- `.workflow/disposed-findings.txt` — seeded (delta 5).
- `.workflow/release-declarations.md` — the new gate, its knob and the advisory, and the two template steps a consumer copying them out re-takes (deltas 2, 3 and 4).
- `docs/enforcement.md` — the generated enforcement map gains the gate (delta 4).
- `docs/check-graph.html` — regenerated for the new descriptor (delta 4).
- `docs/lifecycle-kit/SPEC.md` — the generated mirror (all deltas).
- `docs/canon-kit/SPEC.md` — the generated mirror (delta 3).

## Retired spellings

- None — no delta of this amendment retires a spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — the close and scope template clauses carry no grounds; §The committed gap inbox owns them.
- [ ] **Merged with no information lost** — §The committed gap inbox reads as one section a reader who never saw this file can use.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green over the declaration above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **The entry is done** — `disposed-findings-register` moves to Done in the merging commit, before the drain stage; no remote run is its oracle.
