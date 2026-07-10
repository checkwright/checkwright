# SPEC amendment: prose-enum-drift

## What changes

New gate `checks/check-prose-enum.sh` (`precommit` tier) — the prose analog
of check-kit-enum's hand-list doctrine and the enumeration sibling of
check-manifest-count's restated totals.

Invariant: within one paragraph window of the manifest set
(`spec_manifest_files`, fences skipped — the count gate's walk), prose that
names two or more distinct members of one declared governed set must name
every member of that set, unless an exempt context holds. Engagement at two
mirrors check-kit-enum: a lone member is a mention, not a list. The evasion
this closes: check-manifest-count catches a bare cardinal over a governed
collection, but a row that *enumerates instead of counting* drifts just as
silently when the set grows (attested: the README queue-kit row's
"blocked-by/needs-spec/spec tag algebra" went incomplete when the attend tag
and the harvest tags landed — same restatement, different surface form, no
scanner).

Set declarations are consumer config, never gate literals (the provenance
seam): `SPEC_KIT_ENUM_SETS_CMD` (default empty — clean skip) names a
consumer command emitting one `<set-name><TAB><member>` line per member.
Member tokens are matched word-bounded, bracketed or bare — the attested
drift used bare stems. A declared command that fails, or an emitted line
that does not parse, is fail-closed (exit 2).

Exempt contexts follow the count gate's family shape — mechanical markers
first, per-site tag last: a subset marker inside the window (`e.g.`,
`such as`, `among them` — new to this gate, fixed generic-English mechanism
like the count gate's partitive markers), a partitive marker on the match,
and the per-site `prose-enum-exempt: <reason>` on the line or the one above.
The corrective names both legitimate fixes: cite the owning set by name, or
complete the enumeration — never trim to a silent subset (trimming is the
same defect check-kit-enum's help text pre-empts with the glob token).

Shared machinery — the check-manifest-count half of this unit: the
manifest-prose walk driver (fence tracking, the blank-line paragraph reset,
the per-site exempt window on the line or the one above) moves from
check-manifest-count's awk body into a `lib/spec.sh` adapter both
manifest-prose gates source, parameterized by the per-site marker name; it
feeds the paragraph-join window that is already a lib adapter (§lib/spec.sh).
One walk, one exemption behavior for the manifest-prose surface —
check-comment-tier keeps its caller-owned comment walk (§lib/spec.sh's
surfaces-disagree ruling stands), and check-manifest-count's observable
behavior is unchanged.

This repo's consumer config: `scripts/enum-sets.sh` derives the queue-tag
set from queue-kit's own parsing source plus `QUEUE_KIT_LESSON_TAGS` —
derived, not restated. A set a consumer can only hand-list is that
consumer's own drift to own; the kit contract asks only for the emit
grammar.

## Producers and consumers

- Producer: the generated pre-commit hook / `run-gates.sh` — registered in
  the consumer's `gates.list`; the `# graph:` couples the manifest set plus
  the consumer's sets command (a set change re-fires the gate over the
  docs).
- Consumer: the committing operator via the output contract; each hit is
  read once at the scan transition — file, line, set name, and the missing
  members all appear in the finding message (every emitted field has that
  one reader).
- `SPEC_KIT_ENUM_SETS_CMD` is read by the gate at startup; both fields of
  each emitted line are read at match time (set name in the report, member
  in the matcher).

## Existing sections updated

- §check-manifest-count: the prose-walk paragraph cites the shared
  lib/spec.sh adapters (paragraph-join window, exempt-site helper) instead
  of describing a gate-private walk.
- §lib/spec.sh: gains the walk adapter and the exempt-site helper beside
  the paragraph-join window already documented there.
- Calibration follows the count gate's procedure: tune against this tree,
  disposition every hit (cite the set / complete the list / site-exempt
  with reason). Ships with a good/bad fixture pair;
  `check-prose-enum.test.sh` covers the config-driven paths (a consumer set
  command, the exempt escapes) the empty default cannot reach.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as
      one coherent document a reader who never saw the amendment can use
      alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls spec-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
