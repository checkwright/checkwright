# SPEC amendment: comment-run-cap

`check-comment-tier` is more permissive than its own SPEC: a reason
directive blesses the whole contiguous comment run it opens, so a `spec:`
pointer trailed by relocated prose passes — yet the SPEC rules that a
`spec:` directive blesses only its own one-line binding, never a relocated
block. A build session took that blessing path twice in the drift-kit
iteration before deleting. This amendment bounds the blessing to the
directive's own physical wrap. Surfaced 2026-07-08.

## What changes

- **Bounded run-blessing in `check-comment-tier`.** A directive no longer
  blesses an unbounded contiguous run: it blesses its own line plus
  continuation lines up to `SPEC_KIT_COMMENT_RUN_CAP` total physical
  comment lines (new knob, default **3** — one logical sentence wrapped at
  the file's line width). A directive appearing mid-run opens a fresh
  window from its own line. Every comment line beyond a window classifies
  on its own: another roster directive, `comment-tier-exempt: <reason>`,
  positional rescue, or flagged. Blank `#` lines inside a run count toward
  the window (a paragraph break is a break in the binding, not a free
  continuation).
- **`usage:` joins the built-in reason roster** (colon needle). An
  invocation synopsis with argument semantics is the canonical
  genuinely-local fact — interface, below SPEC altitude — and today it
  survives only by riding a `graph:`/`spec:` header run. As a directive it
  anchors its own window; a long option roster restructures into
  directive-anchored short paragraphs or trims.
- **The FP-tension ruling** (recorded here so build does not re-litigate):
  a within-window continuation is the directive's own wording physically
  wrapped — blessed, neither deleted nor exempted. Beyond-window prose is
  presumed relocated restatement and is **deleted**; `comment-tier-exempt:`
  is reserved for a genuinely-local fact neither tier owns, and exempting a
  restatement is itself the defect (the blessing-relocation doctrine).
  Laundering a paragraph by sprinkling resolvable `spec:` pointers every
  few lines defeats the mechanical floor but stays a review defect — the
  gate is the floor, not the judgment.
- **Corpus calibration is the crux, and the corpus is this repo**: the
  scope-stage measurement found 194 directive-led runs on governed sources,
  161 of them within a 3-line window; the 33 over-window sites are almost
  all file-top headers whose usage prose rides the header directives. At
  build, run the tightened gate over the tree and disposition every hit —
  trim behavioral paraphrase (a header describes invocation, not the SPEC's
  invariant), re-anchor with `usage:`, or per-line exempt with reason. The
  default cap ships only as tuned against that sweep.
- **Fixtures**: the `bad/` fixture gains an over-window run (directive line
  plus trailing relocated paragraph); the `good/` fixture gains a wrapped
  within-window directive and a `usage:`-anchored header. Four gate
  contracts unchanged; `precommit` tier unchanged.

## Producers and consumers

- **Producer:** the generated pre-commit hook / `run-gates.sh`, reachable
  wherever `gates.list` already names `check-comment-tier` — no new
  registration.
- **Consumer:** the committing operator/agent via the gate output contract;
  the violation help text gains the window rule so the corrective form
  rides in the rejection.
- **Fields:** `SPEC_KIT_COMMENT_RUN_CAP` is read once by the classifier at
  scan; the window counter is scan-local state read at each comment-line
  transition. No new persistent state, no new files.
- **Interaction with `check-spec-pointer`:** unchanged — it validates that
  a `spec:` target resolves; this change narrows what the directive
  blesses, not what it points at.

## Existing sections updated

At merge into spec-kit/SPEC.md:

- §check-comment-tier: the invariant's "which also blesses the contiguous
  comment run it opens" becomes the bounded-window rule above; `usage:`
  joins the enumerated reason roster; the calibration paragraph gains the
  cap knob and the delete-over-exempt ruling.
- §Layout and configuration: gains `SPEC_KIT_COMMENT_RUN_CAP` (default 3).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls spec-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
