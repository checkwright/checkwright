# SPEC amendment: guard-tests-cut

The port disposition of **`guard-kit/bin/run-guard-tests.sh` (101 lines), the one owed file
declaring guard-kit/SPEC.md §Testing**: it ports to a bridged `--run-guard-tests`
`Arm::Run`, and its **subject does not move** — `templates/bash-guard.sh` and
`lib/guard.sh` are both permanently shell and both stay spawned exactly as they are today.
A stated-contract cut under the port-first run (TRAJECTORY.md §PRIORITY DIRECTIVE), hosted on
its own per-cut feature entry and packaged by the lead as one of this iteration's four.

**The composer's precondition was run at this stage rather than inherited.**
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --group` trails *0 group(s) formed …
0 takeable at this cut* — no takeable group, which is the budget arm's stated precondition
(gate-sdk/SPEC.md §The first cohort). The selection ground is the **owed column** of
`--emit port-blockers --tree` — *89 file(s) scanned, 66 declared no-port, 0 temporarily held,
23 owed* — where this file reads `owed lines=101`.

**This cut discharges guard-kit's owed column outright**, which is worth stating because it
is the only one of this iteration's four that does. `lib/guard.sh` (1246) and
`templates/bash-guard.sh` (21) both carry `# no-port:` on stated grounds this amendment does
not reopen — the sole-resolver ground and the extension-point ground, both stated at
§The guard framework (`lib/guard.sh`). After this cut, guard-kit has no owed file, and
§Testing gains the counterpart sentence.

## What changes

### (1) The cut is the one owed file declaring §Testing, and it empties the kit's owed column

`bin/run-guard-tests.sh` reads `owed lines=101` and is the only owed file whose `# spec:`
pointer binds guard-kit/SPEC.md `## Testing` {mechanical}. Its reach and its section bound
coincide, so no second section is rewritten by construction — with the one exception every cut
moves, gate-sdk/SPEC.md §The port disposition's owed-corpus prose, and the roster sentence in
§The non-gate arm that delta 6 adds it to.

### (2) The arm is `--run-guard-tests`, an `Arm::Run`, and it declares one knob

The member ports as a **bridged-arm table** row —
`("--run-guard-tests", Arm::Run(run_guard_tests::run), run_guard_tests::KNOBS)` in
`native/src/emit/mod.rs`'s `BRIDGED_ARMS`, with the body in
`native/src/emit/run_guard_tests.rs` {design-bearing}.

**`Arm::Run` on the variant's stated test**: the contract is a three-valued exit — 0 clean,
1 one or more case mismatches, 2 a missing input, an absent `jq`, or a table that parsed no
case at all. The verdict, not the report, is what the evidence-kit suite reads.

**The declared roster is `GATE_KIT_ROOTS_HERE` and nothing else, and the omissions are the
design content of this delta.** The arm needs one thing from configuration: where guard-kit
is vendored, so it can find `templates/bash-guard.sh`, `lib/guard.sh` and the two tables.
That is gate-sdk's knob and it crosses the bridge, which is what makes this a table member
rather than a hardcoded top-level flag (§The non-gate arm, *the family choice is forced for
any tool that needs configuration at all*). It declares **no** `GUARD_KIT_` knob, and both
omissions are deliberate:

- **`GUARD_KIT_LOG` is not declared because the arm overrides it.** guard-kit/SPEC.md
  §Testing rules that any ad-hoc invocation of a consumer's guard script must point
  `GUARD_KIT_LOG` at a scratch path, "because the logger's default is the *live* friction log
  and a throwaway probe otherwise files its synthetic commands as real friction" — measured,
  one such probe once contributed 30 of 237 ranked prompting calls at a single close.
  Declaring the knob would resolve the consumer's live log into the arm and invite exactly
  that. The arm sets the child's `GUARD_KIT_LOG` to a path inside its own sandbox, as the
  shell form does at `bin/run-guard-tests.sh:67`.
- **The guard's own knobs are not declared because the child resolves them.** The spawned
  `bash-guard.sh` sources `lib/guard.sh` through `GUARD_KIT_LIB`, and that library is the sole
  resolver for every `GUARD_KIT_*` default. Declaring them here would resolve them a second
  time, in a second process, for a child that resolves them anyway — the second producer
  criterion 6 refuses, reached by the back door.

**The arm is unresolvable in a tree that does not vendor guard-kit**, which is a property it
shares with `--emit-scan-prompts` and not a defect: `GATE_KIT_ROOTS_HERE` naming no guard-kit
root is a refusal with a message, exit 2.

### (3) The subject stays shell and stays spawned; only the harness moves

This is the cut's central design ruling {design-bearing}.

**What is under test does not move, and the section already says why it cannot.** §Testing
rules that `bin/run-guard-tests.sh` "covers `bash-guard.sh` alone — the one member of this kit
that stays shell, because it is the extension point a consumer's own rules are written in".
The ported arm therefore keeps feeding each case through the **unchanged**
`templates/bash-guard.sh`, spawned as `bash <guard-kit>/templates/bash-guard.sh` with
`GUARD_KIT_LIB` and `GUARD_KIT_LOG` in its environment, its cwd set to the sandbox, and the
hook payload on stdin — the same four inputs `:66-67` supplies today. Nothing about the
guard's decision path is re-expressed in Rust, so the port creates no duplication at all,
which is criterion 6's *unless* clause satisfied in the absent form.

**What moves is the harness: the payload construction, the sandbox, the table parse, the
classification and the accounting.** Each keeps its specified behavior:

- **The payload.** `jq -nc --arg c "$cmd" '{tool_input:{command:$c}}'` and the
  `run_in_background` variant become `serde_json` construction in-crate. The crate already
  depends on `serde_json`, so this adds no dependency, and it removes the harness's own use
  of `jq` — which is a real dividend and a small one, stated honestly in delta 5.
- **The two substitutions.** `@ROOT@` → the sandbox root and `@NL@` → a newline, applied to
  the command cell before the payload is built, in that order, as `:63-64` does. §Testing
  fixes them, and the second is load-bearing: "without the second a heredoc case cannot be
  written at all, and the heredoc class would ship untested".
- **The table grammar.** Tab-separated; a row whose first field is empty-or-whitespace is
  skipped; a row whose first field begins `#` is skipped. `cases.tsv` is
  `<decision> <TAB> <command>`; `background-cases.tsv` is
  `<decision> <TAB> <run_in_background> <TAB> <command>`. The tables stay **on disk** and are
  not transcribed into Rust literals, on §Testing's own stated ground for the escalation
  table: "it is kit test data a reviewer reads, and a literal would trade that review for a
  recompile".
- **The classification.** The five-way `classify()` ladder in exit-code-then-substring order:
  exit 2 → `block`; any other non-zero → `exit<rc>`; `"updatedInput"` present → `rewrite`;
  `"additionalContext"` present → `advise`; `"permissionDecision":"allow"` present → `allow`;
  empty output → `fallthrough`; else `unknown`. The order is load-bearing and is transcribed,
  not re-derived: a rewrite payload may also carry an `additionalContext`, and the ladder is
  what makes the first match win.
- **The sandbox.** A `mktemp -d` tree, `git init -q`, the three-line `.gitignore`, a tracked
  `tracked.md` and an untracked `scratch.txt` (rule 22 splits on tracked versus not), a
  `.tmp/dead-producer.run` carrying a dead PID beside a `.tmp/notes.txt` that is not a record
  (so every mutating-git row asserts rule 14's decline arm rather than the vacuous absence of
  any record), and a `.claude/settings.json` carrying the three-entry allowlist. Each of
  those five is a case's precondition and the amendment enumerates them because a port that
  builds four of them turns real rows green for the wrong reason.

### (4) The cargo-test road is refused, and the ground is adopter reach

guard-kit already has a member whose shell driver was replaced by a crate test rather than by
an arm, and a session sizing this member will reach for it first {design-bearing}.
§Testing records it: the escalation advisory's table "is read by the crate test under that
member's own module", and `native/src/hook/escalation.rs:86-108` is that test, reading
`guard-tests/escalation-cases.tsv` off disk and asserting its row count.

**That road is refused here, on a ground that has nothing to do with taste.** A
`#[cfg(test)]` test runs only under `cargo test`, which in this repo is the `native_crate`
evidence suite and needs a Rust toolchain. The shipped **binary** needs none: gate-sdk/SPEC.md
§Consumer payload ships a prebuilt artifact per declared target precisely so an adopter with
no toolchain has a working battery. Moving this runner into a cargo test would therefore
narrow the set of adopters who can run guard-kit's decision table from *everyone with the
binary* to *everyone with cargo* — a reach narrowing taken inside a port, which is the shape
`native-gate-port-remaining-corpus`' ruling (1) refuses in its own domain and which nothing
here licenses.

**The discriminator, stated so the two roads stay distinguishable after the merge:** a crate
test is the right home when the member's **subject** is in-crate, which is what the
escalation guard's is; an arm is the right home when the subject is a spawned shell surface
the payload ships, which is what `bash-guard.sh` is. §Testing's existing sentence already
draws that line for the escalation lane, and this delta extends it to the road, not just to
the member.

### (5) `jq` stays a precondition, re-expressed rather than dropped

The shell form refuses at `:14` when `jq` is absent, before it builds a single payload
{design-bearing}. The port keeps that refusal as `proc::on_path("jq")` with the same message
and the same exit 2, and does **not** read the harness's own move off `jq` as licence to drop
the check.

**The ground is that the subject still spawns it.** `lib/guard.sh:269` runs
`jq -r '.permissions.allow[]?'` against `GUARD_KIT_SETTINGS` to read the allowlist, and the
sandbox ships a `settings.json` precisely so the allowlist rules are exercised. With `jq`
absent, that read yields nothing, the allowlist rules stop firing, and the affected rows fail
as verdict mismatches — a red that names the wrong cause. Trading a clear
`jq not found on PATH` exit 2 for a scatter of mismatches is not a dividend.

**What the port genuinely removes is one process per case**, the harness's own
`jq -nc` payload build, times the row count of both tables. Stated as what it is rather than
as "the port drops the `jq` dependency", which would be false.

### (6) §The non-gate arm's roster gains the member, undated

The class roster gains `--run-guard-tests` with its owning section named and **no
landing-date cohort label** {mechanical}, for the reason the sibling cuts state: those labels
are the subject of this iteration's `kit-spec-provenance-seam-sweep`, which rules them
provenance and retires them, so a fourth would be landed and removed in one iteration.
The arm's spawned-program set — `bash`, `git`, `mktemp` and, through the subject, `jq` —
joins the prose paragraph that records those sets, since `--needs` answers about registry
members only and a bridged arm is not one.

### (7) The named consumer, re-pointed in the same commit

`scripts/evidence-config.sh:24` reads
`EVIDENCE_KIT_RUN_guard_tests='bash guard-kit/bin/run-guard-tests.sh'` and becomes
`'bash gate-sdk/bin/run-gates.sh --run-guard-tests'`, the form line 28 already uses for the
upgrade suite {mechanical}. That is the member's **only** named caller — verified this
session over the tracked tree rather than assumed — and the suite keeps its name, so
evidence-kit's roster, the validate spine and the baseline slice are all untouched.
The script is deleted in the same commit, after the two tables pass against the arm.

## Producers and consumers

**New interface: the `--run-guard-tests` bridged arm.**
*Producer* — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` row, dispatched in
`native/src/main.rs:485-498` before the registry lookup; its enabling configuration is the
bridged environment `gate_knob_env` builds for it, which `bin/run-gates.sh:32-37` resolves and
execs. Reachable with no front-end edit — the front-end passes every unrecognised leading
`--<token>` to `exec_arm` (`bin/run-gates.sh:45-70`).
*Consumer* — `scripts/evidence-config.sh`'s `EVIDENCE_KIT_RUN_guard_tests`, read by
`--run-validate` at the validate stage, which reads the arm's **exit status** through
`EVIDENCE_KIT_PARSER=exit-code`. A contributor running the arm by hand is the second.

**Existing interface whose producer set does not change: the hook payload on stdin.**
*Producer* — was `jq -nc` in the harness, becomes `serde_json` in the arm. *Consumer* —
`templates/bash-guard.sh`, unchanged, reading stdin exactly as it does today. The payload's
two fields both keep their named readers: `tool_input.command` is read by every generic rule
through `lib/guard.sh`'s command splitter, and `tool_input.run_in_background` is read by
rule 15's harness arm at the transition where the guard classifies a backgrounding call.
No field is added.

**Existing interface whose consumer set does not change: `GUARD_KIT_LIB` / `GUARD_KIT_LOG`.**
*Producer* — the arm, per case, as the shell form does. *Consumer* — `bash-guard.sh` and,
through it, `lib/guard.sh`. Both keep their cwd-relative resolution inside the sandbox.

**This delta set narrows no corpus.** The two tables are unchanged in extent and content, the
sandbox's five preconditions are reproduced item for item, and the classification ladder is
transcribed rather than re-derived. Point 5 of the causal-completeness check therefore does
not bind — but the one reader whose verdict is **not** monotone is named anyway, because it
is what this cut must be run against: §Testing rules that the decision table "fails on a
verdict mismatch **in either direction** rather than on a violation count", so a port is
cleared by the table running green over the unchanged rows and by nothing else. No row's
expected column is edited by this cut, which is the only way that clearance is available.

## Existing sections updated

- **guard-kit/SPEC.md §Testing** — the runner's mechanism restated for the arm: the spawned
  subject, the in-crate payload, the sandbox's five preconditions, the classification ladder,
  the `jq` precondition's honest scope, and the crate-test road's refusal with its
  discriminator (deltas 2, 3, 4, 5).
- **guard-kit/SPEC.md §The guard framework (`lib/guard.sh`)** — its `# no-port:` grounds are
  unaffected and the section gains the sentence saying so, since a reader meeting a ported
  runner beside an unported library will ask (delta 3).
- **gate-sdk/SPEC.md §The non-gate arm** — the class roster gains `--run-guard-tests`,
  undated, and the spawned-program prose gains this member's set (delta 6).
- **gate-sdk/SPEC.md §The port disposition** — the owed-corpus prose every cut moves;
  guard-kit's owed column is empty after this cut and the section says so (delta 1).
- **`scripts/evidence-config.sh`** — `EVIDENCE_KIT_RUN_guard_tests` re-pointed, and the
  file's own `# no-port:` header, which names the runner commands as this repo's test
  topology (delta 7).
- **README.md §This repo, governed** — the per-kit runner roster line, wherever it names this
  script (delta 7).
- <!-- update-target-exempt: a generated projection with its own freshness gate and regen command, rostered in docs/site-architecture.md §Generated projections and their freshness gates; it stales on the line-count change alone and no delta owns its content --> `docs/footprint.md` and `docs/value.md`'s rollup block.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable
      producer and a named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section (not appended); the merged spec reads as one coherent document a
      reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec, config and doc for
      `run-guard-tests.sh`; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
