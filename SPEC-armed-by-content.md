# SPEC amendment: armed-by-content

`# armed-by: <KNOB>` declares that a gate asserts nothing while a knob resolves empty, and `doctor` names such a member disarmed (gate-sdk/SPEC.md §The install disposition). Two disarmed states fall outside that shape.

- `check-commit-msg` and `check-tree-terms` read the pattern files `GATE_SDK_MSG_PATTERN_FILES` and `GATE_SDK_MSG_PATTERN_FILES_LOCAL` name, and both pass with nothing checked when those files hold no pattern line. The first knob can never resolve empty — its row takes the default on an empty value, `<gates-dir>/msg-patterns.list` — so a bare declaration naming it would be a line `doctor` can never fire, which §check-install-disposition assertion D exists to refuse.
- `check-provenance-seam` asserts nothing at its defaults, `CANON_KIT_SCAN_KIT_ROOTS` at `0` with `CANON_KIT_SEAM_SURFACE_GLOBS` empty. That is a conjunction over a `0`/`1` switch and a list, and no single knob's emptiness states it.

**The ruling: the declaration gains a content form, and the conjunction is refused with its ground.**

**The content form.** `# armed-by: content <KNOB> [<KNOB>…]` declares that the gate asserts nothing while no file those knobs name carries a **live line** — a line that, after leading blanks, is non-empty and does not open with `#`. That is the grammar every pattern roster here already takes, and `is_pattern` in `native/src/gates/commit_msg.rs` is its one implementation, already imported by `check-tree-terms` and `check-portability-floor`. A named file that is absent contributes nothing, so the gitignored `_LOCAL` file behaves as the gates treat it. A required file that is absent is the battery's exit 2, reported at its own run, and `doctor` stays out of it: it is not a second runner. The leading token keeps the bare form unambiguous, since no knob is named `content`. The member still carries at most one `# armed-by:` line.

**The refusal.** The declaration names emptiness, never a predicate over a value, so `check-provenance-seam` takes none, and the ground is stronger than grammar. Its default is the state canon-kit intends for an adopter: at `0` the kit SPECs are a dependency's, not the adopter's content (canon-kit/SPEC.md §check-provenance-seam). A `doctor` line would ask every adopter to arm a gate the kit tells them to leave off. The gate's clean line already names both halves when both are off, which is the bound. A predicate grammar (`KNOB=1`, `A|B`) is also refused for the general case: it is an expression language in a header comment, with a reader that must agree with each gate's own resolution, bought for one member whose disarmed default is deliberate.

**Measured at authoring (2026-09-23).** `checkwright-gates check-commit-msg README.md /dev/null` prints `COMMIT-MSG: clean (0 banned pattern(s) configured; message unchecked)`, and `check-tree-terms . /dev/null` prints `TREE-TERMS: clean (0 banned pattern(s) configured; tree unchecked)`, each at exit 0. `git grep -n "armed-by:" -- '*/checks/*.gate'` finds ten declarations, every one in the bare form; the two fixture declarations (under `*/gate-tests/`) sit in `check-install-disposition`'s pair. `native/src/installer/doctor.rs`' `disarmed_block` fires only on a member carrying exactly one line whose knob resolves empty. `native/src/gates/install_disposition.rs`' `armed_by_finding` is assertion D.

## What changes

### (1) The shared live-line filter {mechanical}

**Not yet applied.** `is_pattern` moves out of `native/src/gates/commit_msg.rs` to one crate-wide home, named for what it tests (a live line), and its three gate callers and `doctor` call it there. It does not change.

### (2) The parser, the verifier and `doctor` read the content form {design-bearing}

**Not yet applied.**

- `registry.rs`' `armed_by` keeps returning each line's value. A small helper splits a value into its form — bare knob, or `content` and a knob list.
- `install_disposition.rs`' `armed_by_finding` (assertion D) accepts the content form. It holds every knob in the list to the placement rule a bare knob meets (a declared static knob of the declaring kit's own table), and reds `content` with no knob after it. At most one line per member, as before.
- `doctor.rs`' `disarmed_block` reads the content form. It resolves each knob through the same tree-anchored resolution the bare form uses, splits each value on whitespace, reads each named file relative to the repository root, and renders the member disarmed when no present file carries a live line. An unreadable present file renders nothing, as a knob that refuses to resolve does. The rendered line is `disarmed     <member> asserts nothing until a file <KNOB> or <KNOB> names carries a live line`, the names joined as read off the declaration.

### (3) The two pattern-file gates declare it {mechanical}

**Not yet applied.** `gate-sdk/checks/check-commit-msg.gate` and `gate-sdk/checks/check-tree-terms.gate` each gain `# armed-by: content GATE_SDK_MSG_PATTERN_FILES GATE_SDK_MSG_PATTERN_FILES_LOCAL`.

`check-portability-floor` keeps its bare `# armed-by: GATE_SDK_PORTABILITY_PATHS`. Its emptied-roster residual stays bounded by the detail line alone, because it would need a second declaration line. This amendment does not change the one-line rule.

### (4) The fixture pairs and unit tests hold the form {design-bearing}

**Not yet applied.**

- `check-install-disposition`'s `good/` case gains a member declaring the content form over a knob its kit owns.
- Its `bad/` case gains a `content` declaration with no knob, and one naming a foreign knob.
- `doctor.rs`' unit tests gain three trees:
  - one whose named files carry only comments and blanks, which is disarmed;
  - one where the `_LOCAL` file alone carries a live line, which is armed;
  - one whose required file is absent, which renders nothing.

### (5) The owning sections state the form and the refusal {mechanical}

**Not yet applied.**

**gate-sdk/SPEC.md §The install disposition.** In **The arming declaration** paragraph, after "…a misspelled or foreign knob is a red, never a silent no-op.", add:

> **A content form covers a gate disarmed by what a file holds rather than by what a knob says.** `# armed-by: content <KNOB> [<KNOB>…]` declares that the gate asserts nothing while no file those knobs name carries a live line — non-empty after leading blanks and not opening with `#`, the one filter the pattern-roster gates share. An absent file contributes nothing, and `doctor` reads each present file from the repository root. The bare form cannot say this for `GATE_SDK_MSG_PATTERN_FILES`, whose row never resolves empty. **The declaration names emptiness and never a predicate over a value.** `check-provenance-seam`'s disarmed default is a conjunction over a switch and a list. It is also the default the kit intends for an adopter (canon-kit/SPEC.md §check-provenance-seam), so it takes no declaration, and its clean line naming both halves is the bound.

The honest-limit sentence stays as it is.

**gate-sdk/SPEC.md §check-install-disposition, assertion D.** "its value is a declared static knob that `knobs::owner` places in the kit whose root the gate sits under" becomes "its value is a declared static knob that `knobs::owner` places in the kit whose root the gate sits under, or `content` followed by one or more such knobs". "A second line, an empty value, a name no static kit declares, or another kit's knob is a finding" becomes "A second line, an empty value, a `content` with no knob, a name no static kit declares, or another kit's knob is a finding".

**gate-sdk/SPEC.md §check-tree-terms.** "When the pattern set is empty the tree is unchecked (clean) — the fail-closed obligation is on a missing file, not an empty one." gains: "The member declares `# armed-by: content` over the two pattern knobs, so `doctor` names that state (§The install disposition); `check-commit-msg` declares the same."

**installer/SPEC.md §doctor.** After the rendered example line, the sentence "with the member and knob read off the declaration, never spelled by `doctor`." becomes "with the member and knob read off the declaration, never spelled by `doctor`. A member declaring the content form renders `disarmed     check-tree-terms asserts nothing until a file GATE_SDK_MSG_PATTERN_FILES or GATE_SDK_MSG_PATTERN_FILES_LOCAL names carries a live line`, read the same way, when no present file those knobs name carries one."

**canon-kit/SPEC.md §check-provenance-seam, Corpus paragraph.** After "With both sets empty the gate passes, and its clean line names which set was off.", add: "It declares no `# armed-by:`, because that default is the intended one (gate-sdk/SPEC.md §The install disposition)."

## Producers and consumers

- **The content form** (deltas 2 and 3). Producers: the two descriptors. Consumers: `doctor`'s disarmed block, at an install's `doctor` run; §check-install-disposition assertion D, at every commit touching a descriptor; and `check-comment-tier`, whose directive roster already holds the `armed-by:` prefix, so the new form needs no roster row.
- **The rendered line** (delta 2). Its reader is the adopter at `doctor`, and it leaves the exit status unchanged, as the bare form's line does.
- **Point 5.** Not reached. No corpus narrows.
- **Point 6.** The obliged members are the two declaring gates, each satisfied by delta 3's line.

## Existing sections updated

Roster from `git grep -n "armed-by" -- '*.md' ':!docs'` and `git grep -n "is_pattern" native/src`, run 2026-09-23.

- `gate-sdk/SPEC.md` §The install disposition, §check-install-disposition and §check-tree-terms (delta 5).
- `installer/SPEC.md` §doctor (delta 5).
- `canon-kit/SPEC.md` §check-provenance-seam (delta 5).
- `native/src/gates/commit_msg.rs`, `native/src/gates/tree_terms.rs` and `native/src/gates/portability_floor.rs`, for the filter's move (delta 1).
- `native/src/registry.rs`, `native/src/gates/install_disposition.rs` and `native/src/installer/doctor.rs` (delta 2).
- `gate-sdk/checks/check-commit-msg.gate` and `gate-sdk/checks/check-tree-terms.gate` (delta 3).
- `gate-sdk/gate-tests/check-install-disposition/` (delta 4).
- `.workflow/release-declarations.md`, two bullets:
  - Behavior changes: `checkwright doctor` names `check-commit-msg` and `check-tree-terms` disarmed when the pattern files carry no pattern line (delta 2).
  - Tightened gates: `check-install-disposition` assertion D accepts and verifies the content form (delta 2).
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`, `docs/installer/SPEC.md` and `docs/canon-kit/SPEC.md`.

## Retired spellings

- None — no delta retires a spelling. The bare form keeps its meaning, and `is_pattern` moves under a new name only if build renames it, which the filter's one home makes a single edit.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the content form.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** Each SPEC addition re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `armed-by-content-emptiness-shape` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Nothing retired. If build renames `is_pattern`, `git grep -n is_pattern` is empty after the merge.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
