# SPEC amendment: lane-reach

The variant lane holds a bash row to its re-spellings, where the row carries no quote. Three things stay outside it: a grant that survives a write added to the command, a row carrying a quote, and every PowerShell and consumer row. This amendment brings the four into the lane and states what no lane reaches.

It pairs two entries. `guard-drain-fix-unreviewed` left the choice among three forms to this stage: the lane form is taken, in deltas 1 and 2, and the two forms that reorder a closing stage are not, which delta 6 states as the limit. `variant-lane-underived-tables` is deltas 3 and 4.

Three prototypes sized it, each an out-of-crate deriver whose table was fed to the arm as a positional, so each roster below is an estimate and delta 7 has the build run the crate's own.

- A redirect onto the sandbox's tracked file, added to every row of the two bash tables in six spellings: of the derived rows, seventeen were granted. Twelve are the glued second redirect behind a scratch target that `scratch-grant-glued-redirect` owns. Five are rule `abs_script`'s rewrite carrying the redirect, blank-parted or glued, filed in the gap inbox at this authoring.
- A PowerShell set over `powershell-cases.tsv`: of 371 derived rows, 50 moved from `block` or `advise` to `fallthrough`.
- A count of the bash rows a quote-aware eligibility adds: 78 of `cases.tsv`'s rows carry a quote and no other excluded mark, beside the 298 eligible now.

## What changes

### (1) The write-added transforms and their relation

The lane adds a redirect onto a tracked file to each eligible bash row and fails where the guard grants the result {design-bearing} {user-facing: `guard-drain-fix-unreviewed`'s Deliverable, the lane deriving a doubled redirect — the arm's exit 1 gains the finding and its summary line counts the rows}.

- **The transforms**, each derived once per eligible row of `cases.tsv` and `background-cases.tsv`, the text appended to the row's cell:
  - `added-write`, a blank and `>tracked.md`;
  - `added-write-glued`, `>tracked.md`;
  - `added-append-glued`, `>>tracked.md`;
  - `added-dup-glued`, `>&tracked.md`.
- **The target is the sandbox's tracked file**, precondition 3's `tracked.md`, so the write lands in the tracked tree whatever the row's own targets are.
- **A row is eligible** where its cell carries no `#`, `<<` or `@NL@` and its quotes balance, and its last character is a plain character or closes a quoted span. Each exclusion is a cell whose end is no word a redirect can follow: a comment swallows the redirect, a heredoc body takes it as text, and a trailing operator makes it a statement of its own.
- **The relation is the family's own and reads no base verdict.** The derived row is never granted: `allow` or `rewrite` fails, and `block`, `advise` and `fallthrough` hold. `exit<rc>` and `unknown` fail with no limit admitting them, as they do in the lane's first relation.
- **Why it is sound for every row.** A grant blesses the whole call (§The generic ruleset, the shared directions), and the added redirect makes the call a write to a tracked file whatever the row was. No rule grants that write, so a grant on the derived row is a grant that did not bound its call.
- **Why every row and not the grant rows alone.** A row the guard does not grant can gain a grant through the added redirect as a grant row can keep one. The prototype met none, and the added spawns are a fraction of the lane's.
- **A limit admits a failing row only at the `allow` floor**, since the one failing class is a grant. A `fallthrough`-floor row naming one of these transforms is therefore either stale or below its floor, and fails either way.
- **It is no re-spelling**, and the lane's passage says so: the family changes the command on purpose. A derived row equal to an authored row of the same table and flag is skipped, as a re-spelling is.
- **It reaches a scratch grant only through a row whose target sits under an ignored directory.** A glued redirect is read into the target word, and `scratch.txt>tracked.md` is ignored by no rule of the sandbox's, where `.tmp/a>tracked.md` is. Rule `append_scratch`'s section has such rows. Rule `truncate_scratch`'s has none, by `grep -n ': >' guard-kit/guard-tests/cases.tsv`, so delta 7 authors one.
- **The PowerShell tables and the consumer lane take none of it.** No call the PowerShell tool makes is granted by the guard (§The generic ruleset), so the relation is vacuous there. The consumer lane runs at the consumer's live root, where the kit can name no tracked file.

Replacement for §Testing's variant-lane paragraph, its opening sentence through *It runs after the kit's tables and ahead of the consumer lane, on every run and under no knob, since it reads the kit's tables against the kit's rules and is no consumer calibration.* **Not yet applied.**

> **The variant lane** holds each row of the kit's four tables to the spellings its shell reads as the same command, and each bash row to the grant it must not keep under an added write. It runs after the kit's tables and ahead of the consumer lane, on every run and under no knob, since it reads the kit's tables against the kit's rules and is no consumer calibration.

New §Testing passage, placed after the transform list and the `command` paragraph. **Not yet applied.**

> **The write-added transforms change the command, and their relation is their own.** For every bash row whose cell carries no `#`, `<<` or `@NL@`, balances its quotes and ends in a plain character or a closing quote, the lane appends a redirect onto the sandbox's tracked file: `added-write` behind a blank, and `added-write-glued`, `added-append-glued` and `added-dup-glued` glued, as `>`, `>>` and `>&`. The derived row is never granted, whatever its row's verdict: a grant blesses the whole call, and the call now writes a tracked file. A limit admits one only at the `allow` floor. A scratch grant is reached only through a row whose target sits under an ignored directory, since a glued redirect reads into the target word.

### (2) Rows carrying a quote are derived

The deriver reads a quoted span as part of its token, so a row carrying a quote is eligible {design-bearing}.

- **An eligible row** is one whose cell, before substitution, carries no backslash, `$`, backtick, `#`, parenthesis, brace, `[[`, `<<` or `@NL@`, and whose single and double quotes balance. With those marks excluded a quoted span holds no escape and no expansion, so the lexer needs no rule for either.
- **A span** runs from an opening `'` or `"` to the next occurrence of the same character. A blank or an operator character inside a span is text: it ends no token, and no spacing transform reads it.
- **A token carrying a quote is no site.** The site rule stands as written: the head is the token's leading run of plain characters, and a quote following it is neither a separator nor a redirect.
- **Every transform of the lane applies outside a span**, the quote forms at the row's plain sites, the spacing forms at an operator outside every span, `wrap-command` where the first token is a `lead` site, and delta 1's family at the cell's end.
- **The deriver stays a lexer of its own.** It gains a span rule and shares no function with the bash reader.

**Inferred, cannot run before build:** which pairs of section and transform fail once the 78 rows are derived — the crate's deriver does not read a span until this delta lands, and no prototype lexed one.

Replacement for the eligibility sentence of §Testing's deriver paragraph, *A row is eligible where its cell, before substitution, carries no quote, backslash, …, each of which makes a blank-split token something other than a word.* **Not yet applied.**

> A row is eligible where its cell, before substitution, carries no backslash, `$`, backtick, `#`, parenthesis, brace, `[[`, `<<` or `@NL@` and its quotes balance. A quoted span is part of its token: a blank or an operator inside it is text, and a token carrying a quote is no site.

### (3) The PowerShell transform set

The lane derives re-spellings of the two PowerShell tables' rows and holds each to its row by the lane's first relation {design-bearing} {user-facing: `variant-lane-underived-tables`' Deliverable, a PowerShell transform set over the two PowerShell tables — the arm's exit 1 gains the finding and its summary line counts the rows}.

- **The subject is the member, fed as the row is**: a `PowerShell` payload, the same sandbox, the row's backgrounding flag where its table carries one, and the two substitutions applied to the derived cell.
- **The deriver is a second lexer of its own**, sharing no function with the PowerShell reader, on the ground the bash deriver states.
- **An eligible row** is one whose cell, before substitution, carries no quote, backtick, `$`, `#`, `@`, parenthesis, brace or `<`.
- **A site** is a blank-split token made only of letters, digits and `_ . / \ : -`, at least two characters long and not led by `-`, which is a parameter. It is a `lead` where its token is the row's first or follows a token that is or ends in `;`, `|`, `&&` or `||`, and an `arg` elsewhere. A word after the call operator `&` or the dot-source operator is an `arg`, since that is the position PowerShell takes a quoted command in.
- **The transforms**, each derived once per site:
  - `ps-sq-arg` and `ps-dq-arg` quote an `arg` site. A `lead` site takes neither: a quoted word in command position is a string and no command.
  - `ps-tick-lead` and `ps-tick-arg` put a backtick ahead of one character of the site, the first at or after half its length, rounded down, that is a letter other than `a`, `b`, `e`, `f`, `n`, `r`, `t`, `u` or `v` in either case. A backtick ahead of any other letter resolves to the letter, and ahead of one of those, or of `0`, it spells a control character. A site with no such character takes neither.
  - `ps-case-lead` upper-cases a `lead` site carrying a `-`, a cmdlet's verb-noun name, which every host matches without regard to case. A lead with no `-` may be a native program, whose name a case-sensitive host matches exactly.
  - `ps-alias-lead` replaces a `lead` site that is a cmdlet name with each of that cmdlet's built-in aliases, one derived row per alias.
- **The alias table is the deriver's**, a cmdlet name against its built-in aliases, an alias only a Windows host defines included. The relation is one-sided, so an alias a host does not define costs nothing: the guard is asked to be at least as strict on it. The deriver's crate test reads the two PowerShell tables and fails on a verb-noun `lead` the alias table carries no entry for, an entry with no alias being a legal one, so a new row cannot arrive underived.
- **The relation, the skip of an authored row, the mismatch line and the limits table are the lane's**, unchanged. The mismatch line carries the `[PowerShell]` label the kit lane's does.
- **A section is a section of any of the four tables.** The limits table's `<rule>` names a section across all four, and the transform's name tells the shell: a `ps-` transform is derived from a PowerShell row alone.

**Inferred, cannot run before build:** that a PowerShell host reads each derived row as its base row's command — the authoring machine carries no PowerShell host (`command -v pwsh powershell` printed nothing), and the claim rests on the language's quoting and escape rules and on the tables' own authored rows, which spell a backtick inside an argument and a quoted argument.

New §Testing passage, placed after the write-added passage of delta 1. **Not yet applied.**

> **The PowerShell rows are derived by a second lexer**, sharing no function with the PowerShell reader. A row is eligible where its cell carries no quote, backtick, `$`, `#`, `@`, parenthesis, brace or `<`. A site is a blank-split token of letters, digits and `_ . / \ : -`, at least two characters long and led by no `-`, a `lead` where it opens a statement and an `arg` elsewhere, the word after a call or dot-source operator included. The transforms, each held to the lane's first relation:
>
> - `ps-sq-arg` and `ps-dq-arg` quote an `arg` site, a quoted word in command position being no command;
> - `ps-tick-lead` and `ps-tick-arg` put a backtick ahead of a letter of the site that spells no control character behind one;
> - `ps-case-lead` upper-cases a verb-noun `lead`, the one name every host matches without regard to case;
> - `ps-alias-lead` replaces a cmdlet `lead` with each of its built-in aliases, from the deriver's own table, which its crate test holds to every verb-noun lead the tables carry.

### (4) The consumer lane's derivation is a knob

A consumer names a limits table to have its own rows derived, and with none named the consumer lane runs as it does {design-bearing} {user-facing: `variant-lane-underived-tables`' Deliverable, the consumer lane's derivation as a consumer-selectable knob with off among its values — a new knob name, and a new exit 1 and exit 2 for a consumer that sets it}.

- **`GUARD_KIT_CONSUMER_VARIANT_LIMITS`**, a scalar holding a repository-relative path to a table in `variant-limits.tsv`'s grammar; default empty, which derives nothing. Its one reader is the arm's consumer lane.
- **Set**, the lane derives the bash re-spellings of each row of the table `GUARD_KIT_CONSUMER_CASES` names, by delta 2's eligibility and the quote, spacing and `wrap-command` transforms. Each is fed to the consumer's command as its row is fed, held to the lane's first relation, and green on a failure only under a row of the named limits table, which is held from both sides as the kit's is.
- **A section of a consumer table** opens as a section of a kit table does, at a comment line beginning `# rule` and a backticked name. The name is the consumer's own and the kit reads it as an opaque key. A row under no section takes no limit.
- **Neither delta 1's family nor delta 3's set is derived there**, on the grounds delta 1 gives and because the consumer lane feeds `Bash` payloads alone.
- **Exit 2**, each with a message naming the knob: the knob set while `GUARD_KIT_CONSUMER_CASES` is empty; a path naming no readable file; and a table the lane cannot read, on the kit table's three conditions.
- **The summary line** adds the consumer lane's derived-row count and its limit-row count where the knob is set, and nothing where it is empty.
- **Why a path whose empty value is off, and no selector.** A derivation with no limits home is unusable the first time a consumer rule declines a re-spelling, so the choice to derive and the place its limits live are one fact. `GUARD_KIT_CONSUMER_CASES` has the same shape.
- **Why off is the default.** A kit release adding a transform derives new rows from a consumer's table, and each one its rules do not read reds its lane until it adds a limit row. A consumer that sets the knob accepts that; one that does not is never reddened by a rule it did not write, the ground the consumer lane's own subject rests on.
- **The arm's declared roster** gains the knob, read by the arm itself as the two consumer knobs are.

Replacement for §Testing's *The declared roster is …* sentence. **Not yet applied.**

> **The declared roster is `GATE_SDK_KIT_DIRS`, `GUARD_KIT_CONSUMER_RULES_CMD`, `GUARD_KIT_CONSUMER_CASES` and `GUARD_KIT_CONSUMER_VARIANT_LIMITS`.** The arm reads the three consumer knobs itself, since the consumer lane spawns the consumer's command without the member.

New bullet in §Testing's consumer-lane list, after *A table set with no rule command, or naming no readable file, is exit 2.* **Not yet applied.**

> - Where `GUARD_KIT_CONSUMER_VARIANT_LIMITS` names a limits table, the lane derives each row's bash re-spellings and feeds them to the same command, held to the variant lane's relation and to that table from both sides. A section's name is the consumer's own. The knob set with no cases table, or naming a table the lane cannot read, is exit 2. Off is the default because a kit release adding a transform would otherwise red a consumer's lane for a re-spelling its rules never read.

New item in §Layout and configuration's knob list, after `GUARD_KIT_CONSUMER_CASES`. **Not yet applied.**

> - `GUARD_KIT_CONSUMER_VARIANT_LIMITS` — the consumer's own limits table (§Testing, the consumer lane), a repository-relative path in `variant-limits.tsv`'s grammar; default empty, which derives no re-spelling of the consumer's rows. Naming a table turns the derivation on and is where its limits live.

### (5) This repo derives its consumer rows

This repo's knob file names a limits table for its consumer lane {mechanical}.

- `scripts/guard-config.knobs` sets `GUARD_KIT_CONSUMER_VARIANT_LIMITS = scripts/guard-rules-variant-limits.tsv`, under a `spec:` line naming §Testing.
- `scripts/guard-rules-variant-limits.tsv` is new, carrying its header comment and one row per failing pair the lane names.
- `scripts/guard-rules-cases.tsv` carries no section opener, by `grep -c '^# rule' scripts/guard-rules-cases.tsv` printing 0. Where a limit row is needed, the group its rows sit in takes an opener naming the rule in `scripts/guard-rules.sh` they assert.

**Inferred, cannot run before build:** whether any re-spelling of this repo's rows fails the relation — the consumer lane derives nothing until delta 4 lands.

### (6) What no lane reaches

§Testing's limits paragraph names the class the lane's relations admit by construction and whose only instrument is a second reading {design-bearing}.

- **A decline is outside every relation.** A fix that makes the guard block or withhold a spelling it should pass moves a verdict in the direction the first relation admits and delta 1's holds on. A row authored for that spelling is the one thing that pins it.
- **The kit prescribes no order for the reading that finds one.** Two forms were weighed and not taken: a closing stage promoting a verdict-moving fix in place of landing it, and a fix read by a second party ahead of its commit. Each is a consumer's stage order and its spend, which the kit's templates give no slot for and no knob selects, so the paragraph states the limit and leaves the order to the consumer.
- **The PowerShell and consumer clauses of the paragraph go**, deltas 3 and 4 having derived both.

Replacement for §Testing's *Its honest limits.* paragraph. **Not yet applied.**

> **Its honest limits.** No shell runs a derived row, so the equivalence is the construction's and each deriver's own crate tests pin each transform and each exclusion on a literal. A row carrying a backslash or an expansion is underived. A banner put ahead and a keyword standing as a word change the command and are no re-spelling, so their rows are authored, and of the decorations only the added write is derived. The consumer lane is underived until the consumer names a limits table. **No relation reaches a decline**: a change that blocks or withholds a spelling it should pass moves a verdict in the direction every relation admits, so only an authored row or a second reading of the change meets it, and the kit prescribes no order for that reading.

### (7) The first limit rows and the rows the lane needs

The build runs the widened lane over the tables as they stand and writes one limit row per failing pair of section and transform, at the floor the failing rows reach {mechanical}.

- **Rule `truncate_scratch`'s section gains an `allow` row whose target sits under the sandbox's ignored directory**, so delta 1's family reaches that rule. Where `scratch-grant-glued-redirect` has landed and its rows include one, this bullet is satisfied by them.
- **Rules `truncate_scratch` and `append_scratch`, delta 1's glued transforms.** Where `scratch-grant-glued-redirect` lands ahead of this amendment in the batch, the lane meets no grant there and no row is written. Where this amendment lands first, the rows are written at `allow`, and that unit's commit deletes them, the lane failing on a limit row nothing needs.
- **Rule `abs_script`, all four of delta 1's transforms.** The finding is in the gap inbox and is no unit of this iteration. Where a fix for it lands in this iteration, no row is written. Where none does, the four rows are written at `allow`.
- **The sibling debt units land with the lane green.** `cat-operand-after-double-dash` and `sed-program-escaped-dash` each add rows carrying a quote, which delta 2 derives. Whichever of those units and this amendment lands later adds or deletes the limit rows its own rows and reads move.
- **A PowerShell row group standing under another rule's section takes an opener of its own** where a limit must name it. `powershell-cases.tsv`'s reader-grammar rows and its Bash-only rows follow rule `script_interpreter`'s opener with none between, by `grep -n '^# ' guard-kit/guard-tests/powershell-cases.tsv`, so their failures would be declared as that rule's.
- **No rule's read is lifted by this delta.** A failing pair is declared, whatever its rule.
- **The PowerShell prototype's roster is the estimate**: it failed under the sections of rules `cd_compound`, `scratch_redirect`, `git_grep`, `git_rewrite`, `background_no_record`, `commit_only_paths`, `git_stash`, `rm_tracked` and `script_interpreter`. It quoted a word after `;` and `|` and upper-cased a native program, which delta 3 excludes, so the crate's roster is narrower there.

### (8) The surfaces that name the arm and the knob

The arm's describers, its seam test and the knob's rosters take the widened lane {mechanical} {user-facing: the entries' Deliverables, as deltas 1, 3 and 4 mark them — the release declaration states what a consumer meets}.

- `native/src/knobs/guard_kit.rs` gains the knob's row, a scalar defaulting empty.
- `guard-kit/templates/guard-config.knobs` gains the knob as a commented line under a `spec:` line, after `GUARD_KIT_CONSUMER_CASES`.
- `guard-kit/SPEC.md` §Layout and configuration's tree comment on `variant-limits.tsv` names the pairs of rule and transform the lane admits a failure under, where it names re-spellings alone.
- `guard-kit/README.md`'s comment on the arm and its consumer-table sentence name the added write, the PowerShell rows and the knob.
- `guard-kit/gate-tests/consumer-rules.test.sh` gains the consumer derivation's seam cases: the knob empty deriving nothing, a table under which a declining consumer command is green, the same table reddening on a row nothing needs, and the two new exit-2 refusals. Its expected summary lines take the new counts. A section opener it writes into a sandbox table names a roster rule, since `check-guard-registration`'s assertion D reads a bare citation anywhere in this kit's tree.
- `.workflow/release-declarations.md` gains a Knob changes bullet for `GUARD_KIT_CONSUMER_VARIANT_LIMITS` and a Behavior changes bullet: the arm derives an added write on every bash row, re-spellings of rows carrying a quote and of the PowerShell rows; a call naming its own tables names limits for the pairs its rules decline; nothing is owed by a consumer running the arm with no operands and the knob empty.
- The arm's ok line names the four tables' derived rows where it names the bash rows'.

## Producers and consumers

- **The write-added family and the PowerShell set.** Producer: `--run-guard-tests`, on every run, under no knob, wherever the arm runs. Those are, by `git grep -n -l -e 'run-guard-tests' -e 'run_guard_tests'` over the tracked tree less the generated mirror and the queue: both `gates.yml` steps, `EVIDENCE_KIT_RUN_guard_tests` in `scripts/evidence-config.knobs`, the root `README.md`'s battery line and `guard-kit/README.md`. Consumer: the reader of the arm's exit and mismatch lines, which is the build session and the evidence-kit suite, the suite reading the exit alone.
- **The arm's three-valued exit is unchanged in meaning**, and so are its positionals: the limits table stays the fifth. The remaining files that grep names are `native/src/emit/mod.rs`'s arm-table row, `native/src/emit/run_guard_tests.rs`, `native/src/gates/guard_registration.rs`, `gate-sdk/SPEC.md` (the arm's split between itself and its subject, which stands: the spawned programs are the member, `git` and the consumer's argv, as now), `guard-kit/gate-tests/consumer-rules.test.sh`, `guard-kit/templates/guard-config.knobs`, `guard-kit/guard-tests/variant-limits.tsv`, `scripts/guard-rules-cases.tsv`, `.workflow/release-declarations.md`, `.workflow/gap-inbox.md` and a `check-door-binding` fixture page. Only the seam test passes positional tables, and it passes a limits table already.
- **The limits table's grammar is unchanged.** Its transform set grows by delta 1's four names and delta 3's six, and the lane's own validator is the one roster holding it: a name off the set is exit 2. `guard_registration.rs` reads the table for rule citations and meets none in a table of bare names.
- **The knob.** Producer: a consumer's knob file; this repo's sets it (delta 5), so the enabling configuration is deployed and not test-only. Consumer: the arm's consumer lane, at its one read. Roster-holding readers of a guard-kit knob name, each an update target above: the crate's static table, from which `--emit knob-roster` and the knob validator derive; the kit's knob template; §Layout and configuration's list; and the arm's declared roster, which §Testing states.
- **Every field has a reader.** The knob is one scalar, read once. The consumer limits table's three fields are read as the kit table's are.
- **No corpus is narrowed.** Deltas 1 to 4 each widen what the lane derives. Its red conditions are a failing derived row and a limit row nothing needs, and delta 7 has the build meet both before the commit.
- **Every member of the widened corpus has a satisfying value**: a derived row is green on the relation or under a limit row at the floor it reaches, and delta 1 admits a grant at the `allow` floor, so no pair is unsatisfiable. The members are enumerated by the lane's own run and not here.
- **The run's price.** The arm takes nineteen seconds on this tree for 6688 derived rows. Delta 1 adds at most four spawns per bash row, delta 3 a few hundred, and delta 2 the re-spellings of 78 more rows. The arm is no pre-commit gate.

## Existing sections updated

- `guard-kit/SPEC.md` §Testing — the variant-lane paragraph, the write-added passage, the eligibility sentence, the PowerShell passage, the declared roster, the consumer-lane bullet and the limits paragraph (deltas 1, 2, 3, 4 and 6).
- `guard-kit/SPEC.md` §Layout and configuration — the knob's item and the tree comment (deltas 4 and 8).
- `native/src/emit/guard_variants.rs`, `native/src/emit/run_guard_tests.rs` — the two families, the span rule, the PowerShell deriver and the consumer derivation, in those files or a module beside them (deltas 1, 2, 3 and 4).
- `native/src/knobs/guard_kit.rs` — the knob's row (delta 8).
- `guard-kit/guard-tests/variant-limits.tsv`, `guard-kit/guard-tests/cases.tsv`, `guard-kit/guard-tests/powershell-cases.tsv` — the limit rows, the authored row and the openers (delta 7).
- `guard-kit/templates/guard-config.knobs`, `guard-kit/README.md`, `guard-kit/gate-tests/consumer-rules.test.sh` (delta 8).
- `scripts/guard-config.knobs`, `scripts/guard-rules-variant-limits.tsv`, `scripts/guard-rules-cases.tsv` (delta 5).
- `.workflow/release-declarations.md` — the two bullets (delta 8).
- `docs/guard-kit/SPEC.md`, `docs/guard-kit/README.md` — generated, stale once any delta lands (all deltas).

## Retired spellings

- None — the amendment adds transforms, a deriver and a knob and retires no spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`), discharged at the iteration.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entries moved** — both paired entries move to Done in the merging commit, ahead of the drain stage. `guard-drain-fix-unreviewed` carries an unspent grant for a later stage, which that commit's session hands to the lead before the move, a done line keeping the slug alone.
