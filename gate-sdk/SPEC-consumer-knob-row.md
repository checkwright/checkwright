# SPEC amendment: consumer-knob-row

Every knob resolves against a static kit table. So a consumer-declared gate that
needs a knob **no kit's prefix owns** is refused on every invocation, with no table
able to answer. §The declaration cohort names the question for the first
consumer-owned knob name and leaves it open. **This amendment answers where that row
is declared: on the descriptor of the gate that reads it.** It builds no resolution
code, because no member needs one yet. It also repairs the `.workflow/` asymmetry the
cohort section preserved, which needs no consumer-owned knob at all.

**The measurement (2026-09-21).**

- No consumer-declared member reads a consumer-owned knob. Seventeen descriptors
  live under `scripts/` (`git ls-files 'scripts/*.gate'`). The `.gate` grammar
  carries no declaration line, and a member's knob reads are the crate's declared
  set. The first knob-declaring member of the consumer tranche is still to come.
- **The asymmetry is still there, and it is a kit knob.** `release_bump.rs:10`,
  `release_declaration_parity.rs:14` and `release_change_declared.rs:11` hardcode a
  `.workflow/` path, while `native/src/emit/upgrade_smoke.rs` resolves the same file
  through `GATE_SDK_WORKFLOW_DIR`. That is a gate-sdk row, and a knob resolves by
  its name's prefix, never by the reading gate's location. So all three can declare
  it today, as §The declaration cohort's own correction paragraph states.

**Why the descriptor and not a consumer knob table.** A consumer knob table would be
a new file convention with no kit to own it. It would also carry a second
resolution tier with its own precedence against the kit tables. Its unit would be
the consumer's whole tree rather than the one gate that reads the name. The
descriptor already travels with a consumer gate and is read at dispatch. It is also
already the surface a knob name is admitted against, since `couples=`' `knob:`
admissibility reads the member's declaration. A row declared there has exactly one
owner, the gate, which keeps the static tables' one-owner-per-knob rule at a finer
grain rather than adding a second owner class. **Refused: borrowing a kit prefix.**
The name would resolve to that kit's table and be refused as undeclared, and a kit
row would be a consumer value inside a kit, which the provenance seam refuses.

## What changes

### (1) §The declaration cohort answers the knob-ownership question {mechanical}

**Not yet applied.** Replace the two paragraphs opening **The three declare no knobs,
and the `.workflow/` asymmetry** and **The knob-ownership question this section
named is re-pointed** with one paragraph stating the ruling.

- A knob no static kit's prefix owns is declared on the `.gate` descriptor of the
  consumer gate that reads it. The row is the name, its shape and its default.
- The name carries no static kit's prefix. A prefixed name is that kit's, and a
  consumer's value there is a kit literal.
- It resolves from the environment and then the descriptor default. A knob-file
  layer is not ruled here: the first member that needs file-setting owns that
  addition, in its own unit, together with the descriptor line's grammar.
- Until that member lands, the `knobs::wire` refusal of an unowned name stays, as
  the fail-closed state. It is loud and correct, because no gate in the tree reads
  such a name.
- The paragraph keeps the cohort's own finding: the refusal fires on a name no kit
  owns, and also on a name a kit's prefix spells and its table does not declare.

### (2) The three release gates read the workflow directory through its knob {mechanical}

**Not yet applied.** `release_bump.rs`, `release_declaration_parity.rs` and
`release_change_declared.rs` each declare `GATE_SDK_WORKFLOW_DIR` and derive their
default path under it, in place of the hardcoded `.workflow/` constant. A positional
argument still overrides. Under this repo's configuration the resolved path is
unchanged, which is the oracle: each member's fixture pair and its live-tree run are
unchanged.

## Producers and consumers

- **The ruling (delta 1).** No state or name is minted. The reader is the author of
  the first consumer gate that needs its own knob. The ruling tells that author where
  the row goes and what its unit owes.
- **The knob reads (delta 2).** Producer: gate-sdk's `GATE_SDK_WORKFLOW_DIR` row,
  set in this repo's `scripts/gate-sdk-config.knobs` or defaulted. Consumers: the
  three members. Each member's derived knob file joins its effective `couples=`
  automatically (§The `# graph:` manifest). So each descriptor gains
  `scripts/gate-sdk-config.knobs` as a derived couple with no manual edit, and
  `check-graph`'s couples-to-hook parity regenerates the hook. `check-reads-couples`
  is unaffected, since no walk root changes.
- **Point 5.** Nothing narrows.

## Existing sections updated

Rosters from `grep -n "DEFAULT_DISPOSITION\|DEFAULT_DECL" native/src/gates/*.rs`,
`git ls-files 'scripts/*.gate'`, and reading §The declaration cohort.

- gate-sdk/SPEC.md §The declaration cohort (delta 1).
- `native/src/gates/release_bump.rs`, `native/src/gates/release_declaration_parity.rs`
  and `native/src/gates/release_change_declared.rs` (delta 2).
<!-- update-target-exempt: generated projections, regenerated by their freshness gates' printed commands -->
- `docs/gate-sdk/SPEC.md`, `scripts/git-hooks/pre-commit`.

## Retired spellings

- None — no name is removed; the three constants are private to their modules.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** The cohort's two replaced paragraphs lose
      no fact: the prefix-without-row door and the re-pointing both survive.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `bridged-knob-owner-for-consumer-gate` moves to Done in the
      merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
