# SPEC amendment: release-change-declared

Two holes in the release declaration surface's producer contract
(§upgrade-smoke), closed together because one oracle closes both. First, a kit
template that `init` claims can change as pure data, tightening gates in an
adopter's tree with no gate code moving, and nothing says which note section owns
that change. Second, a kit `bin/` tool can be deleted, and nothing checks that the
note declares it. Both surface only in an adopter's tree, and today neither can
red here. Sited in `gate-sdk/` because the surface, its producer clause and its
grammar's upgrade-smoke reader are that SPEC's. docs/install.md §The upgrade
contract owns the note grammar the deltas edit.

## What changes

### (1) A change to a template `init` claims is declared in both sections, always

A template `init` claims before it writes is the config seam plus gate-sdk's
`msg-patterns.list` (installer/SPEC.md §What init seeds). When a change to one
tightens a gate, it declares in **Tightened gates**, one bullet per gate it can
red, **and** in **Behavior changes**, one bullet whose lead token is the template
path. **{mechanical}**

The two sections serve two adopter populations, and both are real.
`init`'s claim rewrites a seeded path whose recorded hash still matches
(`native/src/installer/init.rs` `claim`), so an adopter who never edited their copy
takes the new content on upgrade. Their gates can then red on a clean run, and
that red is the Tightened-gates allowed-red set's subject. An adopter who did edit
the copy keeps it, and their copy diverges from the kit's. That divergence is the
copied-out-template residue docs/install.md folds into Behavior changes. The
folding sentence there covers the edited population only. It never considered
the unmodified seeded copy that `init` writes through. Declaring in both
sections, every time, removes a per-event judgment about which population a
change reaches, and it costs one extra line per event.

A template no `init` claims, `portability-patterns.list` for one, reaches an
adopter only as a copy they took out themselves. Such a change stays in Behavior
changes alone, which is the existing folding rule, unchanged.

### (2) Where the landing session has ended, the session that discovers the omission declares it

The producer clause gains a second producer. A kit-shipped change that reaches
the surface without its bullet is appended by **the session that discovers the
omission**, whatever its stage, in the commit that discovers it. **{mechanical}**

Today the clause names only the landing session, on the ground that it alone
knows what it changed when it changes it. That ground still makes the landing
session the **first** producer. But once that session has ended, the clause
names nobody, and every later session that could append is out of contract.
That is how `3763bc3e`'s pattern change has stayed undeclared from its landing
until now, and how a portability-pattern change did the same. Appending a bullet
records a fact that has already shipped. It is not a scope decision, it writes no
queue entry, and no stage is barred from making it. The composing close was
already a producer of last resort, since it appends what it finds undeclared
before transcribing (RELEASING.md §The procedure step 1). This delta moves that
duty from the release boundary to the moment of discovery.

### (3) `check-release-change-declared` — a removed kit tool and a changed claimed template must be named on the surface

A new consumer-declared gate in `scripts/`, compiled like its declaration-family
siblings (§The declaration cohort), at `tier=precommit`. **{design-bearing}**

- **Base.** The nearest `v*` tag reachable from `HEAD`, the tag `git describe
  --tags --abbrev=0 --match 'v*'` names. If no tag is reachable, the gate is
  **dormant**, and its clean line says so. It does not report "checked".
- **Changed set.** `git diff --cached --no-renames --name-status <base>`: the
  index against the base. Reading the index is what lets the commit that deletes
  a tool red at its own pre-commit. In the battery the index equals `HEAD`, so
  the two readings agree. `--no-renames` makes a rename count as a deletion of
  its old path.
- **Class R, a removed tool.** A path with status `D` whose first segment is a
  resolved kit root (`GATE_SDK_KIT_DIRS`, gate-sdk's resolver) and whose second
  segment is `bin`. Every file in that directory counts, not only `*.sh`,
  because the packer ships the whole kit root.
- **Class T, a changed claimed template.** A path that exists both at the base
  and in the index, is a template `init` claims, and whose non-comment,
  non-blank lines differ as a multiset between the two. The claimed set comes
  from the installer recipe's own derivation (`config_seam_plan` plus the
  `Seeded::Plan` sources `recipe::seed` returns), never from a list. A seam that
  was added or deleted is outside class T. So is a comment-only edit.
- **Declared.** Some top-level bullet under `## Behavior changes` in
  `<workflow-dir>/release-declarations.md`, read from the index, contains the
  path verbatim, or contains both the path's directory with a trailing `/` and
  the path's basename. The second form admits the directory-level bullets the
  surface already carries (`` `queue-kit/bin/` — deleted (`lesson-sink.sh`, …) ``).
- **Red.** One finding per undeclared path, naming the path, its class and the
  remedy. For class R the remedy is a Behavior-changes bullet naming the path.
  For class T it is that bullet plus a Tightened-gates bullet for each gate the
  change can red, per delta 1.
- **Exit 2.** A surface that is absent or lacks its `# contract:` header, a
  failed `git` call other than describe's no-tag status, or a kit-root set that
  does not resolve.

**Why classes R and T and nothing wider.** Each is decidable from a path and
the diff, and each has a measured instance. Across `v0.25.0..HEAD` the surface
names 34 of 40 deleted kit `bin/` paths under the rule above. The only class-T
path is `gate-sdk/templates/msg-patterns.list`, and it is undeclared (the
probes are under Producers and consumers). A wider class would sweep in every
stage-template edit. docs/install.md's folding makes each of those a Behavior
change in principle, but no release has ever declared them, so a gate over them
would red on day one against a rule nobody applies. That is a scope question for
a later unit, not a calibration to make here.

**Consumer-declared, and why.** Only the repository that authors kits deletes a
kit `bin/` tool or edits a kit template. An adopter's tree receives those
changes at upgrade and never makes them. So the gate lives in this repo's
`scripts/` beside `check-release-declaration-parity`, and no kit ships it. For
the same reason it reads the surface path the way that cohort does, fixed at
`.workflow/`. The knob-ownership question §The declaration cohort leaves open
is left open here too, and not answered for one member.

**Honest limits.** The gate checks that a path is **named**, not that the
bullet's prose is right. It cannot check the Tightened-gates half of delta 1,
because no derivation maps a template to the gates that read it. That half is
held by the producer and by the upgrade smoke, and only for gates that actually
red. A copied-out template outside the claimed set, a template edit anywhere
else, and any semantic change stay on the producer clause, which delta 2 now
extends to the discovering session.

### (4) The attested omissions are declared in the unit that lands the gate

At its landing commit the gate reds on today's tree, so the same commit makes
these declarations. **{mechanical}**

- `gate-sdk/templates/msg-patterns.list` (`3763bc3e`, the account-identification
  pattern): Tightened-gates bullets for `check-commit-msg` and `check-tree-terms`,
  plus a Behavior-changes bullet on the template path. Delta 1's rule, applied.
  The plural arm that `SPEC-account-noun-plural.md` adds to the same template is
  a further change, declared by that unit's own landing commit.
- `gate-sdk/templates/portability-patterns.list` (`dddbc065`, the `cd`-onto-a-
  command-substitution construct): a Behavior-changes bullet only, since `init`
  does not claim it.
- The six class-R paths the probe found unnamed. The bullets for
  `lifecycle-kit/bin/cite-survey.sh` and `file-survey.sh` are re-spelled with
  their paths: they are declared today, but as "lifecycle-kit's `cite-survey.sh`",
  which the rule does not match. New bullets are added for
  `delegation-kit/bin/run-budget-guard-tests.sh`, `run-dispatch-guard-tests.sh`,
  `usage-verdict.sh` and `doctrine-kit/bin/install-doctrine.sh`, each naming the
  arm or lane that replaced the tool. The existing `DELEGATION_KIT_VERDICT_BIN`
  Renamed-knobs bullet stays as it is: a knob removal and a path removal are two
  declarations.

### (5) The build-stage instruction names removal

lifecycle-kit/templates/stages/build.md's declaration paragraph names removing a
kit tool beside changing one, because the class-R red is what a build that
deletes a tool now meets. **{mechanical}**

## Producers and consumers

**Delta 1's obligation.** Producer: the session landing a change to a claimed
template, which is the build stage by the existing clause. Consumers: close,
which composes both sections into the note, and the upgrade smoke, which reads
the Tightened-gates lead tokens as the allowed-red set for the untagged `TO`.
The Behavior-changes bullet has one reader, the human upgrader, as docs/install.md's
honest limit on that section already states.

**Delta 2's second producer.** It is triggered by discovery, and discovery has
two routes. One is judgment, a session reading a diff or a note. The other,
from delta 3 onward, is the gate's red at the discovering session's own commit
for classes R and T. The consumers are the same as delta 1's. The enabling
configuration is the surface, which this repo deploys today.

**Delta 3's gate.**
- Producer of its verdict: the generated pre-commit hook and the whole-tree
  battery, once registered in `scripts/gates.list`.
- Inputs: the tag the base reads, which the `gates` workflow can see because it
  checks out with `fetch-depth: 0` (`.github/workflows/gates.yml`); the index;
  the kit-root resolver; and the installer recipe. Every input is deployed today.
- Consumer of its red: the committing session, which appends the bullet (delta 2).
- Every field of a finding has a reader. The path and the class tell that
  session what to name. The remedy tells it which sections to write.

**Roster-holding readers of the new gate name.** The probe is `git grep -l
check-release-declaration-parity`, run over the tracked tree minus
`docs/posts/` and the queue, as the nearest sibling's footprint:

- `scripts/gates.list`
- `native/src/gates/mod.rs`, the compiled registry
- `scripts/git-hooks/pre-commit`, generated
- `docs/check-graph.html` and `docs/enforcement.md`, generated
- `.workflow/gate-timing-baseline.txt`
- `docs/gate-sdk/SPEC.md`, the generated mirror

The gate also owes a `scripts/check-release-change-declared.gate` descriptor and
a fixture pair beside its siblings' under `scripts/gate-tests/`. The bad side
pins one case per class plus the dir-and-basename form. The good side pins a
comment-only template edit, a seam add, and the dormant no-tag case.

**Point 5.** No delta narrows a corpus. The gate reds on finding an undeclared
path, which is monotone in the violation set. Its no-tag branch is dormant
rather than red, and it reports that on its clean line.

**Point 6.** Delta 4 obliges each member of an enumerable corpus. The probe is a
scratch script run through `--scratch-run`. It diffs `--diff-filter=D v0.25.0
HEAD -- '*/bin/*'` and matches each path against the Behavior-changes bullets by
delta 3's rule; a second pass does the same for the claimed templates. Each
member's value is named in delta 4: 34 bin paths already satisfied, 6 re-spelled
or added, and 1 class-T path declared. The config-seam adds, deletes and renames
the probe also listed are outside class T by delta 3's definition, which is why
none of them is a member.

## Existing sections updated

- `gate-sdk/SPEC.md` §upgrade-smoke, the producer paragraph and "A gate whose
  *input* moved": both producers, the both-sections rule for a claimed template,
  and a pointer to the new gate. (deltas 1, 2 and 3)
- `docs/install.md` §The upgrade contract, the four-residue-classes folding
  paragraph: the unmodified seeded copy is named as the Tightened-gates half.
  (delta 1)
- `gate-sdk/SPEC.md` §The declaration cohort: the new member is named as the
  family's fourth, riding the cohort's `.workflow/` literal and its open
  knob-ownership question. It is not a cohort member by the port record.
  (delta 3)
- `RELEASING.md` §The procedure step 1: "A change the composing session finds
  undeclared is appended to the surface first" now cites delta 2's clause rather
  than standing alone. (delta 2)
- `lifecycle-kit/templates/stages/build.md`, the "Declare what a vendoring
  consumer will meet" paragraph. (delta 5)
- `.workflow/release-declarations.md`, the bullets delta 4 lists. (delta 4)
- `scripts/gates.list`, `native/src/gates/mod.rs`, the new module, descriptor
  and fixtures, and the generated hook, graph and enforcement projections.
  (delta 3)
- `docs/gate-sdk/SPEC.md`, the generated mirror, regenerated by its emitter.
  (all deltas)

### Replacement text

**gate-sdk/SPEC.md §upgrade-smoke, the producer paragraph. Not yet applied.**
Replace

> Its **producer** is the session that lands the change, in the same commit. That
> is the build stage, for every section (lifecycle-kit/templates/stages/build.md),
> and any other session whose commit ships such a change, such as a close fixing a
> drained bullet inline, on the same obligation. The landing session is the only
> one that knows what it changed at the moment it changes it, so the declaration is
> written from knowledge rather than reconstructed from commits or from a red.

with

> Its **producer** is the session that lands the change, in the same commit: the
> build stage for every section (lifecycle-kit/templates/stages/build.md), and any
> other session whose commit ships such a change on the same obligation. The
> landing session knows what it changed at the moment it changes it, so it
> declares from knowledge rather than reconstructing from commits or from a red.
> **Where that session has ended, the session that discovers the omission is the
> producer**, whatever its stage, in the commit that discovers it. Appending
> records a fact that has already shipped. It is not a scope decision and writes
> no queue entry. For a removed kit `bin/` tool and a data change to a template
> `init` claims, discovery is mechanical: `check-release-change-declared` reds
> the committing session until the surface names the path.
> **A template `init` claims declares in both sections when its change tightens
> a gate**: Tightened gates for each gate it can red, and Behavior changes on the
> template path. The unmodified seeded copy is rewritten on upgrade and reds;
> the edited copy is kept and diverges. Those are two populations, so the change
> takes two sections.

**docs/install.md §The upgrade contract, the folding paragraph. Not yet
applied.** Replace

> The
> copied-out-template class earns no section of its own because a template you have
> copied out that then changed *is* depended-on behavior diverging from your copy —
> it is behavior-folded, not dropped. Four classes, three sections, by that
> folding.

with

> The
> copied-out-template class earns no section of its own because a template you have
> copied out that then changed *is* depended-on behavior diverging from your copy —
> it is behavior-folded, not dropped. Four classes, three sections, by that
> folding. A template `init` seeded that you never edited is the one exception,
> because `init` rewrites it on upgrade: you take the change rather than diverge
> from it. So a change that tightens a gate through such a template is declared
> under Tightened gates as well.

## Retired spellings

- None — no delta retires a spelling. Delta 4 re-spells two surface bullets'
  wording without retiring a name, and the one name minted,
  `check-release-change-declared`, had no predecessor.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness
      check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — the build.md edit carries no
      grounds; delta 5 places them.
- [ ] **Merged with no information lost** — each addition re-phrases the text it
      refines; the merged §upgrade-smoke reads as one document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any cross-component gap discovered during the work is filed.
