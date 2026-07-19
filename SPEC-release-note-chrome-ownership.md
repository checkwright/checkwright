# SPEC amendment: release-note-chrome-ownership

## What changes

A release-note post carries three *variable* sections (Tightened gates,
Renamed knobs, Behavior changes) whose grammar docs/install.md §The upgrade
contract owns and `check-release-bump` floors, and it carries a closing
"Upgrading" paragraph plus a reserved opener sentence. The original premise of
this amendment — that the whole closing paragraph is *byte-fixed* chrome — is
**false**, corrected by the align-stage audit of `docs/posts/`:

- **Byte-identical across every current note:** the two closing sentences (the
  "declared for reading … open an issue … a defect in the release rather than
  work for you" tail) and the reserved opener *phrase* RELEASING.md:34 already
  names. This is the genuine fixed chrome — unowned today, propagated by
  copying the previous post.
- **Per-release variable, NOT fixed:** the allowed-red framing (v0.6.0 "**No
  allowed reds.** … Tightened gates … empty" vs v0.5.0 "**The one allowed
  red.** … the mechanical allowed-red set") and the "Sync … regenerate"
  paragraph (v0.5.0 names the lifecycle marker-block regen; v0.6.0 does not).
  These legitimately differ per release; freezing them verbatim would be wrong.

So there are two distinct problems, not one: a small **byte-fixed tail** with
no owner (copied by precedent), and **variable framing** re-derived by
imitating the prior post's structure. Both are derivation-by-precedent; each
needs a different remedy.

**Ruling — a slotted skeleton in RELEASING.md step 1 (align's option 2).**
RELEASING.md step 1 owns a note skeleton with two parts:

1. **The byte-fixed tail + reserved opener, verbatim** — lifted once from the
   current posts to seed the single authoritative source, thereafter owned
   here (that one-time lift establishes SSOT; it is not ongoing
   copy-from-prior-post). RELEASING.md becomes the one place the fixed chrome
   lives.
2. **Named author-fill slots for the per-release variable parts** — a
   `{sync/regen for this release}` slot and a two-way allowed-red framing slot
   (`{reds present}` / `{none}`). The allowed-red slot **points to
   docs/install.md §The upgrade contract's allowed-red-set grammar
   (install.md:186, :208)** rather than restating it — install.md stays the
   single owner of "allowed-red set"; the skeleton cites it (de-literalization:
   cite the owner, never copy the value).

Plus the explicit instruction: *author each note by filling this skeleton —
fixed tail verbatim, variable slots per their cited grammar — never by copying
a prior post.* This closes both problems: the fixed tail gets one owner, and
the variable framing gets a grammar-owned slot instead of a copied structure.

**Ruled out:**

- **Inline the whole closing paragraph verbatim (this amendment's original
  wording).** Rejected on the corrected premise: most of that paragraph is
  per-release variable, so verbatim-inlining would freeze this-release content
  into false boilerplate. The finding that retired it is recorded so it is not
  re-attempted.
- **A freshness-gated skeleton (the deferred entry's option a).** Still
  rejected as disproportionate: notes are dated immutable artifacts, so a
  freshness gate can only assert the newest note, manufacturing gate machinery
  and a template file against a cost the entry rates low and non-rotting.
  Demand-gated — promote only if imitation recurs after the chrome has an owner.
- **Split the allowed-red framing grammar into docs/install.md (align's
  option 3).** Rejected: install.md *already* owns "allowed-red set"
  (:186, :208), so a pointer (part 2 above) consumes that ownership without
  moving anything — the same clean seam at lower cost. Moving the framing
  grammar wholesale would reopen and widen the sibling amendment
  `release-note-section-taxonomy`'s envelope (ruled to two specific edges —
  knob-removal spelling and the four-to-three mapping) and would split this
  amendment's single-surface ownership across two surfaces for no gain the
  pointer does not already give.

**Envelope.** The goal is unchanged from promotion: give the release-note
chrome an owner and kill derivation-by-precedent for it. The mechanism is
refined — from "inline the fixed chrome verbatim" to "a slotted skeleton whose
fixed tail is verbatim and whose variable slots cite their grammar owner" —
because align corrected the false byte-fixed premise. Still *one artifact's*
chrome; the methodology-level generalization remains the sibling
`kfric-trigger-prior-artifact-consultation`. **This amendment does not reopen
`release-note-section-taxonomy`:** it adds nothing to docs/install.md, only
points at that page's existing allowed-red-set ownership.

## Producers and consumers

- **Producer** — RELEASING.md step 1 gains the skeleton: the byte-fixed tail +
  reserved opener as owned verbatim text, the two named slots, the pointer to
  install.md's allowed-red-set grammar, and the "fill the skeleton, never copy
  a prior post" instruction. The producer of each note remains the closing
  session running step 1.
- **Consumer** — the release-note reader (the chrome is reader-facing prose);
  the note author who fills the slots; and, for the allowed-red slot's grammar,
  docs/install.md §The upgrade contract as the cited owner (existing reader, no
  new field). The GitHub Release still points at the post (step 5, unchanged).
  No machine consumer parses the chrome — `check-release-bump` reads only the
  three variable sections, so this change cannot red it.
- **No new file, knob, gate, or tag.** The fixed tail moves from "implicit in
  the last post" to "explicit in RELEASING.md"; the variable framing moves from
  "copied structure" to "a slot citing install.md." The only new reader is the
  note author, who reads the runbook they already run.

## Existing sections updated

- **RELEASING.md step 1** — add the note skeleton: the byte-fixed tail +
  reserved opener verbatim (owned here), the `{sync/regen}` and two-way
  allowed-red slots (the allowed-red slot citing docs/install.md §The upgrade
  contract, not restating it), and the "author from this skeleton, never copy a
  prior post" instruction. Keep the existing pointer to §The upgrade contract
  for the three variable sections.
- **docs/install.md §The upgrade contract** — **no change.** The skeleton's
  allowed-red slot points at this page's existing allowed-red-set ownership
  (:186, :208); nothing is added or moved here. (This is the seam that keeps
  the sibling amendment `release-note-section-taxonomy` un-reopened.)

No wire contract, and no fenced source: the byte-fixed tail is cited to its
current location in `docs/posts/` for the one-time lift, not embedded here.

## Definition of Done

- [ ] **Causal completeness** — the fixed tail's single producer (RELEASING.md
      step 1) and reader (note author + post reader) are named; each variable
      slot names its author (producer) and, for the allowed-red slot, its cited
      grammar owner (install.md); no new field, so no unread field.
- [ ] **Merged with no information lost** — the skeleton (verbatim tail + slots
      + pointer + never-copy instruction) lands in RELEASING.md step 1; the
      division of labor with docs/install.md reads coherently and adds nothing
      to that page.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root for this unit (`ls SPEC-*.md`).
- [ ] **Removals propagated** — confirm no other surface claims to own the
      release-note chrome, and that the allowed-red framing is cited from
      install.md, never duplicated into RELEASING.md.
- [ ] **Gaps filed** — any cross-component gap found during the work filed as a
      debt task.
