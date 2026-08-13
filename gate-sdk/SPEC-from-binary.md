# SPEC amendment: from-binary

`gate-sdk/bin/upgrade-smoke.sh` phase 1 asserts *"the FROM tag alone is healthy
under zero config"* and cannot, for any `.gate`-dispatched member: the harness
never places a FROM-appropriate binary. This amendment rules the fix, retiring
the `upgrade` suite's baseline-held `fail`
(`.workflow/validate-baseline.txt`: `upgrade upgrade fail
upgrade-smoke-from-binary-pairing`). It does not restate §upgrade-smoke's phase
structure or §Consumer smoke's scratch-consumer contract.

## The defect, reproduced rather than reasoned

`csmoke_place_binary` (`gate-sdk/lib/consumer-smoke.sh:5-24`) resolves the binary's
source tree from its **own** `BASH_SOURCE` (`:7`) — always the invoking checkout —
and is called once, from `csmoke_vendor_and_install:42`, which `upgrade-smoke.sh:62`
invokes at FROM-phase setup only. Phase A's kit swap
(`upgrade-smoke.sh:77-81`) is a bare `cp -R` over directories carrying
`smoke/install.sh`; `native/` carries none, so it is never a `fromroots`/`toroots`
member and the binary is never re-placed. **One binary — the current tree's —
serves both phases.**

Run at this iteration's cut, with FROM resolving to `v0.22.0`
(`upgrade-smoke.sh:18`, newest `v*` tag):

```
upgrade-smoke: FAIL(env) — the FROM baseline (v0.22.0) is not green under zero
config; the tag itself is broken, not an upgrade finding
3 of 75 gates FAILED: check-gate-substrate-parity check-action-pinning check-action-gh-repo
```

All three are pairing artifacts, and each was verified against both substrates:

| Gate | Exit | Cause |
|---|---|---|
| `check-action-pinning` | 2 | `native/src/walk.rs:11-15` hard-requires `GATE_SDK_KNOB_GATE_PRUNE_DIRS`; `v0.22.0`'s `gate_command` carries no `--knobs` query, so the bridge never emits it |
| `check-action-gh-repo` | 2 | the same |
| `check-gate-substrate-parity` | 1 | `native/src/main.rs:49-53` prints `<subcommand>\t<owning-kit>`; `v0.22.0`'s vendored parser (`:94-97`) reads each whole row as one bare subcommand name |

**The FROM tag is not broken.** Built from its own source, `v0.22.0`'s binary
emits a one-column `--list` and carries no `--knobs` subcommand at all — exactly
what its own vendored shell expects, and its `walk.rs` names
`GATE_SDK_KNOB_GATE_PRUNE_DIRS` nowhere. FROM's shell and FROM's binary are
self-consistent; the harness pairs FROM's shell with **TO's** binary and reports
the mismatch as a broken tag. Phase 1's own message
(`upgrade-smoke.sh:70`) therefore mis-attributes every finding it can produce for a
dispatched member.

## Two premises the deliverable was framed on, and what probing changed

The queue entry offers two shapes: give phase 1 a FROM-built binary, or narrow
phase 1's assertion to exclude `.gate`-dispatched gates. Probing moves both.

**(i) Narrowing is self-defeating, and the port is what makes it so.** The
2026-08-09 directive ports the whole corpus. A phase-1 assertion that excludes
dispatched members shrinks exactly as the dispatched set grows, and at the port's
completion asserts nothing whatever. The entry states this as a cost; it is
better read as a disqualification, because the option's value goes to zero on the
trajectory the repo is committed to.

**(ii) The per-tag build was priced as expensive and is not — but it cannot be
taken the obvious way.** Measured at this cut: `git worktree add --detach v0.22.0`
takes **35 ms** and a cold `cargo build --release` of the archived crate takes
**546 ms**, producing a 536 KB binary. The crate declares no dependencies at any
tag — enforced by `native/src/walk.rs`'s own build-failing test — so there is no
dependency graph to resolve and the figure is the whole cost. Against a suite that
vendors, installs and runs a 75-gate battery twice, ~0.6 s per ref is not a cost
that decides anything.

**The obvious mechanism is blocked, and this is the finding that shapes the
delta.** `upgrade-smoke.sh:38-43` materialises both refs with `git archive`, and a
`git archive` tree is not a repository. `native/build.rs:57-69` computes the
source stamp by running `git ls-files` under `CARGO_MANIFEST_DIR` and **panics** when
that fails, with its own message: *"This crate builds inside its own git checkout
by construction (it is never vendored)"*. Building FROM's crate out of the archive
tree therefore fails at 120 ms with a build-script panic, not with a compile error —
a failure mode that reads as a broken tag if it is met at implementation time rather
than named here. **A ref's binary is built from a detached git worktree at that ref,
never from an archive.**

## What changes

**(1) `csmoke_place_binary` takes the binary's source tree as an argument.**
[design-bearing] Its `host="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"`
(`consumer-smoke.sh:7`) is the defect's root: a shared library deciding, from its
own location, whose artifact a caller receives. The parameter makes the caller
name it, and `run-consumer-smoke.sh` / `context-kit/smoke/agents-md.sh` pass the
invoking repo exactly as today, so their behavior is unchanged. Everything else in
the function is kept: the descriptor-count derivation (`:11-15`) that decides
whether a binary is needed at all, and the fail-closed refusal (`:17-21`) when one
is needed and absent.

**(2) A per-ref binary builder, in `upgrade-smoke.sh`.** [design-bearing] Given a
ref: add a detached worktree at it, `cargo build --release` its `native/`, take the
artifact, remove the worktree. The build output goes to a scratch
`CARGO_TARGET_DIR` so the host's `native/target/` is untouched — the suite writes
nothing in-tree, the standing constraint every smoke here already meets. Cleanup is
trap-registered, because a leaked worktree outlives the run and the repo's
worktree-prune mechanism should not have to collect after a validate suite.

**(3) Each phase is paired with its own ref's binary.** [design-bearing] Phase 1
places FROM's; the phase-A swap (`upgrade-smoke.sh:77-81`) re-places TO's, in the
same motion that replaces the kit directories, because that swap *is* the upgrade
transition. Phase 1's claim then becomes true as written, and phase B's — TO shell
against TO binary — becomes true **by construction** rather than by the accident
that the host tree happened to be TO.

**(4) A FROM ref carrying no crate is a first-class branch and needs no new
code.** [mechanical] `v0.22.0` is the first tag carrying `native/`; every earlier
tag has none, and a consumer may set `GATE_SDK_UPGRADE_FROM` to one. The existing
descriptor-count derivation already handles it: a FROM whose vendored kits carry no
`*.gate` needs no binary, `csmoke_place_binary` returns 0 without one, and delta (2)
is never invoked for that ref. Recorded as a verified branch rather than left for a
build session to discover it works.

**(5) A FROM crate that will not build is exit 2, under phase 1's existing
rule.** [design-bearing] `upgrade-smoke.sh:69-73` already rules a red FROM baseline
"a broken tag: exit 2, not an upgrade finding". A FROM crate that fails to compile
under the current toolchain is the same class — an environment or tag fact, never an
upgrade finding — and takes the same exit with a message naming the ref and the
build's own stderr. The one thing it must **not** do is fall back to the host's
binary, which is the present behavior wearing a fallback's clothes.

**(6) A second inconsistency the same delta closes, found here rather than
later.** [design-bearing] `upgrade-smoke.sh:38-43` archives TO, so the kits under
test are TO's **committed** content while the binary is the host's **working-tree**
build. A dirty working tree therefore tests committed kits against an uncommitted
binary. Delta (3) makes both sides committed-at-the-ref, which is what a tool named
"upgrade FROM tag TO tag" should have been comparing. Named as its own delta
because a reader who sees only the FROM half will restore the host-binary shortcut
for TO on the ground that TO is usually HEAD.

**(7) The determinism assertion is untouched, and this was verified rather than
assumed.** [mechanical] §upgrade-smoke rules that an installed binary is "neither a
determinism finding nor a determinism exemption", and that widening the assertion to
name it "would claim coverage this tool cannot have". Re-placing the binary at phase
A does not disturb that: the scratch consumer's `.gitignore` carries
`gate_native_bin`'s path (`consumer-smoke.sh:32-33`), so the placed artifact never
appears in the `git status` the determinism assertion reads
(`upgrade-smoke.sh:93-110`) and no allow-set entry is added. The ruling stands
unchanged and is cited, not amended.

**(8) The baseline row retires with the entry.** [mechanical] The `upgrade` row in
`.workflow/validate-baseline.txt` moves from `fail` to `pass` and drops its slug
when the suite goes green. `check-evidence-baseline.sh:68-75` requires a `fail`
row's slug to resolve to a live queue task, so the row and the entry's Done move
happen together or the gate reds — and this is close's step, not build's.

## Producers and consumers

**Changed interface: `csmoke_place_binary`'s signature (delta 1).**
- *Producer* — the two callers that already exist plus `upgrade-smoke.sh`; the
  argument is the checkout whose `native/` was built.
- *Consumers* — the scratch consumer's `gate_command`, which resolves
  `GATE_SDK_NATIVE_BIN` to the placed path at dispatch time for every `.gate`
  member in the vendored `gates.list`.
- *Every field has a named reader* — the one new parameter is read at exactly one
  transition, the `cp` at `consumer-smoke.sh:23`. No field is added to the scratch
  consumer's own state.
- *Enabling config actually emitted* — none is added. `GATE_SDK_NATIVE_BIN` carries
  a kit default (`gate-sdk/lib/gate.sh:198-200`), so the placed path resolves with
  the consumer setting nothing, exactly as today.

**New capability: a per-ref binary build (deltas 2, 3, 5).**
- *Producer* — `upgrade-smoke.sh`, twice per run: once before phase 1 with FROM,
  once at the phase-A swap with TO.
- *Consumer* — `csmoke_place_binary`, receiving the built checkout as delta (1)'s
  argument.
- *Named reader at a named transition* — FROM's artifact is read by the phase-1
  battery (`upgrade-smoke.sh:68`); TO's by the phase-B battery (`:148`). Neither is
  produced at a transition where it is not read, which is exactly the property the
  present single placement violates.
- *Enabling config actually emitted* — `cargo` must be on PATH. This is a **new
  requirement on the suite**, not on an adopter: `upgrade-smoke` is a validate-suite
  tool in this repo, whose contributors already need the toolchain for
  `check-crate-arms` and `bin/build-native.sh`. It reaches no consumer and does not
  touch `GATE_SDK_PROGRAM_FLOOR`, which bounds what a *gate rule* may invoke.
  Stated because a reader meeting a new `cargo` dependency will reasonably ask
  whether criterion 7 binds, and it does not.

**Red conditions of the readers this change touches.** The delta neither narrows
nor widens a scanned corpus, so §The causal-completeness check point 5's
non-monotone trap does not arise; the readers are the suite's own assertions.

- **Phase 1's battery assertion** (`upgrade-smoke.sh:69-73`) — reds on the battery
  not matching `All [0-9]+ gates passed`. Today it reds on three pairing artifacts;
  after delta (3) a red is a genuine FROM-tag fact, which is what its message
  already claims.
- **The determinism assertion** (`:93-110`) — reds on a `git status` entry outside
  the allow-set. Cleared **by inspection** and by delta (7)'s verified `.gitignore`
  entry, not by argument.
- **Phase B's declaration assertion** (`:114-180`) — reds when the failing set is
  not a subset of TO's tightened-gates declaration. Unchanged in what it asserts;
  delta (3) makes its binary TO's by construction, which can only remove
  cross-substrate noise from the failing set.
- **`check-evidence-baseline`** — reds on a `fail` row whose slug resolves to no
  live queue task. Delta (8) is the discharge, at close.
- **`check-gate-binary-fresh`** — reds on a host binary stale against the host's
  source stamp. The per-ref builds write to a scratch `CARGO_TARGET_DIR` and never
  touch `native/target/`, so its verdict is unchanged. Stated because a delta that
  runs `cargo build` invites the wrong edit.

## Existing sections updated

- **gate-sdk/SPEC.md §upgrade-smoke** — owned by deltas (2)-(6): phase 1's claim
  gains the binary it needs to be true, the phase-A step list gains the TO-side
  re-placement, the no-crate-at-FROM branch and the build-failure exit are stated,
  and the committed-vs-working-tree correction from delta (6) is recorded. Its "What
  it does not cover" paragraph is **narrowed, not deleted** — the vendored-kit-only
  reach still holds for the installer path; what leaves that paragraph is the gate
  binary alone, and only for the phases this delta pairs.
- **gate-sdk/SPEC.md §upgrade-smoke, the gate-binary paragraph** — owned by delta
  (7): the determinism ruling is unchanged and gains one sentence saying why
  re-placing the binary does not reach it (the `.gitignore` entry), so a later
  reader does not read delta (3) as having quietly widened the assertion.
- **gate-sdk/SPEC.md §Consumer smoke** — owned by delta (1): `csmoke_place_binary`'s
  contract line states that the binary's source tree is the caller's to name, and
  why a library deciding it from its own location is the defect.
- **gate-sdk/SPEC.md §The port-candidate criteria, criterion 5** — owned by the
  defect statement: the criterion's *"a `.gate` descriptor under a vendoring kit
  root reaches every consumer; the binary does not"* now has a second worked
  consequence beside the vendored-consumer one — a **harness** that vendors kits per
  ref and cannot vendor the binary per ref. One sentence, cited to §upgrade-smoke.
- **`.workflow/validate-baseline.txt`** — owned by delta (8), at close.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather than
      at the commit while sibling amendments are in flight.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
