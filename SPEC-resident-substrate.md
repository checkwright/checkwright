# SPEC amendment: resident-substrate

**No delta is applied; every replacement passage below is Not yet applied.**

This amendment serves `instruction-surface-bash-focus`, the always-loaded half of the
`instruction-surface-tier` rewrite. It also takes the three always-loaded bullets that
`always-loaded-brevity-reach` found restating their owners. That entry's own amendment
(context-kit/SPEC-surface-ratchet.md) is the gate half and edits no `CLAUDE.md` prose.

It is a root-level amendment: `CLAUDE.md` is this consumer's rulebook and belongs to no component.
It merges into `CLAUDE.md` itself.

## The grounds this design rests on

This section is amendment-only rationale and does not merge. Every claim in it was probed on
2026-09-12 at the commit that stamped this stage.

**The entry's premise is largely discharged, and the rewrite is narrower than filed.** The entry
said the gate-authoring conventions, the fixture idiom, the housekeeping rules and the delegation
guidance all name bash as the default case. At HEAD:

- The gate-authoring paragraph already says new gates are born native.
- The fixture idiom (`good/`+`bad/`) is substrate-neutral.
- The delegation guidance is a two-line pointer to `/agent-execution`.

What survives is transition-era framing, plus one sentence that is now incomplete.

**The shell invocations stay, because they are the live case.** Every `bash gate-sdk/bin/run-gates.sh`
and `bash gate-sdk/bin/build-native.sh` command in `CLAUDE.md` names the one entry point this tree
has. `git ls-files '*.ps1'` returns only the installer's `installer/bin/checkwright.ps1` and two
fixture copies, so the battery runner has no PowerShell twin. The entry's threshold trigger has
fired — `--emit port-blockers --tree` owes 0 — which means what remains shell is declared no-port,
not pending. TRAJECTORY.md forbids stating a floor as reached before it is, so the commands keep
naming bash.

**What the rewrite removes, and why each goes:**

- *"Either substrate ships with a `good/`+`bad/` fixture pair"* presents two gate substrates as
  peers. gate-sdk/SPEC.md §The port-candidate criteria retires exception class (a) with the words
  *no gate is permanently shell*, so a new shell gate is the cited exception, not a peer.
- *"that section's retired exception class (a)"* is a pointer to a retirement. The standing rule it
  guards — no gate is permanently shell — is kept; the history of how the rule arrived is dropped.
- *"the Rust crate off the shell substrate"* and *"**The binary is live**"* narrate the transition.
- *"bash up to the boundary its README rules"* is incomplete. installer/README.md §The install
  boundary describes a bootstrap written twice, once in bash and once in PowerShell.
- The non-gate port-cost and install-behaviour pointers in the `native/` bullet are behind load
  triggers that the work needing them already fires. A session porting an arm opens gate-sdk's SPEC;
  a session touching installation opens the installer README.

**The three bullets the brevity entry named, re-read at HEAD:**

- The entry named four, but the `--run-demo` bullet it cited is gone. The close brevity pass
  deleted it on load-trigger residency, as that commit's message records.
- **The `.tmp/`/`.metric/`/`.workflow/` bullet** spans 17 lines. Its mechanism restates
  lifecycle-kit/SPEC.md §bin/enter-stage.sh and gate-sdk/SPEC.md §The workflow directory. Its
  resident-worthy content is the tracking status of each directory, the local-only file roster, and
  the ops runbook's per-write account step, which binds any session and has no trigger that loads it.
- **The `installer/` bullet** (7 lines) keeps its layout owner, its not-a-kit predicate and its
  never-committed payload. It loses the packing route, which installer/README.md §The packer owns.
- **The `reserve/` bullet** is **ruled compliant, correcting the brevity entry's reading.**
  `reserve/crates/README.md` is the published crate readme and carries no "do not develop here"
  guard, so the bullet restates nothing. No trigger loads the guard for a session about to write into
  `reserve/`.

**Two readers whose verdicts the rewrite must keep true:**

- `check-tracking-claim` binds the tracking predicate adjacent to a backticked path on the manifest
  set, and `CLAUDE.md` is in that set. Today it verifies `.tmp/` and `.metric/` as gitignored. Its
  red is a false predicate or an unresolvable path, never absence. Delta 4's text keeps both claims
  in bound, true form, and describes `.workflow/` in prose, because that directory has no rule-based
  two-tier proof (canon-kit/SPEC.md §check-tracking-claim).
- `check-manifest-count` reds on a bare cardinal. The replacement text states none beyond the
  enumerated *four contracts*, which `CLAUDE.md` carries today.

## What changes

### (1) The gate-authoring paragraph states the post-port default

The `CLAUDE.md` paragraph opening "New gates here are **born native**" is replaced whole
{design-bearing}. **Not yet applied:**

> New gates here are **born native** — a Rust module, a `.gate` descriptor and a `good/`+`bad/`
> fixture pair; no gate is permanently shell, and a shell gate needs a cause from the live exception
> classes, stated in its own SPEC section (gate-sdk/SPEC.md §The port-candidate criteria). The four
> contracts (output, fail-closed, fixture-pair, self-lint) are
> [gate-sdk/SPEC.md](gate-sdk/SPEC.md)'s, enforced by the meta-gates, and the port oracle
> (gate-sdk/SPEC.md §port-blockers) answers for the battery and, with `--tree`, for the project. A
> red gate is fixed, never bypassed with `--no-verify` except as a one-off with cause.

### (2) The `native/` bullet keeps the obligation and drops the transition

The §Housekeeping bullet opening "`native/` is the Rust crate off the shell substrate" is replaced
whole {design-bearing}. **Not yet applied:**

> - `native/` is the gate binary's Rust crate — one multi-call binary, a subcommand per gate plus the
>   non-gate arms. The commit-time obligation is the battery, which runs the crate's lint and test
>   arms through `check-crate-arms`, **plus** `bash gate-sdk/bin/build-native.sh`; neither discharges
>   the other. It is **not a kit** — no `checks/`, no `smoke/`, the predicate that makes a root
>   directory one — and `check-gate-binary-fresh` holds the binary's currency. Dispatch, descriptors,
>   port sequencing and the toolchain floor: gate-sdk/SPEC.md §Porting a gate to the binary
>   substrate.

### (3) The `installer/` bullet names both bootstraps and sheds the packing route

The §Housekeeping bullet opening "`installer/` is the published activation surface" is replaced
whole {design-bearing}. **Not yet applied:**

> - `installer/` is the published activation surface — a bash and a PowerShell bootstrap in front of
>   the gate binary, shipped as a Release tarball and an npm package from one payload;
>   repo-root-governed, and not a kit by the predicate under `native/` above. Its payload is never
>   committed. Layout, boundary and packing: installer/README.md.

### (4) The scratch-and-local-files bullet splits into its resident facts

The §Housekeeping bullet opening "`.tmp/` is gitignored" is replaced by two bullets
{design-bearing}. **Not yet applied:**

> - `.tmp/` is gitignored disposable scratch — gate timings, resume journals, `<key>.run` liveness
>   records — that the scope boundary wipes (keep-list: `scripts/lifecycle-config.sh`); `.metric/`
>   is gitignored persistent, account-bearing measurement, **never committed**; `.workflow/` holds
>   tracked projections beside gitignored capture (gate-sdk/SPEC.md §The workflow directory).
> - Local-only and gitignored: `BRIEF.local.md` (private brief), `ENV.local.md` (probed machine
>   profile plus gotchas — context-kit/SPEC.md §bin/env-probe) and `OPS.local.md` (DNS, repo
>   settings, the release account and push transport). Consult `OPS.local.md` before any domain,
>   repo-settings, release or push work, and run its account step before **any GitHub write**: a
>   write needing no permission succeeds silently under the wrong account, so the step is per-write,
>   never per-session.

## Producers and consumers

- **No new state, event, field or interface.** Every delta rewrites always-loaded prose. Producer:
  `CLAUDE.md`, loaded by the harness at every session start. Consumer: every session, at session
  start.
- **Readers of `CLAUDE.md`, from the descriptors whose `# graph:` couples name it, with each red
  condition:**
  - `check-lifecycle-registration` and `check-doctrine-registration` red on their marker blocks. No
    delta touches a block.
  - `check-brevity` reds on an over-budget pointer bullet in the conventions section. No delta
    touches that section.
  - `check-tracking-claim` reds on a false bound predicate or an unresolvable path. Delta 4 keeps
    the `.tmp/` and `.metric/` claims bound and true.
  - `check-md-refs` and `check-spec-pointer` red on an unresolving link or section. Every pointer in
    the replacement text names a section that resolves at HEAD.
  - `check-manifest-count` and `check-manifest-temporal` red on a bare cardinal or a narration
    marker. The replacement text adds neither.
  - `check-install-claim` and `check-payload-claim` red on a scanned line naming a transport or
    disclosure class against the declared owner. Delta 3 names both transports in a housekeeping
    bullet, not an install section, and makes no disclosure claim. Build runs both gates rather than
    inferring their verdicts.
  - `check-shim-restatement` reds on a span a binding shim copies from its corpus, which includes
    `CLAUDE.md`. Removed text can only remove a copied span; added text is new, so build runs the
    gate.
  - `check-footprint-fresh` and `check-value-rollup-fresh` byte-compare pages that read
    `CLAUDE.md`. The footprint's always-loaded tier counts kit marker blocks only, which no delta
    moves; build regenerates on red.
  - `check-docs-cmd` reds on a fenced, invoked repo-relative `.sh` path or a kit-prefixed knob that
    does not resolve. Every command and knob the replacement text keeps is byte-identical.
  - `check-installer-no-deps` reads `installer/package.json`, not `CLAUDE.md`; its `# spec:` pointer
    names `CLAUDE.md` §Housekeeping, a heading no delta renames. That section has never stated the
    gate's invariant, which installer/README.md §What this package is states. The misplaced pointer
    predates this set and is filed through the gap inbox rather than fixed here.
- **Narrowing (point 5).** Deltas 2, 3 and 4 remove prose from the manifest set. Each reader above
  whose red condition could flip on removed text reds on a found violation — a false claim, a copied
  span, a stale page — which is monotone in the removed text. The two regenerated pages are
  byte-compares that red on any change and are regenerated.
- **The always-loaded meter** drops by the net lines removed. Close's brevity pass re-baselines it.
  `check-surface-ratchet` (context-kit/SPEC-surface-ratchet.md) stamps its ceiling after this
  amendment lands, so the cut is what the ceiling holds.

## Existing sections updated

- `CLAUDE.md` §This repo is governed by its own kits — the gate-authoring paragraph (delta 1).
- `CLAUDE.md` §Housekeeping — the `native/`, `installer/` and scratch-directory bullets (deltas 2,
  3 and 4).
- `TASK-QUEUE.md` `always-loaded-brevity-reach` — its "four bullets" paragraph corrected to the three
  that remain and pointed here (delta 4). This lands at promotion, in the commit adding this file.

## Retired spellings

- None — no delta retires a name another surface cites; the removed phrases are descriptions, not
  names, and every kept command and section pointer is byte-identical.

## Definition of Done

- [ ] **Causal completeness** — no new state, event or field; every reader of `CLAUDE.md` named with
      its red condition.
- [ ] **Merged with no information lost** — each dropped clause is either owned by the section the
      bullet points to, or is transition narration; `ls SPEC-resident-substrate.md` fails after merge.
- [ ] **Amendment deleted** — this file removed on merge.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any gap discovered during the work filed through the gap inbox.
