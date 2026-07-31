# SPEC amendment: upgrade-smoke-note-resolution

The standing pre-release assertion — `bin/upgrade-smoke.sh` at its `TO=HEAD`
default — is satisfiable today only by an iteration that tightens nothing. This
amendment makes the allowed-red declaration resolvable at an untagged `TO`, so
the assertion proves containment rather than emptiness.

The premise this rests on, and the reason the unit is feature-shaped: the present
behavior is **specified, not accidental**. The script's `# spec:` line at the
declaration-resolve step and gate-sdk/SPEC.md §upgrade-smoke both state that an
unreleased `TO` resolves no version, so no note, so the red set must be empty. A
fix cannot converge the script onto a contract that already says otherwise; it
revises the contract, which is what earns the amendment.

## What changes

### Delta 1 — the declaration is a property of the TO tree, resolved over a version window {design-bearing}

`TO`'s version is resolved from a tag pointing at `TO`. Every commit before the
iteration's final one is untagged, so the resolution yields nothing and the
declaration is empty for the whole iteration. Replace the key, not the artifact:
the note already lives in the extracted `TO` tree and is already committed before
the tag — only the way `TO` is matched to it changes.

The new resolution, at the same step:

1. **`fromver`** — the version `FROM` stands at: the newest `v*` tag reachable
   from `FROM`. `FROM` is a knob, so it need not itself be a tag; what the window
   needs is the version already released at the baseline. A `FROM` from which no
   `v*` tag is reachable is exit 2 — an unversioned baseline opens no window, and
   guessing one would license reds against nothing.
2. **`TO` tagged** — unchanged. The version is the `v*` tag pointing at `TO`, and
   the note is the `TO`-tree post whose `release:` key names it.
3. **`TO` untagged** — the **pending note**: the `TO`-tree post under
   `docs/posts/` whose `release:` version is strictly greater than `fromver`.
   - Exactly one such note — it is the declaration, read exactly as a tagged
     `TO`'s note is read.
   - None — the declaration is empty and the red set must be empty. This is
     today's rule, kept, but as the narrow case rather than the universal one:
     an iteration that reddened a gate and declared nothing has declared nothing.
     The refusal at that branch names the missing pending note as the remedy
     instead of only reporting that no note was found.
   - More than one — exit 1. Two unreleased notes above the baseline is a tree
     the upgrade contract does not describe, and silently picking one would
     choose which declaration governs without saying so.

The version comparison orders `release:` keys the way `check-release-bump`
already orders this same corpus; the window is `(fromver, ∞)`, open at the
baseline so a note at `fromver` — the release already reconciled at `FROM` —
can never license a red at `TO`. That exclusion is the load-bearing half of the
window: without it, an iteration that tightens a gate while authoring no note
would inherit `FROM`'s own declaration and mask exactly the reds the assertion
exists to catch.

### Delta 2 — the note-authoring step moves ahead of validate for a tightening iteration {design-bearing}

This is the causal consequence, and it is why the delta cannot stop at the
script. The declaration's producer is the release session (RELEASING.md §The
procedure step 1) and its consumer is the `upgrade` validate suite, which runs
**earlier in the same iteration**. Under delta 1 the producer must be reachable
before the consumer, so step 1's "in-iteration" qualifier tightens into a stated
ordering: an iteration that tightens a gate authors the note's Tightened-gates
section before validate, or its `upgrade` suite is honestly red until it does.

An iteration that tightens nothing is untouched — no pending note, an empty
declaration, an empty red set, green. The ordering binds exactly the iterations
that owe a declaration.

### Delta 3 — the two statements of the old contract are revised {design-bearing}

gate-sdk/SPEC.md §upgrade-smoke's untagged-`TO` sentence and its
producers-and-consumers paragraph, and the script's `# spec:` line at the
declaration-resolve step. Both currently assert the empty-declaration rule as
the contract; both state the window instead. Leaving either is the defect this
unit was filed for, one surface over.

### Delta 4 — docs/install.md states that the declaration is a working-tree artifact {design-bearing}

§The upgrade contract describes the note as the thing a release ships. Under
delta 1 a consumer running the smoke against their checkwright clone at an
untagged `TO` resolves the **pending** note, so the declaration is owed from the
moment a gate tightens rather than from the tag. One sentence, in the
Tightened-gates paragraph that already names the section as the mechanical
allowed-red set.

### Delta 5 — the `upgrade` validate baseline row is promoted {mechanical}

`.workflow/validate-baseline.txt` holds `upgrade` at `fail` against this slug.
The hold ends when this lands, and the row goes to `pass` with the slug dropped.

Two traps the queue entry recorded, both binding here. **Do not promote the row
on a post-tag green**: immediately after a tag, `TO` is momentarily a tagged
`HEAD` and the smoke runs clean for that reason alone; the next commit restores
the defect. **Do not read a green as proof the window works**: this iteration
may tighten no existing gate, in which case the red set is empty and the window
is never exercised live. The exercise is delta 1's own argument and the smoke's
assertion messages, not a green run.

## Producers and consumers

- **`fromver`** — produced at the resolve step from `GATE_SDK_UPGRADE_FROM`
  (default: the source repo's newest `v*` tag, already emitted by the script, so
  the zero-config run resolves it). Consumed only by the window predicate, in
  the same script, at the same step. It crosses no process boundary and is not a
  new interface; it is a local derivation named here because the window's
  correctness depends on which version it stands for.
- **The pending note** — produced by the release session as a committed
  `docs/posts/` post in the `TO` tree (RELEASING.md §The procedure step 1).
  Consumed by the smoke's declaration parse and by the human upgrader reading the
  site. The producer is reachable rather than test-only: notes are authored and
  committed before the tag today, so delta 2 changes only the producer's ordering
  relative to this consumer, not whether it runs.
- **The ambiguity refusal** — produced at the resolve step when the window holds
  more than one note; consumed by the operator through exit 1 and the assertion
  message naming each candidate. It is the only new failure mode, and its reader
  is the operator, at the resolve transition.
- **No new field, no new knob.** The knob roster in §upgrade-smoke
  (`GATE_SDK_UPGRADE_REPO` / `_FROM` / `_TO`, and `GATE_SDK_TMP_DIR` for scratch)
  is unchanged, so that list takes no delta. The resolution is derived entirely
  from refs the script already holds — which is the point: a knob here would let
  a consumer configure away the assertion.

**Seam.** Everything in this amendment is generic mechanism: a version window
over a directory of dated notes. The `docs/posts/` literal the script already
carries is unchanged and stays a kit literal — it names a layout convention
site-kit's own defaults reference, not private rule content, and no vocabulary
crosses. No consumer config is created, and none is owed.

## Existing sections updated

- **gate-sdk/SPEC.md §upgrade-smoke** — the untagged-`TO` sentence and the
  producers-and-consumers paragraph (delta 3); the knob roster is explicitly
  unchanged (delta 1's producers-and-consumers note above).
- **docs/install.md §The upgrade contract** — the Tightened-gates paragraph
  (delta 4).
- **RELEASING.md §The procedure step 1** — the authoring-before-validate
  ordering (delta 2).
- **`gate-sdk/bin/upgrade-smoke.sh`** — the `# spec:` line at the
  declaration-resolve step (delta 3), alongside the resolution itself (delta 1).
- **`.workflow/validate-baseline.txt`** — the `upgrade` row (delta 5).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
