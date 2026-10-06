# SPEC amendment: mirror-component

Paired with the queue entry `audit-trigger-mirror-component`. `check-stage-entry`'s assertion C counts a generated mirror of a roster file as a component of its own, so an amendment in one component that names its mirror by path as a regenerate target reads as cross-component and is refused build entry without an audit stamp or a waiver. lifecycle-kit/SPEC.md §check-stage-entry states the behaviour and prices it under C's honest limit, with the waiver as its valve. The waiver is the wrong valve for this case: it takes an explicit user ruling each time, for a question a consumer answers once, in config, by saying where its mirrors are. Probes at authoring: this repository holds 14 roster files under `docs/`, each beside a roster dir of the same name at the root (`git ls-files`, filtered to the roster basename); `docs` is not in the gate-sdk prune set (`--emit knob-roster`); no `LIFECYCLE_KIT_MIRROR` name exists in the tracked tree (`git grep`); and the waiver token has been stamped in six commits of the state file's history (`git log -S`).

The entry offered a second outcome, a ruling that the waiver stays the valve. This amendment takes the first: a declaration is consumer-selectable and off by default, so a consumer who prefers the waiver keeps it by leaving the knob empty.

## What changes

### (1) The mirror fold in assertion C {design-bearing} {user-facing: the entry's deliverable, a consumer declaration that resolves a mirror token to the component it mirrors, in the operator's selection of the unit set, direction 2026-10-06}

`audit_signal` in `native/src/gates/stage_entry.rs` folds a body token under the declared mirror root into the roster dir it mirrors before the roster test. After the contract token is stripped, a candidate dir of the form `<root>/<dir>` is replaced by `<dir>` when `LIFECYCLE_KIT_MIRROR_ROOT` is non-empty and `<dir>` is in the roster set; otherwise the candidate is unchanged. The amendment file's own directory and the two-amendment-dirs signal are not folded.

lifecycle-kit/SPEC.md §check-stage-entry, the roster-dir paragraph. *Not yet applied.*

> **A roster dir is a directory holding `LIFECYCLE_KIT_ROSTER_BASENAME`, and the roster test screens the body tokens alone.** An amendment file's own directory is a component wherever it sits. So a second component arrives from a second amendment file anywhere, or from a body token resolving to a roster dir.
>
> **A generated mirror of a roster file is a roster dir until the consumer declares its root.** With `LIFECYCLE_KIT_MIRROR_ROOT` empty, a single-component amendment obliged to name its mirror as an update target, as a consumer's generated-projection rule may require, reaches ≥2 on that token alone. Set, a body token resolving to `<root>/<dir>` counts as `<dir>` where `<dir>` is itself a roster dir. A projection is its source's component, so naming it reaches nothing the source's own token does not. The fold never drops a token: a mirror of another component still adds that component, and a dir under the root that mirrors no roster dir stays a component of its own.

The same section, the two closing sentences of the waiver paragraph. *Not yet applied.*

> It also says that respelling a body token so it stops resolving is not a remedy, the token being the amendment's reach, and that a generated mirror is declared through `LIFECYCLE_KIT_MIRROR_ROOT`. Every other over-demand's valve is the waiver, per C's honest limit below.

The same section, assertion C's honest limit, one sentence after *strictly better than self-report*. *Not yet applied.*

> The mirror fold moves one case to the under-detecting side: an amendment whose reach into the mirror's generator is carried by its own mirror's token alone is silent under the fold.

The refusal's `help:` line, its last sentence. *Not yet applied.*

> Respelling a body token so it stops resolving is not a remedy: the token is the amendment's reach. A generated mirror of a roster file is declared with LIFECYCLE_KIT_MIRROR_ROOT, which counts it as the component it mirrors

### (2) The knob {mechanical} {user-facing: the entry's deliverable, as delta 1}

One row joins lifecycle-kit/SPEC.md §Layout and configuration after `LIFECYCLE_KIT_CONTRACT_TOKENS`, and `native/src/knobs/lifecycle_kit.rs` as a scalar with an empty default. The member's declared-knob roster in `native/src/gates/mod.rs` gains the name. *Not yet applied.*

> - `LIFECYCLE_KIT_MIRROR_ROOT` — the directory a consumer's generated mirrors of roster files sit under, for assertion C's fold (§check-stage-entry); default empty, which folds nothing. A trailing `/` is ignored. A root under which no roster dir sits folds nothing, so a mistyped value keeps the over-demand and hides no component. Independent of canon-kit's `CANON_KIT_MIRROR_ROOT`, where that kit's generator writes, on the ground `LIFECYCLE_KIT_ACTIVE_SECTIONS` states.

No validator arm: no value is malformed, since one that matches no roster dir is inert in the demanding direction.

`.workflow/release-declarations.md` takes the knob under *Knob changes*: new, default empty, nothing to do; a consumer that generates mirrors of its roster files sets it to their root.

### (3) The scenarios {mechanical}

`lifecycle-kit/gate-tests/check-stage-entry.test.sh` gains assertion-C cases over one sandbox: roster dirs `a/` and `b/`, their mirrors `site/a/` and `site/b/`, a roster dir `site/extra/` with no source, and one amendment in `a/`. Each case differs by the amendment's body token and, through `gate_env`, the knob.

- Knob unset, the body naming `site/a/SPEC.md`: exit 1, the signal naming `a` and `site/a`.
- `LIFECYCLE_KIT_MIRROR_ROOT=site`, the same body: clean.
- The knob set as `site/`, the same body: clean.
- The knob set, the body naming `site/b/SPEC.md`: exit 1, the signal naming `a` and `b`.
- The knob set, the body naming `site/extra/SPEC.md`: exit 1, the signal naming `a` and `site/extra`.

The existing case pinning the help line's respelling sentence pins the rewritten one. lifecycle-kit/SPEC.md §check-stage-entry's closing sentence on coverage is unchanged: the runner already holds C.

### (4) This repository's binding {mechanical}

`scripts/lifecycle-config.knobs` binds `LIFECYCLE_KIT_MIRROR_ROOT = docs`, under a `# spec:` line citing lifecycle-kit/SPEC.md §check-stage-entry.

## Producers and consumers

- **The knob's value** (delta 2): produced by a consumer's lifecycle knob file; this repository's binding (delta 4) is the deployed configuration that sets it. At the kit default it is empty and the fold is inert, so a consumer that binds nothing sees today's verdicts.
- **The fold** (delta 1): its one reader is assertion C's component count, at the audit-entry stage's entry. Assertions D and E read C's amendment walk and never its roster set, so they are untouched.
- **Roster-holding readers of the new knob name**, each red when it is missing: the kit's static knob table, which `--emit knob-roster` prints and `check-knob-citation` resolves the SPEC row's citation against; `check-knob-default-coupling`, which couples the row's stated default to the table's; and the member's declared-knob roster, which `check-reads-couples` and `check-gate-substrate-parity` read. The knob names no file and no walk root, so the descriptor's `couples=` and the member's `--reads` roots take no entry.
- **Corpus narrowing** (point 5): the fold narrows the component set C counts. C reds on a count of two or more and on nothing else, so its verdict is monotone in that set and a narrower one can only clear a refusal. No reader reds on finding none, holds an exact count or holds a floor over it.
- **The help line** (delta 1): read by the session refused at the audit-entry stage and by a lead relaying a `--simulate` refusal.

## Existing sections updated

- `lifecycle-kit/SPEC.md` — §check-stage-entry's roster-dir paragraph, waiver paragraph and honest limit (delta 1); §Layout and configuration, one row (delta 2).
- `native/src/gates/stage_entry.rs` — the fold, the knob read and the help line (deltas 1 and 2).
- `native/src/knobs/lifecycle_kit.rs` and `native/src/gates/mod.rs` — the table row and the member's declared-knob roster (delta 2).
- `lifecycle-kit/gate-tests/check-stage-entry.test.sh` — the five cases and the re-pinned help sentence (deltas 1 and 3).
- `scripts/lifecycle-config.knobs` (delta 4).
- `.workflow/release-declarations.md` — *Knob changes* (delta 2).
- `docs/lifecycle-kit/SPEC.md` — the generated mirror, regenerated by the command its freshness gate prints (all deltas).
- `.workflow/surface-ceiling.txt` — lifecycle-kit/SPEC.md's row, re-stamped in the growing commit where the merge takes the file past it (deltas 1 and 2).

Produced by `git grep -n` over the tracked tree for `LIFECYCLE_KIT_CONTRACT_TOKENS`, the sibling knob whose every site a new assertion-C knob meets, and for the help line's retired phrase.

## Retired spellings

- `a generated mirror named in prose, say` — the help line's parenthetical, replaced by the sentence naming the knob (delta 1).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the knob and the fold.
- [ ] **Instruction surfaces: instruction only** — the help line names the remedy and carries no grounds; §check-stage-entry carries them.
- [ ] **Merged with no information lost** — the roster-dir paragraph is rewritten as two, the waiver paragraph's close is rephrased, and the honest limit gains one sentence no rewrite carries.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — the one declared spelling survives nowhere.
- [ ] **Gaps filed** — a cross-component gap found at build is resolved that session.
- [ ] **Built and run** — `bash gate-sdk/bin/build-native.sh`, the crate tests, the full battery, and lifecycle-kit's fixture suite with `gate-tests/check-stage-entry.test.sh`.
- [ ] **Entry moved** — `--queue done audit-trigger-mirror-component` in the build batch that lands this, a stage before the drain stage; no remote oracle gates it.
