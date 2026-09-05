# SPEC amendment: agents-md-cut

The port disposition of **`context-kit/smoke/agents-md.sh` (139 lines), the unblocked
remainder of context-kit/SPEC.md §Testing**: it ports to a bridged `--agents-md-smoke`
`Arm::Run` on the **duplication-absent road** — the arm spawns `bash` to call
`gate-sdk/lib/consumer-smoke.sh`'s helpers in the library that owns them, exactly as
`--upgrade-smoke` already does. A stated-contract cut under the port-first run
(TRAJECTORY.md §PRIORITY DIRECTIVE), hosted on its own per-cut feature entry and packaged by
the lead as one of this iteration's four.

**The composer's precondition was run at this stage rather than inherited.**
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --group` trails *0 group(s) formed …
0 takeable at this cut* — no takeable group, which is the budget arm's stated precondition
(gate-sdk/SPEC.md §The first cohort). The selection ground is the **owed column** of
`--emit port-blockers --tree` — *89 file(s) scanned, 66 declared no-port, 0 temporarily held,
23 owed* — where this file reads `owed lines=139`.

**This cut takes a proper subset of its section, and the owner doc has already ruled it
takeable.** context-kit/SPEC.md §Testing holds three owed members: `bin/run-index-tests.sh`
and `index-tests/toolfloor-cases.sh`, both sequenced behind the installer's behind-invoke
relocation, and this one. The section says so in its own words — the sequencing "reaches
exactly those two members: `smoke/agents-md.sh` declares the same section, sources neither
the runner nor the library, and its own header calls it a standalone validate suite, so it is
unblocked on its own ground". That is gate-sdk/SPEC.md §Porting a gate to the binary
substrate's *a stated cause reaches only the members it names*, applied by the owner doc to
itself. The two sequenced members stay **owed** and take no `# port-until:`.

## What changes

### (1) The cut is §Testing's unblocked remainder, and it does not discharge context-kit

`smoke/agents-md.sh` reads `owed lines=139` {mechanical}. The section's other two owed
members are unaffected and their sequencing sentence is unchanged; after this cut context-kit's
owed column holds those two plus `lib/toolfloor.sh`, each behind the same relocation and each
named in its own section. The amendment says so because a kit whose owed column shrinks by one
member reads like a discharge otherwise.

### (2) The arm is `--agents-md-smoke`, an `Arm::Run`, and its knob roster is the consumer's

The member ports as a **bridged-arm table** row —
`("--agents-md-smoke", Arm::Run(agents_md_smoke::run), agents_md_smoke::KNOBS)` in
`native/src/emit/mod.rs`'s `BRIDGED_ARMS`, body in `native/src/emit/agents_md_smoke.rs`
{design-bearing}.

**`Arm::Run` on the variant's stated test**: the contract is a verdict — 0 with the
`AGENTS-MD-SMOKE: clean (…)` line, 1 with a `FAIL — <reason>` line, 2 for a precondition the
harness could not meet. An `Emit` member could carry the report but not the verdict, and the
verdict is what the evidence-kit suite reads.

**The declared roster is `GATE_KIT_ROOTS_HERE` and `GATE_SDK_NATIVE_BIN`.** The first is what
`gate_kit_roots` answers today (`smoke/agents-md.sh:23`) and what tells the arm which kits to
vendor; the second is what `csmoke_place_binary` needs and what the arm resolves through the
`gate_native_bin` accessor rather than re-spelling. It declares **no `CONTEXT_KIT_` knob**,
and the omission is the point: every knob this smoke touches — `GATE_SDK_AGENT_FILE`,
`LIFECYCLE_KIT_AGENT_FILE`, `DOCTRINE_KIT_AGENT_FILE`, `CANON_KIT_MANIFEST_FILES`,
`CONTEXT_KIT_SURFACES`, `CONTEXT_KIT_BREVITY_FILE` — it **writes into the scratch consumer's
own config seams** and never resolves for itself. Resolving them here would hand the arm this
repo's posture in place of the one it is constructing.

**`--agents-md-smoke` and not `--emit-agents-md-smoke`.** §The non-gate arm reads the
`--emit-` prefix as load-bearing for the emit family alone, because `bin/run-gates.sh`
composes it from an `--emit <name>` operand; a non-emitting `Arm::Run` is spelled directly,
which is what `--upgrade-smoke`, `--run-validate` and `--scratch-run` already do. No
front-end edit is owed either way: `bin/run-gates.sh:45-70` hands every unrecognised leading
`--<token>` to `exec_arm` untouched.

### (3) The consumer-smoke helpers are called in the library that owns them

This is the cut's central design ruling {design-bearing}.

**The obstacle, stated first.** `gate-sdk/lib/consumer-smoke.sh` carries `# no-port:` under
§Consumer smoke *The port disposition*'s leg 1 — it "sits inside the bridge rather than beside
it", and a crate-side form would be the second producer criterion 6 refuses. This smoke's
whole first act is `csmoke_vendor_and_install` (`smoke/agents-md.sh:39`). A reader will
therefore expect the member to be undeportable.

**It is not, and that library's own header already names the road it takes.** It records that
"the upgrade suite is now a *spawn-side* caller: its bridged arm invokes these helpers in a
bash that sources this unchanged library, which creates no second producer and is why that
port cleared criterion 6 on the **duplication-absent road**". The mechanism is
`native/src/emit/upgrade_smoke.rs:380-460`: a `bash -c` whose script sources
`lib/gate.sh` and `lib/consumer-smoke.sh` and then calls the helper, with the helper's
`SCRATCH` communicated back on stdout because it is set in the callee's own shell.

**The ruling: this arm takes the same road and reuses the same crate-side helper.** The
`csmoke()` spawn wrapper and its `SOURCE_CSMOKE` script prologue are lifted out of
`upgrade_smoke.rs` into a shared crate module both arms call, rather than copied. A second
copy would be the duplication the road exists to avoid, relocated from two shell scripts into
two Rust modules — the same refusal drift-kit's session-derivation extraction took, and the
same one §lib/gate.sh's *exactly one place a value is computed* states.

**The library's own `# no-port:` header is an update target of this cut**, because it names
`context-kit/smoke/agents-md.sh` in its live list of *shell sourcers* that "are outside this
cut and keep sourcing it". After this cut that is false: this member moves to the spawn side
and only `demo/run-demo.sh` remains a shell sourcer, for `csmoke_place_binary` alone. Leaving
the sentence standing would leave the library's stated caller census wrong, which is the exact
class of staleness delta 7 fixes fourteen lines away in another file.

### (4) What the arm does itself, and the one place it must not

Everything after the vendoring is the arm's own {design-bearing}, and each step keeps its
specified assertion:

- **The agent-file conversion** — `git mv CLAUDE.md AGENTS.md` in the scratch consumer, after
  asserting the installed baseline actually wrote a `CLAUDE.md` to convert.
- **The six knob writes into four config seams** — `scripts/gate-sdk-config.sh`,
  `scripts/context-config.sh` (appended), `scripts/doctrine-config.sh`,
  `scripts/canon-config.sh` — written as file content by the arm, since they are the
  consumer's config and not the arm's.
- **The two battery-env values** — `LIFECYCLE_KIT_AGENT_FILE` and `CANON_KIT_CONFIG_FILE` —
  which ride the environment of every spawned battery run because the lifecycle knob is
  scalar with no default config file and canon resolves its manifest only through its config
  file.
- **The regeneration ordering**, which is the step a port most easily loses: each kit's own
  `install.sh` already wrote the hook and `CHECK-GRAPH.html`, but before `canon-config.sh`
  existed, so the baked `# graph:` derivation used canon-kit's bare default. The arm
  regenerates both **under the same env the battery will run with**, or `check-graph` reds the
  AGENTS.md consumer on a hook stale by construction.
- **The four assertions** — the battery green with an `All <N> gates passed` line;
  `always-loaded` measuring a surface count equal to `AGENTS.md`'s line count;
  the footprint emitter reporting a non-zero always-loaded total; and `check-root-tiering`
  accepting an orientation-clean `AGENTS.md` root while rejecting a stray second agent file
  **and naming the stray**.

**The one place it must not act for itself is the root-tiering dispatch.** The smoke resolves
`check-root-tiering`'s argv **once against the vendored tree**, whose knobs and binary are the
consumer's, and then runs it from a separate orientation-clean repo — the shape
`smoke/agents-md.sh:117-127` already spells, through a `bash -c` sourcing `lib/gate.sh` and
calling `gate_command`. That resolution stays on the shell side for delta 3's reason, and the
comment explaining why a literal `checks/<gate>.sh` path would name a substrate the port has
moved stays with it.

**`--keep` ports unchanged.** It is an argument the rule itself consumes, arriving as argv
into the arm — not a selector for where configuration comes from — so §The non-gate arm's
distinguishing test puts it in the second kind. Its documented sentence in the file's usage
header moves to the arm's usage text.

### (5) The scratch lifecycle moves from a trap to the arm's own control flow

The shell form owns two scratch trees and one `trap … EXIT` that is **re-armed** partway
through (`:37` then `:112`), so the second `trap` is what makes the first tree's cleanup
survive the second tree's creation {design-bearing}. A port that transcribes the traps into
Rust `Drop` guards inherits an ordering that was a shell idiom rather than a contract.

**The ruling: the arm cleans both trees on every exit path, `--keep` suppressing both**, and
the re-arming disappears with the traps. The behavior a reader can observe is unchanged — on
`--keep` the arm prints the retained path, as `:35` does — and the arm gains the property the
shell form only had by construction: a failure between the two `trap` calls cannot leak the
first tree.

### (6) §The non-gate arm's roster gains the member, undated

The class roster gains `--agents-md-smoke` with its owning section named and **no
landing-date cohort label** {mechanical}, for the reason the sibling cuts state: those labels
are the subject of this iteration's `kit-spec-provenance-seam-sweep`, which rules them
provenance and retires them. The arm's spawned-program set — `bash`, `git`, `mktemp`, and
through the vendoring whatever `csmoke_vendor_and_install` reaches — joins the prose paragraph
that records those sets, `--needs` answering about registry members only.

### (7) The staleness at gate-sdk/SPEC.md:2149 is corrected on the surface this cut already edits

gate-sdk/SPEC.md §Porting a gate to the binary substrate still states **in the present tense**
that context-kit/SPEC.md §Testing "declares its own group *blocked as a whole*"
{mechanical}. context-kit/SPEC.md:1310-1318 no longer does: the 2026-09-05 correction landed
in the owner doc and was never back-ported to the illustrative passage sitting twenty-four lines
above the corollary that names the same correction (`:2173-2187`, *a stated cause reaches only
the members it names*). So one file states, twenty-five lines apart, both that the group is
declared blocked whole and that the owner doc corrects its own over-declaration.

**This cut owns the fix because it is the member the correction was made for.** The
illustrative passage's whole point is which of context-kit's groups cuts whole, and this
amendment is the cut the answer licenses; a sibling cut fixing it would be repairing a
sentence about a member it does not touch. The correction is *in place* — TRAJECTORY.md's
authoring rule that "a fact that has aged is corrected where it stands", never appended beside
the sentence that corrects it.

## Producers and consumers

**New interface: the `--agents-md-smoke` bridged arm.**
*Producer* — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` row, dispatched in
`native/src/main.rs:485-498`; its enabling configuration is the bridged environment
`gate_knob_env` builds for it, resolved and exec'd by `bin/run-gates.sh:32-37`. Reachable
with no front-end edit.
*Consumer* — `scripts/evidence-config.sh:29`'s `EVIDENCE_KIT_RUN_agents_md_smoke`, re-pointed
to `bash gate-sdk/bin/run-gates.sh --agents-md-smoke`, read by `--run-validate` at the
validate stage through `EVIDENCE_KIT_PARSER=exit-code`, which reads the arm's exit status. The
suite keeps its name `agents_md_smoke`, so evidence-kit's roster, the validate spine and the
baseline slice are untouched.

**New crate module: the shared consumer-smoke spawn helper.**
*Producer* — extracted from `native/src/emit/upgrade_smoke.rs:380-460` into a module both arms
call. *Consumers* — `upgrade_smoke::run` (its existing calls re-pointed in the same commit)
and `agents_md_smoke::run`. Two callers crate-wide, both named; it exports nothing else, so
the extraction adds no reachable surface.

**Existing interface whose caller census changes: `gate-sdk/lib/consumer-smoke.sh`.**
*Producer* — unchanged; the library's body is not edited. *Consumers* — its shell-sourcer set
loses this member and keeps `demo/run-demo.sh`; its spawn-side caller set gains this member
beside `--upgrade-smoke`. The census is stated in the library's own `# no-port:` header and in
§Consumer smoke, and delta 3 updates both.

**Fields and their readers.** The arm introduces no new message and no new field. Its one
published value is its exit status, read by the validate spine and by a contributor; the
`--keep` path additionally publishes a scratch path on stdout, whose reader is the
contributor reproducing a failure and whose transition is the arm's own exit.

**This delta set narrows no corpus.** The vendored kit set, the four config seams, the six
knobs and the four assertions are unchanged in extent. Point 5 of the causal-completeness
check therefore does not bind; the two readers whose red conditions decide whether the port
worked are named instead: `check-graph` reds when the committed hook disagrees with the
`# graph:` derivation under the resolved config, which is what delta 4's regeneration ordering
exists to satisfy, and the battery's own `All <N> gates passed` line is the smoke's primary
assertion and is matched as a pattern rather than a count.

## Existing sections updated

- **context-kit/SPEC.md §Testing** — the smoke's mechanism restated for the arm: the spawned
  vendoring, the arm's own steps, the regeneration ordering, the scratch lifecycle and the
  `--keep` argument (deltas 2, 3, 4, 5). The sequencing sentence naming the other two owed
  members is unchanged (delta 1).
- **gate-sdk/SPEC.md §Consumer smoke, *The port disposition*** — the library's caller census:
  this member leaves the shell-sourcer list and joins the spawn-side list (delta 3).
- **`gate-sdk/lib/consumer-smoke.sh`'s `# no-port:` header** — the same census, in the
  declaration a `--tree` reader meets first (delta 3).
- **gate-sdk/SPEC.md §The non-gate arm** — the class roster gains `--agents-md-smoke`,
  undated; the spawned-program prose gains this member's set (delta 6).
- **gate-sdk/SPEC.md §The port disposition** — the owed-corpus prose every cut moves;
  context-kit's owed column after this cut (delta 1).
- **gate-sdk/SPEC.md §Porting a gate to the binary substrate** — the stale present-tense
  clause about context-kit §Testing's group, corrected where it stands (delta 7).
- **gate-sdk/SPEC.md §upgrade-smoke** — the sentence naming the three consumer-smoke helpers
  as "called in the library that owns them" now has two callers rather than one (delta 3).
- **`scripts/evidence-config.sh`** — `EVIDENCE_KIT_RUN_agents_md_smoke` re-pointed (delta 2).
- **docs/positioning.md §The tiered compatibility claim** — the Tier-two claim names this
  exercise; the exercise keeps its name and its assertions and the pointer stays valid, so the
  page is checked rather than edited (delta 4).
- <!-- update-target-exempt: a generated projection with its own freshness gate and regen command, rostered in docs/site-architecture.md §Generated projections and their freshness gates; it stales on the line-count change alone and no delta owns its content --> `docs/footprint.md` and `docs/value.md`'s rollup block.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable
      producer and a named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section (not appended); the merged spec reads as one coherent document a
      reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec, config and doc for
      `smoke/agents-md.sh`; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
