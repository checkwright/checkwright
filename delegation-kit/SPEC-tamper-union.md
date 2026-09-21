# SPEC amendment: tamper-union

`check-gate-tamper`'s assertion A covers only paths that a `DELEGATION_KIT_GATE_FILES`
glob matches. Two holes follow from that:

- **The library.** The knob's default names `<gates-dir>/lib/gate.sh`, and no
  vendored tree carries that path. The library sits under the gate-sdk root, so a
  consumer on the default leaves it uncovered (queue:
  `gate-tamper-default-library-path-unvendored`).
- **Coverage.** Nothing holds that every registered gate's file is covered. A
  consumer declaration replaces the default whole. This repo's own declaration
  once left all nine consumer-resident gates outside it, and under the default
  every vendored kit's gate lies outside it (queue: `gate-file-coverage-closure`).

**Operator direction (2026-09-21, lead-relayed), option (B): no new gate.** The
reader unions every registered member's resolved declaration path and the
gate-sdk library into its gate-file set. That is the same way it already unions
every kit root into the meta paths. Coverage becomes complete by construction,
and both units pair this one amendment.

## What changes

### (1) The tamper reader unions the registry and the library into its gate files

`check-gate-tamper` extends the resolved `DELEGATION_KIT_GATE_FILES` value before
either assertion reads it, and in both modes (live and `--fixture`).
**{design-bearing}** Two sets of literal paths are added, and each is spelled
against the git toplevel, because the staged names it is matched against are
spelled that way (the rule the meta-path union already follows,
gate-sdk/SPEC.md §Layout and configuration):

- **Every registered member's declaration.** Each non-comment line of
  `<GATE_SDK_GATES_DIR>/gates.list` resolves through the battery's own
  resolution, the one `registry::resolve` holds: the gates dir first, then each
  kit root's `checks/`, with `.sh` before `.gate` inside a dir. The dirs are
  spelled with `walk::kit_roots_under(toplevel)`. A member that resolves nowhere
  adds nothing, because resolving it is another gate's job. An absent `gates.list`
  adds nothing. A `gates.list` that exists and cannot be read is exit 2, the
  fail-closed contract.
- **The gate library.** `<gate-sdk root>/lib/gate.sh`, where the root is the
  `GATE_SDK_ROOT` locator (`walk::sdk_root()`) spelled against the toplevel. It is
  dropped when the root lies outside the toplevel, as `kit_roots_under` drops a kit
  vendored beside the repository: such a tree tracks no file here, so no staged
  path can name it.

A union element is a literal path, and `walk::pattern_match` over a literal is
equality, so the matcher is unchanged. The union is additive and never a filter:
a glob the consumer declared cannot be lost. A glob still matters for a gate file
that is not yet registered, such as a declaration staged in the same commit that
adds it to `gates.list`.

The clean line gains the union's size, so a reader can see that coverage was
derived: `GATE-TAMPER: clean (<n> staged path(s); <m> registered gate file(s)
covered; gate edits meta-isolated, no self-serving path-exemption)`. The
assertion A finding line keeps its glob list and appends "plus every registered
gate and the gate library".

**Replacement text for delegation-kit/SPEC.md §Verify after every agent commit,
assertion A** — **Not yet applied**:

> - **A — gate-edit isolation.** A commit that touches a gate file may touch only
>   meta-layer paths (`DELEGATION_KIT_META_PATHS` prefixes + root `*.md`);
>   co-staging product code with a gate edit is blocked. Split the gate change into
>   its own commit. A gate file is a path a `DELEGATION_KIT_GATE_FILES` glob
>   matches, **or** a registered member's resolved declaration, **or** the
>   gate-sdk library. The reader unions the last two in, each spelled against the
>   toplevel, so no consumer value can leave a registered gate or the library
>   uncovered.

### (2) The knob default drops its library element

The unvendored third element is removed from the default in both places.
**{mechanical}** In `native/src/knobs/delegation_kit.rs` `gate_files`, and in
delegation-kit/SPEC.md §Layout and configuration, the default becomes
`("${GATE_SDK_GATES_DIR}/check-*.sh" "${GATE_SDK_GATES_DIR}/check-*.gate")`. The
row keeps its single input, `GATE_SDK_GATES_DIR`. The library is the reader's to
cover (delta 1), so no default element could be right or wrong about it.

**Replacement text for the `DELEGATION_KIT_GATE_FILES` bullet** — **Not yet
applied** — for everything after the default:

> **Both declaration spellings are on the default**, because a gate's
> declaration path is `<name>.sh` *or* `<name>.gate` (gate-sdk/SPEC.md §The
> `# graph:` manifest). The knob names the gate files **beyond** what the reader
> derives: `check-gate-tamper` unions every registered member's resolved
> declaration and the gate-sdk library into the resolved value
> (§Verify after every agent commit), for the reason it unions kit roots into
> `DELEGATION_KIT_META_PATHS`: a file value replaces the default whole, and a
> registered gate must stay covered under a consumer's own value. A glob still
> covers a gate file not yet registered. The two knobs now behave alike: both are
> replaced whole by a file value, and both are widened by the reader.

The "Criterion 4 binds on this gate as a property of the *consumer's* configured
globs" paragraph is re-phrased to the union. Every tree's gate files now include
the registered kit declarations, `check-gate-tamper.gate` among them, so the
gate reads its own declaration whenever that file is staged, on every tree and
not only on one that widens the globs. The paragraph's criterion finding moves
from "clears under the default" to "binds on every tree", and it cites
gate-sdk/SPEC.md §The port-candidate criteria as before.

This repo's `scripts/delegation-config.knobs` keeps its five elements. They stay
correct, and the union makes the `*/checks/*` and `gate-sdk/lib/gate.sh` lines
redundant for registered members only. That is consumer config, and it does not
change here.

### (3) The live test and a crate test hold the union

**{mechanical}**

- `delegation-kit/gate-tests/check-gate-tamper.test.sh` gains a case,
  `a-registered-gate-outside-every-glob`. The throwaway repo registers
  `check-sample` in `scripts/gates.list` and sets
  `DELEGATION_KIT_GATE_FILES[] = other/check-*.sh` in
  `scripts/delegation-config.knobs`. It stages an edit to
  `scripts/check-sample.sh` beside `product/x.txt`, and the case wants exit 1 with
  `gate edit not isolated`. Before the change this shape was clean. A second case
  stages the same edit alone and wants exit 0 with `registered gate file(s)
  covered`.
- A unit test in `native/src/gates/gate_tamper.rs` builds the union over a
  scratch tree and asserts three things: a vendored root's `lib/gate.sh` is in it;
  a root outside the toplevel contributes nothing; and an absent `gates.list`
  yields the library alone. The library arm is held here because the fixture
  runner exports an absolute `GATE_SDK_ROOT` to the real repository, which lies
  outside a throwaway repo, so a `.test.sh` case cannot reach that arm.
- The good/bad pair is unchanged. Its case dirs carry no `gates.list`, so the
  union adds only the library, which lies outside the case's toplevel and is
  dropped. The pair's verdicts stand.

## Producers and consumers

- **The union (delta 1).** *Producer:* `check-gate-tamper`'s rule, on every
  pre-commit run in every tree that registers it: this repo's `gates.list`, and
  the consumer registration delegation-kit/SPEC.md §Layout and configuration
  prescribes. *Consumer:* assertion A's gate-file test; assertion B's live
  collector (`collect_live`), which reads exemption arrays only from staged gate
  files and so now reads them from registered kit declarations as well; and the
  clean line.
- **Narrowing and widening check (point 5).** The refusal set widens: a commit
  that stages a registered declaration or the library with product code now reds
  where it passed. The reader red condition that changes is assertion A's, and it
  is monotone in the gate-file set. For assertion B, a newly added exemption in a
  kit `.gate` descriptor is now judged. The descriptor grammar holds no
  `# exception-list:` array, which `check-gate-exemption-tasks` counts at zero, so
  B's verdict set is unchanged today.
- **This repo under the change (point 6).** Probe: a resolution script over
  `scripts/gates.list` (consumer-first, `.sh` before `.gate`) resolved all 123
  members, 17 under `scripts/` and 106 under kit `checks/`, none unresolved. Every
  one is already matched by this repo's five globs, so the union adds no gate file
  here and the battery's verdict is unchanged.
- **Descriptor couples.** The gate now reads `<gates-dir>/gates.list` and the
  gate-sdk library path. `check-reads-couples` is the oracle for whether
  `checks/check-gate-tamper.gate`'s `couples=` must name them. Build runs it and
  adds whatever it reports (`scripts/gates.list` is the spelling the registry
  readers' descriptors use).
- **Knob roster readers (delta 2).** `--emit knob-roster` renders the default
  from the table. `check-knob-default-coupling` and `check-knob-citation` read the
  SPEC's knob bullet against the table, which is why both are edited together.
  Probe:
  `grep -rn '}/lib/gate.sh' --include=*.md --include=*.rs --include=*.knobs .`
  matched `native/src/knobs/delegation_kit.rs:29`, delegation-kit/SPEC.md:3325,
  its `docs/` mirror, and one substring false-positive: `native/src/emit/git_hooks.rs:242`'s
  `format!("{}/lib/gate.sh", walk::sdk_root()...)`, `gen-pre-commit`'s own splice of the
  matcher body from the gate-sdk root's library — a different mechanism reading the
  library at its actual root rather than at the knob default's unvendored
  `<gates-dir>/lib/gate.sh`, so delta 2 leaves it untouched.

## Existing sections updated

- delegation-kit/SPEC.md §Verify after every agent commit — assertion A (delta 1).
- `native/src/gates/gate_tamper.rs` — the union, clean line and finding text (delta 1), unit test (delta 3).
- `delegation-kit/SPEC.md` — §Layout and configuration's `DELEGATION_KIT_GATE_FILES` bullet and the criterion-4 paragraph (delta 2).
- `native/src/knobs/delegation_kit.rs` — the default's third element (delta 2).
- `delegation-kit/gate-tests/check-gate-tamper.test.sh` — the two cases (delta 3).
- `delegation-kit/checks/check-gate-tamper.gate` — `couples=` per `check-reads-couples` (delta 1).
- `.workflow/release-declarations.md` — a `## Tightened gates` bullet: ``- `check-gate-tamper` — every registered gate's declaration and the gate-sdk library are gate files whatever `DELEGATION_KIT_GATE_FILES` says, so a commit staging a vendored kit's `.gate`, or `lib/gate.sh`, beside product code now reds; split it. The default's unvendored `<gates-dir>/lib/gate.sh` element is dropped. The clean line's wording changed, so re-pin a log-matching tool.`` (deltas 1 and 2).
- `docs/delegation-kit/SPEC.md` — the generated mirror, regenerated by its arm (all deltas).

## Retired spellings

- `${GATE_SDK_GATES_DIR}/lib/gate.sh` — the default's unvendored library element (delta 2).

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
      component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in
      `## Retired spellings` above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks.
