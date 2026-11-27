# SPEC amendment: migration-silent-skips

Pairs `migration-guard-silent-skips`. check-migration-guard passes three readings unseen, and this amendment fixes one and states two. A ref sharing no merge base with the base ref is skipped by the clash check, and nothing says so, so a shallow CI clone judges no clash and prints a clean line that reads like a full one. The gate now names each such ref on its verdict line and, under a strict value a consumer selects, refuses at exit 3. The section's honest limit also gains the two attribute and merge-driver readings it omits.

**What the tree does today**, each bullet with the probe that settled it:

- `ref_additions` continues past a ref whose `git merge-base` exits 1, and the clean line counts only the refs it pushed, `, <k> other ref(s) read` (`grep -n 'fn ref_additions' -A24 native/src/gates/migration_guard.rs`; `grep -n 'other ref(s) read' native/src/gates/migration_guard.rs`). A ref the base ref's commit contains never reaches that loop, since `for-each-ref --no-merged` drops it.
- Both attribute readings take `core.attributesFile` as well as `.git/info/attributes`: in a sandbox with `core.attributesFile` naming a file holding `*.sql ident`, `git check-attr ident` in the working tree and `git check-attr --cached ident` over a throwaway index read from `HEAD` each answer `set` (sandbox probe, git 2.56.1). The section names `.git/info/attributes` alone.
- Under `*.sql merge=union`, a branch and a base each adding `0002_b.sql` with other content merge at exit 0, the file the union of both copies (same sandbox). The section grounds the already-landed skip on that pair stopping at an add/add conflict.
- The knob roster has no unrelated-ref value (`git grep -n 'GATE_SDK_MIGRATION_' -- native/src/knobs/gate_sdk.rs`).

## What changes

### (1) An unrelated ref is named, and refused under the strict value {design-bearing} {user-facing: the clean line and a red run's header name each ref the clash check could not read; a consumer selecting `error` gets exit 3 on a shallow or orphan ref (adopter direction 2026-11-27, lead-relayed; the knob off by default among a selectable set a lead decision of 2026-11-27 under the operator's delegation)}

**Not yet applied** — gate-sdk/SPEC.md §check-migration-guard, the ref-set paragraph's sentence "A ref the base ref's commit already contains is skipped, and so is one sharing no merge base with it, an orphan or a shallow cut." becomes:

> A ref the base ref's commit already contains is skipped, since its numbers are in the base, which **Order** reads. A ref sharing no merge base with the base ref, an orphan or a shallow cut, is **unrelated**: what it adds cannot be read, so the gate names it. Under `GATE_SDK_MIGRATION_UNRELATED_REFS`'s default `off`, the check reads the other refs, the clean line follows the count of refs read with `, <j> unrelated ref(s) skipped: <ref> …`, and a red run's header line carries the same clause before its closing colon. Under `error`, an unrelated ref is input at exit 3, naming each before any finding prints, with the remedy: fetch the history its merge base needs, or narrow `GATE_SDK_MIGRATION_REFS` past it.

and the paragraph's closing sentence "The clean line counts the refs read, `, <k> other ref(s) read`, or names why the check is off, `; clash check off: <reason>`." becomes "The clean line counts the refs read, `, <k> other ref(s) read`, then names any unrelated ones, or names why the check is off, `; clash check off: <reason>`."

**Not yet applied** — the section's grounds gain, after **Why each ref since its merge base with the base ref**:

> - **Why an unrelated ref is named.** A shallow clone cuts every sibling's merge base, so the clash check there reads nothing, and a clean line silent on it claims a ref set it never read. The name tells the reader which refs the check did not judge.
> - **Why `off` is the default.** An orphan ref, a published-site branch say, matches the default glob and shares no merge base by design, so refusing on it by default would red a clone with no cut. A consumer whose CI must judge every sibling selects `error`, the policy-as-choice rule.
> - **Why exit 3.** An unrelated ref is history the clone lacks, input the gate cannot read, and never a finding against the migrations.

**Not yet applied** — gate-sdk/SPEC.md §Layout and configuration, a bullet after `GATE_SDK_MIGRATION_REFS`':

> - `GATE_SDK_MIGRATION_UNRELATED_REFS` (default `off`): what that member does with a ref sharing no merge base with the base ref, `off` naming it on the verdict line, `error` refusing at exit 3; empty takes the default. Any other value refuses at exit 2.

`native/src/gates/migration_guard.rs`: `ref_additions` returns the unrelated refs beside the read ones, and `inner` composes them into the clean line and the header, or raises `Fault::input` under `error`. The knob row joins `native/src/knobs/gate_sdk.rs` with its value check, and its name joins the member's knob list in `native/src/gates/mod.rs`.

### (2) The attribute files and the merge driver, stated {mechanical} {user-facing: documentation only, two readings an adopter can now see before they bite (adopter direction 2026-11-27, lead-relayed)}

**Not yet applied** — gate-sdk/SPEC.md §check-migration-guard:

- the **Attributes unchanged** bullet's "both share `.git/info/attributes`" becomes "both share `.git/info/attributes` and the file `core.attributesFile` names";
- the honest limit's last sentence becomes "Both attribute readings take `.git/info/attributes` and the file `core.attributesFile` names, so a change in either moves them together and the attribute check does not see it.";
- the **Why the already-landed skip** bullet's "a copy with other content stops at the merge as an add/add conflict" becomes "a copy with other content stops at the merge as an add/add conflict under git's own merge driver";
- the honest limit gains, after the sentence above: "A `merge=union` attribute on the migration files merges such a pair cleanly, as the union of both copies, so under it a branch's copy of a file the base already holds lands with statements the destructive check never read."

### (3) The sandbox cases and the declarations {mechanical}

- **Sandbox cases** in `gate-sdk/gate-tests/check-migration-guard.test.sh`, the section's list bullet "a detached `HEAD`, an orphan ref skipped, …" becoming "a detached `HEAD`, an orphan ref skipped and named on the clean line, the same ref at 3 under `error`, a shallow clone whose sibling's merge base is cut, named, `none`, a narrowing glob, and a glob opening with `-` and an unknown `GATE_SDK_MIGRATION_UNRELATED_REFS` value, both at 2", the `error` case alone at 3 and the cases not so marked clean at 0. The orphan case's existing clean-line expectation follows.
- **Unit tests** in `native/src/knobs/gate_sdk.rs` for the value check, beside `GATE_SDK_MIGRATION_REFS`' own.
- `.workflow/release-declarations.md`: *Behavior changes* gains the named unrelated refs on the verdict lines; *Knob changes* gains `GATE_SDK_MIGRATION_UNRELATED_REFS`.

## Producers and consumers

- **The unrelated-ref clause.** Producer: `ref_additions`, from `merge-base` exiting 1. Consumer: the person reading the verdict line; the clean line stays one line under the output contract, so `run-gates` reads it unchanged.
- **The exit-3 refusal.** Producer: `inner`, under `error`. Consumer: the runner, which reads exit 3 as an `error` result, and the person reading the remedy.
- **`GATE_SDK_MIGRATION_UNRELATED_REFS`.** Producer: the knob table and the consumer's knob file; a consumer with a shallow CI selects `error`. Consumers: `inner` and the knob validator.
- **Point 2, roster-holding readers.** §Layout and configuration's knob roster, held by `check-knob-default-coupling` and `check-knob-citation`; the table rows; the member's knob list in `native/src/gates/mod.rs`.

## Existing sections updated

Roster probe: `git grep -n 'GATE_SDK_MIGRATION_REFS\|other ref(s) read\|info/attributes\|add/add' -- ':!docs/llms-full.txt' ':!docs/gate-sdk/SPEC.md' ':!docs/posts'`, over the tracked tree.

- `gate-sdk/SPEC.md` — §check-migration-guard's ref-set paragraph and grounds, and §Layout and configuration's knob bullet (delta 1); the **Attributes unchanged** bullet, the already-landed grounds and the honest limit (delta 2); the sandbox list (delta 3).
- `native/src/gates/migration_guard.rs` — `ref_additions`, the clean line, the header and the refusal (delta 1).
- `native/src/knobs/gate_sdk.rs` — the knob row and its check (delta 1), and its unit test (delta 3).
- `native/src/gates/mod.rs` — the member's knob list (delta 1).
- `gate-sdk/gate-tests/check-migration-guard.test.sh` — the sandbox cases (delta 3).
- `.workflow/release-declarations.md` — the behavior change and the knob (delta 3).
- `docs/gate-sdk/SPEC.md` — generated mirror (all deltas).
- `docs/llms-full.txt` and `docs/requirements.md`'s install-size block — generated, each regenerated by the command its freshness gate prints on red (all deltas).

## Retired spellings

- None — no delta of this amendment retires a spelling; the clean line's `other ref(s) read` clause keeps its text and gains one after it.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the clause, the refusal and the knob.
- [ ] **Instruction surfaces: instruction only** — the knob bullet states the values and the refusal, the grounds staying in the section.
- [ ] **Merged with no information lost** — each passage is re-phrased in place, not appended to.
- [ ] **Amendment deleted** — this file removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` holds.
- [ ] **Run once** — the shallow-clone sandbox under `off` and `error`, each by hand with the built binary.
- [ ] **Release declared** — `.workflow/release-declarations.md` carries the behavior change and the knob.
- [ ] **Entry moved** — `migration-guard-silent-skips` moves to Done before the drain stage, in the commit that lands the merge.
- [ ] **Gaps filed** — any cross-component gap discovered during the work filed as a debt task.
