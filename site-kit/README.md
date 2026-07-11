# site-kit

Deployment-truth governance for a docs site served from the repo (GitHub Pages
and the like): one gate that holds the tree honest about the site's host, and
one template that watches the live deployment the tree cannot see.

The gate — `check-docs-cname-parity` — makes the CNAME file the single gated
source of truth for the docs host: no tracked file may cite a *configured host
alias* other than that host in a `://` URL, so a domain rename is a one-line
edit to the CNAME that the gate then propagates. The alias set is consumer
config (`SITE_KIT_ALIASES`), never a kit literal — a kit that shipped a
project's host names would publish them.

The template — `templates/site-health.yml` — is a scheduled probe of the live
site (apex/www/http HTTPS, redirects, certificate expiry). It verifies a
*deployment*, not a tree, so it ships as a workflow a consumer copies, never a
gate: a monitor reds on causes no commit produced. See [SPEC.md](SPEC.md#the-monitor-boundary)
for why that boundary is load-bearing.

## Install

Vendor the kit beside [gate-sdk](../gate-sdk/) (required), then:

1. Register the gate — add to your `gates.list`:

   ```
   check-docs-cname-parity
   ```

   Regenerate the hook + graph artifacts: `bash gate-sdk/bin/gen-pre-commit.sh --write`.

2. Establish the host source of truth — a CNAME file holding exactly one host
   line, at the path `SITE_KIT_CNAME` names (site-kit/SPEC.md owns its fallback).

3. Declare your aliases — copy a `site-config.sh` into your gates dir naming
   `SITE_KIT_ALIASES` (every reachable host that is *not* the cited docs host:
   www subdomains, redirect domains, the pre-CNAME Pages host). With the array
   unset the gate holds on defaults and finds nothing.

4. Optional live monitor — copy `templates/site-health.yml` verbatim into
   `.github/workflows/`, set its `ALT_DOMAIN` (or drop the alternate-host probe),
   and it opens/updates/closes a `site-health` issue on its own schedule.

## Test

```bash
bash gate-sdk/bin/run-gate-tests.sh site-kit/gate-tests site-kit/checks
```
