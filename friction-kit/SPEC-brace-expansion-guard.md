# SPEC amendment: brace-expansion-guard

## What changes

One new rule in the generic ruleset, slotted **immediately after rule 6**
(it completes the same expansion family with the same machinery) and
renumbering today's 7–9 to 8–10: **rule 7 — unquoted brace glyph**. The
harness prompts on the bare `{` glyph before allowlist matching — exactly
the behavior class rule 6 pre-empts for `$`-expansions — so a `{` that
survives single-quote stripping (rule 6's existing strip pass, reused) is
handled by shape:

- **Bare `{}` placeholder** (`find … -exec cmd {} +`, `xargs -I{}`), when
  every residual brace in the command is exactly `{}`: silently rewritten
  via `guard_rewrite`, single-quoting each bare `{}` to `'{}'` —
  behavior-preserving (the shell passes a literal `{}` either way) and
  invisible to the harness matcher on the same premise as rule 6's
  single-quote stripping (inside single quotes the glyph is literal).
  Premise fallback: if the harness proves to prompt on quoted braces too,
  the rewrite becomes a corrective block steering to `-exec … +` with the
  quoted placeholder or an explicit shell loop — the shape detection is
  identical either way.
- **Git-ref shorthand** (`@{u}`, `@{upstream}`, `@{-n}`, `<ref>@{n}`):
  blocked; the corrective message names the explicit spelling —
  `origin/<branch>..HEAD` for `@{u}..`, the resolved ref or hash for
  reflog forms.
- **List/range expansion** (`{a,b}`, `{a..b}`): blocked; corrective form is
  the written-out expansion (`mkdir -p a/b a/c`) or a loop for long ranges.
- **Any other residual `{`**: blocked with the generic corrective —
  single-quote it if literal (awk/sed programs in double quotes), write it
  out if it expands.

**The over-block ruling** (the scope call the queue entry deferred): there
is no legitimate brace-glob convenience to preserve. Since the harness
prompts on *every* bare `{` before allowlist matching, each unquoted brace
already costs an operator prompt no allowlist entry can suppress —
`mkdir -p a/{b,c}` saves the agent four characters and spends one human
interrupt. The written-out form is glob-matchable and prompt-free, so
block-and-steer strictly dominates; the only braces that pass untouched
are single-quoted (literal) ones.

**Ordering is load-bearing, and closes a live hole in rule 7-now-8
(`: > file` auto-allow):** its design note reads "expansions (rule 6) are
already blocked, so a surviving target is a literal path" — but brace
expansion survives rule 6, so today `: > {a,b}.log` reaches the
`git check-ignore` probe with the *unexpanded* string, which tests the
wrong path. With the brace rule placed before both auto-allow rules, the
literal-target premise becomes true for braces as well. The renumbered
rule 8's design note is updated to cite both rules 6 and 7.

New name on a governed surface: the ruleset function
`guard_rule_brace_glyph` in `lib/guard.sh` (feature litmus satisfied).

## Producers and consumers

- **Producer:** the consumer's `bash-guard.sh` (from
  `templates/bash-guard.sh`, wired as the `PreToolUse(Bash)` hook) invokes
  `guard_rule_brace_glyph` in generic-ruleset order; the template gains the
  call at the new position. Enabled wherever the template is installed — no
  new config knob.
- **Consumers:** the agent session receives the block/rewrite decision
  (corrective message or `updatedInput`) through the hook protocol;
  `guard-tests/cases.tsv` consumes the new decision rows via
  `bin/run-guard-tests.sh`. The friction log is untouched — blocked and
  rewritten commands never reach fall-through logging (rule 9-now-10),
  same as every earlier rule.
- No new fields, files, or knobs; every branch of the rule is exercised by
  a named guard-test case.

## Existing sections updated

- `friction-kit/SPEC.md §The generic ruleset` — insert rule 7, renumber
  7–9 → 8–10, update rule 8's "already blocked" design note to cite rules
  6 *and* 7, and the §Consumer rules ordering guidance's rule references
  if any shift.
- `templates/bash-guard.sh` — generic-ruleset call sequence gains the new
  invocation.
- `guard-tests/cases.tsv` — firing cases per shape (`git log @{u}..`,
  `echo {1..5}`, `mkdir -p a/{b,c}`, rewrite case `find . -exec rm {} \;`)
  and non-firing cases (`awk '{print $1}'`; an already-quoted
  `find . -exec grep foo '{}' +`; a brace-free command).

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
