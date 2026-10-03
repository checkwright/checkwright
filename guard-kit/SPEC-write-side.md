# SPEC amendment: write-side

**The ruleset gains a write ladder, and its new rung is the `bounded_write` rule: a shell write whose every target lies inside a scratch dir, is gitignored and holds no tracked path is granted silently.** Today a write that no rule steers or grants goes to the harness's out-of-band decision. In auto mode that is a classifier with occasional false positives, and otherwise a prompt. Only the scratch grants (rules `truncate_scratch` and `append_scratch`, with rule `bounded_wait`'s arm (B)) and the `--rewrite` arm take a write off that path. The read side already has its ladder, which ends in rule `ro_pipeline`'s *Why admit a cleared program*. This amendment gives the write side the same order, with a **bounds test** where the read side has a side-effect test.

**Measured at authoring**, over the 211 lines `.workflow/prompt-friction.log` held at this stage's entry (`head -n 211`, every one a fall-through):

- **The redirect is the measured class.** Sixteen lines redirect a command's output to a file, by `grep -nE '> *\.tmp/'`, `'> */tmp/'` and `'> *docs/'` over those lines. Nine name a target under `.tmp/`: the front end's battery run, `git diff` or `git show` output, and a board extract piped through `awk` (lines 17, 56, 85, 172, 174, 179, 199, 203 and 204). Four write `/tmp/…`, outside the checkout. Three write a tracked projection under `docs/`. Two lines also carry a `mkdir -p` (56 and 63).
- **The utilities named in the entry** (`rm`, `mv`, `cp`, `tee` and `find -delete`) occur in none of those lines. The log covering their report was drained at the close that filed the entry, so their frequency is unmeasured. The grant covers them on the entry's naming, and delta 2's roster is a knob, so a consumer that measures otherwise sets the roster.
- **The bound the grants share asks git.** Rules `truncate_scratch`, `append_scratch`, `bounded_wait`'s arm (B2) and `emitter_write` test a target with `git check-ignore --quiet --`, through the one function `Host::ignored`. `git ls-files --error-unmatch -- <dir>` exits 0 for a directory holding a tracked file (`guard-kit`) and 1 for one holding none (`.tmp`). `git check-ignore` answers a path that does not exist yet (`.tmp/nonexistent/x` is ignored) and folds `..` itself (`.tmp/../guard-kit/SPEC.md` is not ignored).

## What changes

### (1) The bounds test {design-bearing} {user-facing: operator direction 2026-10-03, lead-relayed (not a ruling): a write lands strictly under a relative GUARD_KIT_SCRATCH_DIRS member, gitignored, with no tracked path at or under it, no symbolic link on its path, and no live record under a removed target}

A **target word** is a write operand or a redirect target, read as rule `grant_path_slot` reads a word: the dequoted word aligned with the `sq dq hd` skeleton, with its backslashes removed. The test withholds wherever that alignment fails. A target is **bounded** when every clause holds:

- **(a) Relative and contained.** The word is not rooted by gate-sdk's `gate_path_rooted` (`walk::path_root`), does not begin with `~`, and carries no `..` and no `.git` component.
- **(b) Strictly inside a scratch dir.** The word lies strictly under a relative `GUARD_KIT_SCRATCH_DIRS` member, compared lexically with the member read as written. The member itself is no target, so neither `rm -rf .tmp` nor `mkdir -p .tmp` is bounded. An absolute member bounds no target, since comparing a relative word with it would compose the working directory into a root.
- **(c) Gitignored.** `Host::ignored` answers yes, the shared test rule `worktree_confinement` states. A glob character (`*`, `?`, `[`) is admitted only in the word's last component. Clauses (b) to (e) then read its parent directory, and the parent may be the member itself, since every path the glob matches lies strictly under it.
- **(d) No tracked path at or under it.** `Host::tracked` (`git ls-files --error-unmatch --`) answers no. A tracked file force-added under an ignored directory is therefore never removed by the grant.
- **(e) No symbolic link on its path.** No existing component of the word, the word included, is a symbolic link (`std::fs::symlink_metadata`, relative to the working directory). The lexical clauses stop at a link, and a link inside a scratch dir can point anywhere.
- **(f) No live producer record at or under a removed target.** No `.run` record whose PID is alive, read as rule `git_mutation_under_producer` reads the scratch homes' records, lies at or under it, compared lexically. Removing a live record would orphan the producer that rule reads.

The test is one function in `native/src/guard/rules/`, and the `bounded_write` rule calls it. It composes no root: every word it passes to git or to the filesystem is the relative word as written, against the working directory. The one root it meets is the main checkout's, which `Host::ignored` and `Host::live_run_records` already resolve from a linked worktree. So it adds no crossing to gate-sdk/SPEC.md §The path-dialect contract, and the decision table on `crate-tests-windows` exercises it at the iteration's closing push like every other row.

### (2) The `bounded_write` rule {design-bearing} {user-facing: operator direction 2026-10-03, lead-relayed (not a ruling): both kinds, a redirect to a bounded target and the GUARD_KIT_WRITE_BINS roster with a per-binary operand grammar, its empty value turning that kind off}

A new item in §The rule roster, its crate function in `native/src/guard/rules/grants.rs` and its row in the rule table. The row reads: `bounded_write`, shells `bash`, views `raw`, `sq hdq`, `sq dq hd` and `dequoted`. **Not yet applied.**

> - **Auto-allow a bounded write** (`bounded_write`) — Declares `raw`, `sq hdq`, `sq dq hd` and `dequoted`. Granted silently when **every** clause below holds, the call falling through untouched otherwise. Each segment of the compound split is one of three kinds:
>   - a **read**: it passes rule `ro_pipeline`'s segment test, banners included;
>   - a **granted command**: its words, with their redirects removed and read as rule `grant_path_slot` reads them, match a committed allow pattern through the allow match, on the settings read's fail-open read;
>   - a **write utility**: its command word is a `GUARD_KIT_WRITE_BINS` member, invoked only in the forms the member's operand grammar below reads, and every target it names passes the bounds test.
>
>   The clauses:
>   - **(0) Not a backgrounded launch.** A statement-ending `&` refuses first. A backgrounded write is rule `background_no_record`'s subject, as it is rule `append_scratch`'s clause (0).
>   - **(a) Every segment is one of the three kinds**, and at least one segment writes. A write is a write-utility segment, or a redirect whose target is neither `/dev/null` nor an fd-dup. A call with no write is rule `ro_pipeline`'s or the allowlist's, so the two rules never grant the same call.
>   - **(b) Every redirect target is inert or bounded.** `/dev/null` and an fd-dup are inert, on rule `append_scratch`'s carve-out. Every other target passes the bounds test. An input redirect is a read.
>   - **(c) The declared-forms reader clears the call.** Rule `ro_pipeline`'s reader runs once over the whole call, so a read segment's write forms withhold here as there.
>   - **(d) Conservative decline on anything unmodelled.** This covers a command or process substitution or a backtick anywhere in the raw command, a live `$` on the `sq hdq` view, a heredoc-bearing command, and an `xargs` segment that runs a write utility, whose operands the segment does not show.
>   - **(e) Not a call a later block refuses.** Rules `worktree_confinement`, `grant_path_slot`, `rm_tracked` and `script_interpreter` each have their test taken as a predicate, on rule `bounded_wait`'s arm (B) pattern. So the grant reaches nothing a later rule would block.
>
>   **The operand grammar** is a crate literal per member, tool mechanism of the same class as rule `ro_pipeline`'s declared-forms table. A member the table does not carry is withheld. An option outside the member's set withholds. A long option matches its full name or an unambiguous prefix of it. A short cluster carries only no-argument letters, with a valued letter last taking the rest of the word or the next word. `--` ends the options. Every target, **created**, **written** or **removed**, takes the bounds test, and a removed target also takes its live-record clause.
>   - `mkdir`: `-p -v -m --parents --verbose --mode`; every operand created.
>   - `touch`: `-a -c -h -m -d -t -r --no-create --date --reference`; every operand written, and a `-r`/`--reference` value read.
>   - `rm`: `-f -r -R -d -v --force --recursive --dir --verbose`; every operand removed.
>   - `rmdir`: `-v --verbose --ignore-fail-on-non-empty`; every operand removed. `-p` and `--parents` withhold, since they remove the ancestors too.
>   - `mv`: `-f -n -v -T -u -t --force --no-clobber --verbose --no-target-directory --update --target-directory`; every source removed and the destination written.
>   - `cp`: `-r -R -a -p -P -f -n -v -T -u -t --recursive --archive --no-dereference --force --no-clobber --verbose --no-target-directory --update --target-directory --preserve --parents`; the destination written, every source a read.
>   - `tee`: `-a -i --append --ignore-interrupts`; every operand written.
>   - `find`: the start points before the first expression word are removed targets, and the expression must carry `-delete` and none of rule `ro_pipeline`'s declared write and execute forms for `find` besides it. A leading `-H` or `-P` is admitted. `-L`, which follows links, withholds.
>
>   **The write ladder.** A write the ruleset takes off the out-of-band path takes the read ladder's order (rule `ro_pipeline`, *Why admit a cleared program*), with a bounds test in place of the side-effect test:
>   - **A standard spelling with its own grant or tool is steered to.** A tracked deletion goes to `git rm -q` (rule `rm_tracked`). A redirect to a tracked target goes to the Write or Edit tool (rule `emitter_write`). An in-place rewrite goes to `--rewrite` (rule `sed_file`).
>   - **A bounded write is admitted here.**
>   - **A residue with neither gets a write arm of the gate binary.** `--rewrite` is that arm for an edit a fixed replacement expresses.
>
>   A shell write to the tracked tree is never granted, since the harness's write tools are its reviewable route.
>
>   **Why the grant is safe.** A `permissionDecision: allow` blesses the whole call, so the safety argument is per segment, as it is for rule `append_scratch`. A read segment is rule `ro_pipeline`'s grant, and a granted-command segment is one the harness would grant issued alone. A write reaches only a path a scratch dir holds that git ignores, holds nothing git tracks, and reaches through no link. Rule `truncate_scratch` already grants `: > <that same path>`, so destroying a scratch file's content is a door already open. This rung widens the operations through it: a directory removal, a move, a copy, a redirect from a granted command. It adds no path the door did not reach.
>
>   **Placed after rule `bounded_wait` and before rule `allowlist_chain`.** Rule `allowlist_chain` blocks an allowlisted lead decorated by a redirect, which is the granted-command kind with a bounded target, so this rule must decide first. Rule `bounded_wait`'s arm (B) writes its record through an emitter and a `&`, which clause (0) refuses here. Rule `append_scratch` already grants an emitter's write to a gitignored target ahead of this rule, so the two never compete for a call.
>
>   **Honest limits.** The operand grammar reads GNU's and POSIX's spellings, and a member that gains a write option is a hole until its row is narrowed, which is rule `ro_pipeline`'s limit. A granted command's own writes are the allowlist's, not this rule's. Clause (e) of the bounds test reads the filesystem at the call, so a link created between the call and its run is not seen.

### (3) The roster knob `GUARD_KIT_WRITE_BINS` {design-bearing} {user-facing: operator direction 2026-10-03, lead-relayed (not a ruling): both kinds, a redirect to a bounded target and the GUARD_KIT_WRITE_BINS roster with a per-binary operand grammar, its empty value turning that kind off}

An indexed knob in `native/src/knobs/guard_kit.rs`, default `(mkdir touch rm rmdir mv cp tee find)`, read by `native/src/guard/host.rs` into a `write_bins` field beside `append_bins` and by the `bounded_write` rule alone. **Not yet applied.** §Layout and configuration gains, after `GUARD_KIT_APPEND_BINS`:

> - `GUARD_KIT_WRITE_BINS` — the write utilities the `bounded_write` rule admits; default `(mkdir touch rm rmdir mv cp tee find)`. A member takes the kit's operand grammar for its name, and one the grammar does not carry is withheld, so the roster narrows the grant and never widens it past the crate's table. Empty turns the write-utility kind off and leaves the redirect kind standing. Eight POSIX utility names are shipped mechanism, so defaulting them crosses no seam, on `GUARD_KIT_APPEND_BINS`' ground.

`guard-kit/templates/guard-config.knobs` gains the commented example `# GUARD_KIT_WRITE_BINS[] = mkdir`, under a comment line saying an empty roster turns the utility grants off.

### (4) The grant rosters name the new rule {mechanical}

guard-kit/SPEC.md. **Not yet applied.** Each list below gains the `bounded_write` rule, in dispatch order:

- §The recommended allowlist, *Deliberately absent*: the rules granting a write to a gitignored target from the hook.
- §The generic ruleset: the grants and rewrites a PowerShell command never meets.
- Rule `allowlist_chain`'s and rule `git_rewrite`'s placement sentences: the auto-allow rules they follow.
- Rule `grant_path_slot`'s placement paragraph, *No auto-allow above it grants a subject of this rule*: rule `bounded_wait`'s arm (B) and the `bounded_write` rule apply this rule's test as a predicate.
- Rule `worktree_confinement`: the non-block rules taking its test as a predicate, and the rules sharing the ignored-target test. That sentence's maintained count goes, so *the five sites' one spelling* becomes *every site's one spelling*.
- §Layout and configuration, `GUARD_KIT_SCRATCH_DIRS`: the `bounded_write` rule reads the member as written, as rule `scratch_redirect` does.

### (5) Decision-table cases {mechanical}

`guard-kit/guard-tests/cases.tsv` gains a section for the `bounded_write` rule after rule `bounded_wait`'s. The sandbox ignores `.tmp/` and `scratch.txt`, tracks `tracked.md`, and allowlists `git status` and `ls` among others (§Testing). Rows, each refusing row one clause:

- `allow	mkdir -p .tmp/out`, `allow	touch .tmp/a.txt`, `allow	rm -rf .tmp/old`, `allow	rm -f .tmp/*.log`, `allow	mv .tmp/a.txt .tmp/b.txt`, `allow	cp tracked.md .tmp/copy.md`, `allow	grep -n x tracked.md | tee .tmp/hits.txt`, `allow	find .tmp/old -name '*.txt' -delete`: the write-utility kind.
- `allow	git status > .tmp/status.txt 2>&1`: a granted command redirected to a bounded target, which rule `allowlist_chain` blocks today.
- `allow	mkdir -p .tmp/d; git status > .tmp/d/s.txt; wc -l .tmp/d/s.txt`: a compound of the three kinds.
- `fallthrough	rm -rf .tmp`, `fallthrough	mkdir -p .tmp`: a target that is the member itself (clause (b)).
- `fallthrough	rm -f scratch.txt`: gitignored, outside every scratch dir (clause (b)).
- `fallthrough	touch .tmp/../x.txt`, `fallthrough	touch /tmp/x.txt`: clause (a).
- `fallthrough	cp .tmp/a.txt tracked.md`: a tracked destination (clauses (c) and (d)).
- `allow	rm -f .tmp/dead-producer.run`: the sandbox's record names a dead PID, so clause (f) passes. Its live twin is a case in `gate-tests/git-mutation-under-producer.test.sh`, which owns a live process (§Testing): `rm -f` of the live record falls through there.
- `fallthrough	rmdir -p .tmp/a/b`, `fallthrough	rm --no-preserve-root -rf .tmp/x`, `fallthrough	find -L .tmp -delete`: the operand grammar.
- `fallthrough	find .tmp | xargs rm`: clause (d).
- `block	rm .tmp/x &`: clause (0). Rule `background_no_record` blocks it first, and the row pins that order.
- `fallthrough	make build > .tmp/b.log`: a command neither read nor granted (clause (a)).

`gate-tests/guard-config-knobs.test.sh` gains the knob's cases. Under an empty `GUARD_KIT_WRITE_BINS`, `rm -rf .tmp/old` falls through while `git status > .tmp/s.txt` is still granted. A roster naming `shred` withholds it, since the grammar has no row for it. Clause (e) of the bounds test is pinned by that function's own unit tests on a temporary directory, which skip where the host cannot create a link. The table's sandbox carries no link, and its Windows leg could not make one.

**Inferred, cannot run before build:** every row's verdict above — the rule and its predicates are delta 2's, which build lands; a row the table answers differently takes its cause into its comment, and a pre-existing row this rule flips is re-derived under §Testing's non-monotone rule, never assumed.

### (6) The site mirror follows {mechanical}

`docs/guard-kit/SPEC.md` is regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commit landing deltas 2 to 4.

## Producers and consumers

- **The bounds test.** Producer: the function delta 1 names, called once per target word. Consumer: the `bounded_write` rule's write-utility kind and clause (b), by direct call. compound-read-classifier-reach's redirect half is this clause, granted here rather than in rule `ro_pipeline` (SPEC-compound-read.md). No field leaves it but its yes or no.
- **The `bounded_write` rule's verdict.** Producer: the rule table row, reached on every bash call the shell guard reads, wherever the member is wired. This repo wires it in `.claude/settings.json`. Consumers: the harness, through the hook's `permissionDecision: allow` envelope. The friction log loses the granted call, which is the measured target. Roster-holding readers of the new name: `check-guard-registration` (roster against table, assertions A to D), which reads it from the roster and the table both, so the two land in one commit. The citation corpus delta 4 rewrites resolves against it.
- **`GUARD_KIT_WRITE_BINS`.** Producer: the knob table, with the consumer's knob file or the environment over it. Consumers: `Host`, read once per hook process, and the `bounded_write` rule. Roster-holding readers: `--emit knob-roster`, which derives from the table, and `check-knob-citation`, which reds a SPEC citation of a knob the tables lack, so the table row lands with or before §Layout's line.
- **No new event or interface.** The PowerShell reader meets no grant here, by §The generic ruleset's ruling on grants.

## Existing sections updated

Roster by `grep -n "truncate_scratch\`, \`append_scratch\`\|five sites\|APPEND_BINS\|Deliberately absent\|scope of rules" guard-kit/SPEC.md` and `grep -n "APPEND_BINS\|append_bins" native/src/knobs/guard_kit.rs native/src/guard/host.rs guard-kit/templates/guard-config.knobs`, over the tracked tree.

- `native/src/guard/rules/grants.rs` and `native/src/guard/rules/mod.rs`: the bounds test, the rule and its table row (deltas 1 and 2).
- `native/src/knobs/guard_kit.rs`, `native/src/guard/host.rs`: the knob (delta 3).
- `guard-kit/templates/guard-config.knobs`: the commented example (delta 3).
- `guard-kit/SPEC.md`: §The rule roster (delta 2), §Layout and configuration (delta 3), and the grant rosters (delta 4).
- `guard-kit/guard-tests/cases.tsv`, `guard-kit/gate-tests/guard-config-knobs.test.sh`, `guard-kit/gate-tests/git-mutation-under-producer.test.sh` (delta 5).
- `docs/guard-kit/SPEC.md`: the regenerated mirror (delta 6).

## Retired spellings

- None — the change adds a rule and a knob; it retires no name.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
