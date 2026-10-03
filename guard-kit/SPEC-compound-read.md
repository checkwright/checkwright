# SPEC amendment: compound-read

**Rule `ro_pipeline` grants a compound of read-only statements, joined by `;`, `&&` or `||`, as it already grants one joined by newlines.** Today the rule refuses a call whose `sq dq hd` skeleton carries `&&`, `||`, `;` or `&` (`grep_q(r"(&&|\|\||;|&)", …)` in `ro_pipeline`, `native/src/guard/rules/grants.rs`). A newline is not in that set, so `grep foo a`, newline, `grep bar b` is granted today while `grep foo a; grep bar b` falls through (both run at authoring, through `--hook shell-guard`). The separator decides nothing the rule's safety argument rests on. That argument is per segment: each segment leads with a roster read in none of its write forms. Whether the shell runs the next statement after a `;`, an `&&` or an `||` changes which reads run, never what any of them can write.

**The redirect half rides write-side-steering.** A read redirected to a bounded target is a write. The ruleset grants it where it grants every other bounded write, in that unit's `bounded_write` rule, and not here, so rule `ro_pipeline` stays the rule for calls that write nothing. The two rules then never grant the same call (delta 2).

**Measured at authoring.** The entry's *6 of 30* count was taken over the friction log at an earlier iteration's align stage. That log was drained at a close since, so the count cannot be re-run on its corpus. It is replaced by a count over the 211 lines `.workflow/prompt-friction.log` held at this stage's entry, every one a fall-through. A scratch script fed each line through `--hook shell-guard` twice: as logged, and with every top-level `;`, `&&` and `||` respelled as a newline, which is the grant this amendment adds. **26 of the 211 flip from `fallthrough` to `allow`.** Most are a batch of `grep … | head` reads joined by `;`, some with an `echo ---` banner between them. Dropping redirects to gitignored targets as well adds one more line, the redirect half's measured share. Of the 184 left, the leads are mostly `git`, the front end, the gate binary and `gh`, outside the read roster.

## What changes

### (1) Rule `ro_pipeline` reads a compound statement by statement {design-bearing} {user-facing: compound-read-classifier-reach's deliverable, rule `ro_pipeline`'s reach over a compound of read-only statements}

In `ro_pipeline`, the chain refusal narrows to the backgrounding `&`, read by the ruleset's statement-ending-`&` predicate (`shell_backgrounds`). `;`, `&&` and `||` no longer refuse. Every other clause is unchanged and runs over the whole call: the substitution decline, the quote decline, the inert-redirect clause, the declared-forms reader, the banner tolerance, at least one read, and rule `worktree_confinement`'s test taken as a predicate. A segment of the compound split passes the segment test or the call falls through, as it does for a newline compound today.

**The widened lead applies per statement.** A segment that opens a statement may be the bare form of a committed allow entry, as the first segment of the call may be today, wherever the call has more than one segment. So `git status; grep -c x a.md` is granted, as `git status | head` is. The rule reads the statement split, then each statement's pipe split, rather than the compound split alone, since the compound split does not record which separator produced a segment (§The reader and its views).

**Readers of the rule's test.** Rules `bounded_wait` (clause (d)), `background_no_record` (exemption (3)), `find_exec` and `worktree_confinement` (its admitted read) apply the segment test and the declared-forms reader, not the chain refusal. The probe was `grep -n "is_ro_segment\|ro_forms_clear" native/src/guard/rules/*.rs` and a read of each hit. So their verdicts do not move with this delta.

### (2) The redirect half is the `bounded_write` rule's {mechanical}

**Where write-side-steering lands in the same build batch as this unit or an earlier one**, `ro_pipeline`'s redirect clause stays as it is: `/dev/null` and an fd-dup only. A read with any other redirect target falls through here, to the `bounded_write` rule, whose read kind and bounded-redirect clause grant it (write-side-steering's amendment). This delta then adds the decision-table rows under delta 3 that pin that hand-off.

**Where write-side-steering does not land**, nothing here grants the redirect half. The rows are left out, and the half is filed with the write-side entry's disposition as a costed gap, never landed on a bounds test this unit would have to author.

### (3) Decision-table cases {mechanical}

`guard-kit/guard-tests/cases.tsv`, rule `ro_pipeline`'s section:

- `allow	grep -c x tracked.md; wc -l tracked.md`, `allow	grep -q x tracked.md && head -1 tracked.md`, `allow	grep -q x tracked.md || echo none`: each separator.
- `allow	echo ---; grep -c x tracked.md`: a banner statement beside a read.
- `allow	git status; grep -c x tracked.md`: the widened lead opening a statement other than the call's first segment's. Rule `allowlist_chain` would block it today, so it pins the order too.
- `fallthrough	grep -c x tracked.md; md5sum tracked.md`: a statement off the roster.
- `fallthrough	grep -c x tracked.md; sort -o out.txt tracked.md`: a write form in a later statement.
- `fallthrough	grep -c x tracked.md; touch tracked.md`: a write outside every bound.
- `fallthrough	grep -c x tracked.md & wc -l tracked.md`: the backgrounding `&` still refuses.
- The comment over the escaped-`;` rows changes, and `fallthrough	grep foo a\;b` becomes `allow	grep foo a\;b`. The chain test no longer reads the `;`, and the split keeps it in its word. `fallthrough	grep foo a\\;b` keeps its verdict, since its second statement `b` is off the roster.

Under delta 2's first case, the `bounded_write` rule's section gains:

- `allow	grep -c x tracked.md > .tmp/n.txt; wc -l .tmp/n.txt`: a read redirected to a bounded target, compounded with a read.
- `fallthrough	grep -c x tracked.md > out.txt`: a target outside every scratch dir.

**Inferred, cannot run before build:** each row's verdict above — the rule change is delta 1's and the hand-off rule is write-side-steering's, both landed by build; a pre-existing row this delta flips is re-derived under §Testing's non-monotone rule, and every flip takes a cause.

### (4) Rule `ro_pipeline`'s item states the compound reach {mechanical}

guard-kit/SPEC.md §The rule roster, rule `ro_pipeline`. **Not yet applied.** The item's second sentence becomes:

> Conservative by construction: command or process substitution, a leftover quote after normalization, a backgrounding `&`, or a redirect to anything but `/dev/null` or an fd-dup refuses and falls through. A compound is read statement by statement, whatever joins it, `;`, `&&`, `||` or a newline, since the safety argument is per segment and the separator decides only which reads run. A read redirected to a file is a write, and the `bounded_write` rule grants it where its target is bounded.

In the widened-lead carve-out, *The lead segment may instead be the bare form of a committed allow entry* becomes *A segment opening a statement may instead be the bare form of a committed allow entry*.

### (5) The site mirror follows {mechanical}

`docs/guard-kit/SPEC.md` is regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commit landing delta 4.

## Producers and consumers

- **Rule `ro_pipeline`'s verdict on a compound.** Producer: the rule's table row, on every bash call the shell guard reads, wherever the member is wired. This repo wires it in `.claude/settings.json`. Consumer: the harness, through the hook's `permissionDecision: allow` envelope. The friction log loses each granted compound, which the close-stage triage reads through `--emit scan-prompts`.
- **No new state, knob, name, event or interface.** The rule's declared views are unchanged, so `check-guard-registration`'s assertion C holds. The widened lead reads `GUARD_KIT_SETTINGS` on the fail-open read it already takes.

## Existing sections updated

Roster by `grep -n "ro_pipeline" guard-kit/SPEC.md guard-kit/guard-tests/cases.tsv` and `grep -n "fn ro_pipeline" -A35 native/src/guard/rules/grants.rs`, over the tracked tree.

- `native/src/guard/rules/grants.rs`: `ro_pipeline` (delta 1).
- `guard-kit/guard-tests/cases.tsv`: rule `ro_pipeline`'s section and the `bounded_write` rule's (delta 3).
- `guard-kit/SPEC.md`: §The rule roster, rule `ro_pipeline` (delta 4).
- `docs/guard-kit/SPEC.md`: the regenerated mirror (delta 5).

## Retired spellings

- None — the change narrows a refusal; it retires no name.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
