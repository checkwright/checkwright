# SPEC amendment: harness-template-port

The port disposition of the **harness template class** — the executable
`<kit>/templates/*.sh` files a vendored tree wires into its harness or its git
hooks, and their `<gates-dir>/` consumer copies. Cut 3 of 3 of the
`port-declaration-cohort-and-windows-leg` declaration cohort, sequenced last
because it is the class with no precedent on either side.

**What this amendment delivers is a class ruling and the declarations that
ruling licenses — not a subtraction from the owed column.** The ruling is a
**partition**, and the partition is the finding: two stated grounds this tree
already holds reach part of the class and reach no more of it, so part of the
class declares and the rest stays owed with its port work filed. The unit set
was ruled on 2026-08-30 (operator, lead-relay) with a partial-port outcome
explicitly live here, and this is that outcome.

## What changes

### (1) A class ruling, `gate-sdk/SPEC.md §check-template-copy-parity` gains *The port disposition*

The harness template class takes its ruling in the section that already owns its
corpus, and the placement is a finding rather than a convenience {design-bearing}.
§check-template-copy-parity's scope paragraph derives its population **from
layout, never a roster** — `<kit>/templates/<name>.sh` ↔
`<gates-dir>/<name>.sh` — and excludes `*-config.sh` by name suffix. That
exclusion is exactly the sibling cut's corpus, so the two cuts partition one
population with no overlap and no gap, and each cut's stated contract is a real
section rather than a set assembled for the occasion. The section also already
holds the ruling this one rests on: its `case:` class reads an arm's action and
**discards its pattern**, "a privacy boundary rather than a parsing
convenience", because "a consumer's arm patterns are its own rule vocabulary,
which a kit gate must never read (CLAUDE.md §The provenance seam)".

The ruling states one ground with two faces, and names its members on it:

- **A cut narrows the port, never an extension point.** Where a kit template's
  own body carries the marked gap a consumer fills, that template *is* the
  extension point and porting it deletes the thing there is to fill. This is
  `native-gate-port-remaining-corpus`' ruling (1) applied to a seam that is a
  file rather than a knob, and it is the ground `drift-kit/templates/drift-config.sh`
  already declares on in the sibling class — "it **is** the adopter's config seam
  rather than kit mechanism reaching it, so porting it deletes the seam".
- **A consumer copy carrying the consumer's own rule content cannot cross into
  kit mechanism.** CLAUDE.md §The provenance seam, and the same ground
  `drift-kit/templates/kpi-deprecated-surface.sh` declares on.

**The members the ground reaches**, each cited on the fact that puts it there
rather than on the class:

- `guard-kit/templates/bash-guard.sh` — its body is a resolve-source-read
  preamble around one marked gap, and guard-kit/SPEC.md §Consumer rules rules
  the placement: "guard-kit ships no consumer rule and names none. What a
  project blocks or steers is its own toolchain knowledge and stays in its
  copy." The gap is the seam; the file is the seam's carrier.
- `scripts/bash-guard.sh` — this repo's copy, which carries project block rules
  and the `# copy-divergence:` declarations naming them. It is the *only* copy
  in the corpus that declares divergence, and what it declares is rule content
  the kit is forbidden to hold.
- `context-kit/templates/session-context.sh` — carries marked gaps at every
  layout-judgment step, and context-kit/README.md instructs an adopter to edit
  them ("layout judgment, not mechanism").
- `scripts/session-context.sh` — this repo's filled instance of those gaps.
- `gate-sdk/templates/check-skeleton.sh` — angle-bracket fill-ins and nothing
  else. §Layout and configuration lists a consumer's own `check-*.sh` gates as
  "copy-edits of `templates/check-skeleton.sh`", and §templates/check-skeleton.sh
  rules it "a template, never a registry member". A vendoring adopter with no
  crate births a shell gate from this file; porting it deletes the only thing
  gate-sdk hands that adopter.

**The members the ground does not reach are named too, and they stay owed.**
`agent-budget-guard.sh`, `agent-dispatch-guard.sh`, `statusline-usage.sh`,
`subagent-stop-liveness.sh`, `usage-poller.sh`, `escalation-guard.sh`,
`wakeup-guard.sh`, `workflow-state-guard.sh` and the `<gates-dir>/` copies of
the first, second, fourth and last. None carries a marked gap, none carries a
consumer literal of its own, and reading a consumer roster *through the config
bridge* is what the bridge exists for rather than a reason a file cannot port —
`agent-dispatch-guard.sh` reads `DELEGATION_KIT_READONLY_TYPES` from the
consumer config and holds no vocabulary of its own, which is the distinction
that keeps it off the declaring side.

**The copy rule, stated once instead of per member**: a `<gates-dir>/` copy
takes its template's disposition, because the pair is one artifact in two homes
— which this section already asserts as a contract surface. A copy on the owed
side is not un-portable; it is **deleted** when its template ports, which
satisfies TRAJECTORY.md's predicate directly. The two copies on the declaring
side carry an independent ground on top of their template's, so a later ruling
that ported those templates still could not take them.

**What reopens it**, on §Consumer smoke *The port disposition*'s terms, because
`# no-port:` is the permanent tier and a class ruling owes its reader what would
falsify it: the extension-point face dissolves for a member whose marked gap is
removed from its template — the seam having moved elsewhere or ceased to exist —
and the provenance face dissolves for a copy that stops carrying rule content
its template does not, which is the state assertion C of this same gate already
reports on.

**The honest limit, stated because this ruling does not answer it.** The owed
side is the larger half by line count, and nothing here argues those files are
hard to port — only that no *stated* ground reaches them. What they actually
meet is recorded on the entry delta (4) files, not asserted here as if it were a
disposition.

### (2) Five `# no-port:` header declarations

Each declaring file gains one `# no-port: <cause>` header line whose cause names
the ruling that makes it permanent, per §The `# graph:` manifest's authoring
rule that "a cause names the ruling it rests on" {mechanical}. The two
`<gates-dir>/` copies cite the provenance seam directly; the three templates
cite the new subsection delta (1) lands. No file gains any other field, none
carries `# port-until:`, and no `.gate` descriptor is touched — the placement
rule §The `# graph:` manifest states is unchanged by this cut.

### (3) Each owning kit's SPEC section records its own members' disposition

The ruling reaches by ground, not by scope, so a member outside gate-sdk
declares in its own header and its own SPEC section says so — the shape
§Consumer smoke *The port disposition* already set for a class spanning kits
{design-bearing}. `guard-kit/SPEC.md §Consumer rules` records `bash-guard.sh`
and its copy; `context-kit/SPEC.md §The session-context hook (template)` records
`session-context.sh` and its copy; `gate-sdk/SPEC.md §templates/check-skeleton.sh`
records its own. Each is one sentence naming the disposition and pointing at
delta (1)'s subsection, never a second copy of the argument.

### (4) The owed residue is filed as one deferred entry

The port work this ruling leaves standing is filed rather than absorbed, per the
unit set's own boundary {mechanical}. One deferred entry owns the whole owed
side, and it carries the three facts a future porting session would otherwise
buy again, each measured here:

- **The crate has no arm of the required kind.** `native/src/emit/mod.rs`'s
  `BRIDGED_ARMS` table holds emitters and the battery runner; none reads a
  harness hook payload from stdin, none writes a hook-JSON envelope, and none
  uses non-zero exit as protocol — which `subagent-stop-liveness.sh` and
  `wakeup-guard.sh` both do. A port of this residue mints an arm *kind*, not
  another arm.
- **The non-gate arm's own contract cuts against it.** `native/src/main.rs`
  states that the caller may not be assumed to be a POSIX shell, "so every value
  arrives as argv and the arm reads no knob" — and `session-context.sh` and
  `statusline-usage.sh` both read their environment.
- **One member is named by a literal settings grant.** `.claude/settings.json`
  carries a repo-relative `Bash(...)` grant naming
  `delegation-kit/templates/usage-poller.sh`, which is `check-settings-paths`'
  corpus. A port that deletes that file strands the grant and reds that gate
  unless the grant moves in the same commit — `native-gate-port-remaining-corpus`'
  settings-grant carve-out exactly, and the reason it is recorded now.

### (5) The class-width premise on the promoting entry is corrected

`harness-template-port-disposition` records `context-kit/templates/session-context.sh`
at one line fewer than the file has, and carries the two totals derived from it
{mechanical}. The oracle's own row and `wc -l` agree against the entry; the
entry's three figures move together, and nothing else on it changes.

## Producers and consumers

The only new state this amendment introduces is a **port-disposition
declaration** on five tracked shell files. It has no new field, no new tag, no
new knob and no new interface.

- **Producer** — the build session's declaring commit writes one `# no-port:`
  header line per member. Its enabling path is nothing but the file being
  tracked: §The `# graph:` manifest rules the fields' domain to be any tracked
  script, with "no registration step, which is the whole reason the field can
  reach a corpus that owns no descriptor". Nothing must be configured for the
  declaration to be seen.
- **Consumer 1 — `§port-blockers`' `--tree` arm**, which reads the disposition
  from the file's header block and reclassifies the row `owed` → `no-port`,
  moving the trailer's declared and owed counts. This is the reader
  TRAJECTORY.md's completion predicate is stated over.
- **Consumer 2 — `§check-gate-substrate-parity` assertion G**, once the sibling
  amendment `SPEC-port-declaration-shape.md` widens its corpus to the tracked
  shell tree. Until that widening lands, none of these five declarations is
  read by any gate — which is the whole of the enforcement-first case for
  pairing the two, and the reason this amendment names the dependency rather
  than assuming it.
- **Consumer 3 — `§check-comment-tier`**, which reads both spellings out of its
  built-in directive roster over the whole governed tree, so a declaration line
  is a directive rather than a restatement. This needs no widening: the roster
  and the corpus both already reach.
- **Consumer 4 — `canon-kit`'s measured-claim oracle**, transitively.
  `scripts/measured-claims.sh` reads the `--tree` trailer's owed count into the
  `tree-shell-owed` key, and the generated hooks bake that resolved value, so a
  declaration stales them. That is a **named reader at a named transition**, and
  it is why the regeneration below is an update target rather than a courtesy.

**Every field has its reader and no field is added without one.** A `# no-port:`
payload is free text by §The `# graph:` manifest's ruling ("the kit still
constrains nothing but non-emptiness"), read by consumer 2 for non-emptiness and
by a human for the ruling it names. No cause here carries a slug, a vocabulary
token, or anything a machine parses — which is also what keeps this cut on the
right side of the provenance seam it is arguing from.

**This delta widens no corpus and narrows none**, so §The causal-completeness
check point 5's red-condition enumeration does not bind: every reader above sees
strictly more declarations and strictly fewer owed rows, and no reader in the
tree reds on *finding none*, asserts an exact count, or holds a coverage floor
over this corpus. The one count that moves — `tree-shell-owed` — is cited by no
governed sentence today, verified by scanning the `measured:` markers on the
governed surfaces rather than by assuming it.

## Existing sections updated

- `gate-sdk/SPEC.md §check-template-copy-parity` — gains the *The port
  disposition* subsection, and its scope paragraph gains the sentence saying that
  the `*-config.sh` exclusion is the sibling cut's corpus, so a reader arriving
  at either cut meets the partition (delta 1).
- `gate-sdk/SPEC.md §templates/check-skeleton.sh` — records its own member's
  disposition, and why a crate-carrying tree that births no shell gate still
  ships the skeleton (deltas 1 and 3).
- `guard-kit/SPEC.md §Consumer rules` — records `bash-guard.sh`'s and its copy's
  disposition beside the placement contract that grounds it (delta 3).
- `context-kit/SPEC.md §The session-context hook (template)` — records `session-context.sh`'s
  and its copy's disposition beside the marked gaps that ground it (delta 3).
- `TASK-QUEUE.md`, the `harness-template-port-disposition` entry — its three
  width figures corrected in place (delta 5).
- `TASK-QUEUE.md`, one new deferred entry owning the owed residue (delta 4).
- The generated projections this cut stales: the on-site SPEC mirror, and — the
  non-obvious one — the generated `pre-commit`/`commit-msg` hooks and
  `docs/check-graph.html`, which `docs/site-architecture.md` §Generated
  projections already names as staling when "a script header gaining a
  `# no-port:` cause moves the `tree-shell-owed` key". Verified against the
  committed hook, which bakes the owed count today (all deltas).

<!-- update-target-exempt: the composer entry takes no write from a cut by its own 2026-08-28 ruling — each closed cut's record lives in the contract section that cut selected, which is delta 1's subsection -->
- `TASK-QUEUE.md`, the `native-gate-port-remaining-corpus` entry — deliberately
  unwritten.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather
      than at the commit, this iteration carrying sibling amendments.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The oracle moved, and by the roster rather than by a number** — the
      `--tree` arm reports each declaring member `no-port` and every other member
      of the class unchanged, read as a per-file roster diff and not as a trailer
      delta.
- [ ] **The regeneration fan-out is discharged in the landing commit** — the
      generated hooks and the graph artifact, whose staleness this cut causes
      through the `tree-shell-owed` key.
