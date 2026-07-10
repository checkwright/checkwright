# SPEC amendment: site-health-monitor

A scheduled probe of the live docs site — explicitly **not a gate**
(queue: `site-health-monitor`; ruling on record in the ops runbook and
the queue entry): it verifies a deployment, not a tree, so it fails on
causes no commit produced (DNS, a Pages incident, cert renewal) and
breaks both the low-false-positive gate contract and the CI backstop's
"checkout + bash only" hermeticity (gate-sdk/SPEC.md §Enforcement
tiers). It therefore lives beside `gates.yml`, never inside it, and its
failure surfaces as an issue, never a red merge. Repo-root governed
(`.github/` is repo-meta); templatizing it into gate-sdk waits for a
second consumer — out of scope.

## What changes

New workflow `.github/workflows/site-health.yml`: daily cron plus
`workflow_dispatch`, one job, curl-only probes. The apex host is read
from `docs/CNAME` at run time (the single source of truth
`docs-cname-parity` gates); the `.com` alternate domain is a workflow
literal (consumer surface, public hostnames only — nothing from the
local ops runbook lands in the tree). Asserted contract, from the
standing ruling:

1. apex answers `200` over HTTPS with a valid certificate;
2. `www` `301`s to the apex;
3. `http://` redirects to `https://` (proving Enforce-HTTPS has not
   silently flipped);
4. `checkwright.com` `301`s to the apex with the request path kept;
5. **certificate expiry ≥ 14 days** — the real payload: Pages renews
   ~30 days out, so a cert inside 14 days means renewal has silently
   stalled, the failure nothing else surfaces before users see it.

Failure handling: the job opens — or updates, never duplicates — a
`site-health`-labeled issue naming the failed probe(s) and output;
a green run closes any open `site-health` issue (recovery is
self-clearing). The workflow declares `permissions: issues: write`
explicitly (the repo's `GITHUB_TOKEN` default is read-only, a setting
this repo keeps). The job itself may go red on a cron run — that reds no
merge and is the operator's second signal. **No README badge**, by the
standing ruling: the gates badge claims the code; a badge claiming
infrastructure we do not own would red the landing page on a resolver
hiccup, undercutting the pitch that page makes.

## Producers and consumers

- Producer: GitHub Actions cron (daily) and manual dispatch — a deployed
  trigger from day one.
- Consumers: the `site-health` issue (operator-facing signal, each probe
  result read when the issue is written); the Actions run log for the
  probe transcript.
- `docs/CNAME` gains a second reader (the workflow's apex resolution) —
  same read as Pages itself, so a cutover updates the monitor for free.

## Existing sections updated

- None in kit specs. The workflow's header comment states the
  monitoring-not-gating boundary and cites gate-sdk/SPEC.md §Enforcement
  tiers, the same pattern as `gates.yml`'s header.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — the boundary ruling lives in the
      workflow header; no canonical kit spec changes.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root for this unit (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks.
- [ ] **Behavior verified** — a `workflow_dispatch` run green against the live
      site; the issue path exercised once (forced-fail probe or a dry-run
      flag), then closed by the next green run.
