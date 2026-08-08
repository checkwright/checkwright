# SPEC amendment: installer-lifecycle-verbs

Root-level, no owning kit: `installer/` is repo-root-governed (CLAUDE.md
§Housekeeping) and this change reaches two kits besides it, so it merges into
installer/README.md, doctrine-kit/SPEC.md and gate-sdk/SPEC.md rather than into
one component spec — the same placement `SPEC-activation-installer.md` took.

`init` and `doctor` shipped in phase 1. This is phase 2: the three verbs that
manage an install *after* `init` has made one. Nothing here revises the manifest
schema — all six `checkwright.lock` fields already carry a phase-1 reader, and
these verbs are second readers of data that already exists.

That includes the constraint the filing carried forward: a **seventh field**
proposed for `update`'s benefit alone was deferred with this unit, and it stays
refused. Every verb below is specified against the six fields that exist. The
one shape this does add is a *narrowing* of the existing object rather than a
field (§D3, the residual manifest), and it is argued there against the wire
key's own precedent.

## The verb taxonomy this settles

The queue entry poses `diff` without saying what it is, and the entry predates
`doctor`. Scope left the question open; it is ruled here.

**`diff` is a fourth verb, not a widening of `doctor`.** Three separable
questions, three surfaces:

| verb | asks | reads |
| --- | --- | --- |
| `doctor` | can this machine run the battery, and what is installed here? | the toolchain, and the manifest's identity fields |
| `diff` | which of the files `init` wrote have I changed? | the manifest's `files` hashes against the tree |
| `update --dry-run` | what would *this payload* change if I upgraded? | the payload against the tree |

**Why not fold it into `doctor`.** `doctor`'s exit status has exactly one owner
— the toolchain contract — and `init` gates its own precondition on it. That
ownership already forced one carve-out: installer/README.md §doctor rules the
artifact finding to report *without* setting the status, because reddening there
would block the `init` re-run that is the finding's own remedy. Drift would
force the same carve-out a second time and on worse ground. An adopter editing a
vendored file is **sanctioned** — the whole `claim()` contract exists to protect
it — so a folded-in drift report is a permanent expected finding on a verdict
surface, which is the surface teaching a reader to skim its last line. §doctor's
own argument against printing `DIGEST MISMATCH` above and `clean` below is the
argument against this.

And drift *wants* a status of its own: `0` the tree is exactly what `init`
wrote, `1` it has diverged. That is gatable in CI — "is our vendored tree
pristine?" — and it cannot live on `doctor` without colliding with the toolchain
contract that already owns the number.

Second ground, independent of the first: `doctor` runs as `init`'s precondition,
before every install. Per-file hashing there makes every `init` pay for a
computation `claim()` redoes moments later, and print the answer twice.

**What the seam sentence becomes.** installer/README.md §doctor closes with
"`doctor` writes nothing, so it has no `--dry-run`. Every verb that does write
has one." Read as *doctor is the non-writing one* it argues `diff` into `doctor`;
read as what it says, the classifier is **writes / does not write**, and `diff`
joins `doctor` on the non-writing side with no `--dry-run` of its own. The
sentence generalizes rather than yielding, and D9 relocates it out of §doctor so
it stops reading as a fact about one verb.

**Refused: dropping `diff` in favour of the two `--dry-run`s.** They answer the
payload question, not the ownership question, and an adopter deciding whether to
uninstall needs the second without running a mutating verb at all.

## What changes

### D1 — `installer/lib/update.sh` — the upgrade verb {mechanical}

Upgrade is already supported and is spelled `npx checkwright@<newer> init`; what
is missing is that nobody guesses `init`. So `update` is `init` with **one added
precondition** and argv forwarded verbatim:

- `-h`/`--help` is intercepted first and prints `update`'s own usage (it must
  answer outside a repository).
- Otherwise `checkwright.lock` must exist at the repository root and carry a
  known schema. Absent, it refuses (exit 2) naming `init` as the verb that makes
  an install. This is the whole behavioral difference: a verb named `update`
  must not perform a first install.
- Then `exec bash "$INSTALLER/lib/init.sh" "$@"`. Every `init` flag stays valid,
  including `--profile` (moving up the containment chain is an upgrade motion),
  `--force`, `--no-commit`, and `--dry-run` — so the mutating-verb `--dry-run`
  obligation is discharged by delegation rather than by a second implementation
  that could drift from it.

Its discoverability is **derived, not maintained**: the dispatcher's verb roster
is the `lib/*.sh` glob (installer/README.md §Layout), so `update` appears in
`checkwright --help` because its implementation is present.

*Residue, recorded rather than papered over:* a delegated run prints `INIT:`
lines and `checkwright init:` diagnostics. That is left as-is. Threading a
display name through `init` would put a second source for the running verb's
identity into the one file that must stay the single writer, and the shared
voice is the truth about the mechanism — one operation under two names, not two
mechanisms with two failure modes.

### D2 — `installer/lib/diff.sh` — the drift verb {mechanical}

Reads the manifest's `files` map and classifies every entry against the tree,
using `lock_hash` so the comparison is the same one `claim()` makes:

| class | condition | reported |
| --- | --- | --- |
| `same` | tree hash equals the recorded hash | counted only |
| `changed` | tree hash differs | one line per path |
| `missing` | recorded but absent from the tree | one line per path, named apart |

`missing` is a class of its own rather than a flavour of `changed` because a
deletion and an edit have different remedies, and because `init`'s roster exit
rule means the next `init` will silently drop a missing path and rewrite it —
which an adopter should be told before it happens rather than after.

Exit status: `0` when every entry is `same`, `1` when any entry is `changed` or
`missing`. Preconditions: inside a git work tree, and a manifest with a known
schema (else exit 2, naming `init`). No `--dry-run` — it writes nothing.

Its subject is exactly the roster. A file the adopter *added* inside a vendored
kit directory is not on the roster and is not `diff`'s question; it is
`uninstall`'s, and D3 reports it there.

### D3 — `installer/lib/uninstall.sh` — the removal verb {design-bearing}

The one real capability gap the entry names, and — per BRIEF.local.md — the verb
that turns "an install an evaluator can reverse" from a claim into a property.

**Preconditions**, all refusing rather than warning, all checked before anything
is removed: inside a git work tree; a manifest present with a known schema (else
exit 2, naming `init`); and a clean worktree, for `init`'s own reason — it makes
one commit, and a dirty tree would fold the adopter's work into it. `--no-commit`
is the same valve on the same terms.

**The removal rule is `claim()` seen from the other side.** For each `files`
entry: hash the file; remove it when the hash still matches what `init` recorded,
and **keep and report** it when it differs. That is what makes the entry's
acceptance shape true — *removes only manifest-recorded files, never a file the
adopter wrote* — with no new data, because `install-claim-contract` already
records each path at the hash `init` last wrote there. A recorded path already
absent from the tree is a no-op, not an error.

`--force` removes what would otherwise be kept, and means in this third place
exactly what installer/README.md §init says it means in the other two:
**overwrite what `init` would otherwise protect**.

**The gate binary needs no special case.** `init` records it as an ordinary
`files` row *and* under the separate `artifact` key; the roster walk removes it,
and the `artifact` key is identity rather than ownership.

**The manifest is not a `files` row and is disposed of explicitly.** `init`
appends `checkwright.lock` to its written set only *after* emitting the manifest,
so the file never records itself. `uninstall` therefore handles it as its last
act — and the rule is not "delete it":

**The residual manifest.** installer/README.md §The manifest rules that "a path
leaves the roster when the file leaves the tree, and at no other moment", and
that rule decides this. A file `uninstall` kept is a file `init` wrote that is
still on disk, so its ownership has not ended and the roster must retain it.
Deleting the manifest wholesale would disown exactly the paths the hash rule just
protected, and the next `init` would find them unrecorded, read that as *never
installed*, and write straight through the adopter's edits — the precise defect
§The manifest's exit rule exists to prevent.

So: when every entry was removed, `checkwright.lock` is deleted. When any entry
was kept, it is **rewritten over the survivors** and carries `schema` and `files`
only. `version`, `commit`, `profile`, `kits` and `artifact` describe an install
that no longer exists, and a manifest asserting them would be false.

That the narrowed shape is safe is a property of the existing readers, verified
rather than assumed:

- `init`'s downgrade refusal is guarded by `[[ -n "$PRIOR_VERSION" && … ]]`, so
  an absent `version` skips it — correct, since there is nothing to roll back.
- `init` re-reads `profile` only to default it; absent, it falls through to
  `starter`, which is what a virgin tree gets and what the residue is.
- `doctor`'s installed-block reads `commit` for display only
  (`lock_field "$LOCK" commit`), which prints blank on absence — the same
  tolerance as `version` and `profile`.
- `lock_own_file` narrows its fixture-exclusion predicate with `kits`; absent,
  the predicate excludes nothing rather than erroring, so it degrades to "no
  exclusion," not to a crash. Its only readers are `doctor`'s artifact check
  and its omitted-gates block. The premise that makes both moot on a residual
  tree is **not** "a residual tree has no `gates.list`" — a kept `gates.list`
  is in fact the single most plausible residue cause, an adopter-edited seam
  file being exactly what a residual manifest exists to protect. It is instead
  that D7 routes both call sites behind the same residue check that gates the
  identity-field prints, so neither runs at all on a residual tree regardless
  of which files survived.

The wire key stays `checkwright-lock v1`, on §The manifest's own precedent: this
makes previously-always-present fields optional within a shape a reader already
tolerates, which is a change of meaning rather than of shape. *Honest residue:* a
reader built before this meets a residual manifest and prints blank identity
fields — uninformative, never wrong-acting. D7 is what stops this build printing
them.

**The agent file is the one non-whole-file removal.** installer/README.md §The
manifest already names it as the one entry that is a span rather than a file:
`init` creates `CLAUDE.md` only when absent and thereafter authors only the
marker-bounded doctrine block inside it. Two branches:

- Hash matches — the adopter never touched it, `init` created it, remove the
  whole file with every other unmodified entry.
- Hash differs — keep the file and **trim the doctrine span out of it** (D5),
  because the block is prose the adopter did not write, in the one file whose
  purpose is to steer agent sessions, pointing at a `doctrine-kit/DOCTRINE.md`
  this verb just removed. Leaving it inert is not neutral on that surface.

**Scoped to what `init` recorded, and only that.** The trim touches the
doctrine-kit span and nothing else in the file. A consumer whose agent file
also carries a *different* tool's marker-bounded span — lifecycle-kit's own
registration block (`<!-- lifecycle-kit:begin -->` … `<!-- lifecycle-kit:end
-->`) is the concrete instance; this repo's own `CLAUDE.md` carries one —
keeps that span untouched even on the hash-differs branch, because
`checkwright init` never writes it: `installer/lib/common/recipe.sh`'s
`recipe_needs_agent_file()` names only `context-kit` and `doctrine-kit`, and
the `lifecycle-kit` case of `recipe_seed()` never calls
`install-lifecycle.sh`. A block `init` never wrote is never a `files[]` entry,
so it is never manifest-recorded — and trimming it would not close a gap, it
would be the exact violation the acceptance shape above rules out, a removal
reaching outside what the manifest records to touch content this verb was
never told it owns. That is a scoping fact about this delta, not an
incompleteness in it: nothing here claims the agent file carries only one
tool's span, only that `uninstall` acts on the one it is responsible for.

The trim runs through the *payload's* copy of `install-doctrine.sh`, exactly as
`init`'s injection does, so it is independent of removal ordering and works when
the kit is already gone. It runs only when the recorded `kits` include
`doctrine-kit`, and never under `--dry-run`.

**Directory pruning.** After the file removals, remove bottom-up any directory
that is now empty. A directory still holding anything is left alone, whether that
is an adopter's own file or one they edited — `uninstall` removes files it owns,
never directories it merely emptied around.

**The hook opt-in is reported, not rewritten.** `install-hooks.sh` sets
`core.hooksPath` per clone, and the adopter opted in by hand. When that config
points inside the removed gates directory, `uninstall` prints the `git config
--unset core.hooksPath` line and does not run it: git config is outside the
ownership roster, and a `core.hooksPath` naming a directory that no longer exists
is inert rather than breaking, so there is nothing to justify writing outside the
contract.

**`--dry-run`** prints the plan — what would be removed, what would be kept and
why, the residual manifest if any, and any adopter-added file inside a vendored
directory that will be left behind — writes nothing, exits 0.

**The commit.** One commit, `chore: remove Checkwright kits (<profile>, v<ver>)`,
staging the deletions and the manifest disposition. Files kept for the adopter
are never staged, on the same reasoning that keeps `init`'s written set and its
recorded roster apart. A run with nothing to remove says so and exits 0.

### D4 — `gate-sdk/lib/inject.sh` — `remove_marker_block` {mechanical}

The marker mechanics have one copy (doctrine-kit/SPEC.md §install-doctrine:
`inject_marker_block` to write, `read_marker_block` to read, shared with
lifecycle-kit's injector). A remover is the missing third: a kit that can inject
a marker block into a consumer's file and never remove it is a one-way door, and
any uninstall story needs the reverse.

`remove_marker_block <file>` deletes the marker pair and everything between it,
leaves the rest byte-identical, is a no-op exiting 0 when no begin marker is
present, and **refuses (exit 2) on a begin without its end** — the same
malformed-target rule `install-doctrine.sh` already takes, so the two helpers
cannot disagree about what a malformed block is.

### D5 — `doctrine-kit/bin/install-doctrine.sh --remove` {mechanical}

A `--remove` mode over D4: given the agent file, remove the doctrine block and
exit 0, reporting what it did on the same channel the insert path reports on. It
harvests no trims and emits no digest — a removal has nothing to carry forward.
A missing agent file stays exit 2, unchanged.

The markers stay owned by doctrine-kit and gate-sdk. `uninstall` calls this
rather than matching marker literals itself, so the removal path adds no copy of
a string that already has one writer.

### D6 — `installer/lib/common/lock.sh` — one writer of the wire shape {design-bearing}

`lock.sh` is declared the schema owner "so the verb that writes the manifest and
the verbs that read it cannot drift apart". A **second writer** arriving is the
moment that promise has to be kept rather than restated.

`lock.sh` gains `lock_emit`: it reads `<path><TAB><hash>` lines on stdin, takes
the optional top-level identity fields as `key=value` arguments and the nested
artifact as `--artifact <target> <digest>`, and emits **a JSON object whose
keys are sorted at every nesting level** — the top-level object, and `files`
and `artifact` within it alike — with no field ever emitted out of key order.
That is the contract this delta owns; `jq -S .`, which today's `manifest()`
already pipes its output through, is the implementation that happens to
satisfy it, not the definition of it, so a future reimplementation is free to
drop `jq` entirely as long as the sort property holds. The `artifact` key
follows the existing present-only-when-supplied rule: passing `--artifact`
emits the key, omitting the flag omits the key — never a `null` or an empty
placeholder in its stead. `init`'s `manifest()` becomes a call to it — its
`files_hash` carry-forward logic stays in `init.sh`, since which hash an entry
carries is `init`'s rule, not the schema's — and `uninstall` calls it with the
survivor lines, no identity-field arguments, and no `--artifact` flag, so the
residual object carries `schema` and `files` only, matching §The residual
manifest exactly.

This is the delta most able to break every install if it is got wrong, which is
why it is a delta of its own rather than a footnote to D3. It is also, by the
same token, the one enforcement-first owes a gate for rather than a stated
property alone — D8 gains the assertion that catches a divergence in either
half of this contract.

### D7 — `installer/lib/doctor.sh` — residue and the pointer {mechanical}

A manifest carrying `files` and **no `version`** is a residue rather than an
install — the discriminator is the absent `version`, which is the field an
install always has and a residue never does. This is a **guard on the existing
block, not an addition beside it**: today `doctor` unconditionally prints
`installed` followed by the version/commit/profile/kits identity lines, then
the artifact check, then the omitted-gates block
(`installer/lib/doctor.sh:80-124`). A residual tree takes none of that path.
`doctor` instead says so and names the count: no install here, N file(s)
remain that a previous install wrote and you have since edited, they are
yours, and a future `init` will still protect them. The identity lines, the
artifact check, and the omitted-gates block are per-install readings with
nothing to compute once there is no install — printing them beside the residue
message would be exactly the mixed-verdict shape §The verb taxonomy this
settles already rules out on `doctor`'s own surface (the reason `diff` is a
fourth verb rather than folded in), so this amendment does not reintroduce it
one section later. A kept `gates.list` is reported the same way as any other
survivor — by `diff` and by `uninstall --dry-run`'s plan — never by the
omitted-gates block, which does not run on a residue tree regardless of which
files it kept.

One further, unconditional addition: a line naming `diff` as where per-file
divergence is answered, on both the install and the residue path. Without it
`DOCTOR: clean` invites reading as a claim about the tree's contents, which it
has never been and must not become now that a verb makes that claim.

Neither addition touches the exit status.

### D8 — `installer/consumer-smoke/run-smoke.sh` — the reversal arm {design-bearing}

`assert_install` stays the single encoding of the install post-conditions. A new
`assert_reversal <profile> <consumer> <seed-tree>` is called after it from both
call sites — the per-profile loop and the download arm — so `diff` and
`uninstall` are proved Node-free by the masked arm at no extra pack cost. The
seed tree is `git rev-parse 'HEAD^{tree}'` taken on the consumer `consumer()`
just built, before `init` runs.

In order: `diff` must exit 0 and report no drift on the freshly installed tree;
`uninstall --dry-run` must leave the tree object and `git status` unchanged while
naming a non-zero removal count; then `uninstall`; then the consumer's tree
object must equal the seed tree, the worktree must be clean, and no
`checkwright.lock` may remain.

**The tree-object equality is the load-bearing assertion, and it proves more than
uninstall.** No existing arm asserts that the manifest covers *everything* `init`
wrote — the per-profile loop checks that every recorded entry agrees with the
tree, which is the other direction. A file `init` wrote and failed to record
survives uninstall and breaks this equality, so the arm closes that hole as a
side effect. It is also the assertion form of the go-to-market claim: an install
an evaluator can reverse.

The equality holds for a real reason rather than by luck: the surfaces `init`
seeds and then leaves alone — the queue file, the agent file, the evidence
manifests, the workflow state — are recorded at `init`'s hashes, and this
consumer never edits them, so the ordinary hash rule removes them. The same rule
keeps them on a tree where the adopter *has* grown them. No special case either
way, which is why the arm needs none.

**The protection branch chains onto the seam arm's consumer**, not this one. That
arm already has the adopter edit and commit two vendored files, which is exactly
the case `assert_reversal` cannot host (an edit is what breaks tree equality).
After its own assertions it runs `diff`, which must exit **1** and name both
files; then `uninstall`, which must keep both, report both, remove everything
else, and leave a `checkwright.lock` carrying `schema` and a `files` map of
exactly those two paths **at `init`'s hashes, not the adopter's** — the same
apart-naming the upgrade arm already uses, and for the same reason: an entry
dropped reads as never-installed and an entry at the adopter's hash reads as
unchanged, and both would let the next `init` write through them.

**The residual object is asserted for shape, not just for field presence** —
D6's two-writer contract is exactly what a `lock_field`-style read cannot
catch, since a missing key and a present-but-null key both read back as empty
string. So this arm asserts the shape directly on the captured
`checkwright.lock` text: `jq -e 'has("artifact") | not'` holds (the omitted
flag left the key absent, not null), and re-piping the captured object through
`jq -S .` reproduces it byte-for-byte (the recursive-sort property held, not
just a top-level or accidental one). Both assertions run on this arm's
residual object, which is the one call site — `uninstall`'s — that ever emits
`lock_emit`'s no-identity, no-artifact shape; the per-profile loop's fresh
install already exercises the other call site through `assert_install`'s
existing field-by-field checks.

*Cost, stated because arms are not free:* no new pack and no new scratch consumer
— both arms extend consumers that already exist.

### D9 — `installer/README.md` {design-bearing}

New `## update`, `## diff` and `## uninstall` sections; §doctor's closing
`--dry-run` sentence relocated to where the verb taxonomy is stated so it stops
reading as a fact about one verb; §The manifest extended with the residual shape,
its field set, and the v1 argument; §The consumer smoke extended with the
reversal arm and the seam arm's protection chain; §Layout unchanged, because the
verb roster is derived.

### D10 — `doctrine-kit/SPEC.md` and `gate-sdk/SPEC.md` {design-bearing}

§install-doctrine gains the removal mode and its no-op/refuse rules; gate-sdk's
inject-helper section gains `remove_marker_block` beside the two it names. Each
lands in the section that already owns the surrounding mechanism, and the
installer's own prose cites rather than restates them.

### D11 — `docs/install.md` {design-bearing}

The public install page documents `init` and `doctor` and stops. It gains the
three verbs at the tier that page holds — what each is for and its exit status,
citing installer/README.md for mechanism — with `uninstall`'s reversibility
stated plainly, since that is the property the page's audience is evaluating.

### D12 — generated projections {mechanical}

Regenerate every projection this unit stales — the docs mirror for the two kit
SPECs, and the footprint and value rollups for the new files — per
docs/site-architecture.md §Generated projections, each on its own printed regen
command. No gate is added, so the pre-commit hook and the graph artifact do not
move.

### Refused: a gate asserting the `--dry-run` rule {no delta}

Recorded because enforcement-first invites it. The rule "every verb that writes
has a `--dry-run`" goes from governing two verbs to governing five here. A
commit-time gate over `installer/lib/*.sh` could only test a syntactic proxy —
does the file parse `--dry-run` — and would need each non-writing verb to
*declare* itself non-writing, an author opt-in derivation-first forbids and a
gate can be satisfied by a flag that does nothing. The behavioral property is
asserted directly instead, in D8: each mutating verb's `--dry-run` must leave the
tree object unchanged. installer/README.md §Why the payload has no gate of its
own already rules this tier split for the installer; this is that ruling applied,
not a new exemption.

## Producers and consumers

No new message, stream or wire field is introduced — the schema is untouched.
The new state is the **residual manifest shape** and the three verbs' exit
statuses, and each is traced below.

**The residual manifest** (`checkwright.lock` carrying `schema` + `files`, no
identity fields)

- *Producer:* `uninstall` (D3), on the branch where at least one recorded entry
  was kept. Reachable with no configuration: the branch is taken whenever an
  adopter has edited a vendored file, which is the sanctioned case, and D8's
  seam-arm chain drives it on every validate run.
- *Consumers, each named with the transition that reads it:*
  - `init`'s `claim()`, at the next install's per-path write decision — reads
    `files` and refuses the write. This is the reason the shape exists.
  - `init`'s downgrade test, at manifest load — reads the absent `version` and
    skips, verified against the existing `[[ -n "$PRIOR_VERSION" ]]` guard.
  - `init`'s profile default, at manifest load — reads the absent `profile` and
    falls through to `starter`.
  - `doctor` (D7), at its installed-block render — reads the absent `version` as
    the residue discriminator and reports the count.
  - `diff` (D2), at classification — reads `files` and reports the survivors as
    `changed`, which is true of them.
  - the consumer smoke (D8), at the seam arm's chain — asserts the field set and
    the recorded hashes.
- *Every field has a named reader:* `schema` is read by every verb's first act
  (the unknown-key refusal, unchanged); `files` by all six readers above. No
  field is added, and none is retained whose only reader would be a future
  phase — the whole point of narrowing to two keys is that `version`, `commit`,
  `profile`, `kits` and `artifact` have no true value to report here.

**`diff`'s exit status** — *Producer:* `diff.sh`'s classification (D2).
*Consumers:* an adopter or CI step gating on a pristine vendored tree, and the
consumer smoke, which asserts `0` on a fresh install and `1` on the seam arm's
edited consumer. It is deliberately read by **no** other verb: `init` gates on
`doctor` and must keep doing so, since drift is not a reason to refuse an
install.

**`uninstall`'s kept-file report** — *Producer:* the roster walk's keep branch.
*Consumers:* the adopter, at the terminal; and the consumer smoke, which asserts
both paths are named on the seam arm's chain. Its machine-readable counterpart is
the residual manifest above, which is why the report is prose and carries no
second encoding of the set.

**`remove_marker_block`** (D4) — *Producer:* `install-doctrine.sh --remove` (D5).
*Consumer:* `uninstall`'s agent-file branch (D3), calling the payload's copy at
the point where the file's hash was found to differ. The trim is unreachable when
`doctrine-kit` is not in the recorded `kits`, which is correct — nothing injected
the block on that install.

**`update`'s manifest precondition** — *Producer:* `update.sh`'s own check.
*Consumer:* the adopter, as a refusal naming `init`. It introduces no state: the
manifest it reads is the one `init` already writes.

## Existing sections updated

Each names the delta that owns it, so no update target reaches build unclaimed.

- **installer/README.md §doctor** — the closing `--dry-run` sentence relocates
  and generalizes to the writes/does-not-write classifier; the installed block
  gains the residue reading and the `diff` pointer. *Owned by D9, following D7.*
- **installer/README.md §The manifest** — the field table's `files` row gains
  `uninstall` and `diff` as readers; the exit rule gains the sentence that a file
  `uninstall` kept has not left the tree and so has not left the roster; the
  residual shape, its two-key field set, and the wire-key argument land beside
  the existing v1 reasoning. *Owned by D9, following D3.*
- **installer/README.md §init** — `--force`'s "means the same thing in both
  places" becomes all three, naming `uninstall`'s meaning. *Owned by D9.*
- **installer/README.md §The consumer smoke** — the per-profile post-conditions
  gain the reversal arm; the seam arm gains its protection chain. *Owned by D9,
  following D8.*
- **installer/README.md §Layout** — unchanged, and stated so no batch edits it:
  the verb roster is the `lib/*.sh` glob, so three new verbs are already
  described. *Owned by D9.*
- **doctrine-kit/SPEC.md §install-doctrine** — the removal mode, its no-op on an
  absent block, and its refusal on a malformed one. *Owned by D10, following D5.*
- **gate-sdk/SPEC.md, the inject-helper section** — `remove_marker_block` beside
  `inject_marker_block` and `read_marker_block`. *Owned by D10, following D4.*
- **docs/install.md** — the three verbs at the public tier. *Owned by D11.*
- **CLAUDE.md §Housekeeping** — unchanged. `installer/` is still not a kit and
  gains no `checks/` or `smoke/`; stated because three new files under it invite
  the question. *No delta.*

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
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
