# SPEC amendment: selector-knob-union

Queue entry: `run-knob-union-selector-narrowing`, cut 1b of `config-bridge-resolution-cost`. It
is split out and ranked into `config-bridge-floor` under **operator direction, 2026-09-14,
lead-relayed**. The operator ruled that unit set on the `--only` single-gate floor.

**One scope decision was escalated at this stage and landed as a decision, not a direction.** It
is the iteration intent oracle's call, relayed by the lead on 2026-09-14: **`--only` narrows, and
`--for` keeps the tree union** (§The `--for` alternative, refused). The entry's naming of `--for`
came from the scope survey's *inferred* field, which no command established. Withdrawing it
retracts an unverified claim; it does not narrow anything the operator settled.

## The question, and the figures re-measured

`bin/run-gates.sh` resolves one bridged environment for the `--run` arm. What it resolves is the
arm's declared roster: the runner's own four knobs plus the `EVERY_REGISTERED_KNOB` sentinel, which
`emit::knobs` expands to every knob that every member of the **tree's** registry declares
(gate-sdk/SPEC.md §The non-gate arm). A selector run pays for the whole registry, even though the
dispatcher hands each child only its own slice (§run-gates).

**Re-measured at this stage, best of three, at this revision:**

| Set resolved through `gate_knob_env_set` | Names | ms |
|---|---|---|
| The tree union (today's `--only check-core-files`) | 178 | 718–736 |
| The runner's own four | 4 | 32–35 |
| Runner's own plus `check-core-files`' two | 6 | 37–39 |
| `--for`'s selection inputs: runner's own plus `@every-couples-knob` | 14 | 95–96 |

The scope survey's figure for resolving the member's own knobs alone, 28 ms for `check-core-files`,
is consistent with the third row once the runner's own knobs are added.

## What changes

### (1) Under `--only`, the registry sentinel expands over the selected members

gate-sdk/SPEC.md §The non-gate arm, in the paragraphs on how `--run`'s declared knob roster is
derived, changes the registry sentinel's member set from *the tree's registry* to *the members the
arm's argv selects from the tree's registry* {design-bearing}. **Not yet applied:**

> **Under `--only` the union narrows to the selection, and it is derived from the same rule as the
> selection.** When the arm's argv carries `--only`, the sentinel's member set is the names
> between `--only` and the `--` that ends the list (or the end of argv). With two or more names,
> the set is those names **intersected with the tree's registry**. A sole name joins the set
> whether or not the registry holds it, which is the sole-name widening §run-gates already
> states. Without `--only` — a bare run, `--for`, or any other arm that declares the sentinel —
> the set is the whole tree registry, unchanged.
>
> **Why the narrowed union is sufficient.** The dispatcher builds each child's environment by
> filtering the union by that child's own declared roster (§run-gates). A child is dispatched only
> if `select_only` picked it. That is the registry intersection, or the sole name, so the set of
> children is contained in the sentinel's member set. Every knob a dispatched child declares is
> therefore in the union, including a couples-knob sentinel it declares, which the bridge expands
> because it is in the child's roster. The runner's own reads under `--only` are its four named
> knobs plus descriptor *names* from `registry::couples_knob_names`, and none of them needs a
> member's knob value.
>
> **Why the intersection, and not the typed names.** An unregistered name among two or more is
> refused by the runner (§run-gates). If its knobs were in the union, a crate member whose owning
> kit is not vendored would reach the bridge's does-not-define refusal first. The caller would get
> that message instead of the runner's *is not registered in* refusal. That is the starter-profile
> failure the registry scope exists against, arriving through a typo.
>
> **What a selector run gives up, stated because it changes a verdict.** A knob that an
> *unselected* member declares, and that cannot resolve, no longer fails a `--only` run. It still
> fails a bare run, which resolves the whole registry, and it still fails a `--only` naming that
> member. A selected member gets the same bridge verdict it gets in the full battery. That is the
> declared-knob discipline read forward: the runner does not invoke an unselected member, so
> resolving its knobs was a read nobody consumed.

The one implementation stays `registered_members` in `native/src/emit/mod.rs`. Its sole-name
branch generalizes to the name list, so the widening and the narrowing are one function, as
§run-gates' *scope and selection answer the same question* requires. `port-blockers` declares the
same sentinel, and its grammar carries no `--only`, so its expansion is unchanged.

### (2) The `--for` alternative, refused

The same section gains the refusal beside delta 1, so a later reader does not take `--for`'s
whole-registry union for an oversight {design-bearing}. **Not yet applied:**

> **`--for` keeps the tree union, and the ground is that its selection reads bridged values.**
> `select_for` expands each member's trigger through `GATE_KIT_ROOTS_REL` and through the values of
> its `knob:` tokens. `--knobs` answers before any knob resolves, so it cannot compute that
> selection in one round. Two shapes were refused:
>
> - **A two-round bridge.** A sentinel the bridge expands by re-asking `--knobs` once the
>   selection inputs have resolved. That is new protocol in `lib/gate.sh`, and the gate-sdk cut of
>   `config-seam-static-format` deletes that library. It would serve a selector with no hot path:
>   its callers are one smoke line, one fixture test and a README example, and the generated hook
>   bakes its own arrays.
> - **A one-round over-approximation in the crate.** It would be a second trigger matcher, and
>   §run-gates defines exactly one.
>
> **The residue is costed, not flagged.** A `--for` run keeps paying the tree union, about 730 ms
> at this cut against roughly 95 ms plus the selected members' knobs for an exact narrowing. That
> cost is part of the bridge floor `config-bridge-resolution-cost` carries, and it is discharged
> with that entry at the gate-sdk cut, which retires the bridge.

### (3) §run-gates states the narrowing beside the widening

The §run-gates paragraph *The declared-knob union tracks the widening* gains one closing sentence
{mechanical}. **Not yet applied:**

> The same rule narrows the union with two or more names: those names intersected with the
> registry scope the sentinel, so a selector run resolves only what it dispatches
> (§The non-gate arm).

### (4) Directive comments, tests and the landing measurement

Delta 4 covers the directives, the tests, the timing and the release note {mechanical}.

- **Directives.** The directive on `runner::KNOBS` ("every registry member's added") and the
  `registered_members` directive restate the member set from deltas 1 and 2. A comment that
  becomes untrue is rewritten or deleted, never kept.
- **Unit tests** in `native/src/emit/mod.rs` pin the member set for six cases: a bare `--run`, a
  sole registered name, a sole unregistered name, two registered names, a registered and an
  unregistered name, and a name list ended by `--` with forwarded arguments that look like member
  names.
- **Hermetic cases** in `gate-sdk/gate-tests/run-arm-contract.test.sh`: a `--only` run of a member
  succeeds while an unselected member's knob is unresolvable, and a bare run in the same sandbox
  exits 2.
- **Timing.** The landing commit carries before and after best-of-three timings for
  `run-gates.sh --only check-core-files`.
- **Release note.** A **Behavior changes** bullet in `.workflow/release-declarations.md` says that
  `--only` now resolves only the selected members' knobs. A misconfigured knob belonging to a gate
  you did not name no longer fails the run, and a bare run still refuses on it.

## Producers and consumers

- **The narrowed `--knobs --run` answer** (changed interface). *Producer:* `emit::knobs` →
  `expand` → `registered_members`, reading the arm's argv. The front-end composes that argv as
  `--run --gates-dir <dir> --only …` (`bin/run-gates.sh`). *Consumers:* `gate_knob_env` in
  `bin/run-gates.sh`'s `exec_arm`, which resolves it, and the `--run` arm's `child_knobs` filter
  over the resolved union.
- **No new field, state or knob.** The argv read already exists for the widening.
- **Red conditions (point 5), because delta 1 narrows the resolved corpus.**
  - *The child's unset-knob refusal* (`walk::knob_wire`, exit 2) reds on a declared knob absent
    from the union. It is **not monotone**: narrowing can add this red. It is cleared by
    construction rather than by inspection. The dispatched set is contained in the sentinel's
    member set (delta 1's sufficiency paragraph), and the unit and hermetic cases in delta 4 hold
    it.
  - *The bridge's refusals* (does-not-define, element shape; §lib/gate.sh) red on a violation
    inside the resolved set. They are monotone, so narrowing only removes reds, and the removed
    ones are delta 1's declared verdict change.
  - *The runner's `--only` refusals* red on an unregistered name or a broadcast `--`. Neither reads
    a knob value, and the intersection keeps them the first refusal a typo meets.
  - *`--for` and bare runs* are unchanged, and so is `port-blockers`' expansion.
  - *No gate asserts the union's size.* A tree grep at this stage for the sentinel and
    `registered_members` finds only the two declaring rosters and the expansion itself.
- **Existing integration prose.** §The non-gate arm's derivation and scope paragraphs (deltas 1
  and 2) and §run-gates' widening paragraph (delta 3).

## Existing sections updated

- `gate-sdk/SPEC.md` — §The non-gate arm: the *derived, never maintained* paragraph, the
  *tree's registry rather than the crate's* paragraph, and the sentinel-expansion paragraph
  (*`EVERY_REGISTERED_KNOB` expands to…*) (deltas 1 and 2).
- `gate-sdk/SPEC.md` — §run-gates, *The declared-knob union tracks the widening* (delta 3).
- `native/src/emit/mod.rs` — `registered_members` and its name-list reader, directives and unit
  tests (deltas 1 and 4).
- `native/src/runner.rs` — the `KNOBS` directive (delta 4).
- `gate-sdk/gate-tests/run-arm-contract.test.sh` — the narrowing cases (delta 4).
- `.workflow/release-declarations.md` — Behavior changes (delta 4).
- `docs/gate-sdk/SPEC.md` — generated mirror, regenerated (all deltas).

## Retired spellings

- None — no delta retires a name. `sole_only_name` is a private helper, and generalizing it is a
  build-time naming choice that no surface outside the crate spells.

## Definition of Done

- [ ] **Causal completeness** — the narrowed answer's producer reads argv the front-end already
      composes. Its two consumers are named, and the one non-monotone reader is held by
      construction and by test.
- [ ] **Instruction surfaces: instruction only** — the directives state the member set, not the
      grounds.
- [ ] **Merged with no information lost** — the `--for` refusal and its costed residue land in
      §The non-gate arm.
- [ ] **Amendment deleted** — no `gate-sdk/SPEC-*.md` remains at the iteration.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — none expected. The `--for` residue is owned by
      `config-bridge-resolution-cost`.
