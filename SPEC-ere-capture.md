# SPEC amendment: ere-capture

**The stage-entry path runs two ERE interpreters.** `--enter-stage` classifies a
locked linked worktree by the pid that `LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE` captures
from its lock reason. The crate's engine has no capture group, so the capture runs
through `bash -c '[[ $2 =~ $1 ]]'` and reads `BASH_REMATCH[1]`
(`native/src/emit/enter_stage.rs` `capture_group_one`). For the same reason, the knob
validator reads compilability off bash's `[[ =~ ]]` exit status
(`native/src/knobs/lifecycle_kit.rs` `ere_compiles`). Bash's `=~` is the host's
`regcomp`, which accepts GNU extensions that `native/src/ere.rs` refuses. So a pattern
can be judged by one dialect and matched by another. The only thing asserting that
they agree is the port-time cut.

**Measured at spec, 2026-09-18.** Two points were settled by measurement:

- **Seven production `proc::run("bash"` sites.** The deferred entry said six but
  listed seven. Only these two are ERE sites. Of the rest:
  - The two `kill -0` probes (`evidence.rs`, `emit/wait_probe.rs`) reach a builtin
    `std` cannot spell. gate-sdk/SPEC.md §Fail-closed contract rules `bash -c` the
    honest route while the crate carries no `libc`. They stay, filed as a costed gap.
  - `emit/wait_probe.rs`'s launcher, `emit/port_blockers.rs`'s `type -t` query and
    `emit/pub_index.rs`'s consumer extractor are bash because bash *is* their subject.
- **No Pike-VM is owed.** The knob is already specified as having *exactly one*
  capture group (lifecycle-kit/SPEC.md §Layout and configuration). The validator
  enforces only *at least one*, through a hand scan for an unescaped `(`. A single
  group that stands in the top-level concatenation splits the pattern into a prefix,
  the group and a suffix. The POSIX subpattern rule can then be answered with the
  existing span engine: the whole match is leftmost-longest, then each subpattern
  from left to right takes the longest span. No per-thread slot vectors are needed,
  and no general submatch arbitration.

Every configured pattern fits that shape. Point 6 below enumerates them.

## What changes

### (1) The engine gains a fifth item: a one-group capture

`native/src/ere.rs` gains one public item beside the four in gate-sdk/SPEC.md §The
POSIX ERE matcher {design-bearing}. **Not yet applied.**

- **The item.** A compiled one-group pattern type. `compile` takes a pattern.
  `capture(&self, hay) -> Option<(usize, usize)>` returns the group's byte span within
  the leftmost-longest whole match, or `None` where the whole pattern does not match.
  The item's name is build's to choose. Its surface is this compile-and-capture pair
  and nothing else.
- **The admitted shape.** The pattern holds **exactly one** unescaped `(` outside a
  bracket expression. Its group stands in the pattern's **top-level concatenation**:
  it is not the operand of a `*`, `+`, `?` or interval, and it is not inside a
  top-level alternation. The group's own body is any POSIX ERE the engine already
  accepts, alternation included. Any other shape is an `EreError` naming which rule
  failed:
  - no group;
  - a second group;
  - a quantified group;
  - a group inside an alternation.

  Each is refused rather than approximated. A quantified group's POSIX span is its
  *last* iteration, and an alternated group may not participate at all. Both are
  cases the split below cannot answer, so a refusal is the fail-closed form of "not
  implemented".
- **The semantics, stated because they are the contract.** Let `[s, e)` be the
  leftmost-longest match of the whole pattern (`find`). The group span `[i, j)` is:
  - the **largest** `i` in `[s, e]` such that the prefix matches exactly `[s, i)` and
    some `j` exists for which the group matches exactly `[i, j)` and the suffix
    matches exactly `[j, e)`;
  - then, for that `i`, the **largest** such `j`.

  This is the POSIX XBD 9.1 subpattern rule ("each subpattern, from left to right,
  shall match the longest possible string") applied to the three subpatterns the
  shape admits. An exact match honours `^` and `$` at their **absolute** subject
  positions, as `find_from` does, and never at the sub-span's edges.
- **The acceptance oracle is a differential run against bash**, on the ground this
  section already gives for the awk oracle: authoring an engine's tests and its
  implementation from one understanding proves nothing. The unit arm compares
  `capture` against `[[ $s =~ $p ]] && BASH_REMATCH[1]` over a generated
  pattern-and-subject cross product. It runs at `LC_ALL=C`, and the pattern crosses
  in argv, never interpolated into the script. The generator must cover:
  - a prefix and a suffix that compete with the group for the same bytes
    (`(a*)a*`, `a*(a*)`, `(a|ab)(c|bcd)`-shaped splits, with the second group
    replaced by literal text);
  - an empty group;
  - anchors in the prefix and the suffix;
  - bracket ranges.

  A divergence is a commit-time red under `check-crate-arms`. Where bash's `regcomp`
  and the stated rule disagree on a case, build stops and resolves it here before
  landing. That is a design gap, not a test to relax. The oracle is test-scoped, so
  the production binary spawns nothing for it.

### (2) The worktree classifier captures through the engine

`capture_group_one` in `native/src/emit/enter_stage.rs` is replaced by the one-group
capture of delta 1, compiled once per `--enter-stage` run {mechanical}. The
classification table in lifecycle-kit/SPEC.md §bin/enter-stage.sh is unchanged: a
matched reason yields the captured substring as the pid, and an unmatched one is
**unclassified**. The `bash` spawn at that site is deleted. **Not yet applied.**

### (3) The knob validator judges the pattern with the engine that will run it

The `LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE` arm of lifecycle-kit's table validator
(`native/src/knobs/lifecycle_kit.rs`) compiles a non-empty value with delta 1's
`compile` {mechanical}. It refuses under the kit's malformed-config lead line, with
the `EreError` text in the finding. `ere_compiles` and `declares_group` are deleted,
and so is the `bash` spawn. The finding still names the knob and the value, as today.
**Not yet applied.**

The validator's unit test gains a **shipped-pattern case**. The kit default is empty,
so the case uses the non-empty patterns this tree ships: the value in
`scripts/lifecycle-config.knobs`, read from that file rather than restated, and the
smoke pattern. Each must be admitted by delta 1's `compile`, and each must capture
its pid from a matching reason. A later engine change that would refuse a pattern a
consumer already runs then reds at `check-crate-arms`, not at someone's stage entry.

This is a **tightening**, and it is declared. The following patterns were accepted
before and are now refused at exit 2:

- a pattern using a GNU extension that the host `regcomp` accepted (`\w`, `\b`,
  `\<`, a backreference);
- a pattern with two or more groups;
- a pattern whose group is quantified or sits inside an alternation.

Each would previously have been either matched in a dialect the crate refuses, or
captured by a rule nothing in the kit states.

### (4) The spec passages that ground the bash route are re-phrased

These re-phrasings replace the text they name and never append to it {mechanical}.
**Not yet applied.**

- **gate-sdk/SPEC.md §The POSIX ERE matcher.** The owed-engine sentence ("…and no
  substitution engine or capture-group replacement") keeps its substitution half. The
  public-surface list gains the fifth item. "There is no `replace`, no `replace_all`,
  and no capture-group accessor" becomes "no `replace` and no `replace_all`", followed
  by one clause naming this item's reader. The "no future consumer can turn a match
  test into a substitution" sentence is scoped to substitution, because the API
  foreclosure it argued was written before a capturing consumer existed. The
  promotion-trigger paragraph's "fourth item" becomes "a further item".
- **lifecycle-kit/SPEC.md §bin/enter-stage.sh.** The paragraph opening "The lock
  pattern is consumer configuration, so it is *interpreted* and never transported"
  is rewritten: the capture is the engine's one-group item. The two-interpreter
  honest limit is deleted, because it no longer holds.
- **lifecycle-kit/SPEC.md §The stage-machine adapters.** The sentence that reads
  compilation "off bash's own `[[ =~ ]]` status" becomes: compiled by the engine
  that will match it, and refused for any shape outside delta 1's.
- **lifecycle-kit/SPEC.md §Layout and configuration**, the knob bullet: "a POSIX ERE
  with exactly one capture group" gains "standing in the top-level concatenation",
  and cites gate-sdk/SPEC.md §The POSIX ERE matcher for the admitted shape.

## Producers and consumers

- **The one-group item (delta 1).** Its producer is `native/src/ere.rs`. Its
  consumers are the classifier (delta 2), which reads the span to slice the pid out
  of the reason, and the validator (delta 3), which reads only `compile`'s `Result`.
  The span's two fields are both read by the classifier's slice. Enabling config: a
  non-empty `LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE`, which this repo's
  `scripts/lifecycle-config.knobs` sets, so the producer is live and not test-only.
- **The refusal (delta 3).** Its producer is the table validator, at the kit's first
  knob read in a process. Its consumer is every lifecycle-kit arm and gate through
  the malformed-config exit 2 (gate-sdk/SPEC.md §The knob file).
- **Roster-holding readers.** No knob, gate, arm or tag name is minted. New `// spec:`
  lines bind to §The POSIX ERE matcher or §bin/enter-stage.sh, which meet
  `check-comment-tier`'s directive shape. `walk.rs`'s dependency allowlist is
  untouched, because no dependency is taken.
- **Point 5 (narrowing).** Delta 3 narrows the accepted pattern set. Its readers'
  red conditions:
  - The validator reds on a refused shape.
  - `lifecycle-kit/smoke/install.sh` and `lifecycle-kit/gate-tests/boundary-worktree-refusal.test.sh`
    red if their configured pattern is refused (a non-zero config exit where they
    expect classification). Point 6 shows none is refused.
  - `native/src/knobs/lifecycle_kit.rs`'s unit test asserts `ere_compiles` and
    `declares_group` directly. It is rewritten onto delta 1's `compile`:
    `a \([0-9]+\)` refused as no group, and `(["` refused as uncompilable.
- **Point 6 (members).** The obliged corpus is every pattern configured for the knob
  in the tracked tree, enumerated by
  `git grep -n "LOCK_PID_RE\|capture_group_one(re\|RE='\|ere_compiles(\"\|let re = r" -- lifecycle-kit native scripts`.
  Each has one top-level unquantified group, so each is admitted with its capture
  unchanged:
  - `scripts/lifecycle-config.knobs`: `^claude agent [^ ]+ \(pid ([0-9]+) start [0-9]+\)$`
    captures the digits after `pid `.
  - `lifecycle-kit/smoke/install.sh`: `^held by pid ([0-9]+)$`.
  - `lifecycle-kit/gate-tests/boundary-worktree-refusal.test.sh`: `^testharness \(pid ([0-9]+)\)$`.
  - `native/src/knobs/lifecycle_kit.rs` test: `^held by pid ([0-9]+)$`.
  - `native/src/emit/enter_stage.rs` test: the repo pattern above.

## Existing sections updated

- `gate-sdk/SPEC.md` §The POSIX ERE matcher: the fifth item, its admitted shape,
  semantics and bash differential, and the re-phrasings (deltas 1 and 4).
- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh, §The stage-machine adapters, §Layout
  and configuration (delta 4).
- `native/src/ere.rs`: the item, its parser support for recording the one group's
  place in the top-level concatenation, and the bash differential arm (delta 1).
- `native/src/emit/enter_stage.rs`: `capture_group_one` and its test, rewritten
  onto the engine; the test's name and `// spec:` line lose "through bash" (delta 2).
- `native/src/knobs/lifecycle_kit.rs`: the validator arm and its unit test (delta 3).
- `.workflow/release-declarations.md`: one bullet declaring the lock-pattern
  tightening of delta 3 (delta 3).
- `TASK-QUEUE.md`: `ere-matcher-capture-groups-unowned` moves to Done at merge, in
  the build session that merges this file, before the drain stage is entered
  (all deltas).
- `docs/gate-sdk/SPEC.md`, `docs/lifecycle-kit/SPEC.md`: generated mirrors,
  regenerated (all deltas).

Roster produced by
`git grep -n "capture_group_one\|ere_compiles\|declares_group\|LOCK_PID_RE\|capture-group"`
over the tracked tree, excluding `TASK-QUEUE.md` and `.workflow/`. It is a floor that
build re-derives.

## Retired spellings

- `capture_group_one` — the bash-backed capture, replaced by delta 1's item (delta 2).
- `ere_compiles` — the bash-status compile probe (delta 3).
- `declares_group` — the hand scan for an unescaped `(` (delta 3).

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The
      causal-completeness check holds for the item, the classifier and the refusal.
- [ ] **One interpreter**: `grep -rn 'proc::run("bash"' native/src/emit/enter_stage.rs native/src/knobs/lifecycle_kit.rs`
      returns nothing.
- [ ] **Differential green**: the bash oracle arm runs under `check-crate-arms`
      with no divergence, and the competing-prefix cases are in its generator.
- [ ] **Merged with no information lost**: the grounds for the admitted shape
      (the POSIX subpattern rule and why quantified and alternated groups refuse)
      survive in §The POSIX ERE matcher.
- [ ] **Queue move placed before the drain stage**: the Done move lands in the
      session that merges this file.
- [ ] **Amendment deleted**: this file is removed on merge, and none remains at
      the root for this iteration once its last batch lands.
- [ ] **Gaps filed**: the libc route for `kill -0` goes to the gap inbox as a
      costed gap, and so does any cross-component gap build finds.
