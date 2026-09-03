# SPEC amendment: env-probe-cut

The port disposition of **`context-kit/bin/env-probe.sh` (141 lines)** — the
unblocked member behind §bin/env-probe — off the shell substrate as a bridged
`Arm::Emit` member of the gate binary. Cut B of `session-id-and-env-probe-cuts`,
under the port-only run (TRAJECTORY.md §PRIORITY DIRECTIVE); the two-cut unit set
was ruled by the **lead on 2026-09-03 on its own authority**.

**Measured at this HEAD rather than carried from the survey**: the port oracle's
`--tree` arm reads 109 files scanned, 64 declared `no-port`, 0 temporarily held,
**45 owed**, with `context-kit/bin/env-probe.sh owed lines=141` and
`context-kit/lib/toolfloor.sh owed lines=58` among them. The scope survey's
witness passed at this HEAD — an empty `git diff -- '*.sh'` since its rev and an
identical oracle trailer — so its finding (ii), that the crate already holds
`marker.rs::install_block` and a roster parser, is cited rather than re-bought.

**The member is not a gate**: it writes a consumer-local, gitignored file and
returns a status, so no `gates.list` row, no `.gate` descriptor and no
`good/`+`bad/` fixture pair. Its own suite is `context-kit/index-tests/`, whose
runner and cases stay shell (§Testing declares that group blocked as a whole).

## What changes

### (1) The cut takes a **proper subset** of its section, and the 2026-09-03 outer-bound ruling is what makes that lawful

Two tracked non-test `.sh` files declare `context-kit/SPEC.md §bin/env-probe` —
`bin/env-probe.sh` and `lib/toolfloor.sh` — and this cut takes the first alone
{design-bearing}. gate-sdk/SPEC.md §Porting a gate to the binary substrate rules
2026-09-03 (operator, lead-relayed) that **a section is a cut's outer bound, never
its minimum**, and names *this file* as the instance that grounds it: the owner
doc sequences `lib/toolfloor.sh` behind the installer's behind-invoke relocation
and "says nothing of the kind about `bin/env-probe.sh`". The sequencing prose
lives at §Layout and configuration rather than inside §bin/env-probe itself,
which is a precision worth having in writing — the ruling's summary compresses the
two, and a reader checking it against §bin/env-probe alone would not find the
sentence. The substance is unchanged: the owner doc sequences the library and
nothing sequences the script.

**The three refused alternatives are that ruling's own** and are not re-argued
here: *whole section or nothing*, *a `# port-until:` on the remainder*, and
*re-pointing the remainder's `# spec:` at another section*.

### (2) `--emit-env-probe` is a bridged `Arm::Emit` member on **one** knob

The member's contract is an **action that reports** — it rewrites a marker-bounded
block and prints one line naming what it did — and both its failures are already
exit 2, which is `Arm::Emit`'s collapse exactly {design-bearing}, so the variant
is `Emit` and the spelling `--emit-env-probe`, reached through the generic
`--emit <name>` composer rather than a front-end branch of its own (the
`--emit-stage-rules` disposition, on the same ground).

The declared roster is **`CONTEXT_KIT_ENV_PROFILE_FILE`** and nothing else. That
is the forced-family test resolving positively: `context-kit/lib/context.sh`
defaults the knob and is `# no-port:` as the config bridge's sole resolver for the
`CONTEXT_KIT_*` knobs, so the value is computed in exactly one place and the crate
holds no default to drift — **criterion 6 discharged by construction** for this
input, in its strongest form. A hardcoded top-level flag would resolve
`ENV.local.md` and silently ignore every consumer override, which §The non-gate
arm rules "not a calibration between two workable shapes but the difference
between working and appearing to". The contrast with this iteration's sibling cut
is worth one sentence: cut A's arm declares an **empty** roster because its two
names have no library home, and this one declares a single knob because its name
does — the two shapes are the two branches of one test, not a disagreement.

### (3) The marker write reuses `marker::install_block`, and the **one ruled divergence** travels with it

The crate already holds `inject_marker_block`'s contract: `native/src/marker.rs`
carries an *installer* write half that appends a fresh block when the begin marker
is absent — which is exactly what this member needs, because it legitimately
writes into a file that has never carried a block — beside a *generator* half that
refuses that same miss {design-bearing}. §lib/inject.sh states the split and rules
that "a porting session picks by what the caller is rather than by which function
it met first"; this caller is a seeding installer, so it takes `install_block`.
The read half is likewise already the crate's single implementation and serves the
change-detection compare.

**One behaviour changes, and it changes because §lib/inject.sh already ruled it.**
The shell gated a whole-line `awk` replace on a **substring** `grep`, so a marker
occurring inside prose sent it down a replace path that matched nothing and still
reported `replaced`; the compiled writer's marker-presence test is **whole-line**.
This member carries the same mismatch in its own change-detection step — a
substring `grep -qF` guarding a whole-line `awk` extraction — and the port
resolves both to whole-line. Recorded as an adopted ruling rather than as a
tidy-up: spec-over-precedent decides it against the implementation quirk, and
§lib/inject.sh is where that was decided.

### (4) The floor predicate becomes a **second holder**, so criterion 6's *unless* clause applies and a **standing** parity oracle is owed

This is the cut's cost centre and its one real design content {design-bearing}.
The member uses four of `lib/toolfloor.sh`'s five names — `PROBE_SET`,
`tool_floor_parse`, `tool_floor_version` and `tool_floor_check` — and the crate
holds **none** of the three functions today; `native/src/gates/install_toolchain.rs`
parses the roster's quadruple grammar and stops there. Probed rather than assumed:
`installer/lib/doctor.sh:66-70` calls `tool_floor_consumer_side`,
`tool_floor_parse`, `tool_floor_version` and `tool_floor_check` against its own
**payload copy** of the library, so the shell caller set does **not** empty and the
deletion road gate-sdk/SPEC.md §The port-candidate criteria prefers is unavailable.

What discharges the criterion is therefore an executed cross-substrate comparison,
in the shape gate-sdk/SPEC.md §Porting a gate to the binary substrate points at by
name: **evidence-kit/SPEC.md §lib/evidence.sh's**. A top-level
`--toolfloor-parity` arm outside `--list`'s roster drives the crate holder, a
`gate-tests/` harness drives one canned corpus of `(element, banner)` pairs
through both and compares **classification** — the closed verdict set and its
fields — never a rendered representation, and **no committed expected file** is
minted, because the failure this exists to catch is one side edited without the
other and a golden would be a third copy to drift.

**The obvious cheaper alternative is refused with cause.**
`context-kit/index-tests/toolfloor-cases.sh` already drives the *shell* predicate
over exactly such a corpus against a committed golden, so pointing a crate arm at
the same golden looks free. It is the third-copy shape evidence-kit refuses: the
predicate's verdict set would then live in three places, and a contract change
would take three edits where a two-holder comparison takes two. The existing
golden is not retired — it remains §Testing's own oracle for the shell holder,
which `installer/lib/doctor.sh` still runs.

**`sort -V` is preserved in the crate holder rather than replaced by a native
comparison, and this is the delta's sharpest call.** §bin/env-probe's
`uncomparable` verdict is the fail-closed arm for two conditions: a banner the
predicate cannot parse, **and a `sort` without `-V`**. A native Rust comparator
cannot reach the second, so it would silently narrow the verdict's reachable
conditions in the compiled holder while they stay live in the shell one — the two
holders would then disagree on a BSD or stock-macOS userland, which is precisely
the population the verdict exists for, and the parity oracle's canned corpus
cannot express an environmental condition. It would also falsify a published
claim: `docs/install.md` tells a reader that `init` "compares two versions with
`sort -V`, and `context-kit/bin/env-probe.sh` uses the same flag inside the floor
predicate", and calls the second "the sharper edge". Closing a live honest limit
inside a port is the move this track has refused three times — the worktree
predicate, §stage-rules' empty output on an unknown stage, and this section's own
`uncomparable` arm is the third. `sort` is on `GATE_SDK_PROGRAM_FLOOR`, so
criterion 7 is untouched.

### (5) The roster keeps **one** parser, extracted rather than duplicated

`install_toolchain.rs` already reads `PROBE_SET` out of `lib/toolfloor.sh` as
text and normalizes each element to a `name:min:impl:audience` quadruple, with a
`# spec:`-bound note that the grammar is *parsed* rather than sourced because a
fixture path is untrusted input {design-bearing}. The arm calls that reader rather
than writing a second one, so the roster's crate-side holder count stays at one
and the extraction is a promotion to a shared module, not a new copy — the shape
`--emit-port-blockers`'s port took for the corpus rule and the header-block read.
The roster **stays owned by `lib/toolfloor.sh`** and is never restated in the
crate: reading a consumer-visible file is what keeps the seam, and baking the
seven elements into Rust would ship one project's dependency set as a kit literal.

### (6) `lib/toolfloor.sh` stays **owed** and takes no `# port-until:`

Ruled by the lead on 2026-09-03 and identical to the 2026-09-03 outer-bound
ruling's own four binding facts {design-bearing}. The library keeps its `# spec:`
header pointing at this section, gains no hold declaration, and stays in
§port-blockers' **owed** column — a held file leaves that column and the
2026-08-28 completion predicate admits no contributor-side subtraction. Its
sequencing is prose in §Layout and configuration, at no cost, and
`kit-library-port-residue` remains the entry that owns it. So the `--tree` owed
count moves **45 → 44** on this cut alone, never 45 → 43 by also subtracting the
library.

### (7) The two session-context hook copies re-point together, and **nothing reds if they do not**

`scripts/session-context.sh:134` and `context-kit/templates/session-context.sh:132`
both run
`[[ -f "$CTX_BIN/env-probe.sh" ]] && bash "$CTX_BIN/env-probe.sh" >/dev/null 2>&1 || true`
as step 9's per-session auto-refresh {design-bearing}. Both become the front-end
form the same files already use for every other arm —
`[[ -f "$RUN_GATES" ]] && bash "$RUN_GATES" --emit env-probe >/dev/null 2>&1 || true`
— and `CTX_BIN` retires from both, its only reader being this line. Both files are
`# no-port:` and stay so; what changes is one invocation each.

**The largest hazard in this cut is that a stale line reds nothing.** The
invocation sits behind an `-f` guard on the probe's own path and swallows both
streams with `|| true`, so a port that leaves it unchanged means the guard is
simply false forever: the profile is emitted from disk and **never re-probed**,
the whole battery stays green, and the only symptom is a `Probed` date that stops
moving. That is §stage-rules' recorded hazard reappearing, so this cut takes its
three consequences unchanged — the re-point lands in the **deleting commit**, the
swallow is **preserved verbatim** (repairing it inside a port is fixing the rules
the port carries), and the Definition of Done requires the refresh **observed**
from a real hook run rather than read off the diff.

**One gate does read the pair and it is named because it is the only one.**
`check-template-copy-parity` holds the template and its filled consumer copy in
bidirectional parity — `scripts/session-context.sh`'s own `# no-port:` header
says so in as many words — so editing one file and not the other is a red. It
catches divergence between the two; it cannot catch both being left stale
together, which is the hazard above.

### (8) `lib/inject.sh`'s sourcer roster goes **2 → 1**, and a false sequencing claim is deleted with it

`gate-sdk/SPEC.md §lib/inject.sh` names two shell sourcers since 2026-09-03 —
`context-kit/bin/env-probe.sh` and `doctrine-kit/bin/install-doctrine.sh` — and
this cut removes the first {design-bearing}. `kit-library-port-residue` carries
the same roster and says `inject.sh` "moves behind them".

**One remains, so `inject.sh` does not become takeable and this cut claims no
unblock.** `install-doctrine.sh` is sequenced behind the installer's behind-invoke
relocation by an operator ruling recorded at `doctrine-kit/SPEC.md` §install-doctrine,
whose own text says it is written there "so no future composer re-selects it";
that member is deliberately not in this iteration's unit set and this amendment
must not be read as making it available. Both rosters are corrected from two to
one, and **neither may be rewritten to read *unblocked*** — the error a reader
who remembers the three-sourcer roster would make twice over.

**The sentence that goes with the member is false and its deletion is not
incidental.** §lib/inject.sh reads "`env-probe.sh` sits behind the Windows leg",
which nothing in the owner doc supports: probed, the only *blocked as a whole*
claim in `context-kit/SPEC.md` is §Testing's over its own runner and cases, and
the 2026-09-03 outer-bound ruling says in as many words that §bin/env-probe "says
nothing of the kind about `bin/env-probe.sh`". Later authority and
spec-over-precedent both decide it; the clause leaves with the member it
qualified, and the surviving sentence names `install-doctrine.sh`'s sequencing
alone. Recorded rather than quietly dropped, because a reader meeting the deletion
would otherwise take it for a consequence of the port rather than a correction.

### (9) Exactly one permission grant is deleted and none is added

`.claude/settings.json:38` carries `Bash(bash context-kit/bin/env-probe.sh)` and
it is the only line in the file naming this path {mechanical}. It is removed **in
the same commit as the delete**, the window the 2026-08-29 settings-grant
carve-out on `native-gate-port-remaining-corpus` exists to close, and the count it
demands be probed rather than assumed is **one**. No grant is added: the
post-port invocation is already covered by the committed
`Bash(bash gate-sdk/bin/run-gates.sh)` and `Bash(bash gate-sdk/bin/run-gates.sh *)`
entries, which is what keeps this inside the carve-out rather than against the
2026-08-22 bar.

### (10) The install step, both READMEs, the published install page and the tree listing

Five user-facing surfaces name the script and each re-points to the arm
{mechanical}: `context-kit/README.md`'s numbered step 4 plus its `--emit` command
roster, `docs/install.md`'s seed-a-profile paragraph and its GNU-`sort` bullet
(whose claim stays **true** by delta 4's decision — the floor predicate still uses
that flag, now in both holders), `context-kit/SPEC.md` §Layout and configuration's
`bin/` tree listing and its install-seeds-the-profile sentence, and
`context-kit/SPEC.md` §The session-context hook's step-9 description. The kit
README's roster line is the class's stated usage home for a bridged arm, and
`check-docs-cmd` assertion A is what makes each re-point mandatory rather than
optional once the fenced `.sh` path is gone.

### (11) The projection fan-out is the docs mirror alone

`context-kit/templates/session-context.sh` is **shell**, and §bin/footprint's
load-triggered tier measures the skill and template **markdown** a kit ships under
`templates/` {mechanical}, so delta (7) does not move a footprint figure and
`docs/value.md` does not follow — the opposite of this iteration's sibling cut,
and stated because the two look alike. What does regenerate is the docs mirror,
for every touched `SPEC.md` and `README.md`, and `docs/install.md` is
hand-authored except for its marker-bounded toolchain block, which this cut does
not touch. The full fan-out is read off `docs/site-architecture.md` §Generated
projections rather than restated here.

### (12) What the arm spawns, and why this member's set is the widest in its class

`--needs` answers about registry members only and a bridged arm is not one, so an
arm's spawned programs are recorded in prose and nowhere a machine reads
{design-bearing}. This member's set is `uname`, `date`, `sort`, and **every
roster member it probes** — which makes it the first member of the class whose
spawn set is **consumer-configurable**, since `PROBE_SET` is a file a consumer
can shadow. Written down because §The non-gate arm names `--upgrade-smoke` as
carrying "the heaviest set in the class" and a later reader sizing that claim
should meet this one beside it. The open work is unchanged and no entry is
minted: `bridged-arm-requirements-undeclared` already owns making arm
requirements machine-readable. The package-manager detection walk and the
`command -v` resolution become a `PATH` walk in the crate rather than a spawn,
which removes two spawns from the shell form and adds none.

## Producers and consumers

The cut introduces **no new state, event or interface**. It moves one existing
interface — a program that rewrites a marker-bounded block in a consumer-local
file and prints one line — from one substrate to another. The probed content, the
rendered verdict's grammar, the change-detection rule and both exit statuses are
unchanged, so the checklist runs over the **relocation** and over the deletion.

- **Producer.** `--emit-env-probe`, dispatched in `main` ahead of the registry
  lookup and absent from `--list`, reached through
  `bin/run-gates.sh --emit env-probe`. Its enabling configuration is the single
  bridged knob of delta (2), resolved by the front-end out of
  `context-kit/lib/context.sh` — a default a deployed configuration actually sets,
  because the library sets it unconditionally, so §The causal-completeness check's
  point 1 is satisfied by the bridge rather than by a consumer's diligence.
- **Consumers, both named and both re-pointed in this cut.** The
  **session-context hook**'s step 9, in both copies, calls it for effect and reads
  the file it wrote (delta 7); a **session or an installer step** calls it once to
  seed `ENV.local.md` (delta 10). §The non-gate arm's *named caller* property is
  satisfied twice.
- **The second reader of the artifact is a human**, and the artifact is
  gitignored, so no gate asserts its content — which is exactly why delta (7)'s
  hazard has no oracle and why the DoD demands an observed run.
- **No new field.** The block's five bullets are unchanged in grammar and in
  which reader consumes each.

**Every reader's RED condition, because this delta narrows a corpus.**
canon-kit/SPEC.md §The causal-completeness check point 5 binds — a reader is
clearable by inspection only where its verdict is monotone in the violation set,
and reds-on-finding-none, exact-count and coverage-floor shapes are not.

- **`check-docs-cmd` assertion A — reds *because* of the cut.** A governed doc
  still fencing `context-kit/bin/env-probe.sh` is a finding. This is signal, and
  it is what forces delta (10) rather than leaving it to authorial memory.
  `check-settings-paths` has the same shape over the allow-list and forces delta
  (9). **Neither fires from the generated hook's staged-path trigger on a
  deletion** — that gate's own recorded limit — so both are caught by the
  whole-tree battery.
- **`check-docs-cmd` assertion B is the zero-count reader, and here it is
  cleared by the file the cut does *not* take.** Its corpus is the kit roots minus
  `*.md` minus `*/gate-tests/*`, and **`native/` is not a kit root** — a kit root
  is a sibling of gate-sdk carrying `checks/` or `smoke/` — so moving a knob name
  into Rust does not satisfy it. `CONTEXT_KIT_ENV_PROFILE_FILE` is backticked in
  `context-kit/SPEC.md`, and after the delete its holders **inside that corpus**
  are `context-kit/lib/context.sh:44` — which this cut keeps, and which is
  `# no-port:` — and `context-kit/templates/session-context.sh:129`, which delta
  (7) keeps. `scripts/session-context.sh` holds it too and does **not** count:
  `scripts/` is not a kit root. Cleared, and cleared by inspection only because
  those two in-corpus holders are named.
- **`check-install-toolchain`** — reds when `docs/install.md`'s toolchain block
  and `lib/toolfloor.sh`'s `PROBE_SET` disagree on any element's
  name/floor/impl/audience quadruple. **A whole-element equality in both
  directions**, so it is the non-monotone shape by construction. It is untouched:
  the cut edits neither the roster nor the marker-bounded block, and delta (5)
  reuses that gate's own parser rather than minting a second one that could
  disagree with it.
- **`check-install-claim`** — red condition is a **zero count** over the
  `<!-- install-primary: -->` declaration. The cut deletes no marker and edits no
  install section's lead transport; `docs/install.md`'s env-probe paragraph is a
  seed-a-profile instruction, not a transport claim.
- **The port oracle's `--tree` arm and `check-gate-exemption-tasks`** — the
  former reports rather than reds; the latter reds on a `# port-until:` slug with
  no live queue entry, and delta (6) adds no such declaration. Owed goes 45 → 44.
- **`check-template-copy-parity`** — reds on divergence between the template and
  its filled copy, which is delta (7)'s named reader and the whole reason both
  files move in one commit.
- **`check-shellcheck` and `check-comment-tier`** — monotone in the same
  direction; removing a file removes findings, and the crate module's `// spec:`
  headers carry the ported comments' bindings.
- **`check-gate-substrate-parity` assertion B** — an **equality** between the
  `.gate` descriptor set and `--list`'s roster, which a non-gate arm stays outside
  by construction; that is what §The non-gate arm's first property buys.
- The build re-runs the full battery rather than resting on this enumeration,
  which is the enumeration's purpose: it says where to look when one goes red.

## Existing sections updated

- `context-kit/SPEC.md §bin/env-probe` — restated for the arm: the invocation
  form and its front-end reach, the one-knob declared roster, the whole-line
  marker resolution, the second holder of the floor predicate with its parity
  lane, and the `sort -V` preservation with the reason it is not a native
  comparison. The `## bin/env-probe` **heading is unchanged**, every citation to
  it resolving against the name (deltas 2, 3, 4, 5 and 12).
- `context-kit/SPEC.md §Layout and configuration` — the `bin/` tree listing loses
  a line, the *one library member beside `lib/context.sh` is owed* paragraph
  survives **unchanged** because delta (6) changes nothing about it, and the
  install-seeds-the-profile sentence names the arm (deltas 6 and 10).
- `context-kit/SPEC.md §The session-context hook` — step 9's per-session
  re-probe, described there by the tool it invokes (delta 7).
- `context-kit/SPEC.md §Testing` — the group stays blocked as a whole and
  `index-tests/toolfloor-cases.sh` keeps its role as the **shell** holder's
  golden; what is added is that a second holder now exists and is held to the
  first by a different oracle, so the golden's scope is stated rather than assumed
  to have widened (delta 4).
- `context-kit/lib/toolfloor.sh` — its `# spec:` header's sentence about "a
  second reader obtains the roster by sourcing" now has a third reader that parses
  it as text; the file gains **no** `# port-until:` (deltas 5 and 6).
- `context-kit/README.md` — the numbered step 4 and the `--emit` command roster,
  forced by `check-docs-cmd` assertion A (delta 10).
- `scripts/session-context.sh` and `context-kit/templates/session-context.sh` —
  one invocation each, `CTX_BIN` retired from both, moved in one commit because
  `check-template-copy-parity` is bidirectional; the template's `[EDIT ME]` prose
  re-points at the front-end (delta 7).
- `docs/install.md` — the seed-a-profile paragraph and the GNU-`sort` bullet; the
  latter's claim survives *because* of delta (4)'s decision, which is the one
  place a published page depends on an internal port choice (deltas 4 and 10).
- `gate-sdk/SPEC.md §lib/inject.sh` — the sourcer roster goes from two to one,
  the false *behind the Windows leg* clause is deleted with the member it
  qualified, and the library stays owed and **not** unblocked (delta 8).
- `gate-sdk/SPEC.md §The non-gate arm` — the class roster gains
  `--emit-env-probe` and `--toolfloor-parity`; the parity-arm paragraph gains its
  second member beside `--evidence-lib-parity`, and the spawned-programs paragraph
  gains the class's first consumer-configurable spawn set (deltas 4 and 12).
- `gate-sdk/SPEC.md §The port-candidate criteria` — criterion 6's *unless* clause
  gains a third worked instance beside `lib/queue.sh` and `gate_staged_matches`,
  and it is the first where the surviving shell consumer is **behind the install
  boundary** rather than in the battery (delta 4).
- `evidence-kit/SPEC.md §lib/evidence.sh` — named because delta (4) adopts its
  parity shape and its *no committed golden* ruling; no ruling changes there, and
  saying so is what stops a reader taking the reuse for a second copy of the rule
  (delta 4).
- `.claude/settings.json` — one allow entry deleted, none added, in the commit
  that deletes its target (delta 9).
- `docs/site-architecture.md` — no ruling changes; named because delta (11)'s
  fan-out is read off it, including the negative half (delta 11).
- `TASK-QUEUE.md`, the `kit-library-port-residue` entry — gains this amendment's
  `[spec:]` ref at **60 of 100 columns**, and the lead's 2026-09-03 own-authority
  ruling joins its two existing `lead … own-authority` declarations in one
  canonicalized line, which **saves** a line rather than costing one. At build its
  `inject.sh` paragraph goes from two sourcers to one and its member roster keeps
  all three members. It **demotes** rather than reaching `## Done` (deltas 6 and
  8).

<!-- update-target-exempt: sequenced behind the installer's behind-invoke relocation by an operator ruling whose own text says it is written there so no future composer re-selects it; delta 8 names it as inject.sh's remaining sourcer but takes no write on it -->
- `doctrine-kit/SPEC.md §install-doctrine` — already written, deliberately
  untouched.

<!-- update-target-exempt: the composer entry takes no body write from a cut by its own 2026-08-28 ruling, and this cut hosts elsewhere; its lead line already carries the sibling cut's ref -->
- `TASK-QUEUE.md`, `native-gate-port-remaining-corpus`'s body — deliberately
  unwritten.

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
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The per-session re-probe OBSERVED from a real hook run**, not read off
      the diff — delta (7)'s hazard is that a stale line is silent, and the only
      symptom is a `Probed` date that stops moving.
- [ ] **The parity lane runs and can fail** — the harness proved to red when one
      holder is edited and the other is not, since a parity test that passes
      vacuously is the failure evidence-kit's *no committed golden* rule is
      written against.
- [ ] **The `uncomparable` arm exercised on both holders** for a banner the
      predicate cannot parse, and the `sort -V` spawn confirmed present in the
      compiled holder.
- [ ] **`lib/toolfloor.sh` carries no `# port-until:`** and the `--tree` owed
      count re-read to confirm 45 → 44 on this cut, never 43.
- [ ] **The grant count re-probed at the deleting commit** and the allow entry
      removed in it, per the 2026-08-29 carve-out's own terms.
