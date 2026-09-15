# SPEC amendment: recommended-allowlist

Queue entry: `recommended-allowlist-unshipped`, selected for `guard-friction-reach` (operator direction,
2026-09-15, lead-relayed). The harness-adapter seam was left open for this stage to design.

**It waits on `SPEC-rewrite-arm.md` and `SPEC-perl-rewrite-steer.md` (align finding).** Delta 2's
subsection cites `--rewrite` and `§rewrite` as reachable through the front-end, and rule 8's
`--rewrite` arm as one of the ruleset's own steer targets — both minted by those two sibling
amendments, not yet present in the tracked tree. This unit merges after both land, so its SPEC
prose describes a `§rewrite` section and a rule 8 steer that already exist.

**The seam call, ruled: no adapter component. The recommendation ships as a guard-kit template beside
the hook wiring it completes.** The harness settings format is already a guard-kit template tier:
`templates/settings-hooks.json` is harness-native JSON, hardcoding the front-end path and the hook
event names. docs/install.md §Running under an AGENTS.md harness states that settings stay
harness-native because no cross-harness settings surface exists. guard-kit/SPEC.md §The generic
ruleset rules that a harness's public form names cost portability, not privacy, and that a
configurable slot built before a second harness exists is designed against no case. An adapter
component, meaning a directory or knob translating one allowlist into several harness formats, would
be that slot. The template tier is the adapter this kit already has, and a second harness is what
would earn a second template.

**The candidate's projection, refused on derivation-first's own terms.** The entry's candidate was a
generated, freshness-gated projection built from the registered gates and the rules' steer targets.
Probed, neither input is a derivable roster:

- Every registered gate and every arm is reached through one front-end command, so the gate registry
  contributes one path, not a roster.
- The steer targets live in block-message prose inside `lib/guard.sh`, so a generator would first need
  a steer-target table minted to generate from.

That table would itself be the only copy the freshness gate guards. So the set is hand-authored in one
holder, the template. The SPEC's inline list of banner tools moves to it (delta 3), and the smoke holds
the one invariant a template can break: that the ruleset blocks no form the template grants (delta 4).

**Never written by `init`, and never merged by the kit.** A grant is a per-consumer security decision:
§compare-settings-allow rules a widening of the committed settings file high-impact. The installer
also does not wire guard-kit at all (guard-kit/README.md §Install is a manual step), so no activation
path changes.

## The seam

- **Kit mechanism:** one template (`templates/settings-allow.json`), a SPEC subsection giving each
  entry's ground and reach, an install step, and one smoke assertion. The entries are the kit's own
  front-end path, the ruleset's own steer target, and POSIX tool names §The triage criterion already
  names. None is a class CLAUDE.md §The provenance seam names.
- **Consumer config:** none. No knob is added. The consumer's own settings file is where the choice
  lands, by the consumer's hand.
- **Private rule content:** none in reach. This repo's committed allow list is **not** the source: its
  project entries (`cargo`, `gh`, `shellcheck`, per-script globs) stay out.

## What changes

### (1) The template {design-bearing}

New file `guard-kit/templates/settings-allow.json`. It has a `"//"` key in `settings-hooks.json`'s
register: merge-it-yourself, review each entry first, union into `permissions.allow` and never replace
the array, and a pointer to guard-kit/SPEC.md §The recommended allowlist. The `permissions.allow` array
holds exactly:

- `Bash(bash gate-sdk/bin/run-gates.sh)` and `Bash(bash gate-sdk/bin/run-gates.sh *)`
- `Bash(git rm -q *)`
- `Bash(echo *)`, `Bash(wc *)`, `Bash(grep *)`, `Bash(ls)`, `Bash(ls *)`, `Bash(command -v *)`

### (2) The SPEC subsection: each entry's ground and reach, and what is deliberately absent {design-bearing}

guard-kit/SPEC.md, a new `### The recommended allowlist` after §The triage criterion. **Not yet
applied:**

> `templates/settings-allow.json` is a recommendation to merge by hand, entry by entry, into the
> committed settings file's `permissions.allow`, as a **union**: an object merge that replaces the
> array drops every grant the consumer already had. Nothing in the kit writes it, since a grant is the
> consumer's security decision (§compare-settings-allow's impact criterion rates a committed widening
> high-impact). Each entry is there for a stated reason and reaches a stated set:
>
> - **The front-end, bare and with arguments.** Every gate, every arm and the ruleset's own
>   front-end steer targets are reached through it: rule 8's section extractor and `--rewrite` arm,
>   and the `--scratch-run` runner rules 23 and 26 name. Its reach is the whole front-end. That
>   includes:
>   - `--scratch-run`, which runs a scratch script under that arm's echo control;
>   - `--rewrite`, which rewrites files inside the repository under its own bounds (§rewrite);
>   - `--install-hooks`, which opts the clone into the generated git hooks;
>   - every `--emit … --write` mode that regenerates a tracked projection.
> - **`git rm -q`**, rule 22's steer target, spelled as the steer prints it. Its reach is deleting and
>   staging tracked paths, which is recoverable from the last commit. Rule 22 blocks its force flag
>   whether or not a grant matches.
> - **The banner and diagnostic tools** an agent chains around a core command: `echo`, `wc`, `grep`,
>   `ls` and `command -v`. The harness matches each segment of a compound on its own, so these are
>   what keep a chained core command on the match path. They read or print. A redirect on them is a
>   file write the harness checks against the target, not against these entries. **The criterion
>   for this set is that a utility has no write or execute form of its own**, meaning nothing it can
>   do beyond what the harness separately checks. So `find` (`-exec`, `-delete`), `awk` (`system()`,
>   `print >`), `xargs`, `sed` and every interpreter stay out however habitually they are chained,
>   and the steer for a rewrite is `--rewrite`, not a grant. The front-end and `git rm -q` above are
>   the kit's own steer targets, each granted for its stated reach, not members of this set. **The
>   honest limit** is that a Bash read grant reaches paths a harness file-read rule may deny, so a
>   consumer holding such rules drops `grep` and `wc` here rather than widening around them.
>
> **Deliberately absent, so a reader does not re-derive it.** The harness's built-in read-only `git`
> subcommands need no entry. A write to a gitignored target, a truncation of one and a sanctioned wait
> are granted by rules 16, 17 and 19 from the hook, where no settings entry can reach them. The harness's file tools, the steer targets of rules 8 to 11 and 25, are
> governed by its own permission mode, and an `Edit` or `Write` grant is a write widening this
> recommendation will not make on a consumer's behalf. Rule 2's bare-`git` steer target is one
> subcommand per consumer, and a blanket `git` grant reaches a force push. **The template assumes the
> kits are vendored at the repository root**, as `settings-hooks.json` does, and where
> `check-settings-paths` is registered it holds the front-end literal once merged.

### (3) §The triage criterion's banner list moves to the template {mechanical}

guard-kit/SPEC.md §The triage criterion, the per-segment note. **Not yet applied:** `The read-only
banner/diagnostic tools an agent habitually chains (echo, wc, grep, ls, command -v) are therefore
themselves legitimate allowlist entries` becomes `The read-only banner and diagnostic tools an agent
habitually chains are therefore themselves legitimate allowlist entries — §The recommended allowlist
names them —`. The note's corrective clause (`allowlist them, or run the core command bare`) stands. One
list now lives in one surface, and the new subsection and the template carry it.

### (4) The smoke merges the template and holds the no-contradiction invariant {mechanical}

`guard-kit/smoke/install.sh`, after the hook merge:

- It unions the template's `permissions.allow` into the scratch consumer's `.claude/settings.json`,
  deduplicated, and strips the `"//"` key. A consumer's prior allow entries survive: the union is
  asserted by writing one sentinel entry first and checking it remains.
- For each template entry, it strips `Bash(` and `)`, replaces each `*` with the literal word `x`, and
  feeds the result to the installed `scripts/bash-guard.sh` as hook JSON. It fails if any exits 2.
  That is the invariant a hand-authored set can break: a rule later blocking a form the recommendation
  grants, which is the steer/grant contradiction shipped to every adopter.

guard-kit/README.md §Install step 2 gains a sub-step: review `templates/settings-allow.json` against
guard-kit/SPEC.md §The recommended allowlist, and union the entries you accept into
`.claude/settings.json`'s `permissions.allow`.

### (5) Two sentences that read as forbidding a shipped command name {mechanical}

- guard-kit/SPEC.md §Layout and configuration, `GUARD_KIT_BREADTH_PROBES`: `The kit ships **no**
  default probes: every string naming a command is the consumer's vocabulary, never the kit's
  (CLAUDE.md §The provenance seam).` becomes `The kit ships **no** default probes: a probe names a
  command the consumer calls bad, which is the consumer's vocabulary, never the kit's (CLAUDE.md §The
  provenance seam).`. The old wording was already wider than the tree:
  `GUARD_KIT_RO_BINS` and `GUARD_KIT_SCRIPT_INTERPRETERS` ship command names as defaults, on the
  **the test is the content and never the shape** ground that knob's own bullet states.
- guard-kit/SPEC.md §scan-prompts, **What crosses into the binary**: `ships no default allow entry of
  any kind` becomes `ships no default allow entry of any kind — `templates/settings-allow.json` is a
  recommendation the consumer merges, and this arm never reads it`.

The §Layout and configuration tree listing gains
`templates/settings-allow.json  # the recommended allow entries, merged by hand`.

## Producers and consumers

- **The template** — producer: the vendored guard-kit copy. It is a tracked file under the kit root,
  which gate-sdk/SPEC.md §Consumer payload vendors, as it vendors `settings-hooks.json` beside it.
  Consumers: the adopter
  following README §Install step 2, and `guard-kit/smoke/install.sh`, run by the consumer-smoke validate
  suite. No kit binary or gate reads it.
- **The smoke's invariant** — producer: the smoke's feed loop. Consumer: the validate suite's verdict.
  Its red condition is any template entry's literal form blocked by the installed guard, which is not
  monotone in the template's size. The build clears it by running the smoke.
- **No field, knob or state.** No corpus is narrowed. Delta 3 removes a list from prose that no gate
  reads as a roster.

Derivation of the rosters here and below: `git grep -n "settings-hooks.json"` over the tracked tree
(guard-kit README, SPEC layout, smoke and lifecycle-kit/README.md, whose pointer at the hook template
this amendment does not move); `git grep -n "ships no default\|every string naming a command"` over
`guard-kit/SPEC.md`; `grep -rn "settings\|guard-kit" native/src/installer` (no allow-entry writer, no
guard-kit wiring); and a read of guard-kit/SPEC.md §The generic ruleset rules 2, 8 to 11, 16, 17, 19,
22, 23, 25 and 26 for steer targets. The build unit re-derives them.

## Existing sections updated

- `guard-kit/templates/settings-allow.json` — new (delta 1).
- `guard-kit/SPEC.md` — §The recommended allowlist, new (delta 2); §The triage criterion's per-segment
  note (delta 3); §Layout and configuration's tree listing and `GUARD_KIT_BREADTH_PROBES` bullet, and
  §scan-prompts' **What crosses into the binary** paragraph (delta 5).
- `guard-kit/README.md` — §Install step 2 (delta 4).
- `guard-kit/smoke/install.sh` — the union merge and the no-contradiction loop (delta 4).
- `docs/guard-kit/SPEC.md` — the on-site SPEC mirror, regenerated with `--emit docs-mirror --write`
  (deltas 2, 3 and 5).
- `docs/footprint.md` and `docs/value.md` — a new file under a kit's `templates/` moves the per-kit
  footprint and its rollup if the footprint counts it. The build runs `check-footprint-fresh` and
  regenerates on red (delta 1).
- `.workflow/release-declarations.md` — one guard-kit bullet: a new template recommends allow entries
  for the front-end and the ruleset's steer targets, and a consumer may review and union them; nothing
  is merged for them (delta 1).

## Retired spellings

- None — no delta retires a spelling. Delta 3 moves a list of common tool names, none of which is
  retired anywhere.

## Definition of Done

- [ ] **Causal completeness** — the template has a named producer and two named consumers, and the
      smoke's invariant has a verdict reader.
- [ ] **Instruction surfaces: instruction only** — the template's `"//"` key and README step point at
      the SPEC and carry no grounds, and delta 2 places them.
- [ ] **Merged with no information lost** — the subsection and the reworded sentences integrated.
- [ ] **Amendment deleted** — this file removed on merge.
- [ ] **Roster re-derived** — rosters re-derived against the tree before the merge counts as complete,
      with any missed site landed and named in the commit.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any cross-component gap found while landing filed through the gap inbox.
