# guard-kit

Permission-friction reduction for coding-agent sessions. A `PreToolUse` guard
decides at call time — **block** with a corrective message, **steer** to a
better form, **rewrite** to the allowlisted spelling, **auto-allow** the
provably safe, or log the fall-through — a scanner ranks what actually
prompted, a curation tool finds the redundant local overrides, and a
close-stage triage step makes the whole loop a habit.

Why: a permission prompt the human approves is invisible to the agent — nothing
lands in the transcript, so the agent cannot notice, count, or fix the friction
it causes. The cost lands on the operator, silently and per-prompt, and
compounds as the command surface grows. The kit closes the loop by making the
fall-through set — exactly the commands that may have prompted — the one thing
that *is* recorded. See [SPEC.md](SPEC.md) for the framework, the generic
ruleset, and the triage criterion.

Unlike the other kits, guard-kit registers **no gates**: its surfaces are
hooks and advisory `bin/` tools, so nothing joins `gates.list`. It follows
gate-sdk's layout and smoke conventions without depending on its registry.

## Install

Vendor the kit beside [gate-sdk](../gate-sdk/), then:

1. Copy the guard framework into your gates dir (default `scripts/`):

   ```bash
   cp guard-kit/templates/bash-guard.sh      scripts/bash-guard.sh
   cp guard-kit/templates/wakeup-guard.sh    scripts/wakeup-guard.sh   # optional
   cp guard-kit/templates/guard-config.sh    scripts/guard-config.sh
   ```

   Add your project's block/steer/allow rules in `bash-guard.sh`'s marked
   consumer-rules section (before the generic ruleset). The generic ruleset and
   hook primitives stay in the vendored `lib/guard.sh`.

2. Wire the hooks — merge `templates/settings-hooks.json` into
   `.claude/settings.json` (the `bash-guard` on `PreToolUse(Bash)`; the optional
   `wakeup-guard` on `ScheduleWakeup|CronCreate`).

3. Gitignore the two scratch logs (`.workflow/prompt-friction.log`,
   `.workflow/wakeup-attempts.log`) — both are per-iteration, cleared at close.

4. Splice `templates/close-triage.md` into your close-stage skill (it fills
   lifecycle-kit's `tooling-friction triage` placeholder).

Configuration follows the established kit pattern — override any knob in
`guard-config.sh` (log paths, settings paths, `GUARD_KIT_RO_SCRIPTS`,
`GUARD_KIT_RO_BINS`, `GUARD_KIT_SCRATCH_DIRS`); defaults are this repo's
layout.

## Use

```bash
bash guard-kit/bin/scan-prompts.sh                  # rank what prompted, filtered by the allowlist
bash guard-kit/bin/scan-prompts.sh --count          # <patterns>/<occurrences> token (drift KPI)
bash guard-kit/bin/compare-settings-allow.sh        # local-overlay entries a committed glob already grants
```

## Test

```bash
bash guard-kit/bin/run-guard-tests.sh               # decision-table over the generic ruleset
```
