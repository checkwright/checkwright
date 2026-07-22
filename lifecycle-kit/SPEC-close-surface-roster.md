# SPEC amendment: close-surface-roster

Replaces close's hand-written enumeration of its inbound triage surfaces with a
**derived roster** carrying a per-surface `forced` / `advisory` mode, so that
"close never read this surface" becomes a distinguishable state instead of an
invisible one. Adds a declaration directive, a derivation affordance, and one
gate.

The failure this addresses is silent and permanent: today a surface exists on
close's list only because a prose placeholder names it, so adding a sixth inbox
and forgetting to name it costs nothing at the moment of the mistake and
everything afterwards.

## What changes

### New directive: `close-surface:`

A surface declares itself in the SPEC section that already owns it — one
full-line directive, the same shape and altitude as the `spec:` and `contract:`
directives canon-kit governs:

```
close-surface: <path> <mode> [reclaim=<command>]
```

- `<path>` — repo-relative, or a `<file>#<section>` locator when the surface is
  a section of a larger file (the queue's Lessons section is one).
- `<mode>` — exactly one of:
  - `forced=<owner-path> §<section>` — a structural forcing function exists and
    the citation names it. The gap inbox's is the iteration-boundary entry
    refusal; the Lessons section's is the same refusal's sibling assertion.
  - `advisory` — no forcing function. Close reads it by procedure, and a skip is
    **sanctioned and visible** rather than undetected. This is the marking the
    unit exists to add: an advisory surface is not a lesser surface, it is one
    whose skip is a judgment someone may audit.
- `reclaim=<command>` — required when `<path>` is a capture-tier (gitignored)
  member, naming the drain that empties it; the runtime-artifact lifecycle rule
  already demands a paired reclaim path for every write path, and this is the
  first place it becomes machine-readable.

Declaration lives with the owner, never in a central list: a central list is a
second source that drifts from the surface it names, and the one-owner rule puts
the fact where the surface is defined.

**Declarations live on manifest surfaces, and that is load-bearing** rather than
incidental — it is what gives a `forced=` citation its resolver (assertion B
below). A declaration on a non-manifest surface (a kit template) would carry an
unresolved citation, so the directive is not available there.

### New affordance: `lifecycle-kit/bin/close-surfaces.sh`

Prints the derived roster, one line per surface: `<path> <mode> <owner>`. It is
a **derivation, not a registry** — nothing about the roster is maintained by
hand. Two sources, unioned:

1. Every `close-surface:` declaration across the resolved kit roots and the
   consumer's governed manifest surfaces (gate-sdk's multi-kit path resolution,
   consumer-first with kit shadowing — the same resolution order every kit
   registry already uses).
2. Every **gitignored member of the workflow directory** — capture-tier by
   definition, therefore close-inbound by definition (gate-sdk/SPEC.md §The
   workflow directory, landed by the sibling `workflow-file-format-convention`
   amendment).

Source 2 is the closure that makes the roster fail loudly. A capture surface
added with no declaration appears in the roster as `(undeclared)` rather than
not appearing at all — the roster reports the hole instead of inheriting it,
which is the whole difference between a derived roster and a maintained one.

Follows the established affordance contract: repo-root cd, config-via-env, exit
2 on an unreadable configuration. Advisory tooling with no fixture pair owed;
the gate below is what blocks.

### New gate: `check-close-surfaces`

Invariant, over the derived roster: (A) **no undeclared surface** — every
capture-tier workflow-dir member carries a declaration; (B) **every declaration
carries a mode**, and a `forced=` mode's citation is *well-formed* — a
repo-relative `<path>.md` followed by `§<section>`; (C) **every capture-tier
declaration names a reclaim command**.

**Assertion B is shape-only, and resolution is somebody else's job already.**
`check-spec-pointer`'s prose-citation pass sweeps every manifest surface for a
free-prose `<path>.md §<heading>` citation and resolves it in prefix mode
(`canon-kit/checks/check-spec-pointer.sh`, the pass below the directive pass),
skipping fenced blocks. A `forced=` citation on a manifest surface *is* that
shape, so it is resolved today with no new code — which is why the directive is
restricted to manifest surfaces above.

This is the same presence-and-shape / resolution division the sibling
`workflow-file-format-convention` amendment takes, and taking it here is not a
preference: `heading_present` is defined **inside** `check-spec-pointer.sh`, not
exported from a library, so there is no resolver a second gate could call.
Reaching it would mean either copying the resolver — which the tiering rule bans
— or making lifecycle-kit depend on canon-kit, the same ownership-cycle argument
that rules out the log merge in §Ruled out below. The honest arrangement is two
gates independently reading one surface, each asserting what it owns.

Fail-closed on an unreadable declaration surface and on a roster the affordance
could not derive. Tier `precommit`. `# graph:` manifest couples the gate to the
workflow dir and the declaration surfaces.

Calibration and honest limit: the gate asserts the roster is **complete and
moded**, never that close actually *read* a surface. Reading is a session act
with no mechanical residue short of a per-surface disposition stamp, which is
the heavier design this unit deliberately does not take: the marking converts
an invisible omission into a sanctioned one, and a stamp would convert it into a
gated one at the cost of a stamp-per-surface-per-iteration ritual. The lighter
disposition is taken first because an advisory surface's skip is often correct,
and gating a correct action is the failure mode the gap-inbox refusal already
demonstrated.

**Knob.** One: `LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS` — the consumer's declaration
surfaces beyond the kit roots. Its default is a lifecycle-kit-owned glob literal
(`*/SPEC.md`), and the consumer widens it to its own manifest set in
`scripts/lifecycle-config.sh`. It deliberately does **not** default to
`CANON_KIT_MANIFEST_FILES`: reading another kit's knob would make lifecycle-kit
depend on canon-kit's configuration for a value the consumer already owns, and
the one cross-kit knob read in the tree today is precedent, not a ruling. The
declaration vocabulary itself is kit-owned and carries no consumer content; the
roster is derived, never a kit literal — a kit shipping the *names* of a
consumer's inbound surfaces would publish that consumer's private workflow.

### Ruled out: shrinking the roster by merging the two capture logs

The obvious way to make close's inbox count smaller is to merge the two
friction capture logs behind one file with a type column. Recorded here as
ruled out, because this unit's evidence is what settles it:

- The two logs are owned by **different kits**, and the dependency runs one way
  only — the drift-kit KPI already reaches into guard-kit through the shared kit
  root resolution, so guard-kit cannot depend back without a cycle. A merged log
  has no legal owner short of the base gate framework, which is not a friction
  sink.
- Their producers are not the same kind of act: one is a **harness hook
  fallthrough** writing raw command text at the moment of a prompt, undated and
  ungrammared; the other is a **deliberate structured capture** (`<date> <fact> ←
  <surface>`). A type column would not unify them, it would document that they
  were never one stream.
- Their consumers are disjoint: allowlist-filtering and pattern-ranking on one
  side, doc-owner remediation on the other. Every consumer would filter by type
  first — re-deriving the two logs at read time, which is the tell that the merge
  moves the split rather than removing it.
- Their reclaim moments are independent whole-file truncations. Sharing one file
  makes each sweep's `: >` drain erase the other type's untriaged lines.

The complaint the merge was reaching for is real — the two frictions compete for
one triage attention and were ranked against each other by nothing — and this
amendment answers it directly: both appear on one derived roster, with modes,
which is what "ranked against each other" needs. Merging the files was the proxy
for that, not the thing itself.

## Producers and consumers

**New interface: the `close-surface:` directive.**

- *Producer* — a kit or consumer author, in the SPEC section owning the surface.
  Enabling config: none new for kit-owned surfaces (kit roots are already
  resolved by every registry); `LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS` defaults to
  the manifest set the consumer already populates, so no deployment sets a knob
  to make the declaration live.
- *Consumers* — (1) `bin/close-surfaces.sh` at derivation; (2)
  `check-close-surfaces` at pre-commit.

**New interface: the derived roster.**

- *Producer* — `bin/close-surfaces.sh`.
- *Consumers* — (1) the **close skill's housekeeping step**, which replaces its
  prose enumeration with a run of the command and dispositions each line;
  (2) `check-close-surfaces`.

**Fields and their named readers.**

- `<path>` — read by close to open the surface; read by the gate at the
  capture-tier closure check (source 2 against source 1).
- `<mode>` — read by close to decide whether a skip needs a stated reason
  (`advisory`) or is impossible (`forced`); read by the gate at assertion B.
- `forced=`'s citation — read by the gate at assertion B as a shape; read by
  `check-spec-pointer`'s prose pass, which resolves its path and `§section`;
  read by a close session verifying the forcing function still exists before
  trusting it.
- `reclaim=` — read by close's **runtime-artifact lifecycle step**, which today
  asks the same question in prose and has no roster to ask it against; read by
  the gate at assertion C.

No field is introduced without a reader, and none is populated at a transition
where it is not read: `reclaim=` is absent on checked-projection surfaces because
those are not drained.

## Existing sections updated

- **lifecycle-kit/SPEC.md** gains `## The close-surface roster` (the directive
  grammar, the two derivation sources, the closure argument, and the merge
  ruling above) plus `### bin/close-surfaces.sh` and `### check-close-surfaces`
  under §Per-component contracts, and the new knob under §Layout and
  configuration.
- **lifecycle-kit/templates/skills/close.md** — the housekeeping placeholder's
  surrounding prose stops implying the consumer's binding enumerates its triage
  surfaces and instead directs it to run the roster command; the
  **runtime-artifact lifecycle step** cites `reclaim=` as the roster field that
  answers it, rather than re-asking in prose.
- **lifecycle-kit/SPEC.md §The committed gap inbox** — the inbox's own
  declaration lands here (`forced=`, citing the boundary refusal it already
  describes), so the section that documents the forcing function is the section
  that declares it.
- **queue-kit/SPEC.md** — the Lessons section declares itself `forced=` citing
  `lifecycle-kit/SPEC.md §bin/enter-stage.sh`, which owns the sibling refusal
  (both refusals are implemented there, one per surface).
- **guard-kit/SPEC.md** and **drift-kit/SPEC.md** — each capture log declares
  itself `advisory` with its `reclaim=` truncation, in the section that already
  names that reclaim path.
- **doctrine-kit/DOCTRINE.md** — the enforcement-first carve-out's audit-roster
  cadence gains the workflow-surface class, since the "did close actually read
  it" half stays un-gateable by the calibration above.
- **The consumer's `gates.list`**, the generated pre-commit hook,
  `docs/check-graph.html`, and `docs/enforcement.md` take the new gate; the
  consumer's close binding replaces its prose sweep list with the command.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
