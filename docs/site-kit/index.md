# site-kit

Deployment-truth governance for a docs site served from the repository itself
(GitHub Pages and equivalents). It keeps two truths apart: what the tree says
about the site's host, and whether the live deployment is actually up.

The gate — `check-docs-cname-parity` — makes the CNAME file the single gated
source of truth for the docs host. No tracked file may cite a configured host
alias other than that host in a URL, so a domain rename is a one-line edit to
the CNAME that the gate then propagates. The alias set is consumer config, never
a kit literal: a kit that shipped a project's host names would publish them.

The template — `site-health.yml` — is a scheduled probe of the live site
(HTTPS, redirects, certificate expiry). It verifies a deployment, not a tree, so
it ships as a workflow a consumer copies rather than a gate: a monitor reds on
causes no commit produced, and blocking a merge on one would be a false positive
waiting to happen.

## Install

Vendor the `site-kit/` directory into your repo, register `check-docs-cname-parity`
in `gates.list`, name your host in the CNAME file, and declare your aliases in
the kit's external config.

## Quick start

```bash
bash gate-sdk/bin/run-gate-tests.sh site-kit/gate-tests site-kit/checks
```

## Contracts

The gate invariant, the monitor boundary, and every knob are defined in the
kit's `SPEC.md`; its `README.md` lists the mechanism. Back to the
[kit map](../index.md#the-kits).
