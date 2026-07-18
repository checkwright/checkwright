---
release: v0.6.0
---

# Checkwright v0.6.0

*2026-07-18*

Checkwright is the verification layer under agent orchestration, and this
release extracts the stage cursor to a single source. The lifecycle stage a
unit of work is in is now read from the evidence-stamp file's last stamp — the
one surface that already proved each stage was entered — and the queue header
narrows to the iteration name alone. The redundant `[stage:]` header field and
the queue-write that maintained it are retired. No gate landed or got stricter,
so this is a behavior release; residual-field healing keeps a pre-upgrade header
inert, so a clean upgrade reconciles nothing mechanically.

## Tightened gates

None — no gate landed or got stricter. The stage-machine gates
(`check-stage-entry`, `check-stage-evidence`, `check-evidence-manifest`) changed
the surface they read — the queue header's `[stage:]` field gives way to the
state file's last stamp — without tightening what a consistent tree must
satisfy.

## Renamed knobs

None — nothing was renamed or removed.

## Behavior changes

The whole of this release lands here, the fixed section for what shifts that no
gate scans.

- **stage cursor** — the lifecycle stage is now derived from the evidence-stamp
  file's last data line (the shared `lifecycle_current_stage` / `ek_state_stage`
  adapters), not the queue header's `[stage:]` field. Four readers move
  together: evidence-kit's manifest coupling, lifecycle-kit's entry and evidence
  gates, context-kit's session-context hook, and delegation-kit's statusline all
  read the stage from the state file. A consumer that scripted against
  `[stage:]` in the header reads the last stamp instead.
- **enter-stage.sh** — the flip dies: entering a stage no longer writes the
  queue, so the entry stamp is the sole transition, a departing session writes
  nothing, and no uncommitted stage state can strand a merge. Stage motion never
  touches the queue.
- **queue header grammar** — queue-kit retires the `[stage:]` field, narrowing
  the header to `## Iteration: <name>`. The surviving name-axis readers strip an
  optional trailing bracketed field, so a pre-upgrade header still carrying
  `[stage:]` parses to the bare iteration name — the retire is
  blind-upgrade-safe.

## Upgrading

Sync the vendored kit directories wholesale at `v0.6.0` and regenerate the
generated artifacts (the pre-commit hook and the graph projection), then run the
full battery. Residual-field healing means a `TASK-QUEUE.md` header still
carrying `[stage:]` needs no edit — it reads as the bare iteration name — though
you may drop the field at leisure.

**No allowed reds.** The Tightened gates section is empty: a clean upgrade turns
no gate red. The behavior changes above are declared for reading, not a
mechanical scan. If a gate reds that this note does not name, the upgrade smoke
was supposed to catch it first —
[open an issue](https://github.com/checkwright/checkwright/issues), because that
is a defect in the release rather than work for you.
