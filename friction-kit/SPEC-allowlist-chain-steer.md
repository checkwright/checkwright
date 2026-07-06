# SPEC amendment: allowlist-chain-steer

## What changes

One new rule in the generic ruleset, slotted **between rule 9 (read-only
pipeline auto-allow) and fall-through logging** — rule 10, renumbering
today's 10 to 11: **rule 10 — decorated allowlisted command**. When a
submitted command's leading token-sequence exactly matches a committed
**bare** allow entry (a `Bash(<cmd>)` entry with no `:*`/`*` glob) but the
command decorates it — `&&`/`;`/`|` chaining, `2>&1`, a trailing redirect —
the rule **blocks** with the corrective: *run it bare — the bare form is
statically allowed; the decoration forces a prompt.*

Rulings the queue entry deferred to scope:

- **The coupling question — reading `.claude/settings.json` crosses no new
  seam.** `FRICTION_KIT_SETTINGS` is already a kit knob, and two kit tools
  (`scan-prompts`, `compare-settings-allow`) already parse the committed
  allowlist. The new cost is a per-call read inside a `PreToolUse` hook;
  the guard's fail-open posture covers it — no `jq`, no settings file, or
  a parse error means the rule silently declines and falls through.
- **Block, not advise.** The dominance argument from rule 7 transfers: the
  rule fires only on commands that would prompt anyway (see the guard
  condition below), so blocking costs no additional interrupt and converts
  the prompt into a steer toward the durable habit. An advise
  (`guard_advise`) would *grant* the decorated command — wrong, because
  the decoration can carry arbitrary segments the allowlist never
  reviewed.
- **Never intercept a silent grant.** The harness matches per segment; a
  compound whose every segment matches the committed allowlist is granted
  without a prompt, and blocking it would be a regression. The rule
  therefore fires only when at least one non-leading segment fails to
  match any committed allow entry — segment matching reuses
  `compare-settings-allow`'s shell-glob semantics and `:*` normalization,
  lifted into a shared `lib/guard.sh` helper so the two stay one
  implementation.
- **Bare entries only for the lead.** A glob-headed family
  (`Bash(git log:*)`) usually coexists with allowlisted decorators and its
  decorated forms were not the attested recurrences; the recurring
  friction (three hits in one iteration, including the promoted
  `: > prompt-friction.log` truncation) was exact bare forms decorated
  with plumbing. Widening to glob leads is possible later without a new
  name; starting narrow keeps false steers at zero.

New names on governed surfaces: the ruleset function
`guard_rule_allowlist_chain` and the shared match helper
`guard_allow_match` in `lib/guard.sh` (feature litmus satisfied). No new
config knobs — the rule reads `FRICTION_KIT_SETTINGS`.

## Producers and consumers

- **Producer:** the consumer's `bash-guard.sh` (from
  `templates/bash-guard.sh`, wired as the `PreToolUse(Bash)` hook) invokes
  `guard_rule_allowlist_chain` after the auto-allow rules; the template
  gains the call at the new position. Enabled wherever the template is
  installed.
- **Consumers:** the agent session receives the corrective block through
  the hook protocol; `guard-tests/cases.tsv` consumes the new decision
  rows via `bin/run-guard-tests.sh`; `compare-settings-allow` becomes a
  consumer of the shared `guard_allow_match` helper.
- The friction log is untouched — blocked commands never reach
  fall-through logging (rule 10-now-11), same as every earlier rule.

## Existing sections updated

- `friction-kit/SPEC.md §The generic ruleset` — insert rule 10, renumber
  fall-through logging 10 → 11 (its "always last" note is unchanged).
- `friction-kit/SPEC.md §compare-settings-allow` — note the glob-match
  core now lives in `lib/guard.sh` (`guard_allow_match`), shared with
  rule 10.
- `templates/bash-guard.sh` — generic-ruleset call sequence gains the new
  invocation.
- `guard-tests/cases.tsv` — firing case (a decorated bare-allow lead with
  an unmatched segment) and non-firing cases: the bare form itself; a
  decorated command whose lead matches no allow entry (falls through); a
  compound whose every segment matches the allowlist (untouched — the
  harness grants it silently).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls friction-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
