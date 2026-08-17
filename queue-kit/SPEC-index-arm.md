# SPEC amendment: queue-index-non-gate-arm-port

`queue-kit/bin/queue-index.sh` moves onto the binary as a non-gate arm. The
*shape* is already ruled — a ported non-gate tool becomes a non-gate arm
(gate-sdk/SPEC.md §The non-gate arm), the form `freshness-emitter-port-cohort`
records as answered and merged — so this amendment applies that decision and
settles the half the queue entry left open: **what each caller invokes, and
whether the hook's own shell survives it.**

**The caller set is corrected before it is designed against.** The entry names
`queue-kit/bin/queue-counts.sh` as a caller; it is not one. Read end-to-end, that
file never invokes `queue-index.sh` — its only relation is a `spec:` citation
that the two tools count the same unit (§bin/queue-counts.sh), which is an SSOT
coupling and not a call. The caller it omits is `scripts/session-context.sh`,
this repo's own live copy of context-kit's hook template, whose invocation logic
is the template's. The live invoking set, verified by an unsuppressed tree-wide
grep at this rev, is therefore: `context-kit/templates/session-context.sh`,
`scripts/session-context.sh`, `context-kit/bin/always-loaded.sh` (through
`CONTEXT_KIT_HOOK_CMD`, whose default it resolves and which
`scripts/context-config.sh` overrides), and this repo's close skill.

## What changes

### 1. The tool joins the `EMITTERS` registry as one arm with three modes

`queue-index` becomes a member of the non-gate arm class, registered in the
binary's emitter table, and resolved under the derived spelling
`--emit-queue-index` before the gate registry — outside `--list`, owing no
`.gate` descriptor, no `gates.list` registration and no `good/`+`bad/` fixture
pair, exactly as §The non-gate arm specifies. **Design-bearing.**

**The registry family is forced, not chosen, and knob bridging is what forces
it.** The binary carries two non-gate families: hardcoded top-level flags
resolved in `main`, and the emitter table keyed by projection name. Only the
second gets configuration — the table's third tuple element is the arm's bridged
knob list, which `--knobs` prints and which the battery runner's `--emit`
front-end resolves through the shell bridge. A hardcoded flag receives **no
configuration at all**, and `queue-index` is a configured tool: an arm that
cannot see `QUEUE_KIT_ICEBOX_SECTION` silently drops the icebox tally in every
consumer that configures a tier. So the emitter table is the only family that can
carry it.

**The three modes stay three modes on one arm**, selected from the arm's own argv
tail. This is the existing contract rather than a new allowance: the emitter type
is defined over an argv slice precisely "so a projection whose generator has a
write-in-place mode takes it as a flag rather than needing a second arm". The
modes keep their present names and grammars — `index` (with `--collapse-deferred`
a flag within it, not a mode of its own), `extent`, `icebox-candidates` — and
§bin/queue-index.sh's standing refusal to grow a fourth mode is inherited
unchanged, not re-argued.

**The bridged knob list is the section-name knobs, not `lib/queue.sh`'s derived
regexes.** `QUEUE_ACTIVE_RE`, `QUEUE_DEFERRED_RE`, `QUEUE_ICEBOX_RE` and
`QUEUE_SECTION_RE` are shell-internal derivations built from the section knobs,
not a configuration surface, and `QUEUE_LESSONS_RE` is fixed spelling. The arm
declares what the already-ported queue gates declare —
`QUEUE_KIT_QUEUE_FILE`, `QUEUE_KIT_ACTIVE_SECTIONS`, `QUEUE_KIT_DEFERRED_SECTION`,
`QUEUE_KIT_ICEBOX_SECTION` — plus the two this tool alone reads,
`QUEUE_KIT_ATTEND_CAP` and `QUEUE_KIT_ICEBOX_AGE_DAYS`. No knob is minted, none
is retired, and every default stays where its SPEC states it.

**The arm's name is honest about a stretch, and the stretch is precedented.**
`--emit-` reads as document emission, and `extent`'s two integers are a query
rather than a document. The class already broke that reading —
`--emit-close-surfaces` is a member in good standing with no stored projection at
all, on the rule that *a caller* is the requirement and a stored projection is one
shape of it. Renaming the family would be a gate-sdk unit and is not taken here.

### 2. The shell tool is deleted; the shim does not survive

`queue-kit/bin/queue-index.sh` is removed rather than retained as a dispatching
shim. **Design-bearing.** This applies the ported-emitter precedent rather than
taking a fresh decision: context-kit/SPEC.md already records of the last such port
that "the advisory bare mode did not survive the port, and its loss is the ported
script's deletion rather than a separate decision," and no ported emitter left a
shim behind.

A retained shim would be the worst of both: it keeps the 182 shell lines the port
exists to retire, it adds a second entry point into the emission path — which §The
non-gate arm forbids — and it re-buys the dependency floor cost against
TRAJECTORY.md objectives 1 and 6 on the session-context path where every session
pays it.

### 3. Every caller invokes the battery runner's `--emit` front-end

All four call sites move to `bash gate-sdk/bin/run-gates.sh --emit queue-index
[<flags>]`, the same front-end the four already-ported arms are reached through.
**Design-bearing.**

The front-end is not optional dressing: it is what sources the shell library and
supplies the bridged environment in front of the arm, so a caller that invoked the
binary directly would resolve platform defaults and silently ignore every
consumer override. §The non-gate arm states this as the class rule — a member
needing configuration is reached through a caller that sources the bridge — and
this arm is the class's most configured member.

Per call site:

- **`context-kit/templates/session-context.sh` and `scripts/session-context.sh`** —
  the stage-conditioned invocation is unchanged in shape (bare `index` for the
  first and last stages, `--collapse-deferred` otherwise); only the command
  changes. The `|| echo "(queue-index unavailable)"` fallback is **kept
  verbatim**, and its meaning is restated rather than widened: it now covers an
  unbuilt binary as well as an unresolvable tool, which is the same class of
  "this consumer has not finished installing" it always named.
- **`context-kit/bin/always-loaded.sh`** — its default `CONTEXT_KIT_HOOK_CMD`
  resolution currently probes two candidate *file* paths for the shell tool. It
  probes for the front-end instead, keeping the same two-candidate consumer-first
  shape and the same empty-on-unresolvable behaviour. Its own reading of the
  output is unchanged: it counts lines for the always-loaded meter and reads
  nothing else.
- **`scripts/context-config.sh`** — this repo's `CONTEXT_KIT_HOOK_CMD` override
  is rewritten to the front-end. **Mechanical.**
- **this repo's close skill** — its backlog-eviction step invokes
  `--emit queue-index --icebox-candidates`. The generic close template carries no
  `queue-index` reference and gains none; the invocation is consumer binding, and
  stays one. **Mechanical.**

### 4. The hook's own shell survives this port, and that is the answer rather than a deferral

The queue entry asks whether the session-context hook's shell survives under
TRAJECTORY.md objective 6. **It does, and the port still discharges the
objective's claim on this unit.** **Design-bearing.**

Objective 6 shrinks the script-interpreter surface *to the unavoidable*. The hook
script is a harness settings hook — a command the harness executes at session
start — so a script at that boundary is exactly the unavoidable residue the
objective admits, and whether that residue should itself change substrate is a
context-kit question about the hook, not a queue-kit question about the tool it
calls. What this port removes is the 182 lines *behind* the hook: the interpreted
surface shrinks by the whole tool while the hook keeps only its dispatch. Claiming
more than that would be the false-progress reading TRAJECTORY.md §The objectives
explicitly forbids — "no user-facing surface may state the dependency floor they
aim at as though it were reached."

### 5. Rendering tests move into the crate; the fixture-runner member shrinks to the bridge

`queue-kit/gate-tests/queue-index.test.sh` currently pins rendering behaviour —
tag-residue titles, multi-tag lead lines, the drain-exempt echo, the empty title,
the attend block's cap and overflow across both index renderings — by driving the
shell tool. Those assertions move into the ported module's own `#[cfg(test)]`
tests, where `check-crate-arms` runs them at commit time. **Design-bearing.**

The shell member is not deleted, and the reason is that the crate test cannot see
the thing most likely to break: it shrinks to asserting that **the front-end
resolves the arm and bridges its knobs** — that `--emit queue-index` reaches the
arm at all, and that a set consumer knob (a configured icebox section, a lowered
attend cap) actually reaches the rendering through the bridge. That is the seam
between shell and binary, it is invisible to a crate unit test, and it is the seam
every caller in §3 depends on.

### 6. `icebox-worklist-roadmap-blind` is unblocked, not implemented

The port is **behaviour-preserving**: no filter is added, no row is dropped, no
exclusion is printed. **Mechanical.** `icebox-worklist-roadmap-blind`'s deferral
ground was substrate — "a bash predicate patched into it is work the port throws
away" — and this port removes that ground: after it, the predicate lands in Rust
and is not thrown away. Its own open half is presentation (whether an excluded row
vanishes or prints as a stated exclusion), it is `[design-pending]` and
unpromoted, and the carrier entry's own words forbid pre-empting it. It therefore
stays deferred with its deferral re-grounded on this port having landed, and this
amendment decides nothing of it.

### 7. The generated projections this port stales are regenerated, not edited

Deleting a 182-line tracked shell file moves the footprint measurement, so
`docs/footprint.md` and the value rollup that reads it are regenerated by their
own commands. **Mechanical.** The enforcement map is untouched: `queue-index` is
not a gate, registers in no `gates.list`, and no `.gate` descriptor couples it —
verified, so no `couples=` line changes and the generated pre-commit hook's
trigger set is unaffected.

## Producers and consumers

**New interface:** one arm, `--emit-queue-index`, and one crate module behind it.
No new state, no new event, no new field, no new knob, no new file convention.

- **The arm.** *Producer:* the binary, dispatched from `main` before the gate
  registry through the emitter table's derived spelling — reachable in every
  consumer whose binary is built, with no enabling config of its own, which is the
  class's defining property. *Consumers, all four named and all four existing:*
  the session-context hook (template and this repo's copy), which reads the
  `index` rendering into the session brief at session start; the always-loaded
  meter, which reads the `--collapse-deferred` rendering's **line count** at the
  budget-accounting transition; this repo's close skill, which reads the
  `icebox-candidates` worklist at its backlog-eviction step; and a session
  performing a queue edit, which reads `extent`'s line range at the transition
  where it deletes or moves an entry — the caller §bin/queue-index.sh already
  relies on when it states that no queue-mutating tool is needed.
- **The `extent` mode's caller is a session, and that is stated deliberately.**
  §The non-gate arm demands a named caller, and `extent` has no in-tree invoker.
  Its caller is nonetheless named and load-bearing: the SPEC's refusal to ship a
  queue-mutating tool rests on it. A mode whose only caller is a session is a
  member in good standing under the class's own worked counter-instance, where a
  stage step through the front-end counts.
- **The bridged environment.** *Producer:* the battery runner's `--emit`
  front-end, which resolves the arm's declared knob list through the shell bridge.
  *Consumer:* the arm, at dispatch. Reachable everywhere the kit is vendored: the
  front-end is the same one the four existing arms already use, so no consumer
  gains a step it does not already have.
- **Every field has a named reader.** No field is added. Each mode's stdout
  grammar is preserved exactly, so every existing reader — the brief's embedder,
  the meter's `wc -l`, the eviction step's row reader, the editing session's two
  integers — reads what it reads today.
- **No gate reaches this arm in-process**, so §The non-gate arm's descriptor
  source-coupling rule does not attach. This is checked rather than assumed: it is
  the rule whose omission leaves a gate green while its projection goes stale, and
  the reason it does not apply is that this arm has no gate consumer at all —
  which is the very asymmetry the queue entry named as its open half.

**Red conditions — this delta narrows a corpus (a tracked file is pruned), so
each reader's red condition is named rather than its subject.** "A narrower corpus
can only remove violations" is false here and is the first argument this shape
reaches for.

- **`check-docs-cmd`** — reds on *finding some* command in governed docs that
  does not resolve. Monotone in the violation set, but the deletion **adds**
  violations rather than removing them: `README.md` and `queue-kit/README.md`
  carry runnable `bash queue-kit/bin/queue-index.sh …` lines that stop resolving
  the moment the file is gone. They are rewritten to the front-end in the same
  commit as the deletion.
- **`check-footprint-fresh`** — reds on *finding a difference* between the
  committed projection and a live recompute. Non-monotone with respect to the
  prune: removing a measured file makes the committed copy wrong. Cleared by
  regenerating, not by inspection.
- **`check-crate-arms`** — reds on a failing `clippy -D warnings` or a failing
  release test run over the whole crate. Monotone in failures, and it is the only
  reader holding the new module: `check-gate-binary-fresh` covers `.gate`-dispatched
  members alone, so a pure non-gate-arm addition is held by that gate plus the
  standing `build-native.sh` obligation and by nothing else.
- **`check-gate-substrate-parity` assertion B** — reds on *inequality* between
  the `.gate` descriptor set and the roster `--list` prints. Non-monotone in
  either direction. Cleared by the arm staying outside `--list`, which is the
  class's first defining property and the reason it is stated as load-bearing
  rather than stylistic.
- **`check-md-refs`** — reds on *finding some* markdown link resolving to no
  file. Monotone, and the prune adds violations: every link naming the deleted
  path is repointed or removed with it.
- **`check-knob-citation` / `check-knob-default-coupling`** — the first reds on
  *finding some* knob stated with its value outside its owning SPEC; the second
  on *finding none*, a knob with no literal default in its SPEC. The second is
  non-monotone and is named for that reason: it is what a design minting or
  relocating a knob would trip. This port mints none and moves no default, and
  the knob reads move from `lib/queue.sh` to the arm's declared list without
  changing where any default is stated.
- **`check-comment-tier` / `check-spec-pointer`** — red on *finding some*
  non-directive comment and on *finding some* `spec:` citation resolving to no
  heading. Monotone, and the prune adds violations: the deleted file's own
  `spec:` comments go with it, but every surviving citation of
  §bin/queue-index.sh must still resolve, so the section is **renamed rather than
  deleted** (see below).
- **`prose-filename-citation-liveness`, a known and deliberately unclosed
  residue.** `docs/posts/2026-07-31-…` names the shell path in a bolded code span
  with no link and no `§heading`, so it resolves under neither `check-md-refs` nor
  `check-spec-pointer` and **nothing reds** when the file goes. It is left as
  written: the post is immutable prose making a historically true claim about what
  a released version shipped. This is a live instance of that deferred entry's
  class, and naming it here is its attestation, not this unit's work.

## Existing sections updated

- **queue-kit/SPEC.md §bin/queue-index.sh** — owned by deltas 1, 2 and 5, and the
  section carrying the substantive change. It is **renamed to §The queue-index
  arm** and rewritten to describe an arm rather than a script: the three modes and
  their grammars survive verbatim (they are the contract this port preserves), the
  invocation becomes the front-end, the deletion of the shell tool is stated, and
  the bridged knob list replaces the derived-regex reads. Renamed rather than
  deleted because six sibling sections cite it by heading. Its non-load-bearing
  filter paragraph and its no-fourth-mode refusal carry over unchanged.
- **queue-kit/SPEC.md §The queue format, §The tag algebra, §Layout and
  configuration, §bin/queue-counts.sh, §check-queue-entry-budget, §The icebox
  tier** — owned by delta 1, each for its citation of the renamed heading.
  §check-queue-entry-budget's `--extent` parity sentence and §The icebox tier's
  refused `--headroom` mode both keep their reasoning; the latter's ground — that
  a `--headroom` mode "grows a shell tool this repo's port track retires" — is
  **re-read at merge**, since the tool is no longer shell and the refusal now
  rests on the no-fourth-mode rule alone.
- **queue-kit/SPEC.md §Testing** — owned by delta 5, for the fixture-runner
  member's narrowed scope.
- **context-kit/SPEC.md** — owned by delta 3, in four places: the session-brief
  block's description of where the queue index comes from, the icebox tier's note
  that it reads `--icebox-candidates` rather than the brief, the
  `CONTEXT_KIT_HOOK_CMD` default-resolution statement, and the always-loaded
  accounting row. This is the amendment's second component and the reason it is
  cross-component.
- **gate-sdk/SPEC.md §The non-gate arm** — owned by delta 1, for the ported-member
  roster gaining `queue-index` and for the one new fact the class learns from it:
  a member may be a *query* tool rather than a generator, which is the reading
  `--emit-close-surfaces` opened and this member completes. **gate-sdk/SPEC.md**'s
  entry-cap paragraph naming `bin/queue-index.sh` as a double-carrier of the
  line-cap logic is repointed to the arm.
- **docs/site-architecture.md §Generated projections** — owned by delta 7, and
  the update most easily missed. It names `queue-kit/bin/queue-index.sh` as a
  standing instance of a tool that gets no freshness gate because "its only
  consumer is a session with a shell". The **ruling is unchanged** — it still
  earns no freshness gate, because it still stores nothing — but its stated
  ground is now false as written, and is restated as: a tool with no stored
  projection has nothing to hold fresh. The roster's footprint row also changes
  under delta 7 and is regenerated.
- **lifecycle-kit/SPEC.md** — owned by delta 1, for its `[attend]`-injection
  sentence citing the renamed heading.
- **README.md and queue-kit/README.md** — owned by deltas 2 and 3, for the kit-map
  line and the three runnable command lines (see the `check-docs-cmd` red
  condition).
- **`context-kit/smoke/install.sh`** — owned by delta 2. Its assertion is
  predicated on the shell file's *presence*; it is retargeted to the front-end's
  resolvability, keeping the `(queue-index unavailable)` fallback assertion as-is.
- **TASK-QUEUE.md `queue-index-non-gate-arm-port`** — owned by the opening
  correction, for the caller set. **TASK-QUEUE.md
  `icebox-worklist-roadmap-blind`** — owned by delta 6, for its deferral ground.
  **TASK-QUEUE.md `queue-index-blocked-by-assertions`** — owned by delta 5. That
  entry's coverage hole (the blocked-by tag's re-echo, the ready/blocked marker)
  names `queue-kit/gate-tests/queue-index.test.sh` as where two `want()` lines
  would close it; delta 5 shrinks that file to the front-end/bridge assertions
  alone and moves rendering coverage into the ported module's own
  `#[cfg(test)]` tests, so the entry's fix location moves with it — a future
  session taking the entry adds its two assertions in the crate, not the shell
  fixture. **Found by this align audit**, not designed here: the entry's own
  defect is untouched, only where its fix lands.
- **docs mirror of every file above** — regenerated, not edited.

**Not updated, stated so the omission is read as a decision.**
`scripts/pack-installer.sh` needs no edit: it copies every kit root whole from a
derived roster, so the payload follows the tree with no per-file list to
maintain. No `.gate` descriptor changes, no `gates.list` line changes, and the
generated pre-commit hook is unaffected.

## The seam ruling

**Kit mechanism:** the arm, its three modes, the front-end invocation and the
knob bridging. All of it is generic — a queue-shaped file, section names taken
from config, two numeric limits — and it holds in any consumer that vendors
queue-kit.

**Consumer config:** unchanged, and that is the ruling's substance. The port
**preserves every knob and mints none**, which is what makes it a port rather than
a redesign; the derived regexes that disappear were never a configuration surface,
only `lib/queue.sh`'s internal spelling of knobs the arm now reads directly.
`CONTEXT_KIT_HOOK_CMD` stays a consumer knob and keeps its meaning — a command
line the consumer may override — with only its default's spelling changed.

**Private rule content:** none crosses. The one place this port could have
crossed the seam is the deferred tally, and §bin/queue-index.sh already holds the
line — the tally is generic over whatever `###` subsection names the file has,
"a hardcoded tally table would be consumer rule content" — so the arm inherits
that constraint rather than relaxing it under a compiler that would make a literal
table invisible. The `(top)` bucket for entries under no subsection is generic
grammar, not a name. No section name and no slug enters the crate as a constant —
both arrive through the bridge.

The **cost-class opener set** is the one literal that crosses, and it crosses
unchanged: it is already a kit literal in the shell tool, it is generic English
cost vocabulary rather than any consumer's rule content, and §bin/queue-index.sh
already rules the match advisory. The port neither promotes it to a knob nor
widens it. What the port does change is that it stops being *readable* in a
shipped script and becomes a compiled constant — which is objective 3's intent
rather than a seam problem, and is stated here so the change is a decision rather
than a side effect of the substrate.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls queue-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
