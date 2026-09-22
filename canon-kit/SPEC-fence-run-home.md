# SPEC amendment: fence-run-home

site-kit's fixture-suite fence in `site-kit/README.md` §Test stays unmarked for `check-fence-run`. §check-fence-run item 4 pins `HOME` inside the scratch and inherits only `PATH`, and a Ruby user install keeps `kramdown-parser-gfm` under the user gem dir Ruby derives from `HOME`, so the render-fidelity fixtures find no gems in the scratch. The entry asked for one of two rulings: a declared, consumer-named environment passthrough, or a stated refusal naming why a toolchain resolving through `HOME` stays outside fence execution.

**The ruling: the refusal.** Measured 2026-09-22 on the attested host: `GEM_PATH` and `GEM_HOME` are both unset, `gem env gempath` lists the user gem dir first, and both kramdown gems sit only there. Ruby computes that path from `HOME` and reads no variable a passthrough could carry, so a passthrough knob naming `GEM_PATH` reaches nothing on the host that filed the entry. It works only after the contributor exports the variable by hand, and until then marking the fence reds the full battery on that host. The one passthrough that does reach the gems is `HOME` itself, which undoes the pin the fixed environment exists for. The `gates` workflow installs the gem system-wide, so the fixtures' own gate stays witnessed there. What stays unwitnessed is the README's spelling of the command.

## What changes

### (1) §check-fence-run item 4: the `HOME`-resolved toolchain limit {mechanical}

**Not yet applied.** In §check-fence-run, item 4 (**The environment is fixed and not inherited.**), after the sentence ending "and naming it is the consumer's act.", add:

> **A second honest limit:** a toolchain that finds its own packages through `HOME`, such as a Ruby user gem install, finds none in the scratch, so a fence that reaches one cannot be marked on a host that installed that way. No passthrough is offered. Such a toolchain derives the path from `HOME` rather than reading a variable a passthrough could carry, and passing `HOME` through would undo the pin that keeps a fence off the user's own files. A fence of that kind stays unmarked: its spelling goes unwitnessed, while a gate it exercises still runs wherever the battery runs with the toolchain installed.

## Producers and consumers

- **The stated limit** (delta 1). A sentence and no new state: the producer is the SPEC, and the consumers are an author deciding whether to mark a fence and a reader asking why `site-kit/README.md` §Test carries no marker. No new name, knob or field.
- **Point 5.** Not reached; no corpus narrows.
- **Point 6.** Not reached; no member obligation is added.

## Existing sections updated

Roster from `git grep -n -e "fixed and not inherited" -e "fence-runnable" -- canon-kit/SPEC.md site-kit/README.md`, run 2026-09-22.

- `canon-kit/SPEC.md` §check-fence-run, item 4 (delta 1).
- `docs/canon-kit/SPEC.md`, the generated mirror (delta 1).

## Retired spellings

- None — no delta retires a name.

## Definition of Done

- [ ] **Causal completeness.** Every point of §The causal-completeness check holds; the delta introduces no state.
- [ ] **Instruction surfaces: instruction only.** No template or shim changes.
- [ ] **Merged with no information lost.** Item 4 reads as one item carrying two honest limits.
- [ ] **Amendment deleted.** This file is removed on merge (`ls canon-kit/SPEC-*.md`).
- [ ] **Entry moved.** `fence-run-fixed-env-hides-user-gem-dir` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached; the block above declares none.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
