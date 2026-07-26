# SPEC amendment: publish-spec-gate

A root-level amendment: the ruling is repo governance over `.github/workflows/`
with no owning kit component, and the gate it introduces is a **consumer gate
under `scripts/`** rather than kit mechanism. It promotes the queue entry
`publish-spec-gate` and is the enforcement half of the debt entry
`publish-spec-disambiguation` — the two land in one commit, which is what the
always-loaded enforcement-first rule asks for.

**Provenance seam.** Nothing private crosses. npm's spec-resolution grammar is
public vocabulary, the scanned surface is a tracked workflow file, and no
account, credential, or registry identity is named — the publish job
authenticates through a repository secret and this gate never reads one.

**Why a consumer gate and not gate-sdk mechanism.** The rule is *npm* knowledge,
and npm is a channel this repo happens to publish through, not a property of a
governed tree. A kit consumer with no npm package would carry a gate that can
never fire on anything real. gate-sdk's counted-inertness pattern would make
that harmless but not useful, and the kit-landing checklist would owe it a
README roster row and a SPEC section for a rule no consumer asked for.
`scripts/` is where this repo's own product knowledge already lives
(`check-release-bump` reads this repo's release-note grammar; `check-installer-no-deps`
reads this repo's package). This joins them. The sibling amendment
`gate-sdk/SPEC-action-run-shell.md` rules the *opposite* way for its gate, and
the two rulings are consistent: a `run:` block is shell in anybody's workflow;
an `npm publish` spec is shell in this one's.

## What changes

Every delta carries its work class. **mechanical** — executing it demands only
oracle-running (a fixed battery, a substitution sweep, a regen command).
**design-bearing** — executing it demands generative or verificational judgment.

### A. `check-npm-publish-spec` — a new consumer gate

**A1. The predicate, and the trap it must not fall into. {design-bearing}**

This is the one place this iteration can ship a gate that looks right and is
not, so the predicate is stated before the mechanism.

npm resolves a positional package spec as a **path** when it begins with `.` or
`/`, and as the GitHub shorthand **`owner/repo`** otherwise. The trigger is the
**leading character, not the slash.** `publish-spec-disambiguation` carries the
reproduction against a real packed tarball under `npm publish --dry-run`:
`dist/x.tgz` exits 128 (npm shells out to `git ls-remote`), while `./dist/x.tgz`,
an absolute path, **and** `.tmp/pubrepro/dist/x.tgz` — three slashes, leading
dot — all exit 0.

So the predicate is **not** "contains no slash", and it is **not** "starts with
`./`". Either simplification reds a spec that works, which is a false positive
under gate-sdk/SPEC.md §When a gate earns its place, and the second one reds the
very `$PWD`-prefixed form the paired debt entry prescribes.

The predicate applies to the token **with one layer of surrounding shell quoting
removed** — `"…"` or `'…'`. This is not a detail: every real spec on this
surface is quoted, including the shipped defect (`"$(ls dist/*.tgz)"`) and the
prescribed fix (`"$PWD/${tarballs[0]}"`). A predicate that reads the raw token
sees `"` as the first character and reds `"./dist/x.tgz"`, a safe form — the
common case, not a corner. Strip first, then decide.

A positional spec is **unambiguous** when one of these holds:

1. its first character is `.` or `/` — npm's own path rule, verbatim; or
2. it begins with an expansion of a **proven-absolute root** immediately
   followed by `/`, in bare (`$PWD/`) or braced (`${PWD}/`) form. The roster is
   exactly three: `PWD`, `GITHUB_WORKSPACE`, `RUNNER_TEMP`.

   Each of the three is absolute **by a written contract**, which is what
   entitles arm 2 to call itself a proof: POSIX gives `PWD` as "an absolute
   pathname of the current working directory", and the GitHub Actions
   default-environment-variable table documents `GITHUB_WORKSPACE` and
   `RUNNER_TEMP` as runner-absolute paths. **`HOME` was proposed for this roster
   and is excluded on the evidence**: POSIX defines it as "a pathname of the
   user's home directory" with no absoluteness guarantee, and it is not in the
   Actions default-variable contract at all — so it is absolute in practice and
   unproven on paper. Admitting it would apply a looser evidentiary standard
   than the bare-filename ruling two paragraphs below rejects, inside the same
   section. It also has no use site on this surface. Re-proposing it needs a
   contract citation, not a runner observation.

Everything else reds, and the two cases worth naming because they look safe are:

- **A bare filename** (`x.tgz`, no slash). It happens to work today, which is
  why the defect survived review — but only because a file of that name exists
  in the runner's cwd at that moment. npm's resolution there depends on a
  runtime property no gate can see, so the line is lucky rather than correct.

  Reddening a *working* line is the one place this gate brushes the
  low-false-positive contract in gate-sdk/SPEC.md §When a gate earns its place,
  so the reasoning is recorded rather than assumed. It clears the bar on three
  counts: the gate's invariant is **unambiguity**, not breakage, and the bare
  filename genuinely is ambiguous, so the red is a true statement about the
  property being checked; unambiguity is a real drift axis rather than the
  trivially-true proxy that section bars; and the false-positive *friction* the
  section weighs is two characters, with a fix that is always available and
  always correct. What would break the contract is a **failure message that
  overclaims** — text asserting the line will fail is false today, and the
  maintainer who tests it and finds it working learns to distrust the gate.
  `A5` binds the message accordingly.
- **Any command substitution** (`"$(ls dist/*.tgz)"`). The gate cannot evaluate
  it, and this is the exact shape that shipped in `v0.16.0`.

**A2. Reach — `npm publish` alone, and why widening it manufactures false
positives. {design-bearing}** The queue entry leaves open "whether the gate
reaches `npm publish` alone or every path-taking npm verb". Ruled: **`publish`
alone**, on a discriminator rather than on caution.

`npm publish`'s positional argument is by definition a *local* package — a
tarball or a directory — and never a registry spec, so every positional
argument it carries is path-intended and `A1`'s predicate arms with no
ambiguity. Every other npm verb takes registry specs as its ordinary case:
`npm install lodash` is a bare token with no slash and is entirely correct, and
`A1` applied to it would red a correct line on its first run. The reach is
therefore a property of which verb makes the predicate total, not a hedge — and
recording that here is what stops the next reader re-litigating it.

**A3. Scan set and extraction — no extraction at all. {design-bearing}** The
gate scans **lines** of `.github/workflows/*.yml` directly. An `npm publish`
invocation is line-local, so nothing is extracted from YAML and this gate takes
on none of the fidelity surface its sibling amendment spends a whole section on.
That independence is deliberate: this unit is on the critical path to a working
`v0.16.1` publish and must not serialize behind an extractor.

Two reach limits, both stated rather than discovered:

- **A continued line is refused, not missed.** A matched line ending in a
  backslash continuation reds at **exit 2** naming the construct, so the
  line-local premise can never decay into a silent false negative. Fail-closed
  per gate-sdk/SPEC.md §The gate model.
- **Shell scripts under the tree are out of reach, with cause.** A `.sh` file's
  publish spec is typically a variable (`installer/consumer-smoke/run-smoke.sh`
  passes `"$TARBALL"`, absolute in fact but unprovable from the text), and
  reddening it would break the low-false-positive contract. The workflow surface
  is where the spec is written as a literal, which is what makes it gateable
  there and nowhere else.

**A4. Token parsing — the flag-value hazard. {design-bearing}** The positional
spec is not simply "the last token": `npm publish --access public` ends in a
flag *value*. The gate walks the invocation's tokens, skipping any token
beginning with `-` and any token immediately following a value-taking flag
(`--access`, `--tag`, `--otp`, `--registry`, `--workspace`). What remains is the
candidate set; an empty set is clean (npm publishes the cwd, no spec to judge),
and **more than one** candidate reds at exit 2 — npm publish accepts at most one
positional, so two means the parse is wrong and the gate says so instead of
guessing.

**A5. Gate contracts. {mechanical}** `precommit` tier; registration in
`scripts/gates.list`; a `# graph:` manifest coupling `.github/workflows/*.yml`;
a `# spec:` header line binding this section's successor in the canonical
surface; the output and fail-closed contracts per gate-sdk/SPEC.md §The gate
model. The gate takes a positional scan-root argument, the form
`check-action-pinning` and `check-release-bump` already use, so the fixture pair
reaches a synthetic tree. A tree with no `npm publish` line exits clean with a
zero count.

**The failure text states the ambiguity, never a predicted failure** (`A1`). It
names the spec, says its resolution depends on runtime state rather than on the
literal, and gives the unambiguous form — because for the bare-filename arm a
"this will fail" message is factually wrong, and a gate that is wrong in its own
message is the false positive `A1` argues the red is not.

### B. The fixture pair, which must do real work

**B1. `good/` carries a slash-bearing safe spelling. {design-bearing}** This is
the fixture's whole point and the reason `A1` is spelled out above. The `good/`
tree contains, at minimum:

- `./dist/x.tgz` — safe, slash-bearing, and the form a "no slash" predicate reds;
- `"./dist/x.tgz"` — the **same spec double-quoted**, which is what pins `A1`'s
  quote-stripping clause. A predicate that reads the raw token sees `"` and reds
  this row, and since every real spec on this surface is quoted, that failure
  mode would ship as the gate's ordinary behaviour rather than as an edge case;
- `.tmp/pubrepro/dist/x.tgz` — **three slashes and a leading `.`**, verified to
  exit 0. This one row disproves *both* wrong predicates at once: it is not
  slash-free, and it does not start with `./`. A future author who "simplifies"
  the predicate to either shape turns the good fixture red;
- `"$PWD/${tarballs[0]}"` — the prescribed form from the paired debt entry, so
  the gate that guards the fix cannot red the fix. It exercises the quoting
  clause and arm 2 together, which is the exact composite the live workflow will
  carry;
- `npm publish --provenance --access public` with no positional — the arity-zero
  arm that proves `A4` does not mistake `public` for a spec.

**B2. `bad/` carries the shipped defect verbatim. {design-bearing}**
`npm publish --provenance --access public "$(ls dist/*.tgz)"` — the `v0.16.0`
line as released. A regression that reintroduces it reproduces a failure that
already reached a tag, which is the strongest thing a bad fixture can assert.
The expectation file matches on the gate's failure text, not on an exit code
alone, per the fixture contract.

### C. The regen tail

**C1. {mechanical}** A new consumer gate moves a fixed set of generated
projections, each naming its own regen command on a red: the pre-commit hook
(`gen-pre-commit.sh --write`, driven by `A5`'s `# graph:` manifest and tier),
the graph artifact (`check-graph.sh --emit > docs/check-graph.html`), the
enforcement map (`enforcement-map.sh --emit > docs/enforcement.md`), and the
footprint plus value rollup that a new tracked file moves. Executing this is
running commands until the battery is green.

## Producers and consumers

No new state, event, or message is introduced — this unit adds one gate script,
one registry row, and one fixture pair. The causal chains that exist:

- **The unambiguous spec form (`A1`).** *Producer*: a maintainer editing
  `.github/workflows/publish.yml` — concretely, the `publish-spec-disambiguation`
  edit landing in the same commit as this gate. *Consumers*, two, and both must
  exist or the form rots. npm resolves the literal at publish time — the
  functional reader, and the one that exited 128 on `v0.16.0`. And
  `check-npm-publish-spec` reads the same literal at commit and CI time — the
  reader that keeps the fix from silently regressing on the next workflow edit.
  A fix with only the first reader is a one-off repair; this gate is what makes
  it a contract.

- **`check-npm-publish-spec`'s own wiring (`A5`).** *Producer*: the row in
  `scripts/gates.list`, which is what makes `run-gates.sh` resolve and run it —
  a gate file no registry names runs nowhere. *Consumers*: the full battery
  (`gate-sdk/bin/run-gates.sh`), the generated pre-commit hook (through the
  `# graph:` manifest and the `precommit` tier), and the CI backstop
  `.github/workflows/gates.yml`, which runs both the battery and the derived
  fixture suites. The `# graph:` manifest carries a third reader: `check-graph`'s
  artifact and the enforcement map both project from it, which is why `C1` is a
  delta and not housekeeping.

- **The fixture pair (`B1`/`B2`).** *Producer*: the `scripts/gate-tests/check-npm-publish-spec/`
  tree on disk. *Consumer*: `gate_fixture_suites`, which derives the suite roster
  from the gate-tests dirs with no hand-list — so the new pair joins the CI
  fixture step by existing, and `check-gate-fixture-coverage` reds a gate that
  ships without one.

- **The proven-absolute root roster (`A1`.2).** *Producer*: a literal set in the
  gate script. *Consumer*: the predicate itself, at match time. It stays a script
  literal rather than a config knob: this is a consumer gate, so the vocabulary
  is already consumer-side and a knob would add a seam with one possible value.
  `RUNNER_TEMP` and `GITHUB_WORKSPACE` are GitHub Actions runner contract names,
  which is public vocabulary and crosses no seam.

## Existing sections updated

- **`.github/workflows/publish.yml`** — the `npm` job's publish step carries the
  glob-into-an-array form, the arity assertion, and the load-bearing comment
  from the `publish-spec-disambiguation` entry. **The job split and the three
  `uses:` SHA pins are untouched**: that file's own header rules the split
  load-bearing (`pack` assembles and stamps once; every channel is a sibling job
  that `needs: pack`), and `check-action-pinning` walks the file.
- **`scripts/gates.list`** — the registration row from `A5`.
- **`docs/enforcement.md`, `docs/check-graph.html`, `scripts/git-hooks/pre-commit`,
  `docs/footprint.md`** — generated projections, regenerated through their owning
  commands per `C1`, never hand-edited.
- **`docs/posts/2026-07-26-checkwright-v0-16-0.md`** — the dated erratum above
  the install command, owned by the paired debt entry and named here only so the
  two halves are not split across commits.
- **This repo's release note for the shipping version** — a Tightened-gates
  bullet for the new gate, per docs/install.md §The upgrade contract; authored at
  close, not here.

Deliberately **not** updated: `README.md:26` and `docs/install.md:117`, which say
`npx checkwright init` unpinned. Those self-correct the moment the publish lands,
since the unpinned spec resolves to `latest`. Naming them here stops a build
session from editing a claim that is about to become true.

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
      retired; nothing dangles.
- [ ] **The good fixture disproves both wrong predicates** — the slash-bearing
      safe spellings from `B1` are present, and narrowing the predicate to
      slash-detection or to a `./` prefix reds `good/`. Verified by running the
      suite against a deliberately-narrowed predicate before shipping the real one.
- [ ] **The quoting clause is pinned by the fixture** — `good/` carries a
      double-quoted safe spec, and dropping `A1`'s quote-stripping step reds it.
      Verified by making that removal before shipping.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
