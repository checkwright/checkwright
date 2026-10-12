# SPEC amendment: git-word-read

Rule `git_stash` reads git's words by their value, and every other rule that reads a git command reads them off the skeleton or a view that keeps an escape. Run through the hook member on this tree, in the decision-table sandbox:

- `git 'grep' foo`, `'git' grep foo`, `git gr\ep foo` and `command git grep foo` fall through rule `git_grep`, and `git 'commit' --amend` loses rule `git_rewrite`'s advise;
- `git add tracked.md && git comm\it -m x` falls through rule `commit_only_paths`, `git \rm -q -f tracked.md` falls through rule `rm_tracked`'s force arm, and `'git' -c core.pager=cat log --oneline -3` falls through rule `git_c_root`'s arm (b);
- `command git stash`, `nohup git stash`, `FOO=1 git stash`, `/usr/bin/git stash`, `env git stash` and `sudo git stash` fall through rule `git_stash`, which blocks `\git stash`.

The queue entry records the same read under a live producer's record, where the quoted and wrapper-led spellings pass rule `git_mutation_under_producer`. This amendment gives the git rules one read of what a segment passes git, and rules which spellings of the command word that read reaches.

## What changes

### (1) One git read, by value, for every rule that reads a git command

Every rule that reads a git command takes its words from one read, built on the word read as rule `git_stash`'s is, and none reads the skeleton's words or the dequoted view for them {design-bearing} {user-facing: the entry's Deliverable — the word read rule `git_stash` takes, lifted to the walk the git rules share; a behavior change that widens three blocks, one steer and one advise}.

- **The read.** For each segment of the compound split it takes the word read's words, drops a leading shell keyword and every redirect with its target, both read off the live text, and answers the values of what is left. The global-option walk runs over those values. Where the word read answers nothing, the skeleton's words are read and a quoted word there stays unread, rule `git_stash`'s fallback kept for all.
- **Its callers**, by `git grep -n -e 'git_subcommand' -e 'git_walk(' -- native/src/guard/rules` and the two rules that match git with a pattern of their own:
  - rule `git_mutation_under_producer`, which finds the subcommand its act set names;
  - rule `git_c_root`'s arm (b), which finds `-c` among the global options;
  - rule `commit_only_paths`, which finds `add` and `commit` and reads the commit's words;
  - rule `git_stash`, whose own word cut becomes the read;
  - rule `rm_tracked`'s force arm, which finds `rm` and its flag;
  - rule `git_grep`, which reads the first two skeleton words today;
  - rule `git_rewrite`, which matches `git`, a blank run and the subcommand anywhere in the skeleton today.
- **One function, called.** The cut and the fallback live in the read, and no caller keeps a copy.
  - Where `git-stash-glued-redirect-value` has landed, the read takes rule `git_stash`'s repaired redirect cut as it stands.
  - Where it has not, the read takes the cut as this tree has it, and that unit's repair then reaches every caller through the one function.
- **What moves beside the quoted spellings**, since two callers gain the global-option walk they never ran:
  - rule `git_grep` steers `git --no-pager grep foo`, and declines on a global option that moves the repository (`-C`, `--git-dir`, `--work-tree`), as rule `git_stash` does, since the tree it names is not the one the Grep tool reads;
  - rule `git_rewrite` advises `git --no-pager commit --amend` and `git -C <dir> commit --amend`;
  - rule `git_rewrite` reads one segment at a time, so a subcommand in one statement and an option word in another no longer advise: `echo --amend; git commit -m x` falls through, where it advises today.
- **Rule `git_rewrite`'s option test is unchanged and reads each word's value** after the subcommand. A value-taking option's value is not told from an option, so a commit whose message is the one word `--amend` advises. An advise refuses nothing, and the roster of value-taking options is rule `commit_only_paths`', not copied here.
- **The declarations follow the read.** A rule calling the word read declares `raw`, `sq dq hd` and `dequoted` (§The reader and its views). Rules `git_mutation_under_producer`, `git_c_root` and `git_grep` gain `dequoted`, rule `commit_only_paths` gains `raw`, and rule `git_rewrite` gains `raw` and `dequoted`. The roster item and the crate's rule table move together, which `check-guard-registration` holds.
- **Under PowerShell** the read is the same call over the PowerShell reader's word read, so a backtick-escaped subcommand is read by its value.

New sub-bullet in the `git_mutation_under_producer` item, placed ahead of its **Declines** bullet, which loses its last sentence's *the walk reaching `git -C <dir> commit`* clause to it. **Not yet applied.**

> - **The git read** answers what a segment passes git, and every rule that reads a git command calls it. For each segment of the compound split it takes the word read's words (§The reader and its views) and drops a leading shell keyword and every redirect with its target, both read off the live text, so a quoted keyword or `>` is a word like any other and a word glued ahead of a redirect stays one. It then finds the command word (below) and runs the global-option walk over the values of the words after it, reaching `git -C <dir> commit`. So `git 'commit' -m x`, `git "add" -A`, `git comm\it` and `git com'mit'` are the plain spelling's commands. Where the word read answers nothing the skeleton's words are read, and a quoted word there stays unread.

Replacement for the `git_stash` item's *Every word is read by its value* bullet. **Not yet applied.**

> - **Every word is read by its value**, through the git read (rule `git_mutation_under_producer`): the command word, each global option, the subcommand and the verb. So `git 'stash' push`, `git stash "pop"` and `git st\ash` block as the plain spelling does, and `git stash 'list'` passes as `list` does.

Replacement for the `commit_only_paths` item's *The words are read from rule `sed_file`'s dequoted view* bullet. **Not yet applied.**

> - **The words are read through the git read** (rule `git_mutation_under_producer`), so a quoted or escaped subcommand or option is the one the shell passes. Where it falls back to the skeleton, a placeholder for a quoted word reads as a path.

Replacement for the last sentence of the `rm_tracked` item's force-arm bullet, *The words are read from the dequoted view…*. **Not yet applied.**

> The words are read through the git read.

Clause added to the `git_grep` item, replacing *It fires when the command leads with `git grep`, names no revision*. **Not yet applied.**

> It fires when the call is one segment the git read answers as `grep`, under no global option that moves the repository, names no revision

Replacement for the opening of the `git_rewrite` item, through *gets an advise steer*. **Not yet applied.**

> A segment the git read answers as `commit` carrying `--amend`, `-F` or `--file` among its words, or as `reset` carrying `--soft`, gets an advise steer

Replacement for §The reader and its views' sentence *Rules `rm_tracked`, `find_exec` and `git_stash` call it.* **Not yet applied.**

> Rules `rm_tracked` and `find_exec` call it, and so does the git read every git rule takes (rule `git_mutation_under_producer`).

### (2) The command word: read by value, by path and through the harness view

A segment is git's where its command word names git however the word is spelled, and behind every wrapper the permission matcher strips {design-bearing} {user-facing: the entry's Deliverable — a ruling in the section on a wrapper-led or path-spelled `git`, read or stated as a limit; a behavior change that widens the same blocks}.

- **By value.** `'git'`, `"git"` and `\git` are `git`.
- **By path.** A value whose last `/`-parted component is `git` is git: `/usr/bin/git commit` runs the command the plain spelling runs.
- **Through the harness view.** The wrappers that view strips are dropped ahead of the command word, by that view's own walk, called and not copied: `command git stash`, `nohup git stash`, `time git stash` and `FOO=1 git stash` are read. The matcher strips them before it matches, so a grant for the git command reaches the call with no decision, and the block is then the only thing between a wrapper and the mutation.
- **Stated as a limit: a second program that runs git.** `env git …`, `sudo git …` and an option-carrying `xargs` are not read. The matcher strips none of them, so a grant for a git command does not reach the call and it takes a permission decision of its own. The roster of programs that exec their operand has no end a kit could name.
- **A wrapper is read as the harness view reads it**, off the live text, so a quoted wrapper strips nothing and its segment is not git's.
- **Under PowerShell** the harness view is the identity, so the value and path reads apply and no wrapper is dropped.

New sub-bullet in the `git_mutation_under_producer` item, following delta 1's. **Not yet applied.**

> - **The command word names git however it is spelled.** It is read by its value, so `'git'` and `\git` are `git`, and a value whose last path component is `git` is git too (`/usr/bin/git commit`). The wrappers the harness view strips (§The reader and its views) are dropped ahead of it by that view's own walk, read off the live text: the matcher strips `command`, `nohup`, `time` and a leading assignment before it matches, so a grant for the git command reaches the wrapped call and the block is all that stands before the mutation. **The honest limit:** a second program that runs git, `env git …`, `sudo git …` or an option-carrying `xargs`, is not read. The matcher strips none of them, so no grant for a git command reaches the call, which takes a permission decision of its own.

Replacement for the `git_stash` item's honest-limits sentence *A stash behind an alias or a script is not read.* **Not yet applied.**

> A stash behind an alias, a script or a second program that runs git is not read (rule `git_mutation_under_producer`, the command word).

### (3) The rows

Each git rule's rows are re-derived and the spellings above pinned {mechanical}.

- **Where `guard-spelling-variant-oracle` has landed**, its lane derives the quoted, first-character-escaped and `command`-led spellings of every row these sections hold, so this unit authors none of those. Its commit deletes each limit row the read makes stale, as the lane names them, under rules `git_grep`, `git_rewrite`, `git_stash`, `commit_only_paths`, `git_c_root` and `rm_tracked`. A limit row whose failures come from another rule's read stays.
- **Where it has not**, this unit authors a quoted-subcommand, a quoted-command-word and a `command`-led row per caller.
- **Authored in either case, in `guard-tests/cases.tsv`**, since no transform derives them:
  - an escape inside a word, `git gr\ep foo` and `git add tracked.md && git comm\it -m x` → block, and `git comm\it --amend` → advise;
  - a path-spelled command word, `/usr/bin/git grep foo` and `/usr/bin/git stash` → block;
  - a stripped wrapper other than `command`, `nohup git stash` and `FOO=1 git stash` → block;
  - the limit, `env git stash` and `sudo git stash` → fallthrough;
  - the global-option reach, `git --no-pager grep foo` → block, `git -C sub grep foo` → fallthrough, and `git --no-pager commit --amend` → advise;
  - the segment read, `echo --amend; git commit -m x` → fallthrough;
  - one quoted word that is no subcommand, `git 'grep foo'` → fallthrough.
- **`gate-tests/git-mutation-under-producer.test.sh`**, beside its live-record cases: `git 'commit' -m done`, `git "add" -A`, `git comm\it -m done`, `git com'mit' -m done`, `git 'reset' --hard`, `'git' commit -m done`, `\git commit -m done`, `command git commit -m done` and `/usr/bin/git commit -m done` block, and `env git commit -m done` passes. Under a PowerShell payload, `git 'commit' -m done` and a backtick-escaped subcommand block.
- **Every existing row led by `git` is re-derived**, by `grep -n -e 'git' guard-kit/guard-tests/cases.tsv guard-kit/guard-tests/powershell-cases.tsv`. Rule `git_rewrite`'s rows are the ones a narrowing can move: an `advise` row that stops advising for any cause but the segment read stops the build.
- **The tables' comment blocks** for the seven callers name the git read where they name a view.

Replacement for §Testing's sentence *There a tracked-tree mutation blocks while a recorded producer is alive, and so does a PowerShell `git commit`, while one carrying a `$` declines.* **Not yet applied.**

> There a tracked-tree mutation blocks while a recorded producer is alive, spelled plain, quoted, escaped, by path or behind a wrapper the matcher strips, and so does a PowerShell `git commit`, while one carrying a `$` declines and one behind `env` is not read.

## Producers and consumers

- **The read.** Producer: each of the seven callers, on every `Bash` or `PowerShell` payload whose rule reaches its git test. Consumer: the session, through each rule's verdict, whose message texts are unchanged.
- **Citations of the walk the read replaces**, by `grep -n -e 'global-option walk' guard-kit/SPEC.md`: the `git_c_root` arm (b), `commit_only_paths`, `git_stash` and `rm_tracked` items, each citing rule `git_mutation_under_producer`'s walk, which stays that rule's and is now run by the read.
- **Readers of the declarations.** `check-guard-registration` reds on a roster item and a table row that disagree, so delta 1's declaration edits land with the table's in one commit.
- **Widening and narrowing.** Six callers only widen. Rule `git_rewrite` also narrows, by the segment read, and the decision tables red in both directions (§Testing), so delta 3 pins the one narrowing in a row, `echo --amend; git commit -m x`, and names the stop.
- **Predicates.** Rule `rm_tracked`'s test is a predicate of rule `bounded_wait`'s arm (B1) and rule `bounded_write`'s clause (e), so the force arm's wider read withholds a grant from a call carrying a quoted or wrapper-led forced `git rm`. No grant widens.
- **The steers' text.** Each message names `git <subcommand>` by the value read, as rule `git_stash`'s does, so a steer never prints a wrapper or a quote.
- **Instruction surfaces naming these rules**, by `git grep -n -e 'git_mutation_under_producer' -e 'git_stash' -e 'commit_only_paths' -- delegation-kit lifecycle-kit CLAUDE.md .claude`: each names the plain spelling a session is to use, and none promises a pass to a quoted or wrapper-led one.
- **Knobs.** None is minted and none gains a reader.

## Existing sections updated

- `guard-kit/SPEC.md` §The rule roster — the `git_mutation_under_producer` item's two new sub-bullets and its Declines bullet, and the `git_stash`, `commit_only_paths`, `rm_tracked`, `git_grep`, `git_rewrite` and `git_c_root` items' read sentences and declaration clauses (deltas 1 and 2).
- `guard-kit/SPEC.md` §The reader and its views — the word read's caller sentence (delta 1).
- `guard-kit/SPEC.md` §Testing — the live-producer suite's case list (delta 3).
- `native/src/guard/rules/mod.rs` — the read beside the walk, and the rule table's declared views (deltas 1 and 2).
- `native/src/guard/rules/liveness.rs`, `native/src/guard/rules/reach.rs`, `native/src/guard/rules/tools.rs`, `native/src/guard/rules/spelling.rs` — the seven callers (delta 1).
- `guard-kit/guard-tests/cases.tsv`, `guard-kit/guard-tests/powershell-cases.tsv` — the rows and comment blocks (delta 3).
- `guard-kit/guard-tests/variant-limits.tsv` — the stale limit rows deleted, where that table exists (delta 3).
- `guard-kit/gate-tests/git-mutation-under-producer.test.sh` — the live-record cases (delta 3).
- `docs/guard-kit/SPEC.md` — generated, stale once any delta lands (all deltas).
- `.workflow/release-declarations.md` — a Behavior-changes bullet for the widened git rules and the one advise that narrows (deltas 1 and 2).

## Retired spellings

- None — the amendment widens a read and retires no spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`), discharged at the iteration.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry moved** — the queue entry moves to Done in the merging commit, ahead of the drain stage.
