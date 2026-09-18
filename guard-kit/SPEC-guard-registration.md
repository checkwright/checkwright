# SPEC amendment: guard-registration

guard-kit's generic ruleset exists in three places: the numbered roster in
§The generic ruleset, the `guard_rule_*` functions `lib/guard.sh` defines, and the
dispatch order inside `guard_generic_rules`. They must agree and nothing checks
that they do. This amendment writes the convention into the section, so each
roster item names its function, and mints `check-guard-registration` to hold the
three in lockstep.

**Sized against the reach ruling** (doctrine-kit/DOCTRINE.md §De-literalization,
file-level reading, ruled this iteration). `guard_rule_*` and
`guard_generic_rules` are functions of a shipped vendored library, and the
template guard calls `guard_generic_rules` itself. They are public under the
ruling, so the owning section may name them, and a gate may hold those names.

**Probed at authoring, 2026-09-18.** `grep -c '^guard_rule_[a-z0-9_]*() {'
guard-kit/lib/guard.sh` returns 26 definitions. The `guard_generic_rules` body
(`guard-kit/lib/guard.sh:2019-2047`) makes 26 calls, and they run in the same
order as the definitions. §The generic ruleset has 27 top-level numbered items.
Items 1-26 match the dispatch order one for one. Item 27, *Fall-through
logging*, is not a rule function: `guard_log_fallthrough`, which the template
guard calls after `guard_generic_rules` (`guard-kit/templates/bash-guard.sh:20`),
does that work. `grep -n 'guard_rule_\|guard_generic_rules' guard-kit/SPEC.md`
returns nothing. So the tree does not already do what this amendment asks.

## What changes

### (1) §The generic ruleset states the registration convention {design-bearing}

**Not yet applied.** This text replaces the section's opening paragraph ("Rules
that encode … Order is load-bearing where noted."), keeping its first sentence:

> Rules that encode **harness behavior, shell-substrate behavior, or behavior over
> an artifact whose grammar a kit owns**, never any project's toolchain. Each rule
> is one `lib/guard.sh` function named `guard_rule_<name>`. Its roster item below
> names that function, and the template guard runs the ruleset through
> `guard_generic_rules`, one call per item in roster order. Order is load-bearing
> where noted. An item's number, its function and its dispatch position therefore
> say one thing three times, and `check-guard-registration` (§Testing) holds
> them equal.

**The item grammar the gate reads.** Each top-level numbered item carries exactly
one backticked `guard_rule_<name>` token, placed right after its bold title, for
example `1. **\`cd\` in a compound command** (\`guard_rule_cd_compound\`) —
blocked: …`. An item never names another rule's function. A cross-reference
between rules stays by number ("rule 14's walk"), as every one does today.

### (2) Each roster item names its function {mechanical}

Transcribe the token onto items 1-26. This pairing comes from the dispatch order
at `guard-kit/lib/guard.sh:2021-2046` and the item titles (`grep -nE '^[0-9]+\.
\*\*' guard-kit/SPEC.md`). Each member's value:

1 `guard_rule_cd_compound` · 2 `guard_rule_git_c_root` ·
3 `guard_rule_scratch_redirect` · 4 `guard_rule_abs_script` ·
5 `guard_rule_abs_prefix` · 6 `guard_rule_expansion` ·
7 `guard_rule_brace_glyph` · 8 `guard_rule_sed_file` ·
9 `guard_rule_find_glob` · 10 `guard_rule_cat_file` ·
11 `guard_rule_git_grep` · 12 `guard_rule_pgrep_self_match` ·
13 `guard_rule_bare_sleep` · 14 `guard_rule_git_mutation_under_producer` ·
15 `guard_rule_background_no_record` · 16 `guard_rule_truncate_scratch` ·
17 `guard_rule_append_scratch` · 18 `guard_rule_ro_pipeline` ·
19 `guard_rule_bounded_wait` · 20 `guard_rule_allowlist_chain` ·
21 `guard_rule_git_rewrite` · 22 `guard_rule_rm_tracked` ·
23 `guard_rule_script_interpreter` · 24 `guard_rule_grant_path_slot` ·
25 `guard_rule_emitter_write` · 26 `guard_rule_shell_wrapper`.

Re-wrap each lead line the token lengthens. Change no other wording.

### (3) Fall-through logging leaves the numbered roster {mechanical}

Item 27 is the template guard's closing call, not a rule. Leaving it numbered
would force the gate to special-case the last item, and a rule added after it
would silently take that exemption. **Not yet applied.** This text replaces item
27, as a paragraph directly after item 26 and not indented:

> **Fall-through logging closes every call and is no rule:** after
> `guard_generic_rules` returns, the template guard's `guard_log_fallthrough`
> appends anything neither blocked nor auto-allowed to the friction log. It runs
> last and never affects the decision.

Repoint every live citation of the old number, found with `git grep -n "rule
27"`: `guard-kit/SPEC.md:2487` "(rule 27)" becomes "(§The generic ruleset,
fall-through logging)" (its `docs/` mirror follows at delta 5), and
`guard-kit/guard-tests/cases.tsv:483`'s section comment "# rule 27 — anything
else falls through to the friction log" becomes "# fall-through logging —
anything else falls through to the friction log", matching items 1-26's own
"# rule N — …" convention and this delta's "is no rule" framing. The grep's
other hit, `.workflow/release-declarations.md:213`, narrates a past release
("Fall-through logging is now rule 27") and is left as is: a dated release
declaration records history, not a live citation.

### (4) `check-guard-registration`, a repo-local native gate {design-bearing}

**Placement: this repo's gates dir, not a kit `checks/`.** The subject is the
kit's own source against its own SPEC. An adopter never edits either, and a
consumer rule lives in the consumer's copied `bash-guard.sh`, never under
`guard_generic_rules`, so no consumer tree has an instance to red on. Three
standing grounds also rest on guard-kit registering no gates, and a kit-shipped
gate would falsify all three:

- guard-kit/SPEC.md:1401, where declining the RO-forms gate is the ground;
- guard-kit/SPEC.md:3288, why there is no `smoke/violation.sh`;
- the drift-kit README and SPEC:1661, which cite it as precedent.

A repo-local member leaves all three true. The precedent for a repo-local gate
specified in a kit SPEC is `scripts/check-kit-ref-liveness.gate` (canon-kit/SPEC.md
§Layout and configuration). The gate is born native under the CLAUDE.md
rule: a Rust module `native/src/gates/guard_registration.rs`, a `gates::REGISTRY`
row, and the descriptor `scripts/check-guard-registration.gate`:

```
# graph: couples=guard-kit/SPEC.md,guard-kit/lib/guard.sh,native/src/gates/guard_registration.rs dir=one valve=none tier=precommit
# spec: guard-kit/SPEC.md §check-guard-registration — the generic ruleset's numbered roster, its guard_rule_* definitions and guard_generic_rules' dispatch order agree one-for-one and in order, fail-closed on an unreadable section or dispatcher
```

It carries no `# install:` line, as no repo-local descriptor does. The couples
are literal paths, not the `kit:` prefix, which would fan the trigger out to every
kit's SPEC.

**No knob.** No deployed configuration would set one, since a vendored guard-kit
is not edited (causal-completeness point 1). The gate finds guard-kit's root the
way `--run-guard-tests` already does: the kit root whose basename is `guard-kit`
among the derived kit roots (`native/src/emit/run_guard_tests.rs:107`), so a
relocated `GATE_SDK_KIT_DIRS` moves with it. The positional form
`check-guard-registration [spec-file [lib-file]]` points the fixtures at a
synthetic pair.

**The guard-kit/SPEC.md section, not yet applied**, a new `### check-guard-registration`
under §Testing:

> ### check-guard-registration
>
> Invariant: §The generic ruleset's top-level numbered items, the `guard_rule_*`
> functions `lib/guard.sh` defines and the calls `guard_generic_rules` makes are
> one roster. It holds four assertions, each a finding at exit 1:
>
> - **A**, the items are numbered from 1 with no gap or repeat, because prose
>   cites rules by number;
> - **B**, every item carries exactly one distinct `guard_rule_<name>` token;
> - **C**, the items' tokens in roster order equal the dispatcher's calls in
>   order, and a finding prints the diff;
> - **D**, the set of defined `guard_rule_*` functions equals the set dispatched,
>   and each side's extra member is named.
>
> An item runs from its numbered lead line through every following blank or
> indented line, and ends at the first unindented non-blank line. A dispatcher
> line is a call `guard_rule_<name> "$cmd"`, the `local cmd="$1"` binding, or
> blank. **Resolution fails closed, at exit 2:** an unreadable SPEC or library,
> an absent section, a section with no numbered item, an absent or unterminated
> `guard_generic_rules`, or a dispatcher line of any other shape. The gate
> refuses to guess at a body it cannot read, because a guessed body passes a
> skipped rule. The clean line is `GUARD-REGISTRATION: clean (<n> rule(s): roster,
> definitions and dispatch order in lockstep)`. The gate is registered from this
> repo's gates dir rather than shipped in a kit `checks/`, because its subject
> exists only where guard-kit is authored. guard-kit still registers no gates.
>
> **Out of its reach, by construction: the inert classes a rule declares.** An
> item's `Declares sq dq hd` is transcribed from the rule's `guard_skeleton`
> call. It is not one value per rule: rule 6 builds two skeletons with different
> class sets, rule 18 four, and some rules skeletonize inside a helper. Only
> some items carry a declaration at all. An equality check would have no single
> satisfying value per member.

**Fixtures.** `scripts/gate-tests/check-guard-registration/{good,bad}/`, each with
a synthetic `SPEC.md` and `guard.sh` passed through `args`, plus `expect.txt`:

- `good/` has three rules in lockstep.
- `bad/` fires A, B, C and D in one tree: a numbering gap, an item without a
  token, two calls swapped, and a function defined but not dispatched.
- A sibling `scripts/gate-tests/check-guard-registration.test.sh` covers the
  exit-2 cases a one-pair harness cannot hold: section absent, dispatcher
  absent, dispatcher unterminated, and a foreign dispatcher line. This follows
  §check-lifecycle-registration's split.

**Registration.** Add `check-guard-registration` to `scripts/gates.list`.

**The fourth correspondence is declined here and filed.** The inert classes are
excluded on the ground stated in the section above. The gap was filed to
`.workflow/gap-inbox.md` in the commit that authored this amendment, so the
un-gated correspondence outlives the entry's Done move.

### (5) Regenerate the projections the new member feeds {mechanical}

Run the regen command each freshness gate prints on red. The members, from
`git grep -l check-docs-kit-parity`, a repo-local gate of the same shape:

- `scripts/git-hooks/pre-commit`;
- `docs/check-graph.html`;
- `docs/enforcement.md`;
- the mirror `docs/guard-kit/SPEC.md`, which deltas 1-4 stale.

## Producers and consumers

- **The roster token** (deltas 1, 2). The SPEC author writes it. Its reader is
  `check-guard-registration` assertions B and C, at every precommit that touches
  a coupled file.
- **`check-guard-registration`** (delta 4). The battery produces it through the
  `scripts/gates.list` registration and the generated hook. Its consumers are the
  battery's verdict and the roster-holding readers below.
  - **Roster-holding readers of the new name.** The `gates::REGISTRY` row is its
    dispatch: an unregistered name is `no_such_gate`. The meta-gates
    (gate-sdk/SPEC.md §The gate model) read the descriptor, clean line and
    fixture pair. The generated hook, graph and enforcement map, from delta 5.
    `.workflow/validate-baseline.txt` gains the scenario at validate's baseline
    capture, not here.
- **Point 5 (narrowing):** no delta narrows a corpus. Delta 3 moves one item out
  of the roster, and the reader it could red is this gate, whose count is
  born with the move.
- **Point 6 (every member's value):** delta 2 names each of the 26 members'
  tokens above. Delta 4's assertion D is satisfied by the same 26 names on both
  sides.

## Existing sections updated

- `guard-kit/SPEC.md` §The generic ruleset: the opening paragraph, the tokens on
  items 1-26, and item 27 (deltas 1, 2 and 3).
- `guard-kit/SPEC.md` §scan-prompts, the "(rule 27)" citation (delta 3).
- `guard-kit/guard-tests/cases.tsv:483`, the "# rule 27" section comment (delta 3).
- `guard-kit/SPEC.md` §Testing, the new §check-guard-registration (delta 4).
- `native/src/gates/guard_registration.rs`, `native/src/gates/mod.rs`,
  `scripts/check-guard-registration.gate`, `scripts/gates.list` and
  `scripts/gate-tests/check-guard-registration/` (delta 4).
- `scripts/git-hooks/pre-commit`, `docs/check-graph.html`, `docs/enforcement.md`
  and `docs/guard-kit/SPEC.md` (all deltas).

## Retired spellings

- None — no delta retires a spelling: the rule numbers 1-26 keep their numbers,
  and "rule 27" is repointed at its live citations by delta 3 rather than
  retired as a name.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness
      check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them
      (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it; the merged spec
      reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in
      `## Retired spellings` above, and `check-amendment-retired-spelling` runs
      each declaration against the whole tracked tree, not against the specs
      alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The gate lands with the fix** — the tokens, the fall-through move and the
      gate land in one commit, green before and after (the Enforcement-first
      rule).
- [ ] **The entry moves to Done before the drain stage** — at the batch that
      merges this amendment, which is before the drain stage.
