# SPEC amendment: pack-root

`scripts/pack-installer.sh` gains a caller-supplied root, and
`installer/consumer-smoke/run-smoke.sh` passes its own. The smoke stops being
able to pass green while asserting against a tree it never packed.

**The defect, restated from the measured shape rather than the filing.** The
packer resolves the tree it packs from the **current directory**
(`scripts/pack-installer.sh:13`, `git rev-parse --show-toplevel`, then `cd
"$ROOT"`). The smoke resolves the tree it asserts about from **its own script
path** (`installer/consumer-smoke/run-smoke.sh:5-6`, off `BASH_SOURCE[0]`).
When the two are not the same tree — a linked worktree, a second checkout, an
invocation by absolute path — the run packs one tree, asserts against another,
and prints an ordinary success. The `PACK:` line
(`scripts/pack-installer.sh:144`) even names the *other* tree's commit, because
`COMMIT` is the packer's own `git rev-parse HEAD` against its own `$ROOT`. This
is not a red that needs explaining; it is a **green that asserts nothing about
the tree under test**.

**The queue entry left the choice open between a parameter and a refusal, and
this amendment takes the parameter — because the objection that held it open is
false as measured.** The entry deferred the parameter because it "touches every
call site." There are **five** call sites, four of them in one file
(`run-smoke.sh:64`, `:377`, `:478`, `:550`) and the fifth in
`.github/workflows/publish.yml:173`. More decisively: the parameter is
**optional**, defaulting to today's cwd resolution, so the workflow call site
changes not at all and only the four smoke calls gain an argument. The cost the
deferral priced was never incurred.

**And the refusal alternative cannot be built where the entry imagined it.** A
refusal-on-mismatch requires the packer to know the caller's root — which is the
parameter, arriving under a different name. The packer alone has nothing to
compare its resolved root *against*; the disagreement is only visible to a
caller that holds both. So the parameter is not the more expensive of two
options, it is the precondition of the other one.

## What changes

### 1. `scripts/pack-installer.sh` gains `--root <dir>`

A fourth flag beside the existing `--version`, `--out` and `--artifacts`. When
passed, it is the tree the packer packs and stamps; when omitted, the packer
resolves the root exactly as it does today. **[design-bearing]**

The value is validated, not merely accepted: the directory must exist and must
be the top level of a git work tree (`git -C <dir> rev-parse --show-toplevel`
resolving to itself), else the packer refuses with exit 2. A `--root` naming a
subdirectory of a work tree is a refusal rather than a silent promotion to its
toplevel — silently correcting the caller's root is the class of behavior this
whole amendment exists to remove.

Every subsequent `git` invocation and every relative path in the script reads
that root. The script's existing `cd "$ROOT"` at line 17 is what makes this a
small change: the resolution is already funnelled through one variable, and this
delta only widens how that variable may be set.

### 2. The `PACK:` line names the root it packed

`scripts/pack-installer.sh:144` currently reports version, commit, kit count and
artifact count. It gains the **resolved root**. **[mechanical]**

The commit alone is what made the defect survive two green runs: a twelve-character
hash names the wrong tree just as plausibly as the right one, while a path names
it legibly. This is the diagnostic half of the fix and it stays useful for every
caller, including the ones that never pass `--root`.

### 3. `run-smoke.sh` passes its own root at all four call sites

Each of `installer/consumer-smoke/run-smoke.sh:64`, `:377`, `:478` and `:550`
gains `--root "$REPO"`. **[mechanical]**

`$REPO` is already computed at lines 5-6 and is already used to *locate* the
packer (`bash "$REPO/scripts/pack-installer.sh"`). The smoke therefore already
holds the correct root and already hands it to the packer as part of a path — it
simply has no way to say that the path is also the subject. After this delta the
smoke's packed tree and its asserted tree are the same tree **by construction**,
which is the property the queue entry asked for.

### 4. The cleanliness precondition, re-verified rather than hoisted — it is already in the suite's preflight

**Corrected at align, against the tree rather than the filing.** `run-smoke.sh`
already asserts its `$REPO` worktree is clean once, in its preflight, before any
of the ~10-minute run (`installer/consumer-smoke/run-smoke.sh:23-24`, landed
2026-07-26 in `bb4cc6594`, weeks before this iteration). **[mechanical — confirmed,
not added]**

The delta as filed described this as work to do ("hoisted to the suite's own
preflight"), on the framing that the packer's dirty-worktree refusal
(`scripts/pack-installer.sh:44`) is the only cleanliness check in the run and
fires late because the suite calls the packer four times. That diagnosis of the
*lateness* is correct and is what delta 5 answers. But the *remedy* it named —
add a single preflight assertion — is not new work: the preflight assertion
already exists, already runs once, and already sits before the suite's first
`printf 'build …'` line. This amendment's build session owes **no code change**
for this delta; it owes only confirming the existing check still holds after
deltas 1–3 land (it does — it asserts against `$REPO`, the same variable delta 3
threads into the packer, so no divergence is introduced) and citing the existing
line range in the merged spec rather than describing a hoist that never happens.

### 5. The packer's refusal message distinguishes a timing fact from a broken tool

The dirty-worktree refusal names the **root it resolved** and states that the
check is per-invocation. **[design-bearing]**

Delta 4 removes the common case but cannot remove the real one: a concurrent
session dirtying the tree mid-run — which is an ordinary thing during a run that
long, and is the scenario the entry names. That refusal will still fire late,
and the entry's complaint about it is precisely that it reads as a broken
installer rather than as a precondition checked at a moment the caller did not
choose. A message that says which tree and that the check is per-invocation
makes the late refusal self-explaining, which is the only fix available for a
condition that genuinely arises late.

### 6. `-h` / `--help` prints usage on stdout at exit 0

The packer today has no help branch, so `--help` falls to its `*)` arm and
prints `pack-installer: unknown argument: --help` on stderr at exit 2
(`scripts/pack-installer.sh:22-29`). It gains a help case printing the usage
block already written at lines 4-6. **[mechanical]**

**This is adopted on its merits and *not* because §The `bin/`-tool contract
binds — the scope question is ruled here so a later reader does not take this
delta as evidence the contract reaches wider than it says.** That contract
opens "A kit's `bin/` tools", and `scripts/pack-installer.sh` is in the
consumer's own `scripts/`, under no kit. It does not bind, and this amendment
does not extend it. What is adopted is the cost that section *measures*: a
session that "ran a stage writer with `--help`, got `'--help' is not a
lifecycle stage` in place of usage, and went three guards deep working around a
contract the usage text would have told it did not exist." The packer now has
four flags, and this delta adds the fourth — the discoverability cost is real
here on its own terms.

`--` is not adopted: it ends option processing in favor of free-text
positionals, and the packer has none.

## Producers and consumers

**`--root <dir>` (new interface).**

- **Producer** — `installer/consumer-smoke/run-smoke.sh` at its four pack call
  sites, passing `$REPO` (resolved at `:5-6`). The enabling configuration is
  emitted **everywhere the producer runs**, because `$REPO` is unconditional
  script-path derivation with no knob behind it and no branch that leaves it
  unset. This is the point causal-completeness point 1 exists for: a parameter
  whose only producer set it conditionally would be dead outside tests, and this
  one cannot be.
- **Consumer** — `scripts/pack-installer.sh`'s argument loop (`:22-29`), which
  assigns it to the `ROOT` the script already `cd`s into at `:17`. Mechanism: a
  command-line argument, read once at invocation.
- **The non-producer is named too, deliberately.**
  `.github/workflows/publish.yml:173` does **not** pass `--root` and is not
  changed. Its job is a single fresh `actions/checkout` with cwd at the workspace
  root, so cwd resolution is already correct there; making the flag mandatory
  would have forced a change on the one caller that never had the defect.

**`--root`'s validation refusal (new error behavior).**

- **Producer** — the packer, when `--root` names a non-directory, a non-work-tree,
  or a work-tree subdirectory.
- **Consumer** — the invoking caller's exit-status check. Both smoke and workflow
  call sites already treat a non-zero pack exit as fatal (`run-smoke.sh:64-65`
  routes it to `blocked`), so the new refusal has a reader at every call site
  without any of them changing.

**The `PACK:` root field (new field).**

- **Reader** — `run-smoke.sh`'s `say "$(grep -m1 '^PACK:' …)"` at each call site,
  which echoes the line into the suite's transcript, and the human reading that
  transcript. The transition it is read at is exactly the one the defect was
  missed at: a reviewer scanning smoke output to confirm the run described the
  tree under review.

**The preflight cleanliness assertion (pre-existing state, confirmed rather than
added).**

- **Producer** — `run-smoke.sh`'s preflight, unconditionally, against `$REPO`,
  already live at `:23-24` since `bb4cc6594` (2026-07-26).
- **Consumer** — the suite's own `blocked`/`fail` path, before any profile
  installs.

**This delta narrows no corpus.** It adds a flag and a field (deltas 1–2), a
call-site change (delta 3) and a message (delta 5); it prunes no file, no glob
and no declaration, so causal-completeness point 5's red-condition enumeration
has no subject. The existing dirty-worktree refusal is **retained**, not
replaced — delta 5 adds detail to the late message rather than moving the
check, so no fail-closed path is removed. Delta 4 changes no code: the
preflight assertion it describes was already load-bearing before this amendment
was filed.

## Existing sections updated

- **installer/README.md §The consumer smoke** — owned by deltas 1 and 3. The
  paragraph beginning "**Run it with the current directory inside the tree under
  test**" states the disagreement as an *invocation requirement* and closes
  "today nothing detects it, and a version that refuses on a mismatch would
  refuse exactly the invocations this sentence tells you not to make." Both
  clauses stop being true: the smoke now names its own root, so cwd no longer
  selects the packed tree and the `env -C <clone>` instruction is obsolete for
  this path. The paragraph is rewritten to state the property that replaces the
  requirement — the smoke packs the tree it lives in, by construction — and to
  record that the requirement is retired rather than merely unstated, so a later
  reader does not restore it.
- **gate-sdk/SPEC.md §The port-candidate criteria, criterion 5** — owned by
  deltas 1 and 3. Its measurement-ordering paragraph currently instructs: "That
  checkout is reached **by cwd, not by path**: the packer tests dirtiness where
  it is invoked, so running it by absolute path while the cwd sits in the dirty
  tree packs the dirty tree and refuses, however clean the checkout the path
  pointed at." That instruction is written against the defect this amendment
  removes. It is updated to the post-`--root` truth, and this is the delta most
  likely to be missed, because the prose lives in a *third* component and reads
  as being about the port rather than about the packer.
- **installer/README.md, the `pack-installer.sh` usage/flag description** —
  owned by deltas 1, 2 and 6: the flag roster gains `--root`, the `PACK:` line
  description gains the root field, and the help behavior is stated.
- **CLAUDE.md §Housekeeping** — owned by delta 1. Its `installer/` bullet
  describes the packer's knob surface (`INSTALLER_PACK_TMP_DIR` its scratch
  knob, writing nothing in-tree); it gains `--root` only if the bullet's tier
  admits a flag roster. Checked at merge and left unchanged if it does not —
  named here so the check happens rather than being discovered later.

## Definition of Done

- [ ] **Causal completeness** — `--root`, its refusal, the `PACK:` root field
      and the preflight assertion each have a named reachable producer and a
      named consumer; the one caller that deliberately does not produce it is
      named with its ground.
- [ ] **Merged with no information lost** — deltas integrated into
      installer/README.md §The consumer smoke and its packer description, and
      into gate-sdk/SPEC.md criterion 5.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      `installer/` (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec and doc for the retired
      "run it with the current directory inside the tree under test" requirement
      and for the cwd-not-path instruction, including the `docs/` mirror.
- [ ] **Gaps filed** — cross-component gaps found during the work filed as debt
      tasks.
