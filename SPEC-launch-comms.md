# SPEC amendment: launch-comms

Governance/release ruling with no owning kit — repo-root placement per
canon-kit/templates/SPEC-amendment.md. The unit is the first release cut
(`v0.1.0`) and the repeatable release procedure behind it. In-repo residue
only: the campaign itself — channels, venues, timing — is operator work,
planned in the operator's local brief, never in tracked files.

## What changes

- **`RELEASING.md`** (repo root, new) — the release runbook, governed
  repo-meta like `CONTRIBUTING.md` (tracked, joins the spec manifest so its
  links and commands resolve). The ordered procedure every release follows:
  1. Run the release-sweep skill (the deprecation disposition walk;
     binding below) — every marker dispositioned before the tag.
  2. Author the release-note post (shape below).
  3. Tag `vX.Y.Z` on the release commit and push the tag.
  4. Create the GitHub Release; its body points at the post's
     checkwright.dev URL — the post is the note's single home, the Release
     a pointer to it.
  5. Verify the README version badge resolves the new tag.
- **`.claude/commands/release-sweep.md`** (new) — this repo's binding of
  lifecycle-kit's `templates/skills/release-sweep.md`. Bindings: the
  inventory command is the `CANON_KIT_DEPRECATION_MARKERS` roster scan
  (the same resolution `kpi-deprecated-surface` uses); the evidence path is
  `.workflow/release-sweep-evidence.txt` (committed, appended one
  disposition block per release); no gate over the stamp file — demand-gated,
  wire one only if a release ever ships with the sweep skipped.
- **Release-note post shape** — a release's note is a dated `docs/posts/`
  entry (immutable, temporal-exempt, link/command-resolved like every post)
  carrying front-matter key `release: vX.Y.Z` and the two sections
  docs/install.md §The upgrade contract names: a tightened-gates section
  (one bullet per gate that got stricter or landed new, lead token the gate
  name) and a renamed-knobs section (old → new per bullet). "None" is a
  valid section body and must be stated, not omitted. The parseable grammar
  is **owned by docs/install.md §The upgrade contract** — this unit extends
  that section with the front-matter key and bullet grammar; this amendment
  only names the producer. For `v0.1.0` both sections are "none — first
  release; the shipped battery is the baseline".
- **README version badge** (per the 2026-07-11 scope ruling, relocated here
  from the queue entry): a release-version badge beside the existing gates
  badge, sourced from the GitHub tag
  (`img.shields.io/github/v/tag/checkwright/checkwright`, linking to the
  releases page) — never from the registry placeholders; `reserve/` is a
  namespace reservation, not a channel, and a registry-sourced badge would
  advertise a dead install path.
- **Launch copy phrasing ruling**: the `v0.1.0` post and any launch-adjacent
  docs copy use the phrasing the shipped orchestration-positioning docs
  reserved — "the verification layer under agent orchestration" — not an
  unqualified "agentic trust layer" (that term reads as security tooling,
  not delivery governance).

## Producers and consumers

- **Release-note post**: producer — the release session, per `RELEASING.md`
  step 2. Consumers — (a) `gate-sdk/bin/upgrade-smoke.sh`
  (SPEC-upgrade-path.md) resolves the target version's post by the
  `release:` front-matter key and reads the tightened-gates bullets' lead
  tokens as the allowed red set; (b) an upgrading consumer walks the same
  two sections as their phase-B checklist (docs/install.md). The `release:`
  key's reader is (a)'s note-resolution step; Jekyll ignores the unknown
  key, so the docs site renders the post unchanged.
- **`.workflow/release-sweep-evidence.txt`**: producer — the release-sweep
  session (step 2 of the template's ritual). No mechanical reader by design
  (the template's stated honest limit: operator evidence riding the release
  commit); its human readers are the next release-sweep session (prior
  dispositions) and the audit trail.
- **Version badge**: producer — one README edit this unit; thereafter
  shields.io queries the GitHub tag list, so each release updates it with no
  edit (step 5 is a verification, not a write).
- **Tag `v0.1.0`**: producer — release session, step 3. Consumers —
  `GATE_SDK_UPGRADE_FROM`'s newest-tag default (SPEC-upgrade-path.md), the
  badge, the GitHub Release. Push mechanics: the credential-helper push form
  in the local ops runbook (SSH has no key in the agent sandbox).

## Existing sections updated

- **docs/install.md §The upgrade contract** — gains the note-shape grammar
  (front-matter `release:` key, section names, bullet lead-token grammar);
  currently it names the sections but no machine-parseable shape.
- **CLAUDE.md §Housekeeping** — `RELEASING.md` joins the governed repo-meta
  line beside `CONTRIBUTING.md` (one clause; the runbook itself is the
  load-triggered tier).
- The queue's deferred entry body (badge ruling, prerequisite chain) is
  absorbed here; the trimmed active entry carries only the residue list.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical home (RELEASING.md, docs/install.md, CLAUDE.md), not
      appended; each reads whole without this amendment.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
