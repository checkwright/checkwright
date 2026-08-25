# SPEC amendment: install-boundary

Fork 1 of `powershell-installer-surface`: **what moves behind the binary invoke,
and what the PowerShell half must re-implement.** It rules the boundary and takes
the first relocation cut. Fork 2 — two hand-kept bootstraps held in parity by a
smoke leg, versus one bootstrap generated from a single declaration — is
deliberately untouched; this amendment supplies its missing input (how many steps
there are) and answers none of it.

## What changes

### (1) The install boundary, ruled rather than re-derived per step

`installer/README.md` gains **§The install boundary**, which states the
bootstrap's job, the disposition every install step carries, and the test that
assigns one. {design-bearing}

**The bootstrap's job is five steps, and it is the whole of what is written
twice:**

1. resolve the package's own payload directory;
2. resolve the host to one Rust target triple;
3. read the payload's target roster and resolve the artifact and its sidecar,
   refusing a declared target whose pair is incomplete;
4. verify the artifact's SHA-256 against that sidecar;
5. execute the verified artifact, forwarding argv verbatim.

**Every step's disposition takes one of three values**, and the test that assigns
one is *what the step needs that the binary cannot supply at that moment*:

- **`bootstrap`** — the step must precede the invoke because the binary cannot
  select, verify or execute itself. Steps 1–5 above, and nothing else.
- **`behind-invoke`** — conditional install logic. Written once, in Rust. This is
  the default: TRAJECTORY.md §The interpreter policy rules that *everything
  conditional belongs on the far side of that invoke*, so a step claiming
  `bootstrap` owes a reason drawn from the previous bullet and no other.
- **`retired`** — the step exists only to serve a dependency the relocation
  removes, and ceases to exist rather than moving. `init`'s `jq` preflight is the
  worked case: nothing behind the invoke reads JSON with `jq`.

**Step 5 is *execute*, not *install*.** The tracked copy of the binary under the
consumer's gates directory is an install artifact with ownership semantics —
claimed against the manifest, carried in `files[]`, removed by `uninstall` — so
by the rule above it is conditional install logic and sits `behind-invoke`. The
bootstrap runs the artifact **in place, out of the payload**, where step 4 has
just verified it; a copy to a scratch path in order to run it would be a copy
with no reader. TRAJECTORY.md §The interpreter policy's "place the matching
binary" names the job *make the binary runnable*, and its very next sentence is
what settles which half of "place" this is.

**A `behind-invoke` step may spawn `bash`, and one does.** `gen-pre-commit.sh`
does not port — gate-sdk/SPEC.md §gen-pre-commit, ratified by the operator
2026-08-21 on criterion 6's single-producer rule — and `check-graph` is
`install: zero-config`, so a fresh consumer's day-one battery holds the generated
hook against `--emit` and the hook must therefore exist at install. The step is
consequently neither droppable nor portable. It is **not** stuck: the compiled
substrate already spawns `bash <emitter>` for exactly this generator from
`check-graph`'s assertion D, criterion 7 clears that spawn explicitly because
`bash` is on `GATE_SDK_PROGRAM_FLOOR` (gate-sdk/SPEC.md §lib/gate.sh) — the
payload's own assumed-program set and criterion 7's actual test, not
`context-kit/lib/toolfloor.sh`'s consumer-audience `PROBE_SET`, a different
kit's install-time probe roster that bash also happens to sit on — and the arm
declares it. So the step moves behind the invoke as a
declared spawn, and the *bootstrap* — which is what TRAJECTORY.md §The interpreter
policy's standing "assume no POSIX shell" obligation binds — spawns nothing.
Recorded because the natural reading is that this step is a third class that
neither moves nor re-implements, and a later session will re-derive the trilemma
and its dissolution otherwise.

### (2) `--install` — the invoke interface, and the non-gate class's first unbridged member

The binary gains an `--install <op>` arm family: the seam both bootstraps call,
specified so the two calls are byte-identical. {design-bearing}

It satisfies gate-sdk/SPEC.md §The non-gate arm's three properties — a top-level
`--`-prefixed flag resolved in `main` before the registry lookup and absent from
`--list`; no descriptor, registration or fixture pair; a named caller, which is
`installer/lib/init.sh` today and its PowerShell twin after fork 2.

**It is deliberately *not* a `BRIDGED_ARMS` member, and that is the property the
class has not carried before.** A bridged arm's knobs are resolved by
`gate_command`, a bash front-end that sources each owning kit's `lib/*.sh`. The
installer's caller is the bootstrap, which by objective 6 may not be assumed to be
a POSIX shell at all — so **every value the arm needs arrives as argv**, and the
arm reads no kit config and no knob. A bridged install arm would be unreachable
from the half of the boundary this amendment exists to make writable.

**Grammar.** `--install <op> [--<key> <value>]…`, `<op>` from a closed set, an
unknown `<op>` or an unknown key exiting 2.

**Channels, because the caller is a program in two languages.** *stdout* is a
wire: one record per line, tab-separated, `<verb><TAB><field>…`. *stderr* is the
adopter-facing report. **Exit status**: `0` performed — or, under `--dry-run`,
planned; `1` an adopter-actionable refusal; `2` usage or harness error, on
gate-sdk/SPEC.md §Fail-closed contract's terms.

**`--dry-run` is owed by every mutating op**, on installer/README.md §The verbs'
existing classifier: print the plan, write nothing, exit 0.

### (3) The relocation's own precondition: an uncovered platform must still install

A step may move behind the invoke only where **the binary is reachable on every
platform that step runs on today**, and today it is not. {design-bearing}

criterion 5's install model (gate-sdk/SPEC.md §Porting a gate to the binary
substrate) has three outcomes, and two of them leave `init` with no binary: a host
whose triple the payload's roster does not carry (`substrate-unavailable`), and a
host with no SHA-256 hasher (`digest-unverifiable`). Both **proceed** today,
omitting the compiled gates and declaring the omission in the consumer's
`gates.list`. That branch is what keeps a freshly vendored battery alive on an
uncovered platform — and once conditional install logic sits behind the invoke,
the same branch has nothing to run at all. **The failure mode the relocation
introduces is therefore not a smaller battery but no install**, and it is
introduced silently: nothing in tree asserts that a relocated step still runs on
an artifact-less host.

Two consequences, both recorded rather than acted on here:

- **Relocating the *unconditional* remainder of `init` is sequenced behind the
  artifact roster covering every supported platform.** That coverage is
  `platform-support-ci-matrix`'s ground and `native/targets.list`'s content; this
  amendment names the dependency and re-prioritizes nothing.
- **`digest-unverifiable` must become a refusal rather than an omission** at the
  same moment, because step 4 of the bootstrap is irreducible: a host that cannot
  hash cannot verify, and verifying before executing is the whole of the integrity
  claim. On Windows the branch is vacuous — PowerShell carries `Get-FileHash` —
  so the cost lands on a POSIX host missing both `sha256sum` and `shasum`.

**The rule this yields, and the reason the first cut is the one it is:** a step is
takeable now iff it *already* runs only when an artifact was selected. Such a step
costs an artifact-less host nothing, because on that host it never ran.

TRAJECTORY.md's tail measurement — roughly three hundred and fifty `init.sh` lines
not yet behind the invoke — is **not** an update target: it is a dated measurement
of 2026-08-24 and stays true as one.

### (4) The first cut: artifact placement and the gate-sdk config seam move behind the invoke

`installer/lib/init.sh:277-308` becomes one call to
`--install place-artifact`. {design-bearing}

It is the one block that satisfies delta 3's takeability rule outright: every
line of it with an observable effect sits inside `if [[ -n "$ARTIFACT_TARGET" ]]`
— the one line ahead of the guard is `SEAM="$GATES_DIR/gate-sdk-config.sh"`,
whose value is read only by the guarded lines that follow it, so an
artifact-less install never reaches anything the block does. It is also the
block whose PowerShell twin would be the most
intricate thing in the bootstrap — a digest re-read, an executable-bit set, and a
line-filtered atomic rewrite of a sourced shell file — and after the cut that twin
is zero lines.

**Invocation.** `init` runs `"$ARTIFACT_SRC"`, the payload artifact `select_artifact`
verified at step 4, **not** the installed copy — which on a first install does not
exist yet. This is the first live instance of delta 1's *execute in place* reading.

```
<artifact> --install place-artifact
    --root   <absolute repo root>
    --src    <the verified payload artifact>
    --dest   <repo-relative path for the installed binary>
    --seam   <repo-relative path of <gates-dir>/gate-sdk-config.sh>
    --target <rust target triple>
    --digest <the artifact's verified SHA-256>
    [--lock  <repo-relative manifest path>]
    [--force] [--dry-run]
```

**Behavior is `init`'s present behavior unchanged**, restated here only where the
argv makes it a contract rather than a local: `--dest` is claimed against the hash
`--lock` records for it and left alone when it differs, unless `--force`; the copy
is skipped when the recorded target, the recorded digest and the on-disk digest all
agree with `--target` and `--digest`, which is what makes a bare re-run leave the
tree byte-identical; the executable bit is set where the platform has one; `--seam`
is claimed on the same rule and then rewritten preserving every line except
`GATE_SDK_NATIVE_BIN=`, seeding the two shellcheck directives only when the file is
absent. `--lock` is optional and absent on a first install, where nothing is
claimed.

**Two stdout verbs, and each has one reader.** A third — an `unchanged` distinct
from a write — is deliberately **absent**: no caller distinguishes them, including
the `--dry-run` report, and a field with no reader is removed.

| verb | record | the caller's reader |
| --- | --- | --- |
| `own` | `own<TAB><path>` | `record "<path>"`, and `<path>` joins `STAGE` |
| `kept` | `kept<TAB><path><TAB><hash>` | `CHANGED+=("<path>")` and `record "<path>" "<hash>"` |

**What leaves the shell.** Both `claim` call sites in that block, the installed
copy's `digest_of` re-read, and the seam's brace-group rewrite. `claim`,
`record`, `lock_hash` and `digest_of` all stay — every other call site is
untouched by this cut.

**Its own oracles.** `installer/consumer-smoke/run-smoke.sh`'s main loop drives the
verify-then-write row on every profile and its binary-less leg drives the
omit-and-declare row, so both sides of the cut are already asserted behaviorally;
what the cut owes is that those arms stay green, plus `check-crate-arms` and a
rebuilt binary under `check-gate-binary-fresh`.

### (5) The fork-1 disposition roster lands on the queue entry at demotion

The per-step roster below is the fork-1 answer, and it is transcribed onto
`powershell-installer-surface` when the entry demotes, because this file is
deleted at merge and fork 2's session reads the entry. {mechanical}

**The transcription drops the line-span column and keeps the disposition
column**, plus the rev and the one-line re-derive instruction below. The spans
are the rot-prone half and the dispositions are the load-bearing half, and
`dated-measurement-restatement-class` is deliberating exactly that shape over the
deferred pool — the longest rot window in the tree. Taking the stricter reading
here costs the entry nothing, because a reader who needs a span re-derives it from
the step name in one grep. This amendment keeps the spans because it is deleted at
merge and its rot window is one iteration.

Measured off `installer/lib/*.sh` and `installer/bin/checkwright.sh` at
`d0b54919` — re-derive by reading `init.sh` top to bottom against the step names
below — execution order:

| step | `init.sh` | disposition |
| --- | --- | --- |
| argv parse, `--help` | 19-35 | behind-invoke (bootstrap forwards argv verbatim) |
| payload presence | 39-40 | **bootstrap** (step 1) |
| git work-tree resolve | 43-45 | behind-invoke |
| clean-worktree precondition | 48-51 | behind-invoke |
| `jq` preflight | 54 | **retired** |
| package version/commit read | 56-61 | behind-invoke |
| prior-manifest read, downgrade refusal | 62-82 | behind-invoke |
| profile → kit set | 84-99 | behind-invoke |
| `doctor` precondition | 89-94 | behind-invoke |
| `target_of_host` | 102-110 | **bootstrap** (step 2) |
| `select_artifact` roster + completeness | 115-133 | **bootstrap** (step 3) |
| `select_artifact` digest verify | 134-147 | **bootstrap** (step 4) |
| `claim` / `record` / `copy_in` | 155-194 | behind-invoke |
| kit-source vendoring | 196-202 | behind-invoke |
| `gates.list` synthesis, omission lines | 204-233 | behind-invoke |
| config-seam plan, per-kit seeding | 235-268 | behind-invoke |
| queue seeding | 270-275 | behind-invoke |
| artifact placement + seam write | 277-308 | behind-invoke — **this cut** |
| generated projections | 310-325 | behind-invoke, declaring a `bash` spawn |
| prior-roster carry-forward | 327-335 | behind-invoke |
| manifest emit | 338-380 | behind-invoke |
| `git add` / commit flow | 382-423 | behind-invoke |

Every sibling surface is `behind-invoke` whole: `doctor.sh`, `diff.sh`,
`uninstall.sh`, `update.sh`, and `lib/common/`'s `lock.sh`, `profile.sh`,
`recipe.sh` and `digest.sh` — `digest.sh`'s hasher resolution being the one whose
*logic* the bootstrap re-implements rather than calls, since step 4 needs it before
the binary runs. `bin/checkwright.sh` collapses into the bootstrap: its verb roster
is `lib/` itself, and after the relocation there is no `lib/`.

**Fork 1's answer, in one line: five bootstrap steps, one retirement, everything
else behind the invoke.** That count is fork 2's input and fork 2 stays open.

## Producers and consumers

**New interface — `--install <op>` (delta 2).** *Producer:* `main` in
`native/src/main.rs`, resolved before the registry lookup, alongside
`--source-stamp` and `--list`. Its enabling configuration is **none by
construction** — the arm reads no knob and no kit config, which is delta 2's own
ruling, so there is no config that some deployed configuration must set for the
producer to be reachable. *Consumer:* `installer/lib/init.sh`, which invokes the
verified payload artifact directly; after fork 2, the PowerShell twin on identical
argv.

**New interface — `--install place-artifact` (delta 4).** *Producer:* `init.sh` at
the point the present block sits, after every config seam is in place and before
the projections are generated, because the hook generator resolves
`GATE_SDK_NATIVE_BIN` off the seam this op writes. *Consumer:* `init.sh` reads its
stdout.

**Every field on the wire has a named reader**, and the readers are `init.sh`'s
existing locals rather than anything new:

| field | reader | transition |
| --- | --- | --- |
| `own` verb, `<path>` | `record()` → `WRITTEN`, `IS_WRITTEN`; then `STAGE` | before `manifest()` emits `files[]`, and before `git add` |
| `kept` verb, `<path>` | `CHANGED[]` | the changed-file report, and `--dry-run`'s "would leave alone" block |
| `kept` verb, `<hash>` | `record "<path>" "<hash>"` → `CARRIED` | `files_hash()`, so the entry carries init's hash and not the adopter's |

**Every argv key has a named reader inside the op**, which is what keeps the
signature from growing a slot nothing consumes: `--root` resolves every relative
path; `--src` is the copy source; `--dest` and `--seam` are the two claimed paths;
`--target` and `--digest` are compared against the manifest's `artifact` key and
the on-disk copy for the skip-rewrite branch; `--lock` supplies the two recorded
hashes `claim` compares; `--force` and `--dry-run` carry `init`'s existing meanings.

**Existing integration prose describing the prior flow is updated**, not left to
drift — the four sections below.

**The seam holds (CLAUDE.md §The provenance seam).** Nothing here is private rule
content: the op's grammar, its two verbs and the five bootstrap steps are generic
mechanism, and every consumer-specific value — the gates directory, the artifact's
basename, the repo root — arrives as argv rather than as a literal. `--install`
reads no `<KIT>_<KNOB>` and adds none, which is delta 2's ruling and not an
omission from the config-via-env convention.

## Existing sections updated

- `installer/README.md` §The gate binary — "The install location has one owner"
  and "Ordering is load-bearing" describe an inline `init` branch; both now
  describe the op's contract, and the section's honest bound on what the in-payload
  digest proves is unchanged (deltas 2 and 4).
- `installer/README.md` §init — the `--dry-run` paragraph, which the op must honor
  end to end rather than only in the caller (delta 4).
- `installer/README.md` §The consumer smoke — the artifact arm's placement and
  refusal legs drive the op rather than the shell block; the assertions themselves
  do not move (delta 4).
- `gate-sdk/SPEC.md` §The non-gate arm — the class gains its first **unbridged**
  member and its first caller that is neither a gate, a harness, nor a stage step
  (delta 2).
- `gate-sdk/SPEC.md` §Porting a gate to the binary substrate, criterion 5 — the
  omit-and-declare branch now bounds what the *installer* may relocate, not only
  whether a ported gate stays runnable (delta 3).

## Definition of Done

- [ ] **Causal completeness** — `--install` and `--install place-artifact` each
      name a reachable producer and a named consumer; both stdout verbs and every
      argv key have a named reader at a named transition.
- [ ] **Merged with no information lost** — §The install boundary reads as a
      section of `installer/README.md` rather than an appendix, and the four update
      targets above are edited in place.
- [ ] **Amendment deleted** — this file removed on merge; `ls installer/SPEC-*.md`
      empty.
- [ ] **Removals propagated** — the two `claim` call sites, the installed copy's
      `digest_of` and the seam brace-group are gone from `init.sh` with no dangling
      reference; `check-shellcheck` and `check-comment-tier` green on the shrunken
      file.
- [ ] **Roster transcribed** — delta 5's table lands on
      `powershell-installer-surface` at demotion, so fork 2's session reads it
      after this file is deleted.
- [ ] **Gaps filed** — cross-component gaps found during the work filed as debt
      tasks.
