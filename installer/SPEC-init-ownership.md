# SPEC amendment: init-ownership

The delta for `install-claim-contract`: what `checkwright init` owns, when it
stops owning it, and the two write paths that presently bypass the ownership
check entirely. Canonical spec: [installer/README.md](README.md) — §init, §What
init seeds, §The gate binary, §The manifest, §The consumer smoke.

Three queue entries land against this one amendment, because they are one
question: `init-claim-stickiness` (the ruled defect),
`installer-payload-relinquish-stickiness` (its other direction, merged here by
the operator's envelope reversal of 2026-08-05), and
`installer-config-seam-silent-revert` (promoted debt, same governed claim, same
rewritten sentence).

## What changes

### The rule

`checkwright.lock`'s `files[]` is `init`'s ownership roster. §The manifest
presently states its exit condition as **"A path leaves the roster only when
`init` stops shipping it."** That sentence is the defect rather than a
description of one: it reads the roster as a projection of the payload's current
shipping set. It is not. It is the record of what `init` *created in this tree*,
and only the tree can end it.

**The replacement rule.** `init` owns a path because it wrote the file there.
That ownership ends when the file leaves the tree, and at no other moment. A
payload that stops shipping a path does not end it — the file is still on disk,
it may carry the adopter's edits, and disowning it is precisely what lets a later
payload re-adding the same path write straight through them.

**There is no third lock state, and the dilemma that predicted one dissolves.**
`installer-payload-relinquish-stickiness` framed the fix as needing a state the
lock does not have — *once ours, now relinquished* — with a tombstone growing the
lock without bound and inference needing history the lock does not keep. Both
branches assume the roster tracks the shipping set. Under the rule above a
relinquished path is not a state at all: it is an ordinary `files[]` entry, at
the hash `init` last wrote there, protected by the same `claim()` that protects
every other entry. The unbounded-growth objection does not transfer either,
because the existence test is already the reaper — `files[]` is bounded by *files
`init` created that still exist*, which is the ownership set itself and cannot
outgrow the tree.

**The wire key stays `checkwright-lock v1`.** This is a change of meaning, not of
shape — no new field, no new key, no removed key — and it passes the same test
§The manifest already applies to the carry-forward meaning: a reader built before
it meets a retained relinquished entry, and does nothing with it, because the
payload it carries does not ship that path and so never asks whether it may write
there. The old behavior on the new data is the behavior the new meaning wants.

### Delta 1 — the roster's exit condition *(design-bearing)*

`installer/lib/init.sh`: delete `under_kit()` (`:299-303`) and its call
(`:307`), leaving the carry-forward loop's membership test (`:308`) and existence
test (`:309`) as the whole rule. Rewrite the `# spec:` comment at `:298`, which
currently narrates the guarded-seed special case, to state the rule above.

**Why this is a strict-subset removal rather than a widening.** Every path the
payload still ships reaches `copy_in` (`:189-193`) and therefore `claim`
(`:175-187`), and *both* outcomes `record` it — a write records the path
(`:192`), a refusal records it with the hash carried forward (`:185`). So every
live path is already in `WRITTEN` and is skipped by the membership test before
the existence test is reached. The only prior-manifest entries that reach `:309`
are the ones no `copy_in` visited this run: exactly the relinquished set.
`under_kit` guards nothing live. It is a pure carve-out that drops that set.

**The rule is already half-implemented, which is the evidence that it is the
right one.** `under_kit` tests membership in the *current* `KITS`, so a path
under a kit that has left the selected profile is already not `under_kit`,
already falls through to `:309-310`, and is already retained at its recorded
hash. "A path leaves the roster only when `init` stops shipping it" has never
been the implemented rule. Deleting the carve-out makes one behavior uniform; it
does not introduce a second.

**The residue, bounded out deliberately.** `init` still never deletes a file. A
relinquished path stays on disk and stays protected, and nothing prunes it. That
is the correct half to keep — `init.sh:298` already reasons that disowning a
file `init` created would make an uninstall leave it behind — but it means a
long-lived tree accumulates files no kit ships any more. Removing them is a
*verb*, not a roster rule, and belongs with `installer-lifecycle-verbs`'
uninstall rather than here; filed rather than folded in, because a vendoring
installer that starts deleting from the adopter's tree is an envelope change of
its own.

### Delta 2 — the config seam claims before it writes *(design-bearing)*

`recipe_config_seam` (`installer/lib/common/recipe.sh:5-15`) copies every
`templates/*-config.sh` into the gates directory unconditionally and then prints
the destination; `init.sh:250` calls it, and `init.sh:231` claims each printed
path *afterwards*. `claim` hashes the tree at `:180` — after the overwrite has
landed — so the copy destroys the evidence the refusal is computed from. **Eight
kits ship such a template**, and the consumer config seam is the one file class
whose whole purpose is to be edited by the adopter.

Two failure modes, and the second is worse than silence:

- **At the same version.** `want` (recorded) equals `cur` (the template just
  re-copied), so `claim` returns 0, the adopter's edit is gone, and nothing is
  printed. No upgrade and no `--force` are needed.
- **Across an upgrade whose template changed.** `want` differs from `cur`, so the
  path joins `CHANGED` and is carried forward — `init` **reports the file as left
  alone after it has already overwritten it**, and records a hash that disagrees
  with what is on disk.

**The fix: the recipe plans, `copy_in` writes.** `recipe_config_seam` becomes
`recipe_config_seam_plan`, printing one `<src>\t<dest>` line per template and
writing nothing. `init.sh` iterates that plan in the **parent** shell and calls
`copy_in "$src" "$dest"`, which is already claim-then-write. The parent-shell
requirement is load-bearing and is the non-obvious part of the delta: the present
call sits inside a `< <( … )` subshell, where every `record`/`CHANGED` append
would be discarded — the plan may be produced in a subshell, the writes may not.

**A deleted restatement, which is the point rather than a bonus.**
`init.sh:235-240` presently re-spells recipe.sh's `templates/*-config.sh` glob so
`--dry-run` can list what the real path would copy. One enumerator now serves
both, so that copy goes and the dry-run plan can no longer drift from the run it
predicts.

### Delta 3 — `msg-patterns.list` claims before it writes *(design-bearing)*

`recipe_seed`'s gate-sdk arm (`installer/lib/common/recipe.sh:64-67`) copies
`templates/msg-patterns.list` into the gates directory **unconditionally** — the
same defect as Delta 2, on a surface an adopter is equally expected to edit
(their commit-message patterns), and one that ships in the `starter` profile, so
it reaches the smallest install. Every other arm of `recipe_seed` writes only
when the file is absent (`:52`, `:70`, `:75`, `:82`) and is safe; this one is the
exception.

Same fix, same shape: the arm stops copying and instead prints its
`<src>\t<dest>` pair for `copy_in` to claim and write in the parent shell.
`recipe_seed`'s contract changes with it — it becomes *seed what is absent, and
plan what must be claimed* — so its `# spec:` comment (`:48`) and the module
header's "prints one repo-relative path per file it wrote" (`:2`) are rewritten
to the two-channel contract rather than left describing the one it had.

The `:48` comment is a **standing accuracy defect independent of this fix**, and
the rewrite discharges it rather than merely adjusting it: it presently asserts
"a seam surface is written only when it is absent … which is what keeps a re-run
non-destructive on a tree that has grown since" directly above the one arm of
`recipe_seed` that has no such guard. Every sibling arm (`:52`, `:70`, `:75`,
`:82`) does. The comment overclaims uniform behavior the function has never had.

### Delta 3b — the doctrine block *(escalated, not settled here)*

A **third** instance of the same class was found while verifying the two above,
and it is not folded in because closing it properly leaves this amendment's
envelope. `recipe_seed`'s doctrine-kit arm (`recipe.sh:87-90`) invokes
`doctrine-kit/bin/install-doctrine.sh` unconditionally on every run; that script
(`:43`) hands the packaged digest to `gate-sdk/lib/inject.sh`'s
`inject_marker_block`, which always rewrites the whole span between
`<!-- doctrine-kit:begin -->` and `<!-- doctrine-kit:end -->` (`inject.sh:19-32`).

What makes this sharper than Deltas 2 and 3 rather than merely another instance:
the content inside those markers carries a customization doctrine-kit's own SPEC
**grants by contract**. A `<!-- doctrine-digest-trim: <rule name> — <reason> -->`
line inside the digest section is the consumer's sanctioned way to reject a rule
(doctrine-kit/SPEC.md §check-doctrine-registration, assertion B), and
`install-doctrine.sh`'s `block()` emits the full untrimmed digest with no
trim-awareness at all. So `init` silently revokes a right another kit's contract
grants — and silently in the strict sense: if the trim was the adopter's only
edit to the agent file, the post-rewrite hash equals the recorded one and `claim`
emits no `CHANGED` line.

Two fixes with materially different envelopes, which is why this is escalated
rather than decided:

- **Installer-side claim.** `init` claims `AGENT_FILE` *before* calling the
  doctrine arm and skips the call on a refusal. Stays inside this envelope and
  touches one component — but the agent file is the file an adopter is
  *certain* to edit, so the doctrine block would stop updating after their first
  edit, and a consumer whose newly vendored `DOCTRINE.md` gained a rule would
  then red on `check-doctrine-registration` with no `init` path to fix it.
- **Trim-preserving injection.** The injector reads the existing block, captures
  declared trims, and re-emits them — the round-trip `gate-sdk/bin/gen-pre-commit.sh`
  already performs for `gen=manual` regions (`:44-58`, `:88-97`), so the pattern
  is precedented in this tree rather than invented. It is the right design and it
  changes **doctrine-kit's** `§install-doctrine` contract and possibly
  `gate-sdk/lib/inject.sh`'s, making this a cross-component amendment.

Until it is ruled, no delta here touches the doctrine arm, and the claim rewrites
in Deltas 4 and 5 are worded so they do not assert the guarantee for it.

### Delta 4 — the gate binary's replacement premise *(design-bearing)*

§The gate binary (`installer/README.md:219-222`) argues that gate-sdk ships no
config template *because* "the template seam copies unconditionally and would
overwrite an adopter's own overrides on every re-run". Delta 2 kills that
premise, and a conclusion left standing on a dead premise is the defect
`spec-over-precedent` exists to catch.

The conclusion survives on its own ground, and the paragraph is rewritten to it:
**the seam file's content is resolved at install time, not shipped.** Its one
line is `GATE_SDK_NATIVE_BIN=<the path the artifact was actually placed at>`,
which does not exist until selection has run, so a static template could only be
copied and then immediately rewritten — the same file written twice, with the
copy contributing nothing. That reason is independent of how the seam copies and
does not expire with this fix.

### Delta 5 — the downstream prose *(mechanical)*

Every surface that restates or reasons from the claims above is rewritten in the
same pass, so no reader is left holding the old rule. The roster is §Existing
sections updated below; each site's owner is named there, with the sites a
tree-wide sweep cleared named too, so build re-opens none of them. This delta is
mechanical because the replacement text is determined by Deltas 1-4 — the
judgment was spent there, and each site either restates the rule (rewrite to the
new one) or points at it (leave the pointer, verify the target).

One site inside it is **escalated rather than mechanical** and is excluded from
the delta until ruled: `docs/posts/2026-07-26-checkwright-v0-16-0.md:99-105`
tells an adopter that "files you have modified are preserved and reported rather
than overwritten" — unqualified, and false for two file classes on the day it
published. Dated posts are governed as immutable published artifacts
(`CANON_KIT_TEMPORAL_EXEMPT_PATHS`), and that same post already carries an
erratum block for an unrelated defect, so the precedent for correcting one
exists. Whether a published claim that was wrong when published owes an erratum
is a user-facing call no governing spec settles.

### Delta 6 — the smoke's relinquish arm *(design-bearing)*

Nothing in any automated suite can presently catch Delta 1's defect, and the
reason is test infrastructure rather than a missing assertion:
`scripts/pack-installer.sh` assembles every "version" from the same worktree
(`cp -R` at `:79-84`; only `package.json`'s version is rewritten, at `:123-128`),
so the two upgrade hops carry byte-identical payloads and no path ever leaves a
kit's shipped set. The existing three-version arm
(`installer/consumer-smoke/run-smoke.sh:211-292`) is structurally incapable of
reaching the relinquish.

**The fix needs no publishing knob.** The upgrade arm already extracts each
tarball to `$UP/package/` and `$UP2/package/` and invokes
`bash <dir>/package/bin/checkwright.sh` (`:229`, `:248`, `:281`, `:283`), so the
relinquish is performed by deleting the path from the **extracted package's own
`payload/`** before that hop runs. A `--drop` flag on `pack-installer.sh` was
weighed and refused: it would put a payload-mutilating switch on the publishing
path to serve a test, where the test can mutate its own extracted copy and the
publisher keeps no way to ship a payload with a hole in it.

The arm carries a third adopter-visible path `R` alongside the existing `EDITED`,
chosen against a stated criterion rather than pinned by taste: **a `starter`-kit
payload file that `init` records in `files[]` and that no `init` step and neither
generated projection reads** (both generators run against the consumer's own
vendored `gate-sdk`, never the payload, so a payload deletion cannot break them).
`gate-sdk/templates/check-skeleton.sh` satisfies it; build substitutes another
file meeting the criterion if it does not.

- **Hop A** (`$VERSION`): `init --profile starter`; the adopter edits and commits
  both `EDITED` and `R`; record `init`'s hash for `R` from the manifest.
- **Hop B** (`$UP_VERSION`, payload with `R` deleted): `init`. Assert `R` is
  byte-identical on disk, that `files[]` **still carries `R` at `init`'s hash**
  (today it is dropped — this is the assertion that reds without Delta 1), and
  that the worktree is clean.
- **Hop C** (`$UP2_VERSION`, payload with `R` present again): `init`. Assert `R`
  is still byte-identical, is **named in the output as changed**, and is still on
  the roster at `init`'s hash. Without Delta 1 this hop overwrites it silently,
  which is the whole defect reproduced end to end.

### Delta 7 — the smoke's seam arm *(design-bearing)*

Deltas 2 and 3 need an arm of their own, and today they have **no coverage at
all**: the existing upgrade arm's subject is `EDITED="gate-sdk/README.md"`
(`run-smoke.sh:240`), an ordinary vendored file on the working `copy_in`/`claim`
path, and no assertion anywhere in the smoke names a config template or
`msg-patterns.list`. So those arms remain true and need no rewrite — they simply
exercise a different file class than the broken one, which is exactly why the
defect survived them.

The arm must be its own consumer, because the per-profile loop asserts the
manifest agrees with the tree **file by file** against a freshly initialized
consumer (`installer/README.md:371-373`) — an adopter edit inside that loop would
break the assertion it is there to make. A separate scratch consumer, reusing the
already-installed package with no extra pack:

- `init --profile delegation` — `starter` is gate-sdk alone and gate-sdk ships no
  config template, so the templates are only reachable from `delegation` up.
- The adopter edits and commits both `scripts/queue-config.sh` (a
  `templates/*-config.sh` destination) and `scripts/msg-patterns.list` (Delta 3's
  surface).
- Re-run `init` **at the same version, with no flags**. Assert both files are
  byte-identical, both are named in the output as changed, and `files[]` carries
  `init`'s hash for each rather than the adopter's.

The same-version re-run is the point: this defect needs no upgrade and no
`--force`, so an arm that only ran across versions would attribute it to the
upgrade path it does not live on.

**Why smoke arms rather than a gate.** The enforcement-first rule asks for the
gate that catches the class, and the honest answer is that the class — *a
consumer file written outside `claim()`* — has no cheap structural predicate:
`recipe_seed` legitimately writes guarded seeds and doctrine-kit's own installer
legitimately rewrites a block, so a "no `cp` into the consumer root" gate would
be false on landing. The installer's acceptor tier is the consumer smoke by
standing decision (§The consumer smoke; §Why the payload has no gate of its own
states why an installer property cannot ride the pre-commit path at all), and
these two arms put the class inside it. Recorded here so the absence of a gate
reads as a ruling rather than an omission.

## Producers and consumers

**The amendment introduces no new manifest field, no new wire key, no new
environment knob, and no new state.** The causal-completeness surface is
therefore the two interfaces it does introduce and the meaning it changes on one
existing field.

| New interface | Producer | Consumer |
| --- | --- | --- |
| `recipe_config_seam_plan` — one `<src>\t<dest>` line per `templates/*-config.sh` | `installer/lib/common/recipe.sh`, called from `init.sh`'s per-kit seam loop on every non-`--dry-run` *and* `--dry-run` run | `init.sh` in the parent shell: `copy_in "$src" "$dest"` |
| `recipe_seed`'s plan channel — the `<src>\t<dest>` line its gate-sdk arm emits in place of a copy | `installer/lib/common/recipe.sh`, called from the same loop | the same `copy_in` call site |

Field readers, per the every-field-has-a-named-reader rule: `<src>` is read by
`copy_in`'s `cp` (`init.sh:191`) and by nothing else; `<dest>` is read by
`claim` (`:190` → `:175`) at the claim transition, by `record` (`:192`) at the
manifest-assembly transition, and by `files_hash` (`:332`) at emit. Neither line
carries a third field, and neither plan is consumed anywhere but that loop.

**The changed meaning on an existing field**, with its readers named across the
whole component set — the roster was surveyed by grepping every reader of the
lock's `files` map across `installer/`, `gate-sdk/` and `scripts/`, not a
hand-picked subset:

| `files[]` reader | Where | Effect of the rule change |
| --- | --- | --- |
| `PRIOR_FILES` / `prior_hash` / `claim` | `installer/lib/init.sh:81`, `:157-162`, `:175-187` | a relinquished path now has a `prior_hash`, so a re-adding payload meets a claim it must refuse — this *is* the fix |
| the carry-forward loop | `installer/lib/init.sh:304-312` | Delta 1's site |
| `files_hash` / `manifest` | `installer/lib/init.sh:332-362` | unchanged; a retained entry emits its carried hash exactly as a refused one does |
| `lock_own_file` | `installer/lib/common/lock.sh:31-39` | excludes keys prefixed by a recorded `.kits` member, so retained relinquished paths — whose kit is still in `.kits` — stay excluded and `doctor`'s resolution is unaffected |
| `doctor`'s seam and registry reads | `installer/lib/doctor.sh:90`, `:109` | unchanged, via `lock_own_file` above |
| the smoke's roster assertions | `installer/consumer-smoke/run-smoke.sh:241`, `:262-265` | unchanged, and joined by Delta 6's |

**One gap this survey turned up and did not close**, filed rather than folded in
(`lifecycle-kit/bin/file-gap.sh`, 2026-08-05): `lock_own_file`'s exclusion
predicate reads the *recorded* `.kits`, while `files[]` already retains paths
under kits that have **left** the selected profile — so on a tree that narrowed
its profile a vendored fixture's `scripts/gates.list` can win the suffix match
and `doctor` reports it as the consumer's registry. Pre-existing and reachable
today, not introduced here, and outside this amendment's envelope.

## Existing sections updated

Every site below is named against the delta that owns it, so build adopts none of
them on its own authority.

**Delta 1 — the ownership rule:**

- `installer/README.md` §The manifest — the `files` row of the field table, and
  the paragraph ending "**A path leaves the roster only when `init` stops
  shipping it.**" That sentence is replaced by the exit rule; the surrounding
  carry-forward and revert paragraphs stay and are extended to say that a
  relinquished path is an ordinary entry rather than a new state.
- `installer/README.md` §init — "**Re-running is idempotent and
  non-destructive**" and its "that does not expire at the next upgrade" clause
  gain the relinquish case, which is the one direction they do not presently
  cover.
- `installer/lib/init.sh` — the `# spec:` comments at `:166`, `:184` and `:328`.
  The comment at `:298` is a special case worth naming: it already states the
  post-fix principle ("dropping it from the manifest would disown a file init
  created… an adopter-edited seed is exactly the entry the carry-forward exists
  for"), and is simply silent about the kit-path carve-out sitting directly
  beneath it. It is **incomplete rather than false**, and Delta 1 widens it to
  the rule it already half-states rather than reversing it.

**Deltas 2 and 3 — claim before write:**

- `installer/README.md` §What init seeds — "The **config seam is derived, never
  listed**" gains the claim-first property; the seed paragraph's "written only
  when it is absent" is now one of two disciplines and says which files take
  which.
- `installer/lib/common/recipe.sh` — the module header (`:2`), the seam
  function's `# spec:` (`:4`), and the seed function's (`:48`).

**Delta 4 — the replacement premise:**

- `installer/README.md` §The gate binary, "**The install location has one
  owner**" (`:215-226`).

**Delta 5 — restatements and dependents elsewhere:**

- `docs/install.md:246-249` — "Re-running is idempotent and non-destructive… it
  **reports rather than overwrites** anything you have changed since" — a
  restatement of the §init claim on the install page, rewritten with it.
- `TASK-QUEUE.md:3158` — the promoted `installer-payload-relinquish-stickiness`
  entry states "`init` drops it from `checkwright.lock`'s `files[]` **correctly —
  that half works**", asserting as sound design the exact behavior Delta 1
  reverses. Corrected as part of the promotion, since the entry's own framing
  (a residual "from the other direction") does not survive the merge.
- `TASK-QUEUE.md:811-817` — `installer-lifecycle-verbs`' acceptance shape rests
  on "`uninstall` removes only manifest-recorded files — never a file the adopter
  wrote, which is exactly what the per-file hash the manifest already records is
  for." Presently false for the two classes above: an edited `msg-patterns.list`
  hashes as unchanged, so a verb built to that criterion would delete an
  adopter's file believing it its own. Rewritten so a later `spec` on that entry
  does not inherit a false premise; the entry keeps its own scope.
- `installer/README.md` §The consumer smoke — the upgrade-arm paragraph
  (`:426-443`) gains Deltas 6 and 7, and the honest-limit paragraph is re-checked
  against what the new arms cover.

**Deltas 6 and 7 — the acceptor:**

- `installer/consumer-smoke/run-smoke.sh` — the upgrade arm (`:211-292`) and a
  new seam arm; the file header's `# spec:` (`:2`) enumerates the arms and gains
  them.

**Verified as needing no change**, recorded so build does not re-open them:

- `gate-sdk/SPEC.md` §upgrade-smoke (`:1639-1693`) — disclaims this territory
  explicitly ("never re-runs an installer, so a consumer's cross-version init
  path is outside its reach entirely") and points at §The consumer smoke and §The
  manifest rather than restating either claim. Independently re-checked.
- `TRAJECTORY.md:151-167` — the closed `init-claim-stickiness` ruling records
  what was true and fixed at the time; it is the ruling that *produced* the
  sentence Delta 1 replaces, not a restatement of it. A recorded ruling is
  closed, so it stands as history.
- `installer/consumer-smoke/run-smoke.sh:239-292` — remains true; see Delta 7.
- `docs/install.md:27-28` — generic ("both reach the same `init`, write the same
  `checkwright.lock`"); asserts neither claim.
- `TASK-QUEUE.md:17-34` — `installer-config-seam-silent-revert` describes the
  defect *as* a defect and is accurate; it dispositions at close rather than
  being rewritten here.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired (`under_kit`, `recipe_config_seam`, and the "stops shipping it"
      rule); nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
