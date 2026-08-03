# SPEC amendment: native-artifact-install-path

Queue entry: **`native-artifact-install-path`**. The consume half of the seam
**`native-artifact-publish-path`** produces: this amendment rules how a published
artifact is **selected, verified and placed**.

The basename drops the slug's `native-` prefix for the same measured reason its
companion's does, and the arithmetic here is tighter still — see §The
lead-line rulings below, which is also where the entry's roadmap curation and its
`[blocked-by:]` tag are settled and recorded.

## What this amendment inherits and may not re-litigate

TRAJECTORY.md's closed rulings, in force. Three bind this unit directly:

- **The consumer builds nothing and installs no toolchain.** Building from
  vendored crate source at install time is void.
- **The interpreter policy.** Something outside the binary must run first
  because the binary cannot select itself; that bootstrap is the *irreducible*
  interpreter surface, and its whole job is **resolve the platform, place the
  matching binary, invoke it**. Two standing obligations bind every unit
  touching the install path: **add no new shell-only install step**, and assume
  no POSIX shell. Every step this amendment adds sits *inside* that irreducible
  bootstrap — you cannot verify a binary with the binary you have not written
  yet — so each is ruled to be **dual-implementable** in the shape objective 6
  admits (bash for Linux/macOS, PowerShell for Windows, designed by
  **`powershell-installer-surface`**; moving the rest behind the invoke is
  **`install-step-relocation`**'s).
- **Opacity's obligation.** *Ship the achievable floor and claim nothing beyond
  it.* A governed surface may say **verified against a published digest** and may
  **not** say **reproducible**.

## What the installer may assume, and what it must not

Verified against the tree, 2026-08-03, because the whole selection rests on it:
**no platform-triple derivation exists anywhere.**
`context-kit/lib/toolfloor.sh` does no OS or architecture logic at all — it is
tool-version floors end to end. `context-kit/bin/env-probe.sh` emits a single
`OS:` field from an unparsed `uname -s -r -m` string (`:53`), rendered as one
Markdown bullet for a human to read (`:124`) and never parsed back. That field is
**not** the producer, and promoting a display string to a machine-consumed
selector is how a contract nobody declared comes into being. Nothing under
`installer/` inspects `uname`, `$OSTYPE`, or architecture today.

Equally verified: **no pre-write digest verification exists to extend.** The one
real `sha256sum -c` call site in the tree
(`installer/consumer-smoke/run-smoke.sh`:161-162) checks a digest the same script
just computed from the same file — it proves the *mechanism* the install page
documents works, not that anything matches an independently published value.
This is new code, not a generalization.

## What changes

### 1. Platform resolution — derived, never stored {design-bearing}

`installer/lib/init.sh` gains a `target_of_host()` local: `uname -s` and
`uname -m` in, one Rust target triple out, an empty string when the host maps to
nothing. It is called once per `init` run, after `KITS` resolves and before any
write.

**The resolution stays a local and is never persisted.** A stored copy is a
second source for a fact the host answers, and it is stale the first time a tree
moves between machines — the case that matters most, since a vendored tree is
shared by construction. The lock's `artifact` record (delta 4) carries the target
that *was installed*, which is a different claim: what is on disk, not what this
host is.

**Two commands, not `uname -a`.** The two-field form is the smallest input that
answers the question, and it is what a PowerShell half can answer without parsing
prose. Nothing reads the kernel version, so nothing may collect it.

### 2. Selection has three outcomes, and collapsing any two is the defect {design-bearing}

The installer reads the payload roster (`payload/artifact/targets.list`, the
verbatim publication **`native-artifact-publish-path`** delta 4 places) and
branches:

| host resolves to | payload holds | outcome |
| --- | --- | --- |
| a target **not** in the roster | — | **omit and declare** (`substrate-unavailable`) — a supported outcome |
| a target **in** the roster | its artifact + sidecar | verify, then write (deltas 3, 4) |
| a target **in** the roster | nothing, or a partial pair | **refuse the install** — the payload is broken |

The third row is why the roster is read at all rather than inferred from a
directory's presence. Without it the installer cannot tell a platform that was
never committed to from a platform that was committed to and whose artifact went
missing — and collapsing them turns a broken payload into a silently smaller
green battery, the vacuous-green class §Meta-gate conservation for the binary
substrate exists to refuse. **Never a green battery over a silently smaller
roster.**

The refusal is the one place this unit fails an install, and it is correct there:
a missing artifact for a **declared** target is a publisher defect the adopter
cannot act on and must not inherit silently.

### 3. Pre-write digest verification, and the hasher it needs {design-bearing}

The installer computes the artifact's SHA-256 and compares it against the
sidecar **before writing anything**, not after recording a hash. That ordering is
the whole of the delta: a consumer who cannot read the gate has nothing else
standing between them and a substituted binary, and a post-write check has
already put it on disk. A mismatch **refuses the install** — never a warning,
never a write.

**The hasher is resolved, and its absence is not a refusal.** `sha256sum` is
tried first, then `shasum -a 256`; the second exists because stock macOS ships
`shasum` rather than `sha256sum`, the same portability fact
`installer/lib/common/lock.sh`:25 already records as its reason for using
`git hash-object` in the manifest. When **neither** resolves, the install
proceeds and the artifact is **omitted and declared** under its own reason token
(delta 5) rather than written unverified. Both halves matter: never write what
was not verified, and never fail an install over something the adopter did not
choose — the guarantee the vendoring ruling made unconditional.

**Why SHA-256 here when the manifest uses `git hash-object`.** The two hashes
answer different questions and are deliberately not unified. The manifest's hash
is **change detection** — has the adopter edited a file `init` wrote — where
collision resistance is not the property needed and staying inside git's
already-asserted toolchain is worth more. The artifact's digest is an
**integrity claim under the opacity obligation**, published for a reader to
cross-check; `git hash-object` defaults to SHA-1, and a SHA-1 supply-chain digest
would undercut the one claim TRAJECTORY.md rules as the floor. Two hash families
on two classes of file, each stated where it is used.

**The honest bound, stated so no surface overclaims it.** The digest the
installer verifies against travelled **inside the same payload** as the artifact.
It therefore catches corruption and a substitution made to the artifact alone —
not a compromised publisher, which no in-payload value can. What raises it above
a self-check is that the identical bytes are published on the Release
(**`native-artifact-publish-path`** delta 3), so the value is cross-checkable
out of band by a human. This is the same bound `docs/install.md`:198-202 already
states for the tarball checksum, one layer in, and it is not a hedge to soften:
**verified against a published digest**, never *reproducible*.

### 4. `artifact` — the lock schema's first named class {design-bearing}

`checkwright.lock` today is `schema`/`version`/`commit`/`profile`/`kits`/`files`,
where `files` is one flat map of path to `git hash-object` value with no class
discriminator (`installer/lib/init.sh`:220-239). The vendored/generated split is
a **write-time behavior** — `claim()`'s hash-guard (`:106-116`) applies to
vendored paths — rather than something the schema names.

The binary is neither vendored nor generated, so it joins as its own top-level
key rather than as a `files` row:

```json
"artifact": { "target": "<triple>", "digest": "<sha256-hex>" }
```

**Two fields, each with a named reader at a named transition, and no third:**

- `target` — read by `installer/lib/doctor.sh`, which reports which target is
  installed; and by a re-run of `init`, which compares it against
  `target_of_host()` and rewrites when a tree has moved between machines.
- `digest` — read by `doctor`'s re-verification of the binary **in place**, which
  is the only thing standing between a consumer and a binary swapped after
  install; and by a re-run of `init`, which skips the rewrite when the on-disk
  artifact still verifies.

**The path is deliberately absent, and it is not an omission.** Derivation-first:
the install location has exactly one owner, `GATE_SDK_NATIVE_BIN` in the
consumer's `gate-sdk-config.sh` (delta 6), and that value is what `gate_command`
actually dispatches to. A stored copy could disagree with the live one, and the
reader that would want it — **`installer-lifecycle-verbs`**' uninstall — resolves
it from the same owner every other reader does.

**Why not a `files` row.** A `files` entry means "hashed with `git hash-object`,
rewritten when unmodified". The artifact is hashed with SHA-256 against a
published value and rewritten on a different rule, so a `files` row would put two
hash families on one map and break the invariant
`installer/consumer-smoke/run-smoke.sh` asserts over every `.files` entry. The
binary is still staged by `init` and still tracked.

**The artifact is tracked in the consumer's tree, and the footprint cost is real
rather than hidden.** An untracked binary would leave a colleague cloning the
consumer's repo with a `.gate` descriptor and no binary — `gate_command`'s exit 2
is a dispatch-harness error, so it would kill their whole battery, which is
criterion 5's original failure reproduced one layer out. The descriptor and the
binary must travel together in the consumer's history or not at all. Against
objective 4's footprint rule this is one binary for one target, bounded by the
roster and by the revisit trigger already recorded on the publish entry, and it
is removed by uninstall.

### 5. Omission is declared and counted, with a reason token per remedy {design-bearing}

An omitted member rides the registry, not a new file: `plan_gates()` writes it
into the consumer's `scripts/gates.list` as `# omitted: <name> <reason>`, a
comment line `gates_list_members` (gate-sdk/lib/gate.sh:66-68) already strips
from the live set — verified compatible, that function strips any
`^[[:space:]]*#` line.

**Two reason tokens, because two remedies:**

- `substrate-unavailable` — the host's platform has no declared artifact
  (delta 2, row 1). Remedy: none the adopter can take; the platform is not in
  the support roster.
- `digest-unverifiable` — an artifact exists but no hasher does (delta 3).
  Remedy: install `sha256sum` or `shasum`, re-run `init`.

The token is the record's one field beyond the member name, and its reader is the
remedy text below. A third token would need a third remedy to earn its place.

Named readers, both new readers of an existing file:

- `gate-sdk/bin/run-gates.sh` counts those lines and prints the count and the
  remedy as a **separate line** beside its summary. Separate is load-bearing:
  `run-consumer-smoke.sh`:63,215 and `installer/consumer-smoke/run-smoke.sh`:103
  match `All [0-9]+ gates passed` against the runner's output, so the new line
  must not contain that phrase and the existing assertion stays intact. (The
  queue entry says those readers grep the phrase *verbatim*; the greps are
  `grep -qE` regexes. The constraint is the same and the amendment states it
  accurately.)
- `installer/lib/doctor.sh` reports the same count against the reason that caused
  it, which is where an adopter looks for a remedy, plus the installed `target`
  and the in-place digest re-verification of delta 4.

The record sits in the consumer's **tracked** `gates.list`, reviewable in their
history rather than buried in install-time stdout. Re-running `init` on a machine
that has since gained a hasher converts the comment back into a live member with
no hand edit.

**This closes `run-gates.sh`'s roster-collapse tripwire honestly.** §run-gates
names `All N gates passed.` as the tripwire for a battery that silently shrank;
a legitimately omitted member shrinks N, so the omission line is what keeps a
declared omission distinguishable from a regression the tripwire exists to catch.

### 6. The install location has one owner {design-bearing}

`init` writes the artifact to `<gates-dir>/<binary>` — `scripts/checkwright-gates`
under the defaults — beside the `gates.list` it already seeds there, and writes
`GATE_SDK_NATIVE_BIN=<that path>` into the consumer's `gate-sdk-config.sh`
through the existing config seam (`init.sh`:152-187).

**Ordering is load-bearing**, and the existing order already serves it: the
config seam is written at `:152-187`, before `gen-pre-commit.sh --write` at
`:209`. The hook's `run_gate` lines come from `gate_command`, which reads the
knob, so the seam must be in place before the hook is generated. `check-graph.sh
--emit` (`:211`) reads declaration paths as text and is unaffected either way.

**Why the seam rather than changing the knob's default.** `GATE_SDK_NATIVE_BIN`'s
default is `native/target/release/checkwright-gates`, and its **stable relative
path** rationale is unchanged: the generated hook persists the emitted argv, so a
machine-specific path would make `check-graph`'s byte-freshness comparison
machine-dependent. Deriving the default from `GATE_SDK_GATES_DIR` instead was
weighed and refused — it would silently relocate the binary for every existing
reader and make this repo's own layout the exception to a convention whose stated
rule is that this repo's layout *is* the default.

**Why not `native/target/` in the consumer tree.** It would put a crate root in a
tree with no crate and a `target/` directory that reads as build output in a tree
that builds nothing — both false statements about a consumer's repo, in the exact
place the vendoring ruling worked to remove a build step.

### 7. The artifact must not read as determinism drift {design-bearing}

`gate-sdk/bin/upgrade-smoke.sh`'s phase-A determinism assertion covers changes
under the kit roots plus the two regenerated artifacts. Phase A replaces the
vendored kit directories wholesale **in tree** and never re-runs an installer, so
the artifact write is outside its reach entirely — an honest limit the section
already states, and this amendment records the artifact as sitting on the far
side of it rather than quietly widening the assertion.

The install path's own idempotence proof is `installer/consumer-smoke/run-smoke.sh`,
which asserts a bare `init` re-run leaves `HEAD^{tree}` byte-identical. That
assertion is what the artifact must satisfy, and delta 4's re-run rule is what
satisfies it: an on-disk artifact that still verifies against the recorded digest
is not rewritten. The smoke gains an artifact arm — `artifact.target` matches the
host's resolution, `artifact.digest` matches the payload sidecar, the binary is on
disk at the seam's path and executes `--list` — or the omission arm on a host
with no declared target.

## The lead-line rulings

Both settled here rather than left to be re-derived, because neither is visible
from anything but a column count.

**Roadmap curation: publish-only, and the entry is uncurated. Ruled by the
operator; the outcome stands.** The mechanism behind it is
queue-kit/SPEC.md §bin/roadmap.sh's stated honest limit — *"a sufficiently long
slug plus spec pointer leaves no room even for that, and such an entry is
unprojectable until the pointer drops at the amendment's merge. A true state of
the queue rather than a defect to gate around"* — and widening
`QUEUE_KIT_WRAP_BUDGET` to buy a public page's tag *"would trade a parse
guarantee for a presentation one"*. So: no `[roadmap:]` tag here, and none is
owed until this amendment merges and the `[spec:]` pointer drops.

**The stated column count belonged to the pre-promotion line and is corrected
here rather than propagated.** 121 columns is exactly
`- **native-artifact-install-path** [design-pending] [blocked-by: …] [roadmap: now/reliability]`
— the *deferred* lead line plus a roadmap tag, with `[design-pending]` still in
place. Promotion swaps that tag for `[spec:]`, which is far wider, so the
promoted two-tag line measures **122**. The roadmap tag is not what breaks the
budget; the promotion is. Recording the original figure would have put a
falsified number in a governed surface, and the ruling it supports does not need
it — the outcome is unchanged and the mechanism above is the stronger ground.

**`[blocked-by: native-artifact-publish-path]` drops at promotion.** Two grounds:

- *Truth.* The tag asserts the entry is **unpickable** until its blocker
  completes. Both units were cut into **one batch** — they share their entire
  surface set — so the assertion is false the moment both are promoted. The
  tag's recorded premise was a **design** blocker (*"nothing exists to select or
  verify… not even a fixture reaches it without inventing the blocker's
  roster"*), and it is discharged by this stage authoring that roster and digest
  shape in the companion amendment this one reads.
- *Arithmetic.* With the tag retained the entry cannot be promoted at all: the
  lead line needs 35 columns for the slug and 42 for the tag, leaving 23 for the
  spec pointer, so the amendment basename would have to fit in 14 characters —
  `SPEC-install.md`, the shortest spelling that still reads, lands the line on
  101. No name rescues it, and raising the budget is ruled out above.

## Producers and consumers

**Resolved host target (derived local).**
Producer: `target_of_host()` in `init.sh`, once per run, on every install and
every re-run. Consumer: the same function's three-way branch (delta 2), and
delta 4's re-run comparison. **Never persisted** — nothing else may read it,
because the host already answers it.

**Payload roster copy.**
Producer: `scripts/pack-installer.sh`'s artifact loop
(**`native-artifact-publish-path`** delta 4), at pack time. Consumer: `init.sh`'s
selection branch. One field per live line, read by that branch.

**Verified artifact (new written file).**
Producer: `init.sh`'s write step, after delta 3's verification and never before
it. Consumers: `gate_command` (gate-sdk/lib/gate.sh:96-105) at every battery run
in the consumer tree, and `doctor`'s in-place re-verification. Its enabling
config is `GATE_SDK_NATIVE_BIN`, written to the consumer's `gate-sdk-config.sh`
by delta 6 — the one place this amendment must prove a producer's config is
actually emitted, and delta 6's ordering paragraph is that proof.

**`artifact` lock record (new schema class).**
Producer: `manifest()` in `init.sh`, on any run that wrote or re-verified an
artifact; absent entirely on a run that omitted one, so its absence is itself the
omission's machine-readable form. Consumers and transitions: `doctor` (reports
`target`, re-verifies `digest` in place), `init`'s re-run (compares `target`
against the host, skips the rewrite when `digest` still holds), and
**`installer-lifecycle-verbs`**' uninstall. `lock_schema_ok`'s versioned wire key
is unchanged — a new optional top-level key is additive within
`checkwright-lock v1`, and a reader that does not know it sees the same manifest
it always did.

**Omitted-member record.**
Producer: `plan_gates()` in `init.sh`, on a run whose selection omitted a member.
Consumers: `run-gates.sh`'s separate summary line and `doctor`'s report
(delta 5). One field beyond the member name — the reason token — whose reader is
the remedy text each of those two prints.

**Hasher verdict.**
Producer: the `command -v` probe in delta 3, per run. Consumer: the same
function's branch, and delta 5's token selection. A local; nothing persists it,
because the next run's machine may differ.

## Existing sections updated

Owned by this amendment, each named with the delta that claims it:

- gate-sdk/SPEC.md §Consumer smoke — the artifact and omission arms the scratch
  consumer asserts (delta 7).
- gate-sdk/SPEC.md §run-gates — the omitted-member line beside the summary, and
  why it must not match the green token (delta 5).
- gate-sdk/SPEC.md §upgrade-smoke — the artifact recorded as outside phase A's
  determinism assertion, with the honest limit that section already states
  (delta 7).
- gate-sdk/SPEC.md §Layout and configuration — `GATE_SDK_NATIVE_BIN`'s consumer
  value and why the default is unchanged (delta 6).
- `installer/README.md` §The manifest — an `artifact` row in the field table,
  and the two-hash-families paragraph beside the existing
  `git hash-object`-never-`sha256sum` one, which stays true for `files` and gains
  its scope (deltas 3, 4).
- `installer/README.md` §init and §What init seeds — the artifact write and the
  config-seam line (delta 6).
- `docs/install.md` §Requirements — the install path's own digest-tool
  requirement (`sha256sum` **or** `shasum`), stated in the delivery-path prose
  paragraph that already carries `curl`/`tar`/Node rather than in the battery
  roster, which asserts only what the *battery* needs. **This collides with
  `install-path-gnu-userland-undeclared`**, promoted into this same iteration's
  Technical Debt and widening the same paragraph for `sort -V` in `init.sh` and
  in the env-probe: same page, same section, same iteration. Whichever lands
  second states both requirements or reverts the first. `check-install-claim` is
  the reader that must stay green (delta 3).
- `CLAUDE.md` §Housekeeping's `native/` bullet — the install side of the ruled
  model (deltas 1-6).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Nothing unverified is ever written.** A digest mismatch refuses; an
      absent hasher omits. There is no third path in which a binary reaches a
      consumer's disk unchecked.
- [ ] **No new shell-only install step.** Every step added sits inside the
      irreducible bootstrap and is dual-implementable — resolve, verify, place,
      and nothing conditional beyond it.
- [ ] **`docs/install.md` §Requirements reconciled** with
      **`install-path-gnu-userland-undeclared`**, whichever of the two lands
      second.
- [ ] **No surface says *reproducible*.** The published word is *verified
      against a published digest*.
