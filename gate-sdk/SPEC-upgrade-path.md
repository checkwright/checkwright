# SPEC amendment: upgrade-path

Owning component: gate-sdk (the smoke's home). Cross-component: the phase-B
skill lands in lifecycle-kit and the suite registration in this repo's
`scripts/evidence-config.sh` — the align-stage audit trigger applies. The unit
proves docs/install.md §The upgrade contract mechanically (the smoke) and
ships its judgment half as a skill (the disposition ritual).

## What changes

### gate-sdk/bin/upgrade-smoke.sh (new)

Bare bash on the consumer-smoke mechanics (gate-sdk/SPEC.md §Consumer smoke),
so a harness-less consumer keeps the upgrade path. Procedure:

1. Resolve the source repo, FROM, and TO (knobs below). Vendor every
   `gate_kit_roots` kit at FROM into a scratch consumer (`git archive` from
   the source repo at that ref), run each kit's `smoke/install.sh`, commit
   the baseline, run the battery — a red FROM baseline is exit 2 (the tag
   itself is broken, not an upgrade finding).
2. **Phase A at TO**: replace the vendored kit directories wholesale at TO
   and regenerate the generated artifacts (pre-commit hook, graph), exactly
   the contract's consumer steps. Assert **determinism**: the scratch
   consumer's `git status` shows changes only under kit roots and the
   regenerated artifacts.
3. Run the battery. Assert the **red set ⊆ TO's tightened-gates
   declaration**, resolved via the note shape docs/install.md owns (the
   `docs/posts/` entry at TO whose front-matter `release:` names TO's
   version). When TO is unreleased — the HEAD default — and no pending note
   exists, the red set must be empty. A red gate absent from the declaration,
   or a missing/unparseable note while reds exist, is a fail (fail-closed:
   never a skip).

Knobs, config-via-env in the gate-sdk `<KIT>_<KNOB>` shape, defaults this
repo's layout:

- `GATE_SDK_UPGRADE_REPO` — path to the kit-source git repository (default:
  the enclosing repo's toplevel). A consumer points it at their checkwright
  clone; the smoke never touches the network.
- `GATE_SDK_UPGRADE_FROM` — the N ref (default: the source repo's newest
  `v*` tag; no tag resolvable is exit 2, not a skip).
- `GATE_SDK_UPGRADE_TO` — the N+1 ref (default: `HEAD`).
- Scratch base: the existing `GATE_SDK_TMP_DIR` knob; the scratch consumer
  is `mktemp`-created and trap-removed like the demo's.

The HEAD default **supersedes the queue note "buildable once a second tag
exists"**: with TO=HEAD the smoke is a standing pre-release assertion — every
run proves the working tree upgrades cleanly from the last tag — and a
tag→tag run is the same tool with both knobs set. One tag (`v0.1.0`, cut by
launch-comms first — hence the entry's `[blocked-by:]`) suffices.

New-gate delivery, the causal gap this design closes: `smoke/install.sh`
writes the consumer's `gates.list` fresh and phase A never re-runs it, so a
gate landing in N+1 reaches an upgrading consumer **only** through the
release note's tightened-gates declaration (new gates are "tightened from
zero" and listed there; registering them is a phase-B step the skill
narrates). The smoke's scratch consumer registers at FROM, so a new N+1 gate
does not run in step 3 — the declaration bullet is its delivery channel, the
skill its executor; the smoke asserts the declaration's *sufficiency* for
gates that do run, not the consumer's uptake.

### The `upgrade` validate suite (this repo's registration)

`scripts/evidence-config.sh`: `upgrade` joins the hand-listed
`EVIDENCE_KIT_SUITES` tail with
`EVIDENCE_KIT_RUN_upgrade=bash gate-sdk/bin/upgrade-smoke.sh` — every
validate stage re-proves next-release upgradability, the same pattern as the
`demo` suite. Cost note: the smoke runs the battery twice in scratch
(~2× the demo's single run); accepted as validate-stage cost, not
pre-commit. Like the demo, the suite run is the tool's own evidence — a bin
tool, not a gate, so no `good/`+`bad/` fixture pair is owed; smoke-testing it
is the suite's job.

### lifecycle-kit/templates/skills/upgrade.md (new)

The phase-B disposition ritual, a non-stage skill beside `release-sweep.md`
(same precedent: no `enter-stage.sh`, no WORKFLOW-STATE stamp). Ritual:

1. Run phase A (or the upgrade smoke first, where the consumer has it).
2. Read the target release note's tightened-gates and renamed-knobs
   sections; register any new gates the declaration names (the delivery
   channel above); apply knob renames to the consumer's config.
3. Run the full battery; disposition every red — **fix-the-tree** or
   **exempt-with-cause** (a consumer shadow/`gates.list` omission with the
   cause stamped), never weaken the gate.
4. **Semantic-residual judgment step** (the 2026-07-12 three-way split, the
   two mechanical thirds staying with the gates): slot-set drift reds at
   `check-skill-binding`, verbatim absorption reds at
   `check-shim-restatement`; the residual — a slot fill the new template now
   *means* to cover but the consumer worded differently — passes both and is
   un-gateable, so this step surfaces the changed template slots beside the
   consumer's shim slot fills and judges redundancy (the
   ungateable-class-audit-cadence pattern, the upgrade event as the cadence).

Slots for the consumer: their evidence/disposition stamp path, their
gates.list location if non-default. This repo binds no command for it — the
repo is the kit source, never a vendored consumer; the template ships for
consumers and the smoke exercises the mechanics.

### Ruled out: the thin installer CLI

Demand-gated, not built: the git-native vendor copy is documented
(docs/install.md), the smoke exercises it end-to-end, and the registries
stay namespace reservations, never a dependency channel. Reopen as a new
deferred entry on the first consumer report that the copy step itself is the
adoption blocker.

## Producers and consumers

- **The smoke's verdict (exit code + assertion output)**: producer —
  `evidence-kit/bin/run-validate.sh` running the `upgrade` suite each
  validate stage (this repo), or a consumer invoking the script pre-upgrade.
  Consumer — the validate session's evidence file on this repo; the
  operator's go/no-go on a consumer tree.
- **`GATE_SDK_UPGRADE_*` knobs**: producer — the invoking environment
  (defaults emitted by the script itself, so the zero-config run works on
  this repo). Reader — the smoke's resolve step (step 1), each knob read
  exactly there.
- **Tightened-gates declaration**: producer — the release session
  (SPEC-launch-comms.md, `RELEASING.md` step 2). Consumers — the smoke's
  step 3 (allowed-red-set parse) and the upgrade skill's step 2 (the
  consumer checklist). Grammar owner: docs/install.md §The upgrade contract.
- **`upgrade.md` template**: producer — this unit, lifecycle-kit. Consumer —
  a vendored consumer's `.claude/commands/` binding (their shim);
  `check-skill-binding` binds its slot set, `check-shim-restatement` couples
  it into the dedup corpus — both couplings exist today for the skills
  roster and pick the new template up by directory, no gate edit.

## Existing sections updated

- **docs/install.md §The upgrade contract** — cites the smoke as the
  contract's executable proof and the upgrade skill as the phase-B ritual
  (one sentence each; mechanism stays in the SPECs).
- **gate-sdk/SPEC.md** — §Consumer smoke gains the upgrade-smoke sibling;
  a `### upgrade-smoke` subsection lands under §Per-component contracts with
  the knob roster and defaults (the section this amendment merges into).
- **lifecycle-kit/SPEC.md** — the templates/skills roster gains
  `upgrade.md` beside `release-sweep.md`.
- **evidence-kit/SPEC.md** — only if it hand-lists this repo's suite names
  anywhere (verify at build; the config comment says the tail is
  hand-listed in scripts/, not the kit).
- The queue's deferred entry body (the 2026-07-12 shim-upgrade split, the
  2026-07-10 phase-B ruling, the release-sweep extension) is absorbed here;
  the trimmed active entry carries only the component list.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged specs read as
      one coherent document a reader who never saw the amendment can use
      alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
