---
title: site-kit
nav_parent: kits
nav_child_order: 10
---

# site-kit

Deployment-truth governance for a docs site served from the repository itself
(GitHub Pages and equivalents). It keeps three truths apart: what the tree says
about the site's host, whether the committed pages render faithfully through
the parser that will serve them, and whether the live deployment is actually
up.

The host gate — `check-docs-cname-parity` — makes the CNAME file the single
gated source of truth for the docs host. No tracked file may cite a configured
host alias other than that host in a URL, so a domain rename is a one-line edit
to the CNAME that the gate then propagates. The alias set is consumer config,
never a kit literal: a kit that shipped a project's host names would publish
them.

The render gate — `check-docs-render-fidelity` — re-renders every tracked docs
page through the pinned Pages parser and asserts the observed leakage classes
never reach the published artifact: it verifies the real rendered output
rather than trusting that a page reading green on github.com ships intact.
The class list and the gate's honest limit live in the kit's
[`SPEC.md`](SPEC.md#check-docs-render-fidelity).

The template — `site-health.yml` — is a scheduled probe of the live site
(HTTPS, redirects, certificate expiry). It verifies a deployment, not a tree, so
it ships as a workflow a consumer copies rather than a gate: a monitor reds on
causes no commit produced, and blocking a merge on one would be a false positive
waiting to happen.

## Install

Vendor the `site-kit/` directory into your repo, register `check-docs-cname-parity`
(and, if a Pages-style stack renders your site, `check-docs-render-fidelity` —
it needs the pinned parser on the machine) in `gates.list`, name your host in
the CNAME file, and declare your aliases in the kit's external config.

## Quick start

```bash
bash gate-sdk/bin/run-gate-tests.sh site-kit/gate-tests site-kit/checks
```

## Contracts

The gate invariants, the monitor boundary, and every knob are defined in the
kit's
[`SPEC.md`](SPEC.md#check-docs-cname-parity);
its [`README.md`](README.md#site-kit)
lists the mechanism. Back to the
[kit map](../index.md#the-kits).
