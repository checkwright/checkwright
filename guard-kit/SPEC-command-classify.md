# SPEC amendment: command-classify

**Deltas 1 to 4 are applied, with the parts of deltas 10 and 11 that belong to them; every other
passage is still a proposal for the build stage to land.** Where replacement wording is given it is marked **Not yet applied** at the passage itself.

This amendment serves the eight entries of the `guard-command-classification` unit set, which point
at it together:

- `grant-argument-bounding-mechanism` is the lead entry: two committed grants reach a destructive
  form (deltas 6 and 7), and rule 7's `{}` rewrite grants one silently (delta 12).
- `grant-path-traversal-exposure`: the script-runner globs reach a traversing path (delta 7), and
  rule 4's absolute-path rewrite grants a scratch script silently (delta 12).
- `ro-bins-write-option-bypass`: roster membership stands in for a read-only invocation (delta 5).
- `guard-command-prefix-wrapper`: a prefix displaces the token the matcher reads (deltas 1 and 2).
- `guard-read-steer-tool-coverage`: `awk` reads a file with no steer (delta 3).
- `guard-steer-names-absent-tool`: two steers name tools a harness build may not carry (delta 4).
- `backgrounded-shell-child-run-record-unenforced`: the launch record is advised, never required
  (deltas 8 and 9).
- `wait-loop-exemption-blind-behind-a-script-name`: the wait-loop exemption cannot see through a
  name (delta 8).

**Why one amendment rather than eight.** The entries share two helpers and one numbering plan. The
argument bound (delta 7) reads the command through the harness view (delta 1). The recorded-launch
grant (delta 9) applies the bound (delta 7), and rule 19's own pre-existing clauses narrow together
with the declared forms (delta 5). Only one rule is inserted, and only fall-through logging renumbers
(delta 11). Two amendments would each carry a numbering plan the other can invalidate.

## The grounds this design rests on

This section is amendment-only rationale and does not merge. Every claim in it was probed on
2026-09-11 at the commit that stamped this stage.

**The live verdicts.** The consumer guard was driven by payload, with its friction log pointed at
scratch. The table is recorded at `.workflow/survey-record.md` under this stage's block, with its
witness. In summary:

- **Allowed silently:** `sort -o`, `--output=`, `-uo` and `--compress-program=`; `uniq A B`;
  `find -fprint`, bare and piped; `rg --pre=`, bare and piped.
- **Rewritten, which is also an allow:** `find … -exec rm -rf {} \;` and `+`; `xargs -I{} rm {}`;
  an absolute-path `.tmp/check-*.sh`.
- **Passed through, then allowed by a committed glob:** `rm -rf .tmp/../<local file>`;
  `git rm -q -f <modified file>`; `bash ../../evil/checks/check-x.sh`.
- **Passed through:** `time`, `timeout`, `nice` and `env` prefixes; `git -c core.pager=cat log`;
  every `awk` read.
- **Blocked, steering to a tool this harness build does not carry:** bare `find` (Glob) and
  `git grep` (Grep).
- **Advised:** a backgrounded binary arm and a backgrounded battery.
- **Passed through, and the harness then decides out of band:** the canonical recorded launch (delta
  9's shape).

**Vendor facts, fetched from the harness's permissions and hooks references.**

- The matcher strips a fixed wrapper set before it matches a Bash rule. The set is `timeout`,
  `time`, `nice`, `nohup` and `stdbuf`, their arguments included, plus `command`, `builtin`,
  `noglob`, a leading assignment of certain known-safe variables, and bare `xargs`. It does not
  strip `xargs` with flags. It does not strip `git -c`; the reference cites
  `git -c core.fsmonitor=<script>` as an execution reach.
- A rule's `*` matches any text including spaces. Argument-constraining allow patterns are called
  fragile, and the remedies named are deny rules or a `PreToolUse` hook.
- Deny rules are evaluated first, apply per subcommand, and cannot carry allow exceptions. A hook's
  allow does not bypass a matching deny or ask rule.
- A foreground command that reaches its timeout is moved to the background. The result text names a
  task id and an output file under the session scratchpad. No hook payload is documented to list
  running background tasks. In-tree, delegation-kit has recorded that the `SubagentStop` payload
  does carry a populated `background_tasks` array of harness-launched children with a `status`, and
  that it misses a child detached with a shell `&` (§What `background_tasks` carries).
- Native builds with embedded search carry no Grep or Glob tool. The `PreToolUse` payload carries no
  toolset field.

**Two premises in the entries are stale, and the design corrects them rather than inheriting them.**

- `guard-command-prefix-wrapper` measured `time bash <allowlisted>` as prompting. The matcher now
  strips `time` itself, so the only surviving shapes are an absolute wrapper spelling and `git -c`.
  What was measured as a prompt is now a ranker that strips `sudo` and `timeout` and nothing else.
- `guard-read-steer-tool-coverage` costs an `awk` read as an out-of-band decision. A committed
  `Bash(awk *)` grant landed, operator-ruled, on 2026-08-30, so the read is now granted. The steer
  stays, on the tool-hygiene ground in §What the steering is buying and the masking caution in
  §The triage criterion. Its cost line on the entry is stale.

**Rule 15's own re-opening condition has fired.** The advisory shipped on 2026-08-22. The next
day an align session backgrounded two battery runs with it live and wrote no record. That session is
exactly the *firing after this advisory ships* that rule 15 names as the attested record for a block.

## What changes

### (1) The harness view: one helper models the matcher's wrapper strip, with a compiled twin

**Applied**, except one clause: §The guard framework's duplication paragraph does not yet name the
helper's shell caller, because that caller is rule 24 and merges with delta 7.

A new `lib/guard.sh` helper, `_guard_harness_view <segment>`, returns the segment as the permission
matcher reads it: with the leading wrappers the matcher strips removed, repeatedly, from the head
{design-bearing}. The stripped set is the documented one, and each wrapper's arguments are walked by
its own grammar:

- `time`, with an optional `-p`.
- `timeout`: `--preserve-status`, `--foreground`, `-v`, `--verbose`, `-s`/`--signal`/`-k`/
  `--kill-after` in their separate and glued spellings, then exactly one duration word.
- `nice`: `-n N`, `-nN`, `--adjustment=N`, `-N`.
- `nohup`, `builtin` and `noglob`.
- `stdbuf`: any run of `-i`/`-o`/`-e` in separate or glued spelling, and `--input=`/`--output=`/
  `--error=`.
- `command`, except when the next word is `-v` or `-V`: that is a query, not a wrapper.
- A leading run of `NAME=value` words.
- `xargs`, only when the next word is not option-shaped.

**An option a wrapper's walk does not recognize stops the strip at that wrapper.** The view then
begins at the wrapper word. That is the ruleset's bias toward passing: a view that strips less
matches fewer grants.

**It is a classification view and never a grant view.** Delta 7 reads it to see what a grant would
match. No rule grants through it: a wrapper-prefixed command the matcher would grant is granted by
the matcher itself, so the guard has nothing to add. That is why `guard-command-prefix-wrapper`'s
proposed *strip then re-test via `guard_allow_match`* grant is refused. The matcher already takes it
for the stripped set. For the unstripped shapes a grant would bless a program reach (delta 2).

**The env-assignment arm strips every assignment rather than the undocumented known-safe subset.**
The two readers bear that differently:

- For delta 7's bound it only adds scrutiny.
- For the ranker it can under-count prompts on an env-prefixed command.

That under-count is stated rather than hidden. The subset is not in the reference, so any narrower
list would be a guess presented as a model.

**The compiled twin, and why it is owed.** `native/src/emit/scan_prompts.rs`' `strip_decoration`
models the matcher's wrapper strip for the ranker's grant test and ranking key. Its roster —
`sudo `, `timeout `, and a leading digit-led word that was `timeout`'s duration — disagrees with
the matcher in both directions:

- It strips `sudo`, which the matcher does not strip, so it counts `sudo`-prefixed calls as granted.
- It misses `time`, `nice`, `nohup`, `stdbuf`, `command`, `builtin`, `noglob`, assignments and bare
  `xargs`, so it counts those calls as prompting.

The body becomes a call to a new `guard::harness_view` in `native/src/guard.rs`. The `sudo ` strip
and the digit-word strip both retire, since the duration is now walked where `timeout` is. The name
`strip_decoration` stays.

The shell helper and the compiled one are one predicate held twice. That makes five twinned
primitives where §The guard framework says four, and they are held equal the same way: a
`--guard-lib-parity harness-view <cmd>...` mode printing `harness-view<TAB><cmd><TAB><view>`,
compared against the shell holder over a canned corpus in `gate-tests/guard-lib-parity.test.sh`.
The duplication is permanent on the same ground as the other four. The shell caller is rule 24
(delta 7), a function in this permanently-shell file.

**The KPI steps at the landing commit, for a definitional reason.** Changing the strip moves calls
between *granted* and *prompting* and re-keys some rows, so `kpi-prompt-friction` reads a
discontinuity that is not behavioral. §scan-prompts' step rule applies unchanged. Record the
`--emit scan-prompts --count` reading on the live log immediately before and after, in that section,
and leave the KPI as it is.

**The honest limit.** The wrapper set is the reference's at the fetch date. A harness revision that
widens or narrows it is drift no gate here can self-detect. That is the footing §The guard framework
already declares for the payload roster.

### (2) Rule 2 widens to a prefix spelling the matcher does not see through

Rule 2 becomes **a prefix spelling that falls off the match path, steered to the spelling that stays
on it**, and gains two arms beside its `git -C <root>` arm {design-bearing}. Its title changes, and
its number and placement do not. **Applied.**

- **(b) `git -c <key>=<value>`** among a git segment's global options, found by
  `_guard_git_subcommand`'s walk, is blocked. The corrective has three parts:
  - Drop the `-c`. A pager or color override has no effect without a terminal, and the bare
    subcommand is the allowlisted spelling.
  - A `-c` form is never granted, because a `-c` key can name a program the subcommand runs:
    `core.pager`, `core.fsmonitor`, `core.sshCommand`, `alias.*`. The same reach is why the matcher
    does not strip it.
  - A genuine config override runs through `!<command>`.
- **(c) An absolute spelling of a stripped wrapper** — a command word whose basename is `time`,
  `timeout`, `nice`, `nohup` or `stdbuf` and which carries a `/` — is blocked, with the steer to the
  bare wrapper name the matcher strips. The corrective states the one loss: a format option only
  the binary takes (`/usr/bin/time -f`) has no stripped spelling, and runs through `!<command>`.

**Why a steer and not a grant.** Rule 20's reasoning applies. Both shapes are decided out of band
anyway, so blocking converts that decision into a durable steer at no extra cost. A grant of (b)
would bless the program reach above.

### (3) Rule 8 widens to `awk` reading a file, on one program-then-operands walk

Rule 8 becomes **`sed` or `awk` reading a file, or `sed` rewriting one**, and its number and
placement do not change {design-bearing}. **Applied.**

**One walker, not a third per-tool parser.** `_guard_sed_segment`'s option walk generalizes into
`_guard_program_operands <tool> <segment>`, which separates the program word from file operands
through a per-tool table:

- `sed`: `-e`/`-f` consume an argument and supply the script; `--expression=` and `--file=` do the
  same; `-i` in any spelling blocks as today; other short and long options take no argument.
- `awk`: `-F` and `-v` consume an argument; `-f` consumes one and supplies the program; `--` ends
  options. Any other option word declines.

`cat` keeps `_guard_is_cat_read`, because it has no program word.

**The `awk` arm fires on two read shapes and nothing else.** Both require exactly one file operand
and no pipe into the segment.

- **(i) A line-range read.** The program's pattern is built only from `NR` comparisons (`NR>=a &&
  NR<=b`, `NR==a,NR==b`, and the strict forms), with no action or the print-all action (`{print}`,
  `{print $0}`). The steer names the Read tool's offset/limit.
- **(ii) A heading-range read.** The pattern is a `/re1/,/re2/` range on a `.md` operand, with no
  action or print-all. The steer names the section extractor. Its printed command is derived from
  `GUARD_KIT_LIB` exactly as rule 23 derives the runner:
  `bash <root>gate-sdk/bin/run-gates.sh --emit md-section <file> "<heading>"`. That is the same
  cross-kit coupling rule 23 already admits, on the same ground.

An action, a program file, a non-`.md` range, a second operand, or a stream all pass.

**Which steer to emit is decided by the program's shape, never by a measured frequency.** The
recorded measurements flip with what an iteration reads. Three shapes are refused as steer inputs:
a transform, a single-regex filter, and a range on a non-markdown file.

**The committed `Bash(awk *)` grant is not argued with.** A `PreToolUse` block is not overridden by
an allow, so the steer fires on the two read shapes ahead of the grant. The grant stays load-bearing
for `awk` as a stream filter and for every program shape the arm declines.

**`awk` has no honest read-only declaration** (delta 5). A program can print to a file or call
`system()`, so a consumer adding `awk` to `GUARD_KIT_RO_BINS` gets it withheld unless it declares it
falsely. Rule 8's placement before the auto-allow band still guards that consumer.

### (4) Rules 9 and 11 fire only toward a tool the consumer declares present

A new knob, `GUARD_KIT_SEARCH_TOOLS`, lists the dedicated search tools the consumer's harness build
carries, with a default of `(Glob Grep)` {design-bearing}. Rule 9 fires only when `Glob` is a member,
and rule 11 only when `Grep` is; with the member absent the rule is inert. Each firing corrective
adds the allowlisted bare fallback: `find` for Glob, `grep -rn` for Grep. **Applied.**

**Why a knob, when the guard cannot see the toolset.** The payload's documented field set carries no
toolset, and `agent_type` does not determine one. The absence of Grep and Glob is a property of the
harness build, which is installation-level, so a consumer config value is the tier it belongs to.
**The default names the tools, and the seam permits it.** A harness tool name is public and shared by
every consumer of that harness. §The generic ruleset's portability paragraph already rules that such
a literal costs portability, not privacy. The default keeps every existing consumer's behavior
byte-identical.

**This repo sets the knob empty** in `scripts/guard-config.sh`. Before landing that line, build
confirms the absence in a **main** session with `ToolSearch 'select:Grep,Glob'`. This stage probed a
dispatched session only, and a build that carries the tools in one session class and not the other
would make the empty value wrong there.

### (5) Rule 18's roster membership reads a per-member declaration of write and execute forms

A new associative knob, `GUARD_KIT_RO_FORMS`, maps a roster binary to its declared **write and
execute forms**. A segment led by that binary is read-only only when its words carry none of them
{design-bearing}. **Not yet applied.**

**The declaration grammar** is space-separated tokens, each decidable from its own shape:

| Token shape | Matches |
|---|---|
| `none` | nothing: the member has no write or execute form |
| `--long` | `--long` and `--long=…` |
| `-x` (one letter) | `-x` standalone and any short cluster carrying `x` (`-uo`) |
| `-word` (dash and two or more letters) | that exact word (`find`'s predicates) |
| `pos:N` | the segment carries N or more non-option words |

The `pos:N` count treats every non-option word as positional. An option's argument can therefore
over-count, which withholds and never grants: the safe direction.

**The lookup reads three tiers:**

1. The consumer's `GUARD_KIT_RO_FORMS[bin]` when set.
2. Else the kit's declared table for that binary.
3. Else **undeclared**, and an undeclared member is withheld. Its segment is not read-only and the
   call falls through to the harness's own decision.

A missed form costs a prompt and never a hole. That is the property the entry's *a gate refuses a
member added undeclared* candidate was proposed for, and it is refused on this ground: guard-kit
registers no gates, and withholding at call time delivers the property without minting the kit's
first registered surface over a consumer config file.

**The kit table for the default roster** is a `lib/guard.sh` literal. It is tool mechanism, the same
class as rule 9's action list, so it crosses no seam.

- `sort`: `-o --output --compress-program`.
- `uniq`: `pos:2`.
- `find`: `-delete -exec -execdir -ok -okdir -fprint -fprint0 -fprintf -fls`.
- `rg`: `--pre`.
- `xargs`: `none` for its own options. The command it runs is re-tested against that command's own
  declaration.
- Every other default member — `grep egrep fgrep head tail cat wc cut tr nl rev tac paste comm column
  diff jq ls` — is `none`.

Two stated non-forms: `sort -T` writes transient temporary files, and `rg -z` runs fixed
decompressors. Neither writes a named file or runs a named program.

**One reader, four rules.** `_guard_is_ro_segment` applies the declaration, so rule 15's exemption
(3), rule 18, and rule 19's clauses (c) and (d) all narrow together, and `_guard_is_ro_xargs` applies
it to the command `xargs` runs. Rule 18's inline `find` action test retires into the table: it named
four of the nine predicates, which is why `-fprint` was granted. Rule 9's listing test reads `find`'s
declaration, so the action roster has one literal.

**The honest limit.** The grant is exactly as safe as the member's option surface as declared. A
binary that gains a write option in a later release is a hole until its declaration is widened, and a
platform whose spelling differs is a hole until it is declared too. Rule 18's closing sentence — *that
weaker predicate is a separate, already-filed gap this rule inherits* — is replaced by this paragraph.

### (6) Rule 22 widens to `git rm` carrying its force flag

Rule 22 gains an arm: a `git rm` segment whose options carry `-f` or `--force`, standalone or in a
short cluster (`-qf`, `-rf`), is **blocked** {mechanical}. The corrective names three exits. **Not
yet applied.**

- Drop the flag: `git rm` refuses a file with local modifications and says so.
- Use `git rm --cached` to untrack the file and keep it.
- Where the loss is intended, run it through `!<command>`.

The subcommand is found by `_guard_git_subcommand`'s walk.

**Why rule 22 owns it.** Rule 22 steers every tracked-path deletion into `git rm -q`, and the force
flag is the one spelling of that mandated form that destroys uncommitted work. So the arm is the
steer's own safety half. It fires whether or not a grant matches, on rule 20's reasoning. Ungranted,
the call is decided out of band anyway. Granted, as `Bash(git rm -q *)` grants it here, it is silent
data loss.

### (7) New rule 24: a committed grant matched only by reaching past its path slot

A new rule sits immediately before fall-through logging. It **blocks** a segment that a committed
`Bash(…)` allow pattern matches, when that pattern's **path slot** absorbs text reaching outside the
path the pattern names {design-bearing}. **Not yet applied.**

**The path slot.** A `*` in a committed pattern is a path slot when the whitespace-delimited token
of the pattern containing it also contains a `/`. In `rm -rf .tmp/*` and `bash */checks/check-*.sh`
every `*` is one. In `git add *` and `bash gate-sdk/bin/run-gates.sh *` none is. The slot is what the
operator wrote, so no knob and no vocabulary are needed: a consumer's `Bash(rm -rf build/*)` is
bounded exactly as this repo's scratch grant is.

**The test, per segment:**

1. **Read the harness view.** Take the segment's dequoted text, meaning quote characters removed and
   content kept, split on the `sq dq hd` skeleton. Apply `_guard_harness_view` (delta 1).
2. **Parse the matching pattern.** For each committed pattern that `guard_allow_match` finds matching
   the view, translate it into an anchored capture expression, with each `*` becoming a group. Take
   the leftmost-greedy parse.
3. **Split each path slot's capture on whitespace.** The **first** word must be clean. Every **later**
   non-option word must also re-match the slot's own token and be clean. A clean word does not begin
   with `/` or `~` and carries no `..` path component.
4. **Decide.** Any unclean word blocks, and the message names the pattern, the offending word and
   what it reaches past (an `..` component, an absolute path, or a second operand outside the slot).
   The corrective is to spell the path inside the pattern's reach or run the command through
   `!<command>`. A match with every slot clean declines, and the harness grants correctly.

**Declines**, in this ruleset's established directions:

- An expansion, substitution or backtick anywhere in the command.
- A pattern carrying `?` or a bracket class, which is skipped.
- A dequoted segment carrying a statement separator.
- No `jq`, no settings file, or a parse error, on `_guard_allow_inners`' fail-open read.

**It closes both entries' reaches.** `rm -rf .tmp/../<file>` and `rm -rf .tmp/x <local file>` block
against the scratch grants. `bash ../../evil/checks/check-x.sh` and
`bash /elsewhere/checks/check-x.sh` block against the script-runner globs. The committed globs stay
exactly as they are: the grant-path entry's open question, *can the globs be re-spelled without
breaking the battery*, is dissolved rather than answered, because nothing re-spells them.

**Why a guard rule and not a committed deny list.** A deny list matches per subcommand and outranks
a hook's allow, which makes it the one harness-enforced layer. It has three disqualifying
properties:

- A deny glob cannot express *a second operand outside the directory*.
- It inherits the fragility the reference names.
- It is a permission-settings edit only the operator applies (CLAUDE.md), so the mechanism choice
  would decide that no session can land the fix.

The guard rule is derived from the settings it bounds, is testable in the decision table, and lands
in build. A deny list stays available as a harness-enforced backstop the operator may add
independently. It is not this unit's work and is not queued by it.

**Placement.** No auto-allow above it grants a rule-24 subject:

- Rule 16 truncates only.
- Rule 17's emitter bound refuses `rm` and `bash`.
- Rule 18's roster carries neither.
- Rule 19's arm (A) body is `sleep` only, and its arm (B) applies this rule's test as a predicate
  (delta 9).

Rules 4 and 7 run ahead of this rule and each emit `guard_rewrite`, whose envelope carries
`permissionDecision: allow`. Both apply this rule's test as a predicate before that allow attaches
(delta 12), so no rewrite grants a rule-24 subject either.

**Honest limits:**

- The test is lexical, so a symlink under the slot reaches past it undetected.
- The leftmost-greedy parse is one parse of a pattern that may admit several.
- A wrapper option the harness view declines hides a reach the matcher would grant.

### (8) Rule 15 refuses an unrecorded launch

Rule 15 **blocks** a backgrounding launch that writes no record and meets no exemption, where it
advised before {design-bearing}. Both arms, the record-writing test and the three exemptions stand,
with two changes. **Not yet applied.**

- **Exemption (2)'s honest limit is stated in the rule.** A wait loop is recognized only when it is
  spelled inline. A wait behind a script or binary name is invisible to the span walk. It takes the
  record like any launch, and the record's only effect on a waiter is that rule 14 holds tracked-tree
  mutations for the waiter's lifetime, which the producer it waits on already holds.
- **Exemption (3)** reads the declared-forms test (delta 5).

**The corrective names one canonical spelling** and states that rule 19 grants it for an allowlisted
command (delta 9):

```
<command> [<redirects>] & echo "pid=$! run=<key>" > <scratch-dir>/<key>.run; wait; rm -f <scratch-dir>/<key>.run
```

- The `wait` keeps a harness-backgrounded call alive until the child exits, so the completion
  notification means the producer finished.
- The trailing `rm -f` retracts the record at exactly that moment, which is delegation-kit's
  *delete once exited, and not before* rule taken mechanically rather than remembered.

The advisory's closing sentence is corrected to say an inline wait loop and a read-only pipeline owe
no record. It no longer says *a backgrounded wait loop*.

**The paragraph *Advises rather than blocks* is replaced**, and the replacement states two things
**undated**. The re-opening condition that paragraph named — a firing of the omission after the
advisory shipped — is met, so the block is taken. And the false-fire objection it rested on is
answered by cost rather than denied:

- A launch of an allowlisted command complies at no decision cost, through delta 9.
- What remains is launches behind a name the exemptions cannot read, and launches of ungranted
  commands. Each is one re-issue with the record, the second meeting the out-of-band decision it
  would have met anyway.

**The honest limit paragraph is replaced.** The block closes the silent case at every launch a
`PreToolUse` call can see. It does not reach **a command the harness moves to the background on its
timeout**. That call was a foreground call at `PreToolUse`, it writes no record, and no `PreToolUse`
payload carries task state.

The one wired chokepoint holding any view of it is the turn end. The `SubagentStop` payload's
`background_tasks` array lists harness-launched shell tasks with a `status`. delegation-kit rules
that this view may supplement the record set and never substitute for it, because it misses a child
detached with a shell `&` (§What `background_tasks` carries).

Refusing a turn end on a running harness shell task would widen delegation-kit's refusal set, which
is not this amendment's to take. The moved-on-timeout launch is therefore rule 15's stated residue,
and its detector is queued separately as `harness-moved-background-task-unrecorded`.

**Rule 15 keeps its number and its placement.** It sits before the auto-allow band, and rule 19's
arm (B) grants only what rule 15's exemption (1) has already let through.

### (9) Rule 19 gains arm (B): the recorded launch

Rule 19 becomes **auto-allow a sanctioned wait**, in two arms {design-bearing}. Arm (A) is the
bounded in-turn wait exactly as it stands, clause (0) included. Arm (B) is new, and grants delta 8's
canonical launch when **every** clause holds. **Not yet applied.**

- **(B0) The shape.** The call is exactly three or four statements:
  1. a launch ending in a statement-ending `&`;
  2. `echo "pid=$! run=<key>" > <dir>/<key>.run`, with `<dir>` a `GUARD_KIT_SCRATCH_DIRS` member,
     `<key>` matching `[A-Za-z0-9._-]+`, and the filename key equal to the `run=` key;
  3. `wait`;
  4. optionally, `rm -f` of that same record path.
- **(B1) The launched command is granted and bounded.**
  - The launch segment, redirects removed, is one segment with no pipe.
  - It matches the committed allowlist through `guard_allow_match` on `_guard_allow_inners`' fail-open
    read.
  - It passes rule 24's test (delta 7), rule 22's tracked-path test and rule 23's scratch-body
    resolution, each taken as a predicate rather than as a block.
  - Arm (B) therefore grants nothing a later rule would refuse.
- **(B2) The launch writes only gitignored or inert targets**, on rule 17's exact
  `git check-ignore --quiet --` predicate and its inert-target carve-out.
- **(B3) Conservative decline.** A command or process substitution or a backtick anywhere, or any
  expansion other than the single `$!` inside statement 2's double-quoted argument.

**Why the grant is owed.** The record's builtin spelling carries `$!`. Rule 6's expansion match does
not reach that special parameter, so the call passes the guard, but the matcher refuses expansions.
Delta 8's block would therefore convert every compliant launch into an out-of-band decision, and a
floor whose corrective always costs a decision is the one sessions learn to skip.

**The grant adds no capability.** The launched command is one the allowlist already grants. The
record write is the gitignored-target write rule 17 grants with an emitter it already rosters. `wait`
is a builtin, and the `rm -f` retracts the one file the same call created.

**delegation-kit's refusal of a record-writing tool stands.** The write stays a shell builtin, as
that refusal requires. What changes is that the builtin now costs no decision, which answers the
refusal's stated cost without reversing it.

**Placement.** Arm (B) rides rule 19's existing position — after rules 12 through 15, before rule
20 — which is also the position it needs. Rule 15's exemption (1) lets the recording launch through
first. The probe showed rule 20 does not block the canonical form, but arm (B) sits ahead of it
regardless. No number moves.

### (10) The test lanes

The decision table, the knob test and the parity corpus take every behavioral change above
{mechanical}. §Testing's non-monotone rule binds throughout: every row whose expected column a delta
could move is re-derived, never assumed. **Not yet applied**, except the parts for deltas 1 to 4:
the rule 2 and rule 8 rows, the `GUARD_KIT_SEARCH_TOOLS` cases of the knob test, the `harness-view`
parity corpus, and the `scan_prompts` key test.

- **`guard-tests/cases.tsv`:**
  - Firing and non-firing rows for rule 2's arms (b) and (c) (delta 2), rule 8's two `awk` shapes and
    a declined transform (delta 3), rule 22's force arm (delta 6), rule 24's `..`, absolute and
    second-operand arms with a clean-slot pass (delta 7), and rule 19's arm (B) grant with a
    non-allowlisted launch declining (delta 9).
  - Every existing rule 18, 19 and 15 row carrying a roster binary re-derived (delta 5).
  - Every row whose command carries a trailing `&` re-derived, since advise becomes block (delta 8).
- **The table's sandbox allowlist grows beyond its three entries.** It gains a scratch `rm -rf` slot
  grant and a `*/checks/check-*.sh` runner grant, so rule 24's rows have a pattern to bound, plus the
  entries delta 12's rewrite rows need.
  Rule 18's allowlisted-lead widening and rule 20 read the same file, so their rows are re-derived
  too.
- **`guard-tests/background-cases.tsv`:** the harness-form no-record row flips from advise to block,
  and a row for a harness-form canonical launch asserts rule 19's grant.
- **A new bespoke test, `gate-tests/guard-config-knobs.test.sh`,** carries what a `decision <TAB>
  command` row cannot express: a sandbox `guard-config.sh` selected through `GUARD_KIT_CONFIG_FILE`.
  Its cases:
  - `GUARD_KIT_SEARCH_TOOLS=()` leaves rules 9 and 11 inert (delta 4).
  - A consumer roster addition with a `GUARD_KIT_RO_FORMS` declaration grants.
  - The same addition undeclared is withheld (delta 5).

  guard-kit already carries a `gate-tests/` directory, so no fixture-runner registration is owed.
- **`gate-tests/guard-lib-parity.test.sh`** gains the `harness-view` corpus. It covers every wrapper
  in its separate and glued argument spellings, nesting, an unrecognized option stopping the strip,
  `command -v`, `xargs` with and without a flag, and an assignment run (delta 1).
- **`native/src/emit/scan_prompts.rs`' in-crate key test** for `sudo timeout 30 git log` is
  re-derived, because the matcher does not strip `sudo` (delta 1). `gate-tests/scan-prompts.test.sh`'s
  substring assertions are re-run rather than assumed.

### (11) The renumber, the knob surfaces and the mirror

Fall-through logging renumbers from 24 to 25, and the knob and mirror surfaces follow the deltas
above {mechanical}. **Not yet applied**, except the parts for deltas 1 to 4: the
`GUARD_KIT_SEARCH_TOOLS` layout bullet, template line and README sentence, the knob test's layout
line, and the mirror regenerated for them.

- **The renumber has three sites:** the ruleset's own list, `cases.tsv`'s section comment, and
  §scan-prompts' `(rule 24)`, which becomes `(rule 25)`. `lib/guard.sh`'s `guard_generic_rules` gains
  the new rule's call immediately before its last line.
- **§Layout and configuration** gains `GUARD_KIT_SEARCH_TOOLS` and `GUARD_KIT_RO_FORMS`, each with its
  default and its reader, plus the new test file in the layout tree.
- **`templates/guard-config.sh`** gains a commented example line for each knob.
- **`README.md`'s** knob sentence names both knobs.
- **The on-site mirror** regenerates with
  `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

### (12) Rules 4 and 7 attach allow to a rewrite only when the rewritten command is granted and bounded

A `guard_rewrite` emits `permissionDecision: allow`, so a rewrite is a grant, and rules 4 and 7 each
issue one unconditionally {design-bearing}. Both keep their rewrite and attach it only when the
**rewritten** command passes two tests; otherwise each **blocks** with its corrective. **Not yet
applied.**

- **The allowlist test.** Every segment of the rewritten command, split as §The guard framework's
  splitter splits it, matches a committed `Bash(…)` pattern through `guard_allow_match` on
  `_guard_allow_inners`' read. A missing `jq`, a missing settings file or a parse error fails the
  test, so the rule blocks rather than grants: a grant resting on a settings read never turns a
  missing file into an allow, which is rule 18's contract for its own settings read.
- **The bound.** Each segment passes rule 24's test (delta 7), taken as a predicate.

**Rule 4.** The rewrite of an absolute repo-script spelling to its relative form attaches allow only
when both tests hold. Otherwise the existing corrective block fires — use the repo-relative form —
so the re-issued relative command meets every later rule, rule 23's scratch-body refusal included.
`bash <root>/.tmp/check-evil.sh` now blocks.

**Rule 7.** The bare `{}` placeholder rewrite attaches allow only when both tests hold. Otherwise it
blocks, with a corrective naming the quoted `'{}'` spelling for the session to write itself, so the
call reaches the harness's own decision. `find . -type f -exec rm -rf {} \;` and
`ls | xargs -I{} rm {}` now block.

**Why a block and not a fall-through.** An unrewritten `{}` meets the matcher's brace refusal, and
the reference does not document an `updatedInput` carried without a decision, so a fall-through
cannot deliver the rewrite. The corrective hands the session the spelling the rewrite would have
produced: the same behavior-preserving respelling, without the grant.

**What the narrowing costs.** A placeholder call whose command is not allowlisted, or an absolute
script spelling whose relative form is not, now costs a block and a re-issue where it was granted
silently. That is the price of the grant having been unconditional. An allowlisted, bounded call is
unaffected.

**The table.** Rule 4's rewrite row and rule 7's placeholder rows are re-derived. The sandbox
allowlist gains the entries the rewrite rows need to keep asserting the grant, and firing rows assert
the block for an unallowlisted and for an unbounded rewrite, under §Testing's non-monotone rule.

## The seam, ruled

- **Kit mechanism:**
  - Every rule change above.
  - The harness wrapper set (delta 1) and the search-tool default names (delta 4), both harness
    behavior.
  - The declared-forms table for the default roster (delta 5), tool mechanism.
  - The path-slot definition (delta 7).
- **Consumer config:**
  - `GUARD_KIT_SEARCH_TOOLS`'s value for a build without dedicated search tools.
  - `GUARD_KIT_RO_FORMS` entries for roster members a consumer adds.
  - The committed allow patterns rule 24 reads, which the consumer already owns.
- **Private rule content:** none. No delta reads a project vocabulary. This repo's own entries are
  config values, the search-tool knob's emptiness among them, and no project name enters a kit
  literal.

## Producers and consumers

- **The harness view** (delta 1).
  - Producer: `_guard_harness_view` over a segment of the payload's `tool_input.command`, reached on
    every guarded Bash call through `templates/bash-guard.sh`.
  - Consumers: rule 24 at call time, and, through the twin, the ranker's grant test and ranking key at
    `--emit scan-prompts`.
  - The parity mode's one line has three fields — mode, command, view — and one reader, the parity
    test's line comparison.
- **`GUARD_KIT_SEARCH_TOOLS`** (delta 4).
  - Producer: the consumer's `guard-config.sh`, or the kit default in `lib/guard.sh`, resolved when the
    lib is sourced.
  - Readers: rule 9's and rule 11's firing tests, one read each per call.
  - Enabling config: this repo's `scripts/guard-config.sh` line, after delta 4's main-session probe.
- **`GUARD_KIT_RO_FORMS` and the kit table** (delta 5).
  - Producer: the consumer's config for added members, and the `lib/guard.sh` literal for defaults.
  - Reader: `_guard_is_ro_segment` at rules 15 (exemption 3), 18 and 19 (clauses c and d), plus
    `_guard_is_ro_xargs` for the command `xargs` runs.
  - The knob is not declared by any bridged arm, so no crate reader exists and no second resolver is
    owed.
- **Rule 2's arms, rule 8's `awk` arm, rule 22's force arm and rule 24** (deltas 2, 3, 6 and 7).
  - Producer: the payload command.
  - Consumer: the calling session, through `guard_block`'s stderr and exit 2.
  - Rule 24 additionally reads `GUARD_KIT_SETTINGS` through `_guard_allow_inners`, whose producer is
    the operator's committed settings.
- **Rule 15's block** (delta 8).
  - Producers: `tool_input.run_in_background` through `guard_input_field`, and the skeleton's
    statement-ending `&`, both existing.
  - Consumer: the calling session, through exit 2.
- **Rule 19 arm (B)'s grant** (delta 9).
  - Producer: the payload command.
  - Consumer: the harness, through `permissionDecision: allow`.
  - Deny and ask rules still apply on top of it, per the reference.
- **Rules 4 and 7's conditional rewrite allow** (delta 12).
  - Producer: the payload command, rewritten by each rule as today.
  - Readers before the allow attaches: `_guard_allow_inners`' committed allowlist (a failed read
    blocks) and rule 24's test taken as a predicate.
  - Consumer: the harness, through `permissionDecision: allow` with `updatedInput` when both hold,
    and otherwise the calling session, through `guard_block`'s stderr and exit 2.
- **The canonical record** (deltas 8 and 9).
  - Producer: statement 2 of the launching command, at launch.
  - Readers, all unchanged and all reading the existing `pid=<n> run=<key>` grammar:
    - rule 14's `_guard_live_run_records`, at a tracked-tree mutation;
    - `check-producer-liveness` in set mode, at a stage entry;
    - the `subagent-stop-liveness` hook, at a dispatched turn end;
    - the launching session's own wait, in turn.
  - Retraction: statement 4, when the child exits.
  - No field is added, so no reader is owed.
- **Narrowed corpora, with each reader's red condition** (point 5). Deltas 5, 7 and 8 narrow what the
  guard grants or passes.
  - **The decision table reds on a verdict mismatch in either direction.** It is not monotone, so its
    rows are re-derived (delta 10).
  - **`kpi-prompt-friction` asserts only `^[0-9]+/[0-9]+$` and reds on no count.** Calls newly
    withheld reach the friction log and raise its prompting count, so the step is recorded rather
    than inferred (delta 1's KPI paragraph).
  - **`gate-tests/scan-prompts.test.sh` reds on its substring and shape assertions.** It is re-run
    rather than assumed.
  - **Rule 15's exemption (3) and rule 19's clauses (c) and (d)** narrow with rule 18. Each is a grant
    or an exemption, so a narrowing yields a block or a fall-through and never a new grant; the table
    rows for each are re-derived all the same.

## Existing sections updated

- `guard-kit/SPEC.md` §The guard framework (`lib/guard.sh`) — the harness view joins the helper
  roster, and the twinned-primitive count and the duplication paragraph go from four to five
  (delta 1).
- `guard-kit/SPEC.md` §The generic ruleset — rule 2 (delta 2); rule 8 (delta 3); rules 9 and 11
  (delta 4); rule 18, rule 9's action list and rule 19 (c)/(d) (delta 5); rule 22 (delta 6); the new
  rule 24 and the fall-through renumber (deltas 7 and 11); rule 15, its exemption (2) and (3), its
  advisory-to-block paragraphs and its honest limit (deltas 5 and 8); rule 19 arm (B) (delta 9);
  rules 4 and 7's conditional rewrite allow (delta 12).
- `guard-kit/SPEC.md` §scan-prompts — the ranker's wrapper roster becomes the harness view, the KPI
  step is recorded, and `(rule 24)` becomes `(rule 25)` (deltas 1 and 11).
- `guard-kit/SPEC.md` §Layout and configuration — the two knobs and the new test file (deltas 4, 5,
  10 and 11).
- `guard-kit/SPEC.md` §Testing — the grown sandbox allowlist, the background table's flipped row and
  the knob-test lane (deltas 7, 8, 10 and 12).
- `guard-kit/lib/guard.sh` — every helper and rule change, the kit declared-forms table, the two knob
  defaults and the dispatch line (deltas 1, 2, 3, 4, 5, 6, 7, 8, 9, 11 and 12).
- `guard-kit/guard-tests/cases.tsv` and `guard-kit/guard-tests/background-cases.tsv` — rows and the
  renumbered section comment (deltas 10, 11 and 12).
- `guard-kit/gate-tests/guard-lib-parity.test.sh` — the `harness-view` corpus (deltas 1 and 10).
- `guard-kit/templates/guard-config.sh` and `guard-kit/README.md` — the two knobs (deltas 4, 5 and
  11).
- `native/src/guard.rs`, `native/src/main.rs` and `native/src/emit/scan_prompts.rs` — the twin, the
  parity mode and its usage line, and `strip_decoration`'s body and key test (deltas 1 and 10).
- `delegation-kit/SPEC.md` §The delegation model — *now advised at the launch, and still not refused*
  and *What ships is an advisory, not a block* become the refusal at the launch and the grant that
  makes compliance cost nothing (deltas 8 and 9).
- `delegation-kit/SPEC.md` §The turn-end liveness hook — *guard-kit rule 15's advisory residue*
  becomes the moved-on-timeout residue (delta 8).
- `scripts/guard-config.sh` — `GUARD_KIT_SEARCH_TOOLS=()` after the main-session probe (delta 4).
- `docs/guard-kit/SPEC.md`, `docs/guard-kit/README.md` and `docs/delegation-kit/SPEC.md` — the
  generated mirror (all deltas).

## Retired spellings

- None — no delta replaces one literal with another: every rule keeps its name, both knobs are new,
  `strip_decoration` keeps its name while its body changes, and the one renumber (fall-through
  logging, 24 to 25) is the renumber slice `check-amendment-retired-spelling` states as a non-target,
  whose sites delta 11 enumerates instead.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and a
      named consumer; every new field has a named reader at a named transition.
- [ ] **The decision table, the knob test and the parity test are green**, with every row a delta
      could move re-derived rather than carried (§Testing's non-monotone rule).
- [ ] **The KPI step is recorded** in §scan-prompts with its before and after readings.
- [x] **This repo's search-tool knob is probed before it lands**, in a main session.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); the merged spec reads as one coherent document a reader who never saw
      the amendment can use alone, and no dated grounds from this file's evidence section land in a
      kit SPEC.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings`
      above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked
      tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
- [ ] **The battery and `bash gate-sdk/bin/build-native.sh` both pass**, since the crate changes.
