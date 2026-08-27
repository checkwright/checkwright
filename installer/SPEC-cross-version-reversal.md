# SPEC amendment: cross-version-reversal

Closes `installer-uninstall-diff-stale-hash-coverage`. Every reversal the
consumer smoke asserts today runs on a consumer that has only ever met one
version's payload, so the reverse-to-pre-install property — the assertion form
of the claim the install page makes, and the property objective 4 turns into the
product — is asserted nowhere on the path an ordinary adopter is actually on
after a release.

**The fork the entry left open is ruled here, and it is not the cost judgment the
`[design-pending]` note assumed.** The entry offered two answers — chain a
reversal onto the upgrade arm's second hop, or give it its own arm — and made the
choice a smoke-cost one on the ground that the upgrade arm is already the most
expensive in the suite. Both halves of that framing fall to one reading of the
script.

The chain **cannot assert the property the gap names.** The upgrade arm's
consumer carries two committed adopter edits by the time its second hop lands
(`run-smoke.sh:568-573`), and an adopter edit is exactly the case tree-object
equality cannot host — the protection branch chained onto the seam arm says so in
those words at `run-smoke.sh:680` and installer/README.md §The consumer smoke
repeats it. A reversal chained there would have to re-inline the seam arm's
kept-set shape, which is a second copy of an assertion block that already exists,
and it would assert the **keep branch** after a payload change. That is a
neighbouring property. It would leave the named gap open while looking closed,
which is the worst of the three available outcomes.

The cost half falls too. A separate arm needs **no new pack**: `$UP` and `$UP2`
are already extracted under the smoke's scratch and live until the trap fires, so
the arm re-buys none of the three packs that are the upgrade arm's actual
expense. It costs one scratch consumer, three `init` runs, one `diff`, one
`uninstall --dry-run` and one `uninstall` — and it reuses `assert_reversal`
unchanged, adding no assertion code at all.

**So this is a feature and not debt, on canon-kit's own litmus, and the name it
mints is not the one the entry was watching.** The entry read the litmus as "a
new arm mints an arm name". The arm name is real but it is not the governed
surface: `run-smoke.sh:15` records that the unindented `printf` arm headers are a
**parsed contract**, and `scripts/parse-installer-smoke-log.sh:17-26` derives its
scenario roster from those very lines. A new header therefore mints a **baseline
scenario** in `.workflow/validate-baseline.txt`, which evidence-kit/SPEC.md
§Baseline manifest holds constant and rules that tooling never writes. That is a
contract another component must honor, which is the litmus verbatim.

**What this amendment does not claim, stated first because the entry's filing
already lost one premise to exactly this.** It does **not** claim the arm
exercises a recorded hash *moving*. `scripts/pack-installer.sh` assembles every
version from one worktree, so the three payloads carry byte-identical content and
the only shape difference between them is the one the upgrade arm mutates by
hand — the relinquished path deleted from `$UP`'s extracted `payload/`. A hash
that genuinely moves needs payload content that genuinely differs, which this
harness has no lever for and which this amendment does not build one for. What
the arm reaches is stated in delta 1 and is narrower and true.

**The seam, ruled explicitly because this amendment names a kit surface.**
Nothing here ships as kit mechanism. Every surface it writes is
repo-root-governed and owned by no kit — `installer/`, its `consumer-smoke/`
harness, and `.workflow/`'s local capture — and the one kit-owned surface the
amendment names, evidence-kit/SPEC.md §Baseline manifest, is **obeyed rather than
amended**: delta 3 adds an instance of a grammar that section already specifies,
which is why its update target is exempt below. No private rule content lands: no
vocabulary, no term list, no product constant, and no consumer config. It adds
**no knob** — `INSTALLER_SMOKE_TMP_DIR` stays the smoke's only one, and the
config-via-env convention has nothing to bind because there is nothing to
configure. The arm's one roster-shaped input, the scenario set, is **derived** by
the log parser from the smoke's own headers rather than listed anywhere, so the
delta adds no maintained list either.

**What it does not touch.** It does not change `uninstall`, `claim()`, the
manifest wire shape, or any verb's behaviour; it asserts contracts
installer/README.md §uninstall and §The manifest already carry. It does not
re-scope the upgrade arm, whose own assertions and consumer are untouched. And it
does not attempt to make the binary-less leg pass — that is
`binary-less-dispatch-loop-retirement`'s, and delta 3 records the consequence.

## What changes

### (1) A cross-version reversal arm, on its own consumer, after the upgrade arm

`installer/consumer-smoke/run-smoke.sh` gains one arm, placed immediately after
the upgrade arm's second hop and before the seam arm, because it consumes that
arm's two extracted packages and its relinquish mutation and belongs with the
cross-version material a reader is already holding. **{design-bearing}**

Its header is the parsed contract of `run-smoke.sh:15`, an unindented `printf`
with no redirect, naming the arm before its parenthetical:

`cross-version reversal arm (three versions, no adopter edit, PROFILE_MIN)`

The name is `cross-version reversal arm`, so the scenario the parser emits is
`cross-version-reversal-arm`. It does not collide with `upgrade arm`: the parser
matches a header line exactly or followed by `" ("`, and neither name is the
other's prefix under that test.

The arm's shape:

- **Its own scratch consumer**, `consumer cross-version-reversal`, at
  `$PROFILE_MIN`. Its own because the upgrade arm's consumer is edited and this
  arm's whole point is a consumer that is *not* — and at the lattice minimum for
  the reason the upgrade arm is: it is the smallest install carrying the manifest
  behaviour asserted, and it keeps the arm off a membership row that is a
  judgment.
- **`SEED` captured before the first `init`**, exactly as the per-profile loop
  and the download arm capture theirs, since that value is `assert_reversal`'s
  third argument and the whole oracle.
- **Three hops with no adopter edit**: `$CW init --profile "$PROFILE_MIN"` at
  `$VERSION`, then `bash "$UP/package/bin/checkwright.sh" init` with no flags,
  then `bash "$UP2/package/bin/checkwright.sh" init` with no flags. The bare
  `init` on both hops is deliberate and not a shortcut — it is the profile
  re-read from the manifest, the same path the upgrade arm drives.
- **`ENTRY` and `RUN_PATH` set by the arm itself**, to
  `(bash "$UP2/package/bin/checkwright.sh")` and `"$PATH"`. This is load-bearing
  rather than housekeeping: at this point in the script `RUN_PATH` still holds
  `"$TOOLMASK:$PATH"` from the toolchain-free arm (`:447`), because the `jq`-less
  arm deliberately sets none and says at `:511` that the arms below reach their
  own assignments untouched. An arm calling `assert_reversal` without setting
  both would silently run the reversal with `cargo` and `rustc` masked and drive
  the *first* version's entry point, asserting something other than what it
  says. `ENTRY` is the **latest** package because that is what an adopter holds
  after an upgrade, and reversing with the newest verb against a roster three
  versions old is the case under test.
- **`assert_reversal "$PROFILE_MIN" "$C" "$SEED"` unchanged.** No new helper, no
  new assertion body: `diff` clean, `uninstall --dry-run` planning a non-zero
  removal while the tree object and worktree are untouched, then `uninstall`
  returning the committed tree object to `SEED`, a clean worktree, and no
  surviving `checkwright.lock`.

**What the arm actually reaches, narrowly and truly.** Two things, neither
asserted anywhere today:

- **Reverse-to-pre-install across a payload that changed shape.** The roster
  carried the relinquished path across a hop whose payload did not ship it and
  into a hop that re-added it. The end state must still be wholly removable. A
  hop that disowned the path and a hop that re-added it without re-recording both
  leave a file on the tree that `SEED` equality reds on.
- **The roster covering the *upgrade* write set.** installer/README.md §The
  consumer smoke already rules that tree-object equality is the only assertion
  that a file `init` wrote and failed to record cannot survive. That rule is
  asserted today only over a **first** `init`. An upgrade hop has its own write
  set — a rewritten manifest, regenerated projections, whatever a future release
  adds — and nothing anywhere asserts the roster covers it.

**The arm proves its own premise, in the idiom this suite already uses twice**
(the binary-less leg's disclosure count, the upgrade hop's omission count). Three
assertions before the reversal, each naming what a green would otherwise mean:

- `[[ ! -f "$UP/package/payload/$RELINQUISHED" ]]` — the relinquish mutation is
  still in effect. Re-order the script and it silently is not, and the arm
  becomes three identical hops.
- `.version` on the arm's own manifest equals `$UP2_VERSION` — the consumer
  really crossed both hops rather than falling through one.
- `.files` on that manifest records `$RELINQUISHED` — the path survived the
  payload hole and is on the roster `uninstall` must clear. Without it the arm
  is reversing an ordinary install wearing a cross-version name.

**Failure messages carry the re-scope remedy, never a licence to drop the
assertion**, matching the two tripwires above it.

### (2) installer/README.md §The consumer smoke documents the arm and its limit

The section gains a paragraph after the upgrade arm's two, and the amendment
requires it to carry the honest limit rather than only the claim: that the three
payloads are byte-identical by construction, that the relinquish mutation is the
suite's only lever on payload shape, and that what the arm asserts is therefore
removability-after-a-shape-change and roster coverage of the upgrade write set —
not a moving hash. **{design-bearing}**

Two existing claims in the same section are re-read as part of this delta and
one moves. **The tree-object-equality paragraph** ("it proves more than
`uninstall`" — that nothing else asserts the manifest covers everything `init`
wrote) is today true only of a first `init`; it gains the cross-version reach.
**The reversal-arm paragraph's** "every profile is installed *and* reversed"
stays exactly as it is: it is a claim about the per-profile loop and this arm is
not in it.

**Checked and left alone, recorded so the next reader knows it was checked.** The
dated cost measurement ("272 seconds", re-measured 2026-08-13) enumerates cost
*drivers* — four profiles, four packs, an npm install, one release build — not
arms. This arm adds no pack and no build, so that sentence stays true and is not
edited. The `INSTALLER_SMOKE_TMP_DIR`-is-the-only-knob claim also stays true: the
arm adds no knob.

### (3) The baseline gains one row, at `ignore` on the standing slug

`.workflow/validate-baseline.txt` gains
`installer_smoke cross-version-reversal-arm ignore binary-less-dispatch-loop-retirement`,
placed with the other `installer_smoke` rows. **{mechanical}**

`ignore` rather than `pass`, and the reason is a fact about the tree rather than
a hedge: the smoke aborts at the binary-less leg today
(`installer_smoke binary-less-leg fail binary-less-dispatch-loop-retirement`), so
every arm after it is unreached and every one of them is baselined `ignore` on
that same slug. The new arm is one of them. The slug names the **standing** unpaid
price the row is written to hold visible, which is §Baseline manifest's own rule
for choosing among candidate causes, and it resolves to a live deferred task so
`check-evidence-baseline`'s slug-liveness arm holds.

**This is the delta that makes the amendment cross-component**, and it is worth
saying plainly rather than leaving to the gate: `installer/` mints the scenario,
`evidence-kit/` owns the surface that must hold it.

### (4) The two arm rosters this repo keeps in prose are extended

Both are single-line restatements that already enumerate every arm, so both go
stale the moment delta 1 lands. **{mechanical}**

- `run-smoke.sh:2`, the file's own `# spec:` header, whose tail enumerates what
  exit 0 asserts.
- `run-smoke.sh`'s final `INSTALLER-SMOKE: clean (...)` summary `printf`, which
  enumerates the same set for a reader of the log.

Neither is a second copy of a machine-read roster — the machine roster is derived
from the headers by delta 1's parser — so extending them is restatement upkeep
rather than a maintained roster the derivation-first rule would refuse.

## Producers and consumers

**The one new thing this amendment introduces is a scenario name.** Everything
else is a call site of existing interfaces, and the arm's internals are assertions
rather than state. So the causal chain is stated for the scenario, and each other
new call is named against the interface it uses.

**New interface: the baseline scenario `cross-version-reversal-arm`.**

- **Producer** — the unindented `printf 'cross-version reversal arm (…)\n'` at
  the arm's head in `installer/consumer-smoke/run-smoke.sh` (delta 1). Its
  **enabling config is already emitted everywhere it must be**: the producer is
  read only through `EVIDENCE_KIT_PARSER_installer_smoke='bash
  scripts/parse-installer-smoke-log.sh'` and `EVIDENCE_KIT_RUN_installer_smoke`
  in `scripts/evidence-config.sh`, both set today, and `installer_smoke` is
  already in `EVIDENCE_KIT_SUITES`. No config is added and none is test-only —
  this is the same channel the nine existing arm scenarios ride.
- **Consumer 1** — `scripts/parse-installer-smoke-log.sh`, which needs **no
  edit**: it derives its arm roster from the smoke script's own headers
  (`:17-26`) and matches a log line against each (`:37-42`). It emits
  `cross-version-reversal-arm pass|fail`, or, for an arm the run never reached,
  **nothing at all** — the behaviour its own `# spec:` header states, so the
  baseline's absent-scenario rule judges it. This is the derivation-first
  property that makes the delta cheap and it is the reason no roster is
  hand-edited anywhere in the chain.
- **Consumer 2** — `ek_diff` (evidence-kit/SPEC.md §bin/diff-baseline.sh),
  reached from `evidence-kit/bin/run-validate.sh` at the validate stage and from
  `bin/diff-baseline.sh` situationally. The transition is the per-scenario diff:
  a baselined `ignore` scenario observed green is an *unpromoted recovery*, not a
  red, which is exactly the disposition this row needs while the arm is
  unreachable.
- **Consumer 3** — `check-evidence-baseline` (a compiled gate,
  `native/src/gates/evidence_baseline.rs`), at every commit touching the
  baseline. It reads the row's `<suite> <scenario> <status> <slug>` grammar and
  asserts the slug resolves to a live queue task.

**Every field of the new row has a named reader.** The row has four:
`installer_smoke` read by `ek_diff`'s suite slice and by
`check-evidence-baseline`'s suite-coverage arm; `cross-version-reversal-arm` read
by `ek_diff`'s per-scenario key; `ignore` read by `ek_diff`'s status split; and
`binary-less-dispatch-loop-retirement` read by `check-evidence-baseline`'s
slug-liveness arm and by a human reading why the row is not `pass`. No field is
added beyond the shape every existing row already carries.

**Existing interfaces the arm calls, each with the contract it relies on.**

- `consumer <label>` (`run-smoke.sh:150`) → a fresh git-seeded scratch consumer.
  The arm reads its echoed path and captures `HEAD^{tree}` from it as `SEED`.
- `$CW` and `bash <pkg>/bin/checkwright.sh init` → `installer/lib/init.sh`, which
  **commits** its write set (`:390`, `:404`). That commit is why `HEAD^{tree}` is
  the reversal oracle at all, and the arm depends on it exactly as every existing
  reversal does.
- `assert_reversal` (`run-smoke.sh:315`) → unchanged, and its two globals `ENTRY`
  and `RUN_PATH` are set by the arm per delta 1. This is the one place the arm
  could have gone wrong silently and delta 1 pins it.
- `$UP`, `$UP2`, `$RELINQUISHED`, `$R_INIT_HASH`, `$UP2_VERSION`, `$PROFILE_MIN`
  → script-level values the upgrade arm already established, live until the exit
  trap. The arm reads them and writes none of them back, so the upgrade arm's own
  assertions are unaffected by the arm's placement after it.

**No corpus is narrowed by any delta**, so point 5's red-condition enumeration
has no subject here: delta 1 adds a call site, delta 3 adds a baseline row, delta
4 extends two prose lines, and no glob, prune set, roster or file set anywhere
gets smaller. The one reader whose verdict *could* be non-monotone —
`check-evidence-baseline`'s suite-coverage arm, which reds on a suite carrying
**no** row — is moved in the safe direction: `installer_smoke` gains a tenth row
and never approaches zero.

## Existing sections updated

- `installer/README.md` §The consumer smoke — the new arm's paragraph, and the
  tree-object-equality paragraph's extension from a first `init` to a
  cross-version one (delta 2).
- `installer/consumer-smoke/run-smoke.sh`'s file-header `# spec:` restatement and
  its terminal `INSTALLER-SMOKE: clean` summary, both of which enumerate the arm
  set (delta 4).
- `.workflow/validate-baseline.txt` — the `installer_smoke` slice gains its tenth
  row (delta 3).
- `evidence-kit/SPEC.md` §Baseline manifest — <!-- update-target-exempt: the row this
  amendment adds is an instance of the grammar that section already specifies, and its
  `ignore`-plus-standing-slug disposition is the rule that section already states; changing
  the section would be re-specifying a contract this amendment only obeys -->
- `TASK-QUEUE.md` — `installer-uninstall-diff-stale-hash-coverage` promotes from
  `## Deferred` to `## New Features`, dropping `[design-pending]` and its
  `Cost while deferred` field and taking `[spec: SPEC-cross-version-reversal.md]`
  (all deltas).

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
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
