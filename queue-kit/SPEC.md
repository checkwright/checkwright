# queue-kit — a git-native, agent-readable task tracker

One Markdown file is the tracker: sections are queues, bullets are tasks,
bold kebab-case slugs are the task handles, and square-bracket tags are the
state machine. The problem the kit solves: a coding agent selects work by
parsing, not by reading — so everything selection trusts (position, slugs,
tags) is grammar a gate can enforce, and everything a human writes freely
(task prose) is kept out of the parse path. Drift between what the prose says
and what a parser sees is the failure mode; all but one of the gates exist to
close one instance of it each.

Extracted from the governance meta-layer of a private production platform.
The kit carries the queue grammar and its gates only; a consumer's section
names, wrap budget, and protocol vocabulary are config with the platform's
values as defaults. Requires [gate-sdk](../gate-sdk/) (the gates follow its
four contracts and resolve through its registry).

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

Tags are square-bracket literals with fixed spelling (mechanism, not config);
every tag sits on its bullet's **lead line** — the only line the parsing
tools scan (enforced by `check-tag-lead-line`).

- `[blocked-by: <slug>]` — the entry is unpickable until `<slug>` completes.
  Repeat per blocker. Must resolve to a live task (active or deferred — a
  deferred blocker stands; it is unbuilt); a blocker in the done section is a
  *stale* tag that must be removed, because the tag alone marks a task
  unpickable.
- `[needs-spec]` — design-pending marker. queue-kit parses and displays it;
  the placement semantics (deferred-section-wide enforcement, promotion
  rules) are spec-kit's amendment lifecycle and land with that kit.
- `[spec: <file>]` — spec-ready pointer. Same split: syntax here, amendment
  semantics in spec-kit.
- `[precondition-ok: <reason>]` — per-entry opt-out valve for
  `check-queue-prose-precondition`.

## Layout and configuration

The kit is vendored beside gate-sdk (conventionally at `queue-kit/`); its
gates are registered in the consumer's `gates.list` by name and resolve
through gate-sdk's multi-kit path. `bin/queue-index.sh` is a tool, not a gate
(no `# graph:` manifest).

Config follows lifecycle-kit's pattern: copy `templates/queue-config.sh`
into the gates dir as `queue-config.sh` (or point `QUEUE_KIT_CONFIG_FILE`
elsewhere) and override any knob; defaults fill what the consumer left unset,
and the loader exits 2 on a malformed config — a broken grammar must not gate
anything. Knobs:

- `QUEUE_KIT_QUEUE_FILE` — default `${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}`;
  every gate also takes the file as `$1` (fixture capability).
- `QUEUE_KIT_ACTIVE_SECTIONS` — array, default
  `("New Features" "Technical Debt")`, order = selection order.
- `QUEUE_KIT_DEFERRED_SECTION` / `QUEUE_KIT_DONE_SECTION` — defaults
  `Deferred` / `Done`.
- `QUEUE_KIT_WRAP_BUDGET` — default `100` (`check-queue-wrap` gate floor).
- `QUEUE_KIT_PROSE_LEADS` — array of column-0 lead tokens exempt from the
  hygiene gate's no-prose axis, default `("Protocol:")`.
- `QUEUE_KIT_PRECONDITION_REGEX` — the forward-precondition trigger set for
  `check-queue-prose-precondition`, default = the shipped phrase set.
- `QUEUE_KIT_REQUIRED_SECTIONS` — array of `##` headings that must each appear
  exactly once (`check-queue-sections`), default = the iteration header
  (`Iteration:`, prefix-matched for its dynamic suffix) plus `New Features` /
  `Technical Debt` / `Deferred` / `Done` / `Lessons Learned`. A trailing `:`
  marks a prefix-matched heading; every other entry is matched exactly.

Cross-kit note: lifecycle-kit's `LIFECYCLE_ACTIVE_SECTIONS` carries the same
default. The knobs are independent (either kit runs without the other); a
consumer renaming its active sections sets both.

## Per-component contracts

### lib/queue.sh

The sourced config loader plus shared adapters: the section-regex builders
the gates pass to awk (both sides of every section boundary must parse
identically — a shared adapter removes that drift axis), and the slug/tag
extraction helpers. Values and adapters only, never gate structure
(gate-sdk's `lib/gate.sh` rule).

### bin/queue-index.sh

Compact surface of the queue for task selection — the iteration header line
if present, then every top-level active entry as a one-line title plus tags,
ready (`•`) vs blocked (`✗`) marked from the `[blocked-by:]` tag alone.
`--extent <slug>` prints the inclusive line range of one entry's body (parent
slug → whole subtree; boundary = next sibling-or-shallower bullet, heading,
`---`, or EOF, including the trailing blank so a range deletion leaves no
double blank). `--collapse-deferred` replaces the deferred listing with a
per-`###`-subsection tally — generic over whatever subsection names the file
has (the platform's hardcoded tally table does not leave the platform), with
entries under no subsection tallied as `(top)`.

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
(the iteration header, which carries its iteration name and `[stage:]`) and is
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

Invariant: every `[blocked-by:]` / `[spec:]` / `[needs-spec]` tag inside the
task sections (active + deferred) sits on its bullet's lead line — the only
line the tag-reading tools scan; a tag pushed to a continuation line by a
reflow silently unblocks a task or masks a needs-spec state. Couples the
width-only wrap gate to the tag-parsing tools over the same surface: gate the
coupling, not just each side.

Calibration: lead-class rule — a tag of class C on a continuation line is a
violation only when the lead line lacks class C (prose that mentions a tag
the lead already carries is tolerated); tags outside the task sections are
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
construction (parsing prose intent); the blocking grade is justified by an
attested silent pick on the source platform and the bounded scope.

### templates/

`queue-config.sh` — the consumer config template documenting every knob.
`TASK-QUEUE.md` — a starter queue skeleton: the sections in default order,
one example entry per grammar shape shown under `Technical Debt` (the
`[spec:]`-gated `New Features` carries teaching prose only, since a spec-ready
example would dangle a ref). It ships battery-clean when copied verbatim — the
starter-template conformance contract owned by gate-sdk/SPEC.md §Consumer smoke
— and carries lifecycle-kit's iteration header as inert scaffold so a
combined-tree copy clears the stage gates too.

## What stayed on the platform

The amendment lifecycle around `[needs-spec]`/`[spec:]` (section-wide
enforcement, promotion procedure, `check-amendment-queue`) is spec-kit's
scope. Code-comment TODO scanning (`TODO(task:<slug>)` resolution against
the queue) couples to source-file conventions and stays behind until a later
kit rules on it. Task-output readers and delegation tooling are
delegation-kit's scope. The platform's protocol prose, deferred-subsection
taxonomy, and task bodies are rule content and never leave.
