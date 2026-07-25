# SPEC amendment: activation-installer

A root-level amendment: it stands up a new top-level surface that no kit owns
(`installer/`), rewrites a published governance ruling (the registry doctrine),
and reaches context-kit (the floor roster its `doctor` consumes), gate-sdk
(the kit-roots deriver its pack step reads, and the lint scope its own scripts
fall outside of), `docs/`, `.github/workflows/`, and the registries that make a
root surface governed.

It is the activation path: a clean Linux repo reaching first green with one
command, without giving up the property the whole distribution model rests on —
that what governs your tree is committed, auditable source you read before you
run it.

**Provenance seam.** Nothing private crosses. Kit names are public, the profile
rosters are sets of them, the manifest schema is generic mechanism, and the npm
name is a reservation the tree already carries at `reserve/npm/`. The one
private fact adjacent to this work is the account holding that name, which stays
in the local brief: the publish workflow authenticates through a repository
secret and no tracked file names an account, a maintainer identity, or a
registry credential — the repo's standing rule, binding here because a publish
workflow is exactly where such a name would otherwise land.

## What changes

Every delta carries its work class. **mechanical** — executing it demands only
oracle-running (a fixed battery, a substitution sweep, a regen command).
**design-bearing** — executing it demands generative or verificational judgment.

### A. The envelope ruled at promotion

**A1. Phasing, stated so build does not discover the overrun.
{design-bearing}** This is the largest entry in the queue and the phasing is
what makes it iteration-shaped. **Phase 1** — `init`, `doctor`, `--dry-run`, the
manifest, the profile roster, the packaging, and the smoke that proves the path.
**Phase 2** — `update`, `diff`, `uninstall`. Phase 2 is a separate build unit and
this amendment governs both, so the phase-1 half is not designed in a way phase 2
must undo.

One rule binds phase 1 to that promise and is checkable at review: **no manifest
field is written in phase 1 whose only reader arrives in phase 2.** `B5`'s field
table names a phase-1 reader for each of the six fields; a seventh field
proposed for `update`'s benefit alone is deferred with `update`. The
canonical-spec rule is the general one — a field with no named reader is
removed — and this is its phasing form.

**A2. The registry doctrine survives and is rewritten, not repealed.
{design-bearing}** `docs/install.md` opens by ruling that "the package registries
hold the name only, never a dependency channel. There is nothing to
`npm install` or `cargo add`." A published installer reopens that ruling, and the
operator ruling at promotion closes it: an `npx checkwright init` that vendors
pinned source and commits it is **not** a dependency channel — nothing resolves
at your build time and the kits still land as committed, auditable source, which
is the property the doctrine protects.

The page is rewritten to distinguish a one-shot vendoring installer from a
resolved dependency. The distinction is only honest if two properties hold, and
both are asserted rather than claimed:

- **The payload ships inside the published tarball**, so `init` copies from the
  package and fetches nothing at run time — the same "nothing is fetched at
  install time or at run time" the page's hook-review section already claims of
  the hooks, extended to the installer that writes them. `B6`'s smoke installs
  from a packed tarball with no network, which is what proves it.
- **Nothing the installer writes is a resolvable dependency reference** — no
  `dependencies` entry, no lockfile pointing at a registry, no submodule, and no
  install-time lifecycle script. `F1` gates the package side of that.

The crates.io half of the sentence is untouched: `reserve/crates/` stays a bare
reservation, so "nothing to `cargo add`" survives verbatim and the rewrite must
not collapse the two halves into one claim.

### B. The installer surface

**B1. `installer/` — a new top-level directory, bash inside, npm outside.
{design-bearing}**

*Language.* The implementation is **bash**; npm is the delivery vehicle and never
the implementation. The `bin` entry is a bash script with a
`#!/usr/bin/env bash` shebang. Three reasons converge. The supported platform set
is Unix-first and the floor contract already requires bash on `PATH`, so bash
adds no dependency the adopter does not already have. A bash installer is
lintable by the linter the repo already runs, where a JavaScript one would ship a
language no gate in the tree reads — **lintable, not already linted**:
`check-shellcheck`'s scan set is the gates dir plus each *kit root's* `lib/`,
`bin/`, `checks/`, and `templates/`, and the layout rule below rules `installer/`
out of the kit roots, so the coverage is owed rather than inherited and `F3` owes
it. And the codebase's single-language property is load-bearing for the audit
story — a reader reviewing what they are about to run reads one language, not
two.

*Node is not added to the floor roster.* `npx` needs Node, but the battery does
not, and adding `node` to `PROBE_SET` would make the floor contract assert a
dependency the gates do not have. The installer is an optional delivery path;
the manual vendoring path in `docs/install.md` stays, and it needs no Node at
all. The install page says which requirement belongs to which path.

*Layout.* `installer/package.json` (the real package — name, version, `bin`,
`files`), `installer/bin/checkwright.sh` (the verb dispatcher),
`installer/lib/` (the verbs), `installer/profiles.list` (the profile roster,
`B7`), `installer/README.md`, and `B6`'s smoke tree.

*`installer/` must not present as a kit root, and that constrains two names.*
`gate_kit_roots` enumerates a repo-root directory as a kit iff it carries
`checks/` **or** `smoke/` (gate-sdk/SPEC.md §lib/gate.sh) — that predicate, not
a registry, is what makes a directory a kit. So `B6`'s smoke tree is **not**
named `installer/smoke/` and the installer ships no `installer/checks/`; the
smoke tree takes any other name (`installer/consumer-smoke/` reads plainly).
This is not tidiness. Were `installer/` to enumerate as a kit root:
`gate-sdk/bin/run-consumer-smoke.sh` exits 2 on any kit root lacking
`smoke/install.sh`, and the installer's smoke is a different contract entirely;
`check-kit-registration` would demand a kit-registry row in README.md, which
`D2` deliberately does not add; `check-smoke-entry-guard` would demand the
`${SMOKE_KIT_ROOT:?}` guard of a file named `install.sh` — the likeliest name
for an *installer's* smoke; every `kit:<glob>` coupling token would expand into
`installer/`; and the payload set below, being `gate_kit_roots_rel`, would
include `installer/` itself, so the pack step would pack its own output. Every
repo-root directory carrying `smoke/` or `checks/` today is a kit, and `demo/`
carries neither — which is exactly why the demo precedent `B6` follows works.

*The payload is assembled, never duplicated.* The kit directories are not copied
into `installer/` in the tree. A pack step assembles them from the repo's own
kit roots — the set gate-sdk's `gate_kit_roots_rel` derives (gate-sdk/SPEC.md
§lib/gate.sh) — and `npm pack` runs over the assembly. A checked-in second copy
of every kit would be the parallel-copy defect at its largest scale in this
repo; deriving the payload is the derivation-first rule applied to a
distribution artifact. The pack step also stamps the version and the commit
(`C3`, `B5`).

*The assembly is staged out-of-tree, on a `*_TMP_DIR`-class knob.* The pack step
copies `installer/`'s own files and the derived payload into a scratch
directory and packs there; **no `payload/` ever exists inside the worktree**.
Gitignoring an in-tree assembly would not have been enough, because the gates
that sweep the whole tree walk the filesystem rather than the index:
`GATE_PRUNE_DIRS` prunes `target`, `.git`, `node_modules`, `.tmp`, and
`gate-tests` and nothing else, so an in-tree payload would put a second copy of
every kit's `SPEC.md` in front of canon-kit's canonical-spec finder, a second
copy of every in-flight `SPEC-*.md` in front of the amendment finder and
`check-stage-entry`'s whole-tree scans, and a second copy of every workflow in
front of `check-action-pinning` — a red battery for the duration of a pack. Out
of tree, the pack step inherits `demo/run-demo.sh`'s writes-nothing-in-tree
property, which is the same precedent `B6` follows for the smoke.

**B2. `reserve/npm/` retires into `installer/`. {mechanical}** Two
`package.json` files claiming the npm name `checkwright` is the two-sources
defect on a published identity. The reservation graduates: `installer/package.json`
becomes the one package, gaining the `bin` field the reservation deliberately
lacks and the payload the reservation deliberately had no `files` entry for.
`reserve/crates/` is untouched and stays a reservation. The version leaves
`0.0.1` and joins the repo's single semver line — docs/install.md §Versioning
rules the kits move in lockstep, and an installer with its own version line
would be a second answer to "what version am I on".

**B3. `init`. {design-bearing}**

*Preconditions, all three refusing rather than warning.* Inside a git work tree.
Worktree clean — so the commit it makes contains exactly what it wrote and a
reviewer's diff is the whole of what was vendored; `--no-commit` is the valve for
an operator who wants to stage it themselves. And `doctor` runs first: a
below-contract toolchain blocks **before any partial install**, which is the
entry's own acceptance shape and the reason `doctor` is phase 1 rather than a
convenience.

*What it writes.* The selected profile's kit directories; a `gates.list` seeded
with the gates those kits ship; the config seam files the selected kits need; and
the manifest (`B5`).

*What it commits.* One commit, its message naming the profile and the version,
then it prints the two commands that follow — `gate-sdk/bin/install-hooks.sh`
and `gate-sdk/bin/run-gates.sh`. The commit is not a convenience: vendored-committed
is the distribution model, and an installer that left the tree dirty would make
the adopter perform the step that makes the vendoring auditable.

*Re-run is idempotent and non-destructive.* A second `init` reads the manifest's
recorded per-file hash, rewrites files whose hash still matches, and **reports
rather than overwrites** a file that has changed since it was written, unless
`--force`. That is what makes "re-init is idempotent" a property rather than a
hope, and it is the phase-1 reader of the hash field (`B5`).

*`--dry-run`.* Prints the file plan and the manifest that would be written,
writes nothing, exits 0. Required on every mutating verb, phase 2's included.

**B4. `doctor`. {design-bearing}** Two behaviors selected by context rather than
by a flag: outside an installed repo it reports the toolchain verdict alone;
inside one it additionally reads the manifest and reports the installed version,
profile, and kit set. Its exit status **is** the verdict — 0 clean, 1 below
contract — so a script or a CI step can gate on it and `init` can call it as a
precondition without parsing its output.

Its dependency is the causal point of the whole unit set: it sources
`context-kit/lib/toolfloor.sh` **from the installer's payload**, not from the
consumer's tree, because at `init` time context-kit is not vendored yet. That is
why `platform-support-contract`'s amendment makes the roster a sourceable library
rather than leaving it inside an executing script, and why these two units are one
unit set. `doctor` defines no floor of its own; it renders the verdict the
roster's predicate returns.

*The ordering that follows is a constraint on build's plan, not a preference.*
`context-kit/lib/toolfloor.sh` must exist before `doctor` is written, because
`doctor` sources it out of the payload rather than reimplementing it — so the
floor unit's library batch lands **before** this unit's `doctor` batch, and a
plan that interleaves them has `doctor` sourcing a file no batch has produced.

**B5. `checkwright.lock` — the install-ownership manifest. {design-bearing}** A
tracked JSON file at the consumer root, written by `init` and parsed with `jq` —
already a roster member, so the manifest introduces no tool the contract does not
already assert. Six fields, each with its phase-1 reader:

- `schema` — the versioned wire key (`checkwright-lock v1`), the same
  versioned-contract habit evidence-kit's manifest establishes. *Phase-1
  reader*: `init` and `doctor` both refuse a schema they do not know rather than
  guessing at an unknown shape.
- `version` — the semver tag the payload was cut at. *Phase-1 reader*: `doctor`
  reports it; a re-run of `init` compares it against the payload's and refuses a
  silent downgrade.
- `commit` — the 40-hex commit the payload was assembled from, stamped at pack
  time (`B1`). *Phase-1 reader*: `doctor` prints it. This is the field that makes
  the vendoring *auditable* rather than merely committed — it is what lets a
  reviewer resolve the tree in front of them to an exact upstream state, and it
  is the same immutable-reference discipline `check-action-pinning` already
  enforces on workflow refs.
- `profile` — the profile selected. *Phase-1 reader*: a re-run of `init`
  re-applies the same profile without asking again.
- `kits` — the vendored kit set. *Phase-1 reader*: `init`'s re-run file plan and
  `doctor`'s installed-set report.
- `files` — each written path with its content hash. *Phase-1 reader*: `init`'s
  changed-file detection (`B3`).

*The hash is `git hash-object`.* Not `sha256sum`, which macOS does not ship
(it has `shasum -a 256`), and not a new roster member to paper over that: git is
already required, its object hash is stable and content-addressed, and using it
keeps the manifest's integrity story inside the toolchain the floor contract
already asserts. A portability trap avoided by reading the contract this unit set
is writing.

**B6. The consumer smoke, registered as a validate suite. {design-bearing}**
Per profile: pack the package, install it into a scratch consumer **from the
packed tarball** with no registry access, run `init`, assert the battery runs
green, assert the manifest agrees with the tree, re-run `init` and assert
idempotence, and assert `doctor` exits 0. It follows gate-sdk's consumer-smoke
mechanics (gate-sdk/SPEC.md §Consumer smoke), takes a `*_TMP_DIR`-class knob as
its only configuration, and writes nothing in-tree — the `demo/run-demo.sh`
precedent exactly.

It registers as an evidence-kit validate suite beside `demo`, for the same
reason the demo does: the activation path is a claim about what an adopter
experiences, and a claim like that rots invisibly unless a bit-rotted walkthrough
is a red validate rather than a discovery at announcement.

The tarball install is also `A2`'s proof: a package that installs and runs with
no network after the fetch is a one-shot vendoring, and the smoke is where that
stops being an assertion.

**B7. Profiles — three, one derived, and the progression gated.
{design-bearing}** The operator ruling caps them at three: starter, delegation,
full. Their definitions split by whether they are derivable:

- **`full` is derived** — every kit root in the payload, never listed. A hand-
  maintained "all the kits" roster is drift the day a kit lands.
- **`starter` and `delegation` are explicit rosters** in
  `installer/profiles.list`, because neither is derivable from the tree: each is
  a judgment about what an adopter should meet first.

The invariant, asserted by the smoke (`B6`) rather than assumed: every named kit
resolves in the payload, `starter ⊆ delegation ⊆ full`, and there are at most
three profiles. The subset chain is what makes "progressive" a contract instead
of a word — it is what guarantees that moving up a profile only ever adds.

The concrete membership is design-bearing and is derived at build from stated
criteria, not from taste: `starter` is the smallest set that reaches a green
battery with a working generated hook and at least one gate that reds on a real
consumer defect without configuration beyond the gates directory; `delegation`
adds the kits that govern agent sessions; `full` is everything. Each candidate
membership is accepted only when `B6`'s smoke passes for it, so the criterion has
an oracle.

### C. The publish channel and its supply-chain surface

**C1. A pinned, provenance-bearing publish workflow. {design-bearing}** Standing
up a package adds a distribution channel `supply-chain-trust-baseline` does not
cover, and it gets that unit's posture rather than a weaker one: a
`.github/workflows/` publish job, triggered on a version tag, running
`npm publish --provenance` with `id-token: write`, with every `uses:` ref a
40-hex commit SHA. The last is not a new rule — `check-action-pinning` walks the
whole tree and will red the new workflow on a mutable ref the moment it is
written, which is the enforcement-first pairing already in place. Nothing is
published from a laptop.

**C2. The publish is a release act, not a build act. {design-bearing}** Build
lands the package, the workflow, and the tarball-proved smoke; the **first
registry publish rides a release governed by RELEASING.md**, which gains the
publish step. Three reasons: an irreversible outward-facing act on a shared
registry is a release-stage decision and not a build session's to take; `B6`
proves the whole mechanism without it, so nothing is unverified by deferring it;
and the trust baseline that landed last iteration does not yet cover the npm
channel, so the channel is published once its posture is, not before.

**This is a ruling, not a provisional position, and it is not build's to
relitigate.** The live registry publish stays the operator's to authorize at a
release; a build session has no path to it, and a batch that finds itself wanting
one has found a scope question rather than a blocker.

The honest consequence, stated rather than glossed: until that release, the
acceptance shape is proved against a packed tarball rather than against
`npx checkwright init` from the live registry. The mechanism under test is
identical — npm resolves a tarball the same way either way — but the last mile is
a release step, and this amendment records that rather than letting build
discover it.

**C3. The package version is derived from the tag, never maintained beside it.
{design-bearing}** `docs/install.md` §Versioning rules one semver line for the
repo, so a hand-maintained `installer/package.json` version is a parallel copy of
the tag. The pack step (`B1`) rewrites it from the tag it is packing, the
committed file carries a placeholder, and `B6`'s smoke asserts the packed
version matches. RELEASING.md's publish step names the pack script rather than a
version to edit.

### D. Docs and always-loaded surfaces

**D1. `docs/install.md` gains the installer as its fast path. {design-bearing}**
The doctrine paragraph is rewritten per `A2`. The manual vendoring steps **stay**
— they are the audit story, the no-Node path, and the description of what the
installer does on your behalf — and the installer becomes the documented quick
start above them. The page states plainly what the installer does and does not
do: it copies bundled source and commits it; it fetches nothing after the package
itself; removing it leaves a working tree that needs nothing from npm ever again.

**D2. `README.md` gains a one-command quick start. {mechanical}** One command,
placed above the kit table. Deliberately narrow: `front-door-outcome-rewrite`
owns the first-screen rewrite and is **not** in this unit set, so this delta
inserts a quick-start line and preempts none of that entry's rulings about
category line, outcome framing, or table placement.

**D3. `CLAUDE.md` §Housekeeping. {mechanical}** The `reserve/` line loses its npm
half (`B2`). One new line names `installer/` as the published activation surface
and its pack-time-assembly rule, with the mechanism behind the pointer — the
always-loaded shape: one line per rule, the detail in the surface that owns it.

**D4. Registration in the registries that make a root surface governed.
{mechanical}** Two edits, not the three the
`supply-chain-trust-baseline` pattern suggests, because align verified the
third is already covered. `installer` joins `scripts/root-allowlist.list` (an
unallowlisted root entry reds `check-root-tiering`), and the installer's entry
points join `scripts/core-files.list` (silent deletion goes red). The reserve
comment in the allowlist is updated for `B2`.

**No `scripts/canon-config.sh` edit is owed.** `CANON_KIT_MANIFEST_FILES`
already carries `*/README.md`, and canon-kit expands its manifest globs as plain
single-level globs from the repo root, so `installer/README.md` joins the
governed manifest set the moment the file exists — its links and commands
resolve under the doc gates with no registration. (`reserve/*/README.md` is
listed separately only because it is one level deeper, and `reserve/crates/`
keeps that glob populated after `B2`.) Adding a literal `installer/README.md`
row would be a maintained copy of a derivation — the defect this repo's
de-literalization rule names. Recorded so build does not add the row on reflex
and so a reader does not read its absence as an oversight.

**D5. `RELEASING.md` gains the publish step. {mechanical}** Per `C2` and `C3`:
where in the release sequence the publish runs, that it runs from the workflow
rather than by hand, and that the version comes from the tag through the pack
script.

### E. What this unit deliberately does not do

**E1. The install-ownership contract is named for `plugin-marketplace`.
{design-bearing}** The manifest (`B5`) **is** the ownership contract that entry
must package against: a marketplace package that installs kits without writing
this manifest would be a second install model with no upgrade or uninstall story,
which is exactly the sequencing risk the queue entry flags. Recorded here so the
deferred entry has a named contract to point at rather than a re-derivation.

No gate over the packed payload is added, and that is a decision rather than an
omission — see `F2`.

### F. What is gated

**F1. `check-installer-no-deps` — a new consumer gate in `scripts/`.
{design-bearing}**

*Invariant.* `installer/package.json` declares no `dependencies`,
`peerDependencies`, or `optionalDependencies`, and no install-time lifecycle
script (`preinstall`/`install`/`postinstall`). Those are the two shapes that
would turn the package into a resolved-dependency channel or a run-on-install
code path — precisely the two things `A2`'s rewritten doctrine claims it is not.
An unbacked doctrine restatement is the overclaim class the previous iteration
finished removing, so the doctrine gets a reader.

*Placement — consumer, not kit.* It gates this repo's own published distribution
artifact. No vendoring consumer has an `installer/`, so a kit gate would ship
inert to everyone downstream; the seam rule is that a kit ships generic
mechanism, and "this repo publishes an npm package with these properties" is not
generic. It registers in `scripts/gates.list` beside the repo's other
consumer gates.

*Contracts.* `precommit` tier; a `# graph:` manifest coupling
`installer/package.json`; a good/bad fixture pair reached through a positional
argument (the form `check-install-toolchain` already uses); the output and
fail-closed contracts of gate-sdk/SPEC.md §The gate model. It reads one tracked
JSON with `jq` and is hermetic.

**F2. No gate over the packed payload — considered and declined.
{design-bearing}** The obvious sibling gate would assert that the packed payload
equals the tree's kit roots. It is declined: the payload only exists at pack
time, so the gate would have to run `npm pack` at commit time, which breaks the
hermetic-gate contract and puts a network-capable toolchain in the pre-commit
path. The assertion moves into `B6`'s smoke, which already packs — where it costs
nothing extra and runs in the tier that can afford it. Recorded so build does not
re-derive the question, and so a later reader does not read the absence as an
oversight.

**F3. The installer's scripts are brought under `check-shellcheck`.
{design-bearing}** `B1`'s language ruling rests on the installer being linted,
and `B1`'s own layout rule keeps `installer/` out of the kit roots, so nothing
lints it by default — the gap this delta closes, and the reason `B1` states the
claim as owed rather than inherited. The indicated mechanism is a lint-scope knob
on gate-sdk (the scan set gains consumer-named directories on top of the gates
dir and the kit roots), set for `installer/bin` and `installer/lib` in this
repo's `scripts/gate-sdk-config.sh` — the config seam's existing job. A knob
carries the usual tail: the roster entry and default in gate-sdk/SPEC.md
§Layout and configuration, and the invariant in §check-shellcheck restated to
name the added set, both of which `check-knob-citation` and
`check-knob-default-coupling` read.

Two constraints on executing it. The knob **adds to** the default scan set and
never replaces it, so a consumer that sets nothing keeps today's coverage
exactly. And `demo/run-demo.sh` sits in the same blind spot today — it is a
shipped script under no kit root — so it joins the same knob value; it is
already clean at `-S warning`, verified, so the widening costs nothing and
leaving it out would mean landing the mechanism that fixes a gap beside the gap.

**F4. The regen tail. {mechanical}** A new gate, a new root surface, and a new
kit knob move the
fixed set of generated projections, each naming its regen command on a red: the
pre-commit hook (`gen-pre-commit.sh --write`), the graph artifact
(`check-graph.sh --emit > docs/check-graph.html`), the enforcement map
(`enforcement-map.sh --emit > docs/enforcement.md`), the docs mirrors
(`gen-docs-mirror.sh --write`), and the footprint plus the value rollup that
reads it — new always-loaded lines and a new governed surface both carry token
cost. Executing this is running commands until the battery is green.

## Producers and consumers

- **The manifest `checkwright.lock` (`B5`).** *Producer*: `init`, in the
  consumer's repo, on every run — including the re-run, which rewrites it. Its
  enabling configuration is nothing beyond the verb being invoked, so the
  producer is reachable by construction rather than gated behind a flag nobody
  sets. *Consumers*, all phase-1 and all named per field above: `init`'s own
  re-run path and `doctor`. Phase 2's `update`, `diff`, and `uninstall` are
  second readers of the same six fields and add none — the constraint `A1`
  states. *Transition*: every field is read at the same transition it would be
  written at (the next `init` or `doctor` invocation), so no field is populated
  at a transition where nothing reads it.

- **The floor roster, read across a component boundary (`B4`).** *Producer*:
  `context-kit/lib/toolfloor.sh`, created by `platform-support-contract`'s
  amendment, assembled into the payload by `B1`'s pack step. *Consumer*:
  `doctor`, sourcing the payload copy. The enabling condition is `B1`'s
  derivation — if the pack step's kit set ever excluded context-kit, `doctor`
  would have a named producer no packaging actually delivers, which is the dead
  producer the causal-completeness check exists to catch. `B6`'s smoke runs
  `doctor` in a scratch consumer for **every** profile, so the delivery is
  asserted for the smallest profile too, not only for `full`.

- **The vendored payload (`B1`).** *Producer*: the pack step, reading
  `gate_kit_roots_rel` — so the payload's kit set has the same owner the battery
  does and cannot drift from it. *Consumer*: `init`, copying from it into the
  consumer's tree. *Second consumer*: `B6`'s smoke, which is what makes the
  producer's correctness observable rather than assumed.

- **The publish workflow (`C1`).** *Producer*: a version tag pushed to the
  repository, under `C2`'s release step. *Consumers*: npm's registry, which
  receives the tarball and the provenance attestation, and
  `check-action-pinning`, which reads the workflow's `uses:` refs at commit and
  CI time — the reader that keeps the new workflow from being the one file where
  the pinning rule silently lapses.

- **`check-installer-no-deps` (`F1`).** *Producer*: its registration in
  `scripts/gates.list`, which is what makes the runner resolve it — a gate file
  no registry names runs nowhere. *Consumers*: the full battery, the generated
  pre-commit hook through its `# graph:` manifest, and the CI backstop. The
  manifest has the further reader `F4` names: the graph artifact and the
  enforcement map both project from it.

- **The registry rows (`D4`).** *Producers*: the two list files.
  *Consumers*, one gate each: `check-root-tiering` reads the root allowlist and
  `check-core-files` reads the core-files list. A root surface in neither reds
  the first and is silently deletable past the second. Canon-kit's doc gates are
  a third consumer with **no producer to add** — they read
  `CANON_KIT_MANIFEST_FILES`, whose `*/README.md` glob already resolves
  `installer/README.md`, so the edge exists by derivation rather than by a row.

## Existing sections updated

- **docs/install.md** — the opening doctrine paragraph (`A2`), the quick-start
  installer path above the vendoring steps (`D1`), and a sentence in the
  Requirements area distinguishing the installer path's Node requirement from the
  battery's toolchain (`B1`). The §Versioning section is unchanged and is what
  `C3` derives from.
- **RELEASING.md** — the publish step (`D5`).
- **README.md** — the quick-start line (`D2`).
- **CLAUDE.md §Housekeeping** — two edits, and the first is a **repo-root
  convention change**, not a wording tidy. The line today rules that `reserve/`
  holds the crates.io *and* npm name-reservation placeholders and is not to be
  developed in; `B2` retires the npm half out of it, so the line is rewritten to
  govern the crates reservation alone. Development moves to a root surface the
  line does not currently admit exists, which is why this is stated here rather
  than left for build to discover mid-batch. The second edit is `D3`'s new
  `installer/` line.
- **gate-sdk/README.md §Gate roster** — `F1`, as a **no** edit:
  `check-installer-no-deps` is a consumer gate in `scripts/`, not a gate-sdk
  gate, so the kit's roster block (held bidirectionally by
  `check-readme-roster` against `gate-sdk/checks/`) must **not** gain a row.
  Named here because the reflex from the last new-gate unit is to add one.
- **README.md §This repo, governed** — the per-kit fixture-runner battery roster
  gains `F1`'s consumer-gate fixture pair and `B6`'s validate suite, since that
  section owns the roster a session runs before committing.
- **gate-sdk/SPEC.md §Layout and configuration and §check-shellcheck** — `F3`:
  the knob's roster entry and default, and the lint invariant restated to name
  the consumer-added directories. The only gate-sdk contract either amendment
  changes, which is why it is stated here rather than left to a batch.
- **context-kit/SPEC.md §bin/env-probe** — no edit from this amendment; the
  sourceable-library change is `platform-support-contract`'s and is cited by
  `B4` as a dependency, not restated.
- **scripts/root-allowlist.list, scripts/core-files.list** — `D4`.
  `scripts/canon-config.sh` is **not** in this list: `D4` records why no edit is
  owed there.
- **scripts/gates.list** — `F1`'s registration row. A gate file no registry
  names runs nowhere, so this is the delta that makes the gate real rather than
  a tail on it.
- **scripts/gate-sdk-config.sh** — `F3`'s knob value for `installer/bin`,
  `installer/lib`, and `demo`.
- **The local ops runbook** — `C1`: its repo-settings desired state gains the
  publish workflow's required secret and permissions, so the desired-state table
  and the live repo agree. Local-only file, no tracked diff, and leaving it
  stale re-litigates the setting at the next ops pass.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain
      (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. Specifically: no surface still states the
      registries hold the name only, and no second `package.json` claims the npm
      name.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
