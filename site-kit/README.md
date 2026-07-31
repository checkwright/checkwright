# site-kit

Deployment-truth governance for a docs site served from the repo (GitHub Pages
and the like): gates that hold the tree honest — about the site's host, and
about whether each page renders faithfully — and a template that watches the
live deployment the tree cannot see.

`check-docs-cname-parity` makes the CNAME file the single gated source of truth
for the docs host: no tracked file may cite a *configured host alias* other than
that host in a `://` URL, so a domain rename is a one-line edit to the CNAME
that the gate then propagates. The alias set is consumer config
(`SITE_KIT_ALIASES`), never a kit literal — a kit that shipped a project's host
names would publish them.

`check-docs-render-fidelity` renders every tracked docs page through the pinned
Pages parser (kramdown-GFM) and asserts no fence or heading leakage — the
divergence class where a source-green tree ships a garbled site because
GitHub Pages' parser is not github.com's. It fails closed when the renderer is
absent; the dependency joins a consumer's toolchain only when the gate is
registered.

The template — `templates/site-health.yml` — is a scheduled probe of the live
site (apex/www/http HTTPS, redirects, certificate expiry, and release-body note
pointers). It verifies a *deployment*, not a tree, so it ships as a workflow a
consumer copies, never a gate: the line is where the asserted object lives, and
none of what it asserts is in any checkout. See [SPEC.md](SPEC.md#the-monitor-boundary)
for why that boundary is load-bearing.

## Install

Vendor the kit beside [gate-sdk](../gate-sdk/) (required), then:

1. Register the gates — add to your `gates.list`:

   <!-- gate-roster:begin -->
   ```
   check-docs-cname-parity
   check-docs-render-fidelity
   ```
   <!-- gate-roster:end -->

   Regenerate the hook + graph artifacts: `bash gate-sdk/bin/gen-pre-commit.sh --write`.
   `check-docs-render-fidelity` needs ruby plus the kramdown-parser-gfm gem (the
   Pages parser); a consumer without a published docs site simply omits it.

2. Establish the host source of truth — a CNAME file holding exactly one host
   line, at the path `SITE_KIT_CNAME` names (site-kit/SPEC.md owns its fallback).

3. Declare your aliases — copy a `site-config.sh` into your gates dir naming
   `SITE_KIT_ALIASES` (every reachable host that is *not* the cited docs host:
   www subdomains, redirect domains, the pre-CNAME Pages host). With the array
   unset the gate holds on defaults and finds nothing.

4. Optional live monitor — copy `templates/site-health.yml` verbatim into
   `.github/workflows/`, then set two groups of step env or delete the arm each
   belongs to: `ALT_DOMAIN` (drop the alternate-host probe if you serve no
   redirect alias), and `RELEASE_NOTE_GLOB` / `RELEASE_NOTE_TAG_KEY` /
   `RELEASE_NOTE_URL_PATH` (drop the release-body arm if you publish no release
   notes). It opens/updates/closes a `site-health` issue on its own schedule.
   The release-body arm needs the template's `contents: read` permission — it is
   an allowlist, not an addition — and `RELEASE_NOTE_URL_PATH` is the value worth
   checking twice: site-kit/SPEC.md §templates/site-health.yml names the trap.

## Test

```bash
bash gate-sdk/bin/run-gate-tests.sh site-kit/gate-tests site-kit/checks
```
