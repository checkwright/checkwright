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
   consumer has a drift report; silently absent otherwise (drift-kit owns
   the report; the seam is this optional line).
4. **Stage-conditioned nudges** — short reminders keyed on the current
   stage header (this repo's delegation nudge is the exemplar). Marked
   consumer section: which stages get which nudge is consumer judgment.
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
  its drift report), and the kit reads such a file unchanged.

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

`checks/check-settings-pins.sh` (hermetic, `precommit`) is the identity.conf
pattern pointed at harness config. Every pin in `CONTEXT_KIT_SETTINGS_PINS`
(default `${GATE_SDK_GATES_DIR:-scripts}/settings-pins.conf`) holds against
the tracked settings file `CONTEXT_KIT_SETTINGS_FILE` (default
`.claude/settings.json`). Grammar: one `<jq path> = <expected JSON>` per line,
`#` comments and blanks ignored; the expected side is the exact `jq -c`
rendering (`false`, `"1"`, `{"k":1}`). General-purpose by construction — any
settings key is pinnable — this consumer's first pins hold the
auto-memory-disabling keys.

Dispositions: a pin whose path resolves to the expected value passes; a path
present with a different value is the legible violation (exit 1, each finding
reading path, expected, and actual). Fail-closed (exit 2) on an unreadable or
non-JSON settings file, no `jq`, a malformed pin line, or a pin naming a key
**absent** from the settings file — an absent key is a desynced manifest (the
pins and the settings are one repo's tracked config, edited together), not the
legible drift a red is for. Absent pins file: the opt-in-off state, a clean
skip. Ships a `good/`+`bad/` fixture pair and registers in the consumer's
`gates.list` (this repo's included).

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

## Layout and configuration

```
context-kit/
  bin/md-index.sh
  bin/md-section.sh
  bin/pub-index.sh
  bin/always-loaded.sh
  bin/run-index-tests.sh         # expected-output runner for the bin tools
  checks/check-brevity.sh
  checks/check-settings-pins.sh  # hermetic: pins hold against the settings file
  checks/check-memory-off.sh     # local-environment: memory dir + local overrides
  gate-tests/check-brevity/{good,bad}/
  gate-tests/check-settings-pins/{good,bad}/
  gate-tests/check-memory-off/{good,bad}/
  gate-tests/check-memory-off.test.sh   # the local-override axis the pair cannot hold
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
what the consumer left unset. Knobs (this repo's layout as defaults):

- `CONTEXT_KIT_SURFACES` — array of always-loaded files; default
  `("CLAUDE.md")`.
- `CONTEXT_KIT_HOOK_CMD` — command whose output line count approximates
  the steady-state hook body; default queue-kit's
  `queue-index.sh --collapse-deferred` when resolvable, else empty
  (surfaces only).
- `CONTEXT_KIT_DRIFT_REPORT` — path to the consumer's drift-report script;
  the session-context hook runs it with `--trend` for the brief's drift
  line; default empty (the line is omitted).
- `CONTEXT_KIT_BASELINE_FILE` — default
  `${GATE_SDK_WORKFLOW_DIR:-.workflow}/always-loaded-baseline.txt`.
- `CONTEXT_KIT_BREVITY_FILE` — default `CLAUDE.md`.
- `CONTEXT_KIT_BREVITY_SECTION` — heading of the budgeted bullet section;
  default `## Shared conventions`.
- `CONTEXT_KIT_BREVITY_BUDGET` — lines per bullet; default `4`.
- `CONTEXT_KIT_BREVITY_POINTER_RE` — the "cites a deeper doc" pattern;
  default `§`.
- `CONTEXT_KIT_SETTINGS_FILE` — the tracked harness settings file
  check-settings-pins verifies and whose `.local.json` sibling check-memory-off
  scans; default `.claude/settings.json`.
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
kinds; a baseline file) beside expected outputs, and
`bin/run-index-tests.sh` runs each tool over the corpus and asserts exact
output, failing on any diff. `check-brevity`, `check-settings-pins`, and
`check-memory-off` are gates and carry the standard fixture pair. Both
memory-off gates take a `--fixture <dir>` injection (the check-identity
precedent): the settings-pins pair reads `<dir>/settings.json` against
`<dir>/settings-pins.conf`; the memory-off pair scans `<dir>/memory` for
content. The memory-off local-override axis — an untracked
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
`check-brevity`.

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
