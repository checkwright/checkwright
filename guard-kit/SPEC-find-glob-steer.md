# SPEC amendment: find-glob-steer

## What changes

New generic rule in `lib/guard.sh`: a **listing-only `find`** is blocked
with the steer to the harness's Glob tool — the same shape as the `sed`-read
steer (a better form exists, and Glob returns paths registered for a later
Read). Inserted beside the sed rule and, like it, **placed before both
auto-allow rules**, so a bare listing meets the steer rather than a silent
read-only-pipeline grant.

Fire condition — all three, which is exactly the operand logic no allowlist
glob can express (the triage criterion's guard-rule case):

1. the segment leads with `find`;
2. it carries **no action predicate** — `-exec`, `-execdir`, `-ok`,
   `-okdir`, `-delete`, `-fls`, `-fprint`, `-fprint0`, `-fprintf` (find(1)
   semantics: fixed mechanism, not config);
3. its output has **no consumer** — the `find` is the command's only
   segment and carries no redirect.

A `find` piped into a consumer is a legitimate producer and is untouched
(the read-only-pipeline rule may still auto-allow it); an action-predicate
`find` is an executor, not a listing; a redirected `find` has a downstream
reader (and the bare-name scratch rule already polices bad targets). The
corrective message names the Glob tool and the reason.

Evidence: 7 fall-throughs logged in one iteration's friction log, every one
a plain listing under a kit directory (`find <dir> -type f`,
`find <kit> -name '*.test.sh'`).

## Producers and consumers

- Producer: the consumer's copied `bash-guard.sh` sourcing `lib/guard.sh`
  via the PreToolUse hook — the enabling config every guard-kit consumer
  already sets; no new wiring.
- Consumer: the harness permission layer (exit-2 + hook JSON), and the
  agent reading the corrective at the block transition.
- No new config fields: the action-predicate roster is find(1) mechanism
  and ships as a lib literal; a consumer needing different behavior shadows
  the rule in its consumer-rules section (the existing extension point).

## Existing sections updated

- §The generic ruleset: the rule is inserted with its ordering note (before
  both auto-allow rules — same reasoning as the sed steer's placement note,
  extended to name this rule).
- §Testing: decision-table rows land for both sides of the line — blocked
  (`find kit -type f`, a `-name` listing) and untouched (`-exec` and
  `-delete` forms, a pipe into a consumer, a redirected listing).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as
      one coherent document a reader who never saw the amendment can use
      alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
