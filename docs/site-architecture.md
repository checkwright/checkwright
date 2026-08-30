---
title: Site architecture
---

# The docs/ site architecture

`docs/` is the public GitHub-Pages site (served from `docs/` on master via its
`CNAME`), repo-root-governed with no owning kit. This page is the load-triggered
home for the site's chrome, page-authoring rules, generated projections, and docs
gate roster — the mechanism CLAUDE.md §Housekeeping names by pointer rather than
carries inline. It is off-nav by design (`scripts/docs-offnav.list`): maintainer
governance reached by citation, not a reader-nav destination.

## Site chrome and the nav contract

The site chrome — the Jekyll layout, client-side search, and theme selector —
lives in `docs/_config.yml`, `docs/_layouts/`, `docs/_includes/`, and
`docs/assets/`. The nav is Liquid over front matter: `nav_order` / `nav_parent`
place a page; a page's `nav_id` parents its `nav_child_order`-sorted children,
each child carrying derived suffix links to its `generated:`-marked directory
siblings. A nav page naming a `nav_children_key` instead derives its children from
the site pages carrying that key, path-descending, each labeled by the key's
value (the release notes under the Releases page). `check-docs-nav-reachable`
holds every docs page to a `title:` front-matter block and reachability from the
rendered nav (a nav slot, a relative-link walk seeded from the nav set, or the
generated-sibling suffix rule), with `scripts/docs-offnav.list` the allowlist for
pages off-nav by design. That same gate models the front-matter facts and the
derived-children rule; no separate gate covers them.
`check-docs-render-fidelity` asks a different question. It renders every tracked
docs page through the Pages parser and holds the output against markdown
corruption: a code span leaking a stray backtick or a raw tag into text, a
code-fenced heading promoted to a real one, or fewer rendered tables than the
source has GitHub-Flavored Markdown table starts. A missing renderer fails it
closed (site-kit/SPEC.md §check-docs-render-fidelity). The kit registry lives on
`docs/kits.md` (the Kit Reference page); `check-docs-kit-parity` holds every kit's
row there and the nav child block (`nav_parent: kits` + `nav_child_order`) on
every `docs/<kit>/index.md`.

## Page-authoring rules

A page's `title:` is its terse nav label; its opening H1 carries the descriptive
full form (nav stays scannable, the page reads whole). Living pages are governed
prose under the anti-restatement doctrine (cite downward, never restate a SPEC's
invariant); dated `docs/posts/` are immutable, temporal-exempt but still
link/command-resolved (`scripts/canon-config.sh`). A page off-nav by design joins
`scripts/docs-offnav.list` — an embedded data fragment no link targets, or
maintainer governance like this page reached only by citation.

Markdown constructs that render correctly on github.com but corrupt under the
Pages parser are **not** listed here: site-kit/SPEC.md §check-docs-render-fidelity
owns that roster (the consecutive-fence, promoted-heading, collapsed-table, and
severed-inline-span classes) together with the gate that catches them and its
honest limits. Reach for it — not this page — when a well-formed construct
renders wrong or reds that gate: the cause is routinely far upstream of the
symptom in the same file, so the block the gate names is often the victim
rather than the offender.

**Which parser serves which file** is what scopes that roster, and it is the
first thing to establish before calling a rendering a defect. The Jekyll source
is `docs/`, so the Pages parser (kramdown plus `kramdown-parser-gfm`) serves
`docs/` pages and nothing else. Every root manifest page (`README.md`,
`ROADMAP.md`, `CONTRIBUTING.md`, `RELEASING.md`, `CODE_OF_CONDUCT.md`,
`SECURITY.md`) sits outside that source and is not mirrored, since the mirror's
source set is kit SPEC/README/DOCTRINE, below. None of them has a site URL. A
docs page reaching one links it through the self-repo blob grammar, into
GitHub's **repository view**, which is CommonMark-based (`cmark-gfm`) and
markedly more forgiving; it joins a code span across a newline, which is the
severed-span class's whole premise. So a kramdown-only symptom found in a root
page is **not** a defect, because no reader of that page is served by kramdown.
Render a page through the parser that actually serves it before filing.

## Generated projections and their freshness gates

Several docs surfaces are generated and byte-gated for freshness; each gate's red
output names its own regen command, so the command need not stay resident to be
recoverable:

- **The on-site SPEC mirror** (`docs/<kit>/SPEC.md`, `docs/<kit>/README.md`,
  `docs/doctrine-kit/DOCTRINE.md`) — regenerate after editing any kit
  SPEC/README/DOCTRINE: `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`
  (`check-docs-mirror-fresh` byte-gates it).
- **The value rollup** — `docs/value.md` is hand-framed prose around one
  generated marker block that joins the enforcement-map's per-kit class counts to
  the footprint's per-kit token cost. Regenerate on any change either emitter
  reports: `bash gate-sdk/bin/run-gates.sh --emit value-rollup --write`
  (`check-value-rollup-fresh` byte-gates the block, the byte-fresh projection of
  the same arm without `--write`). The join reads the two emitters live — never
  the committed detail pages, so a stale page cannot poison the rollup — and
  reads them as **structured values rather than rendered markdown**, so the class
  taxonomy and the per-kit figures arrive as data and no heading or table row is
  re-parsed. The taxonomy and its hardest-to-softest column order are still owned
  by the enforcement page, now as its section order rather than as text scraped
  back out of it; the cost columns are the footprint's per-kit
  token figure, and the totals row reuses the footprint's pre-summed token totals
  rather than re-summing; the kit axis follows the footprint roster, then any
  enforcement-only label (a surface under no kit) groups as `(consumer)`. It is a
  consumer docs ruling, not kit mechanism — the join axis and column choice live
  here, never in a kit. `docs/value.md` holds the nav slot; `docs/enforcement.md`
  and `docs/footprint.md` persist as its off-nav drill-downs, link-reachable from
  it.
- **The KPI-roster fan-out** — a `scripts/kpis.list` edit is the widest single
  trigger on this page: adding or removing one KPI moves **three** byte-gated
  surfaces, not the one an amendment naturally names — the on-site SPEC
  mirror (the owning kit's SPEC documents the KPI), `docs/enforcement.md` (the
  KPI joins the class registry), and `docs/value.md`'s rollup block (its per-kit
  Advisory count is derived from the enforcement map). Each of the three gates
  names its own regen command on a red, so recovery is mechanical once the
  fan-out is known — knowing it in advance is the part nothing else states.
  `docs/footprint.md` is **not** in this fan-out, though the shape of the list
  invites the guess: the footprint measures no script, so a KPI's bytes never
  reach it — its actual trigger is the row below.
- **The enforcement map** — `docs/enforcement.md` is the class registry's
  projection, stale on any **class-registry** change rather than on a content
  edit: a gate's `tier=`, a `scripts/kpis.list` entry, the settings hooks, a
  `# enforce:` marker. `check-enforcement-fresh` byte-compares it (`bash
  gate-sdk/bin/run-gates.sh --emit enforcement-map > docs/enforcement.md`). Its sibling
  `docs/footprint.md` is the per-kit token cost (`bash
  gate-sdk/bin/run-gates.sh --emit footprint > docs/footprint.md`, the emitter
  having ported to a non-gate arm the runner resolves config for). **Its measured
  set is narrower than "any kit file", and that misreading is the standing mistake
  here** — context-kit/SPEC.md §bin/footprint owns the set, and what follows from
  it is this row's business: the trigger is an injected-block edit, a `templates/`
  markdown edit, or a kit joining or leaving the roster — **not** a SPEC body
  edit and not any script under `bin/`, `checks/` or `scripts/`, since the set
  contains none of those. It reads the worktree rather
  than the index, so no staging order binds its regen. Both are `docs/value.md`'s
  inputs, so a red in either implies a rollup regen.
- **The trajectory projection** — `docs/evidence-data.md` is the published
  evidence extractor's output (`bash gate-sdk/bin/run-gates.sh --emit trajectory >
  docs/evidence-data.md`, `check-trajectory-fresh` byte-gates it), stale on a
  stage stamp or a release disposition. Its regen rides the close stage's
  Clear-Done commit, because the gate is blind at the enter-close commit by
  construction. **The name collides and the two surfaces are unrelated:**
  `check-trajectory-fresh` and the `trajectory` arm govern *this* generated page, never
  the hand-authored `TRAJECTORY.md` ruling record, which no gate byte-checks.
- **The roadmap projection** — `ROADMAP.md` is a root projection of the queue's
  curated `[roadmap:]` tags (`bash gate-sdk/bin/run-gates.sh --emit roadmap --write`,
  `check-roadmap-fresh` byte-gates its marker block), stale on any `[roadmap:]`
  tag edit. Never hand-edited.
- **The graph artifact** — `docs/check-graph.html` and the generated `pre-commit`
  and `commit-msg` hooks are one set with one trigger, a gate's `# graph:`
  manifest — plus, for the hooks, the resolved knob values a ported member's
  invocation bakes into them (gate-sdk/SPEC.md §gen-pre-commit), so a kit-config
  edit stales them too — and so does adding a kit `gate-tests/*.test.sh`, whose
  basename `scripts/enum-sets.sh` derives into the `check-prose-enum` roster the
  hooks bake verbatim, staling them with no manifest or config touched at all —
  and so does any tree edit that *moves* a measured claim, since the baked
  invocation carries `check-measured-claim`'s resolved values, so a script header
  gaining a `# no-port:` cause moves the `tree-shell-owed` key and stales the
  hooks from a file no manifest names either:
  one command emits both hooks
  (`bash gate-sdk/bin/gen-pre-commit.sh --write`), then the artifact
  (`bash gate-sdk/bin/run-gates.sh --emit graph > docs/check-graph.html`), which
  `check-graph` asserts fresh together. The hooks are never hand-edited;
  that rule is resident in `CLAUDE.md` because a session about to edit it is not
  looking at a red gate.
- **The new-gate fan-out** — the other wide trigger, and the one with no single
  owner elsewhere: `gate-sdk/SPEC.md`'s kit-landing checklist covers the kit-side
  obligations (SPEC section, `good/`+`bad/` fixture pair, the README's
  `<!-- gate-roster:begin -->` block, `smoke/`, registration in
  `scripts/gates.list`) and is silent on the projections a new gate stales,
  because a kit may not name a consumer's docs surfaces. Assembled here so the
  next author reads the list instead of discovering it one red gate at a time:
  the on-site SPEC mirror (`bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`),
  `docs/enforcement.md` (`bash gate-sdk/bin/run-gates.sh --emit enforcement-map >
  docs/enforcement.md` — the gate joins the class registry), `docs/value.md`'s
  rollup block (`bash gate-sdk/bin/run-gates.sh --emit value-rollup --write`,
  derived from the map above),
  `docs/check-graph.html` (`bash gate-sdk/bin/run-gates.sh --emit graph >
  docs/check-graph.html`), the owning kit's `smoke/install.sh` expected-gate
  roster (hand-maintained, so a new gate is added there or carries a
  `smoke-unregistered:` declaration — `gate-sdk/SPEC.md` §Consumer smoke owns
  which), `docs/install.md`'s
  `ported-gate-members` measured claim (for a gate born native, which every new
  gate now is), and — for a hook-tier gate — the generated hooks
  (`bash gate-sdk/bin/gen-pre-commit.sh --write`). `docs/footprint.md` is absent
  for the reason the row above gives: a gate is a script or a crate module and
  the footprint measures neither. A prose-only SPEC edit reds the on-site mirror
  alone. **Two of these regenerations are staging-ordered** — the generated hooks
  and the gate binary both derive through `git ls-files`, so a unit adding a file
  stages first and regenerates second; the two hazards and their routes are the
  closing paragraphs of this page, and a new-gate unit adds files by definition.
- **The install-toolchain parity contract** — `docs/install.md`'s Requirements
  section holds the toolchain list to the probe roster:
  `check-install-toolchain` asserts whole-element parity between its
  `<!-- toolchain:begin -->` bullets and `context-kit/lib/toolfloor.sh`'s
  `PROBE_SET` array both directions — name, version floor, implementation
  token, and audience, since each bullet's parenthetical renders its roster
  element verbatim
  (`` - `bash` (≥ 4.3) — … ``, `` - `awk` (GNU) — … ``,
  `` - `cargo` (≥ 1.71, @contributor) — … ``, the axes comma-joined,
  no parenthetical for an unconstrained member). The audience carries a leading
  `@` for the same reason the floor carries `≥`: the gate's reader is
  positional, so an axis with no sigil would be indistinguishable from the
  implementation token. Elements are derivable, purpose
  clauses hand prose, so a roster edit reds the docs list without an emitter
  handshake. The gate greps the roster and never sources it: a fixture path is
  untrusted input, so the reader that lints the array must not be made to execute
  the file it reads.

**A derived surface earns a row here only when it has a reader who cannot run
the emitter** — a public page, a file a fresh clone needs before its tooling
works. Derivation-first is satisfied by deriving on demand otherwise, and a
committed copy of a high-churn source's derivation buys a per-commit
regeneration tax for nobody. So **a tool with no stored projection has nothing to
hold fresh** and stays off this roster: queue-kit's `queue-index` arm and
`bin/queue-edges.sh` are the standing instances, the latter with its refusal
reasoned in its own contract (queue-kit/SPEC.md §bin/queue-edges.sh) and as the
one whose only consumer is still a session with a shell. Their absence is a
ruling, not an oversight — the question to ask of a new derived surface is who
reads it, not whether it could be generated. The `queue-index` ruling survived
its port onto the binary on that stored-projection ground alone, the
shell-consumer half having stopped being true when the consumer became a session
reaching a compiled arm through the `--emit` front-end.

**The compiled gate binary is the third standing instance, and it fails the
admission test in both directions.** It is not committed at all (`native/target/`
is gitignored), so there is no tracked copy for a freshness gate to byte-compare;
and every reader of it in this repo can run the emitter, which is `cargo build`. A
consumer is not a counter-example: a consumer never receives the crate source and
never builds, and the artifact they do receive is held by a published digest
verified before it is written (gate-sdk/SPEC.md §Consumer payload) — a different
guarantee with a different mechanism. What the binary does owe is build currency,
and that obligation is discharged by an oracle rather than by a row here:
`check-gate-binary-fresh` (gate-sdk/SPEC.md §check-gate-binary-fresh) compares the
binary's baked source stamp against the crate's tracked source whenever a `.gate`
descriptor makes it load-bearing. Recorded because the derivation-first reflex
reads "generated artifact" and reaches for this roster; the answer is that the
roster's admission rule is narrower than that reflex, and the obligation has a
home. **It carries a staging-order hazard** — the stamp is computed over *tracked*
crate source, so a unit adding a crate file builds after `git add`, never before,
or the binary is stamped against a source set the gate does not hash. The rule
and its mechanism are the owner's (gate-sdk/SPEC.md §check-gate-binary-fresh); it
is restated here because the reflex is to read the hazard off the artifact's own
freshness rule, which is not where it lives.

**The generated pre-commit hook carries one too, by a different route, and the
two are the whole set.** The hook bakes each gate's resolved argv, and
`check-prose-enum`'s enum-set emitter derives its `*-gate-test` members with `git
ls-files` — tracked, not the worktree. So a unit adding a `gate-tests/*.test.sh`
sibling sees a green whole-tree battery while the file is untracked, and the hook
reds at commit time as a `check-graph` artifact staleness the moment `git add`
admits it. Same discipline as the row above: stage first, regenerate second.
Every other row here reads the worktree and is order-free.
