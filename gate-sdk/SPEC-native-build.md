# SPEC amendment: native-build

Queue entry: `cargo-grant-committed-vs-overlay`. The shape was **operator-ruled
2026-08-09 at scope** and this amendment does not reopen it: the commit-time
crate build is routed through a **tracked `bin/` script granted by path**, like
every other kit tool. The glob over all `cargo build` and the exact grant of the
one manifest-pinned command were both refused there — the glob is the widest
grant for the least thought, the exact command is brittle against a manifest
path that moves. What this amendment adds is the mechanism, its fan-out, and the
seam ruling.

## What changes

### Delta 1 — `gate-sdk/bin/build-native.sh`, the one spelling of the crate build

*Work class: **design-bearing*** — a new kit tool, its fail-closed behavior, and
a consumer-tree disposition that has no precedent in `bin/`.

gate-sdk already owns the binary substrate's knobs (`GATE_SDK_NATIVE_CRATE`,
`GATE_SDK_NATIVE_BIN`, `GATE_SDK_NATIVE_TARGETS_FILE`) and their one-home
accessors in `lib/gate.sh`. The build command is the last fact about the crate
with no owner, spelled out longhand in nine places. This script becomes its
owner.

Contract:

- Invoked `bash gate-sdk/bin/build-native.sh [cargo-arg…]` from the repo root.
- Resolves the crate through the existing `gate_native_crate` accessor — **no
  new knob**, per the config-via-env convention (CLAUDE.md §Conventions
  established in gate-sdk: a knob default gains readers without gaining
  spellings).
- Runs `cargo build --release --manifest-path <crate>/Cargo.toml "$@"`, so a
  per-target build supplies `--target <triple>` and still uses one spelling.
  Trailing-argument passthrough is what lets the publish workflow's matrix leg
  route through the script rather than keeping a tenth copy of the literal.
- Exit code is cargo's, unmodified; stdout and stderr pass through. A caller
  that needs to capture build output (the consumer smoke's build leg) keeps
  working unchanged.

Fail-closed behavior, per §Fail-closed contract — each is **exit 2 with cause**,
never a silent success or a skip:

- `cargo` not on `PATH` — names the contributor-side toolchain floor
  (context-kit/SPEC.md §bin/env-probe) rather than printing a bare
  command-not-found.
- The resolved crate directory absent, or carrying no `Cargo.toml`. This
  message names the **consumer case explicitly**: a consumer receives a
  prebuilt, digest-verified binary per declared target and never the crate
  source (§Consumer payload), so a consumer tree that reaches this script has
  found a misuse, not a missing build. Stating it in the message is what keeps
  a vendored tool from reading as a broken one.

What it is **not**: not a gate. No `# graph:` manifest, no `# install:` header,
no fixture pair — the tool/gate distinction `queue-kit/bin/roadmap.sh` already
carries ("a tool, not a gate; no `# graph:` manifest"). It is `100755` in the
index like every other `*/bin/*.sh` (§check-exec-bit).

**The seam.** The script is generic kit mechanism: it builds *the configured
crate*, and carries no repo-specific path, no product constant, and no private
vocabulary. `native/` reaches it only as `GATE_SDK_NATIVE_CRATE`'s existing
default. It therefore vendors with the rest of `gate-sdk/` (`scripts/pack-installer.sh`
copies whole kit roots) with nothing private riding along.

### Delta 2 — every repo-side call site routes through the script

*Work class: **mechanical*** — a fixed roster of substitutions, each verified by
running the battery. The roster is enumerated under §Producers and consumers
below; it is a roster rather than a grep because two of the sites are not plain
substitutions.

### Delta 3 — the grant lands in the committed allowlist

*Work class: **mechanical***.

`.claude/settings.json` gains, beside the existing by-path kit-tool grants:

```
"Bash(bash gate-sdk/bin/build-native.sh)",
"Bash(bash gate-sdk/bin/build-native.sh *)",
```

That is the whole point of the unit: a fresh clone that installs the hooks
receives the grant for the command the hook effectively requires. The
gitignored overlay's `Bash(cargo build *)` / `Bash(cargo test *)` entries are
**not** governed by this repo and are not this amendment's to delete; what
changes is that a clone no longer depends on them to work. `cargo test` keeps
whatever local grant it has — it is not on the commit-time path
(§check-gate-binary-fresh: `cargo test` does not discharge the build).

### Delta 4 — `### build-native` joins the per-component contracts

*Work class: **design-bearing*** — the contract above, written as the SPEC's own
section, placed after `### install-hooks` and before the checks; plus one bullet
in `gate-sdk/README.md`'s `bin/` list. `check-readme-roster` binds `checks/`
only, so the README bullet is convention rather than gate-held, and is stated
here so build adds it rather than discovering the omission at close.

## Producers and consumers

The one new interface is the script's invocation. Its **producer** is any
session, hook-installing clone, workflow step, or gate help-path that needs the
crate built; its **consumers** are the call sites below. The enabling
configuration is `GATE_SDK_NATIVE_CRATE`'s existing default, which this repo
does not override — so the producer is reachable in the deployed configuration
and not only under a fixture.

**The script emits no new state and no new fields**, so the field-reader arm of
the causal-completeness check is vacuous by construction rather than skipped:
nothing is written, nothing is parsed, and the only value crossing the boundary
is cargo's exit code, whose reader is the invoking shell.

**Every existing reader of the longhand command, and what each becomes.** This
is the roster Delta 2 executes; an update target no delta claims is what reaches
build as an orphan, so it is enumerated rather than left to a grep.

| Reader | Today | Becomes |
|---|---|---|
| `gate-sdk/lib/gate.sh` `gate_command` | build-it message on an absent binary | cites the script |
| `gate-sdk/lib/consumer-smoke.sh` | build-it message | cites the script |
| `gate-sdk/checks/check-gate-binary-fresh.sh` | `REBUILD=` composed from `$CRATE` | `REBUILD=` names the script |
| `gate-sdk/checks/check-gate-substrate-parity.sh` | `help:` line | cites the script |
| `gate-sdk/checks/check-reads-couples.sh` | `help:` line | cites the script |
| `.github/workflows/gates.yml` | `run:` step | invokes the script |
| `.github/workflows/publish.yml` | per-target matrix build | invokes the script with `--target "$TARGET"` |
| `installer/consumer-smoke/run-smoke.sh` build leg | `cargo build … "$REPO/$NATIVE_CRATE/Cargo.toml"` | invokes the script **with cwd `$REPO`** |
| `CLAUDE.md` §Housekeeping | longhand as the commit-time requirement | the script |
| `CONTRIBUTING.md` | longhand | the script |
| `docs/install.md` | longhand | the script |
| `gate-sdk/SPEC.md` §What the dispatch seam does not settle | longhand | the script |
| `gate-sdk/SPEC.md` §check-gate-binary-fresh's remediation prose | longhand | the script |

**The two that are not plain substitutions**, named so build does not discover
them:

1. `installer/consumer-smoke/run-smoke.sh` resolves the crate as
   `$REPO/$NATIVE_CRATE` because it runs from a scratch tree. The script
   resolves the crate **relative to cwd**, so this call site invokes it with cwd
   set to `$REPO`. Its surrounding assertions are unchanged: it still captures
   the output into `build_out`, still expects the binary at
   `$REPO/$NATIVE_CRATE/target/release/$NATIVE_BIN`, and still fails the leg if
   the build leaves the worktree dirty.
2. `.github/workflows/publish.yml` builds **per target** inside a matrix. It is
   routed through the script only because Delta 1 passes trailing arguments
   through; without that passthrough this row would stay longhand, and the row
   is listed here so the passthrough is understood as load-bearing rather than
   as generality for its own sake.

**Red conditions of the readers this touches**, per the causal-completeness
check's point 5. This delta does not narrow a corpus — it substitutes one string
for another inside files that stay in every corpus — so the narrowing hazard
does not arise. The readers that could nonetheless flip are named with what
makes each red, not with what it is about:

- `check-docs-cmd` reds when a fenced command in a governed doc names a path
  that does not resolve. `CLAUDE.md`, `CONTRIBUTING.md` and `docs/install.md`
  all sit in `CANON_KIT_MANIFEST_FILES`, so the new fenced spelling must resolve
  to a tracked file — it does, once Delta 1 lands, and the two must therefore
  land in **one commit**.
- `check-exec-bit` reds when a tracked `*/bin/*.sh` carries index mode other
  than `100755`. A `Write`-authored script acquires `100644`, so the mode is set
  explicitly rather than assumed.
- `check-comment-tier` reds on a non-directive comment in a governed shell
  source. The script's comments are `# spec:` / `# usage:` directives only.
- `check-gate-binary-fresh` reds on a stale binary. Its remediation *string*
  changes; its predicate does not, and its fixture pair asserts the verdict
  rather than the message.
- `check-docs-mirror-fresh` reds when `docs/<kit>/` diverges from the kit
  source. `docs/gate-sdk/SPEC.md` is a generated projection, so the mirror is
  regenerated rather than hand-edited (docs/site-architecture.md §Generated
  projections).
- `check-tree-terms` and `check-commit-msg` are **not** readers here. Their
  banned-pattern set is the leak vocabulary, which this delta does not touch —
  recorded because the shape of "a banned literal string" invites the wrong
  gate.

## Enforcement, and the residual named rather than gated

Enforcement-first says the fix and the gate that catches it land together, **and
that removing the duplication outranks gating it**. Delta 2 removes the
duplication outright: after it, one file spells the build and thirteen readers
cite it. That discharges the doctrine's stronger half, and it is why this
amendment ships **no new gate**.

The residual, stated honestly rather than left to look covered: nothing stops a
*future* session from writing a fresh longhand `cargo build --release
--manifest-path …` into a new file. No existing gate's corpus and no existing
gate's predicate reaches that, and inventing one would mean a banned-literal
gate whose pattern list is this repo's own build command — a consumer-config
vocabulary and a whole mechanism for one string. The judgment is that the
recurrence is not yet attested: the duplication being removed accumulated
because there was no owner to cite, and Delta 4 gives it one. If it recurs, that
is a costed filing, not a silent regrowth.

## Existing sections updated

- **gate-sdk/SPEC.md §Per-component contracts** — gains `### build-native`
  (Delta 1's contract), placed after `### install-hooks`. Owned by Delta 4.
- **gate-sdk/SPEC.md §What the dispatch seam does not settle** — the paragraph
  reading "a fresh clone cannot commit until it has run `cargo build --release
  --manifest-path native/Cargo.toml` once" names the script instead. The ruling
  it states is unchanged; only its spelling of the command moves. Owned by
  Delta 2.
- **gate-sdk/SPEC.md §check-gate-binary-fresh** — its remediation prose names
  the script. Owned by Delta 2.
- **gate-sdk/SPEC.md §Layout and configuration** — no new knob joins the
  roster; recorded as a deliberate non-update so a reader does not go looking
  for one.
- **gate-sdk/README.md** — one bullet in the `bin/` list. Owned by Delta 4.
- **CLAUDE.md §Housekeeping** — the `native/` bullet's commit-time requirement
  is the script; the "and `cargo test` does not discharge it" clause stands
  unchanged. Owned by Delta 2.
- **CONTRIBUTING.md**, **docs/install.md** — the longhand becomes the script.
  Owned by Delta 2.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **The script is `100755` in the git index** (`git ls-files -s
      gate-sdk/bin/build-native.sh`), not merely in the worktree.
- [ ] **Every reader routed** — the §Producers and consumers table is walked row
      by row, and `grep -rn 'cargo build --release'` over the tracked tree
      returns only `gate-sdk/bin/build-native.sh`, the SPEC section that
      documents it (plus its generated `docs/` mirror), and `TASK-QUEUE.md`
      entries quoting the pre-change state.
- [ ] **The committed grant is present** and the tree is workable from a clone
      that installed the hooks and never wrote a local overlay.
- [ ] **Generated projections regenerated** — `docs/gate-sdk/SPEC.md` mirror,
      and any other projection the fan-out in docs/site-architecture.md
      §Generated projections names.
- [ ] **Full battery green** (`bash gate-sdk/bin/run-gates.sh`) plus the
      gate-sdk fixture suite, and the crate built through the new script.
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
