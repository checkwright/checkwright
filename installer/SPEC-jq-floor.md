# SPEC amendment: jq-floor

**The filed premise is corrected before anything is designed on it, because the correction changes
what the fix is.** The queue entry states that on a machine without `jq` the installer *"reads an
empty version, an empty commit, and an empty prior-file and artifact-lock set — and proceeds"*, and
calls the failure direction *fails open*, of the `install-claim-contract` silent-revert class.
Probed at this stage against the source with `jq` masked off `PATH`, that is **not what happens**:

- `installer/lib/init.sh:54` — the version substitution exits **127** and `VERSION` is empty, as
  filed.
- `installer/lib/init.sh:56` — `[[ -n "$VERSION" ]] || die "this package carries no version stamp"`
  **fires and exits 2**. `init.sh` runs under `set -uo pipefail` with no `-e`, so the exit is this
  guard's, not the shell's.
- `installer/lib/init.sh:76`, `:77`, `:78` — the prior-file and artifact-lock reads the entry names
  as reading empty and proceeding are **never reached**.
- The second door closes independently: `installer/lib/common/lock.sh:14`'s `lock_schema_ok` ends
  its substitution with an explicit `|| return 1`, so `jq`'s 127 makes it false and
  `installer/lib/init.sh:64` dies with *"carries a schema this build does not know"*. The same
  guard sits ahead of every `jq` read in `installer/lib/diff.sh` and `installer/lib/uninstall.sh`.

**So the installer does not fail open. It fails closed with a misdiagnosis** — it refuses, and it
blames the package and the manifest for a defect whose cause is a program that is not installed.
The operator-ruled deliverable is unchanged and is exactly right as scoped: **a refusal naming the
missing program**. What is falsified is the entry's severity ground, and that correction is
escalated rather than absorbed, because the admissibility ruling was taken on it.

**Why a misdiagnosis is still worth a unit, on the corrected facts.** The cost is no longer a
corrupt install; it is a diagnostic dead end landing on precisely the population that cannot get
out of it. An adopter told *this package carries no version stamp* has been told the artifact is
broken. The true remedy — install `jq` — is not in the message, not in the `help:` line, and not
in `installer/README.md`'s Requirements section, which names `curl`, `tar`, `sha256sum` and Node
and says nothing about `jq` for the installer's own verbs. The one place that already gets this
right is `installer/lib/doctor.sh:80` — *"Found %s, but jq is absent, so it cannot be read."* — so
the correct idiom is already in this tree and the unit is generalizing it, not inventing it.

**What this unit does not do.** `install-step-relocation` owns moving these steps behind the
binary's invoke, and this amendment does not discharge, pre-empt or narrow it — a refusal naming
the missing program is the cheap floor that is correct whether or not the relocation ever happens,
and it is deleted by that unit if the relocation retires the dependency. Two sibling entries stay
distinct and untouched: `guard-advise-jq-dependency` is guard-kit's `guard_advise` losing an
*advisory*, and `stage-economics-smoke-jq-arm-dormant` is a smoke arm elsewhere that never runs.

## What changes

### 1. The JSON dependency is declared once, in the module that already owns the JSON

**`installer/lib/common/lock.sh` gains a preflight that refuses when `jq` is absent, and it is the
single declaration of the dependency** — design-bearing, because the placement is what stops four
verbs growing four copies **[design-bearing]**.

The check is `command -v jq`, the same probe `doctor.sh:79` already uses, exposed as one function
in `lock.sh`. Every verb that reads JSON calls it **at the top, before its first read** — not
lazily inside an accessor, because `init.sh` reads `package.json` at line 54, before it reaches any
of `lock.sh`'s accessors, and a lazy check would let the first and most misleading refusal through.

**Why `lock.sh` and not a new `common/` file.** Its own header declares it the sourceable owner of
the manifest wire format, existing *"so the verb that writes the manifest and the verbs that read
it share one definition instead of a copy each"* — which is precisely the shape of this fact. A new
file for one function would add a governed name and separate nothing. The boundary is worth stating
because `package.json` is not `lock.sh`'s schema: what `lock.sh` owns here is **the declaration
that this installer reads JSON with `jq`**, which is true of both files it is read from, not the
shape of either.

### 2. The refusal names the program and the remedy, on the exit code the README already defines

**One message shape, and the exit code is derived from the published contract rather than chosen**
— design-bearing, because an exit code is a wire contract adopters and scripts read
**[design-bearing]**.

The refusal takes the verbs' existing `die` idiom unchanged: the message to stderr prefixed
`checkwright <verb>: `, a `  help: ` line carrying the remedy, and an exit code. It names `jq`, says
what could not be done without it, and the `help:` line says to install it.

The code is **2**, and it is read off `installer/README.md`'s own §doctor contract rather than
picked: *"A third exit status, `2`, means the question could not be answered rather than that the
answer was bad."* A missing program is the definition of a question that could not be answered — it
is not a judgment about the tree, the package or the manifest, which is what `1` is reserved for.
This also lands the refusal on the same code the misdiagnosing refusals already exit with, so no
caller's exit-code handling changes; what changes is that the message is now true.

**The verb-prefix convention is inherited, not re-decided.** §update already records that a refusal
not owned by the invoking verb surfaces prefixed with the verb that produced the line, *"because
that is literally which verb produced the line"*. A refusal raised from `lock.sh` under `update`
therefore reads `checkwright init:` exactly as today's do.

### 3. The three misdiagnosing refusals become unreachable, and that is the deliverable

**`init.sh:56`, `init.sh:64` and the `lock_schema_ok` guards in `diff.sh` and `uninstall.sh` keep
their text and stop being how a `jq`-less machine finds out** — design-bearing, because it is a
judgment about which message a reader should meet **[design-bearing]**.

None of those three refusals is wrong in its own terms. *This package carries no version stamp* is
the correct message for a package that genuinely has none, and *carries a schema this build does not
know* is correct for a manifest from another release. Each is only wrong as **the message a
`jq`-less machine receives**, because on that machine each is reached by a path that has nothing to
do with what it says. §1's preflight runs first, so each keeps its text and recovers its accuracy:
after this unit, reaching one of them means the condition it names is actually true.

**This is the whole substance of the fix, and it is worth naming as a deliverable** rather than
being read as a side effect — the unit is not adding a check to a program that had none, it is
putting an accurate refusal in front of three inaccurate ones.

### 4. `COMMIT`'s missing guard is named, and the preflight is what makes it decidable

**`installer/lib/init.sh:55` reads `.checkwright.commit` and no guard follows it** —
design-bearing, because it is a real residual the probe surfaced and leaving it unstated hides it
**[design-bearing]**.

Today an empty `COMMIT` is masked by ordering alone: line 56 kills the run first, so the empty value
never reaches the manifest. That is fragile — the masking is an accident of statement order, and
this unit reorders the neighbourhood.

The preflight is what makes the field decidable rather than ambiguous: after §1, an empty `COMMIT`
can no longer mean *`jq` is missing*, so it means exactly *this package carries no commit stamp*,
and the existing manifest rule settles it without a new ruling — `lock_emit` already specifies that
an identity field is *"present exactly when the caller supplied it — never a null or an empty
placeholder standing in for an omission"*. So an absent commit is **omitted** from the manifest, not
written empty. Whether the current code satisfies that is a probe the build session runs; this
delta's ruling is only that the omission rule governs and no new one is needed.

### 5. A `jq`-masked smoke arm, because nothing in this tree runs `jq`-free today

**`installer/consumer-smoke/run-smoke.sh` gains a masked arm on the pattern its two existing masked
arms already establish** — design-bearing, because the mask has to prove it took and the assertion
has to be the message **[design-bearing]**.

The gap is total and is the reason this defect survived to be filed twice: `run-smoke.sh`'s own
preflight **requires `jq`** before any arm runs, so every arm — including the node/npm-masked
download arm and the cargo/rustc-free toolchain arm — executes with `jq` present. No test anywhere
in the tree exercises a `jq`-less install, and `check-installer-no-deps` does not reach it (its
subject is the package's declared dependencies and lifecycle scripts, a different question).

The new arm follows the two that exist: mask `jq` off `PATH`, **prove the mask took** by resolving
`command -v jq` to the shim exactly as the node and cargo arms do, then assert that the verb
**refuses**, that its message **names `jq`**, and that it exits **2**. Asserting the message and not
only the exit status is the point — the exit status was already 2 before this unit, so an arm that
checked only the code would have passed against the very defect being fixed.

This is enforcement-first: the fix and the check that catches its regression land in one unit. It is
also distinct from `stage-economics-smoke-jq-arm-dormant`, whose subject is a dormant arm in another
suite; this arm is new, is in the installer's own suite, and runs.

### 6. `installer/README.md` declares the dependency it has always had

**The Requirements section names `jq` for the installer's own verbs** — mechanical
**[mechanical]**. Today §Requirements names `curl`, `tar`, `sha256sum` and Node for the delivery
paths and points at the install page for the battery's toolchain; §The manifest mentions `jq` only
in passing, as *"It is JSON, read with `jq`"*. The result is that the installer's own dependency is
inferable and nowhere declared. The declaration is scoped precisely, because the existing
`docs/install.md` toolchain block already names `jq` for the **vendored battery's gates** and the
two are different claims about different programs' users — the verbs `init`, `diff`, `doctor` and
`uninstall` read JSON, and that is what the new line says. `check-install-claim` and
`check-payload-claim` are the gates over claims on these surfaces and re-fire on the edit.

## Producers and consumers

**This amendment introduces one shared preflight function, one refusal, one smoke arm and one
documentation claim. It introduces no new knob, no new tag, no new file, no new exit code, no new
message grammar and no change to any wire format.**

**The `jq` preflight (§1).** *Producer:* `installer/lib/common/lock.sh`, defining it; the four verbs
calling it at their top. Its **enabling configuration is the existing `source` line** each verb
already carries for `lock.sh` — nothing new is wired, so the producer is reachable in every shipped
package on the ordinary path, which is the property that matters for a payload whose failure is
being fixed. *Consumer:* the adopter's shell, at the verb's first statement, reading exit 2 and the
stderr message. That is the only transition, and it is the one the whole unit exists to reach.

**The refusal message (§2).** *Producer:* the verb's local `die`, unchanged. *Consumers:* the
adopter reading stderr, at the moment the verb refuses; and **the new smoke arm** (§5), reading the
same two lines at every validate run of the `installer_smoke` suite. The second consumer is what
makes the message a checked contract rather than prose — and it is why §5 asserts the message text
rather than the exit status alone.

**Every field has a named reader, and there are two.** The refusal's **program name** is read by the
adopter deciding what to install and by the smoke arm's assertion; the **`help:` remedy line** is
read by the adopter alone, at the same moment. No third field, no record written, no state.

**The documentation claim (§6).** *Producer:* this amendment, landing in `installer/README.md`.
*Consumers:* the adopter reading Requirements before installing; and `check-install-claim` /
`check-payload-claim`, at every battery run over that surface.

**Existing integration prose describing the prior flow is updated, not left to drift** — see below.
The one flow that genuinely changes is the `jq`-less install: it stops being *refuse, blaming the
artifact* and becomes *refuse, naming the program*.

**A corpus is narrowed and its readers' red conditions are named.** §3 makes three refusal paths
unreachable on `jq`-less machines, which narrows the set of inputs that reach them, and a narrowing
is not clearable by inspection unless each reader's verdict is monotone. The readers whose red
condition is a **count or a find-none**, and which therefore have to be re-run: the smoke suite's
existing assertions over `init`/`diff`/`uninstall` refusals, several of which assert *that a
specific message appeared* and go red on finding none — reordering a guard ahead of them is exactly
the shape that flips such an assertion; `check-install-claim` and `check-payload-claim`, whose
assertions are over declaration counts on the touched surfaces; and `check-docs-cmd`, whose subject
is that a documented command resolves. The monotone reader is the exit-status assertion, unchanged
at 2 in both the old and the new path — which is precisely why §5 does not rely on it.

## Existing sections updated

- **installer/README.md §Requirements** — §6. The installer's own verbs' dependency on `jq`,
  declared and scoped to those verbs, distinct from the battery's toolchain claim on the install
  page.
- **installer/README.md §The manifest** — §1 and §4. Its existing *"It is JSON, read with `jq`"*
  sentence becomes the anchor for the dependency's consequence: the verbs refuse naming `jq` rather
  than reading empty values, and the identity-field omission rule is what governs an absent commit.
- **installer/README.md §init** — §2 and §3. The preconditions list currently states the three
  refusals that all run before any file is written; it gains the `jq` preflight as the one that runs
  before them, and records that the three keep their text and recover their accuracy.
- **installer/README.md §doctor** — §2's exit-code grounding. The `2` = *the question could not be
  answered* contract is the section that owns it, and it gains the missing-program case as a named
  instance rather than a new code.
- **installer/README.md §update** — §2's prefix convention, unchanged, cited to record that a
  refusal raised from `lock.sh` inherits it.
- **`installer/lib/common/lock.sh`** — §1's function and its `spec:` binding.
- **`installer/lib/init.sh`, `installer/lib/diff.sh`, `installer/lib/uninstall.sh`,
  `installer/lib/doctor.sh`** — §1's call at the top of each. `doctor.sh` keeps its own inline
  `command -v jq` branch, which is not a duplicate: `doctor`'s job is to *report* a toolchain rather
  than refuse on it, so it needs the probe as a reportable fact, and §1's preflight applies to its
  manifest-reading path alone.
- **`installer/consumer-smoke/run-smoke.sh`** — §5's arm, and the preflight's unconditional `jq`
  requirement adjusted so the masked arm can run.
- **TASK-QUEUE.md** — `installer-jq-silent-degradation` moves to `## Done`, dropping its `[spec:]`
  tag; the deliverable is discharged whole rather than incremented. Its body's falsified
  fail-open ground is corrected at the promotion that pairs this amendment, not left standing.

## Definition of Done

- [ ] **Causal completeness** — the preflight has a named, reachable producer and a named consumer;
      both of the refusal's fields have a named reader at a named transition.
- [ ] **The corrected premise is what landed** — no surface asserts that the installer proceeds, or
      completes, or writes anything on a `jq`-less machine. The defect fixed is a misdiagnosis, and
      a merged spec claiming a silent-revert repair would be false.
- [ ] **The masked arm asserts the message, not only the code** — the exit status was already 2
      before this unit, so an arm checking the status alone passes against the unfixed defect. The
      mask proves it took, on the pattern the node and cargo arms already establish.
- [ ] **The narrowing's non-monotone readers were re-run** — every smoke assertion that reds on
      finding no specific message, and both claim gates, re-run green after the guard reordering.
- [ ] **The three refusals keep their text** — `init.sh`'s two and `lock_schema_ok`'s are unchanged
      in wording; the unit's effect on them is reachability, and rewriting them would lose the
      message the genuinely-broken-package case needs.
- [ ] **`COMMIT` is decided, not left ambiguous** — an absent commit is omitted from the manifest
      per the existing identity-field rule, and the probe confirming the code does that was run
      rather than reasoned.
- [ ] **No relocation is pre-empted** — nothing in the landed text moves, blocks, or narrows
      `install-step-relocation`, and nothing claims the dependency is retired.
- [ ] **Merged with no information lost** — each addition integrated into its proper section (not
      appended); the merged README reads as one coherent document a reader who never saw the
      amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls installer/SPEC-*.md`). Discharged at the **iteration**, not at this commit, where sibling
      amendments are in flight.
- [ ] **Removals propagated** — grepped every surface for the claim that the installer reads empty
      values and proceeds; nothing dangles, including the queue entry's own body.
- [ ] **Gaps filed** — the absent `jq`-free coverage for any *other* surface the probe touched, and
      any cross-component gap found during the work, filed as debt rather than absorbed.
