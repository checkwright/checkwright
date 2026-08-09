# SPEC amendment: prose-profile

The fourth install profile — a consumer whose artifacts are documents rather than
code — and what a first prose adopter meets at install. TRAJECTORY.md's priority
directive sequences this next and fixes where its coherence is measured: **at the
adopter's floor — install, get value, uninstall** — so the profile is not done
when it resolves to a kit set, it is done when a prose adopter gets a real red on
a real defect in their own prose and can reverse the install.

Two premises this unit was filed on are **falsified by measurement**, and the
design below is what survives them. Both measurements are in the survey record
with their witnesses; neither should be re-bought.

## Falsified premise 1 — the profile parameter is not the lever

The prior judgment was that the cohort ships as "membership rows plus the first
use of the profile parameter" at `recipe_gates(kit-payload-dir, profile)`.
Measured against the real derivations, that parameter cannot express the cohort,
and the reason is structural rather than a matter of effort:

- `full` is the **payload-derived maximum**, so *every* profile's kit set is
  contained in `full`'s by construction. The consumer smoke asserts gate-roster
  monotonicity over every comparable pair, which means **anything a profile
  registers, `full` must register too**. A gate armed "for prose only" is not
  expressible: the moment it is armed for prose it is owed to `full`, and once it
  is owed to `full` it is simply `zero-config`.
- `starter` (gate-sdk alone) is contained in every profile, so no profile may
  **drop** a gate-sdk `zero-config` member either.
- Subtraction in the remaining band is blocked from the other side:
  `check-install-disposition` assertion C forbids `lib/common/recipe.sh` from
  carrying any literal gate name, and no gate declares anything a profile-keyed
  omission could be derived from.

So the parameter stays exactly what its own directive already calls it — the seam
— and this amendment does **not** exercise it. That is a finding, not a
deferral: the sentence in `installer/README.md` §What init seeds saying a varying
roster "becomes a change to one gate rather than to a signature" is *correct*, and
what it describes is a disposition change. The parameter is the seam for a future
in which a gate declares profile-varying reachability; nothing declares it today.

Measured: `prose` = gate-sdk + canon-kit yields `profile_order` pairs
starter⊆delegation, starter⊆prose, starter⊆full, delegation⊆full, prose⊆full;
one minimum, one maximum, monotone on every pair — the lattice admits the fourth
profile with **zero machinery change**. `prose` and `delegation` are incomparable,
which is the "alternatives, not steps" case the lattice was stated for and which
this profile is the first instance of.

## Falsified premise 2 — the cohort is not where the value is

The prior reading was that canon-kit's document-governance core is agnostic
governance a prose adopter is merely not *receiving*. Measured on a real
`init` consumer: 16 of canon-kit's 18 `on-surface` gates already exit 0 on a bare
init tree — but **12 of them are green vacuously**, because
`spec_manifest_files` is canonical specs + `README.md` at any depth + `CLAUDE.md`
and `_spec_comment_surface` is `.sh`/`.gate`/`.rs`. Neither includes `docs/*.md`.
A prose adopter writing under `docs/` gets nothing from those twelve **at any
disposition**.

The honest consequence, and it is what keeps this amendment's promise small enough
to be true: the value delivered at install is **link, claim, staleness and pointer
governance over every `README.md` at any depth, plus `CLAUDE.md`** — which is real
for a documentation repository, since a docs tree is usually a tree of READMEs —
and widening to `docs/**.md` is one knob in the config seam `init` already
delivers. The profile does not silently promise governance over a corpus the gates
do not read.

## What changes

- **(D1) The `prose` profile.** Two membership rows in `installer/profiles.list`
  — gate-sdk and canon-kit — with the criterion recorded beside them, as
  `starter` and `delegation` each record theirs. The criterion: *the smallest set
  that governs a repository whose artifacts are prose, on a git-only floor.*
  gate-sdk is forced for the same reason it is forced in `starter` (no runner, no
  hook generator, no registry); canon-kit is the kit whose subject is authored
  documents, and every other kit's subject is either the agent session — which is
  `delegation`'s criterion, and why these two profiles are alternatives rather
  than steps — or a deployed site. **design-bearing** — the criterion is the
  judgment `profiles.list` exists to record, and it is what a fifth profile will
  be argued against.

- **(D2) Prune vendored kit roots from the manifest set's README finder.**
  `canon-kit/lib/spec.sh`'s manifest set prunes kit roots from the canonical-spec
  finder per `CANON_KIT_SCAN_KIT_ROOTS`, and does not prune the `README.md`
  finder beside it. So a consumer's governed manifest set includes its
  **dependencies'** READMEs — against that knob's own stated rationale, that a
  kit's own documentation is a dependency's rather than governed content. Apply
  the same prune to the README finder. **design-bearing** — it changes the
  governed corpus for every consumer, which is the widest blast radius in this
  amendment and the one delta that is not about profiles at all.

  It is here rather than in its own unit because it is D3's blocker, and the
  enforcement-first rule puts the fix and the thing that needs it in one unit:
  with the prune missing, `check-md-refs` reds on a **bare** init tree, with no
  adopter action, on `canon-kit/README.md`'s own `../queue-kit/` link, for any
  profile that vendors canon-kit without queue-kit. That is measured, and `prose`
  is exactly such a profile. Fixing the one link instead would leave the class
  live for every future narrow profile.

  This repo sets `CANON_KIT_SCAN_KIT_ROOTS=1`, so D2 is a no-op here and a fix
  for vendored consumers — which means this tree's own battery cannot be the
  oracle for it. Named as an honest limit, not worked around: the oracle is the
  consumer smoke.

- **(D3) Eleven canon-kit gates move `on-surface` → `zero-config`.**
  `check-md-refs`, `check-comment-tier`, `check-docs-cmd`, `check-knob-citation`,
  `check-knob-default-coupling`, `check-manifest-count`, `check-manifest-temporal`,
  `check-spec-fence-balance`, `check-spec-pointer`, `check-todo-task-liveness`,
  `check-tracking-claim`. Each reads a surface a tree `init` makes already has —
  the vendored sources, the READMEs, the agent file — rather than one the adopter
  authors later, so the `on-surface` declaration was conservative rather than
  necessary. The vocabulary stays closed at three values: no gate gains a new
  disposition, eleven gates move between existing ones. **mechanical** — eleven
  header lines; the judgment is spent here and the execution is oracle-running.

  **Seven stay `on-surface`, each for a reason that survives the measurement**,
  and this half is as load-bearing as the move:
  `check-surface-duplication` wants a `GLOSSARY.md` the adopter authors (it is
  also the one canon-kit gate absent from the kit's smoke roster);
  `check-docs-link-convention` wants a docs host **and** is measured hostile to
  honest first prose — it reds on a subpage linking `../README.md`, so arming it
  at install would spend a false-feeling red at the worst possible moment;
  `check-prose-tells` has its entire subject behind `CANON_KIT_PROSE_TELL_GLOBS`,
  default empty; and `check-deprecation-task`, `check-install-claim`,
  `check-payload-claim` and `check-prose-enum` are each green only because their
  vocabulary knob is unset — a vocabulary is a surface the adopter authors, which
  is precisely what `on-surface` names.

- **(D4) The value proof — a prose-shaped consumer that takes a real red.**
  No docs-shaped consumer exists anywhere: `run-consumer-smoke.sh`,
  `demo/run-demo.sh` and `installer/consumer-smoke/run-smoke.sh` all build generic
  near-empty scratch repos. The `install → uninstall` half of the trajectory's
  coherence bar comes free — `installer/consumer-smoke/run-smoke.sh` loops over
  `profile_names`, so D1's row inherits init, a green battery, manifest agreement,
  idempotent re-run, `doctor`, `diff` and the uninstall tree-object equality with
  no new assertion. The **get value** half does not. Add a consumer whose only
  content is markdown, carrying one real prose defect — a dangling relative link
  in a README — and assert the registered battery **reds** on it and goes green
  when it is fixed. **design-bearing** — the assertion has to prove value without
  pinning which gate delivers it, or it becomes a second roster to maintain.

- **(D5) Say it where an adopter reads it.** `installer/README.md` §Profiles and
  `docs/install.md` both describe three profiles that "happen to nest"; with a
  fourth that does not nest, the chain sentence stops being true and the lattice
  the contract already promises becomes the thing to say. **mechanical** — both
  surfaces already state the lattice contract beside the chain, so the edit
  deletes a stale aside rather than authoring a claim.

## The seam

**Kit mechanism:** the disposition declarations D3 moves and the manifest-set
prune D2 applies — both are properties of a gate or of canon-kit's own finder,
carrying no consumer's vocabulary. **Installer mechanism, not a kit literal:**
D1's rows live in `installer/profiles.list`, which is repo-root-governed and
already the hand-authored home for exactly this judgment; no kit learns that a
`prose` profile exists, and no gate names it. **Consumer config, deliberately
left as such:** the corpus. `CANON_KIT_PROSE_SURFACE_GLOBS` and
`CANON_KIT_MANIFEST_FILES` both stay default-empty, and the profile does not set
them, because a kit literal spelling `docs/**.md` would ship one project's prose
layout as everyone's. This is the same ruling this entry has carried since it was
filed — an adapter delivered as optional consumer config, never a kit literal —
now discharged against a specific knob rather than held as a posture.

## What this amendment does not claim

It does not claim the kits govern non-code work. That abstraction program is
separable from an installable profile, as the queue entry says, and nothing here
tests it. What ships is a profile whose adopter installs, receives a battery that
reads their READMEs and their agent file, takes a real red on a real defect, and
can reverse the install — measured at each of those four points.

It also does not widen the governed corpus to `docs/**.md` for anyone. That is
`CANON_KIT_PROSE_SURFACE_GLOBS`, it stays default-empty, and it stays the
adopter's own config edit — a kit literal naming one project's prose layout would
ship that layout as everyone's, which is the provenance seam. D5's prose is where
a prose adopter learns the knob exists.

## Producers and consumers

- **New state: the `prose` profile name.** *Producer* — the two rows in
  `installer/profiles.list`; reachable because `profile_rows` reads that file on
  every install and `profile_names` derives the selectable set from it, with no
  registration step between. *Consumers*, all existing and all reached with no
  edit: `profile_kits` (resolves it to a kit set in payload order), `init.sh`'s
  `KITS[]`, `profile_gates` (the registry `init` writes), `profile_order` (the
  lattice), `checkwright.lock`'s `profile` field, `doctor`'s installed-profile
  report, and `run-smoke.sh`'s per-profile loop. There is no new field on the
  manifest: the profile name rides the field that already exists, which is why D1
  costs no schema change.
- **Changed interface: the manifest set (D2).** *Producer* —
  `spec_manifest_files` in `canon-kit/lib/spec.sh`, enabled by
  `CANON_KIT_SCAN_KIT_ROOTS`, which every consumer has (default `0`) and this repo
  sets to `1`. *Consumers* — the manifest-narration gate family, which must be
  enumerated rather than gestured at, because D2 changes what each one scans:
  `check-md-refs`, `check-docs-cmd`, `check-knob-citation`, `check-manifest-count`,
  `check-manifest-temporal`, `check-tracking-claim`, `check-spec-fence-balance`,
  `check-spec-pointer`, `check-install-claim`, `check-prose-enum`,
  `check-surface-duplication`. Every one of them is a named reader of the change;
  build verifies each rather than assuming the shared finder makes them uniform.
- **Changed state: eleven install dispositions (D3).** *Producer* — the
  `# install:` header line on each gate. *Consumer* — `recipe_install_disposition`
  → `recipe_gates` → `profile_gates` → the registry `init` writes, and
  `check-install-disposition` assertions A and B. **Assertion B is already
  satisfied for all eleven**: canon-kit's `smoke/install.sh` registers 20 of its
  22 gates and omits only `check-surface-duplication`, which stays `on-surface`.
  So the move costs no smoke-roster edit — checked, not assumed.
- **The reader who is not a gate.** An existing adopter's `init` re-run rewrites
  `gates.list`, so D3 adds eleven gates to trees already installed. Named because
  it is the one consumer that is a person: the change reaches them as new reds on
  content they wrote before the gates existed. `gates.list` is claimed before it
  is written, so their edits to it are protected — but the *roster* growing is
  intended behavior, not a claim violation, and the release note owes them the
  sentence.

## Existing sections updated

- `installer/README.md` §Profiles — D5; and the paragraph in §What init seeds
  saying "No disposition varies on the profile today; the argument is the seam"
  stays **true and stays put**, with the falsified-premise finding above recorded
  where it belongs: this amendment's ruling, merged into that section as the
  reason the seam is not exercised.
- `docs/install.md` — D5's fourth profile and the chain aside.
- `canon-kit/SPEC.md` §lib/spec.sh — D2; the section's own directive already
  describes the manifest set as "canonical specs (kit-root pruned per
  `CANON_KIT_SCAN_KIT_ROOTS`) plus README.md at any depth", a sentence whose
  parenthetical currently scopes to the first clause only. D2 makes the prune
  govern both, so the sentence is rewritten rather than extended.
- `canon-kit/SPEC.md` §Layout and configuration — the `CANON_KIT_SCAN_KIT_ROOTS`
  knob entry states the prune's rationale ("a kit's `SPEC.md`/`SPEC-*.md` is a
  dependency's documentation"); D2 widens what that rationale governs.
- `installer/README.md` §The consumer smoke — D4's arm joins the ordered
  post-conditions that section states.
- `installer/profiles.list` — D1's rows and criterion; the file's own header
  already anticipates this ("A fourth profile is admitted exactly when it fits;
  there is no count to raise here"), so nothing there needs relaxing.
- **Not updated, checked rather than assumed:** `installer/consumer-smoke/run-smoke.sh`'s
  lattice assertions, which are set-based and admit the fourth profile unchanged;
  `check-install-disposition`, whose closed vocabulary is unwidened; and
  `canon-kit/smoke/install.sh`, whose roster already covers D3.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred). `check-docs-link-convention`'s hostile-first-contact finding is
      already in the gap inbox, filed 2026-08-09 at this amendment's authoring.
- [ ] **The acceptance oracle is the consumer smoke, run for every profile** —
      not this tree's battery. D2 is a no-op here and D3's effect is a property of
      the tree `init` makes, so a green battery in this repo proves neither. The
      unmeasured case build must not assume: `check-md-refs` on a bare `full`
      consumer, which no run has yet covered.
