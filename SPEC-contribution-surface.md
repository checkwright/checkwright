# SPEC amendment: contribution-surface

## What changes

The public contribution surface — repo-meta, no owning kit. Ruling
(scope 2026-07-09): the license is not a contribution lever — Apache-2.0
stays (copyleft compels service-source disclosure, not upstream PRs, and
would deter the risk-averse adopters any standardization outcome runs
through); provenance is handled by DCO sign-off (`Signed-off-by`, checked
in review — no CLA, no bot dependency at this scale).

Flood defense is the design center (the observed failure mode: a
coding-agent-era repo drowning in thousands of low-triage-value issues
and PRs). Checkwright's native answer: **the fixture is the unit of
contribution** — machine-verifiable by CI, near-zero triage cost.

New files:

- `CONTRIBUTING.md` —
  - bug report = failing fixture pair: a gate missing a violation
    arrives as a `bad/` case the gate wrongly passes; a false positive
    arrives as a `good/` case it wrongly flags — submitted as a PR the
    CI backstop (gate-sdk/SPEC.md §templates/gates-workflow.yml) verifies mechanically;
  - PRs must be battery-green in CI; a PR that weakens a gate instead of
    fixing the tree is the defect (`check-gate-tamper`'s doctrine, cited
    not restated);
  - DCO sign-off required on every commit;
  - the honest bandwidth statement: solo maintainer, no response SLA,
    stale automation may close inactive threads;
  - larger work: read the queue conventions (cite queue-kit/spec-kit
    READMEs), open a Discussion before building.
- `.github/ISSUE_TEMPLATE/gate-defect.yml` — mandatory fields: gate
  name, fixture path or exact gate output, expected vs actual verdict;
  an issue that cannot name a gate and a reproducing input is not an
  issue.
- `.github/ISSUE_TEMPLATE/config.yml` — `blank_issues_enabled: false`;
  contact link routing questions/proposals to GitHub Discussions, where
  volume is harmless.
- `.github/pull_request_template.md` — checklist: battery green,
  fixture pair included for gate changes, DCO sign-off present.

Anti-drift wiring: `CONTRIBUTING.md` joins the governed doc set via the
`SPEC_KIT_MANIFEST_FILES` wiring SPEC-docs-site.md establishes — one more
glob, no second knob (link resolution on land, command/knob resolution
when `check-docs-cmd` follows); `CONTRIBUTING.md` and the three `.github/`
files register in `scripts/core-files.list` so silent deletion is red;
`CONTRIBUTING.md` joins `scripts/root-allowlist.list` (`.github` is already
allowlisted — the CI backstop, gate-sdk/SPEC.md §templates/gates-workflow.yml,
put it there).

## Producers and consumers

- Producer of the surface: this iteration's build session; thereafter
  GitHub renders the templates on every new issue/PR (enabling config is
  the files' presence — nothing else to deploy).
- Consumer: contributors (the templates shape what arrives); the
  maintainer (triage reads the mandatory fields); the CI backstop (PR
  verification); reviewers (DCO line check).
- Inputs read: none at runtime — static files; the PR template's
  checklist couples to the CI workflow by name.

## Existing sections updated

- README.md gains a contributing pointer line (link to CONTRIBUTING.md)
  at merge.
- CLAUDE.md Housekeeping notes the `.github/` templates as governed
  repo-meta (tracked, core-files-pinned) at merge.
- `scripts/core-files.list` and `scripts/root-allowlist.list` as above.

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
      deferred).
