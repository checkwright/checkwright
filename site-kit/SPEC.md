# site-kit — deployment-truth governance for a repo-served docs site

A docs site served from the repository (GitHub Pages and equivalents) has two
truths a gate battery must keep straight: what the *tree* says about the site's
host, and whether the *live deployment* is actually up. site-kit gates the
tree-side truths — the cited host, and whether each page renders faithfully
through the platform's own parser — and ships a template for the deployment
liveness, holding the boundary between gate and monitor so neither leaks into
the other.

## The monitor boundary

A gate verifies the tree: it is deterministic, hermetic, and reds only on a
cause a commit produced, which is what lets it block a merge without false
positives. A live-site probe is none of those — it reds on DNS, a Pages
incident, or a stalled certificate renewal, none of which any commit caused.
Enforcing deployment truth through a pre-commit or CI gate would break both the
low-false-positive contract and the CI backstop's checkout-plus-bash
hermeticity (gate-sdk/SPEC.md §Enforcement tiers). So the deployment probe is
*monitoring*, not a gate: it ships as `templates/site-health.yml`, a scheduled
workflow a consumer copies, and signals through an issue and a red run of its
own, never a blocked merge. The tree-honesty half — that the repo never *cites*
a stale host — is a real gate, because that is a property of the tree.

## Layout and configuration

The kit is vendored beside gate-sdk (conventionally at `site-kit/`); its gate
is registered in the consumer's `gates.list` by name and resolves through
gate-sdk's multi-kit path. `check-docs-cname-parity` registers where a docs
site with a gated host exists; a consumer without one simply omits it.

Config follows the kit pattern: copy a `site-config.sh` into the gates dir (or
point `SITE_KIT_CONFIG_FILE` elsewhere) and override any knob; defaults fill
what the consumer left unset, and the loader exits 2 on a config path it was
told to load but cannot find. Knobs:

- `SITE_KIT_CNAME` — the CNAME file holding the one authoritative host line,
  default `docs/CNAME`.
- `SITE_KIT_ALIASES` — array, default empty: every reachable host that is *not*
  the cited docs host and must therefore never appear in a `://` URL in the
  tree. Rule content by nature, so it is consumer config — a kit literal
  carrying it would publish a project's host names across the provenance seam.
- `SITE_KIT_SCAN_ROOT` — the `git ls-files` root the gate walks, default `.`.
- `SITE_KIT_EXEMPT_PATHS` — array of path globs skipped during the scan,
  default `("*/gate-tests/*" "*docs/posts/*")`: fixture trees deliberately cite
  aliases, and dated posts are immutable published artifacts.
- `SITE_KIT_DOCS_DIR` — the docs-site root `check-docs-render-fidelity` walks
  for tracked markdown pages, default `docs`.
- `SITE_KIT_RENDERER` — array, the stdin→stdout GFM-to-HTML command
  `check-docs-render-fidelity` renders each page through, default the kramdown
  CLI invocation with GFM input — `ruby -e '…Kramdown::Document…input: "GFM"…'`,
  the parser GitHub Pages pins. A consumer whose Pages stack differs points this
  at its own renderer; an unresolvable one fails the gate closed.
- `SITE_KIT_CONFIG_FILE` — the loader override; when set it must resolve, else
  the gate exits 2 rather than silently run on defaults.

## check-docs-cname-parity

Invariant: no tracked file cites a `SITE_KIT_ALIASES` host in a `://` URL,
where the authoritative host `H` is read from `SITE_KIT_CNAME`. The CNAME file
must hold exactly one non-blank host line (else exit 2); `H` itself is exempt
even when listed among the aliases, so a canonical apex that doubles as a
redirect target is cited freely. The scan enumerates tracked files under
`SITE_KIT_SCAN_ROOT` via `git ls-files`, drops the gate-sdk prune set and every
`SITE_KIT_EXEMPT_PATHS` glob, and greps the survivors for `://<host>`; a host
that is a configured alias other than `H` is a finding. A rename is thus a
one-line edit to the CNAME file that the gate re-propagates — the host lives in
one gated place, and drift anywhere else is caught. The scan reads tracked
content only, so an untracked local file is never a source; a `git ls-files`
error is fail-closed (exit 2). The positional form
`check-docs-cname-parity.sh [scan-root] [cname-file] [config-file]` lets a
fixture point all three at a synthetic tree without touching consumer config.

## check-docs-render-fidelity

Invariant: every tracked markdown page under `SITE_KIT_DOCS_DIR`, rendered
through the pinned Pages parser, leaks no code-fence marker into rendered text
and promotes no code-fenced line into a heading. GitHub Pages renders through
kramdown's GFM parser, which diverges from github.com's cmark: consecutive
fenced blocks inside one list item corrupt the page — the second fence prints
literally and a `#`-leading skeleton line becomes a heading — so a tree that
reads green on github.com can ship a garbled Pages site with no gate in the
path. This gate is the faithful-artifact-verification class mechanized for that
artifact: it renders the real output and asserts the observed leakage class,
rather than trusting the source. A second instance of the same class: an inline
code span that wraps across a line break whose continuation begins with a
block-level or unknown HTML tag, which kramdown emits as a raw-HTML block that
swallows the rest of the page — the upstream root cause is `gettalong/kramdown#843`,
and this gate catches it as a leaked fence regardless of the fix's timing.

The scan enumerates tracked `*.md` files under `SITE_KIT_DOCS_DIR` via
`git ls-files` (every underscore-prefixed directory segment excluded — those are
Jekyll internals, not published pages), strips Jekyll front matter so it renders
exactly the body kramdown sees, and asserts two properties per page:

1. **No fence leakage** — the rendered HTML's text content (outside `<pre>` and
   inline `<code>`) carries no literal backtick fence run. A leaked fence is the
   signature of a block the parser failed to close, regardless of which
   construct confused it.
2. **No heading leakage** — the count of rendered heading elements never exceeds
   the count of source heading lines the gate's own fence-aware scan (cmark
   rules: ATX and setext, both skipped inside a fenced or `~`-fenced block)
   places outside any code context. A surplus rendered heading is a `#` line
   promoted out of a broken code block.

The renderer is the gate's oracle. Before scanning, the gate probes it on a
one-line document; an unresolvable or non-producing renderer exits 2 with a help
line naming the dependency (ruby plus the kramdown-parser-gfm gem, or a
`SITE_KIT_RENDERER` override) — a gate that cannot run its oracle refuses, never
a false clean. That dependency joins the *consumer's* toolchain only when the
consumer registers this gate; it stays outside env-probe's probe-set floor, and
`docs/install.md`'s Requirements prose states the tier (SPEC-os-support.md owns
that page's ruling). A consumer with no published docs site simply omits the
gate by the registry-not-array convention and never installs the dependency.

Honest limit: this is not a full render-diff between the two parsers. It
mechanizes the observed leakage class — fences and headings — and stays silent
on divergences that corrupt neither. The positional form
`check-docs-render-fidelity.sh [docs-dir] [config-file]` lets a fixture point
the docs dir and renderer at a synthetic tree without touching consumer config.
`precommit` tier, coupling the docs tree.

## lib/site.sh

The sourced config loader: it loads `SITE_KIT_CONFIG_FILE` (or the gates-dir
`site-config.sh` when that env is unset), then fills every knob's default, so a
gate and a fixture read one resolved configuration. It carries no gate logic —
structure stays in the check, values in config, defaults here.

## templates/site-health.yml

The scheduled live-site probe, copied verbatim into a consumer's
`.github/workflows/`. It reads the apex host from the CNAME file (the same
source the gate trusts), then checks: the apex answers 200 over HTTPS, `www`
and `http` redirect to the canonical origin, an optional `ALT_DOMAIN` redirect
keeps its path, and the certificate is more than a fortnight from expiry. A
failure opens or updates a single `site-health` issue and reds the run;
recovery closes it. The `ALT_DOMAIN` value is a bare hostname, never a `://`
literal, so it does not itself trip the parity gate. A `# enforce:` marker rides
the template so that, once copied, an enforcement map projects it as a monitor.

The template is starter-template-conformant: the kit's `smoke/install.sh`
installs it verbatim as governed surface, so a regression that made it red any
vendored kit's gate would surface in the consumer smoke rather than at a
consumer.

## Out of scope

The kit does not resolve DNS, provision certificates, or configure the host
platform — those are the deployment's concerns, surfaced by the monitor, not
governed by a gate. It does not gate the *content* of the docs site (links,
commands, prose): that is canon-kit's charge over the governed doc set —
`check-docs-render-fidelity` gates how a page *renders*, never what it says. And
it holds no opinion on which host a project uses — only that the tree cites one,
and that the CNAME file names it.
