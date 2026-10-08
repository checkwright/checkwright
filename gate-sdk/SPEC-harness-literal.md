# SPEC amendment: harness-literal

A gate that reds a harness-bound literal in prose outside the surfaces that declare the binding, so the sweep that named the hook anchor generically has a catcher.

## What changes

### (1) `check-harness-literal` reds a listed literal outside a declared site {design-bearing} {user-facing: operator direction 2026-10-06 on the queue entry, in the unit set by operator direction 2026-10-08 — a gate reading a knob-listed set of harness literals, red on any outside a knob-listed adapter-surface set}

Invariant: no file on the configured corpus carries a configured literal, except at a site carrying a `harness-binding: <reason>` valve. It is `precommit` with `trigger=*`, since any prose edit can re-acquire one, and binary-dispatched.

- **The literals** are `GATE_SDK_HARNESS_LITERALS`, a whitespace list of fixed strings, matched as bytes with no pattern syntax. A harness's variable name is a word, and a roster of words a reviewer can read needs no regular expression to hold it.
- **The corpus** is `git ls-files` over `GATE_SDK_HARNESS_LITERAL_PATHS`, a whitespace list of pathspecs passed to git as written, so an adapter surface that is a whole file or directory leaves the corpus by git's exclude magic in the same knob. One knob carries both halves the queue entry named, the governed prose and the adapter-surface set, because a second list of exclusions would be a second spelling of what a pathspec already says. A binary member is skipped and counted.
- **The valve** marks an adapter surface narrower than a file: a section that reads the literal, or a sentence that declares the binding. A matching line is clean when it, or the line above it, carries `harness-binding:` followed by a non-empty reason; a trailing comment closer is not part of the reason. In markdown the valve is an HTML comment on the line above. An empty reason is a finding.
- **Exit 1** on a matching line with no valve, reporting `path:lineno:line` and the literal. **Exit 2** on a non-repository working directory and on an unreadable corpus member.
- **Both knobs default empty, and an empty one disables the gate**, clean with the absence named in its detail line, a different sentence from a nothing-found clean. The member declares `# armed-by: GATE_SDK_HARNESS_LITERAL_PATHS`.

**It is §check-portability-floor's scanner under a second knob pair and a second valve**, on that section's separate-knob-pair ground: a harness literal folded into the portability roster would share one corpus and one verdict with a BSD incompatibility. The module reuses the corpus enumeration, the valve window and the binary skip; it takes fixed strings where that member takes patterns, and has no ASCII arm.

**The valve is refused nowhere it is honest, and the cheap alternative is refused.** Excluding a whole SPEC because one of its sections reads the variable would leave every other sentence of that file uncaught, which is the failure the gate exists for. So a file leaves the corpus only when the whole file is an adapter surface, and a narrower surface is marked at its site, where a new site in the same file is red until its author declares it.

**It is the publisher's own gate and does not ship** (§Consumer payload). Its subject is the harness-neutrality of kit prose, a constraint §The adopter constraints puts on the publishing repository, and an adopter holds no kit prose to keep neutral. The descriptor and its fixture pair sit in the publisher's gates dir and the rule stays in the crate, as §check-crate-arms' do. The kit ships no literal: a default naming one harness's variable would be the vendor literal the knob exists to keep in config.

**What a green run buys.** The next instance of a listed literal on the listed corpus, never harness-neutrality: a literal nobody listed passes, and so does a harness-bound sentence that names no listed word. The wider restatement of the hook contract is its own queue entry.

**Born native, no shell form authored.** No exception class at §The port-candidate criteria applies. Criterion 4 clears where the corpus holds prose alone; this repo's value below names no `.gate` descriptor and no crate module.

The `good/`+`bad/` pair: `good/` holds a literal under the valve in both window positions, a file excluded by pathspec that carries one bare, and a member carrying none; `bad/` holds a bare literal, an empty-reason valve and a valve one line too far away. The disabled cleans, the binary skip and the non-repository fail-close live in a `.test.sh` beside the pair, as §check-portability-floor's do.

### (2) The valve token joins the directive roster {mechanical}

`harness-binding: <reason>` joins the site valves canon-kit/SPEC.md §check-comment-tier lists, cited to this member's section, and the built-in directive roster in `native/src/gates/comment_tier.rs`, beside `portability-declared:`. A full-line valve in a file that gate governs is then a directive rather than a comment to delete.

### (3) This repo arms the gate and declares its three sites {mechanical}

In the tracked gate-sdk knob file: the literal roster holds the project-directory variable's name, and the corpus is every kit SPEC, every kit README, the root README and every kit's markdown templates, with the plugin directory and the generated mirrors under the Pages root excluded. The plugin is the shipped binding whole (§The adopter constraints), and a mirror's size and content are its source's.

The probe is `git grep -n` for the variable's name over `'*/SPEC.md' '*/README.md' README.md '*/templates/*.md' ':!docs/' ':!plugin/'`. It returns three lines, and each member's value is a valve on the line above naming why the site is a binding:

- `context-kit/SPEC.md`, the settings-path reader's placeholder bullet — the reader parses the wiring's own spelling.
- `guard-kit/SPEC.md` §The shell guard, the anchoring paragraph — the sentence that declares the binding.
- `delegation-kit/README.md`, the statusline wiring step — the sentence that declares the binding to an adopter.

The wiring templates are JSON and outside the corpus by its globs. The gate registers in `scripts/gates.list` in the same commit as its valves, so its first run is green.

### (4) §The adopter constraints names its catcher {mechanical}

The master-harness bullet's sentence that kit prose names a harness surface generically gains the citation that §check-harness-literal holds the listed literals to their declared sites. The bullet spells no literal, since the section is on the corpus.

## Producers and consumers

- **`check-harness-literal`** — producer: the pre-commit hook and the battery, through `scripts/gates.list`. Consumer: the committing session, by its verdict.
- **`GATE_SDK_HARNESS_LITERALS`, `GATE_SDK_HARNESS_LITERAL_PATHS`** — producer: this repo's tracked knob file (delta 3). Consumer: the gate alone. Each is read whole on every run.
- **The valve** — producer: an author at a declared site. Consumers: this gate, which reads its reason for emptiness, and `check-comment-tier`, which reads its token as a directive (delta 2). The reason's reader is the reviewer of the diff that adds it.
- **Roster-holding readers of the minted names** — the crate's knob table and rendered roster; the gate registry and the registry document; the comment-tier directive roster; the publisher's gate set that `check-consumer-value-literal` and the meta-gates walk; the release declarations. Each is an update target below.
- **A narrowed corpus** — none: the gate is new, and no existing reader's corpus changes.
- **Every member's value** — delta 3 names each of the three sites'.

**Inferred, cannot run before build:** which meta-gates red a publisher-declared member until its descriptor, module, fixture pair and registry row all exist — the battery's own report on the commit that adds them is the reading.

## Existing sections updated

Produced by `git grep -n 'portability-declared' -- canon-kit/SPEC.md native/src`, the probe delta 3 names, and a read of gate-sdk/SPEC.md §check-portability-floor and §The adopter constraints.

- `gate-sdk/SPEC.md` — a new §check-harness-literal beside §check-portability-floor (delta 1); §Layout and configuration — the two knob bullets (delta 1); §The adopter constraints — the master-harness bullet (delta 4); §Meta-gate conservation for the binary substrate and §Consumer payload wherever they roster the publisher's withheld members (delta 1).
- `canon-kit/SPEC.md` §check-comment-tier — the site-valve list (delta 2); `native/src/gates/comment_tier.rs` — the roster entry (delta 2).
- `native/src/gates/` — the new module, sharing `portability_floor.rs`'s scanner; `native/src/knobs/gate_sdk.rs` — two rows (delta 1).
- `scripts/check-harness-literal.gate`, `scripts/gate-tests/check-harness-literal/{good,bad}/` and its `.test.sh`, new (delta 1); `scripts/gates.list` and `scripts/gate-sdk-config.knobs` (delta 3).
- `context-kit/SPEC.md`, `guard-kit/SPEC.md`, `delegation-kit/README.md` — one valve line each (delta 3).
- `gate-sdk/templates/gate-sdk-config.knobs` and `gate-sdk/README.md` where they list knobs (delta 1).
- `.workflow/release-declarations.md` — a row for the two knobs and the valve token (deltas 1 and 2).
- `docs/kits.md`, `docs/check-graph.html`, `docs/gate-sdk/`, `docs/canon-kit/`, `docs/context-kit/`, `docs/guard-kit/`, `docs/delegation-kit/` and every other generated projection of the surfaces above — regenerated, never hand-edited (all deltas).

## Retired spellings

- None — no delta retires a name; the three sites keep their literal and gain a valve.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Seen red on this tree** — with the gate registered and one valve removed, the battery reds on that site and names the literal; the valve is restored in the same session.
- [ ] **The marker discharged** — the cannot-run claim under Producers and consumers is read off the battery's report and deleted.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`), which the iteration's last gate-sdk batch discharges.
- [ ] **Queue entry done** — `--queue done harness-literal-catcher-gate` in the merge commit, before the stage that drains the queue.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
