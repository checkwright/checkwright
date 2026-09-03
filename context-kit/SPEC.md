# context-kit — token-economics-aware context management

A stateless agent session pays for context twice. On-demand cost: opening a
large SPEC or source file whole when only one section was needed. Standing
cost: the always-loaded surface — everything injected into *every* session
(the instructions file, the session-start hook output) — where each added
line is a recurring per-session tax that grows silently, because no single
session ever sees the trend. The kit attacks both: an index-first reading
toolset, a session-start hook template that assembles a compact brief, a
meter that tracks the always-loaded surface against a per-iteration
baseline, a brevity gate over the densest always-loaded section, the
close-stage brevity pass that reacts to the meter's delta, and a memory-off
gate pair that keeps the ungoverned harness-memory surface disabled rather
than left to accrete.

The kit carries the index mechanism, the hook skeleton, the meter, the
brevity machinery, and the memory-off enforcement; a consumer's product-shaped
surfaces (proto and diagram indexes, its doc roster, its harvest pipeline)
stay in the consumer repo (§Out of scope).

## Index-first reading

"Index, then read the one you need" — the pattern every tool serves. All three
are **advisory arms of the gate binary**, reached through the battery runner's
`--emit` front-end (`bash gate-sdk/bin/run-gates.sh --emit <name> …`); none joins
`gates.list` and none returns a verdict a battery reads, which is the non-gate
class gate-sdk/SPEC.md §The non-gate arm rules. Each is a **bridged-arm table
member** rather than a hardcoded flag, and for the first and third that family is
forced rather than chosen: a tool that resolves a consumer knob and is ported as a
hardcoded flag resolves platform defaults and silently ignores every override.

- **`--emit md-index [paths…]`** — compact structural index for Markdown:
  heading hierarchy with line numbers, each heading followed by its
  section's first sentence, plus a per-file line count (the cost signal —
  whether to read whole or by section). Defaults to the whole tree, minus
  `CONTEXT_KIT_PRUNE_DIRS`, which is its one declared read.

  Four observable properties **are** the contract, because each is a place a
  reimplementation would quietly differ. The first two are pinned by an
  index-tests golden (§Testing); the last two, and the fence asymmetry stated
  below them, are pinned by the arm's own crate tests, which `check-crate-arms`
  runs — the golden corpus carries no heading inside a fence and every golden
  invocation names one explicit file, so no golden reaches them:

  - **Per-file block shape** — `<repo-relative path>  (<N>L)` where `N` is the
    `wc -l` newline count, then one indented row per heading
    `<indent><hashes> <heading>:<lineno>`, or
    `<indent><hashes> <heading>:<lineno>  — <first sentence>` where a first
    sentence was found, then a blank line — after the last block too.
  - **The first-sentence rule** — the first non-blank line after the heading
    that is not inside a fence and is not itself a heading or a `---` rule;
    markdown link syntax reduced to its text, `*`, `_` and backticks stripped,
    cut at the first `.`, `!` or `?`, else at 120 characters.
  - **Traversal and order** — `*.md` under the given paths, `find`'s own entry
    model (a symlinked file is an entry, matched by name and read through),
    `CONTEXT_KIT_PRUNE_DIRS` matched on the leaf basename, results in byte order
    of the walked path — absolute in the default form, the default target being
    the repository toplevel. A file the walk reached but the reader cannot open
    contributes nothing rather than failing the run.
  - **The empty case** — `No Markdown files found in <targets>`, where the
    targets are the ones given and default to the repository toplevel, so the
    default form of the message carries an absolute path.

  The heading scan is deliberately **fence-blind** where the first-sentence
  search above is not: a `##` inside a fenced block is a row. That asymmetry is
  the shell form's and the arm's own tests record it, so it is contract rather
  than oversight.

- **`--emit md-section <file> <heading>`** — prints one section, from the
  matched heading to the next heading of the same or higher level. Match is
  **case-insensitive**, tolerates a **leading `§`** (so a spec citation pastes
  directly), and compares the heading's **text** — hashes and surrounding
  whitespace stripped — rather than a line prefix, so a query is a heading name
  and never a raw heading line. Headings inside fenced code blocks are not
  mistaken for structure **at either end**: one opens no section, and one does
  not close the section around it. The companion: find the heading in the index,
  extract just that body.

  **The matcher is this arm's and the walk is the crate's.**
  `section::sections` bounds a section by heading level exactly as this tool
  does, and that half is reused: the arm masks fenced lines out and hands the
  masked view to that walk, so the fence rule and the match rule stay the
  matcher's while the bounding stays one implementation. Widening
  `section::sections` itself would move its other callers across every call
  site to save one function.

  Matching is **exact on the heading's text**, and the near-miss behaviour that
  follows from it — empty on a near miss, correct on an exact query — is a
  stated limit rather than a defect the port fixed.

  **Its declared knob roster is empty**, because the arm resolves no knob; it is
  a bridged-arm table row regardless, since table membership is what makes
  `--emit md-section` reach it at all. Exit statuses: `0` with the section on
  stdout; **`2`** with a diagnostic on stderr for a missing argument, a file
  that is not there, and a query matching no heading. That last was exit 1 in
  the deleted shell driver; no caller read the 1.

- **`--emit pub-index [paths…]`** — compact public API surface: every public
  item with kind, name, and line, sorted by kind then name, in a per-file
  block headed by a count. It is a **dispatcher over per-language
  extractors**, and the port narrowed the tool without narrowing the extension
  point. The dispatcher owns three jobs: traversal (the prune set is
  `CONTEXT_KIT_PRUNE_DIRS`, the walk `md-index`'s above), the kind-then-name
  sort (bytewise, then the whole row), and the row formatting under a
  `<rel>  (<count>)` block header — the index-tests goldens assert the exact
  shape.

  **Resolution stays consumer-first** (the `gates.list` precedent). For each
  enabled language, a file at `<CONTEXT_KIT_PUB_LANG_DIR>/<lang>.sh` is used if
  it exists; otherwise the built-in extractor for that language; otherwise the
  arm refuses at exit 2 naming the language, the path it looked for and what is
  built in. The built-in roster replaces the deleted `lib/pub-lang/` leg rather
  than removing it, so a consumer's `rust.sh` still shadows the shipped Rust
  grammar.

  **A consumer extractor is still a sourced bash file defining exactly two
  names.** `PUB_LANG_GLOBS` (the find globs, e.g. `*.rs`) and
  `pub_lang_extract <file>` (emitting unsorted `kind name lineno` rows) are
  unchanged contract; two names, both read every run, and no other
  extractor-file surface is contract. The arm runs one through **two `bash`
  spawns per language** — one sources it and prints `PUB_LANG_GLOBS`, one
  sources it and calls `pub_lang_extract` over the file list the arm walked.
  Two rather than one because the globs are needed *before* the walk, and one
  spawn per contract name rather than a private protocol so the seam a consumer
  writes against does not change shape. The per-file framing inside the second
  spawn is the dispatcher's own and reaches no consumer name.

  **Two extractors are built in**, `rust` (every `pub`/`pub(crate)`/`pub(super)`
  item of the eight declared kinds) and `ts` (TypeScript: `export`-declared
  `function`/`class`/`interface`/`type`/`enum`/`const`/`let`/`var`, `const enum`
  folded to `enum`, and `export default` named or falling back to the literal
  `default`, over `*.ts` and `*.tsx` with `.d.ts` included as public surface by
  construction). Both stay **grep-grade**, re-expressed against the crate's
  POSIX ERE matcher: re-exports (`export { x } from`) and multi-line
  declarations are stated honest limits, not parsed. They are **not arms of
  their own** — their only caller is the dispatcher and their only output
  contract is the rows it consumes, so a flag each would mint two spellings with
  one caller between them and put a second entry point into the emission path.
  The dispatcher shipped when demand named it — a second adopter's tree carries
  a TypeScript surface its Rust-only copy could not index — not before: an
  unrequested plugin framework would have been scaffolding, and AST/tree-sitter
  parsing is above the tool's grep-grade portability altitude.

**One traversal-exclusion set, read by both walkers.** `CONTEXT_KIT_PRUNE_DIRS`
is the kit's single exclusion array; the `md-index` and `pub-index` arms share
one walk over it instead of each holding a private literal, and both receive it
through gate-sdk's config bridge, which resolves it by sourcing `lib/context.sh`
— the kit's one owner of the consumer-config seam. Two private literals had
drifted apart *and* from the tree: neither carried the harness's isolated-agent
worktree leaf, so both walkers descended into a second full copy of the
repository and indexed it as tree content, which for `md-index` is every
governed markdown surface twice.

The match is on the **leaf basename**, the same rule and the same reasoning
gate-sdk/SPEC.md §lib/gate.sh fixed for its own set: pruning a parent path also
passes but loses coverage silently, because the governed markdown under it is
reached by explicit globs no prune touches. gate-sdk's set additionally covers a
scratch dir and a fixture-corpus dir, and both are **deliberately absent from
this one** — this set's subject
is the second copy of the repository, and adding either is a corpus narrowing
with its own readers (the index-tests goldens; a session that legitimately wants
a fixture corpus indexed). Whether the two sets should converge is an open
question, filed rather than taken. They are **not one fact spelled twice**: one
carries leaves gate-sdk's does not and omits two that it has,
gate-sdk/SPEC.md §lib/gate.sh owning that set's membership and this knob's
default owning this one,
and deriving this one from the gate library would make context-kit's own
exclusion rule a gate-sdk read, so a consumer assigns this knob in its own config
instead.

## The session-context hook (template)

`templates/session-context.sh` is a consumer copy (the `bash-guard.sh`
pattern): wired as the harness's session-start hook via
`templates/settings-sessionstart.json`, it assembles the session brief.
Every step is guarded and degrades silently — the hook never fails a
session.

**The template and its consumer copy are both permanently shell**, declared
`# no-port:` under the class ruling at gate-sdk/SPEC.md §The harness-template port disposition.
The template carries an `[EDIT ME]` gap at every layout-judgment step below —
tool paths, the dirty-surface pre-run, the stage-conditioned nudges, the index
footer, the probe path — and README.md tells an adopter to fill them as layout
judgment rather than mechanism, so those gaps *are* the extension point and a
compiled form would leave nothing to fill. The copy declares on the second
ground: it is the filled instance, so everything it holds beyond the template is
this repo's own layout content.

**A `hooks[]` edit arms immediately, and this is the kit set's record of it** —
true of every hook registration in the settings file, not just this template's:
the running session and its already-dispatched subagents pick it up with no
restart and no re-dispatch. Measured rather than assumed, by watching a newly
registered hook's log fill inside the same session that wrote the registration.
It bounds what a session-start hook can be relied on for: arming is not the
same event as the session brief, so a hook wired mid-session runs without ever
having assembled one.

Steps, in order:

1. **Queue index** — via queue-kit's `queue-index` arm, reached through the
   battery runner's `--emit` front-end (queue-kit/SPEC.md §The queue-index arm),
   collapsing the
   Deferred section to a tally except on the close and scope stages. The stage
   it routes on comes from the cursor — `CONTEXT_KIT_STATE_FILE`'s last data
   line — with an empty cursor (no such file, or one truncated to its preamble
   at an iteration boundary) taking the collapsed branch every non-close,
   non-scope stage already takes, so the no-cursor window needs no branch of
   its own and can never fail the session.
   Deferred is unpickable and only scope (promotion) acts on it; the full
   board additionally serves close's backlog review, and — by the
   cursor-lag rule below — firing on close is what hands the full board to
   the first scope session, which reads `close` at session start. On any
   other stage the full listing is pure recurring cost.
   A configured icebox tier rides **both** branches as a one-line tally, so
   this step needs no new stage routing: a dormant entry is already one line,
   so there is no full listing of it to withhold, and the eviction work that
   reads the tier reads `--emit queue-index --icebox-candidates` rather than this
   index.
2. **Dirty-surface pre-run** — for each component with uncommitted
   changes, pre-run the matching surface index (default: `--emit pub-index`
   over top-level dirs containing `src/`), so a resumed session's editing
   surface is already in context. Component detection and the index
   command live in a marked consumer section of the template — they are
   layout assumptions, not mechanism.
   **The step's availability guard reads the gate binary, and the ordering is
   the contract rather than a detail.** The index is an arm now, so the guard is
   that the binary is present and executable and it is taken **before** the block
   prints its header: `exec_arm` exits 2 with a diagnostic that this call site
   swallows on both channels, so a guard taken after the header would print
   `Public API surface of those components…` followed by nothing on every host the
   artifact roster does not cover. Read first, the block is **absent rather than
   empty** — the way the deleted script-path guard degraded. The lookup runs in a
   subshell, because the kit library exits 2 on a malformed config and this hook
   never fails a session.
3. **Drift line** — one `--emit <arm> --trend` summary line when the
   consumer has a drift report; silently absent otherwise (drift-kit owns
   the report; the seam is this optional line).
4. **Stage-conditioned nudges** — short reminders keyed on a stage set
   read from the header (this repo's delegation nudge is the exemplar).
   Each keys on {predecessor, own stage} (the cursor-lag rule below) so a
   first-of-stage session and a restarted one both receive it. Marked
   consumer section: which stages get which nudge is consumer judgment.
   Suppressed when the session-role signal (below) marks the session `lead`
   — the nudges are executor-facing.
5. **Memory-off backstop** — one warning line when the harness memory dir
   (`CONTEXT_KIT_MEMORY_DIRS`) holds content, pointing the durable fact at its
   tracked home (§The memory-off doctrine). `check-memory-off` fires only at
   commit; this surfaces pollution at session start, between commits. Silent
   when the dir is empty or absent.
6. **Scratch sweep** — reclaim `${GATE_SDK_TMP_DIR:-.tmp}` entries older
   than a day, depth-first (`-mindepth 1 -depth`) so stray directories are
   reclaimed too, never touching `.gitkeep`. Age-guarded so a concurrent
   same-checkout session's in-flight scratch survives.
7. **Index-reminder footer** — the "index first" ritual with the
   consumer's actual index commands listed (consumer-edited).
8. **Stage-routed craft-rule pointers** — when `CONTEXT_KIT_STAGE_RULES`
   names a **command** (doctrine-kit's `--emit stage-rules` arm), the current
   stage's craft-rule pointer block, so a stage session is reminded of the
   craft rules bearing on it before the matching action. The step runs the
   resolved command with the stage appended and carries **no `-f` existence
   guard**, the drift line's own 2026-08-29 shape: a path test on a command
   passes for nothing. Silently absent
   when the emitter is unset or the stage routes no rules (doctrine-kit owns
   the emitter and its tag grammar; the seam is this optional step, the
   drift-line precedent). Keyed on the derived stage directly, not a
   {predecessor, own stage} set: the pointers are advisory reminders, so
   over-firing to an adjacent stage costs only a few lines, and the
   cursor-lag rule below is about *conditioning*, not about which stage's
   rules to show. Suppressed when the session-role signal marks the
   session `lead` — the craft rules are executor-facing.
9. **Env profile** — when the consumer-local profile file
   (`CONTEXT_KIT_ENV_PROFILE_FILE`, §bin/env-probe) exists, the step first runs
   `run-gates.sh --emit env-probe` to re-probe it (output suppressed so no
   status line reaches the brief), then emits its whole body verbatim so the
   session adapts
   its commands to the box as it is now. The re-probe sits inside the same
   file-present guard, so producer and consumer co-locate here and the probe
   never auto-seeds a profile the operator did not opt into. Silent when the
   file is absent — the harness's own `Platform:` line is the fallback, no
   re-probe fires, and no always-loaded cost is paid where no profile was
   seeded. The consumer owns the file's brevity (they author the gotchas half),
   the drift-line precedent.

**The cursor-lag rule.** The hook runs at session start, before the
arriving skill stamps its entry (its first step) — and that stamp is what
moves the cursor. So a first-of-stage session reads the *predecessor's* stage.
The lag is structural and survives any change of cursor source: the hook reads
a value written after it fires. Stage-conditioned
output therefore keys on a stage *set* spanning {predecessor, own stage} —
guaranteeing the first-of-stage session is served, at the accepted cost of
over-firing to the other sessions that share a cursor value (a restarted
session of a keyed stage, or the first session of the stage after it).

**The session-role signal.** The hook keys every stage-conditioned injection
off the lifecycle stage cursor, which says nothing about whether the
*reading* session is a lead, a stage session, or a manual run — so without a
role signal a lead draws executor-facing craft rules at every hook fire
(startup, plus each compact/resume re-fire, the recurring cost). The signal
is a marker file, **session-id-scoped**: `/lead`'s first step writes one line
`lead <id>` — `<id>` being the `--emit-session-id` arm's value — to
`CONTEXT_KIT_SESSION_ROLE_FILE` (gitignored scratch, default
`${GATE_SDK_TMP_DIR:-.tmp}/session-role`). The hook treats the session as
`lead` only when the marker's id equals the 8-char prefix of **its own
payload's session id** — read from the hook payload, never from
`CLAUDE_CODE_SESSION_ID`, because a subagent is handed its *parent's* id in
that variable (the named assumption: were the harness ever to fire
`SessionStart` in subagents, an env-var read would match every stage session
to its lead's marker and invert the suppression onto exactly its intended
audience; revisit if subagent hook-fire lands). So a concurrent or later
top-level session never bleeds, and a stale marker self-invalidates when the
id rotates. The payload arrives on the hook's stdin, consumable exactly once,
so the single read here is its sole consumer — a later payload-derived signal
(a stage derivation, say) must hoist that one read ahead of this guard, never
add a second. Top-level scoping is sufficient because both producer and
consumer are top-level by construction — `SessionStart` does not fire for
Task-spawned subagents, so the only sessions the hook fires in are leads and
manual runs, and the identity match discriminates exactly those. When the
signal marks the session `lead`, steps 4 and 8 are suppressed; everything
else emits unchanged. Signal absent ⇒ byte-identical to the signal-free
hook; the read is guarded like every step — the hook never fails a session.
Accepted limits (not defects): the initial startup fire precedes `/lead` by
construction, a bounded one-per-lead-session cost against the
per-compact/resume recurrence that is the actual waste; the marker ages out
with the day-horizon scratch sweep (step 6), degrading to absent-signal
behavior; the marker's lifetime is the **lead session's, not the iteration's**,
so a consumer boundary ritual that wipes gitignored scratch (a scope
evidence-reset, say) must spare the marker file — a lead outliving an iteration
boundary otherwise reverts silently to absent-signal behavior until it rewrites
it; and the producer inherits the `--emit-session-id` arm's
`CLAUDE_CODE_SESSION_ID` dependency — unset, the newest-transcript fallback
in a lead with live subagents returns an `agent-` prefix the payload can
never match, and the signal silently no-ops to absent-signal behavior (the
failure costs a suppression, never a correctness property). Rejected
alternatives, recorded so they are not re-derived: a launch-env var (a
perpetual operator ritual whose forgotten export degrades silently with no
signal it happened) and both-producers-with-precedence (two producers plus a
precedence rule to spec and gate, for a gap one hook fire wide).

**Ruled out — lifecycle stamp-id injection.** The hook payload carries the
harness session id, and **in a top-level session** its 8-char prefix equals
what lifecycle-kit's `--emit-session-id` arm computes, so the hook *could* inject the
canonical stamp id with no shell-out. The parity is top-level-only and holds
only while the harness sets `CLAUDE_CODE_SESSION_ID`: a subagent is handed its
*parent's* id in that variable, while the arm deliberately derives the
subagent's own transcript id instead (its `CLAUDE_CODE_CHILD_SESSION` branch),
so the two quantities diverge there by design. The hook does not inject: lifecycle-kit owns its id derivation
end-to-end (the stage-entry ritual derives it via `--emit-session-id`,
whatever invokes that arm), and having the stage skills
read a context-kit-injected value would wire an upstream kit's protocol to
a downstream kit's hook for ergonomics only — the trust model gains
nothing, since `check-stage-evidence` already enforces that the stamped id
is current. A consumer may add a local informational echo; the template
ships none.

## bin/env-probe

The env-probe member derives a local machine profile so a session adapts to the
box it runs on — package manager, toolchain versions, absent tools — without
those machine facts ever landing in the public tree. It writes a
marker-bounded generated block (`<!-- context-kit:env:begin -->` /
`:end`, via gate-sdk's shared marker-block writer) into the file named
by `CONTEXT_KIT_ENV_PROFILE_FILE` (default `ENV.local.md`), replacing an
existing block or appending a fresh one — but only when the probed content
actually changed (Cadence, below), so the block's probe date marks the last
real change, not the last run. The probed half is derivation-first — never
hand-maintained.

**How it is invoked, and its one knob.** It is a bridged non-gate arm
(gate-sdk/SPEC.md §The non-gate arm) rather than a script: `--emit-env-probe`,
reached by every caller as `run-gates.sh --emit env-probe`, so the front-end
resolves its configuration and hands it over. It is an *action that reports* —
it rewrites the block and prints one line naming what it did — and both its
failures exit 2. Its declared knob roster is **`CONTEXT_KIT_ENV_PROFILE_FILE`
and nothing else**: `lib/context.sh` defaults that name and is the config
bridge's sole resolver for the family, so the value is computed in one place and
no second default exists to drift. A hardcoded profile path would resolve
`ENV.local.md` and silently ignore every consumer override, which is the failure
gate-sdk/SPEC.md §The non-gate arm names as the difference between working and
appearing to.

**The marker test is whole-line, on both halves.** The presence test guarding
the change-detection read and the writer's own test agree, so a marker occurring
inside prose opens no block: gate-sdk/SPEC.md §lib/inject.sh rules that
resolution, and this member carries it rather than the shell form's substring
guard over a whole-line extraction — which reported a replacement it had not
made.

**What it probes.** OS/distro (`uname`, `/etc/os-release`); the package manager
(first present of an ordered detection walk over the known managers); each
roster member's version and its floor verdict (below); the absent-tools list
(roster members `PATH` does not resolve); and the below-contract list. The
roster itself is owned by `lib/toolfloor.sh` and never restated here. Its
spawned programs are `uname`, `date`, `sort`, and every roster member it probes
— the first arm of its class whose spawn set a consumer can change, since
`PROBE_SET` is a file a consumer can shadow.

**The roster and its floor axis (`lib/toolfloor.sh`).** The roster lives in a
sourceable library rather than in a member that does its work on execution: a
reader cannot obtain the roster by running such a member, which
is why the parity gate greps the array out of a file instead of sourcing it, and
why a reader that runs before any consumer file exists — an installer's `doctor`
reading its own payload copy — needs an owner it can source. The compiled holder
below reads the same array as text, for the reason that gate does: a fixture path
is untrusted input. The library defines
the array `PROBE_SET` and the predicate below and executes nothing else. It
carries no knob, deliberately: the roster is the kit's own dependency set, and a
consumer who could override it could only make the contract lie.

A roster element reads `<name>[:<min-version>[:<impl-token>[:<audience>]]]`. A
bare name keeps
the original meaning — must be present, no version constraint — so the floor axis
is **per-member** rather than a number demanded of every member. The fields are
positional, so a member constrained by implementation alone carries an empty
min-version field (`awk::GNU`), and an empty field means what an omitted trailing
field means: `awk`, `awk:`, `awk::` and `awk:::` are one unconstrained member. **A member
gains a floor only where a construct the battery actually runs forces one**, and
the forcing construct is recorded with it — a floor nobody's code forces is not
pinned, which is what stops a version number from rotting into an aspiration
(de-literalization applied to a version: the value is owned where the constraint
is provable).

**The audience axis.** The fourth field names *whose* floor a member is, because
the roster has two kinds of reader and one flat array gated both of them on all
of it. Its value set is closed and kit-owned exactly as the floor predicate's
verdict set is: the only declarable value is `contributor`, meaning *a
contributor-side floor with no install-time role*. The unmarked case is not
spelled — declaring the complement on every other member would be a roster
maintained against itself — so the emptiness rule above carries it: an empty or
omitted audience means every audience. `tool_floor_consumer_side <element>`
answers the one question a consumer-side reader asks, and exists so no such
reader re-implements that rule against a value set it does not own. A
consumer-side reader — `installer/lib/doctor.sh`, whose exit status is `init`'s
last precondition — filters its roster walk through that predicate and does not
probe, render or fail on a member the predicate excludes. A contributor-side
reader — the env-probe arm — walks the roster whole and marks the audience
instead, since a contributor-side floor is exactly what it is reporting on.
The field is a grammar axis rather than a filter in the reader that needs it,
because a hard-coded exception is a literal de-literalization forbids and one
that re-fires the day a second contributor-only member lands.

The constrained members and what forces each:

- `bash:4.3` — the floor is set by the **highest** construct the battery runs,
  not the most numerous. Three bash-4.0 constructs are widespread — `declare -A`
  (gate-sdk, guard-kit, delegation-kit, evidence-kit checks), `mapfile` (across
  the kits), the `${x,,}` case expansion (canon-kit's `lib/spec.sh` and checks, a
  delegation-kit template) — but the **nameref** (`local -n`, bash 4.3) outranks
  them: `gate-sdk/lib/gate.sh`'s couples expander, which every gate sources. The
  leaf gate that carried the second instance, `check-comment-tier`, has since
  ported to the binary substrate and its script is gone — which changes nothing
  about the floor, because the nameref in the shared gate
  library makes 4.3 universal rather than one leaf gate's requirement, so a
  consumer below it cannot run the battery at all. Recorded here because the
  earlier `4.0` was a fail-open: `env-probe` reported `ok` on a 4.2 box the
  battery would fail with an obscure syntax error.
- `awk::GNU` — no version floor, one implementation constraint, whose last live
  holder the eighth port cut retired; narrowing the element is filed as
  `interpreter-floor-gawk-residue-empty` (gate-sdk/SPEC.md §check-gate-assertions).
- `sort::coreutils` — no version floor, one implementation constraint, and one
  member standing for a whole package family: GNU coreutils is forced by
  `realpath --relative-to` and its `-m` form (gate-sdk's shared gate library and
  hook emitter, canon-kit's link and command resolvers), `sort -V`
  (`check-release-bump`), `date -d` (drift-kit's KPIs and trajectory), and
  `stat -c` (delegation-kit's usage verdict). None is BSD-portable, and the
  first sits in the library every gate sources — so a box without it cannot run
  the battery at all rather than failing one gate. The representative member is
  the binary carrying a forcing construct, which is also the floor predicate's
  own comparison tool.
- `cargo:1.71::contributor` — a **contributor-side** floor, never a runtime one,
  and that reading is now declared on the element and read by name rather than
  left as an aside: the audience field is what the consumer-side predicate
  above resolves, so the sentence is enforced instead of merely written. It
  carries
  two tiers because two kinds of tree read it. Where a `.gate` descriptor is live
  it is a **commit-time** floor — `gate_command` puts the binary on the pre-commit
  path and is fail-closed on an absent one, so the battery will not run without a
  built crate (gate-sdk/SPEC.md §What the dispatch seam does not settle); that is
  this repo today, its first cohort having landed. Everywhere else it stays the weaker
  **contributor/build** floor: a consumer tree receives a prebuilt binary and never
  a crate, so nothing there compiles at commit time. Both tiers rest on the same
  forcing fact — the **highest MSRV in the crate's resolved dependency graph**,
  which since the settings cohort took the crate's first dependency
  (gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency) is
  above the `edition = "2021"` floor of 1.56 that governed while the graph was
  empty. It is re-derived against the lock at any dependency change rather than
  recalled, and a move carries every surface stating it — including **§The
  rendered verdict below and `docs/site-architecture.md`, both of which quote
  this element as a *format example***, and both machine-checked by nothing. That
  pair is named rather than described because a format example is the shape a
  floor move keeps missing: it reads as illustration, so it survives the grep a
  reader runs for the surface. The 1.56 → 1.71 move missed
  `docs/site-architecture.md` at authoring and this section's own example at the
  merge, the second caught only by a close-stage audit. It is also a `check-crate-arms` input: clippy
  suppresses a lint whose suggested API postdates the declared floor, so raising
  the floor un-suppresses lints against code no change touched.
  `cargo` is the member rather than `rustc` because `cargo build` is what the
  contributor routine and the `gates` workflow actually invoke, and the two ship as
  one toolchain sharing a version banner — the representative-member rule the
  `sort::coreutils` entry above states. The floor tracks what the crate and its
  graph actually require, not whatever rustc a given box happens to carry; pinning
  the latter would be exactly the aspiration this section's rule forbids. Runtime is
  unaffected: git remains the sole runtime dependency of a ported gate, shelled out
  rather than embedded (TRAJECTORY.md §The closed rulings owns that constraint).
Every other member is a bare name — no construct in the battery forces a version
on it (the `jq` usage is 1.5-era throughout), so none is pinned.

An implementation token is matched as a **substring of the tool's own version
banner** — gawk prints `GNU Awk`, GNU sort prints `sort (GNU coreutils)` — so the
constraint is checked against the binary actually on `PATH` rather than against a
package name nothing can probe. Its honest limit is the same one: the roster
asserts what `PATH` resolves at probe time, so an installed-but-not-`PATH`-ordered
GNU userland probes as below contract, correctly, since that is what the gates
will invoke.

**The floor predicate.** `tool_floor_check <element> <banner>` returns one verdict
from a closed set — `ok`, `absent` (an empty banner), `below <found> <floor>`,
`wrong-impl <found>`, `uncomparable`. Numeric comparison is `sort -V`.
`<found>` is the banner's first dotted-version token for `below` and its
first word — the implementation's own name — for `wrong-impl`. `uncomparable` is
the fail-closed arm: a banner the predicate cannot parse, or a `sort` without
`-V`, is reported unverified and never silently as `ok` — the posture
gate-sdk/SPEC.md §The gate model requires of a gate, applied to a probe that is
not one.

**The predicate has two holders, and a standing oracle is what licenses that.**
`installer/lib/doctor.sh` calls the shell library off its own payload copy, so
the shell caller set does not empty and the deletion road
gate-sdk/SPEC.md §The port-candidate criteria prefers is unavailable; criterion
6's *unless* clause applies instead, and what discharges it is an executed
cross-substrate comparison in the shape evidence-kit/SPEC.md §lib/evidence.sh
established. `gate-tests/toolfloor-parity.test.sh` drives one canned corpus of
`(element, banner)` pairs through both holders and compares **classification** —
the parse's four fields and the verdict's own words — with **no committed
expected file**, because the failure it exists to catch is one holder edited
without the other and a golden would be a third copy to drift. The existing
golden is not retired: §Testing's `index-tests/toolfloor-cases.sh` remains the
**shell** holder's own oracle, the one `installer/lib/doctor.sh` still runs.

**`sort -V` is preserved in the compiled holder rather than replaced by a native
comparison.** The `uncomparable` verdict is fail-closed for two conditions, and
the second — a `sort` without `-V` — is one no in-process comparison can reach.
Removing the spawn would narrow the verdict's reachable conditions on one holder
while they stay live on the other, and the population that disagreement lands on
is exactly the BSD or stock-macOS userland the verdict exists for. The parity
lane asserts this directly rather than trusting the reading: it puts a `sort`
that rejects `-V` on `PATH` and requires `uncomparable` from both holders, a
condition a canned corpus cannot express.

**The rendered verdict.** Each toolchain bullet carries the probed banner and,
for a constrained member, the constraint and its verdict — `` (floor 4.3, ok) ``,
`` (requires GNU — below contract) ``, `` (floor 4.3 — unverified) ``; an
unconstrained member carries no parenthetical. A member carrying an audience
carries it here too, as `` (floor 1.71, contributor-only, ok) ``, and every line
that names such a member is marked the same way — the absent list and the
below-contract list included, because those are the two lines on which *below a
floor that is yours* and *below a floor you are not on the hook for* would
otherwise read alike. A `**Below contract:**` line joins
the existing `**Absent:**` line, reading `none` when clean and otherwise naming
each failing member through the verdict's own fields: `below` and `wrong-impl`
are distinguished because the remedies differ — upgrade versus install a
different implementation — and `uncomparable` is listed as explicitly unverified
rather than folded into the clean state. Both version probes read from
`/dev/null`: `-V` prints a version banner for most tools but is an ordinary flag
for some — GNU sort's version-*sort* — so a tool that rejects `--version` would
otherwise fall through to a `-V` that reads inherited stdin and hangs the probe.

**The content seam (consumer-local, gitignored).** Hand-authored gotchas — the
"no `dig`/`host`; use `getent`/DoH" class a probe cannot know — live *outside*
the markers in the same file and survive every re-probe; when the file is
absent the probe seeds that scaffold once, then only ever rewrites the block.
The per-session re-probe (Cadence, below) does not trigger that seeding: it
runs only against a file that already exists, so seeding stays a
first-run/on-demand action.
The file is `BRIEF.local.md`-class: local-only, gitignored (this repo's
`.gitignore` and the CLAUDE.md housekeeping line carry it), so machine facts
stay private.

**Cadence — per-session auto-refresh.** The session-context hook re-probes
once per session, at its step-9 profile emit (§The session-context hook), so a
session always adapts to the box as it is now — the install step seeds the
profile once, and every session thereafter refreshes it. Change-detection keeps
this cheap and the date honest: the probe rewrites the block only when the
probed content differs from what is on disk (the `Probed` date line excluded
from the comparison), so an unchanged box writes nothing and the date marks the
last real change, not the last run. Still no freshness gate — env truth is not
cheaply machine-verifiable and the probe is already the derivation (the
enforcement-first carve-out; the per-session re-probe is now the enforcement,
replacing the install-step-only cadence). The hook re-probes only when the
profile file already exists, so a never-seeded consumer pays no cost and seeds
nothing unbidden.

## The always-loaded meter

`bin/always-loaded.sh` measures the standing surface: the summed line
count of the configured surface files (default `CLAUDE.md`) plus the
steady-state hook body, approximated by the configured hook-body command
(default: queue-kit's `queue-index` arm through the battery runner's `--emit`
front-end, `--collapse-deferred`, when resolvable). The approximation is deliberate: the meter must never run the
session-context hook itself — the hook emits this meter's own output line,
so self-measurement would recurse and inflate.

The meter lives here, not in drift-kit's collator, because the *metric* is
context economics and the *report* is drift reporting — drift-kit's
`kpi-always-loaded` consumes this script for its row instead of re-embedding
the measurement.

- **Default invocation** prints one line: total, per-part breakdown, and
  the delta against the baseline when one exists.
- **`--update-baseline`** rewrites the baseline file — a close-stage act,
  because the brevity pass reacts to the *delta*, not the level (close is
  net-additive by design; only growth since the iteration started is
  actionable).
- **Baseline file** (`${GATE_SDK_WORKFLOW_DIR:-.workflow}/`
  `always-loaded-baseline.txt`, committed): a `# contract:` header
  pointing here, then one data line
  `<total-lines> <surface-lines> <baseline-commit>`. Trailing extra
  fields are tolerated and preserved-ignored — a consumer's file may carry
  a fourth (a settings-local count, say, a guard-kit-adjacent KPI owned by
  its drift report), and the kit reads such a file unchanged. The baseline is
  the consumer-side floor-holder for the footprint contract
  (§The consumer footprint): its delta is what the close-stage brevity pass
  reacts to.

## bin/footprint

The footprint emitter publishes the kits' measured context footprint — the
adoption-cost evidence a consumer weighs before vendoring, the concrete form of
the token-economics positioning. Where the meter reads one consumer's live
always-loaded total, this reads the tracked kit surfaces and attributes the cost
per kit, split by when it lands.

The measured set is the kit roster, **derived not maintained**: the top-level
directories carrying a `SPEC.md`. No knob names the set — a new kit joins the
page by existing.

Each kit is measured in the always-loaded and load-triggered tiers:

- **Always-loaded** — the block a kit generates between its own
  `begin`/`end` markers in the configured surface files
  (`CONTEXT_KIT_SURFACES`), the agent-file text the kit injects into every
  session's context. This reuses the always-loaded meter's surface set; most
  kits inject nothing and score zero here.
- **Load-triggered** — the skill and template markdown the kit ships under its
  `templates/` tree, pulled into context only when its trigger fires. Gate-test
  fixtures sit outside `templates/`, so they never enter the count.

**Numbers ruling.** Line counts are exact. The token column is a
*labeled estimate* — a bytes/4 heuristic, carried with a leading `~` and stated
inline as model-tokenizer-dependent, never a false-precision figure.

**Attribution ruling: kit share only.** A kit's advertised cost is what the kit
ships. Consumer bindings (the skill shims pointing at a vendored template),
consumer config, the on-demand SPEC/README pages, this repo's own `CLAUDE.md`
residue, and the session hook's dynamic body (consumer state, not fixed kit
text) are all excluded, and the page states the exclusion so the number is
honest. The exclusion is also a determinism requirement: the freshness gate
byte-compares the emission, so every measured surface must be a static tracked
file — a live hook run, being state-dependent, could never be byte-gated.

**Emission, and the substrate it runs on.** The emitter is a **non-gate arm of
the gate binary**, `--emit-footprint` (gate-sdk/SPEC.md §The non-gate arm), and
it prints the committed `docs/footprint.md` page whole: front matter, method and
exclusion prose, then the table with a totals row. It is reached as
`bash gate-sdk/bin/run-gates.sh --emit footprint`, because an arm receives no
configuration of its own and that front-end resolves `CONTEXT_KIT_SURFACES`
across the config bridge before invoking it.

The emission is a **library function** the arm wraps rather than the arm itself,
which is what lets §check-footprint-fresh call it in-process and the value
rollup consume its per-kit figures as data rather than re-parsing the rendered
page.

**The advisory bare mode did not survive the port**, and its loss is the ported
script's deletion rather than a separate decision: the shell emitter printed a
human header plus the table on a bare invocation, and the replacement arm's
contract is exactly what `--emit` printed. The table is still reachable — it is
in the emitted page — so what went is the header, which had no reader that the
page does not serve. Advisory by construction is unchanged: the arm never joins
`gates.list`, and the freshness gate (§check-footprint-fresh) is what blocks a
stale page.

## The brevity gate

`check-brevity` — a section-agnostic name: the governed section
is a knob, so no section name binds the gate (a consumer's
`check-convention-brevity` would be its section-specific counterpart). It scans one designated
always-loaded file for a bulleted section where each `- **name:**` bullet
carries a line budget, and flags a bullet that is **over budget and cites
a deeper doc** (carries a `§` pointer) — over-long while admitting its
detail already has a home elsewhere. Under-budget bullets and over-budget
bullets with no pointer pass (the latter may genuinely own their content);
`<!-- brevity-exempt: <reason> -->` on the bullet's first line or the line
above blesses a bullet whose every line is load-bearing.

Section resolution fails closed: a `CONTEXT_KIT_BREVITY_SECTION` matching no
heading in the governed file exits 2, never a clean 0. The knob and the heading
are a coupling no other gate holds, so a renamed or deleted section would
otherwise disarm the gate while it reported an empty section as clean — a gate
whose target vanished is a broken machine, not a clean tree. A section that
resolves and holds no bullets is clean: resolution is what fails closed, not
emptiness.

The pointer default is any `§`, not a single doc name like `HANDBOOK §`:
"cites a deeper doc" is the mechanism-level meaning and a consumer's handbook
is one instance of it, so the superset matches every such pointer. Ships with
a `good/`+`bad/` fixture pair and registers in the
consumer's `gates.list` (this repo's included).

## The close-stage brevity pass

`templates/close-brevity.md` is the recurring close-stage step a consumer
splices into its close skill (the guard-kit `close-triage.md` pattern).
The procedure: run `always-loaded.sh`; walk the growth since baseline
asking two distinct questions — staleness (*is it still true?*) and
brevity (*is each block worth its standing per-session token cost?*);
resolve by rewording or deleting, never by annotating (outdated context
<!-- manifest-temporal-exempt: names the "formerly…" note as the anti-pattern this pass forbids, not written as narration -->
goes to git history, not to a "formerly…" note); on-demand files (SPECs,
handbooks) are exempt — their cost is paid only when opened; finish with
`always-loaded.sh --update-baseline` and commit the baseline.

The lexical share of this narration judgment — a fixed set of `formerly…`-class
markers in the manifest set — is a blocking gate
(canon-kit/SPEC.md §check-manifest-temporal); this pass keeps the semantic
residue (*is this sentence about the past?*) that no marker set can decide.

## The consumer footprint

A consumer project pursues its own objectives; the tooling must stay
near-invisible in its context budget. This section states and holds the kits'
consumer-resident footprint — what checkwright asks a consumer's always-loaded
surface to carry — owned in one place, with the always-loaded meter's baseline
as the consumer-side floor-holder. It is prose contract only: no new state,
event, knob, or gate. Its producer is the kit author at kit-landing time (the
kit-landing checklist gains no step; the review seam is the roster row); its
readers are a consumer evaluating adoption cost before vendoring and this
repo's own close-stage brevity pass, which reads the roster when judging
whether a new resident line is a kit ask or repo content.

**The budget rule.** A kit's resident ask is at most one pointer line on the
consumer's always-loaded surface — the load-trigger-residency and
always-loaded-shape doctrine rules applied to kit shipping. The one sanctioned
block-sized ask is doctrine-kit's digest, itself bounded by its one-line-per-rule
shape (the always-loaded-shape rule); its name-lockstep with the doctrine is
held separately (the doctrine-rule-lockstep unit), because a re-vendor upgrade
stales the digest by construction.

**The roster, by citation.** Each kit's resident ask is named by citing the kit
SPEC section that owns it, never by restating it here — so the roster cannot
drift out of lockstep with what the kit actually ships. A kit adding a resident
ask adds its row below, which is the review seam:

- **delegation-kit** — the pre-authorization sentence plus skill pointer:
  delegation-kit/SPEC.md §One template, a resident pointer.
- **doctrine-kit** — the doctrine link plus the digest block: doctrine-kit/SPEC.md.
- **drift-kit** — the knowledge-friction capture bullet:
  drift-kit/SPEC.md §The knowledge-friction loop, which already states the
  one-bullet cost and its earn-back condition.
- **every other kit** — none: their hooks, skills, gates, and SPECs are load-
  or event-triggered, so they cost nothing until opened.

**The floor-holder ruling.** The meter plus its committed baseline
(§The always-loaded meter) ship as the consumer's floor-holder: the consumer
install (§Layout and configuration) seeds `always-loaded-baseline.txt`, so
growth of the resident surface is a visible delta at every close-stage brevity
pass. The hold is *advisory by design*. A hard total-line gate cannot attribute
growth — the consumer's own content shares the file and is theirs to grow — so
a level gate would be a noisy check breeding exemptions, the high-false-positive
case the enforcement-first rule sanctions for keeping a class as stated manual
duty rather than a gate. The mechanical holds that do exist stay: `check-brevity`
bounds the one bulleted section its knob designates (§The brevity gate; this
repo points it at the conventions block, not the digest), and the meter delta
feeds `kpi-always-loaded`.

The measured counterpart to this budget doctrine is the published footprint page
(§bin/footprint): this section owns the budget rule, the page owns the measured
per-kit numbers, and the two never restate each other.

## The memory-off doctrine

Harness memory — the per-session store the harness offers to persist facts
across sessions — is an always-loaded surface the meter cannot read and no
gate scans: standing per-session context that accretes outside the tier
contract, ungoverned by construction. The methodology already routes durable
knowledge through a star topology, and those routes are the replacement —
durable facts to their doc owner (the knowledge-friction loop), iteration-
scoped attention to the lesson channels, private context to the operator's
local brief. So the kit disables harness memory and enforces it off rather
than governing its content.

The banned class is *harness-side silent accumulation*, not every local file: an
explicit, derived, operator-curated local file is config, not memory.
`ENV.local.md` (§bin/env-probe) sits with `BRIEF.local.md` on the config side of
that line — its probed half is a re-runnable derivation, its gotchas half is
hand-curated, and both are gitignored operator surfaces the operator chose to
keep, never a store the harness wrote to behind the tier contract.

Blast-radius honesty rides the doctrine: the gates hold the tree regardless of
a polluted session, so a memory that quietly re-accumulated degrades one
session's judgment, never the committed baselines. This is therefore a
lightweight gate pair — a hermetic pin and a local-environment scan — not
machinery. Enforcement splits on the tree-vs-environment seam: what a commit
can carry (the tracked settings file) is hermetic and CI-real
(§check-settings-pins); what only the operator's machine holds (the memory
dir, the untracked local settings) is a local-environment scan, CI-neutral
(§check-memory-off).

## check-settings-pins

`checks/check-settings-pins.gate` (hermetic, `precommit`, dispatched to the gate
binary — gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency)
is the identity.conf pattern pointed at harness config. Every pin in
`CONTEXT_KIT_SETTINGS_PINS`
(default `${GATE_SDK_GATES_DIR:-scripts}/settings-pins.conf`) holds against
the tracked settings file `CONTEXT_KIT_SETTINGS_FILE` (default
`.claude/settings.json`). Grammar: one `<path> = <expected JSON>` per line,
`#` comments and blanks ignored. General-purpose by construction — any
settings key is pinnable — this consumer's first pins hold the
auto-memory-disabling keys.

**The left-hand side is a path expression, not a `jq` filter**, and the grammar
is complete:

```
path   := '.' | step+
step   := '.' ident | '.' '"' string '"' | '[' '"' string '"' ']' | '[' int ']'
ident  := [A-Za-z_][A-Za-z0-9_]*
int    := '-'? [0-9]+
```

A path **opens with `.`**; everything else — pipes, filters, `?` error
suppression, slices, iteration, functions, arithmetic — is refused, and the gate
turns that refusal into **exit 2 naming the offending pin, the knob it came from,
and the construct**. The leading-`.` rule is load-bearing rather than tidy: `jq`
reads a leading `["k"]` as an array *literal* and returns `["k"]`, so admitting
one would answer a question `jq` was never asked. This is a narrowing of a
consumer config surface, taken openly and refusing loudly outside the subset — not
the silent mis-scan the ERE cohort's foreclosure forbids, which binds *sizing an
implementation* to one consumer's usage while the documented grammar stays wider.
The knob's whole documented job is naming a settings key, which a path expression
expresses in full. Operator-ruled 2026-08-14, with a filter crate (`jaq`) priced
against it and declined; a later reader must not reopen it on the observation that
such a crate exists, because that observation is what the ruling was made against.

**The right-hand side is a JSON value, and the comparison is structural** — the
`jq -c` *rendering* is not the contract. A byte-rendering contract is unachievable
across `jq` versions and therefore cannot be a parity target: `jq` 1.6 re-renders
every number through a double where 1.7 preserves an unmutated literal, so
`{"x":1e3}` renders `1000` under one and `1e3` under the other, and a crate
holding byte parity with "`jq -c`" would be holding parity with whichever `jq` the
comparison ran against. Structural comparison has no such dependency and diverges
from the shell only in the forgiving direction — an expected side written with
non-canonical spacing now matches. Part of what *structural* means is one explicit
rule the object model does not give free: **numbers compare by their `f64` value
wherever they occur, every other shape by the parsed value's own equality**. The
parser's own equality separates `1` from `1.0` by variant where `jq` calls them
one value, and `jq`'s equality is not shallow, so a pin nested one level deep
would otherwise report a mismatch nobody would think to test for.

Two semantics are **preserved deliberately rather than improved**, because a port
proves parity and does not repair rules. A path evaluating to `null` is the
**absent** branch, whether the key is absent or explicitly `null`: the shell
cannot tell the two apart and the compiled form can, so reproducing the
conflation is the faithful port, and a session wanting the distinction files an
entry against this section. And **indexing follows `jq`'s own type rules** — a
field step on `null` yields `null`; a field step on a string, number, boolean or
array, and an index step on an object, are errors the gate classifies as a
malformed pin.

Dispositions: a pin whose path resolves to the expected value passes; a path
present with a different value is the legible violation (exit 1, each finding
reading path, expected, and actual). Fail-closed (exit 2) on an unreadable or
non-JSON settings file, a malformed pin line, a pin outside the path grammar, or
a pin naming a key **absent** from the settings file — an absent key is a desynced
manifest (the pins and the settings are one repo's tracked config, edited
together), not the legible drift a red is for. Absent pins file: the opt-in-off
state, a clean skip. Ships a `good/`+`bad/` fixture pair and registers in the
consumer's `gates.list` (this repo's included).

**What a pin is worth depends on which tiers can outrank the file it pins**, and
the gate reads exactly one tier. The harness resolves settings across five, in
order: managed policy, then command-line, then the untracked local settings file,
then the tracked settings file this gate reads, then the operator's user-level
file. Objects deep-merge; the exceptions that merge rather than override are the
permissions arrays, MCP servers, HTTP hook URLs and `fallbackModel` chains. Two
consequences the gate's own verdict does not carry. A pin **does** beat the
operator's user-level file, which is what makes a tracked pin meaningful at all
rather than a suggestion the machine can quietly ignore. And a pin is **beaten**
by three tiers above it, of which `check-memory-off` covers one — the local
settings overlay — leaving managed policy and command-line flags outside any
gate's reach. That residue is unclosable here rather than merely unbuilt: neither
tier leaves an artifact in the tree, and the command line does not exist until
the session starts.

## check-settings-paths

`checks/check-settings-paths.gate` (hermetic, `precommit`, dispatched to the gate
binary alongside its sibling — gate-sdk/SPEC.md §The settings cohort, and the
crate's first dependency) holds the second
invariant over the same tracked file: every entry in
`CONTEXT_KIT_SETTINGS_FILE`'s `permissions.allow[]` whose command token is a
**literal** repo-relative `.sh` path resolves in the working tree. The knob is
reused, not introduced — §check-settings-pins already owns it and its default.

The class it defends is a standing property of a gate port rather than a
one-time count: replacing a kit's `checks/<gate>.sh` with a `<gate>.gate`
descriptor strands every allow entry naming the old path, and this repo
accumulated such entries one cohort apart before the gate existed. A dead grant
breaks nothing — no command can reach it — so the cost is a permission roster
whose every reader must re-verify which lines still mean anything.

**The extraction predicate**, scoped against the shapes a permission array
actually carries. A candidate is taken only from an entry of the form
`Bash(<command>)`; leading `env NAME=VALUE` assignments and a `bash`/`sh`
interpreter word are skipped, and the first remaining token is the candidate. A
candidate not ending in `.sh` is out of scope — a bare command names no path,
and a non-`.sh` path entry (a log truncation, a scratch removal) names a
runtime-created gitignored path whose absence is its ordinary state, so
existence is the wrong predicate for it. A candidate **containing `*`** is a
pattern, intentionally polymorphic over files that need not exist today, and is
skipped. That rule is scoped to the *command token*, not the entry: this repo's
standing shape for a grant taking arguments is a bare entry beside a
`*`-suffixed twin, and the twin's path is as literal — and as strandable — as
the bare form's, so it stays in scope.

Splitting the grant into tokens must not expand it. Globbing a pattern grant
against the tree and then asserting an arbitrary first match would green the whole
pattern class instead of skipping it, while leaving the checked count silently
inflated. The shell form protected that with `read -ra`, which does not expand;
the compiled form splits on ASCII whitespace, which has no expansion to suppress.
The property is the same and it is the property, not the idiom, that the fixture
pair's checked count pins.

Dispositions: exit 1 lists each violating entry verbatim beside the path that
did not resolve, so the reader can repoint or drop the grant without re-deriving
which token was read. The clean line reports the **checked count**, which is
what distinguishes a predicate that scoped to the array from one that vacuously
matched nothing. Fail-closed (exit 2) on a settings file that is
unreadable or not JSON — the sibling gate reads the same file on the same terms,
and the file is this gate's sole subject rather than an opt-in manifest, so
there is no absent-surface skip to grant. A `--fixture <dir>` mode reads
`<dir>/settings.json` and resolves candidates against `<dir>`, which is the
hermetic mode the `good/`+`bad/` fixture pair drives.

The pair pins the scoping, not merely the verdict. `good/` carries every
skipped shape — pattern tokens, bare non-path commands, non-`.sh` paths —
alongside resolving literals in all three extraction shapes (bare,
`env`-prefixed, trailing-flag), and its expectation pins the **checked count**:
the pattern-expansion defect above passes an exit-code-only fixture, because an
expanded pattern resolves by construction, and is visible only in the count.
`bad/` carries a dead path in the bare, `*`-twin and `env`-prefixed shapes
beside one resolving grant, so a broken extraction arm shows as a missing
finding rather than a still-red exit.

Its `# graph:` manifest names the settings file as its subject and the check
script globs as a **reverse trigger** — a cohort deleting a ported gate's `.sh`
is exactly the edit that strands a grant, so it must re-run the gate. The
distinction is recorded because gate-sdk/SPEC.md's port criterion 4 turns on it,
and a later port reading that couple as content would misclassify the member.
The trigger is a partial route by construction: the generated hook reads staged
`ACMR` paths, so a *deleted* `.sh` never matches it. What the trigger catches is
the ordinary edit that strands a grant; what catches the cohort is the full
battery, which runs whole-tree with no trigger filter.

**The criterion-7 debt this gate landed carrying is paid, and this paragraph is
the record that it was.** `jq` is not on `GATE_SDK_PROGRAM_FLOOR`, so the gate
failed port criterion 7 the day it landed and owed designed-away work at its port.
That work is the settings cohort (gate-sdk/SPEC.md §The settings cohort, and the
crate's first dependency), which retired the requirement for this member and its
sibling by taking a JSON reader into the crate rather than by hand-rolling a
parser to dodge a dependency — the outcome this paragraph predicted when it said
one parsing story for the settings file is worth more than the criterion. `jq`
remains required by `check-installer-no-deps` alone (excluded with cause) now that
`check-memory-off` has taken the same reader, so it is retired from the battery
but for that one member, and not from the shipped install path at all.

**The gate is not the prune.** It reads the settings file and writes nothing,
but it reds the moment it registers on a tree carrying stranded entries, and the
settings file is operator-owned configuration a session may not edit. The
landing order is therefore fixed: the operator prunes, then the gate registers.

## check-memory-off

`checks/check-memory-off.gate` (local-environment class, binary-dispatched, the
check-identity precedent) scans the operator's machine, not the tree — its
`# graph:` manifest couples the pins file it reads and triggers on `*`, because
the surfaces it guards (the memory dir and the untracked local settings) never
stage. Two red conditions:

- the harness's per-project memory dir holds content — any regular file that
  is not the dir-preserving `.gitkeep`;
- the untracked local settings file (`settings.local.json` beside the settings
  file) sets a pinned key to a value other than its pin — the override the
  hermetic gate cannot see, since it reads only the tracked file.

`CONTEXT_KIT_MEMORY_DIRS` (a space-separated glob list) names the dirs to
scan; its default derives the current project's dir from the harness layout
(§Layout and configuration). CI-neutral: where the surface is absent the gate
is clean, and the clean line states the fail-open caveat — an absent dir
proves nothing about another clone.

**It has one arm, and the knobs are it.** The `--fixture <dir>` arm the shell
form carried is deleted: every path it redirected — the scanned dirs, the pins
manifest, the local settings file derived from `CONTEXT_KIT_SETTINGS_FILE` by
swapping `.json` for `.local.json` — a knob already redirects, so the arm bought
a shorter spelling and paid for it with a second code path that never drove the
derivation being checked. The pair and `check-memory-off.test.sh` reach the gate
through the three knobs instead, which is what makes them a parity oracle for the
live arm rather than for a fixture-only one.

**Comparison is structural and a null actual is a skip**, and the two are
recorded because they part company with `check-settings-pins` over one manifest.
The pins file declares *expected JSON*, not an expected byte form, so `1` and
`1.0` are one value here as they already are there; a right-hand side that is not
JSON cannot be compared structurally at all and is skipped on this member's one
disposition for a pin it cannot read — that same line of that same manifest is
what the sibling gate fail-closes on, so the condition is graded, not lost. A
path evaluating to **null**, by contrast, is this gate's ordinary clean case: the
local file simply sets no override for that key. `check-settings-pins` reads a
null as an absent pin and refuses, which is right for the tracked file it reads
and would be a correctness regression here, reddening every clone whose local
settings merely omit a pinned key.

Fail-closed (exit 2) when it cannot read what is present to check: a local
settings file that is unreadable or not valid JSON, a memory dir it cannot walk,
or a `HOME` it cannot read when the default derivation is the one in play.

## check-footprint-fresh

`checks/check-footprint-fresh.gate` (hermetic, `precommit`) byte-compares the
committed `docs/footprint.md` against the footprint emitter, the
`check-docs-mirror-fresh`/`check-trajectory-fresh` posture: a generated,
freshness-gated projection is Derivation-first's sanctioned copy, so the
maintainer re-runs the emitter after any change to a measured surface and a
stale page reddens the battery. Its `# graph:` manifest couples the measured
surfaces — the configured agent file and each kit's `templates/` tree — so an
edit to what the page counts re-fires the gate.

**It is a registry member of the gate binary, and its emitter is a function call
rather than a spawn.** The comparator and the emitter ported in one unit, so
where the shell form ran `bash <emitter> --emit` in a subprocess, the compiled
member calls the emitter module's `emit()` **in-process** — which is what
retires the family's `bash` hop for this member (gate-sdk/SPEC.md §The first
cohort, and the rule that selects the next). The `CONTEXT_KIT_SURFACES` the
emitter reads arrives across the config bridge, declared by this member.

Bare, it runs the live emitter; a two-argument form
(`check-footprint-fresh <projection> <emit>`) compares two pre-baked files,
the hermetic mode the `good/`+`bad/` fixture pair drives. Fail-closed (exit 2)
on a missing projection or emit source; the stale byte-compare is the exit-1
violation. The page's generated numbers ride the `docs/evidence-data.md`
precedent past the prose gates on content, not a named valve — the figures live
in table cells the count gate does not read as prose, and the method prose names
no bare collection total.

## Layout and configuration

```
context-kit/
  lib/context.sh                 # sourced config loader + the kit's knob defaults; the config bridge sources it
  lib/toolfloor.sh               # sourceable owner: the probe roster + the floor predicate
  bin/always-loaded.sh
  bin/run-index-tests.sh         # expected-output runner for the index-first tools
  checks/check-brevity.gate      # hermetic, binary-dispatched: the budgeted section's over-budget pointer bullets
  checks/check-settings-pins.gate  # hermetic, binary-dispatched: pins hold against the settings file
  checks/check-settings-paths.gate # hermetic, binary-dispatched: literal .sh grants resolve in the tree
  checks/check-memory-off.gate   # local-environment, binary-dispatched: memory dir + local overrides
  checks/check-footprint-fresh.gate # hermetic, binary-dispatched: docs/footprint.md byte-fresh vs the emitter it calls in-process
  gate-tests/check-brevity/{good,bad}/
  gate-tests/check-settings-pins/{good,bad}/
  gate-tests/check-settings-paths/{good,bad}/
  gate-tests/check-memory-off/{good,bad}/
  gate-tests/check-footprint-fresh/{good,bad}/
  gate-tests/check-brevity.test.sh      # the unmatched-section axis the pair cannot hold
  gate-tests/check-memory-off.test.sh   # the local-override axis the pair cannot hold
  gate-tests/check-settings-pins.test.sh # the refusal axis the pair cannot hold
  gate-tests/toolfloor-parity.test.sh   # the floor predicate's two holders, classification compared
  index-tests/                   # fixture corpus + expected outputs
  templates/session-context.sh   # consumer copy: marked consumer sections
  templates/settings-sessionstart.json
  templates/context-config.sh
  templates/close-brevity.md
  smoke/install.sh
  smoke/violation.sh
  smoke/agents-md.sh             # the AGENTS.md adapter smoke (its own validate suite)
```

**The one library member beside `lib/context.sh` is owed to the port** —
gate-sdk/SPEC.md §The kit-library port disposition does not reach it, so the
silence here is not an undecided class. `lib/toolfloor.sh` rides the config
bridge's `lib/*.sh` glob and resolves no bridged knob (`PROBE_SET` carries no kit
prefix); what sequences it is that its roster is read on the **installer** path
and by `check-install-toolchain`'s parity assertion, so it moves behind the
installer's own behind-invoke relocation, whose live entry is
`powershell-installer-surface`. The entry that owns its port is
`kit-library-port-residue`. The `lib/pub-lang/` extractors that sat beside it are
**discharged**: they were the bundled members of the registry `pub-index`
resolves, and they moved in-crate behind the surviving seam in the cut that
ported that resolver (§Index-first reading).

The install also seeds the committed baseline the footprint contract holds
(§The consumer footprint): after wiring the hook it runs
`always-loaded.sh --update-baseline` once to write
`always-loaded-baseline.txt`, and `smoke/install.sh` asserts that step by
running the meter and checking the baseline lands. Install also seeds the local
env profile — `run-gates.sh --emit env-probe` writes the first `ENV.local.md`
block (§bin/env-probe); being an operator-local, gitignored surface, no smoke
asserts it (the stated install step is its enforcement).

Config follows the established kit pattern: copy
`templates/context-config.sh` into the gates dir (or point
`CONTEXT_KIT_CONFIG_FILE` elsewhere) and override any knob; defaults fill
what the consumer left unset, and a set-but-missing `CONTEXT_KIT_CONFIG_FILE`
exits 2 rather than silently running on defaults. That template and the
`<gates-dir>/context-config.sh` it seeds are **permanently shell**, each
carrying the `# no-port:` cause of the class ruling at gate-sdk/SPEC.md §The
config-seam port disposition. Knobs (this repo's layout as defaults):

### lib/context.sh

**The one home of the consumer-config load and of every knob default above**, a
sourceable library on queue-kit's `lib/queue.sh` shape: it loads the consumer
config first, then defaults each knob the consumer left unset, then refuses a
malformed value rather than running on it. Every context-kit gate and `bin/` tool
sources it instead of re-defaulting, which is the single-home form
`check-knob-default-coupling` asserts and what keeps a shell-side and a bridged
value one value rather than two.

**The config bridge is what forces a library rather than a convention.**
gate-sdk's config bridge resolves each declared knob by sourcing one kit's
library in a subshell, and **exits 2 on a knob that library does not define**
(gate-sdk/SPEC.md §lib/gate.sh), so a
`.gate`-dispatched member whose knobs were defaulted inside a check script would
resolve none of them. That mechanism is indifferent to what any crate links: a
member receives its knobs this way whatever the binary carries.

**Which is also why it is permanently shell, and it declares so in its own
header**: being the bridge's sole resolver for the `CONTEXT_KIT_*` knobs is
exactly the property above, read as a port disposition — gate-sdk/SPEC.md §The
kit-library port disposition rules the class. The ruling reaches this file alone
among the kit's libraries; `lib/toolfloor.sh` is owed and its sequencing is below.

**Every default here is repo-relative, and that is a bridge requirement rather
than a style.** A bridged value is baked verbatim into the tracked pre-commit
hook, so an absolute path would pin one clone's layout into a committed artifact.
The one knob whose natural value *is* absolute — `CONTEXT_KIT_MEMORY_DIRS`,
naming a harness directory under `HOME` — defaults to **empty**, and the empty
value means "derive it" rather than "no dir": the derivation belongs to the one
member that reads the knob (`native/src/gates/memory_off.rs`), which folds `/`
and `.` to `-` in the repo toplevel exactly as the harness names each project's
dir. Deriving it lazily at that reader is also what keeps a `git` subprocess off
the path of every unrelated knob resolution. The shell library once carried the
derivation as `context_memory_dir_default()`; it left with the shell gate that
was its only caller, so the layout rule has one implementation rather than two
agreeing ones.

**Empty-means-derive has a second member, and the ground is the same one read
from a different direction.** `CONTEXT_KIT_PUB_LANGS` defaults to empty too, and
its reason is not that its value is absolute but that no repo-relative literal
can express it at all: the roster it names is the `pub-index` arm's built-in
extractor set, which lives in the crate. Transcribing that set into shell would
be the maintained list derivation-first forbids and a second producer of one
roster, so the expansion belongs to the arm — the one reader of the knob — and
the library states the default and stops. **Its sibling `CONTEXT_KIT_PUB_LANG_DIR`
moved here verbatim** from the deleted dispatcher that held it inline, in the
commit that deleted it, so the documented default and the supplying site became
one string; the rule that forced the move is gate-sdk/SPEC.md §The non-gate arm's,
and it is load-bearing rather than tidy — the bridge resolves a declared knob by
sourcing exactly this library and **exits 2 on a knob it does not define**, so a
default left beside the compiled reader would refuse the whole arm.

- `CONTEXT_KIT_SURFACES` — array of always-loaded files; default
  `("CLAUDE.md")`. The measured surface is agent-file-name-agnostic: a consumer
  whose harness reads `AGENTS.md` (or any other always-loaded agent file) sets
  this to that file and the meter, the footprint, and `check-brevity` all follow
  — no kit mechanism resolves the agent file by literal. The
  `smoke/agents-md.sh` adapter smoke exercises exactly that: an `AGENTS.md`
  scratch consumer whose battery is green and whose meter and footprint measure
  `AGENTS.md`.
- `CONTEXT_KIT_PUB_LANGS` — array naming the `pub-index` extractors to enable;
  default **empty**, and empty means *derive it* rather than *no languages*: the
  one reader of the knob expands it to the arm's built-in extractor roster
  (§lib/context.sh, the `CONTEXT_KIT_MEMORY_DIRS` shape). Setting it to an
  explicitly empty array is therefore the same input as leaving it unset, which
  is the one behaviour the port collapsed — no shipped sentence promised that
  spelling as a way to disable the tool, and a sentinel meaning "none" would mint
  vocabulary for a use nobody has.
- `CONTEXT_KIT_PRUNE_DIRS` — array of **leaf basenames** both index walkers
  exclude from their `find` (§Index-first reading); default the union of what the
  two tools carried privately plus the harness worktree leaf — `.git`,
  `node_modules`, `target`, `dist`, `build`, `worktrees`. Deliberately not
  gate-sdk's set and deliberately not derived from it.
- `CONTEXT_KIT_PUB_LANG_DIR` — the consumer extractor dir searched before the
  arm's built-in roster (a same-basename file shadows the shipped grammar);
  default `${GATE_SDK_GATES_DIR:-scripts}/pub-lang`, supplied by
  §lib/context.sh — the documented default and the supplying site are one string.
- `CONTEXT_KIT_HOOK_CMD` — command whose output line count approximates
  the steady-state hook body; default queue-kit's
  `run-gates.sh --emit queue-index --collapse-deferred` when resolvable, else empty
  (surfaces only).
- `CONTEXT_KIT_DRIFT_REPORT` — the **`--emit` arm name** of the consumer's
  drift report, not a path: the hook runs
  `run-gates.sh --emit <name> --trend` for the brief's drift line; default
  empty (the line is omitted), and this repo's own copy sets `drift-report`.
  **It was a script path until 2026-08-29**, and the change is called out
  because the guard changed with it: a `-f` existence test on an arm name
  passes for nothing, so a hook that kept testing the value as a path would
  have dropped the drift line with no red anywhere. A consumer whose config
  still holds the old path degrades to no drift line, which is the same
  degrade an absent report always had.
- `CONTEXT_KIT_STAGE_RULES` — the **command** that emits stage→craft-rule
  pointers, not a path: the session-context hook runs it with the current stage
  appended for the brief's craft-rule block, and carries no `-f` guard. Default
  empty (the block is omitted); this repo's own copy sets doctrine-kit's
  `--emit stage-rules` invocation (doctrine-kit/SPEC.md §stage-rules).
  **It was a path to a bash script until 2026-09-03**, widened when that emitter
  ported to a compiled arm — a command a consumer can still point anywhere,
  which is what porting a knob's *value* leaves intact. The honest limit, stated
  because the migration is not free for everyone: a consumer whose config holds a
  bare **executable** script path keeps working, a command of one word; a
  consumer whose config holds a path to a **non-executable** script — which is
  what this repo's own default was, run under an explicit `bash` — stops working,
  because the `bash` prefix the hook used to supply is gone. That is one line of
  config to migrate, and it is what lets the knob name a compiled arm at all.
  **The knob is not renamed to `…_CMD`**: a rename costs every consumer a config
  edit for a contract that widens rather than changes subject, and strands its
  citations in two SPECs and a template.
- `CONTEXT_KIT_STATE_FILE` — the lifecycle evidence file whose **last data
  line** carries the stage cursor the hook routes on (§The session-context
  hook); default `${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt`. Read
  as a named file, never through stdin — the session-role signal consumes stdin
  exactly once, and a second reader there would starve it.
- `CONTEXT_KIT_ENV_PROFILE_FILE` — the consumer-local env profile file the
  env-probe arm writes and the session-context hook's step 9 emits (§bin/
  env-probe); default `ENV.local.md`.
- `CONTEXT_KIT_SESSION_ROLE_FILE` — the session-role marker `/lead` writes and
  the session-context hook's identity match reads (§The session-context hook);
  default `${GATE_SDK_TMP_DIR:-.tmp}/session-role` (gitignored scratch).
- `CONTEXT_KIT_BASELINE_FILE` — default
  `${GATE_SDK_WORKFLOW_DIR:-.workflow}/always-loaded-baseline.txt`.
- `CONTEXT_KIT_BREVITY_FILE` — default `CLAUDE.md`.
- `CONTEXT_KIT_BREVITY_SECTION` — heading of the budgeted bullet section;
  default `## Shared conventions`.
- `CONTEXT_KIT_BREVITY_BUDGET` — lines per bullet; default `4`.
- `CONTEXT_KIT_BREVITY_POINTER_RE` — the "cites a deeper doc" pattern;
  default `§`.
- `CONTEXT_KIT_SETTINGS_FILE` — the tracked harness settings file
  check-settings-pins and check-settings-paths each verify, on a different
  invariant, and whose `.local.json` sibling check-memory-off scans; default
  `.claude/settings.json`. **Explicitly setting it to a path that does not exist
  is refused (exit 2) by `lib/context.sh` at resolution time**, while leaving it
  unset and having no file at the default path is not-adopted and degrades at
  each reader. The refusal lives in the library because that is the last place
  *set-ness* is visible: once the value crosses gate-sdk's config bridge a
  compiled reader sees one path string and cannot tell the misconfigured case
  from the unadopted one. Emptiness is **not** the signal here — the config
  validation below rejects an empty value as malformed, an invariant older than
  the bridge — which is why this knob refuses where `DRIFT_KIT_KPIS_FILE`
  resolves empty (drift-kit/SPEC.md §lib/drift.sh).
- `CONTEXT_KIT_SETTINGS_PINS` — the pins manifest; default
  `${GATE_SDK_GATES_DIR:-scripts}/settings-pins.conf`.
- `CONTEXT_KIT_MEMORY_DIRS` — space-separated glob list of harness memory dirs
  check-memory-off scans; default the current project's dir under the operator's
  home, `$HOME/.claude/projects/<slug>/memory`, where `<slug>` is the project's
  absolute path with every `/` and `.` folded to `-` (the harness's own
  encoding). A knob because the layout moves (the plugin-marketplace ruling:
  design against the live layout, keep it config).

The hook template itself is consumer-edited rather than knob-driven (the
guard-kit guard precedent): its variation points are layout judgment,
and a template with a dozen knobs is harder to own than a marked section.

## Testing

The three index arms and the meter are advisory and speak plain text, so
the gate contracts do not fit; the kit ships an expected-output runner
instead: `index-tests/` holds a small fixture corpus (Markdown with nested
headings, fences, and link-bearing first sentences; Rust with the pub-item
kinds; TypeScript with every kind the `ts` grammar claims — including
`const enum` and `export default` — beside re-export and non-export lines it
must skip; a baseline file) beside expected outputs, and
`bin/run-index-tests.sh` drives each one through the `--emit` front-end over
that corpus and asserts exact output, failing on any diff. **The goldens are the
port's parity oracle and they are unusually strong**: they were produced by the
shell implementations the arms replaced, so holding them byte-for-byte is a
cross-substrate comparison over a committed corpus rather than an assertion of
parity. The runner reaches the arms through the front-end rather than the binary,
because that is what resolves the bridged environment two of them declare.
A consumer-shadowing case points `CONTEXT_KIT_PUB_LANG_DIR` at a scratch dir
whose `rust.sh` emits a marker row: it is the extractor seam's **end-to-end
proof**, the consumer-first resolution order and the `bash` spawn that executes a
consumer extractor both exercised, with the shadow's output rather than the
built-in grammar's recorded in the golden. **The runner itself stays shell**, and its ground is its own section rather than
the arms it drives: it declares §Testing, and that group is blocked as a whole by
`index-tests/toolfloor-cases.sh`, which exercises `lib/toolfloor.sh`'s floor
predicate — a library sequenced behind the installer's behind-invoke relocation.
The floor predicate rides the same runner rather than a fixture pair —
it is a sourced function, not a gate: `index-tests/toolfloor-cases.sh` sources
`lib/toolfloor.sh` and prints one line per (element, banner) pair, so the closed
verdict set, the spellings of an unconstrained member, and the
`uncomparable` fail-closed arm are asserted against a golden rather than assumed.
**That golden's scope is the shell holder alone**, and saying so is the point: a
compiled holder now exists (§bin/env-probe), and it is held to this one by
`gate-tests/toolfloor-parity.test.sh` rather than by this golden. Pointing the
compiled holder at the same file would make the verdict set a third copy, which
is what evidence-kit/SPEC.md §lib/evidence.sh's *no committed golden* rule
refuses for a two-holder comparison.
The audience axis is pinned in a second table in the same file, printing the
parsed field and the consumer-side predicate per element rather than a verdict,
because no verdict reads that field: its present, empty and omitted forms are
each a case, the emptiness rule being the part of the grammar a reader is
likeliest to get wrong.
The runner registers as its own evidence-kit validate suite
(`index_tests`, the `demo` precedent): the golden the refactor leans on now
has an automated validate-stage consumer. The footprint emitter is advisory the same way, but its
projection is gated rather than runner-tested: `check-footprint-fresh` byte-holds
`docs/footprint.md` against `--emit`. `check-brevity`, `check-settings-pins`,
`check-memory-off`, and `check-footprint-fresh` are gates and carry the standard
fixture pair; the footprint pair drives the hermetic two-argument mode
(`<projection> <emit>`), the `check-trajectory-fresh` precedent. Both
memory-off pairs reach their subject differently, and the difference is the
point: the settings-pins pair takes a `--fixture <dir>` injection (the
check-identity precedent) reading `<dir>/settings.json` against
`<dir>/settings-pins.conf`, while the memory-off pair drives
`CONTEXT_KIT_MEMORY_DIRS` from its own case-dir config, because that member's
fixture arm was deleted for being a code path its live arm never took.
The direct unit tests beside the pairs hold the axes those pairs fix and so
cannot express: `check-brevity.test.sh` holds the unmatched-section resolution (the
pair fixes `CONTEXT_KIT_BREVITY_SECTION` at the stock default and always
supplies a file carrying it, so neither case can express a section that
resolves to nothing — an unmatched section is exit 2, a broken machine rather
than a clean tree). The memory-off local-override axis — an untracked
`settings.local.json` that re-enables a pinned key past an empty dir — cannot
be a good/bad pair (the pair fixes the dir axis), so `check-memory-off.test.sh`
holds it. `check-settings-pins.test.sh` holds the **refusal** axis: the pair
fixes the holds-vs-mismatch axis (exit 0 vs exit 1) alone, so a pin outside
the documented path grammar — a jq filter, an iteration, a slice, an array
literal — needs its own case to prove the refusal is exit 2, loud and naming
the pin, the knob and the construct, never a silent clean verdict.

`smoke/install.sh` copies the templates into the scratch consumer (config
into the gates dir, hook wiring into the harness settings), runs the hook
end-to-end asserting it exits zero (and, when queue-kit is co-vendored,
emits the queue index — the installer assumes only gate-sdk, so the queue
integration is exercised only alongside queue-kit), and runs
`always-loaded.sh --update-baseline` asserting the baseline file appears.
`smoke/violation.sh` crafts an over-budget pointered bullet in the scratch
consumer's brevity file and asserts the battery reddens via
`check-brevity`. It inserts the bullet inside the budgeted section rather than
appending at end-of-file: a co-vendored kit may append a trailing section (the
doctrine-kit installer adds one), and an EOF-appended bullet would land outside
`check-brevity`'s scanned section and silently disarm the smoke.

Both of those scripts stay on the shell substrate permanently and carry
`# no-port:` saying so. The disposition is not this section's to argue: it is the
class ruling at gate-sdk/SPEC.md §Consumer smoke, *The port disposition*, which
reaches them by its **ground** rather than by its scope — that ruling's
stated-contract cut covers the recipes answering to §Consumer smoke, and these
two answer here, but its legs 2 and 3 hold of them identically. `smoke/agents-md.sh`
below is **not** a member and stays owed to the port: it is a standalone
validate-suite driver, not an install or violation recipe, and neither leg
reaches it.

`smoke/agents-md.sh` is the agent-file adapter smoke — the exercise behind the
Tier-two compatibility claim (docs/positioning.md §The tiered compatibility
claim). It vendors a scratch consumer through the shared consumer-smoke
mechanics (gate-sdk/SPEC.md §Consumer smoke), converts its agent file from
`CLAUDE.md` to `AGENTS.md`, sets the agent-file knobs in the consumer's config
seams (`GATE_SDK_AGENT_FILE`, `LIFECYCLE_KIT_AGENT_FILE`, `DOCTRINE_KIT_AGENT_FILE`,
`CANON_KIT_MANIFEST_FILES`, `CONTEXT_KIT_SURFACES`, `CONTEXT_KIT_BREVITY_FILE`),
then asserts the battery is green, `always-loaded.sh` and the footprint emitter
measure the `AGENTS.md` surface, and `check-root-tiering`'s built-in allowlist accepts
`AGENTS.md` at root while rejecting a stray second agent file. It is a standalone
harness — not driven by `run-consumer-smoke.sh`, which asserts the kit defaults
under zero config — and registers as its own evidence-kit validate suite
(`agents_md_smoke`, the `demo` precedent).

## Out of scope

Product-shaped indexes — a `proto-index` over a proto layout, a
`diagram-index` over architecture HTML — are consumer surfaces; a consumer
names its own extra indexes in the hook template's footer. `check-md-refs`
(an orientation-doc roster is rule content; the link-resolution mechanism is
unclaimed, not this kit's) and `check-md-sections` (a required-heading map is
rule content, and the queue surface it guards is already gated by queue-kit).
A close-stage harvest pipeline (`[pub]` lessons, publication paths) is
product workflow, not context mechanism. The drift report itself is
drift-kit's surface — only the always-loaded meter lives here. A consumer's
session-context content — its delegation nudge wording, component roster,
and extra index commands — stays in its own copied hook. Memory **content**
is out of scope by construction: the memory-off gates govern presence (the
dir stays empty) and pins (the disabling keys hold), never a live session's
context, which is not a scannable surface — a session polluted mid-flight is
caught by the tree the gates hold, not by reading the session.
