# context-kit — token-economics-aware context management

A stateless agent session pays for context twice. On-demand cost: opening a
large SPEC or source file whole when only one section was needed. Standing
cost: the always-loaded surface — everything injected into *every* session
(the instructions file, the session-start hook output) — where each added
line is a recurring per-session tax that grows silently, because no single
session ever sees the trend. The kit attacks both: an index-first reading
toolset, a session-start hook template that assembles a compact brief, a
meter that tracks the always-loaded surface against a per-iteration
baseline, a brevity gate over the densest always-loaded section, and the
close-stage brevity pass that reacts to the meter's delta.

Extracted from the governance meta-layer of a private production platform.
The kit carries the index mechanism, the hook skeleton, the meter, and the
brevity machinery; the platform's product-shaped surfaces (proto and
diagram indexes, its doc roster, its harvest pipeline) stay behind.

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
- **`pub-index.sh [paths…]`** — compact public API surface for Rust
  sources: every `pub`/`pub(crate)`/`pub(super)` item with kind, name, and
  line, sorted by kind then name. The one language-specific tool in the
  kit, and deliberately so: it ships as the Rust extractor, not a
  pluggable framework — a consumer in another language writes its own
  `<lang>-index` and names it in the hook template's marked section and
  index-reminder footer. A speculative plugin interface for extractors
  nobody has written yet would be scaffolding, not mechanism.

## The session-context hook (template)

`templates/session-context.sh` is a consumer copy (the `bash-guard.sh`
pattern): wired as the harness's session-start hook via
`templates/settings-sessionstart.json`, it assembles the session brief.
Every step is guarded and degrades silently — the hook never fails a
session. Steps, in order:

1. **Queue index** — via queue-kit's `queue-index.sh`, collapsing the
   Deferred section to a tally except on the scope stage: Deferred is
   unpickable and only scope (promotion) acts on it, so its full listing
   every other session is pure recurring cost.
2. **Dirty-surface pre-run** — for each component with uncommitted
   changes, pre-run the matching surface index (default: `pub-index` over
   top-level dirs containing `src/`), so a resumed session's editing
   surface is already in context. Component detection and the index
   command live in a marked consumer section of the template — they are
   layout assumptions, not mechanism.
3. **Drift line** — one `drift-report --trend` summary line when the
   consumer has a drift report; silently absent otherwise (drift-kit is a
   later extraction; the seam is this optional line).
4. **Stage-conditioned nudges** — short reminders keyed on the current
   stage header (the platform's delegation nudge is the exemplar). Marked
   consumer section: which stages get which nudge is consumer judgment.
5. **Scratch sweep** — reclaim `${GATE_SDK_TMP_DIR:-.tmp}` entries older
   than a day, depth-first (`-mindepth 1 -depth`) so stray directories are
   reclaimed too, never touching `.gitkeep`. Age-guarded so a concurrent
   same-checkout session's in-flight scratch survives.
6. **Index-reminder footer** — the "index first" ritual with the
   consumer's actual index commands listed (consumer-edited).

**Ruled out — lifecycle stamp-id injection.** The hook payload carries the
harness session id, and its 8-char prefix equals what lifecycle-kit's
`session-id.sh` computes, so the hook *could* inject the canonical stamp
id with no shell-out. It does not: lifecycle-kit owns its id derivation
end-to-end (the stage-entry ritual derives it via `session-id.sh`,
whatever invokes that script), and having the stage skills
read a context-kit-injected value would wire an upstream kit's protocol to
a downstream kit's hook for ergonomics only — the trust model gains
nothing, since `check-stage-evidence` already enforces that the stamped id
is current. A consumer may add a local informational echo; the template
ships none.

## The always-loaded meter

`bin/always-loaded.sh` measures the standing surface: the summed line
count of the configured surface files (default `CLAUDE.md`) plus the
steady-state hook body, approximated by the configured hook-body command
(default: queue-kit's `queue-index.sh --collapse-deferred` when
resolvable). The approximation is deliberate: the meter must never run the
session-context hook itself — the hook emits this meter's own output line,
so self-measurement would recurse and inflate.

On the platform this meter lived inline in `drift-report.sh` as one KPI;
it is extracted here because the *metric* is context economics and the
*report* is drift reporting — when drift-kit lands (kit 7) it consumes
this script for its always-loaded row instead of re-embedding the
measurement. New name on a governed surface: `always-loaded.sh` (the
platform had no standalone name to keep).

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
  fields are tolerated and preserved-ignored — the platform's file carries
  a fourth (its settings-local count, a guard-kit-adjacent KPI owned by
  its drift report), so the kit reads the platform's file unchanged.

## The brevity gate

`checks/check-brevity.sh` — renamed from the platform's
`check-convention-brevity`: the governed section is now a knob, so the
platform's section name leaves the gate name. It scans one designated
always-loaded file for a bulleted section where each `- **name:**` bullet
carries a line budget, and flags a bullet that is **over budget and cites
a deeper doc** (carries a `§` pointer) — over-long while admitting its
detail already has a home elsewhere. Under-budget bullets and over-budget
bullets with no pointer pass (the latter may genuinely own their content);
`<!-- brevity-exempt: <reason> -->` on the bullet's first line or the line
above blesses a bullet whose every line is load-bearing.

The pointer default widens from the platform's `HANDBOOK §` to any `§`:
"cites a deeper doc" is the mechanism-level meaning, the platform's
handbook was its instance, and the superset still matches every platform
pointer. Ships with a `good/`+`bad/` fixture pair and registers in the
consumer's `gates.list` (this repo's included).

## The close-stage brevity pass

`templates/close-brevity.md` is the recurring close-stage step a consumer
splices into its close skill (the guard-kit `close-triage.md` pattern).
The procedure: run `always-loaded.sh`; walk the growth since baseline
asking two distinct questions — staleness (*is it still true?*) and
brevity (*is each block worth its standing per-session token cost?*);
resolve by rewording or deleting, never by annotating (outdated context
goes to git history, not to a "formerly…" note); on-demand files (SPECs,
handbooks) are exempt — their cost is paid only when opened; finish with
`always-loaded.sh --update-baseline` and commit the baseline.

## Layout and configuration

```
context-kit/
  bin/md-index.sh
  bin/md-section.sh
  bin/pub-index.sh
  bin/always-loaded.sh
  bin/run-index-tests.sh         # expected-output runner for the bin tools
  checks/check-brevity.sh
  gate-tests/check-brevity/{good,bad}/
  index-tests/                   # fixture corpus + expected outputs
  templates/session-context.sh   # consumer copy: marked consumer sections
  templates/settings-sessionstart.json
  templates/context-config.sh
  templates/close-brevity.md
  smoke/install.sh
  smoke/violation.sh
```

Config follows the established kit pattern: copy
`templates/context-config.sh` into the gates dir (or point
`CONTEXT_KIT_CONFIG_FILE` elsewhere) and override any knob; defaults fill
what the consumer left unset. Knobs (platform values as defaults):

- `CONTEXT_KIT_SURFACES` — array of always-loaded files; default
  `("CLAUDE.md")`.
- `CONTEXT_KIT_HOOK_CMD` — command whose output line count approximates
  the steady-state hook body; default queue-kit's
  `queue-index.sh --collapse-deferred` when resolvable, else empty
  (surfaces only).
- `CONTEXT_KIT_BASELINE_FILE` — default
  `${GATE_SDK_WORKFLOW_DIR:-.workflow}/always-loaded-baseline.txt`.
- `CONTEXT_KIT_BREVITY_FILE` — default `CLAUDE.md`.
- `CONTEXT_KIT_BREVITY_SECTION` — heading of the budgeted bullet section;
  default `## Shared conventions`.
- `CONTEXT_KIT_BREVITY_BUDGET` — lines per bullet; default `4`.
- `CONTEXT_KIT_BREVITY_POINTER_RE` — the "cites a deeper doc" pattern;
  default `§`.

The hook template itself is consumer-edited rather than knob-driven (the
guard-kit guard precedent): its variation points are layout judgment,
and a template with a dozen knobs is harder to own than a marked section.

## Testing

The three index tools and the meter are advisory and speak plain text, so
the gate contracts do not fit; the kit ships an expected-output runner
instead: `index-tests/` holds a small fixture corpus (Markdown with nested
headings, fences, and link-bearing first sentences; Rust with the pub-item
kinds; a baseline file) beside expected outputs, and
`bin/run-index-tests.sh` runs each tool over the corpus and asserts exact
output, failing on any diff. `check-brevity` is a gate and carries the
standard fixture pair.

`smoke/install.sh` copies the templates into the scratch consumer (config
into the gates dir, hook wiring into the harness settings), runs the hook
end-to-end asserting it exits zero (and, when queue-kit is co-vendored,
emits the queue index — the installer assumes only gate-sdk, so the queue
integration is exercised only alongside queue-kit), and runs
`always-loaded.sh --update-baseline` asserting the baseline file appears.
`smoke/violation.sh` crafts an over-budget pointered bullet in the scratch
consumer's brevity file and asserts the battery reddens via
`check-brevity`.

## What stayed on the platform

`proto-index` and `diagram-index` — indexes over platform-shaped surfaces
(its proto layout, its two architecture HTML diagrams); a consumer names
its own extra indexes in the hook template's footer. `check-md-refs` (its
orientation-doc roster is rule content; the link-resolution mechanism is a
possible later extraction, not this kit's) and `check-md-sections` (its
required-heading map is rule content, and the queue surface it guards is
already gated by queue-kit here). The close-stage harvest pipeline
(`[pub]` lessons, publication paths) — product workflow, not context
mechanism. `drift-report.sh` itself remains kit 7's extraction; only its
always-loaded KPI moves now, and the platform's drift report keeps its
inline copy until the platform migrates. The platform's session-context
consumer content — its delegation nudge wording, component roster, and
extra index commands — stays in its own copied hook.
