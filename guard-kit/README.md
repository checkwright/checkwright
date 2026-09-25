# guard-kit

Permission-friction reduction for coding-agent sessions. A `PreToolUse` guard decides at call time — **block** with a corrective message, **steer** to a better form, **rewrite** to the allowlisted spelling, **auto-allow** the provably safe, or log the fall-through — a scanner ranks what the allowlist failed to cover, a curation tool finds the redundant local overrides, and a close-stage triage step makes the whole loop a habit.

Why: a command no allowlist entry matches is decided **out of band** — by interrupting a human, or by a model asked to judge the call — and that decision is invisible to the agent either way, so it cannot notice, count, or fix the friction it causes. The cost is paid per call, out of the operator's attention or out of latency and tokens, and compounds as the command surface grows. The kit closes the loop by making the fall-through set — exactly the commands nothing granted — the one thing that *is* recorded. See [SPEC.md](SPEC.md) for the framework, the generic ruleset, what the steering buys, and the triage criterion.

An installer-vendored tree does not carry this file. The payload withholds each kit's `SPEC.md` and its `smoke/`, and every `SPEC.md` link on this page is repointed at the location `GATE_SDK_SPEC_BASE_URL` names when the payload is packed; with no base set the link stays relative (gate-sdk/SPEC.md §Consumer payload).

Most of what guard-kit ships is not a gate: its surfaces are hook members of the gate binary and templates. It registers exactly one, which holds the door binding its own steer messages and its settings templates depend on — and, over the surfaces `GUARD_KIT_DOOR_ROOTS` names, holds a door on your own pages to a declared audience.

## Install

Vendor the kit beside [gate-sdk](../gate-sdk/), then:

1. Copy the config template into your gates dir (default `scripts/`):

   ```bash
   cp guard-kit/templates/guard-config.knobs scripts/guard-config.knobs
   ```

   Register the gate it ships in your `gates.list`:

   <!-- gate-roster:begin -->
   ```
   check-door-binding
   ```
   <!-- gate-roster:end -->

   It resolves through gate-sdk's registry path (your gates dir first, then each kit's `checks/`), and its `# graph:` manifest puts it in the generated pre-commit hook.

   No guard script is copied: the shell guard, the optional wakeup-guard and escalation-guard are binary arms, wired by pointing a hook's `command` field at `bash gate-sdk/bin/run-gates.sh --hook shell-guard` (or `--hook wakeup-guard`, `--hook escalation-guard`). Where the binary is absent the front end fails open and the guard steers nothing.

   Your project's own block/steer/allow rules are a command you write, named by `GUARD_KIT_CONSUMER_RULES_CMD` in `guard-config.knobs`: the shell guard runs it before the generic ruleset, on the harness's own hook protocol, and `--guard-json` on the gate binary is its toolkit (guard-kit/SPEC.md §Consumer rules).

2. Wire the hooks — merge `templates/settings-hooks.json` into `.claude/settings.json` (the `shell-guard` on `PreToolUse(Bash|PowerShell)`; the optional `wakeup-guard` on `ScheduleWakeup|CronCreate`; and an optional third block showing the path-shaped shape a consumer kit's own guard registers under — lifecycle-kit's `workflow-state-guard` is the shipped instance).

   Then review `templates/settings-allow.json` against guard-kit/SPEC.md §The recommended allowlist, and union the entries you accept into `.claude/settings.json`'s `permissions.allow`.

3. Gitignore the two scratch logs (`.workflow/prompt-friction.log`, `.workflow/wakeup-attempts.log`) and their drain companions (`.workflow/*.drain`, `.workflow/*.drain.part`) — both logs are per-iteration, drained at close.

4. Splice `templates/close-triage.md` into your close-stage skill (it fills lifecycle-kit's `housekeeping` slot, close step 4).

Configuration is a knob file — override any knob in `guard-config.knobs`, one `NAME = value`, `NAME[] = element` or `NAME[key] = value` line each (gate-sdk/SPEC.md §The knob file), and `--emit knob-roster` on the gate binary `GATE_SDK_NATIVE_BIN` names prints every default (log paths, settings paths, `GUARD_KIT_CONSUMER_RULES_CMD`, `GUARD_KIT_RO_SCRIPTS`, `GUARD_KIT_RO_BINS`, `GUARD_KIT_RO_FORMS`, `GUARD_KIT_SCRATCH_DIRS`, `GUARD_KIT_SEARCH_TOOLS`, `GUARD_KIT_BREADTH_PROBES`, `GUARD_KIT_BREADTH_DECLARED`, `GUARD_KIT_DOOR_ROOTS`); defaults are this repo's layout, and the probe set, the declaration map and the door-root set all default to empty because their contents are your project's vocabulary, not the kit's. Drop a tool from `GUARD_KIT_SEARCH_TOOLS` (guard-kit/SPEC.md §Layout and configuration) when your harness build does not carry it, so the `find` and `git grep` steers never name a tool that is not there. A member you add to `GUARD_KIT_RO_BINS` also takes a `GUARD_KIT_RO_FORMS` declaration of its write and execute forms (guard-kit/SPEC.md §The generic ruleset, rule `ro_pipeline`), or the read-only pipeline grant withholds it.

## Use

Run these arms with the gate binary `GATE_SDK_NATIVE_BIN` names:

```bash
. "${GATE_SDK_ROOT:-gate-sdk}/lib/gate.sh" && gates="$(gate_native_bin_spelled)"
"$gates" --emit scan-prompts          # rank what nothing granted, filtered by the allowlist
"$gates" --emit scan-prompts --count  # <patterns>/<occurrences> token (drift KPI)
"$gates" --emit compare-settings-allow  # local-overlay entries a committed glob already grants, those a probe proves too broad, and those naming a script that does not exist
"$gates" --rewrite [--regex] [--expect <n>] [--] <find> <replace> <file>…  # replace text in tracked files, printing every changed span
```

`--emit scan-prompts` takes an optional log path, which overrides `GUARD_KIT_LOG` and composes with `--count` in either order; `--` ends option processing, so a log path spelled with a leading dash is still reachable. An unrecognized `-`-prefixed argument is a refusal at exit 2 — there is no per-arm `--help`, because a non-gate arm's usage lives here and under the gate binary's `--help`.

## Test

Run this arm with the gate binary `GATE_SDK_NATIVE_BIN` names:

```bash
. "${GATE_SDK_ROOT:-gate-sdk}/lib/gate.sh" && gates="$(gate_native_bin_spelled)"
"$gates" --run-guard-tests    # decision-table over the generic ruleset
```
