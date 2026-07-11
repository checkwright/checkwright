# SPEC amendment: read-steer

## What changes

The generic ruleset's read-steer coverage (today `sed`-read only,
`guard_rule_sed_file`) extends to the three bare working-tree read commands
that dominated convention-hardening's prompt friction — 18 of that
iteration's 38 prompting calls. Enforcement-first disposition: the form to
reinforce is the dedicated tool, so each is a steer rule in
`guard-kit/lib/guard.sh` §The generic ruleset, never an allowlist entry.

- **`guard_rule_cat_file`** — a bare `cat <path>` (the whole command is the
  read, flags allowed) steers to the Read tool. Out of scope for the steer:
  `cat` feeding a pipe or heredoc, `cat` with redirection, multi-file
  concatenation — those are composition, not a file read the Read tool
  replaces.
- **`guard_rule_find`** — a print-only `find` (no `-exec`/`-ok`/`-delete`)
  steers to the Glob tool. A mutating `find` passes this rule (other rules
  or the permission prompt own it).
- **`guard_rule_git_grep`** — a `git grep` over the working tree steers to
  the Grep tool. A `git grep` naming a revision passes: it searches history
  the Grep tool cannot reach.

Each rule follows the `guard_rule_sed_file` shape: same steer verdict class
(deny with guidance naming the replacement tool), registered in the ruleset
dispatch, and shipped with guard-test fixtures per the decision-table
contract — for every rule, at least one steered form and one passing
boundary form (piped `cat`, `find -exec`, `git grep <rev>`) — run by
`guard-kit/bin/run-guard-tests.sh`.

No new knobs: the rules encode harness behavior (dedicated read tools exist
and are cheaper), not any project's toolchain, so they are generic-ruleset
content per the SPEC's consumer-rule boundary.

## Producers and consumers

- **The three rule verdicts** — producer: the consumer guard
  (`scripts/bash-guard.sh` here) sourcing `lib/guard.sh` and dispatching the
  generic ruleset on every PreToolUse Bash call; consumer: the harness hook
  surfaces the steer text to the calling session, which retries with the
  named tool. No new fields — the rules emit the existing verdict shape.
- **The fixtures** — producer: this unit; consumer:
  `run-guard-tests.sh` (the decision-table runner already registered in this
  repo's battery block).

## Existing sections updated

- `guard-kit/SPEC.md` §The generic ruleset — the rule roster gains the three
  entries beside `guard_rule_sed_file`, each with its steer/pass boundary
  stated.
- `guard-kit/README.md` — only if it enumerates the ruleset; a pointer-only
  README needs no edit.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
