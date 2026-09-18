# SPEC amendment: smoke-alone

A kit's `smoke/install.sh` "may assume gate-sdk is already installed (it runs
first), nothing else" (gate-sdk/SPEC.md §Consumer smoke, the `smoke/` per-kit
contract). The harness never tests that sentence. Its default run vendors every
kit into one scratch consumer, so an install that leans on a sibling kit's
install stays green there. This amendment makes the harness check each kit on its
own in the same run, and fixes the kits that fail the check today.

**Probed at authoring, 2026-09-18, at `bb6cf04c`.** The command was
`bash gate-sdk/bin/run-gates.sh --run-consumer-smoke <kit>`, run once for each of
the 11 kit roots. **All 11 are red**, and the default (all-kit) run is green in
22s. Five failure classes, each confirmed against a `--keep` scratch tree:

| class | kits red on it | first red | cause |
|---|---|---|---|
| A | context-kit, delegation-kit, doctrine-kit, evidence-kit, gate-sdk, queue-kit | accounting: `check-action-{gh-repo,permissions,pinning,run-shell}` unaccounted, scratch exit 0 | gate-sdk ships the four gates, and only site-kit's install writes the Actions surface and registers them |
| B | drift-kit, guard-kit | `check-graph` exit 1, hook and graph `ARTIFACT` stale | gate-sdk's install regenerates both artifacts under its own `GATE_SDK_KIT_DIRS="$SDK"` export. A later install that regenerates repairs them, and these two kits' installs do not regenerate |
| C | canon-kit | `check-docs-link-convention` exit 2, `not a directory: docs` | only site-kit's install writes `docs/` |
| D | lifecycle-kit | `install-lifecycle: agent file not found: CLAUDE.md`, harness exit 2 | context-kit's install seeds `CLAUDE.md` |
| E | site-kit | `check-workflow-tiering` exit 2 after the final restore, `.workflow` not found | gate-sdk's install creates `.workflow/` empty, so it is never committed, and the restore's `git clean -fd` deletes it. It survives the full run only because lifecycle-kit writes files into it |

A run stops at its first red, so fixing these five can expose further failures in
later phases. Delta 7 owns those.

**The failure is general, and so is the fix.** A narrowed run could instead be
refused wherever a kit has an install dependency. The table shows why that would
not work: every kit fails when run alone, so the refusal would cover every
narrowed run.

## What changes

### (1) The self-sufficiency phase joins the default run {design-bearing}

`--run-consumer-smoke` gains one phase, after the union run's final green
assertion and before its clean line. When the run vendors more than gate-sdk
plus one other kit root, it repeats the whole union pass once per root. The first
repeat is gate-sdk alone. Each later repeat is gate-sdk plus exactly one other
root, in root order. A repeat covers the scratch build through
`vendor_and_install`, the zero-config green assertion, the registration
accounting, that kit's violation, and the final green. Every repeat is a fresh
scratch tree, torn down whatever `--keep` says. `--keep` keeps the union tree
only.

- **Skipped** when the operands name one kit root, because the union run already
  is that kit's run alone. This is the cheap narrowed check the queue entry was
  about, and it costs nothing extra.
- **Installer output** of a repeat goes to stderr, the `Sink` that
  `--upgrade-smoke` already uses. A repeat's verdict lines are not printed when it
  is green.
- **A red repeat is exit 1, whatever exit the same failure would give in a
  union run.** That includes an installer that exits non-zero. The union run
  already ran the same installer to green in the same environment, so a failure
  alone is a finding about the kit, not about the environment. Reporting it as
  exit 2 would put the one failure this phase exists to find into the
  could-not-run band. The red names the kit and prints that repeat's own failure
  lines. It ends with
  `  help: reproduce with bash gate-sdk/bin/run-gates.sh --run-consumer-smoke --keep <kit-root>`.
- **The phase line**, printed on stdout before the clean line on a green run:
  `CONSUMER-SMOKE: alone — <n> kit root(s) green alone in <ms>ms`. It reports
  the phase's cost on every run, on the same ground as the accounting phase's
  cost line.
- **The clean line keeps its grammar.** `evidence-kit`'s default exit-code parser
  reads the `consumer_smoke` suite, and no reader parses the clean line's fields,
  so the phase adds a line and changes none.

**Not yet applied.** This text replaces the paragraph in gate-sdk/SPEC.md
§Consumer smoke that opens "**Its default run vendors every kit, so its silence
on a subset vendoring is not coverage.**" (currently lines 8702-8709):

> **Each kit is also run alone, because the union run cannot see an install that
> leans on a sibling.** A kit's `smoke/install.sh` may assume gate-sdk and
> nothing else, and a tree that vendors every kit satisfies every
> cross-dependency by accident. So when the run vendors more than gate-sdk and one
> other root, it repeats the pass for gate-sdk alone and then for gate-sdk plus
> each root, each in a fresh scratch tree, and prints
> `CONSUMER-SMOKE: alone — <n> kit root(s) green alone in <ms>ms` before the clean
> line. A red repeat is exit 1 even when its cause is an installer's exit. The
> union run has already shown the environment works, so a failure alone is the
> kit's. Its help line names the narrowed `--keep` run that reproduces it. The
> phase is skipped for a single-root run, which is already its own repeat. That
> keeps the narrowed run a session reaches for as the cheap check, and the
> default run as validate's. The repeats also put the harness into the subset
> vendoring on every run: one kit vendored while the shared binary carries every
> ported kit's subcommands. `check-gate-substrate-parity` still reaches that
> subset at commit time (§check-gate-substrate-parity), and this phase now
> reaches it at validate too.

**Also not yet applied:** the harness paragraph's success-token sentence (the one
that opens "The harness (`run-gates.sh --run-consumer-smoke [--keep]
[kit-root...]`") gains one clause after the final green assertion: "then, for a
run vendoring more than one kit root beyond gate-sdk, the self-sufficiency phase
below." The phase's cost belongs in *Never at pre-commit*'s cost reasoning. That
paragraph needs no change, because the arm stays a non-gate `Arm::Run`.

The build lands the harness change in `native/src/emit/run_consumer_smoke.rs`,
reusing `smoke`'s pass rather than copying it. It also appends a release
declaration bullet: a shipped arm now does more by default (lifecycle-kit's
build template, *Declare what a vendoring consumer will meet*).

**Inferred, cannot run before build:** the phase's wall-clock — no green repeat exists yet to time it against the green union run's 22s for 11 roots.
Eleven repeats of a two-root tree should cost less than eleven full runs, and the phase line reports the cost from the first green run.

### (2) The per-kit contract states self-sufficiency and names its oracle {mechanical}

**Not yet applied.** In the `smoke/install.sh (required)` bullet of §Consumer
smoke's `smoke/` per-kit contract, this replaces the sentence "It may assume
gate-sdk is already installed (it runs first), nothing else.":

> It may assume gate-sdk is installed (it runs first) and nothing else. It
> establishes every surface its registered gates read, never a surface only a
> sibling kit's install writes, and leaves the scratch tree green with only
> gate-sdk beside it. The self-sufficiency phase above checks this on every
> default run.

In the same bullet's first sentence, "and regenerate the hook + graph
artifacts" becomes "and regenerate the hook + graph artifacts whenever it
changes the registry". That is what the kit set does today: guard-kit's and
drift-kit's installs register nothing and regenerate nothing. It holds once delta
4 makes gate-sdk's leg generate for the whole vendored tree.

### (3) gate-sdk's leg writes the Actions surface its own gates read {design-bearing}

This fixes class A. The four `check-action-*` gates are gate-sdk's, declared
`# install: on-surface`. gate-sdk also ships an Actions-shaped template,
`templates/gates-workflow.yml`. §Consumer smoke's *Starter-template conformance*
already requires a kit that ships a starter template to install it verbatim, and
gate-sdk's install never has. So the owning kit writes the surface and registers
the gates that read it:

- `gate-sdk/smoke/install.sh` copies `$SDK/templates/gates-workflow.yml`
  verbatim to `.github/workflows/gates.yml` and registers `check-action-pinning`,
  `check-action-run-shell`, `check-action-gh-repo` and `check-action-permissions`
  in its `gates.list` heredoc. It also drops the four
  `# unregistered: check-action-* —` placeholder lines.
- `site-kit/smoke/install.sh` drops the `grep -v` retraction and its
  `mv` (lines 9-13), and the four `check-action-*` lines with their `spec:`
  comment from its registration heredoc. It keeps
  copying `templates/site-health.yml`, which the four gates, now registered by
  gate-sdk's leg, go on linting in every run that vendors site-kit.

**Probed:** in a gate-sdk + guard-kit scratch tree, copying the template
verbatim, registering the four gates, dropping the placeholders and regenerating
gave `All 32 gates passed`. A registration split across two legs has no
disposition that is green when the owning kit runs alone. Placeholder plus
retraction is one such split. So no gate's registration is split across legs
any more, and the retraction mechanism has no remaining instance.

**Not yet applied — gate-sdk/SPEC.md §Consumer smoke.** Two rewrites:

1. In the paragraph that opens "**It does place the gate binary**", replace "`site-kit`'s does: it copies
   `templates/site-health.yml` in, and that template is the only Actions-shaped
   surface any install writes, so §check-action-pinning and §check-action-gh-repo
   earn their scratch-battery slot" with "gate-sdk's own does: it installs its
   `templates/gates-workflow.yml` verbatim, so the four `check-action-*` members
   earn their scratch-battery slot on gate-sdk's leg, and site-kit's
   `templates/site-health.yml` is linted by them wherever site-kit is vendored".
2. Rewrite the paragraph "**§check-gate-substrate-parity assertion I sees every
   kit vendored from the first leg on.**" as:

> **§check-gate-substrate-parity assertion I sees every kit vendored from the
> first leg on.** Its kit-vendored scope is a directory scan, and the harness
> copies every kit's tree before any `smoke/install.sh` registers a gate. So
> gate-sdk's leg, which runs first with only its own lines in the registry,
> scopes itself with `GATE_SDK_KIT_DIRS` for its own process. Every gate-sdk-owned
> member that leg omits is declared `# unregistered:` there, and no later leg
> registers one. A registration split across legs cannot be green in the owning
> kit's alone run, and the self-sufficiency phase is that run.

**Not yet applied — site-kit/SPEC.md**, the two paragraphs at lines 659-672. The
sentence "this kit contributes `check-docs-cname-parity` and — because it writes
the only Actions-shaped surface any install writes — the gates that lint a
workflow's Actions shape: `check-action-pinning`, `check-action-run-shell`,
`check-action-gh-repo`." becomes "this kit contributes
`check-docs-cname-parity`, and its workflow is linted by the `check-action-*`
gates gate-sdk's own leg registers beside the workflow template gate-sdk
installs". The next paragraph's "Those four qualify … because this install
writes the workflow they lint" becomes the same predicate applied to gate-sdk's
leg, with a pointer to gate-sdk/SPEC.md §Consumer smoke rather than a restated
reason.

**Not yet applied — gate-sdk/SPEC.md, the "necessary and not sufficient"
paragraph** (currently lines 5673-5684). Its "site-kit's `smoke/install.sh`
registers both members of this cohort because it installs the workflow template
they lint" becomes "gate-sdk's own `smoke/install.sh` registers both members of
this cohort because it installs the workflow template they lint". Only the
subject changes, and the paragraph's reasoning stands.

### (4) gate-sdk's leg generates its artifacts for the whole vendored tree {mechanical}

This fixes class B. In `gate-sdk/smoke/install.sh`, the two artifact lines (the
`--emit git-hooks --write` and `--emit graph >` pair) run without the leg's
`GATE_SDK_KIT_DIRS` narrowing. Spell each as `env -u GATE_SDK_KIT_DIRS …`, or run
both before the `export`. Either way the hook and graph describe every vendored
kit root, which is the tree the battery reads. The narrowing stays on every other
command, and assertion I's scope is what the narrowing is for.

**Probed:** in the guard-kit scratch tree, an un-narrowed regeneration took the
battery from `check-graph` red to `All 28 gates passed`. drift-kit's scratch tree
printed the identical two `ARTIFACT` lines.

### (5) gate-sdk's leg commits a `.workflow/` member {mechanical}

This fixes class E. `check-workflow-tiering` is gate-sdk's, and its subject is
the workflow directory. An empty directory is not tracked, and the restore's
`git clean -fd` removes it. `gate-sdk/smoke/install.sh` therefore writes
`.workflow/release-declarations.md`, holding the drained form of the surface
gate-sdk owns (§upgrade-smoke): its `# contract:` header line and nothing else,
byte-for-byte this tree's line 1 of `.workflow/release-declarations.md`.

**Probed:** a `.workflow/.gitkeep` does not work. It reds
`check-workflow-tiering` assertion B, because every tracked member needs a
`# contract:` header.

**Inferred, cannot run before build:** whether that header file stays green under the full run's other `.workflow/` readers — delta 1's phase and the union run are the oracle, and neither exists before build lands delta 1.
Delta 7 covers a red.

### (6) canon-kit's leg writes the `docs/` page its gate reads {mechanical}

This fixes class C. `canon-kit/smoke/install.sh` registers
`check-docs-link-convention`, so it writes `docs/index.md` holding one `#`
heading line, and only when `docs/index.md` is absent. site-kit writes `docs/`
after canon-kit in the union run, and this guard keeps canon-kit's page from
depending on that order.

**Probed:** in the canon-kit scratch tree, adding `docs/index.md` with a single
heading cleared `check-docs-link-convention`. The only remaining red was the
`.gitkeep` from delta 5's refuted candidate.

**Inferred, cannot run before build:** that the page stays green under site-kit's docs gates in the union run — no union run under delta 1's phase exists yet with delta 6's page and site-kit's registrations both landed.

### (7) Residual failures are landed in the owning kit's install {design-bearing}

This fixes class D, then every failure the phase turns up after it.
`lifecycle-kit/smoke/install.sh` seeds `CLAUDE.md` when it is absent, before its
`--install-lifecycle` call, in the form `doctrine-kit/smoke/install.sh:28`
already uses (`if [[ ! -f CLAUDE.md ]]; then … fi`).

The build then runs the default arm with delta 1 in place. A repeat that reds
further on is in this delta's envelope when its fix is the same kind of act: an
install establishing a surface its own registered gates read. The build lands
it in that kit's install and names it in the commit message. A residual whose fix
would need a harness change, a new name, or a registration split across legs is
outside the envelope and goes to the lead.

### (8) The generated mirrors {mechanical}

`docs/gate-sdk/SPEC.md` and `docs/site-kit/SPEC.md` are regenerated from their
sources after deltas 1-3 land. Each mirror's freshness gate prints its own
regeneration command.

## Producers and consumers

- **The self-sufficiency phase** (delta 1). Producer: `--run-consumer-smoke`
  itself, on every default invocation, with no knob. Its enabling configuration
  is the configuration that already runs: this repo's `consumer_smoke` validate
  suite (`scripts/evidence-config.knobs`, `EVIDENCE_KIT_RUN_consumer_smoke`) runs
  the arm with no operand. Consumer: that suite, through the arm's exit status,
  since evidence-kit's default parser reads the exit. The phase line's fields
  (`n`, `ms`) are read by whoever owns the iteration at validate, the same reader
  as the accounting cost line. No field goes unread: `n` shows the phase ran
  over the full root set and was not skipped. The help line's reader is the
  session reproducing a red. No roster-holding reader exists for the new line,
  because no gate parses the arm's stdout.
- **The contract sentence** (delta 2). Producer: kit authors. Consumer: the phase
  (delta 1), which executes it. That makes the sentence enforced, where before
  it was prose alone.
- **The Actions surface on gate-sdk's leg** (delta 3). Readers of the moved
  registration: the registration accounting, which now finds the four gates
  registered in every run, so they leave the probe set. Assertion I, which loses
  four `# unregistered:` lines because the gates are registered. `check-graph`
  and the hook, through the regeneration. `check-install-disposition` assertion
  B, which reads install bodies as text and holds `zero-config` gates only, while
  the four are `on-surface`, so the move is neutral to it. The one reader whose
  verdict could turn, `check-template-copy-parity`, ran in the probe that gave
  32 passing gates.
- **Red conditions for the narrowed site-kit roster** (point 5: site-kit stops
  registering four gates). The accounting reds on an unaccounted gate. The four
  are registered by gate-sdk's leg in every run that includes site-kit, since
  gate-sdk always leads, so no gate becomes unaccounted. The accounting also
  reds on a stale `# smoke-unregistered:` declaration, and site-kit carries none
  for these four. No reader asserts a count of site-kit's registrations.
- **Every member's satisfying value** (point 6, the obligation that every kit
  root is green alone). The 11 roots and their values: gate-sdk, context-kit,
  delegation-kit, doctrine-kit, evidence-kit and queue-kit get delta 3.
  drift-kit and guard-kit get delta 4. canon-kit gets delta 6. lifecycle-kit gets
  delta 7's seed. site-kit gets delta 5. Beyond each root's first red, no value
  can be named before build, which is why delta 7 keeps an envelope instead of
  claiming the list is complete.

## Existing sections updated

Roster probe:
`git grep -n "smoke/install.sh\|Actions-shaped\|subset vendoring\|nothing else" -- '*.md'`
over the tracked tree, with `docs/` mirrors and `docs/posts/` excluded, then
reading each hit's paragraph.

- `gate-sdk/SPEC.md` §Consumer smoke: the harness paragraph, the
  subset-vendoring paragraph, the binary-placement paragraph, the assertion-I
  paragraph, and the `smoke/install.sh` contract bullet (deltas 1, 2 and 3).
- `gate-sdk/SPEC.md`, the "necessary and not sufficient" paragraph before
  §Consumer smoke (delta 3).
- `site-kit/SPEC.md`, the smoke-install paragraphs at lines 659-672 (delta 3).
- `native/src/emit/run_consumer_smoke.rs`, the phase (delta 1).
- `gate-sdk/smoke/install.sh` (deltas 3, 4 and 5).
- `site-kit/smoke/install.sh` (delta 3).
- `canon-kit/smoke/install.sh` (delta 6).
- `lifecycle-kit/smoke/install.sh` (delta 7).
- `.workflow/release-declarations.md`, the arm's behaviour change (delta 1).
- `docs/gate-sdk/SPEC.md` and `docs/site-kit/SPEC.md` (delta 8).

## Retired spellings

- None — no delta retires a spelling a reader cites. The four
  `# unregistered: check-action-*` lines are data lines in one install script,
  and nothing outside that script and site-kit's `grep -v` names them. Both sites
  are rostered.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them
      (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it; the merged spec
      reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` above is accurate, and
      `check-amendment-retired-spelling` is green.
- [ ] **The oracle is green** — `bash gate-sdk/bin/run-gates.sh
      --run-consumer-smoke` exits 0 with its phase line naming all 11 roots, and
      `--run-consumer-smoke lifecycle-kit` exits 0 on its own. Both are pasted in
      the landing commit.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
