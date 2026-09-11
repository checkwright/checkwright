# SPEC amendment: spawn-failure-verdict

## What changes

### (1) The bounded reader call tells a child that never started from one that did

The bounded call's error becomes typed, so a spawn failure and a wait failure are
two cases rather than one string {design-bearing}.

`proc::run_bounded` has one caller, the turn-end liveness hook, and its `Err` arm
currently folds three failures into one `String`: `spawn_target` refusing, the
`spawn` itself failing, and `try_wait` failing on a child that **did** start. The
hook's `read_liveness` maps all three to `None`, so each reads `unavailable`.

The call's error becomes typed. A **spawn** failure carries the operating system's
own error, and a **wait** failure is a separate case. The hook maps them apart:

- a wait failure on a started child reads **`error`**. It is a reader that ran and
  did not answer, and that is `error`'s existing definition word for word;
- a spawn failure reads delta 2's verdict.

The existing ground that a failed spawn is never `error` stands unchanged: `error`
names a reader that **ran**, and reporting it over a spawn that never started
would name a reading that was never taken.

### (2) A reader that resolved and could not start takes its own verdict

The verdict table gains a seventh row, `unstarted`, and `unavailable` narrows to
its name {design-bearing}.

**Operator direction, 2026-09-11, lead-relayed: `unstarted` allows, and the refusal set
is unchanged.** The alternative, `unstarted` refusing once under the `stop_hook_active`
bound `unresolved` takes, was put to the operator and not taken. It would close the
fail-open by widening a refusal set this section records as separately authorized.

A new verdict value, **`unstarted`**, for a reader whose argv resolved (the
default always does; an override passed the executable-bit predicate) and whose
spawn the operating system refused: `ETXTBSY`, `ENOENT` from an absent shebang
interpreter, `EACCES`, `ENOEXEC`, `ENOMEM`, `EAGAIN`.

| reading | the hook holds | decision |
| --- | --- | --- |
| `unavailable` | no reader resolved: an **override** that fails the resolution predicate — the default always resolves | log, exit 0 |
| `unstarted` | a resolved reader the operating system would not start | log, exit 0 |

`unavailable` narrows to its name. Today the verdict table says it is an
override that "resolves to nothing spawnable", while the section's prose also
files an override that "cannot be spawned" under it. Those are two cases with two
different fixes: a knob to correct, and a reader or host that refused to start.
One verdict name hid the difference, and that is the fail-open this entry
records: a reader that cannot start was indistinguishable from one never
configured.

Why a distinct name rather than a wider `unavailable` row: the section already
splits reader exit 2 into `corrupt` and `unresolved` because "the fix is a
different one". This split rests on the same ground.

**Why it allows.** The degradation posture governs it. `unresolved` refuses only
because it is the reader's own fail-closed verdict, and a spawn the host refused is
no reading at all, the class of `unavailable` and `error`. So the fail-open stays.
What changes is that it is no longer silent: `unstarted` and delta 3's `spawn` value
make it a distinct line the close-stage triage can count.

### (3) The record carries the spawn error

The line gains one key, **`spawn`**, placed immediately after `verdict`
{design-bearing}.

- **Value.** The operating system error's own rendering, sanitized to one token by
  the record's existing `sanitize` rule (for example `Text_file_busy_(os_error_26)`).
  No errno table is maintained, so the value is derived and never transcribed.
- **Where it is populated.** Only on `verdict=unstarted`. Every other firing
  writes `spawn=-`, so the field is never filled at a transition that has no
  reader for it.

The field set is already open, and a reader parses by key. So the key's position
moves no existing reader. `keys` stays last.

### (4) The module's reader stubs are never executed from a process that held their write descriptor

A stub is written by a child process, so the test process never holds a write
descriptor a sibling fork could inherit {design-bearing}.

`Scratch::stub` writes a script and the case spawns it at once. The write happens
inside a multithreaded process whose sibling test threads fork children of their
own.

A sibling that forks while the stub's write descriptor is open hands its child a
copy of that descriptor, and the copy lives until the child's `exec`. That is
true even with `O_CLOEXEC`, because the flag closes the copy only at `exec`, not
at the `fork`. If this case's spawn lands inside that window it fails with
`ETXTBSY`, the hook reads `unavailable`, and it exits 0.

**Measured at this amendment's authoring, not inferred.**

- **The rate.** The module alone, at default parallelism, went red in **8 of 30**
  runs.
- **The timeout is excluded.** Every red run finished in 104–106 ms, so the
  10-second reader bound cannot have fired. A timeout would also have read
  `error`, not the verdict observed.
- **The swallow is observed.** The captured log lines of the red cases read
  `verdict=unavailable decision=allow` over a stub reader that had resolved: the
  bounded call's `Err`, folded away by `read_liveness`.
- **Every failure is a stub spawn.** Five distinct cases went red, and each had
  just written its stub.

**What was not measured: the errno.** No syscall tracer was available.
`ETXTBSY` is the only spawn error a present, already-executable stub can
plausibly hit. Delta 3 is what confirms or refutes that at the first residual
red, which is why it lands with this fix rather than after it.

**The fix.** The stub file is written by a **child process**, and the test
process never opens it for writing, so no sibling fork can inherit a write
descriptor to it. What is refused, and why:

- **A retry on `ETXTBSY`.** In a test, it weakens the assertion the test exists
  to make. In the hook, it changes a refusing hook's behavior.
- **A lock.** It cannot reach the forks of other modules' tests.

The module also gains two things:

- **A pinned spawn-failure case.** An executable stub whose shebang names an
  interpreter that does not exist. The spawn fails with `ENOENT`
  deterministically, and the case asserts `verdict=unstarted`, a non-`-` `spawn`
  value, `decision=allow` and exit 0.
- **Legible failure messages.** Every assertion in the module on a firing's exit
  code prints that firing's log line in its failure message. A future red then
  names its verdict and its `spawn` value instead of reporting `left: 0`.

## Producers and consumers

- **The typed bounded-call error** (delta 1).
  - *Producer:* `proc::run_bounded`, on every firing that spawns a reader, which
    is every firing whose argv resolved. That covers the default (the knob
    unset, the shipped configuration) and every executable override, so the
    producer is reached in the deployed configuration and not only in tests.
  - *Consumer:* `read_liveness`, the call's sole caller, through the match arm
    that selects the verdict.
- **`verdict=unstarted`** (delta 2).
  - *Producer:* `fire`, from delta 1's spawn case.
  - *Consumer:* `fire`'s decision mapping, at the same match that maps every
    other verdict, where `unstarted` takes `allow` and exit 0.
  - *Readers:* the log line's `verdict` column, read by the close-stage triage at
    the close-surface drain — the transition where `unavailable`, `error` and
    `unresolved` are already read — for the question of whether the reader is
    mis-configured or refused to start. Also the module's pinned spawn-failure
    case.
- **`spawn=`** (delta 3).
  - *Producer:* `fire`, from delta 1's spawn case, and `-` on every other verdict.
  - *Readers:*
    - The close-stage triage at the same transition, asking whether a firing's
      failure was transient (`ETXTBSY`, `EAGAIN`, `ENOMEM`: the host, and a
      re-run clears it) or persistent (`ENOENT`, `EACCES`, `ENOEXEC`: the
      override needs fixing).
    - The module's assertion messages (delta 4), at a red, to name the cause the
      two flake filings could not.
- **Stubs written by a child** (delta 4): test-only. Its producer is
  `Scratch::stub`, and its consumer is every case that spawns a stub.
- **Red conditions** (causal-completeness point 5): no corpus narrows. What widens
  is the `verdict` value set, by one member, plus one key.
  - `delegation-kit/smoke/install.sh` asserts a green firing through an ordered
    glob whose `*` spans the inserted `spawn=-`. It stays green.
  - `scripts/gate-tests/subagent-stop-reader.test.sh` asserts named substrings on
    three arms. Its red condition is a different verdict on those arms, and
    `unstarted` is not one they produce.

## Existing sections updated

- `delegation-kit/SPEC.md` §The turn-end liveness hook, the verdict table and the
  "refuses on three of six arms" lead-in (deltas 1 and 2). The lead-in becomes
  "refuses on three of seven arms".
- `delegation-kit/SPEC.md` §The turn-end liveness hook, "`unavailable` and `error`
  allow on the degradation posture" (delta 2). It gains `unstarted` beside them, on
  the same posture.
- `delegation-kit/SPEC.md` §The turn-end liveness hook, "The default is spawned
  rather than called in process" (delta 1). The paragraph keeps its closing
  sentence, that a failed spawn is never `error`, and names `unstarted` as where a
  failed spawn now reads.
- `delegation-kit/SPEC.md` §The turn-end liveness hook, "`unavailable` is not
  retired by the default" (delta 2). The paragraph drops "or cannot be spawned"
  from `unavailable`'s surviving producers.
- `delegation-kit/SPEC.md` §The turn-end liveness hook, the log line grammar and
  its field list (deltas 2 and 3). The grammar gains the `unstarted` verdict value
  and the `spawn` key, and the field list gains a `spawn` bullet and a
  `verdict=unstarted` bullet, each naming its reader.
- `delegation-kit/SPEC.md` §The turn-end liveness hook, "The hermetic stub-driven
  lane" (deltas 2 and 4). The lane now covers seven arms, and the paragraph gains
  the stub-writing rule and its ground.
- `native/src/hook/stop_liveness.rs` (deltas 1, 2, 3 and 4). The change covers
  `read_liveness`, `FIELDS`, `fire`, the `spec:` comment claiming "a spawn that
  never started is `unavailable`", and the test module.
- `native/src/proc.rs` (delta 1): `run_bounded`'s signature and its `spec:` line.
- `docs/delegation-kit/SPEC.md` (all deltas). It is the generated mirror;
  regenerate it with `--emit docs-mirror --write`.

## Retired spellings

- None — no delta removes a literal. `unavailable` keeps its name with a narrower
  row, and a new value and a new key are additions.

## Definition of Done

- [ ] **Causal completeness** — every new state, event and interface has a
      named, reachable producer and a named consumer, and every new field has a
      named reader at a named transition.
- [ ] **Flake discharge measured, not asserted** — the full `cargo test --release`
      suite is run repeatedly, against the entry's recorded rate of one run in
      four to two runs in five. Any residual red names its `verdict` and `spawn`
      value in its message, and a residual red is read, not re-run.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section, and the merged spec reads as one coherent
      document.
- [ ] **Amendment deleted** — this file is removed on merge, and none remain for
      the component.
- [ ] **Removals propagated** — `## Retired spellings` holds, re-run by
      `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — a cross-component gap found in build is resolved that
      session.
