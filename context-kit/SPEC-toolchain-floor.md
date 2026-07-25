# SPEC amendment: toolchain-floor

A context-kit amendment: the kit owns `PROBE_SET`, so it owns the version-floor
axis added to it. The change reaches this repo's `scripts/check-install-toolchain.sh`
(the parity gate over that roster), `docs/install.md` (the page whose
Requirements block is the roster's other half), and `gate-sdk/README.md` (the
page carrying the contradicting floor claim today).

This is the floor half of `platform-support-contract` and nothing else. The
per-platform CI install-smoke legs were carved out at promotion as
`platform-support-ci-matrix` and stay deferred; no delta here adds, changes, or
plans a CI leg.

**Provenance seam.** Nothing here approaches it. The roster is the kit's own
dependency set — `bash`, `git`, `jq`, `awk`, `sort`, `shellcheck` are public
toolchain vocabulary, not consumer rule content, and the roster is a legitimate
kit literal for the same reason it already is one: it names what the kit's own
scripts invoke. The one machine-specific output the axis produces — a per-box
verdict — lands in the gitignored, consumer-local profile file the probe already
writes (§bin/env-probe's content seam), never in a tracked surface.

## What changes

Every delta carries its work class. **mechanical** — executing it demands only
oracle-running (a fixed battery, a substitution sweep, a regen command).
**design-bearing** — executing it demands generative or verificational judgment.

### A. The floor axis and its owner

**A1. `context-kit/lib/toolfloor.sh` — a sourceable owner for the roster.
{design-bearing}** `PROBE_SET` moves out of `bin/env-probe.sh` into a small
library that defines the roster and the comparison predicate and executes
nothing else. The name is deliberately **not** changed: the array keeps the
spelling `PROBE_SET`, so the parity gate's existing `^PROBE_SET=\(` anchor
survives and the release note carries no rename.

The move is forced, not cosmetic. `env-probe.sh` is a script that does its work
on execution, so a second reader cannot obtain the roster by sourcing it — which
is why the parity gate greps the array out of the file today, and why a third
reader would have to grep it too. The third reader is the sequencing the unit set
already carries: `launch-activation-cli`'s `doctor` runs in a repo where
context-kit is not vendored yet, so it must read the roster out of the
installer's own payload copy, before any consumer file exists. A sourceable
owner serves that reader; a script does not.

**A2. The floor token grammar. {design-bearing}** A roster element becomes
`<name>[:<min-version>[:<impl-token>]]`. Both trailing fields are optional and
a bare name keeps today's meaning exactly — "must be present, no version
constraint" — so the axis is *per-member*, not a number demanded of every
member.

That optionality is the design. `docs/install.md` today refuses floors with a
real argument: "a version floor baked into this page would rot." The refusal is
right about aspirational floors and wrong about forced ones, and the grammar
encodes the distinction: **a member gains a floor only when a construct the
battery actually uses forces it, and the roster records that construct beside
the token.** A floor nobody's code forces is not pinned. This is the
de-literalization doctrine applied to a version number — the value is owned
where the constraint is provable, and prose cites it.

The third field exists because one constraint is not a number. `awk`'s
requirement is an implementation family, not a version: the forcing construct is
gawk's 3-arg `match()`. The token is matched as a substring of the tool's own
version banner (gawk prints `GNU Awk`, GNU sort prints `sort (GNU coreutils)`),
so the constraint is checked against the binary actually on `PATH` rather than
against a package name nobody can probe.

**A3. The roster's initial content, each floor citing its forcing construct.
{design-bearing}** Verified by tree-wide grep this session; build re-verifies
before pinning and pins nothing it cannot cite.

- `bash:4.0` — three independent bash-4.0 constructs: `declare -A` (gate-sdk,
  guard-kit, delegation-kit, evidence-kit checks), `mapfile` (widespread across
  the kits), and the `${x,,}` case expansion (canon-kit's `lib/spec.sh` and
  checks, a delegation-kit template). This is the floor `gate-sdk/README.md`
  already asserts, now carrying its justification and a reader.
- `awk::GNU` — no version floor, one implementation constraint: 3-arg `match()`
  in `gate-sdk/checks/check-gate-assertions.sh`, whose own header comment and
  gate-sdk/SPEC.md §check-gate-assertions already name the dependency. That
  sentence in the SPEC is the token's citation target and stays where it is; it
  is not a duplicate to strike.
- `sort::coreutils` — a **new roster member**, and the only one. GNU coreutils
  is forced three ways and probed nowhere today: `sort -V`
  (`scripts/check-release-bump.sh`), `date -d` (drift-kit's KPIs and
  `bin/trajectory.sh`, a delegation-kit template), and `stat -c`
  (`delegation-kit/bin/usage-verdict.sh`). None is BSD-portable. One member per
  package family, the representative being the binary carrying a forcing
  construct — `sort`, which is also the predicate's own comparison tool (A4).
  This is a member added to the existing roster, not a second roster: the whole
  point of the entry's verified premise.
- `git`, `jq`, `shellcheck` — bare names, unchanged. No construct found this
  session forces a version on any of them, so none is pinned. The jq usage is
  1.5-era throughout (`walk`, `-r/-c/-e/-s/-n/-R`); if build finds a construct
  that forces a number, the token gains it with the citation, and otherwise it
  does not.

**A4. `tool_floor_check` — the comparison predicate. {design-bearing}** In the
same library: given a roster element and the probed version string, it returns
one verdict from a closed set — `ok`, `absent`, `below <found> <floor>`,
`wrong-impl <found>`, `uncomparable`. Numeric comparison is `sort -V`, which is
itself a roster member, so the predicate has no dependency the contract does not
already assert. `uncomparable` is the fail-closed arm: a version banner the
predicate cannot parse, or a `sort` without `-V`, is reported as unverified and
never silently reported as `ok` — the same fail-closed posture gate-sdk/SPEC.md
§The gate model requires of a gate, applied to a probe that is not one.

**A5. `bin/env-probe.sh` renders the verdict. {design-bearing}** The script
sources the library rather than defining the roster, and its generated block
carries the floor beside each probed version — `` - `bash` — 5.2.37 (floor 4.0,
ok) ``, `` - `awk` — mawk 1.3.4 (requires GNU — below contract) `` — plus a
`**Below contract:**` line beside the existing `**Absent:**` line, reading
`none` when clean.

This is what gives the floor axis a named reader **inside this unit**, which is
the causal-completeness point: the floor must be consumed by something that
exists when this amendment merges, not only by the `doctor` a sibling unit
ships. The session-context hook's step-9 profile emit is the downstream reader
of that block and needs no change — it emits whatever the block holds
(§The session-context hook).

### B. The parity gate covers the new axis

**B1. `check-install-toolchain` asserts triple parity. {design-bearing}** The
gate's invariant widens from name-set parity to parity over the whole element:
name, floor, and implementation token must agree between `docs/install.md`'s
marker block and the roster. A page listing `bash` while the roster says
`bash:4.0` is now a red, which is precisely the drift class this unit exists to
close — the contradiction it is fixing is a floor stated in one place and denied
in another.

The gate keeps **grepping** the roster rather than sourcing the now-sourceable
library. A gate takes a fixture path positionally (`check-install-toolchain.sh
[install-md] [probe-script]`) and a fixture is untrusted input: sourcing it would
execute `bad/` fixture content inside the gate. Sourceability serves the probe
and the installer's `doctor`; the gate stays a reader that cannot be made to run
what it lints.

**B2. The install-page bullet grammar. {design-bearing}** Each bullet in the
`toolchain:begin`/`toolchain:end` block renders its constraint in a
parenthetical the gate parses: `` - `bash` (≥ 4.0) — … ``, `` - `awk` (GNU) — … ``,
`` - `git` — … `` for an unconstrained member. The prose after the dash keeps its
present job — what breaks without the tool — and gains, for a constrained
member, the construct that forces the constraint, so a reader who asks "why 4.0"
gets the answer on the page rather than in a commit message.

**B3. The gate's `# graph:` manifest. {mechanical}** The manifest's `couples=`
moves from `context-kit/bin/env-probe.sh` to the library (and keeps
`docs/install.md`), so the hook fires on an edit to the roster's real owner.

**B3a. The fixture pair covers the widened invariant. {design-bearing}** The
good/bad pair gains a floor-mismatch case: today's pair can only express a name
mismatch, and a widened invariant with an unwidened bad fixture is the
trivially-passing shape gate-sdk/SPEC.md §The gate model bars. Authoring a bad
fixture that fails for the new reason and only the new reason is generative
work, which is why it is split from `B3`'s manifest edit rather than riding it.

**B4. The regen tail. {mechanical}** A `# graph:` manifest edit and a SPEC edit
move the fixed set of generated projections, each naming its regen command on a
red: the pre-commit hook (`gen-pre-commit.sh --write`), the graph artifact
(`check-graph.sh --emit > docs/check-graph.html`), the enforcement map
(`enforcement-map.sh --emit > docs/enforcement.md`), the on-site SPEC/README
mirrors (`gen-docs-mirror.sh --write`, which carries C3's `gate-sdk/README.md`
edit and this amendment's merged SPEC text), and the footprint plus the value
rollup that reads it. Executing this is running commands until the battery is
green.

### C. The published contradiction, resolved

**C1. `docs/install.md` — the no-floors sentence is replaced by the ruling.
{design-bearing}** The paragraph beginning "No minimum versions are pinned here"
goes. What replaces it states the rule the grammar encodes: floors are pinned
only where a construct in the battery forces one, each pinned floor names that
construct, and the roster is the owner this page renders — so the page cannot
drift from the code and a floor cannot rot into an aspiration. The env-probe
pointer stays and gains the verdict: the local profile now says whether this box
meets the contract, not only what it carries.

**C2. `docs/install.md` — the platform paragraph states the macOS reality.
{design-bearing}** The page declares macOS supported while the battery requires
bash 4 and GNU userland, and stock macOS ships bash 3.2 and BSD `sort`, `date`,
`stat`. The paragraph is rewritten to say what is actually true: the engine is
portable to any Unix that presents a GNU-first toolchain, and on macOS that is
an adopter action — installing GNU bash, coreutils, and gawk and putting them
ahead of `/usr/bin` on `PATH` — not something the stock system delivers. That is
the engine-portability-versus-full-harness-experience distinction the entry
asked for, delivered as an honest support statement.

Its honest limit is stated with it, because the probe cannot close it: the
roster asserts what is on `PATH` at probe time. A macOS box with Homebrew
coreutils installed but not `PATH`-ordered probes BSD `sort` and reports below
contract — correctly, since that is what the gates will invoke.

This delta asserts nothing about per-platform CI. The claim it makes is that the
requirement is stated truthfully; the claim that it is *tested* per platform is
`platform-support-ci-matrix`'s and is not made here.

**C3. `gate-sdk/README.md` §Requirements stops restating the toolchain.
{mechanical}** The line "bash 4+, git, GNU coreutils/findutils, GNU awk
(`check-gate-assertions`), ShellCheck (`check-shellcheck`)" is a parallel copy of
a roster that now has one owner and a gate — the one-owner-per-fact rule. It
becomes a pointer to the install page's Requirements block. The `findutils`
clause does not survive the move regardless: no GNU-only find predicate was found
this session (`gate-sdk/lib/gate.sh`'s `gate_find` is a POSIX
`-prune -o … -print` form, and no `-printf` appears in the tree), so it is an
unbacked claim — build re-verifies and, on the same finding, the claim is dropped
rather than rostered. Dropping an unbacked requirement is the same de-claim
discipline `supply-chain-trust-baseline` applied to the pinned-parser phrase.

## Producers and consumers

No new message or event is introduced. One new state — a per-tool floor verdict
— plus one relocated interface and one widened gate invariant.

- **The roster (`A1`–`A3`).** *Producer*: `context-kit/lib/toolfloor.sh`, a
  tracked kit file, read by every consumer at source-or-grep time; its enabling
  configuration is nothing, which is the point of a literal over a knob — the
  roster is the kit's own dependency set and a consumer who could override it
  could only make the contract lie. *Consumers*, three, and all three exist or
  are named: `bin/env-probe.sh` sources it (`A5`);
  `scripts/check-install-toolchain.sh` greps it (`B1`); and
  `launch-activation-cli`'s `doctor` sources the payload copy — that unit's
  amendment names this file as its dependency, so the cross-unit edge is
  declared on both sides rather than assumed.

- **The floor verdict (`A4`, `A5`).** *Producer*: `tool_floor_check`, called by
  `env-probe.sh` once per roster member on every probe run — and the probe runs
  per session through the session-context hook's step-9 re-probe
  (§bin/env-probe's cadence), so the producer is reachable on a real cadence and
  not only when someone runs it by hand. *Consumer*: the generated block in the
  consumer-local profile file (`CONTEXT_KIT_ENV_PROFILE_FILE`), read by the
  session-context hook's step-9 emit and by the operator. The verdict is not
  gated and deliberately so: env truth is not cheaply machine-verifiable from
  inside the repo, which is the same carve-out §bin/env-probe already records for
  the probe itself.

- **The verdict's field set.** `ok` has a reader (the profile block renders it as
  the clean state and the `Below contract` line reads `none`). `absent` has one
  (the existing `**Absent:**` line, unchanged). `below` and `wrong-impl` are read
  by the new `**Below contract:**` line, which distinguishes them because the
  remedies differ — upgrade versus install a different implementation.
  `uncomparable` is read there too, as an explicitly unverified entry, because
  folding it into `ok` is the silent-pass this predicate exists to refuse.

- **The parity assertion (`B1`, `B2`).** *Producer*: the widened gate, registered
  in `scripts/gates.list` already — no registration change, so the battery and
  the CI backstop pick it up unchanged. *Consumers*: the generated pre-commit
  hook, through the `# graph:` manifest `B3` edits, and the full battery. The
  manifest has the second reader `B4` names — `check-graph`'s artifact and the
  enforcement map both project from it.

- **The published statement (`C1`–`C3`).** *Producer*: the hand edits to
  `docs/install.md` and `gate-sdk/README.md`. *Consumer*: `gen-docs-mirror.sh`,
  which regenerates `docs/gate-sdk/README.md` from the kit README with
  `check-docs-mirror-fresh` byte-gating it — so the mirrored copies at
  `docs/gate-sdk/README.md` and `docs/gate-sdk/SPEC.md` are derived readers of
  the edit, never a second place to edit.

## Existing sections updated

- **context-kit/SPEC.md §bin/env-probe** — "What it probes" gains the floor axis,
  the token grammar, and the verdict set; the probe set's enumeration moves to
  citing `lib/toolfloor.sh` as its owner rather than restating the names. The
  content-seam and cadence paragraphs are unchanged and stay where they are.
- **context-kit/SPEC.md §Layout and configuration** — the layout listing gains
  `lib/toolfloor.sh`; the knob roster is untouched, because this delta adds no
  knob (`A1`'s deliberate no-knob call: a consumer-overridable floor could only
  make the contract lie).
- **gate-sdk/SPEC.md §check-gate-assertions** — untouched, and named here so the
  merge does not mistake it for a duplicate of `A3`'s citation. It is the
  forcing construct's own home; `A3` points at it.
- **docs/install.md §Requirements** — `B2`'s bullet grammar, `C1`'s replaced
  paragraph, `C2`'s platform paragraph.
- **gate-sdk/README.md §Requirements** — `C3`.
- **docs/site-architecture.md** — its projections roster names
  `check-install-toolchain`'s coupling; the coupled path changes with `B3`.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. Specifically: no surface still claims that no
      minimum versions are pinned, and no surface restates the toolchain
      requirement outside the roster and the page that renders it.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
