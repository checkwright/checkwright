# SPEC amendment: permission-grammar

Pairs `permission-grammar-neutral`. Kit prose states the shipped harness's permission-rule grammar as the kit's own: the `Bash(…)` rule wrapper, the settings file's `permissions.allow` key, the closing `:*` prefix idiom and the matcher's wrapper strip. gate-sdk/SPEC.md §The hook contract states the hook protocol in roles behind one binding table, and that table has no permission row, so an adopter on another harness reads guard-kit's allowlist sections as one vendor's settings format. This amendment does five things:

- **It states the permission rules in roles in §The hook contract and spells the shipped binding's grammar in one new binding-table row** (delta 1).
- **It names the permission rules in the adopter constraint's master-harness bullet** (delta 2).
- **It rewrites guard-kit's four owning passages** to the roles: the triage criterion, the recommended allowlist, the harness view and the allow match (delta 3).
- **It sweeps every other line the widened gate reds**, one disposition per line (delta 4).
- **It widens this repository's `GATE_SDK_HARNESS_LITERALS`** so `check-harness-literal` reds a grammar spelling that leaves the table, and takes the fixture trees out of that gate's corpus (delta 5). The catcher lands with the fix.

**Reach: gate-sdk, guard-kit, context-kit, queue-kit and this repo's knob file.** The amendment is cross-component, so the audit stage follows spec. It sits at the repository root because it spans several components (canon-kit/templates/SPEC-amendment.md).

**The row's scope, decided at spec within the entry's corpus.**

- **What it covers.** These are the three parts the entry names, plus the key the rules sit under:
  - the rule wrapper, a tool's name around a pattern;
  - the prefix idiom;
  - the wrapper strip;
  - the allow list's settings key.

  They are the spellings guard-kit's readers parse and its allow match models.
- **What it leaves out:**
  - The settings file's name and location. Config already owns them (`GUARD_KIT_SETTINGS`, `GUARD_KIT_SETTINGS_LOCAL`, `CONTEXT_KIT_SETTINGS_FILE`), and the adopter constraint names them generically already.
  - The permission modes and their `defaultMode` key, which guard-kit/SPEC.md §The reader and its views argues about but no member parses.
  - The harness's deny and ask rules, which no kit surface reads.
  - The headless measurement invocation (`claude -p …`). It is a measuring tool's command line and not a grammar spelling, and it stays beside the records it produced.

**The measured matcher records stay where guard-kit models them.** These are the compound split's separator class and escapes, and the allow match's five measured cases. They argue the model they pin, and the allow match is the one holder of that model (guard-kit/SPEC.md §The reader and its views). Moving them into gate-sdk would split a model from its evidence. So their site keeps the spelling under a valve as the adapter's own record, and the binding table's measured record cites it (delta 1). The payload record moved into the table under `hook-contract-harness-neutral`, and it differs: many members read it, so it had no single model to sit beside.

**The row was read off the published contract.** It was fetched at spec rather than recalled (the harness's permissions documentation, `code.claude.com/docs/en/permissions`). That page states:

- rules take the form `Tool` or `Tool(specifier)` under `permissions.allow`;
- `:*` is recognized only at a pattern's end, as a trailing ` *`;
- the stripped wrappers are `timeout`, `time`, `nice`, `nohup` and `stdbuf`, the builtins `command` and `builtin`, zsh's `noglob`, a leading assignment of known-safe variables, and a bare `xargs`;
- PowerShell rules take the same shape, spelled `PowerShell(<pattern>)`.

guard-kit's harness view rosters the same set, so no roster edit is owed.

**The worklist is the oracle's, not a hand count.** The entry recorded 22 `Bash(` lines in guard-kit/SPEC.md and one each in gate-sdk and context-kit, and its grep misses queue-kit, a guard-kit template and the `permissions.allow` key. Run at spec (revision `92ee5d70c`), with delta 5's roster and corpus set through the environment, `check-harness-literal` reds **33** lines:

| file | lines |
|---|---|
| guard-kit/SPEC.md | 26 |
| context-kit/SPEC.md | 2 |
| guard-kit/templates/close-triage.md | 2 |
| gate-sdk/SPEC.md | 1 |
| guard-kit/README.md | 1 |
| queue-kit/SPEC.md | 1 |

The command, the current roster and corpus with delta 5's additions appended:

```text
GATE_SDK_HARNESS_LITERALS="<current roster> Bash( PowerShell( permissions.allow" GATE_SDK_HARNESS_LITERAL_PATHS="<current paths> :!*/gate-tests/*" ./native/target/release/checkwright-gates --only check-harness-literal
```

Without the exclusion it reds a 34th line, `scripts/gate-tests/check-guard-registration/good/guard-kit/SPEC.md:18`, a fixture (delta 5).

**The prefix idiom is swept by hand, and no gate holds it.** A byte match on `:*` finds every bold lead ending in a colon, so delta 5 leaves it unlisted. Its code-span uses come from ``git grep -nE ':\*[`)]' -- guard-kit/SPEC.md``, which lists eleven lines. Six of them carry a `Bash(` and sit on the oracle's worklist; the other five are delta 4's last group.

## What changes

### (1) gate-sdk states the permission rules in roles and spells them in the binding table {design-bearing}

`gate-sdk/SPEC.md` §The hook contract. **Not yet applied.** Four edits:

- **A role bullet, after the transcript bullet:**

  > - **The permission rules.** The settings file's *allow list* holds *rules*, each a tool's name wrapping a *pattern*. A *shell rule* names a shell tool, and a `*` in its pattern spans any text. The harness matches a shell rule against each segment of a compound command after its *wrapper strip*, and a closing *prefix idiom* matches the head alone or the head and a space-led rest. No member speaks this protocol. guard-kit's readers parse the allow list, and its allow match models the matcher (guard-kit/SPEC.md §The reader and its views). No kit surface writes a rule (guard-kit/SPEC.md §The recommended allowlist).

- **A binding-table row, after the Transcript row, valved as its siblings are:**

  > - Permission rules: allow list `permissions.allow`; a rule `<Tool>(<pattern>)`, the shell rules `Bash(<pattern>)` and `PowerShell(<pattern>)`; prefix idiom a closing `:*`; wrapper strip, the documented set guard-kit/SPEC.md §The reader and its views' harness view models. <!-- harness-binding: the shipped binding table -->

- **The measured record, replacing its paragraph:**

  > **The measured record.** The payload rows above were read off the harness's published hook contract and off recorded firings, and the permission row off its published permission rules, each fetched rather than recalled. That row's matcher behavior is measured where guard-kit models it (guard-kit/SPEC.md §The reader and its views). A harness revision reshaping either is drift no gate here detects, so only re-reading the contract catches it.

- **The honest limits, replacing their paragraph:**

  > **Honest limits.** The crate spells each member's payload paths and envelopes, and each allow-list reader its rule wrapper, in that module's own code. So no gate holds this table to the code: a renamed key reds a member's fixture, not this section. The table names the payloads the members read and not every field a harness sends. It names the permission spellings the kit reads, not the harness's whole permission grammar.

The section's opening sentence ("Every hook member speaks one contract") stays. The role bullet states that no member speaks the permission protocol, so the contract stays the members' and the row is the one spelling site of a second protocol the kit reads.

### (2) The adopter constraint names the permission rules {mechanical}

`gate-sdk/SPEC.md` §The adopter constraints, the bullet **The master harness is a binding, never an assumption.**, its second sentence. **Not yet applied.** It reads:

> Kit prose names a hook member's protocol, the project directory, the session and config-home variables, the dispatch tool's parameters, the permission rules and the transcript's shape by their roles in §The hook contract, whose binding table is the one site spelling Claude Code's binding.

### (3) guard-kit's owning passages name the roles {design-bearing}

`guard-kit/SPEC.md`. **Not yet applied.** Four passages:

- **§The triage criterion, the allowlist bullet:**

  > - **Allowlist** (a shell rule in the committed settings, gate-sdk/SPEC.md §The hook contract) when the command is safe *and already in the form to reinforce* — static, glob-matched, declarative.

- **§The recommended allowlist, the opening paragraph's first two sentences:**

  > `templates/settings-allow.json` is a recommendation to merge by hand, entry by entry, into the committed settings file's allow list, as a **union**: an object merge that replaces the array drops every grant the consumer already had. Its entries are spelled in the shipped binding's rule grammar (gate-sdk/SPEC.md §The hook contract), so an adopter on another harness writes each pattern as its own binding's shell rule.

  The paragraph's remaining sentences stay.

- **§The reader and its views, the harness view's lead paragraph:**

  > **The harness view** is a segment as the harness's permission matcher reads it: the leading wrappers the matcher strips before it matches a shell rule (gate-sdk/SPEC.md §The hook contract), removed from the head **repeatedly**. It is rostered here because it models harness behavior every reader of it must share. The stripped set is the shipped binding's documented one, and each wrapper's arguments are walked by its own grammar:

- **§The reader and its views, the allow match's lead paragraph.** "It models the harness `:*` prefix idiom as the harness grants it" reads "It models the shipped binding's prefix idiom, a closing `:*` (gate-sdk/SPEC.md §The hook contract), as the harness grants it". The line keeps its examples and takes the valve `<!-- harness-binding: the allow match models the shipped binding's matcher -->` at its end, since it is the adapter that reads the spelling.

### (4) Every other red line takes one disposition {design-bearing}

Every line the widened gate reds, beyond delta 3's, takes one of two dispositions. The test is whether the sentence states *what a reader parses* or names the grammar as a subject:

- **(a) A sentence naming the grammar as a subject** is rewritten to the role, citing gate-sdk/SPEC.md §The hook contract once per section. An example pattern is spelled bare, as the pattern the rule wraps: `git status *`, not `Bash(git status *)`. A prefix-idiom example is named as "a prefix-idiom grant of `git log`". The rewrite re-phrases, never appends.
- **(b) An adapter site** keeps the spelling under a `harness-binding: <reason>` valve, on the line or the line above. It is one of three kinds:
  - a sentence stating what a reader parses;
  - a measured record of the shipped matcher;
  - a wiring instruction an adopter pastes.

The lines at revision `92ee5d70c`, from the oracle's output, each with its disposition. A line delta 3 rewrites is listed there, not here:

| site | section | disposition |
|---|---|---|
| guard-kit/SPEC.md:199 | §The reader and its views, the separator class measured | (b), measured record |
| guard-kit/SPEC.md:205 | same, the escapes measured | (b), measured record |
| guard-kit/SPEC.md:236–240 | the allow match's five measured cases | (b), measured record, one valve per bullet at its end |
| guard-kit/SPEC.md:287 | §Consumer rules, `allow-entries` | (b), parses the key |
| guard-kit/SPEC.md:344 | §The generic ruleset, the matcher-ground steers | (a): "the spelling a shell rule for the bash tool grants … the kit recommends no rule for the PowerShell tool" |
| guard-kit/SPEC.md:382 | rule `allowlist_chain` | (a): "a committed shell rule" |
| guard-kit/SPEC.md:419 | rule `script_interpreter` | (a): "a committed `python3 -*` grant" |
| guard-kit/SPEC.md:544 | rule `append_scratch` | (a): "a shell rule grants the command and never the target" |
| guard-kit/SPEC.md:593 | rule `bounded_wait` | (a): "no shell rule can match it" |
| guard-kit/SPEC.md:662 | rule `allowlist_chain`, bare leads | (a), both examples bare |
| guard-kit/SPEC.md:719 | rule `grant_path_slot` | (a): "a committed shell rule's pattern", "a consumer's `rm -rf build/*` grant" |
| guard-kit/SPEC.md:896 | §scan-prompts, per-segment matching | (a), the example a prefix-idiom grant |
| guard-kit/SPEC.md:906 | §scan-prompts, what crosses | (a): "the consumer's own allow list" |
| guard-kit/SPEC.md:918 | §scan-prompts, a PowerShell line | (a): "reads the bash tool's rules alone" |
| guard-kit/SPEC.md:923 | §scan-prompts, a write redirect | (a): "no shell rule can grant" |
| guard-kit/SPEC.md:935 | §scan-prompts, the grant test | (a), the example `echo *` |
| guard-kit/SPEC.md:1016 | §check-door-binding, a permission pattern | (b): the gate parses the wrapper |
| guard-kit/SPEC.md:1071 | §compare-settings-allow, the ruling's subject | (a): "the three allow-list readers" |
| guard-kit/SPEC.md:1345 | §Testing, the consumer smoke | (b): the smoke strips the wrapper |
| guard-kit/templates/close-triage.md:5 | the allowlist bullet | (a), as delta 3's triage bullet |
| guard-kit/templates/close-triage.md:7 | the PowerShell section | (a): "its allowlist entry is a rule for the PowerShell tool" |
| guard-kit/README.md:37 | §Install, the union step | (b), a wiring instruction |
| context-kit/SPEC.md:397 | §check-settings-paths | (b), parses the key |
| context-kit/SPEC.md:403 | same, the candidate | (b), parses the wrapper |
| gate-sdk/SPEC.md:855 | §Meta-gate conservation for the binary substrate, the `check-settings-paths` row | (a): "no shell-rule grant invokes a descriptor path" |
| queue-kit/SPEC.md:441 | §The queue verbs | (a): "a shell rule for `<door> --queue promote *` grants that verb alone" |

**The prefix idiom's lines outside the oracle** take disposition (a), "the prefix idiom", with one exception. The exception is guard-kit/SPEC.md:1193, a §Testing fixture roster, which stays as written: no gate lists the idiom, so it needs no valve. The other four are guard-kit/SPEC.md:979, 1028, 1042 and 1399. On 1399, "A `make:*` family" reads "A prefix-idiom family of `make`".

**A sibling unit's new line is the oracle's, whatever the order.** The four guard-kit debt entries of this set rewrite guard-kit/SPEC.md passages, in §The reader and its views, §The generic ruleset and §The rule roster. If one lands first, build re-runs the oracle and its output, not this table, is the worklist. If this unit lands first, a later `Bash(` reds that sibling's commit. Neither order changes an act here.

### (5) This repository lists the grammar's spellings and drops the fixture trees from the corpus {mechanical}

`scripts/gate-sdk-config.knobs`. **Not yet applied.**

- `GATE_SDK_HARNESS_LITERALS` gains `Bash(`, `PowerShell(` and `permissions.allow`. Each is distinctive enough that a byte match finds no kit-neutral word, as the bare tool names are not, and `:*` stays out on the grounds above.
- `GATE_SDK_HARNESS_LITERAL_PATHS` gains `:!*/gate-tests/*`. Every kit's fixture trees and the publisher's `scripts/gate-tests/` hold 108 `SPEC.md`, `README.md` and template members the corpus globs reach (`git ls-files` over the current paths, counting `gate-tests/`). Their bytes are chosen by the gate each fixture tests, and they are not the kit prose the comment names. One of them, check-guard-registration's good fixture, carries `Bash(git status *)` as that gate's non-citation case. The exclusion is git's, as `:!docs/` and `:!plugin/` are.
- The `# spec:` comment above the pair names the fixture trees beside the plugin and the Pages mirror as what leaves by pathspec.

**It lands in the commit that clears the last red line**, so no commit carries the wider roster red. Until then the command above is the sweep's worklist.

## Producers and consumers

- **The permission role bullet and binding row (delta 1).**
  - Producer: this amendment's merge.
  - Consumers:
    - the kit prose citing it (deltas 3 and 4);
    - an adopter writing a binding of their own, who needs the rule shapes the kit's readers parse.
  - Its one roster-holding reader is `check-harness-literal`, which holds the row's listed spellings to the valved lines (delta 5).
  - Point 4: the row's fields each have a reader in the crate. The allow-list key is read by the `scan-prompts` ranker, the `compare-settings-allow` arm, `check-settings-paths` and the consumer-rule host's `allow-entries` mode. The rule wrapper is stripped in `native/src/emit/scan_prompts.rs`, `native/src/guard/host.rs`, `native/src/gates/settings_paths.rs` and `native/src/gates/door_binding.rs`. The prefix idiom is read by `guard::allow_match`, and the wrapper strip by the harness view. `PowerShell(<pattern>)` has no parser: the row carries it because guard-kit's PowerShell triage names it, and it is marked as the second shell tool's rule, not as a read.
  - Probe: `git grep -n -F -e 'Bash(' -e '"permissions"' -e ':*' -- native/src`.
- **The widened roster and narrowed corpus (delta 5).**
  - Producer: this repo's knob file, a deployed configuration.
  - Consumer: `check-harness-literal`, at `precommit` with `trigger=*` and on the battery. The kit's default stays empty, so no adopter meets it.
  - Point 5, a narrowed corpus. `:!*/gate-tests/*` removes members, and deltas 3 and 4 remove literals from prose. Neither red condition is an absence or a count:
    - The gate's red condition is a listed spelling on an unvalved line, monotone in the violation set.
    - Its disabled-clean branch fires on an empty knob, and both knobs stay non-empty.
    - Its own suite drives its own knobs (`scripts/gate-tests/check-harness-literal.test.sh`), so the repo's corpus does not reach it.
  - `git grep -n "HARNESS_LITERAL" -- ':!docs/'` names no other reader of either knob. The audit roster's `unresolvable-walk-root` class lists "a gate's … scan pattern changing" as a due trigger, so the exclusion makes that class due at close. Its corpus is the reads census, and the gate's corpus is a declared pathspec knob, so the census is not expected to move.
- **Point 6.** Delta 4 obliges every line the oracle lists, and its table names each line's disposition. A line the table does not name, or one the build's re-run adds, takes the disposition its test gives, and one neither fits is a gap the build resolves that session.

## Existing sections updated

The roster came from the oracle command above over `GATE_SDK_HARNESS_LITERAL_PATHS`' corpus at revision `92ee5d70c`, with delta 5's additions in the environment, grouped by the heading above each line. The prefix-idiom lines came from ``git grep -nE ':\*[`)]' -- guard-kit/SPEC.md``.

- `gate-sdk/SPEC.md` §The hook contract (delta 1); §The adopter constraints (delta 2); §Meta-gate conservation for the binary substrate, its table row for `check-settings-paths` (delta 4).
- `guard-kit/SPEC.md` §The triage criterion, §The recommended allowlist and §The reader and its views (delta 3), and the sections in delta 4's table (delta 4).
- `guard-kit/templates/close-triage.md` and `guard-kit/README.md` (delta 4).
- `context-kit/SPEC.md` §check-settings-paths (delta 4).
- `queue-kit/SPEC.md` §The queue verbs (delta 4).
- `scripts/gate-sdk-config.knobs` (delta 5).
- The generated mirrors under `docs/` for every SPEC and README above, and `docs/llms-full.txt` (all deltas).
- <!-- update-target-exempt: dated release notes record what shipped and are history, never rewritten --> The dated posts under `docs/posts/` that carry a `Bash(` stay as written.

## Retired spellings

- None — no name is retired; every spelling stays, relocated behind the binding table or valved at its adapter site.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The oracle clean** — `check-harness-literal` green with delta 5's roster and corpus committed, and every kit suite the edited SPECs reach green: gate-sdk, guard-kit, context-kit and queue-kit.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, at a stage before the drain stage.
