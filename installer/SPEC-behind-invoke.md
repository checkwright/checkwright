# SPEC amendment: behind-invoke relocation

The relocation half of `powershell-installer-surface`: `init`'s conditional
install logic — and, on the ruling below, every other verb's — moves to the far
side of the gate binary's invoke, so it is written once in Rust rather than twice
by hand in bash and PowerShell.

**The ruling this amendment implements is TRAJECTORY.md's
`bootstrap-one-success-path`, cited and not restated.** Its grounds, its three
refused alternatives and its two bounds live there; its provenance — the consult
session's own reading under the operator's direction to convene it, and the
operator's ratification of it — lives there too. What follows is mechanism, which
is `installer/README.md`'s to state undated, per CLAUDE.md §The provenance seam.

## What changes

### (1) The bootstrap has one success path

`substrate-unavailable` and `digest-unverifiable` both become **bootstrap
refusals**, and omit-and-declare retires at install. {design-bearing}

The selection table in §The gate binary **keeps its three outcomes** — that is a
bound of the ruling, not a detail. What changes is that only one of them
proceeds. The three stay told apart by **message and remedy**, never by exit
status alone: an undeclared host and a broken payload remain different answers to
an adopter, and collapsing them is the same defect the table has always named.

| the host resolves to | the payload holds | outcome |
| --- | --- | --- |
| a target **not** in the roster | — | **refuse** — this platform is not in the support roster; there is no adopter action |
| a target **in** the roster | its binary and sidecar | verify, then execute |
| a target **in** the roster | nothing, or half the pair | **refuse** — the payload is broken |

The ground is that nothing remains behind the invoke to proceed *into*. Once
every step of an install is on the far side of it, an artifact-less host has no
code path at all, and the failure the relocation would otherwise introduce is not
a smaller battery but a silent non-install. A refusal that names the platform is
the honest form of what that host was already getting.

`digest-unverifiable` **dissolves behind the invoke rather than moving**: the
binary hashes in-process (`native/src/sha256.rs`), so nothing past step 5 needs an
external hasher. What survives is the *bootstrap's* step 4, which cannot use the
binary to verify the binary — so a POSIX host carrying neither `sha256sum` nor
`shasum` is refused rather than served an unverified artifact. On the PowerShell
half the branch was always vacuous (`Get-FileHash`), which is the same fact §The
install boundary already records from the other direction.

### (2) The bootstrap is branchless, and the binary owns the verb roster

Both bootstraps apply **one unconditional argv rule**: if the first token is
dashless, prefix it with `--`; forward everything after it verbatim.
{design-bearing}

`checkwright init --profile starter` reaches the artifact as
`--init --profile starter`; `checkwright --help` is forwarded unchanged. The rule
is unconditional, so it introduces **no conditional logic into the bootstrap** —
which is what §The install boundary's interpreter policy forbids there, and what a
verb-routing table maintained in two languages would have been.

Step 5 of the bootstrap narrows accordingly, from *forwarding argv verbatim* to
*prefixing a dashless leading verb and forwarding the remainder verbatim*. It is
the only one of the five steps this amendment touches.

Two consequences follow, and both are improvements rather than costs:

- **The verb roster's owner moves from the filesystem to the binary.** §Layout's
  rule — the dispatcher advertises `lib/*.sh` — retires with the directory. An
  unknown verb is refused by the binary's own usage arm rather than by a bash
  roster, so `checkwright --help` cannot promise a verb the artifact does not
  carry.
- **The two halves stop disagreeing about the verb word.** `checkwright.ps1`
  forwards it today and `checkwright.sh` consumes it; after this rule they agree,
  which is a parity defect closed rather than one introduced.

### (3) All five verbs become non-gate arms of the binary, and `installer/lib/` is deleted

`init`, `doctor`, `diff`, `update` and `uninstall` become top-level `--`-prefixed
arms of the gate binary, resolved in `main` before the registry lookup and absent
from `--list`. {design-bearing}

The shape is gate-sdk/SPEC.md §The non-gate arm's, taken unchanged: each owes no
`.gate` descriptor, no `gates.list` registration and no `good/`+`bad/` fixture
pair, and each owes a **named caller** instead — the bootstrap, at step 5. The
`--`-prefixed placement is what keeps §check-gate-substrate-parity assertion B's
equality true in both directions, which is why delta 2's prefix rule exists rather
than a bare-word subcommand.

They are **not bridged arms**, on the ground §The install boundary already gives
`--install`: the caller is the bootstrap, which may not be assumed to be a POSIX
shell, so every value arrives as argv and the arms read no kit config and no knob.

`installer/lib/` is deleted entire. The discharge oracle is the ruling's own:
`test ! -e installer/lib`.

**`init` is not folded into the `--install <op>` family**, and the reason is a
channel conflict rather than taste: that family specifies stdout as a *wire* —
tab-separated records — while `init`'s stdout carries the adopter-facing
follow-up block whose grammar §init states and whose reader is the consumer
smoke's follow-up arm. One of the two contracts would have had to yield.
`--install place-artifact` survives unchanged beside the new arms.

### (4) `init`'s `jq` preflight retires rather than moving

The preflight is deleted, not relocated, and `init`'s **four** preconditions
become **three**. {design-bearing}

This is the worked case §The install boundary already classes as `retired`:
nothing behind the invoke reads JSON with `jq`, because the crate reads it with
`serde_json`. §Requirements' claim that *the verbs themselves need `jq`* retires
with it — all five verbs are Rust after delta 3, so none of them refuses for want
of it.

**A `jq`-less machine is still refused an install, and by a better message.**
`jq` is a consumer-audience member of the toolchain floor, so the `doctor`
precondition — which survives, and still runs before any file is written —
blocks the install naming the floor the battery needs. What is lost is a refusal
that named the verb's own dependency; what replaces it is the one that names the
adopter's actual problem. §Requirements' `doctor`-reaches-its-diagnosis paragraph
keeps its point unchanged.

### (5) `context-kit/lib/toolfloor.sh` is deleted, and `--toolfloor-parity` retires with its second holder

The floor roster and predicate move in-crate behind `--doctor`, taking
`context-kit/index-tests/toolfloor-cases.sh` with them. {design-bearing}

`native/src/toolfloor.rs` already carries a full port of the parse-and-check
predicate; what it lacks is the orchestration `installer/lib/doctor.sh` supplies,
which delta 3 owes anyway. The case table is a projection of the library's own
verdict set and the two move together — context-kit/SPEC.md §Testing states that
coupling and this amendment does not restate it.

**`--toolfloor-parity` retires**, on gate-sdk/SPEC.md §The non-gate arm's own
rule that a parity arm's caller is its second holder and the arm retires with it.
That library's shell caller is the installer's `doctor`; deleting the library
empties the holder, and one holder cannot be held equal to itself. This is the
same door `--declaration-parity` left by, and the census discipline the same
section asks for — *which other helpers' caller sets does this cut empty* — is
what surfaced it.

**The same census, run a second time over the roster's *storage path* rather
than its caller, surfaces one more site the first pass did not reach.**
`native/src/toolfloor.rs`'s `ROSTER` constant is the literal string
`"context-kit/lib/toolfloor.sh"`, read at runtime — not sourced as shell, parsed
as data — by two callers neither of which is `doctor`: `check-install-toolchain`
(`native/src/gates/install_toolchain.rs`, `DEFAULT_ROSTER`) and the `env-probe`
emit arm (`native/src/emit/env_probe.rs`, which reads it for the `PROBE_SET`
array underlying docs/install.md §Requirements' toolchain list). Deleting the
file without repointing
either breaks both at first run. "The floor roster ... move[s] in-crate" above
already covers this by design — the roster text moves beside the predicate it
already shares a home with — so this is enumeration, not a new decision: both
callers repoint at the in-crate roster in the same commit as the deletion.
`scripts/check-install-toolchain.gate`'s own `couples=` line names the same path
and narrows with it, the same shape delta 12 already gives
`check-install-disposition`'s `couples=`. The prose asserting today's file as the
roster's home lives at `gate-sdk/README.md`, `docs/install.md` and
`docs/site-architecture.md`'s freshness-gate roster, plus their generated
mirrors and `gate-sdk/SPEC.md`'s own mentions (the last two already rostered
below for other delta-5 reasons).

### (6) `doctrine-kit/bin/install-doctrine.sh` and `gate-sdk/lib/inject.sh` move in-crate

Both follow their own stated sequencing rather than a fresh judgement.
{design-bearing}

doctrine-kit/SPEC.md §The port disposition sequences its installer behind exactly
this relocation and names this entry as the live owner; `gate-sdk/lib/inject.sh`
sits behind it as its one remaining shell sourcer. Their two adopter-path call
sites — `init` seeding the reference block, `uninstall` trimming it — are both
inside delta 3's arms after this cut, so the marker strings keep their one writer.

### (7) The gate-registry `# omitted:` class keeps its generic arm and loses its two named ones

`native/src/runner.rs`'s reason-agnostic `# omitted: <name> <reason>` class is
**kit mechanism and stays**; only its two hard-coded reason arms go.
{mechanical}

This is the ruling's second bound and it must not be conflated with delta 1. Two
distinct mechanisms share one vocabulary: the **installer's selection outcome**,
which retires with both tokens, and **gate-sdk's registry class**, which is
reason-agnostic, already carries a generic fallback arm, and belongs to any
consumer who omits a member for any cause. Deleting the class because its two
best-known values retired would publish an installer decision as a kit narrowing.

### (8) The consumer smoke stops sourcing installer internals

`installer/consumer-smoke/run-smoke.sh` sources four modules out of
`$PKG_ROOT/lib/common/`, all of which delta 3 deletes; each is re-hosted by a
stated rule rather than re-implemented in bash. {design-bearing}

The rule is the smoke's **own declared ground**: its `# no-port:` header says it
drives the packer and `init` as black boxes, and sourcing the implementation it
is testing was always in tension with that. So — **a fact that is data is read as
data; a fact that is a derivation is read from the package through the binary.**

| module | what the smoke used it for | disposition |
| --- | --- | --- |
| `lock.sh` | manifest accessors | read `checkwright.lock` with `jq`, already a smoke preflight tool |
| `digest.sh` | artifact hashing | `sha256sum`, already a smoke preflight tool |
| `profile.sh` | the profile lattice and gate-set union | `profiles.list` and `payload/*/` are data; the installed `gates.list` is the derivation's own output, so monotonicity is asserted over installed registries |
| `recipe.sh` | `recipe_queue_source`, the layout names | a derivation, so read from the package through the binary's `--install` wire |

Only the last needs a new read op, and it takes the `--install` family's existing
grammar, channels and exit statuses rather than minting a contract. §Profiles'
one-derivation property is thereby **preserved and strengthened**: the smoke and
the installer still share one derivation, and the smoke now reads it across the
package boundary instead of inside it.

### (9) The smoke's omit-and-proceed legs become refusal legs, and the `jq`-less arm re-targets

Three arms assert behaviour delta 1 and delta 4 change, and each inverts rather
than disappears. {design-bearing}

- The **binary-less leg** packs a payload with no artifact directory and asserts
  an install with a non-zero omitted count; it now asserts a refusal that wrote
  nothing — the same shape the artifact arm's tampered leg already uses.
- The **artifact arm's undeclared-host leg** asserts `rc -eq 0` and a manifest
  without an `artifact` key; it now asserts a refusal, distinguished from the
  declared-but-absent leg by **message and remedy** per delta 1's bound.
- The **`jq`-less arm** asserts that `init`, `diff` and `uninstall` each refuse
  naming `jq`; it now asserts that they no longer need it and that the install is
  blocked by `doctor`'s floor verdict instead.

Each arm's `printf` header is a **parsed contract** — the `installer_smoke`
validate parser derives its scenario roster from those lines — so a reworded
header renames a baseline scenario and `.workflow/validate-baseline.txt` moves in
the same commit.

### (10) The PowerShell install-smoke leg extends to payload coverage

`install-smoke-powershell` gains an assertion that a whole `checkwright init`
completes through the PowerShell bootstrap, and its verdict block is rewritten.
{design-bearing}

This is the obligation the queue entry holds as landing with this half or not at
all. §The install boundary's two-part oracle — bootstrap parity, discharged; and
payload coverage, owed — closes here: the arm minted by delta 3 is what makes a
completion reachable, so the leg can assert one for the first time. The leg's
current verdict block says in terms that it does *not* assert this and names the
relocation as what it waits on; that text is a literal edit target of this delta,
not merely a section to revisit.

### (11) `installer/package.json`'s `files` roster drops `lib/`

The published roster loses the directory delta 3 deletes. {mechanical}

§Layout already states that what makes a module reachable at an installed
`PKG_ROOT` is this roster and not the directory listing, so the roster is where
the deletion has to be spelled. `bin/` stays: both bootstraps ride it.

### (12) The three install-path knob corpora narrow with the deleted tree

`GATE_SDK_LINT_EXTRA_DIRS`, `GATE_SDK_PORTABILITY_PATHS` and
`check-install-disposition`'s `couples=` all name paths this cut deletes.
{mechanical}

They are listed as one delta because they are one edit class and none carries a
judgement: after the cut the install path an adopter's machine executes is the
bash bootstrap alone, plus what `init` leaves behind and runs. `installer/lib`,
`installer/lib/common` and `context-kit/lib/toolfloor.sh` leave the portability
corpus; `installer/lib/common/recipe.sh` leaves the disposition gate's trigger.
Their **red conditions are enumerated in the next section** rather than assumed
harmless, because this delta narrows corpora and a narrowing is where that
assumption fails.

## Producers and consumers

**New interfaces, each with its producer, its consumer and the transition.**

| interface | producer | consumer | transition |
| --- | --- | --- | --- |
| `--init`, `--doctor`, `--diff`, `--update`, `--uninstall` | `main`'s top-level flag dispatch, before the registry lookup (delta 3) | both bootstraps, at step 5 | every `checkwright <verb>` invocation |
| the dashless-verb prefix rule | `installer/bin/checkwright.sh` and `installer/bin/checkwright.ps1` (delta 2) | the arms above | step 5, on every invocation |
| the `--install` read op for `recipe_queue_source` | the `--install` family (delta 8) | `run-smoke.sh`'s queue post-condition | once per profile, per smoke run |
| the two bootstrap refusals | steps 3 and 4 of both bootstraps (delta 1) | the adopter, and the smoke's three refusal legs | before anything is written |

**The producers' enabling configuration is emitted.** Every arm reads argv alone
and no knob, so there is no enabling config that a deployed configuration could
fail to set — the property `--install` was given for this reason and the one the
bootstrap's language-agnostic caller requires. The one configuration that does
move is delta 12's three knob corpora, and it moves in the same commit as the
deletion that empties it.

**Every new field has a named reader.** The read op of delta 8 emits exactly the
queue-source fact its one caller asserts on; no second field is added, because a
field with no reader is removed. The refusals of delta 1 add no field at all —
they replace a written record (`# omitted:`) with an exit status and a message.

**Existing integration prose is updated in this amendment**, not left to drift:
the roster below is that update, and the table in delta 1 is the replacement text
for §The gate binary's selection table. **Not yet applied** — this stage authors
and build lands, so every passage named below is a proposal until the build stage
merges it.

### Each reader's red condition, because this amendment narrows corpora

Deltas 3, 5, 8 and 12 delete files and empty corpora. Per canon-kit/SPEC.md §The
causal-completeness check point 5, a reader is clearable **by inspection** only if
its verdict is monotone in the violation set, so what follows enumerates what
makes each reader **red** rather than what it is about. "A narrower corpus can
only remove violations" is false and is the first argument a narrowing reaches
for.

**Not monotone — must be inspected and moved in the same commit:**

- `check-install-disposition` — its third assertion is *the installer keeps no
  second copy of the roster*, held today against
  `installer/lib/common/recipe.sh`'s `recipe_gates`. Deleting that file empties
  the holder. Red condition is a **comparison against a holder that no longer
  exists**, which is the `--toolfloor-parity` shape one layer out; the assertion
  moves to the in-crate derivation or retires with its second holder, and delta 5
  is the precedent for deciding which.
- `check-portability-floor` — `GATE_SDK_PORTABILITY_PATHS` names three paths this
  cut deletes. Red condition includes a **declared path that does not resolve**,
  so the knob moves with the deletion (delta 12).
- `check-gate-substrate-parity` assertion B — an **equality** between the `.gate`
  descriptor set and `--list`'s roster, not a threshold. The five new arms stay
  outside `--list` (delta 3), which is what keeps the equality true in both
  directions.
- `check-gate-binary-fresh` — an **equality** between the binary's source stamp
  and git's content identity for every tracked file under the crate root. Every
  commit touching `native/` owes `bash gate-sdk/bin/build-native.sh`.
- `check-manifest-count` — reds on a **bare cardinal quantifying a governed
  collection**. Delta 4 takes `init`'s preconditions from four to three and
  §The install boundary states *the bootstrap count is two*; the first moves, the
  second does not.
- `check-install-claim` and `check-payload-claim` — the attested case: pruning
  the file holding a declaration's sole instance flips `check-install-claim`
  green to red, because its red condition is a **zero count**. The install-path
  prose this cut rewrites is exactly where that declaration lives.
- `check-docs-cmd` — reds on a doc **fencing** a deleted `.sh` path. Its reach is
  fenced invocations only, so unfenced prose mentions of `installer/lib/…` give
  no red and are the roster's job below, not the gate's.
- the `installer_smoke` validate baseline — reds on a **scenario-roster
  difference**. Delta 9 rewords parsed arm headers, so
  `.workflow/validate-baseline.txt` moves in the same commit.
- `check-install-toolchain` — the same **comparison against a holder that no
  longer exists** shape named for `check-install-disposition` above, reached a
  second way: its `couples=` names `context-kit/lib/toolfloor.sh` directly, and
  its own default roster argument (`native/src/toolfloor.rs`'s `ROSTER`)
  resolves to that same deleted path at runtime. Both the `.gate` file's
  `couples=` and the constant move to the roster's in-crate home in the same
  commit as the deletion (delta 5).

**Monotone — clearable by inspection:** `check-comment-tier`,
`check-portability-floor`'s per-file findings once its corpus is corrected,
`check-tree-terms`, and `--emit port-blockers --tree`, whose owed count falls from
sixteen to one and whose completion predicate is satisfied downward.

## Existing sections updated

- `installer/README.md` — the whole of it that describes the prior flow:
  §Implementation's *the verbs are bash*, §Requirements' verb-`jq` claim and its
  `doctor` exception, §Layout's dispatcher and `lib/` roster, §The verbs' derived
  roster, §init's four preconditions, §The install boundary's port disposition,
  step 5, and both consequences of the now-discharged precondition, §The gate
  binary's selection table and its omission vocabulary, §doctor's omitted-gates
  block, and §The consumer smoke's three affected arms (all deltas).
- `installer/bin/checkwright.sh` — becomes the bash bootstrap, twin of the
  PowerShell half, and stops dispatching to `lib/` (deltas 2 and 3).
- `installer/bin/checkwright.ps1` — the omit branch becomes a refusal and the
  verb-prefix rule lands (deltas 1 and 2).
- `installer/lib/init.sh` — deleted; its bootstrap opening moves to `bin/checkwright.sh` and the
  rest of it becomes the `--init` arm (deltas 2, 3 and 4).
- `installer/lib/doctor.sh` — deleted; becomes the `--doctor` arm (deltas 3 and 5).
- `installer/lib/diff.sh` — deleted; becomes the `--diff` arm (deltas 3 and 4).
- `installer/lib/update.sh` — deleted; becomes the `--update` arm (delta 3).
- `installer/lib/uninstall.sh` — deleted; becomes the `--uninstall` arm (deltas 3, 4 and 6).
- `installer/lib/common/argv.sh` — deleted; the argv-width batching moves in-crate with its one
  caller (delta 3).
- `installer/lib/common/digest.sh` — deleted; the hasher resolution it owns is what delta 1
  retires, and in-crate hashing replaces it (deltas 1 and 3).
- `installer/lib/common/lock.sh` — deleted; the manifest schema moves in-crate and the `jq`
  preflight it owns retires (deltas 3 and 4).
- `installer/lib/common/profile.sh` — deleted; the profile lattice moves in-crate (deltas 3 and 8).
- `installer/lib/common/recipe.sh` — deleted; the install recipe moves in-crate, taking the
  disposition gate's second-copy holder with it (deltas 3, 8 and 12).
- `installer/package.json` — the `files` roster (delta 11).
- `installer/consumer-smoke/run-smoke.sh` — the four sourced modules, the three
  inverted arms, and the parsed headers (deltas 8 and 9).
- `.github/workflows/gates.yml` — the PowerShell leg's payload assertion and
  verdict block, and the `substrate-unavailable` assertion in it (deltas 1 and 10).
- `.workflow/validate-baseline.txt` — the renamed `installer_smoke` scenarios
  (delta 9).
- `native/src/main.rs` — the five arms' dispatch, `TOP_LEVEL_FLAGS`, and
  `--toolfloor-parity`'s removal (deltas 3 and 5).
- `native/src/install.rs` — `--install`'s op set gains the read op delta 8 needs
  (deltas 3 and 8).
- `native/src/runner.rs` — the two named reason arms go, the class stays (delta 7).
- `native/src/gates/install_disposition.rs` — the second-copy assertion's holder
  (delta 12).
- `native/targets.list` — its header rests omit-and-declare's non-vacuity on the
  undeclared complement, which delta 1 retires (delta 1).
- `native/src/toolfloor.rs` — the `ROSTER` constant repoints from the deleted
  shell file to the roster text moving in-crate beside it (delta 5).
- `native/src/gates/install_toolchain.rs`, `native/src/emit/env_probe.rs` — each
  reads the roster through `toolfloor::ROSTER` alone, so each moves with the
  constant and owes no separate read-site edit (delta 5).
- `scripts/check-install-toolchain.gate` — its `couples=` names the deleted path
  and narrows with it, the shape delta 12 already gives
  `check-install-disposition.gate`'s (delta 5).
- `docs/install.md` — §Requirements' platform block prose, now that an
  undeclared platform is refused rather than served a declared omission (delta 1);
  and, separately, the toolchain-roster-parity prose naming
  `context-kit/lib/toolfloor.sh` as the probe roster's home, and the GNU-`sort`
  paragraph's second site, both stale once the file that grounds them is deleted
  (delta 5).
- `context-kit/SPEC.md` — the sequencing sentence for `lib/toolfloor.sh` and
  `index-tests/toolfloor-cases.sh`, and §Testing's parity coupling (delta 5).
- `context-kit/lib/toolfloor.sh`, `context-kit/index-tests/toolfloor-cases.sh` —
  deleted (delta 5).
- `context-kit/gate-tests/toolfloor-parity.test.sh` — retires with its arm (delta 5).
- `gate-sdk/SPEC.md` — §The non-gate arm's roster gains five members and loses
  `--toolfloor-parity`; §check-install-disposition's third assertion; and its own
  three mentions of `context-kit/lib/toolfloor.sh` as the probe roster's home
  (deltas 3, 5, 12).
- `gate-sdk/README.md` — the sentence holding `docs/install.md` to
  `context-kit/lib/toolfloor.sh` "the roster's owner" (delta 5).
- `gate-sdk/lib/inject.sh`, `doctrine-kit/bin/install-doctrine.sh` — moved
  in-crate (delta 6).
- `doctrine-kit/SPEC.md` — §The port disposition's sequencing, now discharged
  (delta 6).
- `gate-sdk/checks/check-install-disposition.gate` — its `couples=` (delta 12).
- `scripts/gate-sdk-config.sh` — `GATE_SDK_LINT_EXTRA_DIRS` and
  `GATE_SDK_PORTABILITY_PATHS` (delta 12).
- `scripts/git-hooks/pre-commit` — a generated projection, regenerated by its own command; it
  persists each member's emitted argv, so a retired arm survives there until it is (all deltas).
- `docs/check-graph.html` — a generated projection, regenerated by its own command (all deltas).
- `docs/site-architecture.md` — `check-install-toolchain`'s own freshness-gate
  entry, which names `context-kit/lib/toolfloor.sh`'s `PROBE_SET` as one half of
  the parity it holds (delta 5).
- `docs/gate-sdk/SPEC.md` — the generated site mirror of `gate-sdk/SPEC.md` (all deltas).
- `docs/context-kit/SPEC.md` — the generated site mirror of `context-kit/SPEC.md` (all deltas).
- `docs/gate-sdk/README.md` — the generated site mirror of `gate-sdk/README.md`
  (delta 5).
<!-- update-target-exempt: a published dated release note is immutable by the same rule CANON_KIT_TEMPORAL_EXEMPT_PATHS and CANON_KIT_INSTALL_CLAIM_EXCLUDE already carry for docs/posts/*; it is named here so the retired-spelling reconciliation does not read an intentional historical mention as a missed site -->
- `docs/posts/2026-08-06-checkwright-v0-22-0.md` — a historical mention of
  `installer/lib`, left standing.
<!-- update-target-exempt: a published dated release note is immutable, same ground as the bullet above -->
- `docs/posts/2026-07-26-checkwright-v0-16-0.md` — a historical mention of
  `context-kit/lib/toolfloor.sh` (its introduction), left standing.
<!-- update-target-exempt: a published dated release note is immutable, same ground as the bullet above -->
- `docs/posts/2026-08-01-checkwright-v0-21-0.md` — a historical mention of
  `context-kit/lib/toolfloor.sh` (a later floor bump), left standing.

## Retired spellings

- `substrate-unavailable` — the installer's selection outcome and its omission
  record retire; the reason token has no producer left once the outcome refuses.
  gate-sdk's reason-agnostic registry class is **not** this spelling and survives
  (deltas 1 and 7).
- `digest-unverifiable` — retires for the same reason on the bash half and was
  always vacuous on the PowerShell one (delta 1).
- `lock_require_jq` — the `jq` preflight is deleted rather than relocated, and its
  three callers go with `installer/lib/` (deltas 3 and 4).
- `--toolfloor-parity` — retires with its second holder, on gate-sdk/SPEC.md
  §The non-gate arm's own rule (delta 5).
- `installer/lib` — the directory and every path under it (delta 3).
- `context-kit/lib/toolfloor.sh` — the roster's storage path, not merely its
  predicate's; the file is deleted and both the roster text and every literal
  citation of this path move with it (delta 5).

## Definition of Done

- [ ] **Causal completeness** — every new arm has a named, reachable producer and
      a named consumer; the one new field has a named reader at a named
      transition; every non-monotone reader above is inspected, not assumed.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper section of `installer/README.md` and the kit SPECs above, not
      appended; the merged spec reads as one document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` runs each
      declaration above against the whole tracked tree.
- [ ] **The discharge oracle passes** — `test ! -e installer/lib`, and
      `--emit port-blockers --tree` reports one owed file, `scripts/pack-installer.sh`.
- [ ] **Gaps filed** — cross-component gaps discovered during the work resolved
      that session, not deferred.
