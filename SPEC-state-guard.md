# SPEC amendment: workflow-state-direct-edit-guard

Rules the operator question `workflow-state-direct-edit-guard`: can a direct edit
to the lifecycle state file that bypasses `lifecycle-kit/bin/enter-stage.sh` be
blocked? The entry's four findings are done and this amendment does not re-derive
them — content-based detection is impossible, most of the bypass is already closed
at commit time, a `PreToolUse` matcher on `Write|Edit` is the missing lever, and
the actual hole is the **uncommitted** window: the cursor is the worktree file's
last stamp, every reader reads the working tree, and every gate that would catch a
hand-stamp fires only at commit. A session that hand-edits and never commits moves
the stage cursor for its entire life and is never caught.

The entry left two things unsettled and marked itself `[design-pending]` on them.
Both are resolved here **from the governing specs**, not by preference.

## Unsettled point 1: which kit owns it

**Resolved: lifecycle-kit ships the guard; guard-kit moves by exactly one clause.**

The entry framed this as a tie — "guard-kit owns the tool-call guard mechanism
while lifecycle-kit owns the state file". guard-kit's own SPEC has already broken
that tie for its other consumers, and states the rule it used. §The guard
framework records delegation-kit as its second and third consumer and distinguishes
them precisely: `agent-budget-guard.sh` composes guard-kit primitives into a
`PreToolUse(Agent)` hook and is *"cite-only; no guard-kit mechanism moves for it"*,
while `agent-dispatch-guard.sh` is the one where *"guard-kit mechanism **does**
move … by exactly one clause"*.

So the shipped pattern is that **the kit owning the rule ships the guard**, riding
`lib/guard.sh` through the `GUARD_KIT_LIB` indirection, and guard-kit moves only
where its lib lacks a primitive. lifecycle-kit owns the rule and the file, so it
ships the guard and becomes the framework's fourth consumer.

guard-kit's clause here is exact and small: `guard_read_command` reads
`.tool_input.command`, which is Bash-shaped and the only reader the lib has. A
`Write`/`Edit` call carries `.tool_input.file_path`. That reader is the missing
primitive. Everything else the guard needs is already tool-agnostic —
`guard_block` writes to stderr and exits 2, `guard_advise` emits the PreToolUse
envelope, and neither reads a command.

## Unsettled point 2: the settings surface

**Resolved, and the entry's premise is wrong in a way that matters.** The entry
says *"the settings hooks are a pinned surface, so the unit changes one"*. They
are not pinned. `scripts/settings-pins.conf` carries exactly two pins, both
memory keys, and `check-settings-pins` never reads `.hooks`.
`.claude/settings.json` is not in `scripts/core-files.list` either.

What actually governs the hooks block is a **derivation**, which is a sharper
constraint than a pin and points at real work:
`gate-sdk/bin/enforcement-map.sh` reads `.hooks.PreToolUse` out of the tracked
settings file to emit the enforcement page's **Guards** section;
`check-enforcement-fresh` byte-compares that emission against `docs/enforcement.md`;
and `gen-value-rollup.sh` derives `docs/value.md`'s columns from that page's
headings, where the Guards count reads 3 today.

**Consequence, and it is a build instruction rather than a caveat**: registering
the hook reds two freshness gates until both projections regenerate. A pin would
have meant "do not touch"; the truth means "touch, then regenerate two
projections". The full fan-out and its regen commands are owned by
`docs/site-architecture.md` §Generated projections, and each gate prints its own
command on red.

## What changes

- **(D1) `guard_read_path` in `guard-kit/lib/guard.sh`** — the file-path
  counterpart of `guard_read_command`: read the hook payload once, extract
  `.tool_input.file_path`, return non-zero when absent so a matcher that fires on
  a call without one falls through instead of blocking. Generic over any
  path-bearing tool; names no path and no rule. **design-bearing** — the
  absent-field return contract is the whole design, and getting it backwards
  wedges every call the matcher covers.

- **(D2) guard-kit's fail-open-but-loud posture applies, and the SPEC says so.**
  §The guard framework already specifies that a guard which cannot enforce its
  rule allows the call and emits an advisory naming the rule, rather than denying
  or passing silently — with the delivery constraint that `guard_advise` is itself
  jq-backed, so a guard degrading *because* jq is absent must emit the envelope
  directly. D3's guard is an instance and inherits both halves. The SPEC's
  consumer paragraph gains lifecycle-kit as its fourth consumer, recorded on the
  same does-mechanism-move axis as the other three. **design-bearing**.

- **(D3) `lifecycle-kit/templates/workflow-state-guard.sh`** — the guard. Resolves
  `GUARD_KIT_LIB` with the existence guard the shipped pattern uses, reads the
  path via D1, and blocks a `Write`/`Edit` whose target resolves to the lifecycle
  state file, with a message naming `bin/enter-stage.sh` as the sanctioned writer
  and the reason (the stamp is the transition; a hand-stamp moves the cursor for a
  whole session before any gate can see it). Path comparison is resolved, not
  textual, so an absolute path, a `./` prefix, and a path through a symlink all
  match the same file. **design-bearing** — the path-equality rule is where this
  guard is defeated or not.

- **(D4) `.claude/settings.json`** — a third `PreToolUse` block, matcher
  `Write|Edit`, invoking the copied guard from `scripts/`. The alternation matcher
  is the shape guard-kit's own wiring template already ships for
  `ScheduleWakeup|CronCreate`. **mechanical** — one JSON block on an established
  shape.

- **(D5) `guard-kit/templates/settings-hooks.json`** — the wiring template gains
  the block as an optional third entry, on the same terms its comment already sets
  for the wakeup-guard ("drop the block to skip it"). Without this the template
  documents two of the three guard shapes guard-kit supports. **mechanical**.

- **(D6) Regenerate `docs/enforcement.md` and `docs/value.md`** and commit them
  with D4. Not optional and not a follow-up: D4 stales both, and the two freshness
  gates red until it is done. **mechanical** — two named regen commands, each
  printed by its own gate on red.

- **(D7) `lifecycle-kit/SPEC.md` §check-stage-evidence** — state the guard as the
  closure of the uncommitted-hand-edit window, and state the residual honestly:
  `--no-verify`, a human editing outside the agent tooling, and any writer that is
  not a `Write`/`Edit` tool call. **design-bearing** — an unstated residual reads
  as a closed hole, which is the failure mode this section's neighbouring
  honest-limit clauses exist to avoid.

## What this amendment deliberately does not do

**It does not attempt content-based detection**, which the entry ruled out on
grounds this amendment adopts rather than re-argues: a hand-written stamp is
byte-identical to an `enter-stage` one by design.

**It does not claim to close the window.** The guard reaches agent `Write`/`Edit`
calls and nothing else. The entry already named the residual as unclosable
in-repo, and D7 is where that lands in a spec rather than in a queue body that
gets cleared.

**It does not put the state-file path into guard-kit.** guard-kit ships the reader;
the path lives with the kit that owns the file. This keeps guard-kit free of a
dependency on a kit it does not otherwise know about, in the direction the shipped
consumers already run.

## The seam

Nothing private crosses. D1 is a field accessor over a harness payload shape —
generic mechanism, no rule content, no path. D3 names `.workflow/WORKFLOW-STATE.txt`,
but that is lifecycle-kit's **own** surface, already named throughout its gates and
its `bin/`, so the guard names nothing the kit did not already own. The
`GATE_SDK_WORKFLOW_DIR` indirection those readers use is honored, so a consumer who
relocated the workflow directory gets a guard that follows it.

The consumer-config seam is unchanged: an adopter who does not want the guard does
not wire D4's block, exactly as with the wakeup-guard. No knob is added, because
there is no value to configure — the path derives from lifecycle-kit's existing
resolution and the rule has no threshold.

## Producers and consumers

- **Producer of the guard verdict** — the harness, invoking D4's hook on every
  `Write`/`Edit` tool call. Enabling config is D4's settings block; without it the
  guard is inert, which is why D4 and D5 are deltas rather than documentation.
  This is the enabling-config point the causal-completeness check asks about, and
  it is the one that would otherwise be missed: shipping D3 without D4 yields a
  file that enforces nothing while looking like it does.
- **Consumer of the verdict** — the harness's PreToolUse dispatcher, reading exit
  2 as a block and the advisory envelope as an allow-with-context. Both shapes are
  guard-kit's existing contract; no new envelope is introduced.
- **Producer of the guard's input** — the harness payload on stdin. Its
  `.tool_input.file_path` field is the one D1 reads; the absent-field path is
  specified because `Write`/`Edit` are not the only tools an alternation matcher
  could later cover.
- **Named reader of the enforcement-map's new Guards row** — `check-enforcement-fresh`
  (byte-compare) and, downstream, `gen-value-rollup.sh`'s column join. Named
  because they are the readers that turn D4 into two red gates, and because they
  are the cross-component path a build session would otherwise discover the hard
  way.
- **Consumer of D7's prose** — the next session that reads the stage cursor and
  wonders what stops a hand-edit. The entry records the concrete instance: a close
  entry where `enter-stage`'s own refusal text openly offered a deliberate
  hand-stamp and the session declined on judgment alone. That session is the
  reader D7 has to reach, and the request is precisely to stop relying on it.

**Existing integration prose describing the prior flow**: `guard-kit/SPEC.md`
§The guard framework's consumer paragraph (D2) and its lib component contract,
which lists the primitives and must list `guard_read_path` beside
`guard_read_command`; `guard-kit/README.md`'s guard roster if it enumerates the
shipped guards; `lifecycle-kit/SPEC.md` §check-stage-evidence (D7).

No new persistent field or record is created — the guard emits a verdict and keeps
no state — so the every-field-has-a-reader obligation applies only to D1's read
field, whose reader is D3.

## Cross-component notice

This amendment changes the contracts of **three** components — guard-kit (a new
lib primitive and a fourth consumer), lifecycle-kit (a shipped template and a
SPEC clause), and the repo's own harness settings plus two generated docs
projections. That is over the audit stage's trigger threshold and is called out
here so the next stage entry does not have to discover it.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **The acceptance oracle, named per delta.** D1: a case in guard-kit's
      existing guard-test harness feeding a synthetic `Write` payload and a
      payload with no `file_path`, asserting the extracted value and the non-zero
      return respectively — the absent-field case is the discriminating one, since
      an implementation that returns success on a missing field passes the happy
      path and wedges in production. D3: cases for a block on the state file and a
      pass on an ordinary file, plus the path-equality cases the guard turns on —
      absolute, `./`-prefixed, and a path reaching the file through a different
      spelling — because a textual comparison passes the first case and fails the
      rest. D4+D6: `check-enforcement-fresh` and `check-value-rollup-fresh` green,
      with the Guards count moved 3 → 4, which is the only assertion that proves
      the hook is actually registered rather than merely written.
- [ ] **The negative oracle, run once at build** — with the hook wired, an agent
      `Edit` against `.workflow/WORKFLOW-STATE.txt` is refused. Named separately
      because no committed artifact records it and the unit test cannot: it
      exercises the harness's dispatch, not the guard's logic.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change retired;
      nothing dangles. Specifically: no merged section repeats the entry's
      "settings hooks are a pinned surface" claim.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not deferred).
