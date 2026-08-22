# site-kit — deployment-truth governance for a repo-served docs site

A docs site served from the repository (GitHub Pages and equivalents) has two
truths a gate battery must keep straight: what the *tree* says about the site's
host, and whether the *live deployment* is actually up. site-kit gates the
tree-side truths — the cited host, and whether each page renders faithfully
through the platform's own parser — and ships a template for the deployment
liveness, holding the boundary between gate and monitor so neither leaks into
the other.

## The monitor boundary

**The line is where the asserted object lives.** A gate asserts over the tree: a
checkout plus bash is all it needs, which is what makes it deterministic,
hermetic, and safe to block a merge with. Deployment truth is in no checkout —
the live host's responses, its certificate, and the bodies of the Releases it
publishes are state no commit contains — so no gate can assert it, and enforcing
it through a pre-commit or CI gate would break both the low-false-positive
contract and the CI backstop's checkout-plus-bash hermeticity
(gate-sdk/SPEC.md §Enforcement tiers).

Object location governs rather than *whether a commit caused the failure*, and
the difference is load-bearing. Some host-side failures are caused by nothing any
commit did — DNS, a Pages incident, a stalled certificate renewal. Others are
caused by a session skipping a step, which is commit-shaped in spirit and still
ungateable, because the artifact it damaged sits on the host rather than in the
tree. A criterion built on cause admits the first kind and stumbles on the
second; a criterion built on where the object lives covers both, and stays true
of every probe arm instead of only the arms whose causes it happens to list.

So the deployment probe is *monitoring*, not a gate: it ships as
`templates/site-health.yml`, a scheduled workflow a consumer copies, and signals
through an issue and a red run of its own, never a blocked merge. The
tree-honesty half — that the repo never *cites* a stale host — is a real gate,
because that is a property of the tree.

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
- `SITE_KIT_RENDERER` — array, the stdin→stdout **single-document** GFM-to-HTML
  command, default the kramdown CLI invocation with GFM input —
  `ruby -e '…Kramdown::Document…input: "GFM"…'`, the parser GitHub Pages pins.
  It is the contract `check-docs-render-fidelity` renders each page through when
  `SITE_KIT_RENDERER_BATCH` is empty: one of the gate's two renderer contracts,
  and the fallback of the pair. A consumer whose Pages stack differs points this
  at its own renderer; an unresolvable one fails the gate closed.
  **Its value is also a port blocker, and repointing it moves one.** The first
  element of whatever this knob holds is an external program
  `check-docs-render-fidelity` requires, which is what gate-sdk's criterion 7
  screens for and what `gate-sdk/bin/port-blockers.sh` derives by resolving this
  knob (gate-sdk/SPEC.md §port-blockers). The dependency is spelled nowhere in the
  gate's own source, so it is recorded here, where the knob lives: a consumer who
  points this at a renderer their payload already carries removes that blocker,
  and one who points it at a heavier toolchain deepens it.
- `SITE_KIT_RENDERER_BATCH` — array, **optional**: a command rendering N
  documents over one stream, `NUL`-terminated in both directions and
  count-preserving, the framing specified in §check-docs-render-fidelity. Setting
  it collapses the gate's per-page interpreter restarts into a single process,
  which is where substantially all of that gate's cost lives. Its default is the
  batch form of the same kramdown-with-GFM-input invocation `SITE_KIT_RENDERER`
  defaults to — a `ruby -e` loop splitting stdin on `NUL` and writing one
  `NUL`-terminated HTML document per input — filled **only when
  `SITE_KIT_RENDERER` is itself still at its kit default.** Filling it
  unconditionally would defeat a deliberate pin: a consumer who points
  `SITE_KIT_RENDERER` at the version-locked bundle below, not knowing a second
  knob now exists, would have that pinned oracle replaced by this unpinned one
  and the gate would report clean against a parser build they explicitly
  rejected — a false clean produced by an upgrade. Under the rule they instead
  keep the per-document path at its own cost and its own semantics, and opt in by
  pointing this knob at the batch form of their own pinned renderer.
- `SITE_KIT_CONFIG_FILE` — the loader override; when set it must resolve, else
  the gate exits 2 rather than silently run on defaults.

`templates/site-health.yml` is governed by none of these. Its knobs — `ALT_DOMAIN`
and the three `RELEASE_NOTE_*` below — are **step-level workflow env** set in the
copied file itself, not `SITE_KIT_*` knobs, and `lib/site.sh` never loads them.
The template is copied and edited rather than sourced, so reaching into the gate
config loader would couple a monitor to the gates dir and hard-code a vendored
kit path into a workflow whose whole distribution model is verbatim copy.

## check-docs-cname-parity

`checks/check-docs-cname-parity.gate` (`precommit`, binary-dispatched).
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
`check-docs-cname-parity <scan-root> <cname-file>` lets a fixture point both at
a synthetic tree without touching consumer config.

**Its default scan root makes the whole tracked tree its content corpus, and
that is a port cost worth stating where the rule lives.** With `SITE_KIT_SCAN_ROOT`
unset the walk is every tracked file, so every kit's `checks/*.sh` and `*.gate`
is inside the corpus this gate reads *as content* — gate-sdk's criterion 4
verbatim, reached through the walk rather than through the trigger field. The
`# graph:` couple is the single literal CNAME file, so the derived
substrate-sensitive set never selects it and gate-sdk's conservation assertion
structurally cannot see the hold; it is the first worked instance in that
direction (gate-sdk/SPEC.md §The fourth budget batch). The discharge is the
general one: the fixture pair carries the arm, because `gate-tests` is pruned
from every live-tree walk and no port can change what is inside it. The pair's
good case therefore takes the default arm — no positional, scan root and CNAME
both off the config bridge, a fixture declaration path inside its own corpus —
and the bad case keeps the explicit-root arm.

**There is no third positional selecting a config file, and the ground is a
finding rather than a preference.** Such an argument could only work by exporting
`SITE_KIT_CONFIG_FILE` and re-sourcing `lib/site.sh` mid-run. A
`.gate`-dispatched member cannot do that: the config bridge resolves every declared knob *before* the binary starts
(gate-sdk/SPEC.md §lib/gate.sh), so an argument that selects which config file
the knobs come from arrives a process too late, and a documented flag that
silently changes nothing is worse than none. A fixture supplies its own alias set
the way any consumer does — a `site-config.sh` in the gates dir the loader
already resolves — which is the shape gate-sdk/SPEC.md §The third budget batch
settled for the same cause, and it costs the pair nothing.

## check-docs-render-fidelity

Invariant: every tracked markdown page under `SITE_KIT_DOCS_DIR`, rendered
through the Pages parser, leaks no code-span corruption symptom into
rendered text — neither a literal backtick nor a raw non-HTML-element tag
surviving outside a code context — promotes no code-fenced line into a heading,
and renders no fewer tables than its source GFM table starts. GitHub Pages
renders through kramdown's GFM parser, which diverges from github.com's cmark:
consecutive fenced blocks inside one list item corrupt the page — the second
fence prints literally and a `#`-leading skeleton line becomes a heading — so a
tree that reads green on github.com can ship a garbled Pages site with no gate in
the path. This gate is the faithful-artifact-verification class mechanized for
that artifact: it renders the real output and asserts the observed leakage class,
rather than trusting the source.

The leakage class has more than one root cause, and the assertion keys on the
**shared symptom** rather than on any single cause — one low-false-positive
assertion covering every shape that leaves the same signature. The causes
observed on this tree: (1) a code span that wraps across a line break whose
continuation begins with a block-level or generic XML tag; (2) a single-line code
span whose embedded angle-bracket token (a placeholder such as `<verdict>` or
`<n>`) is consumed as a raw HTML span before the span can close, no line break
involved; (3) a code span nesting escaped backticks alongside such a token. All
three are the same kramdown behavior — parsing blocks before spans, it treats the
angle-bracket token as the start of an HTML block, severing the span so it emits
raw HTML that swallows the rest of the page (`gettalong/kramdown#843`, closed
works-as-designed: a documented, permanent divergence, not a pending fix). A
severed span never forms the closing fence run, so it leaves not a multi-backtick
marker but a **single** stray backtick and a **raw placeholder tag** in the
rendered text — which is exactly the symptom the assertion keys on, so the gate is
a standing defense against the whole class rather than a stopgap against one
backtick-fence shape of it.

**Reach is `SITE_KIT_DOCS_DIR`, and stays there.** A repo-root page has no site
URL and is served by the forge's own CommonMark view, so a kramdown-only
divergence there is not reader-visible and this gate is right not to walk it.
The renderer-to-surface map is a consumer's own documentation concern, not a
widened scan.

The scan enumerates tracked `*.md` files under `SITE_KIT_DOCS_DIR` via
`git ls-files` (every underscore-prefixed directory segment excluded — those are
Jekyll internals, not published pages), strips Jekyll front matter so it renders
exactly the body kramdown sees, and asserts three properties per page:

1. **No span-corruption leakage** — the rendered HTML's text content, taken
   after the `<pre>` blocks and inline `<code>` spans are removed (a legitimate
   backtick renders inside `<code>` and a legitimate embedded tag renders as
   escaped entities inside it, so both are excluded before the check), carries
   neither of two symptoms: (a) **any literal backtick** — a single stray one,
   not only a `` `{3,} `` fence run — and (b) **any raw tag whose element name is
   not a known HTML element**, matched by name so an attribute-bearing legitimate
   tag (`<a href…>`) is excluded and a placeholder token (`<verdict>`, `<n>`,
   `<KIT>`) is not. Content inside an `<svg>` or `<math>` subtree is exempt from
   (b), tracked by open/close depth: those two roots open foreign content whose
   vocabularies (`<circle>`, `<mi>`) are not HTML element names, so scanning them
   by the HTML set would red a page for legitimate inline markup. The exemption is
   scoped to the subtree rather than granted to the names, so a bare `<path>` or
   `<use>` outside any `<svg>` still reds as the placeholder token it is. Keep
   such a subtree on one source line: kramdown severs a foreign root split across
   a line break, emitting its tail as escaped text, and that is a real render
   divergence the gate is right to red.
   Either symptom is the signature of a code span or fenced block
   the parser failed to form, regardless of which construct confused it: a
   backtick that never paired, or a placeholder tag kramdown passed through as a
   raw HTML block. The two are complementary — a swallowed page region whose own
   inline code re-leaks as literal backticks trips (a), while a swallowed
   backtick-free region (pure prose or headings) trips only (b) — so neither
   alone covers the class and the assertion keys on both.
2. **No heading leakage** — the count of rendered heading elements never exceeds
   the count of source heading lines the gate's own fence-aware scan (cmark
   rules: ATX and setext, both skipped inside a fenced or `~`-fenced block)
   places outside any code context. A surplus rendered heading is a `#` line
   promoted out of a broken code block.
3. **No table leakage** — the count of rendered `<table>` elements is never
   *less than* the count of source GFM table starts the same fence-aware scan
   places outside code. A table start is a pipe-carrying row immediately
   followed by a delimiter row (dashes, colons, pipes — the `| --- |` shape).
   The direction is one-sided: rendered may *exceed* source (a raw-HTML
   `<table>` in source is legitimate and renders without a GFM start); only a
   deficit reds. A deficit is a source table that shipped as literal-pipe
   paragraph text — kramdown terminates a table only on a blank line, so a
   table whose last row abuts a following non-blank line collapses into a
   paragraph.

The renderer is the gate's oracle. Before scanning, the gate probes it on a
one-line document; an unresolvable or non-producing renderer exits 2 with a help
line naming the dependency (ruby plus the kramdown-parser-gfm gem, or a
`SITE_KIT_RENDERER` override) — a gate that cannot run its oracle refuses, never
a false clean. That dependency joins the *consumer's* toolchain only when the
consumer registers this gate; it stays outside env-probe's probe-set floor, and
`docs/install.md`'s Requirements prose states the tier. A consumer with no
published docs site simply omits the gate by the registry-not-array convention
and never installs the dependency.

**This member is held on shell, and the hop from its declaration lands here.** The
program the rule requires is the **first element of `SITE_KIT_RENDERER`**, so
gate-sdk's criterion 7 blocks the port, and the blocker is class (i) under that
criterion's hold-worthiness test: the renderer *is* the contract this gate renders
through, so removing it changes the verdict and designing it away means
re-deciding the rule. The declaration therefore carries `# port-until:` naming the
queue entry that owns the blocker (gate-sdk/SPEC.md §The `# graph:` manifest). The
dependency itself, and the fact that a consumer who repoints the knob moves it, are
owned at §Layout and configuration under that knob and are not restated here — the
knob's value is what the blocker is a property of, so that bullet is its right home
and this paragraph is the pointer a reader arriving from the declaration needs.

**The batch stream, and why the count is the fail-closed.** One renderer process
per page is the gate's whole cost — the interpreter restarts, not the rendering —
so where `SITE_KIT_RENDERER_BATCH` is non-empty the gate renders the corpus over
one stream instead. The framing is `NUL`-terminated in both directions: the gate
writes each front-matter-stripped body, `NUL`-terminated, to the command's stdin,
and the command writes each rendered document to stdout in the **same order**,
each `NUL`-terminated. `NUL` is a *terminator* rather than a separator, so N
documents produce exactly N `NUL`s with no trailing-empty ambiguity, and it is
unforgeable by page content for a stronger reason than rarity: bash cannot hold a
`NUL` in a variable, so the gate's own reader has already dropped any `NUL` in
the source before framing happens. A sentinel line would be forgeable — by a docs
page describing this contract, first of all. Length-prefixed framing is equally
sound and loses only on the obligation it puts on a consumer, who would have to
implement exact-byte reads where splitting on a byte is a one-liner in every
language a Pages stack might use.

Two implementation constraints follow, and a natural reading of the contract
breaks both. **Command substitution strips `NUL`**, so the batch output must
never pass through `$(…)`; the gate reads it through a **process substitution**
instead, which also keeps the loop body in the current shell so the findings it
appends to survive. Writer and reader being separate processes
is what makes the exchange deadlock-free. **Process substitution discards the
renderer's exit status**, so the batch path cannot keep the per-page fail-closed
the per-document loop has — and a renderer that dies mid-stream yields fewer
documents than pages. The gate therefore compares documents-read against
pages-enumerated after the loop, which detects renderer death, truncation and
framing error alike, one check standing in for the status the shell threw away.
It exits 2 — a refusal to run the oracle — never 1, which would report a finding
about the docs. Per-document scanning is unchanged: each document read off the
stream goes through the same assertions, and the source-side scans are untouched.

**Probe routing.** The gate probes the oracle it will actually run, and only that
one. With the batch knob empty it probes `SITE_KIT_RENDERER` on a one-line
document exactly as above. With the batch knob set it probes the batch command
instead, on a probe that exercises the framing as well as the parser: two
one-line documents in, exactly two non-empty documents back. Probing the
per-document renderer too would refuse a batch-only consumer over a renderer this
run never invokes. A **failing** batch probe exits 2 and does not fall back: a
set knob is the consumer's deliberate statement about which parser is
authoritative, so quietly rendering through a different one is the same
false-clean class the fill condition in §Layout and configuration closes from the
other direction. A gate that cannot run its configured oracle refuses.

The false-positive floor is the assertion's hard boundary, deliberately set. The
backtick symptom rests on a property of well-formed markdown: every backtick
belongs to a code span, which renders inside `<code>` and is excluded from the
scanned text, so a backtick surviving into paragraph, heading, or list text is a
code span that failed to form — not legitimate prose. Its one residual
legitimate case is documenting the backtick *character* itself; the faithful way
to do that is a doubled-backtick code span, which renders the character inside
`<code>` — excluded from the scan — so the honest form does not trip the gate and
only the fragile bare-backtick form would. The raw-tag symptom rests on
the HTML-element allowlist: kramdown passes an unrecognized `<name>` through
verbatim as raw HTML, while a `<name>` whose element is in the HTML standard is
legitimate markup — so a tag matched by element name against that set is
excluded, and only a token outside it (a placeholder kramdown mistook for an HTML
block) reds. Its residual legitimate case is a page that deliberately embeds a
*non-standard* element (a custom element or web component); a placeholder meant
as literal prose is instead written as a code span (`` `<verdict>` `` renders
`<code>&lt;verdict&gt;</code>`) and never surfaces as a raw tag. Both floors hold
empirically: across the tracked corpus each symptom fires on exactly the
corrupted pages and no clean page. The element set is generic mechanism — the
HTML standard's element list is universal render truth, not consumer rule
content — so it is a kit built-in, not a config seam; a consumer that
legitimately ships non-standard elements in a docs page is the narrow accepted
false positive, and if that demand ever attests, an optional allowlist-extension
knob is the config-via-env answer, deferred until then rather than built against
a case no page presents.

Honest limit: this is not a full render-diff between the two parsers. It
mechanizes the observed leakage class — the code-span corruption symptom (a
surviving backtick or raw placeholder tag), headings, and tables — and stays
silent on divergences that corrupt none of the three. The first assertion keys
on the shared symptom rather than a multi-backtick fence run, so it implements
the severed-span defense (`gettalong/kramdown#843`) the section describes — prose
and assertion cover the same class. The table count can be
masked by an offsetting raw-HTML `<table>` on the same page: one collapsed GFM
table plus one HTML table balances the counts. The table detector is
deliberately conservative (delimiter-row anchored), so a table kramdown accepts
but the scan does not count can only *under*-count source starts — which
false-cleans, never false-reds. The observed table incident (2026-07-13): the
value page's generated rollup table abutted its `:end` marker and shipped as a
literal-pipe paragraph with the gate silent; the emitter fix (a trailing blank
line) landed then, and this assertion mechanizes the channel. The good/bad
fixture pair exercises the span-corruption symptom (a bad page whose severed span
leaks a stray backtick and a raw placeholder tag, a good page whose faithful code
spans render clean) alongside the fence/heading case. Because that bad page would
red on its fence run alone, every assertion the pair cannot isolate carries a
hermetic unit test of its own, named here rather than described:
`check-docs-render-fidelity-span.test.sh`,
`check-docs-render-fidelity-table.test.sh`,
`check-docs-render-fidelity-foreign.test.sh` and
`check-docs-render-fidelity-batch.test.sh`.

The span test runs a page with no fence, no surplus heading and no table, whose
only defect is the severed span: it reds span-only and clears in the
doubled-backtick form, so a widening that reds nothing new cannot pass. The table
test is that same shape — a collapsed table reds table-only, a trailing blank
clears. The foreign test holds the SVG/MathML exemption to its *scope*: a page
carrying inline `<svg>` clears, because the exemption follows the subtree by
open/close depth rather than a name list, while a bare `<path>` outside any
`<svg>` still reds — and that second half is what keeps the first from passing
under a blanket widening, which would clear the good page at the cost of the
placeholder tokens the assertion exists for. The batch test exists because the
good/bad pair sets no renderer knob, so it already runs the batch path and cannot
by itself distinguish that path from the fallback; it asserts that the kit's two
renderer defaults render a corpus byte-identically, that a pinned
`SITE_KIT_RENDERER` suppresses the batch default, that the batch path and the
per-document fallback return the same verdict on the same pages, and that a wrong
document count and an unresolvable batch command each exit 2. The count case uses
a stub that always emits two documents, so it passes the two-document probe and is
caught only by the corpus count — a stub the probe already rejected would never
reach the assertion under test.
The positional form `check-docs-render-fidelity.sh [docs-dir] [config-file]`
lets a fixture point the docs dir and renderer knobs at a synthetic tree without
touching consumer config. `precommit` tier, coupling the docs tree.

Parser-version fidelity (a second honest limit). The oracle is faithful to the
*parser* GitHub Pages uses (kramdown with GFM input), not necessarily to the
exact kramdown *version* Pages pins: a locally-installed kramdown that differs
in patch or minor version from the Pages-locked one can render a construct
differently, so a green local run is not a categorical proof for a divergence
introduced between the two versions. The exact-pin recipe closes that gap for a
consumer that needs it: point `SITE_KIT_RENDERER` at a version-locked bundle —
a `bundle exec ruby …Kramdown…` invocation whose `Gemfile.lock` pins kramdown
(and `kramdown-parser-gfm`) to the versions the `github-pages` gem resolves — so
the oracle and the deploy render byte-for-byte the same parser build. Pin
`SITE_KIT_RENDERER_BATCH` in the same motion, at the batch form of that same
locked bundle, or leave it unset: those are the two states in which the gate runs
the parser build the pin names. Overriding only the per-document knob already
leaves the batch knob empty by its fill rule, so a half-applied pin costs speed
rather than fidelity — but a consumer who set a batch renderer earlier and pins
only the per-document one afterwards has pinned the knob the gate will not use.
The kit does not auto-resolve the pin: fetching the Pages-locked gemset at gate
time would break the hermetic no-network render contract the oracle depends on (a
gate must run offline and deterministically), so it stays a consumer's deliberate
override, not kit-run machinery.

Renderer agreement (a third honest limit). Where a consumer sets both renderer
knobs, the kit cannot verify that their batch renderer agrees document-for-
document with their per-document one; a divergent pair yields a divergent oracle,
and nothing in the framing contract can detect that. What the kit does hold is
its own pair: the two defaults are asserted byte-identical over a corpus by
fixture, so the zero-config path is covered by construction rather than by
assumption.

## lib/site.sh

The sourced config loader: it loads `SITE_KIT_CONFIG_FILE` (or the gates-dir
`site-config.sh` when that env is unset), then fills each knob's default, so a
gate and a fixture read one resolved configuration. It carries no gate logic —
structure stays in the check, values in config, defaults here.

Every knob's default is filled whenever the consumer left it unset, with one
exception the loader is the only place to state: `SITE_KIT_RENDERER_BATCH` is
filled only where the loader also owns `SITE_KIT_RENDERER` — where the consumer
overrode neither. A consumer who overrode the per-document renderer and left the
batch knob unset gets an empty batch array instead of a default, so the gate
keeps running the oracle they chose; §Layout and configuration states the rule
with the false clean it exists to prevent.

## templates/site-health.yml

The scheduled live-site probe, copied verbatim into a consumer's
`.github/workflows/`. It reads the apex host from the CNAME file (the same
source the gate trusts), then checks: the apex answers 200 over HTTPS, `www`
and `http` redirect to the canonical origin, an optional `ALT_DOMAIN` redirect
keeps its path, the certificate is at least a fortnight from expiry, and every
published release note is pointed at by a resolving URL in its Release body. A
failure opens or updates a single `site-health` issue and reds the run;
recovery closes it. The `ALT_DOMAIN` value is a bare hostname, never a `://`
literal, so it does not itself trip the parity gate. A `# enforce:` marker rides
the template so that, once copied, an enforcement map projects it as a monitor.

**The arm roster is a floor, not a ceiling, and "copied verbatim" describes the
distribution model rather than forbidding an edit.** The copy is the consumer's
file: nothing syncs it back, and no gate couples the two, so a consumer may add
arms of its own. Which side of the seam an arm belongs on follows from what it
asserts. An arm implementing an invariant this kit ships belongs in the template,
where every consumer inherits it. An arm implementing a **consumer-local**
invariant belongs in that consumer's copy and must stay out of the template —
copied into a tree that never declared the convention it derives from, it would
assert something no kit ships and no consumer agreed to, which is dead config
that reads as coverage. An arm cannot be truer than the declaration it composes,
so it lives at the widest tier that declaration is true for. A consumer-local
arm becomes template material only if its underlying declaration becomes kit
mechanism — and then it arrives as an optional arm on the existing
set-the-env-or-delete-the-arm pattern, the way the release-note knobs already do.

The workflow's `permissions:` block is an **allowlist, not an addition** — every
scope it omits is `none` — so reading the tag list and the Release bodies needs
`contents: read` declared beside `issues: write`, and the probe step needs its own
`GH_TOKEN`. On a public repository the declaration is redundant and the arm
appears to work without it; at a private-repo consumer it is the difference
between the arm working and the arm 404ing on every note, because GitHub masks an
unauthorized read as an absent resource — the failure arrives looking like "no
such Release" rather than "not permitted". No gate parses a `permissions:` block
(workflow-security linting is an explicit non-goal, gate-sdk/SPEC.md
§check-action-run-shell), so this one is held by review rather than by an oracle.

**The release-body arm.** For every tracked release note whose front-matter tag
key names a tag that exists on the remote, the arm asserts two properties over
that tag's Release:

- **Presence** — a Release exists for the tag, and its body *contains* the note's
  canonical URL, derived **scheme-qualified** as `https://` plus the apex host
  from the CNAME file plus the path from `RELEASE_NOTE_URL_PATH`. This is what
  catches a body that was never filled in.
- **Resolution** — every apex-hosted URL *as literally written in the body*
  answers 200. This is what catches a body whose pointer is present but dead.

**The two run over two different strings, and collapsing them re-opens the defect
the other closes.** A body carrying a trailing-slash URL *contains* the
slash-less form as a substring, so a presence-only check passes a pointer that
404s; and a probe that derived the URL itself and then resolved *that* would
confirm only its own arithmetic while the body stayed wrong. Presence runs over
the URL the arm derives; resolution over the URLs the body actually carries.

Presence is deliberately loose in one direction and strict in the other.
*Containment, never equality*: a body may write a suffixed or otherwise decorated
variant of the URL, so a token-equality test reds a corpus that is in fact
correct — and the looseness that permits it is exactly what lets the
trailing-slash form through, which is why resolution exists rather than a
stricter presence. *Scheme-qualified*: a markdown link whose visible label
repeats the URL scheme-less satisfies a scheme-less presence test while the link
*target* goes unchecked, and a wrong-but-resolving target then passes both
assertions with nothing red.

URL extraction is **scheme-anchored over the whole body text**, never
markdown-link-aware — only the scheme reliably separates a link target from a
visible label — and a trailing **run** of punctuation is stripped from an
extracted URL before it is resolved. The punctuation that occurs in practice is
markdown-structural rather than sentential: a `)` closing a link target, or `)**`
closing one inside bold. A stripping set written for the period captures those
closers into the URL and reds a healthy corpus, so the set carries the markdown
closers alongside sentence punctuation (the template owns the set), and strips a
run rather than a single character — or `)**` merely becomes `)*`.
The extracted set is **not** narrowed to note URLs: the assertion is over
apex-hosted URLs, so a body's other apex links are covered deliberately.

The tag list comes from the **API, paginated**, never from `git tag`.
`actions/checkout` defaults to `fetch-depth: 1` and fetches no tags, so a
`git tag`-driven arm finds zero released notes and reports green forever. Two
properties of the call are load-bearing: `gh api --paginate` discharges the
paging with no manual page loop, and because it *concatenates* the pages' arrays,
`--jq 'length'` returns the last page's length rather than the total — so the
count is taken by streaming the elements, and the pagination is forward-looking
insurance no run will ever confirm until a consumer passes its first page. The
tag list is not replaceable by a per-note Release-by-tag lookup: a 404 there
cannot distinguish a tag that was never pushed (a legitimate skip for a note
whose release is deferred) from a tag whose Release is missing (a real finding),
and conflating them either reds every deferred note or hides a missing Release.

**The arm's three knobs** are step-level workflow env on the probe step, set in
the copied file the way `ALT_DOMAIN` already is (§Layout and configuration):

- `RELEASE_NOTE_GLOB` — the tracked path glob enumerating release-note files.
- `RELEASE_NOTE_TAG_KEY` — the front-matter key whose value is the tag a note
  belongs to. Load-bearing rather than decorative: a posts directory holds posts
  that are not release notes, and this key is what separates them. It is read
  **anchored inside the opening front-matter fence**, so the same key spelled in
  a note's body is not mistaken for the note's own tag.
- `RELEASE_NOTE_URL_PATH` — the site path a note is published at, written with a
  `{slug}` token the arm substitutes with the note filename minus its extension.
  It holds the **path only**; the host is never repeated here, because the CNAME
  file stays the single source for it and a host rename must stay a one-file
  edit. This is the knob with no cross-check anywhere — nothing outside the site
  generator derives a note's served URL — so it is the one a consumer can set
  wrongly and see nothing red. The concrete trap: a generator that emits
  `/posts/<slug>.html` while the host *also* serves the extensionless form makes
  the suffixed value look the more literally correct of the two, and it reds
  every body written to the bare form.

**The zero-cases, ruled exhaustively.** Every count the arm ranges over can be
zero, and each zero is either a finding or a legitimate pass; an unruled one is
how the arm becomes a green that means nothing.

- **The glob matches no notes** — a **finding**. A misconfigured
  `RELEASE_NOTE_GLOB` is itself the defect, which is what makes "set these knobs
  or delete the arm" an instruction rather than advice.
- **The tag-list call fails** — a **finding**, asserted on the call's own exit
  status with the API's error text (which carries the HTTP code) in the message,
  never inferred from the count that follows. The probe step runs under `set +e`,
  so a failed invocation yields an empty tag list and a zero exit; every note then
  reads "not yet released" and the arm reports a clean run forever. On the
  per-Release lookup a failure reads like "no such Release"; on the tag-list call
  the same failure reads like "green".
- **The per-Release lookup fails for a tag that exists** — a **finding**. The tag
  list has already established that the tag is real, so this is the
  missing-Release case rather than the deferred-release one.
- **The glob matches notes and none carries the tag key** — **not** a finding,
  stated so it is not added by symmetry with the first case. A consumer whose
  notes genuinely precede its first release is in this state legitimately. It is
  nonetheless the likelier of the two knob misconfigurations, precisely because
  the glob's failure is loud and the key's is silent.
- **Zero *released* notes** — not a finding, for the same reason.

**The census line makes those states readable; the rulings above are what make a
vacuous pass impossible.** On every run the arm prints a census of what it ranged
over — notes found, of those carrying the tag key, of those released against the
remote tag count, and of those checked. No count in it is another by
construction: the gap between *found* and *keyed* separates a typo'd
`RELEASE_NOTE_TAG_KEY` from a corpus that simply mixes notes with ordinary posts,
the gap between *keyed* and *released* is the legitimate pre-release state, and
the gap between *released* and *checked* is a Release the arm could not read.
Collapsing *found* into *keyed* would render the first two of those states
identically, which is what reporting them apart buys. This is the same shape as
gate-sdk's vacuous-pass
tripwire (§run-gates): a *reading* available to whoever opens the run log, not an
assertion, and on a green run nobody opens it. It earns its place as the
diagnostic naming which state a reader is in; it is not itself what stands
between this arm and a vacuous pass.

**Stated coverage limit.** The arm is driven from the tree's notes, so it is total
over note→Release and silent on the reverse: a Release carrying no note anywhere
in the tree reds nowhere. The deliverable is the note's pointer, and the
population that has ever gone wrong is the pointer's.

**Cost per run.** The paginated tag list, one Release lookup per released note,
and one resolution request per distinct apex URL in each body — linear in release
count, on a daily schedule, well inside the authenticated rate limit.
Deliberately uncapped and unknobbed: a "newest N only" bound would stop probing
exactly the old releases where this class of defect has actually been found.

The kit's `smoke/install.sh` installs the template verbatim into the scratch tree
as governed surface, and registers the gates that read it. The scratch battery is
the union of what each kit's `smoke/install.sh` registers; this kit contributes
`check-docs-cname-parity` and — because it writes the only Actions-shaped surface
any install writes — the gates that lint a workflow's Actions shape:
`check-action-pinning`, `check-action-run-shell`, `check-action-gh-repo`.
`check-tree-terms` reads the installed file too, for tree terms. So a template
regression in the workflow's bash, its action pins, or its `gh` repository
context reds the smoke rather than surfacing at a consumer.

Those four qualify under the registration accounting's predicate — a gate earns a
scratch-battery slot when it reads a surface the install writes — because this
install writes the workflow they lint. The predicate binds every kit and is owned
by gate-sdk/SPEC.md §Consumer smoke, which also rules on the omissions: what this
kit leaves unregistered is decided by the accounting's probe, not by a judgment
recorded here.

## Out of scope

The kit does not resolve DNS, provision certificates, or configure the host
platform — those are the deployment's concerns, surfaced by the monitor, not
governed by a gate. It does not gate the *content* of the docs site (links,
commands, prose): that is canon-kit's charge over the governed doc set —
`check-docs-render-fidelity` gates how a page *renders*, never what it says. And
it holds no opinion on which host a project uses — only that the tree cites one,
and that the CNAME file names it.
