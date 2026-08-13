# SPEC amendment: diff-renderer

The crate's `diff`-normal-format renderer moves out of the one gate module that
had to build it and becomes a shared crate mechanism with a contract, ahead of
the seven ports that will each need it.

The unit is small and its whole value is in being taken *before* the first of
those ports rather than during it. Two of the three deltas below are things a
porting session would otherwise decide silently, one of them wrongly, at the
moment it is least able to notice.

## The facts this is sized on, probed at this rev

`normal_diff` is defined exactly once in the crate, at
`native/src/gates/lifecycle_registration.rs:14`, spanning lines 14–71: it takes
two line slices, walks an LCS table, and returns an allocated `Vec<String>` of
normal-format hunks. It performs no I/O and spawns nothing. Its unit tests live
in the same file's `#[cfg(test)] mod tests`, lines 170–198 — two tests, one per
hunk-kind and one for multi-line ranges. Eighty-seven lines in total, no
dependencies. Its sole production call site is line 154, in the same module.

**Not one of the seven prospective consumers is ported yet.** All of
`check-footprint-fresh`, `check-trajectory-fresh`, `check-enforcement-fresh`,
`check-value-rollup-fresh`, `check-docs-mirror-fresh`, `check-roadmap-fresh` and
`check-gate-tamper` are still shell. So this unit adds no consumer and proves
nothing about one; it removes a cross-kit reach from the path of whichever
cohort takes the first of them, and it settles two questions that cohort would
otherwise answer under port pressure.

The crate already has the convention this lands into: `ere.rs`, `proc.rs`,
`walk.rs`, `spec.rs`, `queue.rs` and `stages.rs` are one shared module per
shared mechanism, declared as a flat `mod` list in `native/src/main.rs`.

## What changes

**(1) `normal_diff` and its two unit tests move to `native/src/diff.rs`.**
[mechanical] A new module in the flat `mod` list, `pub(crate)`, with
`lifecycle_registration.rs` importing it the way its siblings import `ere` and
`spec`. The move is byte-preserving on the function body and the tests; nothing
about the LCS walk or the hunk format changes.

It is mechanical because the target convention already exists and the compiler
is the oracle for the whole delta. Its one non-obvious step is delta (2)'s.

**(2) The renderer gains a contract section, and the comment that binds it
repoints to the new owner.** [design-bearing] Today the function's `spec:`
pointer is `native/src/gates/lifecycle_registration.rs:11-13`, naming
lifecycle-kit/SPEC.md §check-lifecycle-registration — correct while the renderer
was that gate's private mechanism, and **false the moment it is shared**. A
relocated comment left pointing at a section that no longer owns the code is the
`spec:` one-line-binding defect exactly, and it is the defect the doctrine names
worst: prose relocated behind a tag rather than repaired.

So the section is created in the same commit that moves the function:
**gate-sdk/SPEC.md §The diff renderer**, a sibling of §The POSIX ERE matcher and
scoped the same way — the crate's one rendering of the `diff` normal format,
carrying the format and its cap and no consumer vocabulary. The moved comment
names it.

This ordering is load-bearing and not merely tidy. Once the
`spec_comment_surface` cohort lands, `check-spec-pointer` reads `*.rs` sources
and reds a pointer resolving to no heading. A commit that moves the function and
defers the section leaves a dangling pointer in the corpus of a gate that is
being ported in the same iteration.

**(3) The truncation divergence is settled here, once, rather than seven times
under parity pressure.** [design-bearing] The shell freshness gates all render
through the external `diff` program and cap the report — the shape is
`diff <(...) "$PROJECTION" | head -20 || true`, and it is identical across
`check-trajectory-fresh`, `check-enforcement-fresh`, `check-footprint-fresh`,
`check-value-rollup-fresh` and `check-roadmap-fresh`. The crate's renderer caps
nothing: `lifecycle_registration.rs:154` iterates the whole hunk sequence.

Both are correct for their own gate today and they cannot both be correct at a
port. Criterion 2 proves a ported member byte-identical to its shell original,
so the first freshness gate to port discovers a divergence on any case whose
diff exceeds the cap — and discovers it in the arm least likely to be exercised
by a fixture, since a `bad/` case is normally built with a small planted
difference. A latent parity trap that fixtures pass and the live tree trips is
the same shape §The first cohort already recorded once, arriving through the
report rather than through the corpus.

The ruling: **the renderer stays uncapped and the cap is the caller's, but the
cap is one crate constant rather than seven literals.** A renderer that
truncates is a reporter, and the one existing caller legitimately wants every
hunk. The constant's *value* is the shell's, so each port's parity run compares
equal rather than needing a per-member adjustment — which is the point of
settling it before the first port rather than at it. The value lives in the
module, cited by the new SPEC section and never restated in prose.

**(4) The no-spawn property is stated as a contract, not left as an accident.**
[mechanical] The shell form's renderer *is* an external program, and `diff` sits
on `GATE_SDK_PROGRAM_FLOOR`, so a porting session reading criterion 7 finds no
blocker and has every reason to reach for `Command::new("diff")`. The crate's
answer is already the right one and is currently unwritten: `native/src/proc.rs`
is the crate's one sanctioned spawn site, and the renderer is a pure function
that needs none. The section states it, so the cheapest wrong implementation is
refused by a rule rather than by whoever reviews that port.

**(5) `native-diff-renderer-hoist` moves to `## Done`.** [mechanical] Its
deliverable is the hoist and the contract, both complete at merge — unlike the
port entry, this one is not a corpus and carries no roadmap tag, so the terminal
move is a Done move rather than a demotion.

## Producers and consumers

**`native/src/diff.rs` — producer.** Created by the porting commit as a
`pub(crate)` module in `native/src/main.rs`'s `mod` list. Its **named consumer
today is exactly one**: `native/src/gates/lifecycle_registration.rs:154`, which
is also its only consumer before the move. The seven prospective consumers are
**named as prospective and are not producers of anything here** — none is
ported, so none imports the module, and this amendment adds no code for them.
That is stated rather than implied because a shared module with no second caller
is otherwise indistinguishable from speculative generality: what makes it
justified is the cross-kit reach it removes from a *later* unit's path, and that
justification is the queue entry's, already costed.

**The cap constant — producer and reader in the same commit.** The constant is
defined in `native/src/diff.rs` and read by nothing yet; the one existing caller
is uncapped by design under delta (3). A constant with no reader would be
removable under the field-with-no-reader rule, and this one clears it on a
different ground than usual: its reader is the SPEC section that cites it, which
is what makes the value a stated contract rather than dead code, and the first
freshness port is its first code reader. **If the merging session cannot state
that citation, the constant comes out and delta (3) becomes prose in the section
alone** — the rule is not bent to keep it.

**gate-sdk/SPEC.md §The diff renderer — producer of a heading with two named
readers.** `check-spec-pointer` resolves the moved comment's pointer against it,
and `check-md-refs` resolves any prose citation of it. Both readers are live
today against `*.sh` sources and both reach `*.rs` sources through the corpus
the `spec_comment_surface` cohort is porting in this same iteration, which is why
delta (2) makes the section a same-commit obligation rather than a follow-up.

**No knob is created, no descriptor changes, no gate's assertion moves.** The
crate's public surface is unchanged: `--list`, `--knobs` and `--reads` all
report the same rosters after the move as before, so
`check-gate-substrate-parity` assertions B and D are unaffected by construction,
and assertion E is unaffected because `native/` is outside every kit root
whatever modules it carries.

**The narrowing check.** This delta narrows nothing — no corpus loses a file, no
glob tightens, no prune is added. The one removal is a private function from one
module, whose sole reader moves with it in the same commit.

## Existing sections updated

- **gate-sdk/SPEC.md §The diff renderer** — *new*, owned by deltas (2), (3) and
  (4): the crate's one normal-format renderer, its locus, its uncapped contract,
  the cap constant its callers share, and the no-spawn rule.
- **lifecycle-kit/SPEC.md §check-lifecycle-registration** — owned by delta (2).
  The section currently owns the renderer as that gate's private mechanism; it
  keeps the gate's stale-block *report* and cites the new section for the format,
  so one fact keeps one owner.
- **gate-sdk/SPEC.md §Porting a gate to the binary substrate** — owned by delta
  (3), one line placing the cap where a session sequencing the freshness family
  will read it, since that is the family the divergence bites.
- **TASK-QUEUE.md `native-diff-renderer-hoist`** — owned by delta (5), the Done
  move.

## Definition of Done

- [ ] **Causal completeness** — the module's one live consumer is named and
      compiles against it; the cap constant either has its stated reader or is
      removed; the moved `spec:` pointer resolves to a heading that exists in the
      same commit.
- [ ] **Merged with no information lost** — the new section reads as the
      renderer's home to someone who never saw this amendment, and
      lifecycle-kit's section reads as the gate's own without a hole where the
      renderer used to be described.
- [ ] **Amendment deleted** — this file removed on merge. The none-remain
      assertion (`ls gate-sdk/SPEC-*.md`) is discharged **at the iteration**: a
      sibling gate-sdk amendment is in flight (`SPEC-comment-cohort.md`), so only
      the batch merging the last of the two can satisfy it.
- [ ] **Removals propagated** — no citation anywhere still places the renderer
      inside `lifecycle_registration.rs`; `cargo test` and `check-crate-arms`
      green, and `bash gate-sdk/bin/build-native.sh` run beside the battery
      because neither discharges the other.
- [ ] **Gaps filed** — anything found and not fixed routed to the gap inbox with
      its cost, never flagged and skipped.
