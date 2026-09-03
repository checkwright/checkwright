# SPEC amendment: session-id-cut

The port disposition of **lifecycle-kit's one member behind §bin/session-id.sh**
— `bin/session-id.sh` (75 lines) — off the shell substrate as a bridged-arm-table
member of the gate binary. Cut A of `session-id-and-env-probe-cuts`, under the
port-only run (TRAJECTORY.md §PRIORITY DIRECTIVE). The iteration's two-cut unit
set was ruled by the **lead on 2026-09-03 on its own authority**; the third cut
scope proposed, `doctrine-kit/SPEC.md` §install-doctrine, is sequenced behind the
installer's behind-invoke relocation by an operator ruling recorded in that
section's own text and is deliberately not pulled in here.

**Measured at this HEAD rather than carried from the survey**: the port oracle's
`--tree` arm reads 109 files scanned, 64 declared `no-port`, 0 temporarily held,
**45 owed**, `lifecycle-kit/bin/session-id.sh owed lines=75` among them. The
scope-stage survey record's own witness passed at this HEAD — an empty
`git diff --name-only <rev> HEAD -- '*.sh'` and an identical oracle trailer — so
its finding (i), that this member "carries an explicit dated operator ruling …
and wants only a host", is cited rather than re-bought
(lifecycle-kit/SPEC.md §The survey record).

**The member is not a gate** — §bin/session-id.sh says so in its own words, "Not
a gate — a `bin/` helper" — so no `gates.list` row, no `.gate` descriptor, no
`good/`+`bad/` fixture pair, and no binary-less residual roster moves.

## What changes

### (1) The cut is a singleton behind one stated contract and takes the whole section

`lifecycle-kit/bin/session-id.sh` declares
`lifecycle-kit/SPEC.md §bin/session-id.sh` in its own line-2 `# spec:` header,
and **no other tracked non-test `.sh` declares that section** {design-bearing}.
The declaring set behind this section is one file, so the cut takes all of it and
the 2026-09-03 *a section is a cut's outer bound, never its minimum* ruling has
no sequenced member to leave behind — this is the section shape that ruling
contrasts with, not the one it was written for.

**The host is the composer entry and the ground is the rule's default branch.**
gate-sdk/SPEC.md §Porting a gate to the binary substrate hosts a cut either on
`native-gate-port-remaining-corpus` or on an entry whose own text names the cut's
subject as its blocker; §bin/session-id.sh already records that a sweep of the
whole queue found **no entry naming this file at all**, and that record stands.
What that section also records — that the member was dropped from
`declaration-install-and-stage-helper-cuts` for want of a host and that the
absence of a blocker is what made the deferral invisible — is the thing this cut
discharges.

### (2) `--emit-session-id` is a bridged-arm table member with an **empty** declared knob roster

The member's contract is a **document** — one normalized id on stdout, exit 0, or
a diagnostic on stderr and exit 2 — so the `Arm` variant is `Arm::Emit`, whose
collapse of every error to 2 is exactly this member's existing behaviour rather
than a loss of one {design-bearing}. The spelling is `--emit-session-id`, reached
through the generic `bin/run-gates.sh --emit <name>` composer rather than a
front-end branch of its own.

**The declared roster is empty, and that is what makes the arm reachable rather
than an omission.** §The non-gate arm rules that table membership is what the
front-end's `--emit <name>` operand resolves through `gate_knob_env`, so a member
reading nothing still needs a row; `--emit-md-section` is the class's first such
member and this is its second. The roster **must** be empty here, and the reason
is mechanical rather than stylistic: `lib/stages.sh` defines neither
`LIFECYCLE_KIT_SESSION_ID` nor `LIFECYCLE_KIT_SESSIONS_DIR`, and a member
declaring a knob its owning kit's library does not define is the config bridge's
**undeclared-knob refusal** (gate-sdk/SPEC.md §lib/gate.sh) — it would fail-close
on every single invocation.

**Adding the two defaults to `lib/stages.sh` so the names could be declared was
weighed and refused, because it is a behaviour widening rather than a port.**
Probed rather than assumed: `bin/session-id.sh` sources no library at all, and
`lib/stages.sh` exports nothing, so a consumer setting `LIFECYCLE_KIT_SESSION_ID`
in their `LIFECYCLE_KIT_CONFIG_FILE` gets nothing today — with the config file
setting it to a sentinel and every environment source unset, the script falls
through to source 3 and exits 2 rather than printing the configured value.
Bridging the name would make that config file start working, which is a widening
a faithful port may not take on its own authority. The knob is documented on
§Layout and configuration as though it were bridged, and **that mismatch is filed
to the committed gap inbox rather than resolved here** — it outlives the port in
either direction.

### (3) The two environment sources reach the arm because the bridge **adds** to the environment rather than replacing it

Stated as a delta because it is the property the whole cut rests on and the one a
reader would most reasonably doubt {design-bearing}. Sources 2 and 3 of the
derivation order read `CLAUDE_CODE_SESSION_ID`, `CLAUDE_CODE_CHILD_SESSION`,
`CLAUDE_CONFIG_DIR` and `HOME` — harness and system variables, none of them a kit
knob and none of them declarable. Both dispatch paths compose
`env <resolved knobs> <binary> <arm>`: `gate_command` for a `.gate` member and
`exec_arm` for a front-end arm (gate-sdk/SPEC.md §lib/gate.sh, §run-gates). That
is an addition to the inherited process environment, never an `env -i`, so every
one of those four reaches the arm exactly as it reaches the script. The sessions
dir's computed default — `${CLAUDE_CONFIG_DIR:-$HOME/.claude}/projects/<cwd-slug>`,
with every non-alphanumeric character of the cwd mapped to `-` — moves in-crate
verbatim. §The non-gate arm's rule that *a default the deleted shell driver held
inline moves into the owning kit's library in the same cut* does not reach it:
that rule's stated ground is a **declared** knob resolving empty through the
bridge, and this arm declares none.

**The provenance seam is unchanged, and it is stated because the port moves
harness-specific names into the crate.** `CLAUDE_CODE_SESSION_ID`, its child flag
and the `<config-home>/projects/<slug>` layout are one harness's vocabulary, and
§bin/session-id.sh already rules the seam for them: source 1 is the
**harness-neutral** consumer override and source 2 "the shipped default source,
harness-specific by nature", with `LIFECYCLE_KIT_SESSIONS_DIR` the override for
the layout. That is kit **mechanism** with a consumer escape hatch, not private
rule content, so it travels in-crate as a literal exactly as it sits in the script
today — and the escape hatch is what keeps it mechanism. Nothing here becomes
consumer config that was not already, and nothing private lands.

### (4) The cwd is an **input** to the derivation, so the production caller reaches the binary directly rather than through the front-end

The one place a faithful port is not free, and it is invisible until probed
{design-bearing}. Source 3's default sessions dir is
`<config-home>/projects/<cwd-slug>`, and `bin/session-id.sh` computes the slug
from `pwd` — so **the process cwd is an argument to this rule**, unlike every
other ported member, whose inputs are config or argv.
`gate-sdk/bin/run-gates.sh:13` does `cd "$(git rev-parse --show-toplevel)"` before
dispatch, which §run-gates states as a rule for every entry point. Routing through
the front-end therefore **changes the derivation** whenever a caller's cwd is not
the git toplevel, and turns a non-repository cwd from *working* into *exit 2*
before the arm is even reached.

**So `enter-stage.sh` invokes the binary directly**, through the
`gate_native_bin` accessor it already has in scope from `lib/gate.sh`. §The non-gate
arm sanctions this in as many words — "what the class forbids is a *second* entry
point into the **emission path**, not a caller" — and the arm's empty roster
(delta 2) is what makes it free: there is no bridged environment for a front-end
to supply, so the front-end buys nothing here and costs the cwd. **Recorded as the
faithful-port road rather than as a preference**: pinning the sessions dir to the
toplevel is *arguably* a repair, since this harness keys `projects/<slug>` on the
project root, and taking an arguable repair inside a port is the move this track
has refused three times.

**The front-end route stays available and is documented with its caveat**, because
`templates/lead.md` and the smoke both reach the arm that way and both run with
cwd at the repo root, where the two routes agree.

**One further spelling is load-bearing and is stated so a Rust rewrite does not
silently change it.** bash's `pwd` prints the **logical** path — the `PWD` the
shell carries — where `std::env::current_dir()` returns the **physical** one, so a
session under a symlinked checkout would slug differently. The arm reads `PWD`
where it is set and falls back to `current_dir()`, which is `pwd`'s own semantics.

### (5) The sole production caller re-points in one line

`lifecycle-kit/bin/enter-stage.sh:290` runs `bash "$KIT/bin/session-id.sh"`
{mechanical}. It becomes the front-end arm. The file already resolves
`SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"` and sources `$SDK/lib/gate.sh` at load,
declaring that dependency in its own header, so nothing new is introduced and no
second resolver is written. The `if ! id="$(…)"` guard and its
"could not read the session id (see above) — nothing written" diagnostic are
preserved verbatim: the arm's stderr is the *see above*.

### (6) `templates/lead.md` invokes the helper directly and six stage templates name it as the id's source

§bin/session-id.sh already records this as one of two facts a session taking this
cut should not re-derive, and it is the delta with the widest touch
{mechanical}. `lifecycle-kit/templates/lead.md` carries a live invocation —
`echo "lead $(bash lifecycle-kit/bin/session-id.sh)"` in the session-role marker
step, plus a sentence naming the helper and one pointing at the mis-pick limit —
and each of `templates/stages/{scope,align,build,validate,close,spec}.md` carries
prose reading "reading `<session-id>` from `bin/session-id.sh` (the newest
transcript — never hand-picked)". Every one is re-pointed at the arm; the
mis-pick limit and the never-hand-picked rule are unchanged, only the name of
what prints the id.

### (7) The smoke's five assertions re-point to the front-end and survive case for case

`lifecycle-kit/smoke/install.sh` drives the helper directly through
`SID="$SMOKE_KIT_ROOT/bin/session-id.sh"` and a `sid_run` helper that unsets the
four inputs before setting the ones a case needs {mechanical}. The five cases are
the derivation order's own axes — source 1 with the `agent-` strip, source 2 with
`CLAUDE_CODE_CHILD_SESSION` unset, the child-narrowed `subagents/` scan excluding
a *newer* top-level lead transcript, the spurious-flag fall-back to the env uuid,
and the widened glob with no env id at all. `SID` becomes
`bash "$SDK/bin/run-gates.sh" --emit session-id`, the shape the same file already
uses one screen above for `--install-lifecycle`, and `sid_run`'s
`env -u … "$@" bash …` prefix keeps working unchanged by delta (3). The harness
itself stays `# no-port:`; nothing about the five assertions weakens, and the
build proves that by running them rather than reasoning it.

### (8) Exactly one permission grant is deleted and none is added — the count probed, as the carve-out requires

`.claude/settings.json:31` carries `Bash(bash lifecycle-kit/bin/session-id.sh)`
and it is the only line in the file naming this path {design-bearing}. It is
removed **in the same commit as the delete**, which is the window the 2026-08-29
settings-grant carve-out on `native-gate-port-remaining-corpus` exists to close;
§bin/session-id.sh already names this member as the one exercise of that carve-out
in its iteration's candidate set, and the count it demands be probed rather than
assumed is **one**.

**No grant is added, and that is what keeps this inside the carve-out rather than
against the 2026-08-22 bar.** The post-port invocation is
`bash gate-sdk/bin/run-gates.sh --emit session-id`, already covered by the
committed `Bash(bash gate-sdk/bin/run-gates.sh)` and
`Bash(bash gate-sdk/bin/run-gates.sh *)` entries. Stated explicitly because the
scope-stage survey recorded a sibling candidate — a guard-kit §scratch-run cut —
whose port would need a grant *added*, which the carve-out does not cover and
which is therefore operator-class; this cut is on the other side of that line and
the difference is one probe.

### (9) Criterion 6 is discharged by construction: the cut creates no twin

The member's whole derivation is self-contained — environment reads, a directory
walk and an mtime comparison — and it sources no kit library, so there is no
corpus derivation to duplicate and no shared primitive whose two copies would
need holding together {design-bearing}. Deletion is the disposition rather than
duplication because the shell caller set **empties**: `enter-stage.sh` and
`templates/lead.md` are the only invokers and both re-point in this cut, so
gate-sdk/SPEC.md §The port-candidate criteria's *whether the shell caller set
empties* test resolves to the deleting road with no standing parity oracle owed.

### (10) The section's port-disposition paragraph and its two facts become the cut record

§bin/session-id.sh today ends in a paragraph headed *The port disposition: owed,
unblocked and takeable, and deferred once for want of a host*, followed by two
facts a session taking the cut should not re-derive {design-bearing}. Both were
written for a session that had not yet taken it. They are **replaced by the cut
record**, as every closed cut's record is, in the contract section the cut
selected: what the arm is, what its empty roster means, that the grant count was
one, and that the deferral for want of a host is discharged. **The
`### bin/session-id.sh` heading is not renamed** — nine in-SPEC citations across
lifecycle-kit, delegation-kit, drift-kit and context-kit resolve against it, and
renaming strands every one, which is the disposition §upgrade-smoke and
§lib/declaration.sh each took for the same reason. **Seven citation sites**
resolve against it, probed rather than recalled: `delegation-kit/SPEC.md:566`,
`drift-kit/SPEC.md:630`, `lifecycle-kit/SPEC.md:212` and `:520`,
`lifecycle-kit/templates/lead.md:31`, `TASK-QUEUE.md:5219`, and
`lifecycle-kit/smoke/install.sh:340`'s `# spec:` header — each doubled in the
docs mirror.

### (11) The projection fan-out has **two independent triggers**, and the second is the one a reader would miss

The obvious trigger is delta (6)'s seven `templates/*.md` edits, which is one of
the three §bin/footprint's load-triggered tier measures {mechanical} — that tier
is the skill and template **markdown** a kit ships under `templates/`, so the line
counts move, `docs/footprint.md` goes stale, and `docs/value.md` follows because
it takes footprint's per-kit figures as an input.

**The second trigger is the deletion itself, and it fires a projection nothing
about this cut points at.** `scripts/measured-claims.sh` derives a
`tree-shell-owed` key from the port oracle's `--tree` trailer, and the **generated
pre-commit hook bakes that value verbatim** — `scripts/git-hooks/pre-commit`
carries `GATE_SDK_KNOB_CANON_KIT_MEASURED_VALUES=…\t45` today. Deleting one owed
`.sh` moves it to 44 and stales **both generated hooks and `docs/check-graph.html`
together**, which `check-graph` byte-holds. So the regeneration set is the docs
mirror, footprint, the value rollup, the two hooks and the graph page — plus the
binary itself, whose `--source-stamp` `check-gate-binary-fresh` holds against the
tracked crate source. `docs/site-architecture.md` §Generated projections is where
the fan-out is read off rather than restated here, and it states this trigger.

**Two ordering hazards ride with it**, both because the hook and the binary derive
through `git ls-files`: **stage the deletion first and regenerate second**, and
build the binary after the crate source is staged. A regeneration taken before the
deletion is staged bakes the old count and reds on the next run.

### (12) Two questions this cut deliberately does not answer, each recorded where it lives

Stated as a delta so a later reader does not read the silence as a discharge
{design-bearing}.

- **The `session8` divergence.** The scope survey's finding (iii) records that
  `drift-kit/bin/overhead-meter.sh` and this member derive the eight-character id
  **differently** — the meter takes the basename's first eight characters with no
  `agent-` strip and scans one flat tier — and that both owning SPEC sections are
  silent on it. The live entry `overhead-meter-measures-the-lead` owns exactly
  that seam question and is `[design-pending]`; the port-only run answers nothing
  from the deferred pool, so this cut touches neither the meter nor the entry. One
  thing about that entry does change and is worth its sentence: its cheapest
  option, "depend on lifecycle-kit's tool across a kit boundary that no kit
  dependency currently spans", becomes *call another arm of the same binary* once
  this cut lands. That reshapes the option; it does not rule on it.
- **`LIFECYCLE_KIT_SESSION_ID`'s knob-versus-environment status**, from delta (2),
  filed to the committed gap inbox at this stage with its probe.

## Producers and consumers

The cut introduces **no new state, event or interface**: it moves one existing
interface — a program printing one line on stdout — from one substrate to
another, and the derivation order, the normalization and both exit statuses are
unchanged. So the checklist is run over the **relocation** rather than over a new
message, and over the deletion it performs.

- **Producer.** `--emit-session-id`, dispatched by `main` ahead of the registry
  lookup and absent from `--list`, reached through `bin/run-gates.sh --emit
  session-id`. Its enabling configuration is *none* — the empty roster of delta
  (2) — so there is no config a deployed tree must set for the producer to be
  reachable, which is the strongest form of §The causal-completeness check's
  point 1 and the reason that point needs no further argument here.
- **Consumers, both named and both re-pointed in this cut.**
  `lifecycle-kit/bin/enter-stage.sh` reads the id on stdout and substitutes it
  into the stamp's third field (deltas 4 and 5); a **session** running `/lead`'s first
  step reads it through `templates/lead.md`'s command substitution and writes it
  into the session-role marker (delta 6). §The non-gate arm's *named caller*
  property is satisfied twice, and the second of those is the class's already
  precedented "a session reaching a mode through the front-end counts exactly as
  a stage step does".
- **No new field.** The output is the same single normalized token it is today,
  read at the same two transitions.

**Every reader's RED condition, because this delta narrows a corpus.**
canon-kit/SPEC.md §The causal-completeness check point 5 binds: a reader is
clearable by inspection only where its verdict is monotone in the violation set,
and three shapes are not — reds on finding none, asserts an exact count, holds a
minimum or a coverage floor. Enumerated rather than described:

- `check-stage-evidence` — reds on a malformed stamp, a stale head, or two
  distinct stages of one iteration sharing a session id. Monotone in stamps, and
  the stamp's producer is unchanged; deleting the script cannot add a stamp.
- The port oracle's `--tree` arm and `check-gate-exemption-tasks` — the former
  reds on nothing (it reports) and the latter on a `# port-until:` slug with no
  live queue entry. Deleting an **owed** file removes a row and lowers the owed
  count from 45 to 44; neither reader has an exact-count or floor arm.
- `check-shellcheck` — reds on a lint finding in the scanned corpus. Monotone:
  removing a file removes findings.
- `check-comment-tier` — reds on a non-directive full-line comment. Monotone in
  the same direction; the crate module's `// spec:` headers are the ported
  comments and carry their bindings.
- **`check-docs-cmd` assertion A — reds *because* of this cut over the governed
  doc set, and its reach stops short of the templates.** A governed doc still
  fencing the deleted `lifecycle-kit/bin/session-id.sh` path is a finding, so it
  forces the kit README's roster line. `check-settings-paths` has the same shape
  over the committed allow-list and forces delta (8): a stranded grant naming a
  deleted path reddens. **Neither fires from the generated hook's staged-path
  trigger on a deletion** — that gate's own recorded limit — so both are caught by
  the whole-tree battery and not by the pre-commit; the build runs the battery,
  not just the hook.
- **Delta (6)'s seven template edits have NO gate backstop at all, and that is
  the sweep's most important negative.** Three independent reasons, each probed:
  `CANON_KIT_PROSE_SURFACE_GLOBS` is `*/templates/*.md`, a **one-level** glob that
  `templates/stages/*.md` never matches; the prose pass excludes **slot-bearing**
  templates by design, and `lead.md` carries slots; and even in corpus, assertion
  A's invocation test would miss `lead.md`'s line, whose first word is `echo`
  rather than a path or `bash`. §bin/session-id.sh already half-records this — "one
  of them **invokes it directly**" — and this is the other half: nothing will red
  if the sweep is skipped. The same holds for `TASK-QUEUE.md`'s two entries naming
  the path, `.workflow/survey-record.md`, and
  `drift-kit/bin/stage-economics.sh:45`'s comment attributing the normalization to
  this file, which `check-comment-tier` judges for tier and never for truth. **A
  manual authoring obligation stated as one**, because the alternative is a build
  session inferring a backstop that is not there.
- **`check-shim-restatement` reds on a shared ≥9-word n-gram between a
  `.claude/commands/*.md` shim and the dedup corpus — and it reds at exit 2 on an
  empty corpus, so it is non-monotone twice over.** This repo leaves
  `LIFECYCLE_KIT_SHIM_DEDUP_CORPUS` empty, so the corpus is derived as `CLAUDE.md`
  plus every kit's template markdown — **including all seven files delta (6)
  edits**. New prose describing the arm can collide with existing shim text, and
  no reading settles it: only a live run does.
- **`check-docs-cmd` assertion B is the ZERO-COUNT reader here, and it is one
  delta away from red.** Its rule is that every backticked kit-prefixed knob name
  in the governed doc set must occur in the kits' tracked **code**; its corpus is
  the kit roots minus `*.md` minus `*/gate-tests/*`, and **`native/` is not a kit
  root** — a kit root is a sibling of gate-sdk carrying `checks/` or `smoke/`, so
  moving these names into Rust does not satisfy it. `lifecycle-kit/SPEC.md`
  backticks both `LIFECYCLE_KIT_SESSION_ID` and `LIFECYCLE_KIT_SESSIONS_DIR`, and
  once `bin/session-id.sh` is deleted **`lifecycle-kit/smoke/install.sh` is the
  sole remaining holder of either name** in that corpus. It is clearable by
  inspection here and only because the smoke's *other* uses are independent of
  delta (6) — six occurrences at lines 110-113, 174-177, 219-222, 398-401 and
  460-463 belong to the stage-stamp cases, not the five session-id ones — so a
  rewrite that drops the five still leaves the names held. **Stated because the
  reasoning that would have skipped it is exactly the one point 5 names**: "a
  narrower corpus can only remove violations" is false, and here the narrowing
  would have added one had the smoke's session-id block been the only holder.
- **The non-monotone class, named and cleared by execution rather than by
  inspection.** `check-install-claim`'s red condition is a **zero count** — the
  attested case where a narrowing *added* a violation by pruning the file holding
  a declaration's sole instance. It is cleared here by the fact that neither
  `bin/session-id.sh` nor any surface this cut edits carries an install-path
  claim; the same is asked and answered of `check-manifest-count` (a bare cardinal
  quantifying a governed collection), `check-gate-fixture-coverage` (a coverage
  floor, which this member is outside because it is not a gate), and
  `check-gate-substrate-parity` assertion B (an **equality** between the `.gate`
  descriptor set and `--list`'s roster, which a non-gate arm stays outside by
  construction — that is precisely what §The non-gate arm's first property buys).
  The build re-runs the full battery rather than resting on this enumeration,
  which is the enumeration's purpose: it says where to look when one goes red.

## Existing sections updated

- `lifecycle-kit/SPEC.md §bin/session-id.sh` — restated for the arm: the
  invocation form and its front-end reach, the empty declared roster and why the
  two names cannot be declared, the environment-inheritance property the
  derivation order now rests on, and the port-disposition paragraph plus its two
  facts replaced by the cut record. The `### bin/session-id.sh` **heading is
  unchanged**, seven probed citation sites resolving against it (deltas 1, 2, 3,
  4, 10 and 12).
- `lifecycle-kit/SPEC.md §bin/enter-stage.sh` — the sentence naming
  `session-id.sh` as what it reads the id from; the role survives and the name
  changes (deltas 4 and 5).
- `lifecycle-kit/SPEC.md §The state machine` and `§Layout and configuration` —
  the former's "`bin/session-id.sh` prints the canonical id by a fixed derivation
  order" sentence, the latter's `bin/` tree listing and its
  `LIFECYCLE_KIT_SESSION_ID` roster entry, whose "source 1 of the derivation
  order" cross-reference survives while the file it points into moves (deltas 2
  and 10).
- `lifecycle-kit/bin/enter-stage.sh` — one invocation and its `# spec:` header's
  dependency sentence (deltas 4 and 5).
- `lifecycle-kit/templates/lead.md` and
  `lifecycle-kit/templates/stages/{scope,align,build,validate,close,spec}.md` —
  one live invocation and seven prose namings; the `templates/` markdown edit
  that fires delta (11)'s fan-out (deltas 6 and 11).
- `lifecycle-kit/smoke/install.sh` — the `SID` binding and the `sid_run` helper
  re-pointed at the arm, five assertions preserved; the harness stays `# no-port:`
  (delta 7).
- `lifecycle-kit/README.md` — the command roster gains the arm's line beside the
  two survey affordances — the class's stated usage home for a bridged arm, and
  `check-docs-cmd` assertion A is what makes the re-point mandatory rather than
  optional once the fenced `.sh` path is gone (delta 2).
- `gate-sdk/SPEC.md §The non-gate arm` — the class roster gains
  `--emit-session-id`, and the **empty-roster** paragraph gains its second member
  beside `--emit-md-section`, which is what stops the first from reading as a
  one-off (deltas 2 and 3).
- `drift-kit/bin/stage-economics.sh` — its `normalize8` comment attributes the
  normalization to "lifecycle's `session-id.sh`", which is a name this cut
  retires. Ungated for truth — `check-comment-tier` judges a comment's tier and
  never its accuracy — so an explicit target (delta 6).
- `.claude/settings.json` — one allow entry deleted, none added, in the commit
  that deletes its target (delta 8).
- `docs/site-architecture.md` — no ruling changes; named because delta (10)'s
  fan-out is read off it and a reader must not take the silence for absence
  (delta 11).
- `TASK-QUEUE.md`, the `native-gate-port-remaining-corpus` lead line — gains this
  amendment's `[spec:]` ref at **97 of 100 columns**, the arithmetic run on the
  entry rather than carried, and the lead's 2026-09-03 own-authority ruling joins
  the existing `lead … own-authority` declaration at 83 columns for **zero added
  lines**. It **demotes** at build rather than reaching `## Done`, and its counted
  extent is **50 against a cap of 50**, so any roster the build transcribes onto
  it is compressed in the same commit (deltas 1 and 10).

<!-- update-target-exempt: the composer entry takes no body write from a cut by its own 2026-08-28 ruling — each closed cut's record lives in the contract section that cut selected, which is delta 1's section -->
- `TASK-QUEUE.md`, `native-gate-port-remaining-corpus`'s body — deliberately
  unwritten.

<!-- update-target-exempt: a live design-pending entry owning a seam question this cut reshapes but does not rule; the port-only run answers nothing from the deferred pool, and writing the entry would be a mid-iteration edit on a question no stage here decides -->
- `TASK-QUEUE.md`, `overhead-meter-measures-the-lead` — deliberately untouched.

<!-- update-target-exempt: sequenced behind the installer's behind-invoke relocation by an operator ruling whose own text says it is written there so no future composer re-selects it; not this cut's subject and it takes no write from it -->
- `doctrine-kit/SPEC.md §install-doctrine` — already written, deliberately
  untouched.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The five smoke cases OBSERVED green through the arm**, not read off the
      diff — the derivation order is the one thing here a Rust rewrite can get
      subtly wrong, and the `agent-` strip and the child-narrowed scan are the two
      axes with no other oracle.
- [ ] **The grant count re-probed at the deleting commit** and the allow entry
      removed in it, per the 2026-08-29 carve-out's own terms.
- [ ] **`docs/footprint.md` and `docs/value.md` regenerated** in the commit that
      edits the `templates/` markdown, and the port oracle's `--tree` owed count
      re-read to confirm 45 → 44.
- [ ] **Both knob names still held in `check-docs-cmd` assertion B's corpus after
      the smoke rewrite** — re-probed at the deleting commit, not inferred from
      this amendment's reading, because the crate cannot satisfy that assertion.
- [ ] **New port prose checked against `check-unmarked-claim`'s `gate-substrates`
      class** — a rostered phrase such as *dispatches to a compiled subcommand* in a
      governed manifest paragraph owes a `measured:` marker, and the wording does
      not exist yet, so no reading settles it ahead of the write.
