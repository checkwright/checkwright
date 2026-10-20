# SPEC amendment: grant-literal-shrink

Pairs `guard-grant-literal-shrink` and `bounded-write-link-reach-grant`, whose two wrong grants are cases of this shrink. The shell guard's grants and rewrites (§The generic ruleset: rules `abs_script`, `brace_glyph`, `truncate_scratch`, `append_scratch`, `ro_pipeline`, `bounded_wait` and `bounded_write`) each rest on a model of how bash passes a word and how the harness's matcher reads it. That model has to cover every quoting, escape and glob spelling bash resolves, and a spelling it misses is a blessed call. This amendment takes the model off the grant side: a grant attaches only to a call whose words pass as spelled, and each write grant refuses a target reached through a link or a directory. The steers and blocks keep their by-value reads, where a misread costs a re-issue.

**What the tree does today**, asked at spec through `--guard-verdict` from the toplevel, with `.tmp/spec-l` a symbolic link to the tracked `README.md` and `.tmp/spec-d` a directory, both made for the probe and removed after it:

| command | verdict |
|---|---|
| `: > .tmp/spec-l`, `echo x >> .tmp/spec-l`, `echo x > .tmp/spec-l` | **allow**: rules `truncate_scratch` and `append_scratch` write through the link into a tracked file |
| `cp README.md .tmp/spec-d`, `mv .tmp/spec-x .tmp/spec-d` | **allow**: rule `bounded_write` writes into the directory, at a path its bounds test never read |
| `ls > .tmp/spec-o*` | **allow**: a glob redirect target, read as its parent directory |
| `echo "a b" >> .tmp/j.md`, `grep -rn "foo bar" src`, `until [ -f .tmp/x ]; do sleep 5; done` | allow |
| `touch .tmp/spec-l`, `tee .tmp/spec-l < /dev/null` | fall through |

The first row is a wrong grant of the class `bounded-write-link-reach-grant` filed for rule `bounded_write` alone; rules `truncate_scratch` and `append_scratch` carry it too, with no entry filed. The release bar reads over the grant set this amendment leaves, so delta 2 closes it here.

**The reach, counted on the decision tables** (`cases.tsv` and `background-cases.tsv`, an awk pass over each `# rule` section's `allow` and `rewrite` rows): 162 `allow` and `rewrite` rows, of which 83 carry a quote or a backslash and one more, `rm -f .tmp/*.log`, carries an unquoted glob. By section: `append_scratch` 16 of 17, `ro_pipeline` 29 of 50, `bounded_wait` 5 of 15 (three of them arm (B)'s template rows, which delta 4 keeps granted), `abs_script` 4 of 8, `bounded_write` 7 of 22 plus that glob row, `brace_glyph` 3 of 4; the other 19 sit in other rules' non-firing sections. So 81 rows move, each to `fallthrough`, or to `block` where a later rule or a rewrite's corrective claims it (delta 8). `variant-limits.tsv` carries no grant rule's row, so no limit row goes stale.

**The test's scope and its one exemption are operator directions (2026-10-20, lead-relayed, not rulings).** The literal test reads the whole call, over the full character list delta 1 names, and the operator chose it with the cost above in view. Rule `bounded_wait`'s arm (B) grants the recorded launch whose record statement carries a double-quoted `$!`, and delegation-kit/SPEC.md §The delegation model rests on that grant. So arm (B) stays, its record a byte-exact template and the test's one exemption; retiring it would reach a second component. Pairing `bounded-write-link-reach-grant` here and delta 2's link clause are the lead's decisions of the same date, within the narrowed bar.

**No act here depends on a sibling's landing.** `hook-binding-reader-rows` builds first, by the set record's order, and edits `native/src/guard/host.rs`'s `Bash(` strip and `native/src/guard/mod.rs`'s `:*` idiom. Neither is a site this amendment changes. Rows are named by their text, never their line.

## What changes

### (1) A grant reads only a literal call {design-bearing} {user-facing: guard-grant-literal-shrink's Deliverable, "the grants shrink to literal spellings, falling through on any glob, quoting, backslash, or `cp` or redirect into a directory; the steering and block rules stay", adopted by the set direction of 2026-10-20; the whole-call scope and its character list, operator direction of 2026-10-20}

Replacement for `guard-kit/SPEC.md` §The generic ruleset, the shared direction **A grant bounds the whole call.** **Not yet applied.**

> - **A grant bounds the whole call, and reads only a literal one.** The allow envelope (gate-sdk/SPEC.md §The hook contract) blesses the whole call, so a grant bounds the command rather than recognizing the shape it keys on. It also grants only a **literal** call, one whose every word passes to its program as spelled. The received command carries:
>   - no quote, `'` or `"`, and no backslash;
>   - no `$`, no backtick and no parenthesis, so no expansion, substitution, subshell or process substitution;
>   - no glob character: `*`, `?`, or a `[` other than the test commands `[` and `[[` standing as words;
>   - no word opening `~`;
>   - no heredoc or herestring opener, and no carriage return.
>
>   A call failing the test takes no grant and no rewrite. Each rule of the auto-allow band declines it, and rules `abs_script` and `brace_glyph` meet it with the corrective block their items name. The engine holds the test over every grant verdict, so a rule that omits it still grants nothing outside it. Rule `bounded_wait`'s arm (B) is the one exemption, stated there. **Why literal.** A grant reading quotes, escapes and globs must model every spelling bash resolves, and each spelling the model misses becomes a blessed call. A literal call has one reading, the one its bytes spell, so no grant rests on that model. The steers and blocks keep their by-value reads, since a misread there costs a re-issue rather than a blessed call. A call the test refuses costs one decision on the harness's own path, which reads the call itself.

`native/src/guard/`: **Not yet applied.**

- **One predicate**, `literal_call`, over the command's raw text, in the engine's module or the rules module, with a `spec:` comment naming this section.
- **The engine's backstop.** `engine::decide` treats a rule's `Allow` or `Rewrite` on a call the predicate refuses as a decline and walks on, so a later rule still reads the call. A crate test drives `decide` over a stub rule granting a quoted call and asserts no verdict.
- **Rules `abs_script` and `brace_glyph`** call the predicate themselves, through their `raw` declaration, so a refused call takes the block their items already name. The band rules need no call of their own and no declaration change.

### (2) The scratch writes refuse a target reached through a link {design-bearing} {user-facing: the release bar narrowed by the operator's direction of 2026-10-20 on tarball-attestation-observed, "the zero-wrong-grant bar now reads over the grant set left after guard-grant-literal-shrink"}

Replacement for `guard-kit/SPEC.md` §The rule roster, rule `truncate_scratch`'s sentence **Granted silently when the command is *only* `:` followed by redirects and every target is gitignored, after rule `worktree_confinement`'s test as a predicate: truncating scratch is safe, and a tracked file still takes the decision.** **Not yet applied.**

> Granted silently when the command is *only* `:` followed by redirects, every target is gitignored, and no target reaches through a link, after rule `worktree_confinement`'s test as a predicate: truncating scratch is safe, and a tracked file still takes the decision. **A target reaches through a link** when an existing component of its path, the target included, is a symbolic link, rule `bounded_write`'s bounds clause (e). A link in an ignored directory can name a tracked file, and git's ignored test reads the path, not where the write lands.

Replacement for rule `append_scratch`'s clause (b), its first sentence. **Not yet applied.**

> - **(b) Every target gitignored and reached through no link**, on rule `truncate_scratch`'s exact `git check-ignore --quiet --` predicate and subprocess and its link test, reached only once (a) and (c) have matched.

Rule `append_scratch`'s herestring sentence, from **A herestring takes no grant** to **Spell the write as a heredoc.**, reads: "**A heredoc and a herestring take no grant**, the literal test refusing each opener (§The generic ruleset). A write whose text needs either reaches the harness's own path or its write tool. The residue reading above still serves rule `emitter_write`'s statements." **Not yet applied.**

`native/src/guard/rules/grants.rs`: both rules call the link test bounds clause (e) already calls, one function, on every target they test. **Not yet applied.**

### (3) A write into a directory, or through a glob, takes no bounded grant {design-bearing} {user-facing: guard-grant-literal-shrink's Deliverable, as delta 1}

Replacement for `guard-kit/SPEC.md` rule `bounded_write`, the bounds test's opening paragraph, from **The bounds test** to **A target is **bounded** when every clause holds:**. **Not yet applied.**

> **The bounds test** reads a target word, a write operand or a redirect target, as written, which on a literal call is the word the shell passes. A target is **bounded** when every clause holds:

Its clause (c) reads: "**(c) Gitignored**, on the ignored-target test rule `worktree_confinement` states." The glob sentence leaves it. **Not yet applied.**

The operand grammar's `mv` and `cp` rows read: **Not yet applied.**

> - `mv`: `-f -n -v -T -u --force --no-clobber --verbose --no-target-directory --update`; every source removed and the destination written.
> - `cp`: `-r -R -a -p -P -f -n -v -T -u --recursive --archive --no-dereference --force --no-clobber --verbose --no-target-directory --update`, and `--preserve` with or without its list; the destination written and every source a read.
>
> **A destination naming an existing directory withholds**, for `mv` and `cp` alike, `-T` or not. The write then lands at a path under it that the bounds test never read, and through any link already there: GNU `cp` follows a link it overwrites. `-t`, `--target-directory` and `--parents` are absent from both rows on the same ground, since each names a directory to write into.

The section's **Honest limits** sentence **Clause (e) reads the filesystem at the call, so a link created between the call and its run is not seen.** reads: "The link and directory tests read the filesystem at the call, here and in rules `truncate_scratch` and `append_scratch`, so a link or a directory created between the call and its run is not seen." **Not yet applied.**

`native/src/guard/rules/grants.rs`: the glob admission leaves the bounds test, and `-t`, `--target-directory` and `--parents` leave the `mv` and `cp` rows of the operand table; `mkdir`'s `--parents`, `touch`'s `-t` and `rmdir`'s withheld `--parents` stay as they are. A destination that `symlink_metadata` reads as a directory withholds. The `spec:` comment above `fn write_words`, which says its words are dequoted with backslashes removed, is reworded to what the function then reads, pointing at rule `bounded_write`. **Not yet applied.**

### (4) Rule `bounded_wait`'s arm (B) record is the literal test's one exemption {design-bearing} {user-facing: operator direction of 2026-10-20, "keep bounded_wait arm (B), its record statement matched byte-for-byte to the template as the literal test's one exemption"; delegation-kit/SPEC.md §The delegation model, "rule `bounded_wait` grants the one canonical recorded launch of an allowlisted command, so the compliant spelling costs no permission decision"}

Replacement for `guard-kit/SPEC.md` rule `bounded_wait`, arm (B)'s clauses (B0) and (B3). **Not yet applied.**

> - **(B0) The shape.** The call is exactly three or four statements, separated after the launch by `;` or a newline: a launch ending in a statement-ending `&`; the record statement; `wait`; and optionally `rm -f` of that same record path. **The record statement is the template `echo "pid=$! run=<key>" > <dir>/<key>.run` byte for byte**, `<dir>` a `GUARD_KIT_SCRATCH_DIRS` member, `<key>` matching `[A-Za-z0-9._-]+` and the filename key equal to the `run=` key. It is the literal test's one exemption (§The generic ruleset): the engine sets the template statement aside and holds the rest of the call to the test, so the launch, `wait` and `rm -f` are literal.
> - **(B3) Conservative decline.** The literal test refuses every substitution and expansion outside the template, whose `$!` is its own.

`native/src/guard/`: the predicate takes the deciding rule, and for rule `bounded_wait` alone it reads the call with a statement matching arm (B)'s template set aside. The template match is arm (B)'s own function. **Not yet applied.**

### (5) The two rewrites take the literal test {mechanical} {user-facing: guard-grant-literal-shrink's Deliverable, as delta 1}

Rule `abs_script`'s second clause bullet reads: "no redirect in it names a file, which a pattern's trailing star would reach; an fd-dup and `/dev/null` are no file." Its sentence **Otherwise the corrective block fires, a heredoc-bearing command's among them since its dequoted view does not align** reads "Otherwise the corrective block fires, a call failing the literal test (§The generic ruleset) among them". **Not yet applied.**

Rule `brace_glyph`'s sentence **The rewrite is a grant, so it attaches on rule `abs_script`'s test** reads "The rewrite is a grant, so it attaches only to a literal call (§The generic ruleset), a bare `{}` passing as spelled, and on rule `abs_script`'s test". **Not yet applied.**

### (6) Rule `emitter_write` reads the literal test where it promises a grant {mechanical}

Rule `emitter_write`'s arm (a) reads "The command holds more than one statement, and one of them, issued alone, satisfies every clause of rule `append_scratch` and the literal test (§The generic ruleset)." Arm (c)'s last-but-one sentence, from **It is the composition of (a) and (b)**, reads "It is the composition of (a) and (b), so every emitter write statement meets rule `append_scratch`'s or rule `bounded_write`'s grant, one of these three arms, one of the declines below, or, refused by the literal test, the harness's own path." **Not yet applied.**

### (7) The variant lane holds a grant row's quoted re-spelling ungranted {design-bearing}

Replacement for `guard-kit/SPEC.md` §Testing, the relation's last bullet **an `allow` or `rewrite` row's takes any of the four.** **Not yet applied.**

> - an `allow` or `rewrite` row's takes any of the four, except under the `sq`, `dq`, `esc` and `part` transforms, where it takes anything but a grant: each adds a quote or a backslash, which the literal test refuses (§The generic ruleset).

The same section's sentence **The relation is one-sided because a grant declines on a spelling it does not model** reads "The relation is one-sided because a grant declines on a spelling it does not model, and on a quoted or escaped one it must". **Not yet applied.**

`native/src/emit/run_guard_tests.rs`, the variant lane's relation: a granted re-spelling of an `allow` or `rewrite` row under those four transforms fails, at the kit lane's mismatch form, and no limit row admits it. **Not yet applied.**

### (8) The decision tables, the sandbox and the link cases {mechanical}

- **The sandbox** (guard-kit/SPEC.md §Testing, precondition 4) adds a `.tmp/held/` directory beside `.tmp/notes.txt`, which rule `bounded_write`'s directory-destination rows write into. The precondition count stays five. **Not yet applied.**
- **Every grant row the literal test refuses** moves to the verdict the member returns, read row by row, and its section comment says why. A row that moves to `block` names the rule that claims it in that comment. Each grant rule keeps at least one literal firing row.
- **New rows, one per case, each asserting the verdict the member returns, which is never a grant**: `cp tracked.md .tmp/held`, `mv .tmp/notes.txt .tmp/held`, `cp -t .tmp/held tracked.md`, `touch .tmp/o*` and `ls > .tmp/o*`, beside the granted `cp tracked.md .tmp/held/copy.md`. The quoted rows `echo "a b" >> .tmp/j.md` and `grep -rn "foo bar" tracked.md` are added where no moved row already pins the class.
- **The link cases are crate tests**, on the pattern of `grants.rs`' `a_link_anywhere_on_the_path_is_seen`: the link is made on either host and its assertions skipped where the host refuses one. A scratch repository holds a gitignored `.tmp/` with a link to a tracked file and a link to a directory. `: > <link>`, `echo x >> <link>`, `echo x > <link>`, `touch <link>/f` and `cp <tracked> <dir-link>` each take no grant, and each same call on a plain scratch target is granted. The decision table's sandbox carries no link, since a host may refuse to make one and a row cannot be skipped, so rule `bounded_write`'s **Honest limits** gain the sentence "The link test is held by the crate's tests, which skip where the host refuses a link; the decision table's sandbox carries none." **Not yet applied.**

### (9) The release declaration and the mirror {mechanical}

`.workflow/release-declarations.md`, under **Behavior changes**, gains one bullet. It opens **guard-kit shell guard, the grant rules** and says that rules `truncate_scratch`, `append_scratch`, `ro_pipeline`, `bounded_wait`, `bounded_write`, `abs_script` and `brace_glyph` now grant only a literal call. A quoted, escaped, globbed or heredoc-bearing call, a write through a scratch link and a `cp` or `mv` into a directory fall through to the harness's own decision, or meet a rewrite's corrective block. It ends "Nothing to do; a call that used to pass silently may now ask." **Not yet applied.**

The docs mirror regenerates: `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

## Producers and consumers

- **The literal test** (delta 1). Producer: `literal_call`, reached on every hook call the bash reader serves, from `engine::decide`'s grant-verdict check and from rules `abs_script` and `brace_glyph`. Consumers: the engine, which declines the grant, and those two rules, which block. It reads the command's raw text, which every rule's context already carries. It mints no view and no knob. Rule `worktree_confinement`'s predicate list and §Consumer rules are untouched: a consumer's decision is relayed before the engine runs, so its grants stay bounded by its author.
- **The link test** (delta 2). Producer: the one function bounds clause (e) calls, now also called by rules `truncate_scratch` and `append_scratch` on each target. It reads the filesystem through `symlink_metadata`, relative to the working directory, as clause (e) does. Consumers: those two rules' grant decisions.
- **The directory test** (delta 3). Producer: rule `bounded_write`'s operand walk, on `mv`'s and `cp`'s destination. Consumer: that rule's grant decision.
- **The template exemption** (delta 4). Producer: arm (B)'s template match, called by the predicate when the deciding rule is `bounded_wait`. Consumer: the engine's check. No other rule reads past the template.
- **The variant relation** (delta 7). Producer: the variant lane's classifier. Consumer: the lane's verdict, at exit 1 through the existing mismatch form.
- **Roster-holding readers, each red condition named:**
  - `check-guard-registration` (§check-guard-registration), assertion C, reds where an item's declared views differ from the table's. No rule's declaration changes here: the engine reads the raw text outside any rule, and rules `abs_script` and `brace_glyph` already declare `raw`. It reds only if build adds a view read without its declaration.
  - `--run-guard-tests`' kit lane reds on every row whose authored verdict the member no longer returns. That is each grant row delta 1 refuses (counted above), until delta 8 moves it. Its variant lane reds, under delta 7's relation, on any quoted re-spelling a grant still blesses, and on a limit row nothing needs, of which none is added.
  - `--run-guard-tests`' §Testing obligation, at least one firing and one non-firing case per generic rule, holds: delta 8 keeps a literal firing row in each grant section. It reds on a section left with no firing row.
  - `check-amendment-queue` reds a `[spec:]` ref resolving to no file, and a file no entry points at. Both paired entries carry this file's name.
  - `check-release-change-declared` reads kit `bin/` deletions and claimed templates. This amendment touches neither, so it does not red; delta 9's bullet is the declaration a user-facing change owes.
  - `check-docs-mirror-fresh` reds the mirror until it regenerates (delta 9).

## Existing sections updated

- `guard-kit/SPEC.md` §The generic ruleset, the shared direction on what a grant bounds (delta 1).
- `guard-kit/SPEC.md` §The rule roster: rules `truncate_scratch` and `append_scratch` (delta 2), rule `bounded_write` (delta 3), rule `bounded_wait`'s arm (B) (delta 4), rules `abs_script` and `brace_glyph` (delta 5), rule `emitter_write` (delta 6).
- `guard-kit/SPEC.md` §The recommended allowlist, the absent-entries bullet **A write to a gitignored target, a truncation of one, a sanctioned wait and a write bounded to a scratch dir**, reads "…grant their literal spellings from the hook, where no settings entry can reach them" (delta 1). **Not yet applied.**
- `guard-kit/SPEC.md` rule `ro_pipeline`, the paragraph **Why admit a cleared program**, gains the sentence "A program spelled with a quote takes the literal test's refusal (§The generic ruleset), so the discriminator admits a program spelled bare" (delta 1). **Not yet applied.**
- `guard-kit/SPEC.md` §Testing: the variant relation (delta 7) and the sandbox's precondition 4 (delta 8).
- `guard-kit/guard-tests/cases.tsv` and `guard-kit/guard-tests/background-cases.tsv` (delta 8), found by the census above: an awk pass over each `# rule` section's `allow` and `rewrite` rows for a quote, a backslash or a glob character.
- `native/src/guard/engine.rs`, `native/src/guard/rules/grants.rs`, `native/src/guard/rules/spelling.rs` and `native/src/guard/rules/liveness.rs` (deltas 1, 2, 3, 4 and 5), found by `grep -n "Verdict::Allow\|Verdict::Rewrite" -r native/src/guard`, which names `grants.rs`'s `allow` helper, `spelling.rs`'s two rewrites and `liveness.rs`'s arm (A) grant. Arm (B)'s grant site is the same rule's function.
- `native/src/emit/run_guard_tests.rs`: the variant relation (delta 7) and the sandbox's `.tmp/held/` (delta 8).
- `.workflow/release-declarations.md` (delta 9).
- `docs/guard-kit/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- None — no delta renames or retires a name; the options leaving the `cp` and `mv` rows are GNU's own spellings, which other surfaces still name.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Both entries moved to Done** by the build stage that merges this amendment, before the drain stage. Every oracle here runs locally, the link cases on the Unix host, so neither entry waits on a remote run.
