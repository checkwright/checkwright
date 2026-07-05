# friction-kit — permission-friction reduction for agent sessions

A permission prompt the human approves is invisible to the agent — nothing
about it lands in the transcript, so the agent cannot notice, count, or fix
the friction it causes. The cost lands on the operator, silently and
per-prompt, and compounds as the command surface grows. The kit closes the
loop: a `PreToolUse` guard decides at call time (block with a corrective
message, steer to a better form, auto-allow the provably safe, log every
fall-through), a scanner ranks the logged prompt sources, a curation pass
keeps the committed allowlist durable and the per-user overlay small, and a
recurring close-stage triage step makes the whole loop a habit instead of a
one-off cleanup.

Extracted from the governance meta-layer of a private production platform.
The kit carries the guard framework, the harness-generic ruleset, and the
triage tooling; every project-specific guard rule (build-tool hygiene,
container concurrency, test-suite serialization) is consumer rule content
and never ships. Unlike the earlier kits, friction-kit registers **no
gates**: its runtime surfaces are hooks and advisory `bin/` tools, so
nothing joins `gates.list`; it follows gate-sdk's layout and smoke
conventions without depending on its registry.

## The friction loop

1. **Call time** — the consumer's `bash-guard.sh` (copied from
   `templates/`, wired as the `PreToolUse(Bash)` hook) inspects each
   command: block, steer, rewrite, auto-allow, or fall through. Every
   fall-through — exactly the set of commands that may have prompted — is
   appended to the friction log. An *approved* prompt is invisible to the
   agent, so this log is the only record.
2. **Close time** — `bin/scan-prompts.sh` filters the log against the
   committed allowlist and ranks what actually prompted, grouped by command
   pattern. Each recurring pattern is resolved by the **triage criterion**
   (below); `bin/compare-settings-allow.sh` lists the local-overlay entries
   a committed glob already grants (the deterministic prune set). Then the
   log is cleared — its named reclaim path.
3. **Steady state** — friction low and justified: the committed
   `settings.json` carries every durable pattern (reviewable, shared), the
   local `settings.local.json` stays near-empty, and the guard encodes the
   steering rules no static glob can express.

### The triage criterion

Allowlist and guard are two tools with distinct jobs — resolve each
recurring pattern by the criterion, never by defaulting to the allowlist:

- **Allowlist** (`Bash(...)` in the committed settings) when the command is
  safe *and already in the form to reinforce* — static, glob-matched,
  declarative.
- **Guard rule** when a *better form exists* and the agent should be steered
  to it, or when the allow/deny needs logic a static glob cannot express.
- **Habit change** (a noted convention) for true one-offs.

Caution: an allowlist entry can *mask* a steering opportunity — before
blessing a form, confirm it is the one to reinforce; if a better form
exists, steer to it in the guard rather than permitting the worse one.

Note: the harness matches the allowlist **per segment** of a compound
command, so a glob on the core command does not cover the `echo` banners,
`wc`, redirects, or `;`-chained diagnostics wrapped around it — one
unmatched segment re-prompts the whole line. The read-only banner/diagnostic
tools an agent habitually chains (`echo`, `wc`, `grep`, `ls`, `command -v`)
are therefore themselves legitimate allowlist entries; allowlist them, or
run the core command bare.

## The guard framework (`lib/guard.sh`)

Primitives a consumer guard composes; each emits the harness's
`PreToolUse` hook protocol:

- `guard_read_command` — parse the hook JSON on stdin, emit the command;
  **fail-open** on any parse problem (exit 0) so a guard can never wedge
  the agent.
- `guard_block <msg>` — exit 2 with the message on stderr. Every block
  message is self-describing: it names the offending pattern *and* the
  corrective form, so the why of a rule rides to the agent in the rejection
  itself, not in a code comment.
- `guard_advise <msg>` — allow, but feed the message back as
  `additionalContext` (steering without blocking).
- `guard_allow <reason>` — silent grant via `permissionDecision: allow`.
- `guard_rewrite <cmd> <reason>` — behavior-preserving rewrite via
  `updatedInput` (grant the better spelling of the same command).
- `guard_log_fallthrough` — append the (truncated, newline-flattened)
  command to the friction log; best-effort, never affects the decision.

Fail-open is the default posture. The one sanctioned fail-closed shape is a
deny-guard whose hook *matcher* already proves the tool identity (see
wakeup-guard): there, a logging or parse failure still denies.

## The generic ruleset

Rules that encode **harness behavior**, not any project's toolchain —
shipped as `lib/guard.sh` functions the template guard invokes. Order is
load-bearing where noted.

1. **`cd` in a compound command** — blocked: cwd drift plus a permission
   prompt no allowlist entry suppresses. Corrective form: absolute paths,
   or `git -C <dir>` for git.
2. **`git -C <repo-root>` when cwd is the root** — blocked: the absolute
   `-C` target matches no allowlist entry and re-prompts, while the bare
   `git` form is allowlisted. Pinned with a trailing space to the exact
   root, so `git -C <root>/subdir` and a foreign-repo `-C` are untouched.
3. **Bare-name scratch redirect** — a `>`/`>>` to a slash-free
   `*.err`/`*.out`/`*.log` target is blocked (it lands in the tracked tree
   and risks a `git add -A`); the no-slash class lets path-bearing targets,
   `/dev/null`, and fd-dups through. The corrective message points at the
   consumer's gitignored scratch dirs (`FRICTION_KIT_SCRATCH_DIRS`).
4. **Absolute-path execution of a known read-only repo script** — silently
   rewritten to the repo-relative form via `guard_rewrite` (the relative
   spelling is what allowlist globs match). The roster is the
   `FRICTION_KIT_RO_SCRIPTS` globs (default `check-*.sh`). Any other
   absolute repo-script spelling gets a corrective block. **Placed before**
   rule 5, which would otherwise steer the same command less precisely.
5. **Repo-root absolute prefix (non-script)** — any other command carrying
   the literal repo-root prefix is steered to the repo-relative spelling.
   `git` is excluded — rule 2 already owns its `-C` handling.
6. **Shell expansion / assignment** — a residual `${…}`/`$(…)`/`<(…)`/
   `$NAME` after single-quoted regions are stripped is blocked (the harness
   prompts on every expansion before allowlist matching). Only
   *single*-quoted regions are stripped: inside single quotes `$` is
   literal (`awk '$1'`), but a double-quoted `"$x"` still expands and must
   stay visible. A standalone `NAME=value` assignment is caught separately,
   since the expansion check only sees a *used* `$VAR`.
7. **Auto-allow `: > file` truncation** — a leading `:` plus redirect
   defeats the permission matcher, so it always prompts. Granted silently
   when the command is *only* `:` followed by redirects and every target is
   gitignored (`git check-ignore`): truncating scratch is safe; a tracked
   file must still prompt. The `git` subprocess is gated behind the rare
   `:`-redirect match; expansions (rule 6) are already blocked, so a
   surviving target is a literal path.
8. **Auto-allow read-only pipeline** — granted silently when every pipe
   segment leads with a roster binary (`FRICTION_KIT_RO_BINS`, default the
   grep/head/cat/find/jq family) and every redirect target is `/dev/null`
   or an fd-dup. Conservative by construction: command/process
   substitution, a leftover quote after stripping, any statement separator,
   a non-`/dev/null` redirect, or a `find` with a write action all refuse
   and fall through.
9. **Fall-through logging** — anything neither blocked nor auto-allowed is
   appended to the friction log. Always last; never affects the decision.

### Consumer rules

Project rules — build-cache hygiene, container-build concurrency,
test-suite serialization, disk reclaim, tool-specific steering — live in a
marked section of the consumer's copied `bash-guard.sh`, before the generic
ruleset. Two ordering disciplines from the source platform, kept as
guidance: a command about to be *blocked* must never first trigger a
side-effecting rule (place blocks before any reclaim/cleanup rule), and a
steering rule must precede the broader rule that would catch the same
command less precisely.

## scan-prompts

Advisory: surfaces recurring permission-prompt sources from the friction
log, filtering against the committed allowlist and the harness's built-in
read-only git/docker auto-allows, so only commands that actually prompted
remain. Grouped and ranked by pattern (leading binary, plus subcommand for
the common multi-command binaries). Triaged at close by the criterion
above. `--count` emits a compact `<patterns>/<occurrences>` token for a
drift-KPI consumer; an explicit file argument overrides the log path (test
capability).

## compare-settings-allow

Advisory: lists local-overlay allow entries already granted by a glob in
the committed settings — the deterministic prune-candidate set for the
close-stage audit. A committed pattern subsumes a local entry when the
local string matches it under shell-glob semantics; the harness `:*` prefix
idiom (`Bash(printf:*)` ≡ any `printf …`) is normalized to a trailing `*`
so one glob test covers both forms. Read-only — reports candidates, never
mutates (the operator prunes). It is the detector, not the policy: a
non-redundant local entry can still be one-off junk worth pruning by
judgment. `--count` emits the bare count.

## wakeup-guard (template)

Optional second guard, same framework, opposite posture: blocks
self-scheduled wakeups (`ScheduleWakeup`/`CronCreate`) unconditionally and
logs each attempt — a stored prompt re-fires in a later session as if the
user typed it, long after its premises are stale, and the scheduling call
is invisible at the moment it matters. Fail-closed (the matcher proves the
tool identity, so a logging failure still denies). The attempt log is
reviewed and deleted in the same close-stage triage pass as the friction
log. Deliberate scheduling stays possible by disabling the hook for a
session — the block is the default, not a capability removal.

## The close-stage triage step

`templates/close-triage.md` is the recurring step a consumer splices into
its close-stage skill — it fills the tooling-friction placeholder in
lifecycle-kit's close template. The step: run `scan-prompts`, resolve each
recurring pattern by the triage criterion; review and delete the wakeup
log if present; run `compare-settings-allow` and prune the listed local
entries, then by judgment prune the remaining one-off exact-string local
entries and promote recurring safe patterns to the committed settings as
globs; clear the friction log. Goal: the local set stays small and every
durable pattern lives in the committed, reviewable allowlist.

## Layout and configuration

```
friction-kit/
  lib/guard.sh              # primitives + generic ruleset functions
  bin/scan-prompts.sh
  bin/compare-settings-allow.sh
  bin/run-guard-tests.sh    # decision-table runner
  guard-tests/cases.tsv     # expected-decision <TAB> command
  templates/bash-guard.sh   # consumer copy: generic rules on, marked
                            #   consumer-rules section
  templates/wakeup-guard.sh
  templates/friction-config.sh
  templates/settings-hooks.json  # the PreToolUse wiring snippet
  templates/close-triage.md
  smoke/install.sh
```

Config follows the established kit pattern: copy
`templates/friction-config.sh` into the gates dir (or point
`FRICTION_KIT_CONFIG_FILE` elsewhere) and override any knob; defaults fill
what the consumer left unset. Knobs (platform values as defaults):

- `FRICTION_KIT_LOG` — default
  `${GATE_SDK_WORKFLOW_DIR:-.workflow}/prompt-friction.log`.
- `FRICTION_KIT_WAKEUP_LOG` — default
  `${GATE_SDK_WORKFLOW_DIR:-.workflow}/wakeup-attempts.log`.
- `FRICTION_KIT_SETTINGS` — default `.claude/settings.json`.
- `FRICTION_KIT_SETTINGS_LOCAL` — default `.claude/settings.local.json`.
- `FRICTION_KIT_RO_SCRIPTS` — array of globs eligible for the
  absolute→relative rewrite (rule 4); default `("check-*.sh")`.
- `FRICTION_KIT_RO_BINS` — read-only pipeline roster (rule 8); default the
  grep/head/cat/find/jq family.
- `FRICTION_KIT_SCRATCH_DIRS` — gitignored scratch dirs named in the
  rule-3 corrective message; default `(".tmp")`.

Both logs are per-iteration scratch: a consumer gitignores them even where
its workflow dir is otherwise committed.

## Testing

The gate contracts do not fit hooks (a guard speaks exit-2 + hook JSON, not
`OK:`/`FAIL:` lines), so the kit ships its own decision-table runner
instead of `gate-tests/`: `guard-tests/cases.tsv` pairs an expected
decision (`block`/`advise`/`allow`/`rewrite`/`fallthrough`) with a command;
`bin/run-guard-tests.sh` feeds each through the template guard as hook JSON
on stdin and asserts the exit code and output class, failing on any
mismatch. Every generic rule carries at least one firing and one
non-firing case (the fixture-pair discipline, transplanted).

`smoke/install.sh` copies the templates into the scratch consumer (guard
and config into the gates dir, hook wiring into `.claude/settings.json`,
log paths gitignored) and then drives one crafted payload directly through
the installed guard, asserting a block — the install is self-verifying.
There is no `smoke/violation.sh`: the kit registers no gates, so no
battery-reddening violation is craftable (gate-sdk/SPEC.md §Consumer smoke
makes that file conditional on exactly this).

One gate-sdk reshape rides this extraction: `gate_kit_roots` recognizes a
sibling kit by its `checks/` directory, which this gateless kit lacks —
undiscovered, its `smoke/install.sh` never runs under
`run-consumer-smoke.sh` and its `lib/` and `bin/` escape `check-shellcheck`'s
self-lint sweep. Discovery widens to siblings holding `checks/` *or*
`smoke/`; the gate-sdk SPEC and code change land in the same build commit as
this kit's `smoke/install.sh` (the harness requires an install script from
every discovered root, so the widening cannot precede it).

## What stayed on the platform

Every toolchain- and product-coupled guard rule: build-cache hygiene,
container-build concurrency and restart discipline, test-suite
serialization rosters, proactive disk reclaim, tool-preference steering,
and the platform's expanded read-only script roster beyond `check-*.sh`.
Its allowlist contents beyond the read-only core, and its memory-policy
write guard (a product ruling, not friction mechanism), stay behind with
it. The platform's guard remains the exemplar consumer: framework and
generic rules here, its rules in its own copied guard.
