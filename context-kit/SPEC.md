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

"Index, then read the one you need" — the pattern every tool serves. All
three are advisory `bin/` tools; none joins `gates.list`.

- **`md-index.sh [paths…]`** — compact structural index for Markdown:
  heading hierarchy with line numbers, each heading followed by its
  section's first sentence, plus a per-file line count (the cost signal —
  whether to read whole or by section). Defaults to the whole tree,
  skipping `.git/`, `node_modules/`, and build dirs.
- **`md-section.sh <file> <heading>`** — prints one section, from the
  matched heading to the next heading of the same or higher level. Match
  is case-insensitive and tolerates a leading `§` (so a spec citation
  pastes directly); headings inside fenced code blocks are not mistaken
  for structure. The companion: find the heading in the index, extract
  just that body.
- **`pub-index.sh [paths…]`** — compact public API surface: every public
  item with kind, name, and line, sorted by kind then name, in a per-file
  block headed by a count. It is a **dispatcher over per-language
  extractors**: the dispatcher owns traversal (the prune set is `.git`,
  `target`, `node_modules`, `dist`, `build`), the kind-then-name sort, and
  the row formatting (the index-tests goldens assert the exact shape); each
  extractor is a sourced bash file
  defining exactly two names — a `PUB_LANG_GLOBS` array (the find globs, e.g.
  `*.rs`) and a `pub_lang_extract <file>` function emitting unsorted
  `kind name lineno` rows. Two names, both read every run; no other
  extractor-file surface is contract.

  Extractors resolve registry-style (the `gates.list` consumer-first
  precedent): a shipped extractor lives at `lib/pub-lang/<lang>.sh`, and a
  consumer extractor of the same basename under `CONTEXT_KIT_PUB_LANG_DIR`
  (default `${GATE_SDK_GATES_DIR:-scripts}/pub-lang`) shadows it. The enabled
  set is `CONTEXT_KIT_PUB_LANGS`, whose default is every shipped extractor,
  derived from the `lib/pub-lang/` roster at run time — never maintained as a
  list (derivation-first). Both knobs are read through the consumer config
  seam (`context-config.sh`), which the tool sources like the meter and the
  footprint do.

  Two extractors ship: `rust.sh` (every `pub`/`pub(crate)`/`pub(super)` item —
  the grep-grade grammar the tool has always carried) and `ts.sh`
  (TypeScript: `export`-declared `function`/`class`/`interface`/`type`/`enum`/
  `const`/`let`/`var`, `const enum`, and `export default`, over `*.ts` and
  `*.tsx` with `.d.ts` included as public surface by construction). Both are
  grep-grade: re-exports (`export { x } from`) and multi-line declarations are
  stated honest limits, not parsed. The dispatcher shipped when demand named
  it — a second adopter's tree carries a TypeScript surface its Rust-only copy
  could not index — not before: an unrequested plugin framework would have
  been scaffolding, and AST/tree-sitter parsing is above the tool's grep-grade
  portability altitude (bare bash + coreutils, Tier one).

## The session-context hook (template)

`templates/session-context.sh` is a consumer copy (the `bash-guard.sh`
pattern): wired as the harness's session-start hook via
`templates/settings-sessionstart.json`, it assembles the session brief.
Every step is guarded and degrades silently — the hook never fails a
session. Steps, in order:

1. **Queue index** — via queue-kit's `queue-index.sh`, collapsing the
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
   reads the tier reads `queue-index.sh --icebox-candidates` rather than this
   index.
2. **Dirty-surface pre-run** — for each component with uncommitted
   changes, pre-run the matching surface index (default: `pub-index` over
   top-level dirs containing `src/`), so a resumed session's editing
   surface is already in context. Component detection and the index
   command live in a marked consumer section of the template — they are
   layout assumptions, not mechanism.
3. **Drift line** — one `drift-report --trend` summary line when the
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
   names a present emitter (doctrine-kit's `stage-rules.sh`), the current
   stage's craft-rule pointer block, so a stage session is reminded of the
   craft rules bearing on it before the matching action. Silently absent
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
   `bin/env-probe.sh` to re-probe it (output suppressed so no status line
   reaches the brief), then emits its whole body verbatim so the session adapts
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
`lead <id>` — `<id>` being `session-id.sh`'s value — to
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
it; and the producer inherits `session-id.sh`'s
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
what lifecycle-kit's `session-id.sh` computes, so the hook *could* inject the
canonical stamp id with no shell-out. The parity is top-level-only and holds
only while the harness sets `CLAUDE_CODE_SESSION_ID`: a subagent is handed its
*parent's* id in that variable, while `session-id.sh` deliberately derives the
subagent's own transcript id instead (its `CLAUDE_CODE_CHILD_SESSION` branch),
so the two quantities diverge there by design. The hook does not inject: lifecycle-kit owns its id derivation
end-to-end (the stage-entry ritual derives it via `session-id.sh`,
whatever invokes that script), and having the stage skills
read a context-kit-injected value would wire an upstream kit's protocol to
a downstream kit's hook for ergonomics only — the trust model gains
nothing, since `check-stage-evidence` already enforces that the stamped id
is current. A consumer may add a local informational echo; the template
ships none.

## bin/env-probe

`bin/env-probe.sh` derives a local machine profile so a session adapts to the
box it runs on — package manager, toolchain versions, absent tools — without
those machine facts ever landing in the public tree. It writes a
marker-bounded generated block (`<!-- context-kit:env:begin -->` /
`:end`, via gate-sdk's shared `inject_marker_block` helper) into the file named
by `CONTEXT_KIT_ENV_PROFILE_FILE` (default `ENV.local.md`), replacing an
existing block or appending a fresh one — but only when the probed content
actually changed (Cadence, below), so the block's probe date marks the last
real change, not the last run. The probed half is derivation-first — never
hand-maintained.

**What it probes.** OS/distro (`uname`, `/etc/os-release`); the package manager
(first present of an ordered detection walk over the known managers); each
roster member's version and its floor verdict (below); the absent-tools list
(roster members `command -v` cannot resolve); and the below-contract list. The
roster itself is owned by `lib/toolfloor.sh` and never restated here.

**The roster and its floor axis (`lib/toolfloor.sh`).** The roster lives in a
sourceable library rather than in the script, because `env-probe.sh` does its
work on execution: a second reader cannot obtain the roster by running it, which
is why the parity gate greps the array out of a file instead of sourcing it, and
why a reader that runs before any consumer file exists — an installer's `doctor`
reading its own payload copy — needs an owner it can source. The library defines
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
reader — `bin/env-probe.sh` — walks the roster whole and marks the audience
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
- `awk::GNU` — no version floor, one implementation constraint: the 3-arg
  `match()` in `check-gate-assertions`, whose dependency gate-sdk/SPEC.md
  §check-gate-assertions already owns.
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
(default: queue-kit's `queue-index.sh --collapse-deferred` when
resolvable). The approximation is deliberate: the meter must never run the
session-context hook itself — the hook emits this meter's own output line,
so self-measurement would recurse and inflate.

The meter lives here, not in `drift-report.sh`, because the *metric* is
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

`checks/check-brevity.sh` — a section-agnostic name: the governed section
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
remains required by `check-memory-off` (held on criterion 2) and
`check-installer-no-deps` (excluded with cause), so it is retired from the battery
for these two members and not from the battery outright, and not from the shipped
install path at all.

**The gate is not the prune.** It reads the settings file and writes nothing,
but it reds the moment it registers on a tree carrying stranded entries, and the
settings file is operator-owned configuration a session may not edit. The
landing order is therefore fixed: the operator prunes, then the gate registers.

## check-memory-off

`checks/check-memory-off.sh` (local-environment class, the check-identity
precedent) scans the operator's machine, not the tree — its `# graph:`
manifest couples the pins file it reads and triggers on `*`, because the
surfaces it guards (the memory dir and the untracked local settings) never
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
proves nothing about another clone. It fails closed only when it cannot read
what is present to check: a local settings file with no `jq`.

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
  bin/md-index.sh
  bin/md-section.sh
  bin/pub-index.sh               # dispatcher over the per-language extractors
  lib/context.sh                 # sourced config loader + the kit's knob defaults; the config bridge sources it
  lib/toolfloor.sh               # sourceable owner: the probe roster + the floor predicate
  lib/pub-lang/rust.sh           # shipped extractor: Rust public items
  lib/pub-lang/ts.sh             # shipped extractor: TypeScript export surface
  bin/always-loaded.sh
  bin/env-probe.sh               # derives the marker-bounded local env profile
  bin/run-index-tests.sh         # expected-output runner for the bin tools
  checks/check-brevity.sh
  checks/check-settings-pins.gate  # hermetic, binary-dispatched: pins hold against the settings file
  checks/check-settings-paths.gate # hermetic, binary-dispatched: literal .sh grants resolve in the tree
  checks/check-memory-off.sh     # local-environment: memory dir + local overrides
  checks/check-footprint-fresh.gate # hermetic, binary-dispatched: docs/footprint.md byte-fresh vs the emitter it calls in-process
  gate-tests/check-brevity/{good,bad}/
  gate-tests/check-settings-pins/{good,bad}/
  gate-tests/check-settings-paths/{good,bad}/
  gate-tests/check-memory-off/{good,bad}/
  gate-tests/check-footprint-fresh/{good,bad}/
  gate-tests/check-brevity.test.sh      # the unmatched-section axis the pair cannot hold
  gate-tests/check-memory-off.test.sh   # the local-override axis the pair cannot hold
  index-tests/                   # fixture corpus + expected outputs
  templates/session-context.sh   # consumer copy: marked consumer sections
  templates/settings-sessionstart.json
  templates/context-config.sh
  templates/close-brevity.md
  smoke/install.sh
  smoke/violation.sh
  smoke/agents-md.sh             # the AGENTS.md adapter smoke (its own validate suite)
```

The install also seeds the committed baseline the footprint contract holds
(§The consumer footprint): after wiring the hook it runs
`always-loaded.sh --update-baseline` once to write
`always-loaded-baseline.txt`, and `smoke/install.sh` asserts that step by
running the meter and checking the baseline lands. Install also seeds the local
env profile — `env-probe.sh` writes the first `ENV.local.md` block
(§bin/env-probe); being an operator-local, gitignored surface, no smoke asserts
it (the stated install step is its enforcement).

Config follows the established kit pattern: copy
`templates/context-config.sh` into the gates dir (or point
`CONTEXT_KIT_CONFIG_FILE` elsewhere) and override any knob; defaults fill
what the consumer left unset, and a set-but-missing `CONTEXT_KIT_CONFIG_FILE`
exits 2 rather than silently running on defaults. Knobs (this repo's layout
as defaults):

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

**Every default here is repo-relative, and that is a bridge requirement rather
than a style.** A bridged value is baked verbatim into the tracked pre-commit
hook, so an absolute path would pin one clone's layout into a committed artifact.
The one knob whose natural value *is* absolute — `CONTEXT_KIT_MEMORY_DIRS`,
naming a harness directory under `HOME` — defaults to empty and is derived
lazily by `context_memory_dir_default()` at its one reader, which also keeps a
`git` subprocess off the path of every unrelated knob resolution.

- `CONTEXT_KIT_SURFACES` — array of always-loaded files; default
  `("CLAUDE.md")`. The measured surface is agent-file-name-agnostic: a consumer
  whose harness reads `AGENTS.md` (or any other always-loaded agent file) sets
  this to that file and the meter, the footprint, and `check-brevity` all follow
  — no kit mechanism resolves the agent file by literal. The
  `smoke/agents-md.sh` adapter smoke exercises exactly that: an `AGENTS.md`
  scratch consumer whose battery is green and whose meter and footprint measure
  `AGENTS.md`.
- `CONTEXT_KIT_PUB_LANGS` — array naming the `pub-index` extractors to
  enable; default every shipped extractor, derived from the `lib/pub-lang/`
  roster at run time (never a maintained list).
- `CONTEXT_KIT_PUB_LANG_DIR` — the consumer extractor dir searched before the
  kit's `lib/pub-lang/` (a same-basename file shadows the shipped one);
  default `${GATE_SDK_GATES_DIR:-scripts}/pub-lang`.
- `CONTEXT_KIT_HOOK_CMD` — command whose output line count approximates
  the steady-state hook body; default queue-kit's
  `queue-index.sh --collapse-deferred` when resolvable, else empty
  (surfaces only).
- `CONTEXT_KIT_DRIFT_REPORT` — path to the consumer's drift-report script;
  the session-context hook runs it with `--trend` for the brief's drift
  line; default empty (the line is omitted).
- `CONTEXT_KIT_STAGE_RULES` — path to a stage→craft-rule pointer emitter
  (doctrine-kit's `stage-rules.sh`); the session-context hook runs it with the
  current stage for the brief's craft-rule block; default empty (the block is
  omitted).
- `CONTEXT_KIT_STATE_FILE` — the lifecycle evidence file whose **last data
  line** carries the stage cursor the hook routes on (§The session-context
  hook); default `${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt`. Read
  as a named file, never through stdin — the session-role signal consumes stdin
  exactly once, and a second reader there would starve it.
- `CONTEXT_KIT_ENV_PROFILE_FILE` — the consumer-local env profile file
  `bin/env-probe.sh` writes and the session-context hook's step 9 emits (§bin/
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

The three index tools and the meter are advisory and speak plain text, so
the gate contracts do not fit; the kit ships an expected-output runner
instead: `index-tests/` holds a small fixture corpus (Markdown with nested
headings, fences, and link-bearing first sentences; Rust with the pub-item
kinds; TypeScript with every kind the `ts.sh` grammar claims — including
`const enum` and `export default` — beside re-export and non-export lines it
must skip; a baseline file) beside expected outputs, and
`bin/run-index-tests.sh` runs each tool over the corpus and asserts exact
output, failing on any diff. The Rust golden is the no-regression assertion
for the extractor-dispatcher refactor: it stays byte-identical across it. A
consumer-shadowing case points `CONTEXT_KIT_PUB_LANG_DIR` at a scratch dir
whose `rust.sh` emits a marker row, exercising the consumer-first resolution
order (the shadow's output, not the shipped grammar's, is what the golden
records). The floor predicate rides the same runner rather than a fixture pair —
it is a sourced function, not a gate: `index-tests/toolfloor-cases.sh` sources
`lib/toolfloor.sh` and prints one line per (element, banner) pair, so the closed
verdict set, the spellings of an unconstrained member, and the
`uncomparable` fail-closed arm are asserted against a golden rather than assumed.
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
memory-off gates take a `--fixture <dir>` injection (the check-identity
precedent): the settings-pins pair reads `<dir>/settings.json` against
`<dir>/settings-pins.conf`; the memory-off pair scans `<dir>/memory` for
content. Two direct unit tests hold the axes the pairs fix and so cannot
express: `check-brevity.test.sh` holds the unmatched-section resolution (the
pair fixes `CONTEXT_KIT_BREVITY_SECTION` at the stock default and always
supplies a file carrying it, so neither case can express a section that
resolves to nothing — an unmatched section is exit 2, a broken machine rather
than a clean tree). The memory-off local-override axis — an untracked
`settings.local.json` that re-enables a pinned key past an empty dir — cannot
be a good/bad pair (the pair fixes the dir axis), so `check-memory-off.test.sh`
holds it.

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
product workflow, not context mechanism. `drift-report.sh` itself is
drift-kit's surface — only the always-loaded KPI lives here. A consumer's
session-context content — its delegation nudge wording, component roster,
and extra index commands — stays in its own copied hook. Memory **content**
is out of scope by construction: the memory-off gates govern presence (the
dir stays empty) and pins (the disabling keys hold), never a live session's
context, which is not a scannable surface — a session polluted mid-flight is
caught by the tree the gates hold, not by reading the session.
