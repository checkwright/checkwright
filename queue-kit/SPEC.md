# queue-kit — a git-native, agent-readable task tracker

One Markdown file is the tracker: sections are queues, bullets are tasks,
bold kebab-case slugs are the task handles, and square-bracket tags are the
state machine. The problem the kit solves: a coding agent selects work by
parsing, not by reading — so everything selection trusts (position, slugs,
tags) is grammar a gate can enforce, and everything a human writes freely
(task prose) is kept out of the parse path. Drift between what the prose says
and what a parser sees is the failure mode; all but one of the gates exist to
close one instance of it each.

The kit carries the queue grammar and its gates only; a consumer's section
names, wrap budget, and protocol vocabulary are config, with this repo's
layout as the defaults. Requires [gate-sdk](../gate-sdk/) (the gates follow
its four contracts and resolve through its registry).

## The queue format

The governed surface is one queue file (default `TASK-QUEUE.md`), structured
as `##` sections over column-0 bullets:

- **Active sections** (default `New Features`, `Technical Debt`) — the
  pickable queue, in work-order. Selection discipline: pick the first entry
  carrying no `[blocked-by:]` tag, in section order; do not invent work order.
- **The deferred section** (default `Deferred`) — parked tasks, excluded from
  selection; `###` subsections are presentation, not semantics. An entry's
  prose may carry a `Surfaced <date>` mark — an ungated convention recording
  when the premise was filed; drift-kit's deferred-age KPI is its reader.
  A deferred body is free prose, but four **ungated** bold-lead-in fields
  recur and are worth reaching for, each answering a question a later scope
  asks: `Deliverable` (what landing looks like), `Why [design-pending]` (what
  the open design actually is), `Cost while deferred` (the
  Gap-disposition rule's costing — the most-used of the four), and a
  closing `Filed <date> by <stage>` provenance line. None is required and no
  gate reads them; an entry uses the ones that carry weight for it.
- **The done section** (default `Done`) — one line per completed task, the
  bare slug only; prose about what happened lives in git history.
- Any other section (an iteration header, a lessons section) is outside the
  grammar and ignored by every gate except the file-wide hygiene axes.

An entry is a column-0 `- **slug** — prose…` bullet; continuation lines are
indented, never column 0. An *indented* bullet with a bold lead-in is a
sub-task (same grammar); with a plain or italic lead-in it is a prose note
and is left alone. Slugs match `[a-z0-9][a-z0-9-]*` in one global, unique
namespace across active + deferred + sub-tasks — a slug is the task's stable
handle for its whole life: `[blocked-by:]` references it and the done-section
line carries it verbatim. The slug grammar is kit mechanism, not config.

### The tag algebra

Tags are square-bracket literals with fixed spelling (mechanism, not config
— the one exception is the consumer-named harvest vocabulary below); every tag
sits on its bullet's **lead line** — the only line the parsing tools scan
(enforced by `check-tag-lead-line`).

Outside the queue file, living prose claims a task's queue membership with one
grammar: a **bold-code token** — `` **`<slug>`** `` whose token matches the
slug grammar — which `check-queue-slug-liveness` resolves against the live
slug set on the configured prose surfaces (§check-queue-slug-liveness).

- `[blocked-by: <slug>]` — the entry is unpickable until `<slug>` completes.
  Repeat per blocker. Must resolve to a live task (active or deferred — a
  deferred blocker stands; it is unbuilt); a blocker in the done section is a
  *stale* tag that must be removed, because the tag alone marks a task
  unpickable.
- `[design-pending]` — design-pending marker. queue-kit parses and displays it;
  the placement semantics (deferred-section-wide enforcement, promotion
  rules) are canon-kit's amendment lifecycle and land with that kit.
- `[spec: <file>]` — spec-ready pointer. Same split: syntax here, amendment
  semantics in canon-kit.
- `[drain-exempt: <reason>]` — drain-stage residue marker: the entry
  legitimately stays active into the drain stage (a drain-spanning feature
  whose remaining half *is* drain-stage work). Same split: syntax here;
  placement semantics — the drain-entry exemption, the non-empty-reason
  grammar, the successor-entry backstop — are lifecycle-kit's
  `check-stage-entry` assertion B. The Done move takes the tag with it
  (done entries drop their lifecycle tags; every reader scans live task
  sections only). The lead-line reason is a terse marker, not the audit
  trail: `check-tag-lead-line` pins the tag to the lead line while
  `check-queue-wrap` caps its width, so a long slug can squeeze the reason
  to a few columns — keep it a keyword and carry the full exemption
  rationale in the entry body.
- `[precondition-ok: <reason>]` — per-entry opt-out valve for
  `check-queue-prose-precondition`.
- `[roadmap: <horizon>/<track>]` — public-projection marker: the entry is
  curated onto the generated roadmap page (§bin/roadmap.sh), under `<horizon>`
  and labelled `<track>`. The tag's spelling is fixed mechanism; its two field
  *values* are drawn from the consumer-configured `QUEUE_KIT_HORIZONS` and
  `QUEUE_KIT_TRACKS` arrays — the same fixed-spelling/configured-values split
  `[spec:]` already carries, and for the same reason: a kit literal spelling one
  project's horizons would ship its roadmap posture as everyone's. One tag with
  two slash-joined fields rather than two tags, because the fields are never
  independently meaningful — a track with no horizon has no slot on the page and
  a horizon with no track has no label in it, so splitting the pair would invent
  two governed names where the domain has one. An entry carries at most one
  `[roadmap:]` tag; an untagged entry is simply not projected, the normal case.
  Placement is unconstrained by section — an active entry and a deferred entry
  may both be projected, because the projection is about public direction, not
  selection order. Field validity is `check-roadmap-fresh`'s assertion B.

A tagged entry also carries a **`roadmap-summary: <text>` declaration** — one
indented line in the entry body holding the single sentence the public page
prints. It is a declaration, not a tag, and the difference is the point: a tag is
bracketed and lead-line-scoped because its readers scan lead lines alone, while
this is read off a line of its own, the shape the `close-surface:` declaration
above already uses (and `QUEUE_KIT_PROSE_LEADS`' `Protocol:` token at column 0).
Spelling is fixed mechanism; `check-tag-lead-line` does not govern it, and it
cannot collide with the `[roadmap:` scan, which keys on the bracket.

The declaration is a **whitelist, and that is a privacy boundary before it is a
design one**. Only text an author explicitly marked is ever projected, so the
mechanism's failure mode is a thin page rather than published internal prose —
the direction a consumer whose queue is private and whose roadmap is not can
afford to fail in. The emitter reads the text verbatim, so no sentence heuristic
decides what a public page inherits. One line, not a block: an indented
declaration has roughly 81 columns behind it under the default wrap budget —
comfortably a sentence, and needing no block-end rule a reflow could silently
truncate. `check-roadmap-fresh`'s assertion C makes the tag and the declaration
mandatory for each other in both directions.

Two tags ride **Lessons Learned** entries — a lesson is a top-level bullet
under the fixed-spelling `## Lessons Learned` heading, and `bin/queue-index.sh`
plus `check-tag-lead-line` read that section's lead lines too. That section is
one of close's inbound triage surfaces, and its forcing function is the sibling
of the gap inbox's — the first-stage entry refuses while it holds entries — so it
declares itself on the close-surface roster here:

close-surface: TASK-QUEUE.md#Lessons-Learned forced=lifecycle-kit/SPEC.md §bin/enter-stage.sh

- `[attend]` — fixed spelling, kit mechanism (the inbound channel): the filing
  session marks a lesson as a live attention point for later sessions *of the
  same iteration*. `queue-index.sh` emits an attention block of `[attend]`
  lead lines (§bin/queue-index.sh); the injection dies at the iteration
  boundary because lifecycle-kit's first-stage entry refuses a non-empty
  Lessons section.
- **Harvest tags** — `QUEUE_KIT_LESSON_TAGS` (array of bare tag names, default
  empty) is the *one* configured exception to fixed spelling (the outbound
  channel): the tag names, their sinks, and their handling are consumer rule
  content — a kit literal carrying them would publish a private vocabulary
  (the provenance seam, `check-graph`/`graph-vocab` pattern). The close ritual
  streams a tagged entry's body through `bin/lesson-sink.sh <tag>`
  (§bin/lesson-sink.sh), which resolves the sink from `QUEUE_KIT_LESSON_SINKS`;
  the tracked close skill names that mechanism, never a sink value. queue-kit
  only parses the tag's placement.

## Layout and configuration

The kit is vendored beside gate-sdk (conventionally at `queue-kit/`); its
gates are registered in the consumer's `gates.list` by name and resolve
through gate-sdk's multi-kit path. `bin/queue-index.sh` is a tool, not a gate
(no `# graph:` manifest).

Config follows lifecycle-kit's pattern: copy `templates/queue-config.sh`
into the gates dir as `queue-config.sh` (or point `QUEUE_KIT_CONFIG_FILE`
elsewhere) and override any knob; defaults fill what the consumer left unset,
and the loader exits 2 on a malformed config — a broken grammar must not gate
anything. A gitignored `queue-config.local.sh` beside that file sources last
(§lib/queue.sh) — the home for a private value a tracked config cannot carry.
Knobs:

- `QUEUE_KIT_QUEUE_FILE` — default `${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}`;
  every gate also takes the file as `$1` (fixture capability).
- `QUEUE_KIT_ACTIVE_SECTIONS` — array, default
  `("New Features" "Technical Debt")`, order = selection order.
- `QUEUE_KIT_DEFERRED_SECTION` / `QUEUE_KIT_DONE_SECTION` — defaults
  `Deferred` / `Done`.
- `QUEUE_KIT_WRAP_BUDGET` — default `100` (`check-queue-wrap` gate floor).
- `QUEUE_KIT_PROSE_LEADS` — array of column-0 lead tokens exempt from the
  hygiene gate's no-prose axis, default `("Protocol:")`.
- `QUEUE_KIT_PROSE_SURFACE_GLOBS` — array of repo-root-relative globs naming
  the living-prose surfaces `check-queue-slug-liveness` scans, default empty
  (no surface makes queue claims until the consumer opts one in). Which pages
  claim queue membership is the consumer's editorial posture — a kit literal
  would presume a docs layout — so the kit ships none; a bold-code token on a
  named surface is then a checked membership claim (§check-queue-slug-liveness).
- `QUEUE_KIT_PRECONDITION_REGEX` — the forward-precondition trigger set for
  `check-queue-prose-precondition`, default = the shipped phrase set.
- `QUEUE_KIT_REQUIRED_SECTIONS` — array of `##` headings that must each appear
  exactly once (`check-queue-sections`), default = the iteration header
  (`Iteration:`, prefix-matched for its dynamic suffix) plus `New Features` /
  `Technical Debt` / `Deferred` / `Done` / `Lessons Learned`. A trailing `:`
  marks a prefix-matched heading; every other entry is matched exactly.
- `QUEUE_KIT_LESSON_TAGS` — array of bare harvest-tag names, default empty; the
  consumer-named outbound lesson vocabulary (§The tag algebra), read by
  `check-tag-lead-line` for placement. Names, sinks, and handling are consumer
  rule content — the kit ships none.
- `QUEUE_KIT_LESSON_SINKS` — associative array, harvest tag → sink command,
  default empty; read by `bin/lesson-sink.sh`, which owns resolution and the
  fail-open default (§bin/lesson-sink.sh). A private sink value belongs in the
  local overlay, not this tracked file.
- `QUEUE_KIT_ATTEND_CAP` — positive integer, default `3`; the maximum `[attend]`
  lead lines `bin/queue-index.sh` emits in its attention block before folding
  the rest into an overflow note.
- `QUEUE_KIT_HORIZONS` — ordered array of horizon names for the `[roadmap:]`
  tag's first field, default empty. The order is the projection's emitted
  section order.
- `QUEUE_KIT_TRACKS` — array of track labels for the tag's second field,
  default empty.
- `QUEUE_KIT_ROADMAP_FILE` — repo-relative path of the projection page, default
  empty: no page, and `check-roadmap-fresh` skips clean.
- `QUEUE_KIT_ROADMAP_MARKER` — the marker-block token, default `roadmap`,
  delimiting the generated span as `<!-- roadmap:begin -->` /
  `<!-- roadmap:end -->`. One knob with two readers — the `--write` splice and
  the gate's block locator — which must agree, so it resolves through this
  loader rather than being re-defaulted in each.

The roadmap vocabulary is configured as a **pair**: `QUEUE_KIT_HORIZONS` or
`QUEUE_KIT_TRACKS` set while the other is empty is malformed config and
`lib/queue.sh` exits 2, per the loader's broken-grammar contract. A
half-configured vocabulary would silently accept every value of the
unconfigured field. Both empty is the unconfigured default — a consumer that
publishes no roadmap gets a clean skip rather than a kit-shaped one.

Cross-kit note: lifecycle-kit's `LIFECYCLE_KIT_ACTIVE_SECTIONS` carries the same
default. The knobs are independent (either kit runs without the other); a
consumer renaming its active sections sets both.

## Per-component contracts

### lib/queue.sh

The sourced config loader plus shared adapters: the section-regex builders
the gates pass to awk (both sides of every section boundary must parse
identically — a shared adapter removes that drift axis), and the slug/tag
extraction helpers. Values and adapters only, never gate structure
(gate-sdk's `lib/gate.sh` rule).

The loader validates what it loads: an empty or non-numeric knob is a
broken grammar, and the roadmap vocabulary is validated as a pair (§Layout and
configuration) rather than per-array, because the failure the check exists to
stop is one array set alone. Validation failures are collected and reported
together, then exit 2.

`queue_roadmap_entries <queue-file>` is the single `[roadmap:]` and
`roadmap-summary:` parse, shared by `bin/roadmap.sh` and `check-roadmap-fresh` so
the emitter and the gate can never disagree about what an entry claims — the same
one-adapter rule the section regexes follow. It walks the live task sections in
queue order and prints one tab-separated line per entry carrying **either**
marking: the entry's `[roadmap:]` tag count, the raw field text, the slug, the
`roadmap-summary:` declaration count, and the declaration's text. Emitting on
either marking rather than on the tag alone is what lets assertion C see a dead
declaration — an entry the tag has fallen off is exactly the case a
tag-triggered walk cannot report. An untagged entry's field column prints as `-`
rather than empty, because a tab counts as IFS whitespace: a reader splitting on
it coalesces an empty column and silently shifts every field after it, so a
fixed-arity line must never carry one.

The loader sources the consumer config, then a `<config>.local.sh` overlay
beside it when present — last write wins. This is the tracked-name /
gitignored-value split (the `msg-patterns.local.list` precedent): a private
sink value tracked in `queue-config.sh` would itself be the leak, so it lands
in the gitignored overlay instead. The overlay is optional — its absence is
fail-open, not an error.

### bin/queue-index.sh

Compact surface of the queue for task selection — the iteration header line
if present, then every top-level active entry as a one-line title plus tags,
ready (`•`) vs blocked (`✗`) marked from the `[blocked-by:]` tag alone. Tags
are stripped from the one-line title; `[blocked-by:]` and
`[drain-exempt: <reason>]` are re-echoed after it — the two tags a picking
session acts on without opening the entry body.
`--extent <slug>` prints the inclusive line range of one entry's body (parent
slug → whole subtree; boundary = next sibling-or-shallower bullet, heading,
`---`, or EOF, including the trailing blank so a range deletion leaves no
double blank). `--collapse-deferred` replaces the deferred listing with a
per-`###`-subsection tally — generic over whatever subsection names the file
has (a hardcoded tally table would be consumer rule content), with
entries under no subsection tallied as `(top)`.

Both modes append an **attention block** when the `## Lessons Learned` section
carries `[attend]` entries (§The tag algebra): each such entry's lead line,
capped at `QUEUE_KIT_ATTEND_CAP` (default 3) with overflow noted as
`(+N more [attend])`. Lead lines only, never bodies — the block is
always-loaded through the session-context hook that embeds this output, so the
cap is its token budget. This is the inbound lesson channel reaching every
later session of the iteration with zero consumer-hook edits.

### bin/roadmap.sh

The public-roadmap emitter: a tool, not a gate (no `# graph:` manifest),
following `bin/queue-index.sh`. `--emit` prints the generated block to stdout;
`--write` splices it between the markers in `QUEUE_KIT_ROADMAP_FILE` through
gate-sdk's `inject_marker_block`, leaving every byte outside them untouched. It
reads the live task sections only — active plus deferred, never the done section,
because a shipped item is history, not direction.

The emit grammar is a contract on both sides: `check-roadmap-fresh` byte-compares
it and a reader consumes it as the page's body.

- One `### <horizon>` heading per configured horizon, in configured order, each
  emitted even when the queue puts nothing there — an empty horizon is
  information, and a section that vanishes when it empties reads as a page that
  forgot it. An empty horizon carries a one-line placeholder instead of bullets.
- Under each, one bullet per tagged entry, in queue order:
  ``- **`<slug>`** *(<track>)* — <summary>``.
- The summary is the entry's `roadmap-summary:` declaration (§The tag algebra),
  printed verbatim. Nothing else in the entry is projected — not the lead line,
  not the body, which carries internal design rationale and cost accounting,
  precisely the content a public page must not inherit. The emitter neither
  summarizes nor truncates: an author decided what the page says, and an entry
  without a single non-empty declaration contributes no bullet at all, so
  unmarked prose cannot reach the page even with the gate bypassed.

The block ends on a blank line. That blank is load-bearing rather than cosmetic:
the Pages parser closes a list only on one, so without it the `:end` marker abuts
the last bullet and renders *inside* that list item. Command substitution strips
the trailing newline on both sides of the freshness compare, so the byte-compare
is unaffected — the same arrangement `gen-value-rollup.sh` uses to keep a table
from swallowing its marker.

Honest limit — the lead line is a shared budget, and `[roadmap:]` competes for it
with every other tag the entry carries. Against `check-queue-wrap`'s floor a
`[spec: <file>]` tag is far wider than `[design-pending]`, so an entry carrying
a spec pointer and a long slug may have room for the `[roadmap:]` tag but none
for prose after it. That costs nothing now the summary lives on its own line:
the lead line needs room for the tag alone. It does bound the tag itself — a
sufficiently long slug plus spec pointer leaves no room even for that, and such an
entry is unprojectable until the pointer drops at the amendment's merge. A true
state of the queue rather than a defect to gate around: the wrap floor exists so a
runaway never reflows to column 0, and widening it to make room for a public page
would trade a parse guarantee for a presentation one.

Ordering inside a horizon is queue order, not a rank; the page states so in its
framing rather than implying a priority the queue does not carry. The
three-to-five items per horizon band is editorial posture, stated in the page's
framing prose, and deliberately **not gated**: a horizon legitimately holding two
items is a true state of the queue, and reddening a commit over it would buy a
curation opinion at the cost of the low-false-positive contract every gate here
is held to.

### check-roadmap-fresh

Invariant, in two assertions over `QUEUE_KIT_ROADMAP_FILE`:

- **(A) Freshness.** The committed marker block byte-matches
  `bin/roadmap.sh --emit`, on the `check-footprint-fresh` /
  `check-value-rollup-fresh` model — diff the emission against the block and name
  the regen command in the red output. A missing file, missing markers, or an
  unbalanced marker pair is fail-closed (exit 2): an absent projection under a
  configured path is a broken install, not a clean skip.
- **(B) Tag-field validity.** Every `[roadmap:]` tag in the live task sections
  parses as `<horizon>/<track>` with both fields members of their configured
  arrays. An unknown horizon or track, a missing slash, or a second `[roadmap:]`
  tag on one entry is a violation naming the entry and the offending field.
- **(C) Marking parity**, in both directions. A `[roadmap:]`-tagged entry carries
  exactly one `roadmap-summary:` declaration — none, or two, is a violation,
  because a curated entry with no marked text would otherwise reach a reader as a
  bullet with no prose. And a declaration on an entry whose lead line carries no
  `[roadmap:]` tag is a violation too: a dead marking is what a dropped or
  reflowed tag looks like from the page's side, and the tag is what decides
  projection.

Assertions B and C live here rather than in `check-tag-lead-line` because this is
the gate that already loads the horizon and track vocabularies and the shared
`[roadmap:]` parse; putting them in the lead-line gate would make a placement gate
load a value roster and a body-scoped declaration it has no other use for.
`check-tag-lead-line` still governs the tag on its *placement* axis, and governs
the declaration not at all — it is body-scoped by design.

Calibration: `QUEUE_KIT_ROADMAP_FILE` empty is a clean skip for both assertions
— the correct no-op for a consumer that publishes no roadmap, matching
`check-queue-slug-liveness`'s empty-globs behavior. The two-argument form
(`[projection-file] [emit-file]`) steers assertion A off the live emitter onto
pre-baked fixture files, the interface both cited models carry: a freshness gate
offering only its bare form has no fixture that does not read the live queue, so
the pair's verdict would turn on tree state outside the fixture directory.

### bin/lesson-sink.sh

The outbound channel's router: reads a lesson body on stdin and resolves the
sink for its `<tag>` argument from `QUEUE_KIT_LESSON_SINKS`. A configured entry
runs as a **command** with the body streamed to its stdin — a command, not a
path, so the sink may reformat the body into a downstream backlog's own
grammar; the command's exit status becomes the tool's, so a failing sink is a
red close step and harvest material is never half-routed to a silent fallback.
An unconfigured tag falls **open** to appending the body to
`${GATE_SDK_WORKFLOW_DIR:-.workflow}/<tag>-harvest.md` — the honest
manual-drain default that keeps a fresh clone (no overlay) closing cleanly and
preserves the staging file's documented reclaim path. A tool, not a gate (no
`# graph:` manifest); the consumer close skill invokes it so the tracked skill
names the mechanism, never a sink value, and a private sink command lives in
the `queue-config.local.sh` overlay (§lib/queue.sh).

### check-queue-hygiene

Invariant: the queue contains only tasks, tags, and section structure — no
HTML comments (provenance belongs in git history), no exact-duplicate
non-blank non-`---` lines (copy-paste artifacts), no column-0 prose (every
column-0 line is a heading, a bullet, `---`, or a configured
`QUEUE_KIT_PROSE_LEADS` token — the shape that carries protocol duplication
is banned, not semantic duplication, which is not mechanizable).

Calibration: indented lines are never flagged on the prose axis; the
lead-token allowance is a whole-line lead match, not a substring. Division of
labour with `check-queue-sections`: hygiene owns line *shape* (what a column-0
line may be), the sections gate owns heading *presence* (that each required
`##` heading exists exactly once) — neither subsumes the other.

### check-queue-sections

Invariant: the queue file carries each `QUEUE_KIT_REQUIRED_SECTIONS` heading
exactly once — zero occurrences (missing/typo'd) and two-or-more (accidental
paste) are both red. This is the fail-closed floor under every section-scoped
scanner: `check-amendment-queue`, `check-task-names`, `check-task-conservation`,
and the session-context index all locate work by `## <section>` boundaries and
silently find *nothing* — passing open — when a heading is dropped or
misspelled. A trailing `:` on a required entry marks a dynamic-suffix heading
(the iteration header, whose suffix is its iteration name) and is
prefix-matched; every other entry is matched exactly. A grep error (not a
no-match) is fail-closed (exit 2). The `# graph:` couples the queue file at
`tier=precommit`.

### check-queue-wrap

Invariant: no line exceeds the `QUEUE_KIT_WRAP_BUDGET` gate floor (default
100 columns; the authoring target is ~80). The tools key on the column-0
`- ` lead, so an unwrapped runaway that reflows to column 0 corrupts the
parse; the tripwire fires before that lands.

Calibration: three exemptions mirror the wrapping convention — table rows,
fenced-code blocks, and lines over budget solely due to one unbreakable token
(URL, path). Width is Unicode code points, not bytes.

### check-tag-lead-line

Invariant: every **lead-line-scoped** tag sits on its bullet's lead line — the
only line *its* readers scan; such a tag pushed to a continuation line by a
reflow silently unblocks a task, masks a design-pending state, voids a drain
exemption, or drops a lesson out of the attention block. Membership tracks reader semantics, not §The tag
algebra: a tag is governed here when its readers scan lead lines alone, so the
set is narrower than the algebra's and `[precondition-ok:]` is deliberately
outside it — `check-queue-prose-precondition` honors that tag anywhere in the
entry, leaving it no lead-line requirement to enforce. The governed set and
scanned surface both widen with the lesson channels: `[blocked-by:]` /
`[spec:]` / `[design-pending]` / `[drain-exempt:]` / `[roadmap:]` in the task
sections (active + deferred), plus
`[attend]` and every `QUEUE_KIT_LESSON_TAGS` name in the `## Lessons Learned`
section — the section `queue-index.sh` now reads, which retires the old "parsed
by no reader" exemption for it. Couples
the width-only wrap gate to the tag-parsing tools over the same surface: gate
the coupling, not just each side.

The gate states that set **once**, as a class table of `<name><terminator>`
tokens (`]` for a bare tag, `:` for a field tag) from which both the match
literal and the class key are derived. That single statement is the whole
point: a table entry naming the tag once cannot desync its matcher from its
key, and the desync had one silent direction — a key renamed without its regex
leaves every downstream reader agreeing while the matcher hunts a token no
entry carries, and the lead-line guard for that class dies with nothing
reddening. The table is also the **derivation surface** a consumer's enum
emitter reads for the tag vocabulary (this repo's `scripts/enum-sets.sh` →
`check-prose-enum`), so it is the single source for the spelling on both the
enforcement and the prose side.

Calibration: lead-class rule — a tag of class C on a continuation line is a
violation only when the lead line lacks class C (prose that mentions a tag
the lead already carries is tolerated); tags outside the scanned sections are
parsed by no reader and ignored. Fenced-code and table lines exempt.
Residual accepted gap: a same-class duplicate sliding off while one stays on
the lead — negligible severity, not reflow-realistic.

### check-task-names

Invariant: every active-section and deferred-section entry (and every
sub-task) leads with a valid kebab-case slug, unique across the file; every
done-section entry is a bare slug only; every `[blocked-by: X]` resolves to a
live task, and one pointing at a done slug is flagged stale.

Calibration: active and deferred share one namespace so resolution works
regardless of where the target lives; the non-bold indented bullet is the
documented prose-note escape; done slugs are validated as tokens but not
cross-checked against the live namespace. Help texts cite this SPEC.

### check-task-conservation

Invariant: every live slug (active + deferred, sub-tasks included) present at
HEAD is still present in the working tree — live or done. A slug vanishing
from both is a lost task: the absence class diff-review reliably misses (you
notice wrong things present, not right things missing). Half-applied moves,
botched renames, and undocumented withdrawals all fire.

Calibration: diffs `git show HEAD:<queue>` against the worktree, hence
`no-fixture:` (a committed fixture has HEAD == worktree; the bad case needs
an uncommitted deletion — infeasible, not a stopgap). No git baseline ⇒ clean
exit (nothing to compare is not a violation). Clearing the done section is
safe: only HEAD's *live* slugs are conserved. A rename intentionally fires —
move the old slug to done, sweep refs.

### check-queue-prose-precondition

Invariant: no active-section entry states a forward precondition in prose
("revisit when …", "once X lands", "gated on …") without a `[blocked-by:]`
tag — such an entry is latently blocked yet mechanically pickable as "first
unblocked", because selection trusts tags, not prose. Resolution: tag the
real blocker, move the entry to the deferred section, rephrase past-tense if
the precondition is met, or the `[precondition-ok: <reason>]` opt-out (a
queue tag, not an HTML comment, so it survives the hygiene gate).

Calibration: the trigger set (`QUEUE_KIT_PRECONDITION_REGEX`) is deliberately
narrow — forward-looking phrasing only, past-tense narration stripped before
matching — and scoped to the active sections (the deferred section uses
"revisit when" as normal vocabulary and is exempt). FP-bearing by
construction (parsing prose intent); the blocking grade is justified by a
silent pick attested in production use and the bounded scope.

### check-queue-slug-liveness

Invariant: on every prose surface named by `QUEUE_KIT_PROSE_SURFACE_GLOBS`,
every slug-shaped bold-code token — `` **`<token>`** `` whose token matches the
slug grammar (`[a-z0-9][a-z0-9-]*`, so a `--flag` mention falls outside) —
resolves against the queue's live slug set (`lib/queue.sh`'s `queue_live_slugs`,
active + deferred). A token naming no live task is a dead membership claim,
reported by file, line, and slug. The slug set excludes done tasks by
construction: prose about landed work must drop the bold-code form and cite the
owning SPEC instead, so a living page can never keep calling a landed task
queued.

The bold-code token is prose's *only* sanctioned queue-membership claim
(§The tag algebra) — a grammar narrow enough for a gate to read, so the
living-page current-state rule stops being a standing duty and becomes a
checked contract. Default-empty knob: a consumer with no configured surface
no-ops clean; the surfaces are consumer editorial config, never a kit literal
(the provenance seam). Resolution direction is one-way (`dir=one`): the queue
is ground truth, the prose the audited follower.

### templates/

`queue-config.sh` — the consumer config template documenting every knob (the
`# spec:` pointer to the §Layout knob table, which now includes
`QUEUE_KIT_LESSON_SINKS` and its local-overlay reminder).
`TASK-QUEUE.md` — a starter queue skeleton: the sections in default order,
one example entry per grammar shape shown under `Technical Debt` (the
`[spec:]`-gated `New Features` carries teaching prose only, since a spec-ready
example would dangle a ref). It ships battery-clean when copied verbatim — the
starter-template conformance contract owned by gate-sdk/SPEC.md §Consumer smoke
— and carries lifecycle-kit's iteration header as inert scaffold so a
combined-tree copy clears the stage gates too.

## Out of scope

<!-- prose-enum-exempt: names the two amendment-lifecycle tags specifically; [blocked-by:] is a dependency tag outside that lifecycle, not a dropped task-tag member -->
The amendment lifecycle around `[design-pending]`/`[spec:]` (section-wide
enforcement, promotion procedure, `check-amendment-queue`) is canon-kit's
scope. Code-comment TODO scanning (`TODO(task:<slug>)` resolution against
the queue) couples to source-file conventions and is canon-kit's, on the
governed comment surface (`check-todo-task-liveness`). Task-output readers and delegation tooling are delegation-kit's
scope. A consumer's protocol prose, deferred-subsection taxonomy, and task
bodies are rule content and never ship.
