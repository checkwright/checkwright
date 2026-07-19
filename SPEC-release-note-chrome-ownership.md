# SPEC amendment: release-note-chrome-ownership

## What changes

A release-note post carries three *variable* sections (Tightened gates,
Renamed knobs, Behavior changes) whose grammar docs/install.md §The upgrade
contract owns and `check-release-bump` floors, and it carries *fixed chrome*:
the opening reserved-phrasing sentence and the whole closing "Upgrading"
paragraph (the "mechanical allowed-red set" / "open an issue … a defect in the
release rather than work for you" boilerplate). RELEASING.md step 1 names the
variable sections and the reserved opener *phrase*, but hands the author **no
skeleton for the fixed chrome**. So each note's chrome propagates by copying
the previous post — derivation-by-precedent, which the de-literalization /
derivation-first doctrine rules out, and which passes every content gate
byte-identically to a derived note (a correctness gate cannot see the
derivation path).

**Ruling — give the chrome one owning surface: RELEASING.md step 1.** Inline
the canonical fixed chrome (the reserved opener sentence and the full closing
"Upgrading" paragraph) into RELEASING.md step 1 as the **single authoritative
source**, and add an explicit instruction: *author each note by filling this
runbook's skeleton — the fixed chrome verbatim from here, the three variable
sections per docs/install.md §The upgrade contract — never by copying a prior
post.* This closes the no-owner gap: the chrome stops propagating by precedent
and derives from one cited source, satisfying SSOT / derivation-first at
proportionate cost.

**Ruled out — a freshness-gated skeleton (the deferred entry's option a).** A
`docs/posts/` template with slot markers plus a freshness gate asserting each
note's chrome matches it was considered and **rejected as disproportionate**:
notes are *dated immutable artifacts*, so a freshness gate can only assert the
newest note against the skeleton (older posts legitimately carry older chrome),
manufacturing gate machinery and a new template file against a cost the entry
itself rates low and non-rotting. This alternative is recorded so a later
session does not re-derive it: promote to the gated form only if
copy-from-prior-post imitation actually recurs after the chrome has an owner —
demand-gated, the repo's standing posture for enforcement machinery.

**Envelope.** This owns *one artifact's* chrome. The methodology-level
generalization — making precedent-imitation recognizable everywhere — is the
sibling unit `kfric-trigger-prior-artifact-consultation`, out of this
amendment's scope. No change to the three variable sections, their grammar
owner, or `check-release-bump`.

## Producers and consumers

- **Producer** — RELEASING.md step 1 is the release-note author's procedure;
  it gains the canonical chrome as inline text plus the "fill the skeleton,
  never copy a prior post" instruction. The producer of each note remains the
  closing session running that step.
- **Consumer** — the release-note reader (the chrome is reader-facing prose)
  and the launch-copy phrasing rule (RELEASING.md already names the reserved
  opener phrase; this amendment supplies the full sentence it opens). The
  GitHub Release points at the post (RELEASING.md step 5, unchanged). No
  machine consumer parses the chrome — `check-release-bump` reads only the
  three variable sections, so the chrome is outside its parse and this change
  cannot red it.
- **No new file, knob, gate, or tag.** The chrome moves from "implicit in the
  last post" to "explicit in RELEASING.md"; the only new reader is the human
  author, who reads the runbook they already run.

## Existing sections updated

- **RELEASING.md step 1** — add the canonical fixed chrome (reserved opener
  sentence + closing "Upgrading" paragraph) as the single source, and the
  explicit "author from this skeleton, never copy a prior post" instruction.
  Keep the existing pointer to docs/install.md §The upgrade contract for the
  three variable sections (unchanged division of labor: this runbook owns the
  fixed chrome, that page owns the variable-section grammar).
- **docs/install.md §The upgrade contract** — no change required; a one-line
  cross-reference that the fixed chrome is RELEASING.md's is optional and
  additive if it reads more clearly, but the grammar this page owns is the
  variable sections only.

No wire contract, no fenced source to embed. The canonical chrome text is
reader-facing prose authored in RELEASING.md, not a copy of a tracked source
file.

## Definition of Done

- [ ] **Causal completeness** — the chrome's single producer (RELEASING.md
      step 1) and its reader (the note's human author + the post's reader) are
      named; no machine consumer, no new field.
- [ ] **Merged with no information lost** — the canonical chrome and the
      never-copy-a-prior-post instruction land in RELEASING.md step 1; the
      variable-section division of labor with docs/install.md still reads
      coherently.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root for this unit (`ls SPEC-*.md`).
- [ ] **Removals propagated** — confirm no other surface claims to own the
      release-note chrome (there is none today; the gap is that nothing did).
- [ ] **Gaps filed** — any cross-component gap found during the work filed as a
      debt task.
