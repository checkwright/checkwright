# SPEC amendment: identifier-reach

De-literalization bans prose that restates a source's **internal** identifier
roster and allows naming the public contract, but it never said which
identifiers are internal. So every `internal-identifier-restatement` audit
re-derived the answer, and two auditors could reach opposite verdicts on the
same prose. This amendment writes a reach test into the rule. The operator
directed its content on 2026-09-18, relayed by the lead, in three parts:

- **The file-level reading.** A shipped vendored library is a public contract,
  so the SPEC of the kit that owns it may name its functions.
- **A cross-kit clause.** Another kit's underscore-prefixed helper is internal,
  and naming it is a finding.
- **One sentence for native-crate identifiers.** The test turns on whether an
  item is *visible past its module*, and a private item belongs to the
  component its nearest `spec:` pointer names. This is option N1, given after
  an advisory consult.

The amendment then sweeps the citations the test turns into findings.

**Probed at authoring, 2026-09-18.**

- doctrine-kit/DOCTRINE.md rule 3 states no reach test. Its judgment line,
  "stays in prose iff a consumer outside the file must type or configure it",
  says nothing about a library's own SPEC or about a citation that crosses
  kits. So the tree does not already do what this amendment asks.
- The audit roster's scope line defers the question
  (`.workflow/audit-roster.txt:4`: "which identifiers count as internal is
  vendored-library-identifier-reach's").

## What changes

### (1) De-literalization states the reach test {design-bearing}

**Not yet applied.** This text replaces the body of doctrine-kit/DOCTRINE.md
rule 3 from "The duty widens past values" through "…only the source's own
maintainer meets it." The lead line, *Under agent work:*, *Enforced by:* (delta
2 rewords it) and *Digest:* stay. The digest is untouched, so
`check-doctrine-registration` stays green. The merged text carries no date,
iteration or direction reference (the provenance seam).

> The duty widens past values to *source identifiers*: prose names the public
> contract — the knob, the command, the entry point another surface calls —
> never a source's internal identifiers or its step-by-step algorithm.
> **Reach, judged at the file and not the kit, decides whether a name may
> appear.** A shipped vendored library is a public contract by construction,
> so its owning kit's SPEC may name any function it defines. Beyond that kit
> only its exports may be named: an underscore-prefixed helper is private to its
> kit, and another kit's prose naming one is a finding. The native crate reads
> the same way, with the module in place of the file. An item visible past its
> module (`pub`, `pub(crate)`, `pub(super)`) may be named. A private item
> belongs to the component its nearest `spec:` pointer names (the item's own
> pointer, else its module's first line), and any other component naming it is
> a finding. **Reach is checked first.** Only a name it admits goes on to the
> citation test, so a private name out of reach stays a finding even when it is
> a decision's subject. Among admitted names, naming one as the subject of a
> contract or decision is a citation. Inventorying a source's helpers or
> narrating its branch conditions is the same defect as a copied knob value: a
> second source of a fact the code owns, stale at the next refactor. The WHY,
> the invariant and the public contract stay SPEC prose; the WHAT and the how
> live in the source behind a pointer.

### (2) *Enforced by:* points the audit at the test {mechanical}

**Not yet applied.** In the same rule's *Enforced by:*, this text replaces "the
source-identifier class cannot be gated cleanly — a SPEC legitimately names
public functions as contracts, so …":

> the source-identifier class cannot be gated cleanly — a SPEC legitimately names
> an admitted identifier as a contract, and whether it cites one or inventories
> them is a judgment — so …

The rest of the clause stays: the carve-out, the audit roster seed member and
its cadence.

### (3) The audit roster's scope line cites the test {mechanical}

**Not yet applied.** In `.workflow/audit-roster.txt`, class
`internal-identifier-restatement`, `scope:` line, this text replaces "which
identifiers count as internal is vendored-library-identifier-reach's":

> which identifiers are internal is doctrine-kit/DOCTRINE.md's De-literalization
> reach test, checked before the citation test

Leave the line's other clauses as they are. `check-audit-roster` holds the
line's grammar.

### (4) Sweep the cross-component citations the test makes findings {mechanical}

**The corpus** is tracked governed markdown outside `docs/`, `.workflow/`,
`TASK-QUEUE.md` and the `gate-tests/` fixtures. Two passes find the candidates:

- **Shell:** every backticked name matching `` `_[a-z][a-z0-9_]+` ``. Skip
  `_layouts` at site-kit/SPEC.md:309, which is a Jekyll directory, not a helper.
- **Native:** every backticked name that matches a native/src item that is not
  visible past its module, owned by the component its nearest `spec:` pointer
  names.

Re-run both at build before acting. The findings, each with its de-literalized
value:

- `gate-sdk/SPEC.md:4627`. "`_guard_redirect_pairs`" names guard-kit's private
  helper, so it becomes "guard-kit's redirect-pair scan". The same sentence's "one
  of them is an `_`-prefixed internal helper outside the documented surface"
  becomes "one of them is internal to guard-kit". It is a decision's subject, but
  reach is checked first.
- `gate-sdk/SPEC.md:6964`. "`_spec_prune_kit_roots` is a prefix test" names
  canon-kit's retired helper in the present tense. It becomes "The shell prune
  was a prefix test".
- `gate-sdk/SPEC.md:14901`. "The installer's `run_vendored`" names an item
  private to installer: the nearest pointer is `native/src/installer/init.rs:759`,
  citing installer/SPEC.md §init. It becomes "The installer's spawn of the
  vendored front-end".

Clean on probe, and left as they are:

- The 37 `` `_guard_* `` citations in guard-kit/SPEC.md, from `grep -oE
  '`_guard_[a-z_]+' guard-kit/SPEC.md | wc -l`. Own kit, own library.
- `_gate_prebinary_knob` at gate-sdk/SPEC.md:9399 and :10674. It is defined at
  `gate-sdk/lib/gate.sh:61`: own kit, own library.
- gate-sdk/SPEC.md:4927 (`_gate_knob_value`, stated as defined nowhere), :5985
  (`_with_templates`, past tense, deleted) and :15119 (`_header`/`_footer`, stated
  retired). Each records history and claims no present membership.
- `replace_all`, whose own `spec:` pointer names gate-sdk/SPEC.md §The POSIX ERE
  matcher, and `pack_tracked`, whose nearest pointer names gate-sdk/SPEC.md
  §Consumer payload. Every other private item gate-sdk/SPEC.md names is
  gate-sdk's own by its module's first-line pointer.

### (5) Replace the dead name at gate-sdk/SPEC.md:3633 {design-bearing}

This is off-class: an accuracy defect found on the way, fixed here and not
counted as a finding. The passage says the two prefix families cost the arm no
new mechanism, "`_gate_knob_prefix_emit` resolving each inside the owning kit's
already-sourced subshell". `git grep -n _gate_knob_prefix_emit` finds no
definition anywhere in the tree, so the sentence names a mechanism that does not
exist. Build finds how `--run-validate` resolves a prefix family today, starting
from `walk::knob_prefix` and the arm's own module. It rewrites the clause to
state that mechanism by its public name, or by none. If it finds the families
now cost a mechanism, the claim "no new mechanism" is corrected with it.

### (6) Regenerate the mirrors {mechanical}

`docs/doctrine-kit/DOCTRINE.md` and `docs/gate-sdk/SPEC.md` are refreshed with
the command `check-docs-mirror-fresh` prints.

## Producers and consumers

- **The reach test** (delta 1). The doctrine produces it. Its consumers are:
  - every `internal-identifier-restatement` audit run, which reaches the rule
    through the roster scope line (delta 3);
  - every authoring session applying De-literalization.

  No gate reads the prose. The digest is unchanged, so
  `check-doctrine-registration`'s per-rule lockstep is untouched.
- **Ownership of a private native item** (delta 1). It is derived from the
  `spec:` pointers native modules already carry: all 256 `native/src/*.rs` lead
  with one, per `git ls-files 'native/src/*.rs' | xargs -n1 head -1 | grep -c
  '^// spec: '`. Its reader is the auditor. The rule mints no field.
- **Point 5:** no corpus narrows.
- **Point 6:** delta 4 enumerates its corpus's members and names each member's
  value above. Delta 5's value is what build finds, stated as its method.

## Existing sections updated

- doctrine-kit/DOCTRINE.md rule 3, De-literalization (deltas 1 and 2).
- `.workflow/audit-roster.txt`, the `internal-identifier-restatement` scope line
  (delta 3).
- gate-sdk/SPEC.md lines 4627, 6964 and 14901 (delta 4) and line 3633 (delta 5).
- `docs/doctrine-kit/DOCTRINE.md` and `docs/gate-sdk/SPEC.md` (all deltas).

## Retired spellings

- None — no delta retires a name. Deltas 4 and 5 remove citations of names that
  are still live in their own component, were already retired, or were never
  defined. Delta 1 rewords a rule without renaming it.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness
      check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them
      (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it; the merged spec
      reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in
      `## Retired spellings` above, and `check-amendment-retired-spelling` runs
      each declaration against the whole tracked tree, not against the specs
      alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks.
- [ ] **The entry moves to Done before the drain stage** — at the batch that
      merges this amendment.
