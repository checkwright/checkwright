---
title: Site architecture
---
<!-- door-contributor: maintainer governance, off-nav by design and reached only by citation; every door on the page is a regeneration command for a generated projection -->

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
siblings. **Every such sibling except the SPEC mirror**, which is a reference
tier: a surface read when something sends you to it, not one a reader is invited
into from every page. It stays reached from its own kit's `index.md` and from a
red gate's resolved pointer (gate-sdk/SPEC.md §Consumer payload). The
reachability gate models the same exclusion, since a model wider than the include
is a green the rendered site does not earn. A nav page naming a `nav_children_key` instead derives its children from
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
link/command-resolved (`scripts/canon-config.knobs`). A page off-nav by design joins
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

- **The on-site SPEC mirror** <!-- projection: check-docs-mirror-fresh --> (`docs/<dir>/SPEC.md`, `docs/<dir>/README.md`,
  `docs/doctrine-kit/DOCTRINE.md`, for every top-level directory holding a
  `SPEC.md` — the kits and `installer/`) — regenerate after editing any mirrored
  SPEC/README/DOCTRINE: `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`
  (`check-docs-mirror-fresh` byte-gates it).
- **The value rollup** <!-- projection: check-value-rollup-fresh --> — `docs/value.md` is hand-framed prose around one
  generated marker block that joins the enforcement-map's per-kit class counts to
  the footprint's per-kit token cost. Regenerate on any change either emitter
  reports: `bash gate-sdk/bin/run-gates.sh --emit value-rollup --write`
  (`check-value-rollup-fresh` byte-gates the block, the byte-fresh projection of
  the same arm without `--write`). The join reads the two emitters live, never
  the committed detail pages, so a stale page cannot poison the rollup; it reads
  them as **structured values rather than rendered markdown**, so the class
  taxonomy and the per-kit figures arrive as data and no heading or table row is
  ever re-parsed. The enforcement page owns the taxonomy. Its section order is
  the hardest-to-softest column order; the cost columns are the footprint's
  per-kit token figure; the totals row reuses the footprint's pre-summed
  totals rather than re-summing them; the kit axis follows the footprint roster,
  then any enforcement-only label (a surface under no kit) groups as
  `(consumer)`. This is a consumer docs ruling. The join axis and column choice
  live here, never in a kit. `docs/value.md` holds the nav slot, while
  `docs/enforcement.md` and `docs/footprint.md` persist as its off-nav
  drill-downs, link-reachable from it.
- **The KPI-roster fan-out** — a `scripts/kpis.list` edit is the widest single
  trigger on this page: adding or removing one KPI moves **three** byte-gated
  surfaces, not the one an amendment naturally names. They are the on-site SPEC
  mirror (the owning kit's SPEC documents the KPI); `docs/enforcement.md` (the
  KPI joins the class registry); and `docs/value.md`'s rollup block (its per-kit
  Advisory count is derived from the enforcement map). Each of the three gates
  names its own regen command on a red, so recovery is mechanical once the
  fan-out is known. Knowing it in advance is the part nothing else states.
  `docs/footprint.md` is **not** in this fan-out, though the shape of the list
  invites the guess: the footprint measures no script, so a KPI's bytes never
  reach it. Its actual trigger is the row below.
- **The enforcement map** <!-- projection: check-enforcement-fresh --> <!-- projection: check-footprint-fresh --> — `docs/enforcement.md` is the class registry's
  projection, stale on any **class-registry** change rather than on a content
  edit: a gate's `tier=`, a `scripts/kpis.list` entry, the settings hooks, a
  `# enforce:` marker. `check-enforcement-fresh` byte-compares it (`bash
  gate-sdk/bin/run-gates.sh --emit enforcement-map > docs/enforcement.md`). Its sibling
  `docs/footprint.md` is the per-kit token cost (`bash
  gate-sdk/bin/run-gates.sh --emit footprint > docs/footprint.md`, the emitter
  having ported to a non-gate arm the runner resolves config for). **Its measured
  set is narrower than "any kit file", and that misreading is the standing mistake
  here**. context-kit/SPEC.md §bin/footprint owns the set, and what follows from
  it is this row's business: the trigger is an injected-block edit, a `templates/`
  markdown edit, or a kit joining or leaving the roster. A SPEC body edit is
  **not** a trigger, nor is any script under `bin/`, `checks/` or `scripts/`,
  since the set contains none of those. It reads the worktree rather
  than the index, so no staging order binds its regen. Both are `docs/value.md`'s
  inputs, so a red in either implies a rollup regen.
- **The trajectory projection** <!-- projection: check-trajectory-fresh --> — `docs/evidence-data.md` is the published
  evidence extractor's output (`bash gate-sdk/bin/run-gates.sh --emit trajectory >
  docs/evidence-data.md`, `check-trajectory-fresh` byte-gates it), stale on a
  stage stamp or a release disposition. Its regen rides the close stage's
  Clear-Done commit, because the gate is blind at the enter-close commit by
  construction. **The name collides and the two surfaces are unrelated:**
  `check-trajectory-fresh` and the `trajectory` arm govern *this* generated page, never
  the hand-authored `TRAJECTORY.md` ruling record, which no gate byte-checks.
- **The install-evidence projection** <!-- projection: check-install-evidence-fresh --> — `docs/install-evidence.md` is the
  install-evidence arm's output (`bash gate-sdk/bin/run-gates.sh --emit
  install-evidence > docs/install-evidence.md`,
  `check-install-evidence-fresh` byte-gates it), stale on any capture into the
  install-observation record and on a `scripts/gates.list` edit, that roster
  being the classifier the per-gate block sorts a red's gate name against
  (drift-kit/SPEC.md §The install-evidence projection). **Its gate is inert
  everywhere but the observing machine, by construction**: the record is
  gitignored, so in CI, in a fresh clone and in an adopter's tree there is
  nothing to re-emit from and the gate reports a counted zero. The page is
  committed carrying published zeros from the first build rather than held back
  until an install exists, so the freshness gate has a target and the roster row
  above it names a file that is there.
- **The roadmap projection** <!-- projection: check-roadmap-fresh --> — `ROADMAP.md` is a root projection of the queue's
  curated `[roadmap:]` tags (`bash gate-sdk/bin/run-gates.sh --emit roadmap --write`,
  `check-roadmap-fresh` byte-gates its marker block), stale on any `[roadmap:]`
  tag edit. Never hand-edited, and never regenerated by the whole-file redirect
  the trajectory projection takes: the arm emits only the block, so a redirect
  destroys the prose above it.
- **The graph artifact** <!-- projection: check-graph --> — `docs/check-graph.html` and the generated `pre-commit`
  and `commit-msg` hooks are one set with one trigger, a gate's `# graph:`
  manifest. The hooks also stale on three edits no manifest names. A kit-config
  edit stales them, since a ported member's invocation bakes its resolved knob
  values in (gate-sdk/SPEC.md §gen-pre-commit). Adding a kit
  `gate-tests/*.test.sh` stales them: `--emit-enum-sets` derives its basename into
  the `check-prose-enum` roster the hooks bake verbatim. Any tree edit that
  *moves* a measured claim stales them, since the baked invocation carries
  `check-measured-claim`'s resolved values; a script header gaining a
  `# no-port:` cause moves the `tree-shell-owed` key this way. One command emits
  both hooks
  (`bash gate-sdk/bin/run-gates.sh --emit git-hooks --write`), then the artifact
  (`bash gate-sdk/bin/run-gates.sh --emit graph > docs/check-graph.html`), which
  `check-graph` asserts fresh together. The hooks are never hand-edited;
  that rule is resident in `CLAUDE.md` because a session about to edit it is not
  looking at a red gate.
- **The new-tag-class-member fan-out** — adding a member to
  `check-tag-lead-line`'s class table is a one-line edit with a four-surface
  wake, and it is invisible from the edit: `--emit enum-sets` derives the tag
  members, so `check-prose-enum` reds every hand-written prose enumeration of
  the queue's tag set — `README.md`, `queue-kit/README.md` and
  `docs/queue-kit/index.md`, none of them generated, each repaired by hand. The
  generated hooks bake that same derived roster verbatim and stale with it
  (`bash gate-sdk/bin/run-gates.sh --emit git-hooks --write`, then the graph artifact the
  row above pairs with them). The new-gate row is the fan-out this file already
  rostered; this is the other one, and nothing derived it for the author.
- **The new-gate fan-out** — the other wide trigger, and the one with no single
  owner elsewhere. `gate-sdk/SPEC.md`'s kit-landing checklist covers the kit-side
  obligations (SPEC section, `good/`+`bad/` fixture pair, the README's
  `<!-- gate-roster:begin -->` block, `smoke/`, registration in
  `scripts/gates.list`) and is silent on the projections a new gate stales,
  because a kit may not name a consumer's docs surfaces. Assembled here so the
  next author reads the list instead of discovering it one red gate at a time:
  - the on-site SPEC mirror (`bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`);
  - `docs/enforcement.md`, which the gate joins as a class-registry member
    (`bash gate-sdk/bin/run-gates.sh --emit enforcement-map > docs/enforcement.md`);
  - `docs/value.md`'s rollup block, derived from that map
    (`bash gate-sdk/bin/run-gates.sh --emit value-rollup --write`);
  - `docs/check-graph.html`
    (`bash gate-sdk/bin/run-gates.sh --emit graph > docs/check-graph.html`);
  - the owning kit's `smoke/install.sh` expected-gate roster, hand-maintained, so
    a new gate is added there or carries a `smoke-unregistered:` declaration
    (`gate-sdk/SPEC.md` §Consumer smoke owns which);
  - `.workflow/surface-ceiling.txt`, since `check-surface-ratchet` governs the
    `docs/` pages and `docs/enforcement.md` grows by the new row
    (`bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling`);
  - for a hook-tier gate, the generated hooks
    (`bash gate-sdk/bin/run-gates.sh --emit git-hooks --write`);
  - for a freshness gate, a `# projection: <outputs>` header line, which makes
    its keyed row on this roster mandatory (`check-projection-roster` holds both;
    gate-sdk/SPEC.md §check-projection-roster owns the key).

  `docs/footprint.md` is absent for the reason the row above gives: a gate is a
  script or a crate module and the footprint measures neither. A prose-only SPEC
  edit reds the on-site mirror alone. **Two of these regenerations are
  staging-ordered.** The generated hooks and the gate binary both derive through
  `git ls-files`, so a unit adding a file stages first and regenerates second.
  The two hazards and their routes are this page's closing paragraphs; a
  new-gate unit adds files by definition.
- **The install-toolchain parity contract** — `docs/install.md`'s Requirements
  section holds the toolchain list to the probe roster:
  `check-install-toolchain` asserts whole-element parity between its
  `<!-- toolchain:begin -->` bullets and `native/src/toolfloor.rs`'s
  `PROBE_SET` array both directions. Each bullet's parenthetical renders its
  roster element verbatim, so all four axes are held: name, version floor,
  implementation token and audience. The axes are comma-joined, and an
  unconstrained member takes no parenthetical:

  ```text
  - `<name>` (≥ <floor>) — …
  - `<name>` (<impl-token>) — …
  - `cargo` (≥ 1.71, @contributor) — …
  ```

  The audience carries a leading `@` for the same reason the floor carries `≥`:
  the gate's reader is positional, so an axis with no sigil would be
  indistinguishable from the implementation token. Elements are derivable and
  purpose clauses hand prose, so a roster edit reds the docs list without an
  emitter handshake. The gate reads the crate's own constant by default — and,
  where an element's audience is `derived` (context-kit/SPEC.md §bin/env-probe),
  resolves it by walking the kit roots before comparing. That half is why this
  contract is not satisfiable by editing both sides: one of them is a
  measurement of the tree, so a kit shipping an undeclared surface reds the page
  rather than joining it silently. A derived element the walk cannot resolve
  exits 2 rather than comparing the page against an empty audience. A
  hermetic fixture may steer it onto a roster file instead, and that file is
  **parsed and never sourced**, because a fixture path is untrusted input and the
  reader that lints the array must not be made to execute the file it reads.

- **The install-platforms parity contract** — `docs/install.md`'s Requirements
  section carries a **second** marker block (`<!-- platforms:begin -->`), and
  this row is where a reader looks for what holds it, as the row above is for the
  first. It is hand-authored like its neighbour. It declares one entry per
  **supported platform** in its neighbour's grammar, so one positional reader
  shape serves both: `` - `<triple>` `` followed by a parenthetical carrying the
  **join state**, then an em dash and free prose. There are two states and no
  third. `(joined)` means the triple is a live line in `native/targets.list`.
  `(held: <precondition>)` means the platform is documented as supported but is
  **not** in the roster, and it carries the named run that would join it. A hold with no stated cause is how a pile grows
  silently, so the precondition is mandatory rather than conventional. The
  parenthetical is read to its first `)` and the triple is the line's first
  backticked run, so neither may carry a nested parenthesis and the state must
  sit on the bullet's own first line; continuation lines are prose and are not
  read. A platform the page does not state as supported is **absent** rather than
  held: a held entry is still a support claim, so absence is the state for a
  platform nothing has been promised about.
  Two readers: `.github/workflows/gates.yml`'s `native-artifacts` roster step,
  which derives the producer's build matrix from **every** declared triple
  regardless of state, since a platform that is never built is a platform that
  can never stop being held; and `check-install-platforms`, which holds the block
  and the roster in lockstep in both directions and is what mechanizes
  gate-sdk/SPEC.md §Consumer payload's first bound. **That gate has landed**, a
  born-native repo-root member at `precommit` tier, so the lockstep is
  machine-held rather than discipline. A `joined` declaration with no roster
  line reds; so does a roster line no bullet declares `joined`; a hold reds on
  an empty precondition or on a roster line it should not have.
  **The binding is four-way rather than three-way, and the fourth and fifth
  surfaces are the two host detectors:** `target_of_host` in
  `installer/bin/checkwright.sh` and `Get-HostTarget` in
  `installer/bin/checkwright.ps1`, whose extraction shapes installer/SPEC.md
  §The gate binary pins. Each detector's **emitted** triple set is held equal to
  the block's **declared** set. It is equality rather than containment because
  each direction closes a distinct failure. A triple a detector emits that the block
  does not declare is the attested case: detected by the installer while on no
  roster, with nothing going red. A triple the block declares that no detector
  emits is a support claim the installer can never honour, and it has no live
  instance, which is exactly why it is asserted rather than assumed. It carries a
  fourth arm that asserts nothing and reports instead. Per held platform it
  prints the count of registry members a host with no published artifact loses,
  on the clean line as well as the red one. **That arm carries two limits, and it
  is not the standing instrument a reader looking for the aggregate cost
  wants.** Its subject is the held set, so it reports only *while* something is
  held. A join that empties the block of holds silences it (the declaration's
  state with `x86_64-apple-darwin` joined), and a report with no subject is the
  arm working rather than a gap. And the aggregate cost
  gate-sdk/SPEC.md §The port-candidate criteria names has a different
  instrument, the **binary-less leg**; that section owns how the two relate and
  why this arm is not a second measurement of it. Two
  readers of one grammar is the shape `scripts/gates.list` already ships (a bash
  reader and a compiled one), not a duplication to collapse.
  **The state word now decides a binding posture on both sides of the artifact
  hand-off, and that is a wider consequence than "which platforms get built".**
  The roster step publishes what it derives twice, as the producer matrix and
  as an object keyed by triple. That second output is read at *job* level by
  the Intel `install-smoke` leg, whose `runs-on` and `continue-on-error` both
  resolve from it. So flipping a bullet from `held:` to `joined` makes that
  triple's **producer** leg binding and its **consumer** leg binding, in one
  edit, with no workflow change and nobody remembering to make it so. Read the
  state word as a support commitment taking effect rather than as documentation:
  the legs are green at the moment of a flip, so nothing reds at the landing. The
  cost arrives the first time that platform breaks.
  **The mechanism has worked instances now, and the second is what makes it a
  mechanism rather than one file's arrangement.** `x86_64-apple-darwin`'s flip to
  `joined` made that triple's producer leg and its Intel `install-smoke` consumer
  leg binding on master at the landing commit, and the diff that did it edits
  `.github/workflows/gates.yml` nowhere; a roster line and a state word are the
  whole of it. `x86_64-pc-windows-msvc` then took the same route, and its
  `install-smoke-windows` leg reads its `runs-on` and `continue-on-error` off the
  roster step's keyed output exactly as the Intel leg does, so no platform leg's
  posture is hard-coded any more.

- **The remedy blocks** — `docs/install.md`'s Requirements section carries two
  more marker blocks, hand-authored, each read by its platform's install-smoke
  legs and run verbatim. The HTML-comment pair `macos-remedy:begin` and
  `macos-remedy:end` holds one `sh` fence; its readers are `install-smoke-macos`
  and `install-smoke-macos-intel`, and each runs the body in the step's shell and
  persists the `PATH` entries the body prepended to `$GITHUB_PATH`, then asserts a
  fresh login shell resolves the ordering. The pair
  `windows-remedy:begin` and `windows-remedy:end` holds one `powershell` fence;
  its readers are `install-smoke-windows` and `install-smoke-powershell`, and both
  run the body under PowerShell, the shell a native-Windows adopter types it
  into — the bash leg extracts it with the macOS legs' awk program and hands it
  to `pwsh -NoProfile -Command`, the pwsh leg extracts it in PowerShell; the pwsh
  leg then runs `doctor` under the `PATH` a new terminal would compose. In every
  leg only the fence lines and blank lines are skipped, and an empty extraction
  reds the leg by name. **No gate holds either block.** Each is hand-authored
  source with no emitter, and the binding legs are its enforcement: a malformed
  block, or one missing a package its platform needs, reds the push that carries
  it. **Honest limit:** that red arrives at push rather than at commit, since no
  local battery runs a Mac or a native-Windows host. **Two shapes were refused.**
  A shared script the legs call would merge the copies the legs once carried, yet
  would still hold nothing equal to the page. A gate holding the legs' package
  set to the toolchain list would need a mapping from floor members to Homebrew
  formulae or Chocolatey packages, and that mapping is neither one-to-one nor
  derivable, so no surface for it exists. Running the page's own block removes
  the duplication instead of policing it. A package the page drops leaves the legs
  on the same run; one the legs need but the page lacks is the adopter's broken
  path showing up as a red. `scripts/ci-macos-floor.sh` reads nothing here. It is
  the `native-artifacts` build legs' runner floor, and it answers to what that
  job executes.

**A derived surface earns a row here only when it has a reader who cannot run
the emitter** — a public page, a file a fresh clone needs before its tooling
works. Otherwise deriving on demand satisfies derivation-first, and a committed
copy of a high-churn source's derivation buys a per-commit regeneration tax for
nobody. So **a tool with no stored projection has nothing to hold fresh** and
stays off this roster: queue-kit's `queue-index` and `queue-edges` arms are the
standing instances, the latter with its refusal reasoned in its own contract
(queue-kit/SPEC.md §The queue-edges arm). Their absence is a ruling. Ask of a
new derived surface who reads it, not whether it could be generated. **Both rulings survived a port onto
the binary on that stored-projection ground alone**, and the shell-consumer half
of each stopped being true at its own port, when the consumer became a session
reaching a compiled arm through the `--emit` front-end.

**The compiled gate binary is the third standing instance, and it fails the
admission test in both directions.** It is never committed. With
`native/target/` gitignored there is no tracked copy for a freshness gate to
byte-compare, and every reader of it in this repo can run the emitter, which is
`cargo build`. A
consumer is not a counter-example: a consumer never receives the crate source and
never builds, and the artifact they do receive is held by a published digest
verified before it is written (gate-sdk/SPEC.md §Consumer payload), a different
guarantee. The binary owes build currency, discharged by an oracle rather than a
row here: `check-gate-binary-fresh` (gate-sdk/SPEC.md §check-gate-binary-fresh)
compares the binary's baked source stamp against the crate's tracked source
whenever a `.gate` descriptor makes it load-bearing. **It carries a
staging-order hazard.** The stamp is computed over *tracked* crate source, so a
unit adding a crate file builds after `git add`, never before, or the binary is
stamped against a source set the gate does not hash; the rule is the owner's
(gate-sdk/SPEC.md §check-gate-binary-fresh), named here because the reflex is to
read the hazard off the artifact's own freshness rule, which is not where it
lives, and the next paragraph's hook hazard is its only sibling.

**The generated pre-commit hook carries one too, by a different route, and the
two are the whole set.** The hook bakes each gate's resolved argv, and
`check-prose-enum`'s enum-set emitter derives its `*-gate-test` members with `git
ls-files` — tracked, not the worktree. So a unit adding a `gate-tests/*.test.sh`
sibling sees a green whole-tree battery while the file is untracked, and the hook
reds at commit time as a `check-graph` artifact staleness the moment `git add`
admits it. Same discipline as the row above: stage first, regenerate second.
Every other row here reads the worktree and is order-free.
