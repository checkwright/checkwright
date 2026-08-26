# queue-kit — a git-native, agent-readable task tracker

One Markdown file is the tracker: sections are queues, bullets are tasks,
bold kebab-case slugs are the task handles, and square-bracket tags are the
state machine. The problem the kit solves: a coding agent selects work by
parsing, not by reading — so everything selection trusts (position, slugs,
tags) is grammar a gate can enforce, and everything a human writes freely
(task prose) is kept out of the parse path. Drift between what the prose says
and what a parser sees is the failure mode; all but two of the gates exist to
close one instance of it each. The two exceptions hold a different axis —
projection freshness (`check-roadmap-fresh`) and the deferred pool's
per-entry budget (`check-queue-entry-budget`).

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
  when the premise was observed. A deferred body is free prose, and four
  bold-lead-in fields recur, each answering a question a later scope asks:
  `Deliverable` (what landing looks like), `Why [design-pending]` (what the
  open design actually is), `Cost while deferred`, and a closing
  `Filed <date> by <stage>` provenance line. Three are conventions no gate
  reads; **`Cost while deferred` is required** — the Gap-disposition rule's
  costing, held by `check-queue-entry-budget` (§check-queue-entry-budget),
  which also caps the entry's total length. The field's bold lead-in is
  line-local like a tag: split across a reflow it is invisible to the scanner
  and the entry reads as uncosted.
- **The icebox section** (optional, `QUEUE_KIT_ICEBOX_SECTION`, default
  **empty** — no section, no tier) — dormant tasks compressed to one line
  each, sitting **after the deferred section and before the done section**.
  Live work, not history: §The icebox tier owns the tier, its grammar, its
  eligibility rule, and the position contract a cross-kit reader depends on.
- **The done section** (default `Done`) — the record that an entry has **left
  the live pool**: one line per exit, the bare slug only, with prose about what
  happened living in git history. An entry reaches it when a **landed unit or a
  closed ruling has mooted it**, or as a **ruled wontfix**. It is not a delivery
  claim, and never was — delivery is recorded by the landing commit's *type*,
  which is where every reader that cares about it already looks (§The icebox
  tier enumerates the exits; drift-kit/SPEC.md §Bundled KPIs owns what the
  commit-type reading yields for an exit that shipped nothing).
  **The grammar is unchanged** — a bare slug line, no tag, no disposition
  token — so every existing reader parses exactly what it parsed before. What
  widened is which entries may lawfully arrive, not how they are written.

The **defer date** of an entry is its `Surfaced <date>` mark when present, else
the date on its `Filed <date>` provenance line. One definition, two readers —
drift-kit's deferred-age KPI and the `queue-index` arm's `--icebox-candidates`.
Widening, not replacing: `Surfaced` records when the premise was observed and
is the better premise-rot datum, `Filed` is the honest available fallback, so
no entry owes a migration. drift-kit re-implements the definition rather than
sourcing `lib/queue.sh`, because a kit dependency the other way would close a
cross-kit cycle; both implementations cite this section. That is the general
rule for this format, not a one-off for drift-kit: **a kit that cannot depend on
queue-kit re-implements the predicate and both ends cite this section.**
gate-sdk's `check-gate-exemption-tasks` re-implements it for a reason stronger
than a cycle — gate-sdk is the substrate every kit vendors, so depending on
queue-kit would invert the layering and make a queue format a precondition for
running any gate. The holder roster and the residue accounting for the whole set
live at gate-sdk/SPEC.md §check-gate-exemption-tasks, and are deliberately not
restated here: two hand-maintained copies of one roster are worse than one,
because a holder added or dropped is then owed two edits in two SPECs and only
the one that prompted the change is likely to get it.
- Any other section (an iteration header, a lessons section) is outside the
  grammar and ignored by every gate except the file-wide hygiene axes.

**Where this format has two holders, what is compared between them is
classification over a corpus, never the derived literals.** The shell library
exposes ERE strings and an array; its compiled counterpart exposes a section
value and free predicates over it, so there is no value either could hand the
other — comparing them would compare nothing, or force one side to grow an
accessor whose only reader is the test. The comparable question is the one this
section defines: given the same queue file, do both agree on which lines are
section boundaries, which are task bullets, which is the lessons line, which
slugs are live? Two witnesses held to each other over a golden *corpus*, with no
committed expected file — a maintained golden is a third copy to drift, and the
failure the comparison exists to catch is one side edited without the other,
which it catches directly (§lib/queue.sh names the holder).

An entry is a column-0 `- **slug** — prose…` bullet; continuation lines are
indented, never column 0. An *indented* bullet with a bold lead-in is a
sub-task (same grammar); with a plain or italic lead-in it is a prose note
and is left alone. Slugs match `[a-z0-9][a-z0-9-]*` in one global, unique
namespace across active + deferred + icebox + sub-tasks — a slug is the task's
stable handle for its whole life: `[blocked-by:]` references it and the
done-section line carries it verbatim. The slug grammar is kit mechanism, not
config.

**Section order sequences an iteration; it does not rank it.** The default
`("New Features" "Technical Debt")` is read as selection order (§Layout and
configuration), and the argument for it is not that features outrank debt:
position starves nothing, because an active entry's residency is one iteration
by the drain rule — the drain stage's entry requires the active sections empty
(lifecycle-kit/SPEC.md §check-stage-entry) — so every promoted entry lands in
the same iteration whichever section holds it. What ranks is the promoting
stage's decision to put an entry in an active section at all; order within them
is a sequencing default, the posture the roadmap projection already states
outright for queue order generally (§The roadmap arm).

Features lead on the feature/debt asymmetry canon-kit owns (canon-kit/SPEC.md
§The amendment lifecycle): a feature merges an amendment into the canonical
spec, debt converges an implementation on the spec as it already stands.
Reversed, a debt entry sharing an owner doc with a sibling amendment converges
on text the same iteration is about to rewrite. It stays a default rather than
a rule — the array is consumer config, and a consumer whose iterations do not
mix the two kinds reorders it freely.

### The icebox tier

The deferred pool has an intake asymmetry the delivery doctrine itself creates:
gap disposition and scope-gated intake make filing mandatory and cheap, while
the only exit is *building* the entry. The icebox is the missing exit — a third
**live** task section holding entries whose own cost field says they are
dormant, each compressed to its lead line.

close-surface: TASK-QUEUE.md#Deferred advisory

`advisory` is the honest mode: no structural forcing function refuses on an
untriaged backlog, and inventing one would redden an unrelated commit over a
curation opinion. That is what makes eviction *symmetric* to promotion —
promotion is forced by the scoping stage's exit condition, eviction gets roster
visibility and an auditable skip.

**It joins the shared task adapters rather than standing outside them**, and
that is the whole design:

- **Eviction is a conserved move.** `check-task-conservation` diffs the live
  slug set at HEAD against the worktree, so deferred → icebox keeps the slug
  live and the gate stays silent. No sanctioned disappearance needs inventing,
  and none exists — every exit from the design-pending pool is conserved:
  deferred → icebox, icebox → deferred on real recurrence, and either → done
  for a ruled wontfix, **or for an entry a landed unit or a closed ruling has
  mooted**. The added route is conserved like the rest — the slug stays visible
  in the file, on a bare done line — which is why the claim above is
  strengthened by it rather than disturbed.
- **One namespace, one parse.** `check-task-names` keeps slug uniqueness and
  `[blocked-by:]` resolution across the whole pool; an iceboxed slug is a legal
  blocker target, because it is unbuilt.
- **The living-prose contract keeps reaching it**, so a page citing an
  iceboxed task stays green — correct, because the task is dormant, not landed.
- **Tags stay lead-line-governed**, with no gate edit.

Every gate keyed on the shared task regex therefore widens with **no change to
any gate file** — the widening is entirely in `lib/queue.sh`, which is what the
one-adapter rule was built to buy. `check-queue-prose-precondition` deliberately does not
reach the tier: forward-looking phrasing is normal vocabulary in a parked
entry, the same exemption the deferred section already carries.

**A mooted entry takes the done exit, not this one, and the two routes into it
are worth naming because neither reads like the criterion that covers it.** A
slug **merged away as a duplicate** — two entries found to own one gap, one of
them kept — is an entry *a landed unit has mooted*: the first criterion exactly,
not a new class. A slug **mooted by supersession**, where the work became
unnecessary rather than being folded into a sibling, is covered by whichever
criterion did the mooting — a landed unit that made it unnecessary, or a closed
ruling that did. Both routes are instances and neither is a class needing its
own state. They are enumerated because the criteria are phrased as supersession,
so a session holding a duplicate merge will not recognise itself in them.

**Distinct from this tier, which is the reason a merged duplicate does not land
here.** The icebox holds *unbuilt* work that stays promotable and dormant. An
absorbed duplicate is not dormant, it is redundant: there is nothing to promote
it back to, and returning it to any live section would re-open the namespace
collision the merge closed. The done exit is what closes that collision, because
a slug reaching Done leaves the live namespace (§check-task-names).

**No disposition token marks which route was taken, and that is a refusal
rather than an omission.** It would have no reader:
`check-task-conservation` reads membership, drift-kit's split KPI reads the
landing commit, and a human reads git history. A field with no named reader is
removed rather than added.

**The grammar is the lead line and nothing else:**

```
- **<slug>** [design-pending] — <one sentence: what it is, and why it is dormant>
```

- **It carries the same `[design-pending]` tag every deferred entry carries.**
  No `[icebox]` tag is minted: section membership *is* the state, and a tag
  restating its own section is the two-sources defect. What generalizes across
  the amendment lifecycle is the design-pending **section set**, not its tag
  set. The "why it is dormant" clause is written where it fits and otherwise
  left to the tier — membership already declares it.
- **No `###` subsections and no sub-tasks.** Grouping is presentation on a
  surface whose entire purpose is minimum residency, and a sub-task is a
  continuation line the shape rule rejects.
- **A `[roadmap:]`-tagged entry is not icebox-eligible**, and the rule needs no
  new code: the roadmap parse walks the live task sections, so the tier enters
  its walk, and `check-roadmap-fresh` then reds any `[roadmap:]` entry carrying
  no `roadmap-summary:` declaration — which a one-line entry has nowhere to
  put. **The enforcement is conditional:** that gate exits clean at its top
  when `QUEUE_KIT_ROADMAP_FILE` is empty, so the guarantee holds for a consumer
  publishing a roadmap page and degrades to convention for one that is not.
  The eligibility rule binds either way. Keeping the tier *inside* the roadmap
  walk rather than excluding it is deliberate: excluded, a `[roadmap:]` tag
  that drifted into the icebox would silently drop a public commitment;
  included, it reds.
- **The narrative is recoverable, not discarded.** The commit that iceboxes an
  entry necessarily contains the removed body in its own diff, so recovery is
  `git log -p -S'<slug>' -- <queue-file>` — a pickaxe that depends on no
  commit-message convention. Nothing is copied into the queue to point at it.
  **Recovery is mandatory before any ruling on the entry**, not merely
  available: the lead line is a one-clause summary that carries neither the
  deliverable nor the blocking design question, so promoting a dormant entry,
  or ruling a build has subsumed one, is judged against the recovered body.
  Ruling off the line alone is ruling off a summary written to be dropped.

**Eligibility**, judged at the closing stage: the entry's cost field opens in
the low class; it carries no `[roadmap:]` tag; and it has **no live promotion
trigger**. **What counts as live was narrowed 2026-08-23 by operator ruling**,
on a measured pool where the prior reading let almost nothing qualify: a live
trigger is a **live, unbuilt queue slug** the entry names, a dated `recurrence:`
line, or a `[roadmap:]` tag. A trigger that is itself gated on launch — a first
external adopter, a preview cohort, an install count — is **dormant**: the entry
waits in the icebox until the event occurs and returns on it, which is the round
trip the tier already conserves. The cost field's opening token is the class word the
`--icebox-candidates` arm reads (`low`, `zero`, `bounded`, `cosmetic`), so a
cost field that opens in prose has declared no class and is read as not-low;
authoring the class word first is the contract, and the gate that holds it is
owed — filed 2026-08-23 into the gap inbox, drained by the next close.

**Direct filing into the icebox is permitted on one condition — ruled
2026-08-23, reversing the prior rule.** The prior rule held that nothing files
directly into the icebox because a newly filed finding has never been triaged.
What changed is who triages: the closing stage's drain *is* the triage, and a
drained bullet whose cost opens in the low class and names no live trigger may
land as a one-line icebox entry without first being written as a deferred body
that the next close compresses. The net-delta KPI stays honest because the arm
counts the tier on the way in. A mid-iteration filing still goes through the
gap inbox; what this permits is the drain landing it one tier lower.

**Position is a contract, not a preference.** The read order is pickable →
parked → dormant → history, and a cross-kit reader depends on the tier sitting
between the deferred and done sections (gate-sdk/SPEC.md
§check-gate-exemption-tasks). **Accepted residual, and it is quiet: no gate
enforces section order.** `check-queue-sections` counts occurrences per
required section with no positional state, and no other gate carries one. A
misplaced icebox surfaces only as an exemption naming an iceboxed slug
reddening with "no live task" — which requires such an exemption to exist.
Otherwise the misplacement is silent. The contract is carried by this SPEC and
by review rather than by a gate, and saying so is the honest form of it.

### The tag algebra

Tags are square-bracket literals with fixed spelling (mechanism, not config
— the one exception is the consumer-named harvest vocabulary below); every tag
sits on its bullet's **lead line** — the only line the parsing tools scan
(enforced by `check-tag-lead-line`).

Outside the queue file, living prose claims a task's queue membership with one
grammar: a **bold-code token** — `` **`<slug>`** `` whose token matches the
slug grammar — which `check-queue-slug-liveness` resolves against the live
slug set on the configured prose surfaces (§check-queue-slug-liveness).

Inside the queue file, that same token in **single backticks** — matching the
slug grammar, appearing in an entry's **body** rather than on its lead line —
is a **citation** of the named entry: the relation an author already drew while
writing about something else. The two forms are deliberately different claims.
The bold-code form is a *membership* claim, and a dead one is a false statement
`check-queue-slug-liveness` reds on; the in-body citation is a *reference*,
aggregated by `bin/queue-edges.sh` (§bin/queue-edges.sh) and audited by
nothing. The rules below make citations parseable, each covering a shape a live
corpus contains:

- **Resolution is against the live slug set** — `queue_live_slugs`: active,
  deferred, and a configured icebox, the existing source of truth — **and
  against the retired one**, the slugs the file's own history shows were once
  live (§bin/queue-edges.sh). A citation resolving to a retired slug is an edge,
  marked retired; the two sets are disjoint by construction.
- **An unresolved token is not an error; it is simply not an edge.** This is the
  load-bearing rule. Entries legitimately name *landed* work — a closed defect
  class, a shipped contract, a settled ruling — and that citation is valuable
  prose no gate may punish, which is why no liveness assertion reaches inward
  from the configured prose surfaces (§check-queue-slug-liveness).
  **What the remainder actually contains was measured, and the measurement is
  why no report over it and no gate on it can exist.** After the retired slugs
  are taken out, what is left is overwhelmingly *not citations at all*: commit
  SHAs, content digests, iteration names, KPI and gate names, runner labels, CI
  job names, doctrine rule names, and ordinary backticked words a slug-shaped
  grammar cannot tell from a slug. On a mature corpus that remainder is the
  large majority of unresolved tokens. A listing of it is unusable and a red on
  it would fire on prose that is correct.
- **Whether a citation of landed work should be distinguishable in prose at all
  — no**, and the answer follows from the two refusals below rather than adding
  a third. A prose marker separating landed from live is the maintained roster
  under a new spelling: it would need writing on every citation a corpus has
  already written, and re-writing on every disposition. The distinction is
  **derived** instead (§bin/queue-edges.sh's retired block), which is the form
  both refusals leave open, and no author writes anything.
- **Self-citation is not an edge** — an entry naming its own slug in its own
  body is narration, not a relation.
- **`[blocked-by: <slug>]` is an edge too**, and the one already-structured
  class: including it makes an inbound set complete rather than merely
  prose-derived.

**No relational vocabulary is declared, and that refusal is the design.** Prose
phrases a relation many ways and keeps inventing more, so an enumerated verb set
would be brittle the day it landed — and, decisively, a kit literal spelling one
project's relational verbs would ship that project's vocabulary as everyone's.
That is the provenance seam, the same reason `[roadmap:]`'s horizons and tracks
are consumer-configured arrays rather than kit literals. Declaring no vocabulary
means there is no vocabulary to leak and no consumer config to invent: a
relation's *kind* rides the citing line itself, quoted verbatim
(§bin/queue-edges.sh).

**A `relates: <kind> <slug>` declaration on the `roadmap-summary:` pattern was
weighed and refused.** It is precise and it carries a kind — and it is wrong
here on three counts. Every citation a corpus has already written would need
hand-authoring, the maintained-roster anti-pattern derivation-first forbids;
those lines would land inside entries measured against
`check-queue-entry-budget`'s raw-line cap, where sub-tasks do not relieve a
parent's budget, so precision would be paid for in evictions; and a
hand-declared edge can be forgotten in exactly the moment it matters, whereas a
citation written in prose is written because the author was already thinking
about the relation. The derived grammar costs zero lines in every entry and is
satisfied by any corpus that already cross-references — no migration, and no
adoption step between landing it and reading it.

- `[blocked-by: <slug>]` — the entry is unpickable until `<slug>` completes.
  Repeat per blocker. Must resolve to a live task (active or deferred — a
  deferred blocker stands; it is unbuilt); a blocker in the done section is a
  *stale* tag that must be removed, because the tag alone marks a task
  unpickable.
- `[design-pending]` — design-pending marker, spanning **both** design-pending
  sections (deferred and, where configured, the icebox). queue-kit parses and
  displays it; the placement semantics (section-wide enforcement, promotion
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
  curated onto the generated roadmap page (§The roadmap arm), under `<horizon>`
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
  selection order. The **icebox is the one exclusion**, and it falls out of the
  declaration rule rather than needing one: a one-line entry has nowhere to put
  a `roadmap-summary:`, so a `[roadmap:]` tag that drifted into the tier reds
  on assertion C wherever `QUEUE_KIT_ROADMAP_FILE` is set (§The icebox tier
  states the knob-conditional limit on that backstop). Field validity is
  `check-roadmap-fresh`'s assertion B.

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

An entry that has been **re-filed** carries a second declaration on the same
pattern, `recurrence: <slug> <YYYY-MM-DD> [<YYYY-MM-DD>…]` — one indented body
line naming the entry's own slug, then one date per re-filing, appended in order
and never rewritten. The initial filing is not a recurrence and is not listed, so
the count is the number of dates. It is a declaration rather than a tag for the
reasons the paragraph above gives: its readers scan a line of its own, and it
marks no move across a pending/ready boundary, so it fails canon-kit's
further-tag test. `check-tag-lead-line` does not govern it, and it cannot collide
with the bracket scans.

The **self-naming slug field is mechanism, not decoration**. `check-queue-hygiene`
rejects any exact-duplicate non-blank line across the whole file, unnormalized, so
a slug-free `recurrence: <date>` on two entries stamped the same day would red the
gate — and same-day recurrence on two entries is exactly the case the declaration
exists to record. Naming the slug makes the line unique by construction. It also
makes the declaration resolvable by one anchored grep with no entry-boundary
parsing, which is what lets its two readers — lifecycle-kit's scope pre-emption
rule (lifecycle-kit/SPEC.md §Layout and configuration,
`LIFECYCLE_KIT_RECURRENCE_THRESHOLD`) and drift-kit's `kpi-incident-recurrence`
(drift-kit/SPEC.md §Bundled KPIs) — each read it without depending on the other's
kit. Self-citation is already narration rather than an edge (above), so the field
costs no new rule. That one-grep property is also why **no `queue-index`
recurrence mode exists** — the obvious home, refused: drift-kit could not call it
without the cycle its own KPI already records, which would leave lifecycle-kit's
rule as the only caller, and §The queue-index arm already resists a further mode on
the grounds that folding jobs together gives one tool two output grammars.

**One line with appended dates, not one line per recurrence.** The per-recurrence
form is byte-unique on the (slug, date) pair and is refused anyway: it grows an
entry linearly against `check-queue-entry-budget`'s cap, so a *machine* writer
could push a deferred entry past that cap and red some later, unrelated session's
commit — and the cap's remedy is only half self-served (§check-queue-entry-budget:
relocating grounds into an entry that already owns their subject is self-served
for a mandated write, while minting a *new* entry to hold them stays
authorization-gated). That ground survives the discount §check-queue-entry-budget
grants this declaration, **because the discount is one line**: the second and
later lines of a per-recurrence variant are counted like any other, so the variant
still grows an entry linearly against the cap. The single-line form costs **no**
counted line at all, which strengthens the case for it rather than weakening it.
Its own ceiling is
`check-queue-wrap`'s budget, reached after a handful of dates on a long slug, and
reaching it is the *correct* complaint: a slug recorded as recurring that many
times without anyone promoting it is a governance failure, surfaced loudly rather
than absorbed.

Unlike the `relates:` declaration refused above, no corpus needs migrating. The
line is **session-written under judgment** — the closing stage's gap-inbox drain
is its only mechanized producer, while any session that judges a recurrence is
obliged to stamp one directly, and each date records that session's ruling that
the finding re-occurred, read off the filed prose rather than derived from it
(lifecycle-kit/SPEC.md §The committed gap inbox owns the rule and the standard) —
and hand-read; an entry with no declaration is simply an entry that has not
recurred.

Two tags ride **Lessons Learned** entries — a lesson is a top-level bullet
under the fixed-spelling `## Lessons Learned` heading, and the `queue-index` arm
plus `check-tag-lead-line` read that section's lead lines too. That section is
one of close's inbound triage surfaces, and its forcing function is the sibling
of the gap inbox's — the first-stage entry refuses while it holds entries — so it
declares itself on the close-surface roster here:

close-surface: TASK-QUEUE.md#Lessons-Learned forced=lifecycle-kit/SPEC.md §bin/enter-stage.sh

- `[attend]` — fixed spelling, kit mechanism (the inbound channel): the filing
  session marks a lesson as a live attention point for later sessions *of the
  same iteration*. The `queue-index` arm emits an attention block of `[attend]`
  lead lines (§The queue-index arm); the injection dies at the iteration
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
through gate-sdk's multi-kit path. The `queue-index` arm is not among them: it
is a non-gate arm of the binary, registered in no `gates.list` and carrying no
`.gate` descriptor (§The queue-index arm).

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
  `("New Features" "Technical Debt")`, order = selection order (§The queue
  format argues the default).
- `QUEUE_KIT_DEFERRED_SECTION` / `QUEUE_KIT_DONE_SECTION` — defaults
  `Deferred` / `Done`.
- `QUEUE_KIT_ICEBOX_SECTION` — default **empty**: no icebox tier at all, and no
  kit-shaped empty section on the consumer's queue. A twelve-entry backlog has
  no carry problem, and shipping the section anyway would leak the same posture
  `QUEUE_KIT_HORIZONS` and `QUEUE_KIT_PROSE_SURFACE_GLOBS` already ship empty to
  avoid. Naming the deferred section is malformed config.
- `QUEUE_KIT_ENTRY_LINE_CAP` — positive integer, default `50`; the per-entry
  line cap `check-queue-entry-budget` assertion A holds over the deferred
  section.
- `QUEUE_KIT_ICEBOX_AGE_DAYS` — positive integer, default `30`; the defer-date
  age filter for the `queue-index` arm's `--icebox-candidates`, and nothing else.
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
  marks a prefix-matched heading; every other entry is matched exactly. A
  configured `QUEUE_KIT_ICEBOX_SECTION` is **appended by derivation**, never by
  a second consumer list: an icebox configured but absent from the file would
  otherwise let every icebox assertion pass open on a section that is not
  there, the exact fail-open class `check-queue-sections` exists to stop.
- `QUEUE_KIT_LESSON_TAGS` — array of bare harvest-tag names, default empty; the
  consumer-named outbound lesson vocabulary (§The tag algebra), read by
  `check-tag-lead-line` for placement. Names, sinks, and handling are consumer
  rule content — the kit ships none.
- `QUEUE_KIT_LESSON_SINKS` — associative array, harvest tag → sink command,
  default empty; read by `bin/lesson-sink.sh`, which owns resolution and the
  fail-open default (§bin/lesson-sink.sh). A private sink value belongs in the
  local overlay, not this tracked file.
- `QUEUE_KIT_ATTEND_CAP` — positive integer, default `3`; the maximum `[attend]`
  lead lines the `queue-index` arm emits in its attention block before folding
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
consumer renaming its active sections sets both. The icebox section name spans
every kit that reads the tier, on the same independent-knob shape —
`QUEUE_KIT_ICEBOX_SECTION` here, `CANON_KIT_ICEBOX_SECTION` for the
amendment lifecycle, `DRIFT_KIT_ICEBOX_SECTION` for the queue KPIs — so a
consumer enabling the tier sets each of them, and one left unset degrades that
kit to "no icebox" rather than to a wrong section.

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

The icebox is *derived* here rather than configured twice: a non-empty
`QUEUE_KIT_ICEBOX_SECTION` joins both the shared task regex and the effective
required-sections set, and when it is empty every icebox regex is the empty
string so each reader degrades to "no icebox" rather than to "every section".

**That degradation is the reader's to complete, not the library's.** The empty
regex only *carries* the "no icebox" answer; an awk consumer must test it
explicitly (`iceboxre != "" && $0 ~ iceboxre`), because awk's `$0 ~ ""` matches
every line and an unguarded match therefore degrades to exactly the "every
section" reading the empty string was chosen to avoid. Any reader of an
optional section regex owes that guard.

`QUEUE_TASK_SECTIONS` is that composition itself — the active sections, the
deferred section, and a configured icebox, in configured order — exposed rather
than consumed and discarded. `QUEUE_TASK_RE` is built from it, and a reader that
needs the sections *individually* rather than as one alternation
(§bin/queue-counts.sh) reads the array. One composition, two shapes of the same
answer: a second reader recomposing the set from the three knobs is the drift
axis a shared adapter exists to remove.

`roadmap_entries` — `native/src/queue.rs`'s, this library holding no counterpart
— is the single `[roadmap:]` and `roadmap-summary:` parse, shared by the roadmap
arm (§The roadmap arm) and `check-roadmap-fresh` so the emitter and the gate can
never disagree about what an entry claims — the same one-adapter rule the section
regexes follow. It walks the live task sections in queue order and returns one
record per entry carrying **either** marking: the entry's `[roadmap:]` tag count,
the raw field text, the slug, the `roadmap-summary:` declaration count, and the
declaration's text. Emitting on either marking rather than on the tag alone is
what lets assertion C see a dead declaration — an entry the tag has fallen off is
exactly the case a tag-triggered walk cannot report.

**The record is a typed value rather than a TSV line, and that difference is a
deletion rather than a translation.** A tab-separated form exists only because a
shell function's only return channel is stdout, and such a form must carry a
defensive `-` in an untagged entry's field column: a tab counts as IFS
whitespace, so a reader splitting on it coalesces an empty column and silently
shifts every field after it. In-crate the five fields travel as fields, so the
field-count and embedded-tab hazards have no spelling and an absent field is
simply empty.

The in-body citation scan (§The tag algebra) is deliberately **not** here: it
has one reader, and this library holds adapters two or more readers share. It
lives in `bin/queue-edges.sh`, which sources this library for the section
regexes and `queue_live_slugs` it does share. A second reader of body-position
slug tokens is what would promote the scan into this roster.

**The entry lead-line grammar is here, and it is here by that same rule rather
than by convention.** `QUEUE_LEAD_RE` (a bullet opening an entry) and
`QUEUE_SLUG_BOLD_RE` (the bold slug token inside it) are exported globals like
the section regexes, because three readers now share them: `queue_live_slugs`
above, `bin/queue-edges.sh`'s bullet scan, and that tool's history walk
(§bin/queue-edges.sh), which asks the *same* question of an older revision. A
grammar answering one question in three places is the shape this library exists
to hold, and a history walk carrying its own spelling would let the retired set
and the live set disagree about what an entry is.

Both are written with **bracketed literals** (`[*][*]`) rather than backslash
escapes. They reach `awk` through `-v`, where awk's string-escape pass runs
before the regex engine sees the value and eats a `\*` with a warning; the
bracket form has no escape to lose. This is mechanism, not style — the same
value spelled with backslashes is a different regex by the time it is applied.

**The one-adapter guarantee is split for `queue_live_slugs` and the section
regexes, and the split is machine-held rather than filed as debt.** The gates
that read the queue ported to the binary substrate (gate-sdk/SPEC.md §Porting a
gate to the binary substrate), so `check-queue-slug-liveness` and
`check-task-conservation` call a Rust reimplementation of that helper while
`bin/queue-edges.sh` keeps calling this one; the same is true of the section
regexes, every one of which a `bin/` script still reads directly. The split is
**permanent** — every `bin/` script in the kit still sources this library
(`grep -rl lib/queue.sh queue-kit/bin queue-kit/checks` is the roster, and the
`checks/` half of it is empty now that every gate here dispatches to the binary)
— so the
shell form cannot be deleted the way a ported primitive's is, and a port-time
byte-identity proof would expire at the next edit to either side. What holds the
two equal from here is `gate-tests/queue-lib-parity.test.sh`: it feeds one canned
corpus to both and compares their **classification** of it byte for byte
(§The queue format owns why classification is the comparable thing), which is
criterion 6's *machine-held* disposition rather than its duplication-absent one
(gate-sdk/SPEC.md §The port-candidate criteria, criterion 6).

The obligation covers **one helper — `queue_live_slugs` — and every derived
global this library exports** (`grep -n '^QUEUE_[A-Z_]*=' queue-kit/lib/queue.sh`),
and nothing else is owed one. The crate's `done_slugs` has no counterpart here and needs none: a shell
twin with no caller is a duplication removal disposes of, which enforcement-first
ranks above gating it, and the bound on that disposition is undocumented surface
— no section named it, which is what separates it from the section regexes, every
one of which is documented and load-bearing.
`roadmap_entries` is **not** in the split
either, for the opposite reason: its only two consumers are the roadmap arm and
`check-roadmap-fresh`, and both now sit on the **crate** function. The exclusion
is therefore **satisfied on the other substrate, not repealed** — the two still
cannot disagree, by the identical argument, and what would have spent it is
porting exactly one of the pair, which is why the arm and the gate landed in one
commit.

The loader sources the consumer config, then a `<config>.local.sh` overlay
beside it when present — last write wins. This is the tracked-name /
gitignored-value split (the `msg-patterns.local.list` precedent): a private
sink value tracked in `queue-config.sh` would itself be the leak, so it lands
in the gitignored overlay instead. The overlay is optional — its absence is
fail-open, not an error.

### The queue-index arm

A **non-gate arm of the binary** (gate-sdk/SPEC.md §The non-gate arm), reached
through the battery runner's `--emit` front-end:
`run-gates.sh --emit queue-index [<flags>] [<queue-file>]`. It registers in the
emitter table under the derived spelling `--emit-queue-index`, stays outside
`--list`, and owes no `.gate` descriptor, no `gates.list` registration and no
`good/`+`bad/` fixture pair. The shell tool it replaces is deleted rather than
retained as a dispatching shim: a shim keeps the interpreted lines the port
exists to retire and adds a second entry point into the emission path, which the
class forbids.

**The front-end is not optional dressing.** A caller that invoked the binary
directly would resolve platform defaults and silently ignore every consumer
override; the front-end sources the shell library and supplies the bridged
environment in front of the arm. The arm is configured — its bridged reads are
declared beside it in the emitter table — which is why it is a table member
rather than a hardcoded top-level flag: a hardcoded flag receives no
configuration at all, and an arm that cannot see `QUEUE_KIT_ICEBOX_SECTION`
silently drops the tally in every consumer that configures a tier. The derived
regexes the shell library built (`QUEUE_ACTIVE_RE` and its siblings) were never a
configuration surface, only that library's internal spelling of these knobs, so
none of them crosses into the arm and nothing is lost by their not crossing.

**The three modes stay three modes on one arm**, selected from the arm's own
argv tail — the emitter type is defined over an argv slice precisely so a mode
rides as a flag rather than needing a second arm. The refusal to grow a *fourth*
mode is unchanged, and so are the three grammars: they are the contract this port
preserves.

**One observable moved, and it is stated rather than absorbed.** A `--extent`
slug that resolves to nothing reported `queue-index: slug not found: <slug>` at
exit 1 and now reports the arm's error at exit **2**, because the emitter type
returns a `Result` and the dispatcher maps every error arm to 2. Preserving the
old code would mean widening the class's return contract for one mode's error
path; no caller reads it — `extent`'s caller is a session reading two integers —
so the widening is not taken. Every mode's **stdout** grammar is byte-preserved.

Compact surface of the queue for task selection — the iteration header line
if present, then every top-level active entry as a one-line title plus tags,
ready (`•`) vs blocked (`✗`) marked from the `[blocked-by:]` tag alone. Tags
are stripped from the one-line title leaving no residue of the separator they
sat beside, so an entry whose lead line is nothing but tags has no title and
renders as the bare slug rather than a dangling `—`; `[blocked-by:]` and
`[drain-exempt: <reason>]` are re-echoed after it — the two tags a picking
session acts on without opening the entry body.
`--extent <slug>` prints the inclusive line range of one entry's body (parent
slug → whole subtree; boundary = next sibling-or-shallower bullet, heading,
`---`, or EOF, including the trailing blank so a range deletion leaves no
double blank). `--collapse-deferred` replaces the deferred listing with a
per-`###`-subsection tally — generic over whatever subsection names the file
has (a hardcoded tally table would be consumer rule content), with
entries under no subsection tallied as `(top)`.

A configured icebox contributes exactly **one line** to the index —
`<section>: N entries` — and never an entry listing, on **both** `index`-mode
renderings (`--collapse-deferred` is a flag within that mode, not a mode of its
own; the arm's modes are `index`, `extent`, and `icebox-candidates`). The
tally must not reach `extent`, whose contract is two integers and nothing else.
The reasoning is the surface's: this output is embedded in the always-loaded
session-context brief, so listing the tier would re-import the very tokens the
tier exists to remove, while a bare count keeps it visible. An unconfigured
icebox emits no line at all.

`--icebox-candidates` prints the closing stage's eviction worklist over the
deferred section: one line per entry whose defer date (§The queue format) is
older than `QUEUE_KIT_ICEBOX_AGE_DAYS` **and** whose cost field opens in the low
class, carrying the entry's line count and that opener. An entry with no defer
date is listed as `(undated)` and an uncosted one as `(uncosted)` rather than
filtered out — an absent input appears rather than vanishing.

Each row is led by the `•`/`✗` mark the index rendering already uses: `•` for an
entry §The icebox tier's eligibility rule admits, `✗` for one it excludes. An
excluded row prints its **cause** in place of the cost opener, because the opener
is inclusion evidence and decides nothing once a categorical exclusion has
settled the row. The cause names the trigger class — the `[roadmap:]` tag, a
dated `recurrence:` declaration, or the first live slug named in file order,
which stands for the class rather than enumerating it. A slug counts as *named*
only where it stands as a whole token, the neighbouring bytes falling outside the
slug alphabet: a substring hit inside a longer slug is a different entry, and
holding a row against it would invent a trigger nobody wrote.

Both filters are deliberately **non-load-bearing**: this is a worklist filter
in a projection tool, not a threshold in a gate, so miscalibration costs a
longer or shorter review list and never a wrong disposition. That is what
dissolves "an eviction age threshold needs a policy owner" — the *judgment* is
the closing stage's, held to §The icebox tier's eligibility rule, and the
worklist only bounds how much it must look at. Matching a cost-class opener on
prose would be an unacceptable heuristic in a gate; in an advisory worklist it
is exactly the right ceiling. **A pool younger than the age threshold yields an
empty worklist**, which is correct and is not a reason to seed a tier by hand
against the tool: eligibility is the rule, the worklist is the convenience.

**The worklist applies the tier's categorical exclusions, reopening a ruling that
held them off.** Operator-ruled 2026-08-18 that it deliberately would not, on
five consecutive measurements at zero precision: filtering the machine-readable
half would have moved the eligible count from zero to zero, and the other half
read as a prose-parsing fork nothing justified. **Reopened and taken by operator
ruling 2026-08-24**, on measurement rather than on preference. Both halves of the
ground fell: all three triggers are mechanically decidable — the `[roadmap:]` tag
is a lead-line tag, the dated `recurrence:` declaration is one indented body line
with a fixed lead token (§The queue format), and a named live slug is the
slug-liveness test the shared adapter already answers for §check-task-names — and
zero precision is what makes the exclusions worth *reporting*, since a worklist
whose rows are all re-adjudicated by hand pays that read at every close.

**It prints them rather than filtering, which is this arm's own rule and not a
new one.** `(undated)` and `(uncosted)` above already list an entry that fails an
input test instead of dropping it, and a categorical failure is the same case
under the same sentence. Two further grounds hold in the same direction: dropping
the row would turn an advisory projection into a silent disposition, which is
what the non-load-bearing ruling above exists to prevent; and it would deny the
closing stage a row §The icebox tier requires it to have recovered *before*
ruling on the entry. So the division of labour is unchanged — the arm reports,
the closing stage rules, and a row the rule excludes is still a line read rather
than a defect. What changed is that the line now carries its reason instead of
leaving every close to re-derive one.

No queue-mutating tool is added — `--extent <slug>` already yields the line
range an eviction deletes.

Both index renderings append an **attention block** when the `## Lessons Learned` section
carries `[attend]` entries (§The tag algebra): each such entry's lead line,
capped at `QUEUE_KIT_ATTEND_CAP` (default 3) with overflow noted as
`(+N more [attend])`. Lead lines only, never bodies — the block is
always-loaded through the session-context hook that embeds this output, so the
cap is its token budget. This is the inbound lesson channel reaching every
later session of the iteration with zero consumer-hook edits.

The age cutoff keeps its `date -d` derivation rather than moving to an in-crate
civil-date computation, and the reason is behaviour preservation: `date` resolves
the operator's local zone where an in-crate computation would resolve UTC, and a
port that shifts the cutoff by a day is not the behaviour-preserving port this
was. Its refusal on a `date` without `-d` survives with it.

This is a *task-selection* surface, walking bullet lead lines. Its sibling
`bin/queue-edges.sh` walks entry **bodies** to aggregate citations — a
different question over the same file (§bin/queue-edges.sh). They stay two
tools rather than one with a fourth mode, because folding them would give
one tool two jobs and two output grammars.

### bin/queue-counts.sh

One job: the size of each **task section**, for a caller that wants the shape of
the queue rather than its contents. It emits one `<section-name><TAB><count>`
line per task section, in configured order, and nothing else — no flags, no
modes, one output grammar.

The section set is **derived, never listed**: it is `QUEUE_TASK_SECTIONS`, the
same composition `QUEUE_TASK_RE` is built from (§lib/queue.sh). So
`QUEUE_KIT_DONE_SECTION` is out because Done is not a task section, a configured
`QUEUE_KIT_ICEBOX_SECTION` is in because the icebox is a live one, and a consumer
who renamed their sections gets their own names back. Nothing here enumerates a
section name, which is what keeps the vocabulary inside this kit when a caller
outside it renders the numbers.

The counted unit is the **top-level entry bullet** — the same unit
§The queue-index arm lists, so the index and the counters cannot report different
sizes for one queue. Not lines, and not bullets: an indented bullet inside an
entry body is body.

**Why a second tool rather than a fourth mode on the `queue-index` arm.** That
arm's modes are fixed at `index`, `extent` and `icebox-candidates`
(§The queue-index arm), on the stated grounds that folding jobs together gives one
tool two output grammars — the same refusal that keeps `bin/queue-edges.sh`
separate. A tally keyed by section name is a different job with a different
grammar, so it lands beside the index rather than inside it. Recorded here
because the next reader meeting two queue tools would otherwise read the split as
an oversight and merge them.

**It is invoked as a subprocess, never sourced.** `lib/queue.sh` exits 2 at
source time on a missing `QUEUE_KIT_CONFIG_FILE` and on any malformed-config
assertion — correct for a gate, and fatal for a long-lived caller that sourced
it. A subprocess turns that contract into a non-zero exit and empty stdout, which
a caller degrades on. The caller this was built for is delegation-kit's
statusline template, whose own contract records the degradation
(delegation-kit/SPEC.md §The statusline template).

### bin/queue-edges.sh

The inbound-citation aggregator: a tool, not a gate (no `# graph:` manifest),
following the queue-index precedent. It reads the queue, writes **stdout only**, and
mutates nothing.

```
usage: queue-edges.sh [--inbound <slug>] [queue-file]
  default: live slugs with inbound edges in queue order, then retired targets
  --inbound <slug>: the inbound set for one live or retired slug
```

**Inbound only.** An outbound view is refused for want of a reader: an entry's
outbound edges *are* its own body, which a session asking the question is
already reading. Inbound is the direction invisible without a scan of the whole
file — which is the entire finding this tool answers. Nothing anywhere sums an
entry's inbound edges into that entry's own cost and benefit, so a survey that
reads every sibling entry individually can still misrank a unit that several of
them separately depend on. The tool is that missing sum, which is also what
makes splitting an entry safe: a split scatters an entry's weight across
siblings, and this is what adds it back up.

Targets print in queue order — the order every other reader of this file walks
— each with its inbound count, then one line per citing edge. A slug with no
inbound edges is absent from the default listing and yields empty output under
`--inbound`; that is the normal case, not a finding. A `--inbound` slug that
resolves to neither a live nor a retired target exits 1 with a message on stderr
rather than printing nothing, because silence from this tool has to mean "no
inbound edges" and nothing else. Widening the addressable domain to retired
slugs grew that domain and left the meaning of silence exactly where it was.

**The resolution set is the live slugs plus the file's retired ones**, and a
citation resolving to a retired slug is an edge like any other, marked as such.
Retired targets print **after** the live block, in a trailing block, each as
`<slug> (<N> inbound, retired)` above its edges in the same two-column shape.
They sort **alphabetically**: a retired slug has no queue position to order by,
and inventing one — last-seen revision, say — would be a datum with no reader.

A **retired** slug is one that held a top-level entry lead line in some earlier
revision of the queue file and holds none now. That is the whole discriminator,
and it is what makes the report readable rather than merely longer. The naive
alternative — listing every backticked token that resolves to no live entry — is
dominated by commit SHAs, content digests and ordinary backticked words, because
the token grammar `[a-z0-9][a-z0-9-]*` is not a slug detector. Splitting the
unresolved remainder against the ever-live set leaves a small retired minority,
and it is the half a reader wants: on this repo's own corpus, both attested
instances of a citation instructing a session to sequence against work that no
longer exists fall in it. Run the tool for the current numbers on your own
corpus — they are one adopter's queue statistics, not a property of the kit.

**One tool, one job, and this is not a second one.** §bin/queue-counts.sh's
refusal — folding jobs together gives one tool two output grammars — is honoured
rather than worked around. The job is *aggregate in-body citations by target*,
and a retired target is a target whose entry has been disposed of, not a
different question over the same file.

**The retired set is derived from the file's own history, so nothing is
maintained.** One `git log -p --format= -- <queue-file>` pass at start-up, its
added, removed and context lines matched against `lib/queue.sh`'s exported
lead-line grammar (§lib/queue.sh) — the same grammar the live reader applies,
never a second spelling. `git` joins `awk` as a dependency of this tool alone;
the section's "reads the queue, writes **stdout only**, and mutates nothing"
contract is untouched, and now has a second input. The cost is a *tool's* budget
rather than a gate's: on a queue file with roughly fifteen hundred revisions the
whole pass measures well under a second, and it scales with history depth rather
than with queue size.

**Two degradations are declared rather than discovered.** A queue file **not in
a git work tree**, or a `git` that is absent, yields an **empty** retired set and
the tool prints its live block alone — byte for byte the output it printed before
retired targets existed, which is why the degradation is silent-safe rather than
misleading. And the derivation sees only the history **this clone has**: a
shallow clone, or a queue file whose history was rewritten under it, reports
fewer retired slugs and never more, so the report **under-**claims and never
invents. Both are stated because the natural reading of "derived from history"
is that history is total, and here it is whatever the clone holds.

**Each edge carries the citing entry's slug and its citing line verbatim**
(surrounding whitespace stripped, since the output re-indents). This is what
buys a relation's *kind* without declaring a vocabulary for one: the nature of
the relation is already written, in the citing author's own words, by the
person who understood it. Quoting it beats a one-token classification from a
fixed set, and it is free. It also keeps the tool honest about precision — a
citation that is a passing mention rather than a relation is *visibly* a
passing mention once its line is on screen, so a reader discards it in the time
it takes to read one line. Recall is what the surface needs, and a
high-recall list whose noise self-identifies is the right trade.

A citation is attributed to the **nearest preceding slug bullet**, so a
sub-task cites in its own name rather than its parent's, and a lead line
contributes its `[blocked-by:]` tag alone — never its prose, which is title and
tags rather than relation.

The body-citation scan lives **in this tool, not in `lib/queue.sh`**: it has
exactly one reader, and the library's rule is shared adapters (§lib/queue.sh).
It reuses that library's section regexes and `queue_live_slugs`; a second
reader of body-position slugs is what would move it.

**No tracked projection, and no freshness gate.** A tracked artifact needs a
reader who cannot run the tool, and there is none — the one consumer is a
session with a shell. A generated public page like the roadmap is tracked
because its audience is outside the repo; this has no such audience. Against
that, a committed copy of derived edges over what is typically a repo's
highest-churn file would restale on essentially every queue edit, buying a
per-commit regeneration tax for zero benefit. Derivation-first is satisfied by
deriving on demand: that rule's "generate and freshness-gate" clause governs a
copy that is *needed*, and this one is not. The `queue-index` arm writes nothing
and is gated by nothing, for the same reason.

**Not a gate, either.** There is nothing to red: an entry with no inbound edges
is normal, and an entry with many is not a defect. The adjacent temptation —
redding on a backticked token that resolves to no live slug — is refused
outright, because those are overwhelmingly legitimate citations of landed work
(§The tag algebra), and `check-queue-slug-liveness` already owns the liveness
invariant on the surfaces where a dead reference *is* a false claim. Extending
it inward would red on good prose.

**The refusal and the retired block are the same ruling, not a softening of
it.** Reporting is what a no-red posture always left available, and the retired
block is only the listing half: an entry citing landed work is legitimate prose,
so a retired edge is a *finding to read* and never a violation. Nothing about
it is stricter than the day the refusal was written — the tool gained an output
section, not a verdict.

**Two named readers, at two named transitions**, because a report nobody reads
is the failure this listing was built to end:

- **scope, at its ranking survey** — the primary, and the attested harm's own
  moment: a citation pointing at disposed work is a false premise in a survey
  input at exactly the point a session decides what to promote. Scope already
  runs this tool there, so it gains the retired block as one more thing that
  call's output carries and no new step.
- **close, at the gap-inbox drain** — the corrective transition, where an
  instance found is fixed inline on its owning entry. This is that stage's
  **first** invocation of this tool: its drain dispositions bullets from a
  different file entirely, so unlike scope's it is a new command in the step
  rather than a wider reading of an existing one. Still no new mechanism — one
  more command a session already running shell tools invokes.

Neither stage gains a gate and neither gains a refusal.

### The roadmap arm

The public-roadmap emitter: a **non-gate arm** of the gate binary
(gate-sdk/SPEC.md §The non-gate arm), reached as
`bash gate-sdk/bin/run-gates.sh --emit roadmap`, following the queue-index
precedent. The bare arm prints the generated block to stdout; `--write` splices
it between the markers in `QUEUE_KIT_ROADMAP_FILE` through the crate's shared
marker writer, leaving every byte outside them untouched.

**Table membership is forced rather than chosen, and the alternative is the
failure mode that looks like success.** An arm receives no configuration and only
an emitter-table member is bridged, so this tool ported as a hardcoded top-level
flag would resolve platform defaults and silently ignore every consumer override
of the horizons, the tracks, the queue path or the marker name. The table's knob
column is what keeps `QUEUE_KIT_HORIZONS` and `QUEUE_KIT_TRACKS` on the caller's
side of the bridge, which is the provenance seam holding: the crate ships the
projection *grammar* — the headings, the bullet shape, the placeholder, the
trailing blank — and not one lane name (§The tag algebra).

The section trio (`QUEUE_KIT_ACTIVE_SECTIONS`, `QUEUE_KIT_DEFERRED_SECTION`,
`QUEUE_KIT_ICEBOX_SECTION`) rides the same roster, because the shared adapter is
what scopes the scan and a knob the bridge does not carry is a knob the arm
cannot read. It reads the live task sections only — active, deferred, and a configured icebox,
never the done section, because a shipped item is history, not direction. The
icebox enters that walk on purpose even though no icebox entry can be
projectable: excluded, a `[roadmap:]` tag that drifted into the tier would
silently drop a public commitment; included, it reds (§The icebox tier).

The emit grammar is a contract on both sides: `check-roadmap-fresh` byte-compares
it and a reader consumes it as the page's body.

- One `### <horizon>` heading per configured horizon, in the knob array's own
  order — never a sort the crate imposes — each
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
the last bullet and renders *inside* that list item. Both sides of the freshness
compare strip their trailing newlines, so the byte-compare is unaffected — the same arrangement the value-rollup emitter uses to keep a
table from swallowing its marker.

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

- **(A) Freshness.** The committed marker block byte-matches the roadmap arm's
  emission, called **in-process** rather than spawned — the arm ported in the same
  commit, so there is no shell left to reach — on the `check-footprint-fresh` /
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

**Assertion B runs before assertion A, and the ordering is carried deliberately.**
A tag naming an unconfigured horizon is silently dropped from the emission, so a
freshness verdict taken before the fields are validated would pass a page that
quietly lost an item. That is a correctness property of the pair of assertions
rather than an implementation accident, and it is written here because a reader
taking the three assertions as an unordered set would lose it with nothing
reddening.

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

**What the fixture pair proves is bounded, and it is explicitly not the arm's
parity proof.** Both cases steer assertion A off the live emitter, so the pair
would have gone green over an arm with no implementation at all. The pair proves
the comparator and the two-argument plumbing; what proved the emission was the
transition-scoped parity run against the shell emitter, held while both existed
(gate-sdk/SPEC.md §The first cohort, and the rule that selects the next).

The declaration is `checks/check-roadmap-fresh.gate` — hermetic, `precommit`,
binary-dispatched. Its `couples=` names every crate module the gate reaches
transitively, the emit module and the two shared with the emit side of the
compare included: the generated hook's `staged_matches` trigger is derived from
that field, so an omitted module leaves the gate registered and green while the
page it holds goes stale at commit time.

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
no-match) is fail-closed (exit 2). The required set is the configured array
**plus the derived icebox heading** (§Layout and configuration), which is what
closes the half-configured hole by construction. The `# graph:` couples the
queue file at `tier=precommit`.

### check-queue-entry-budget

Invariant, one statement from three sides: **a deferred entry is a costed filing
— bounded above so it is not an inlined amendment, bounded below so it is not a
flag-and-skip, and bounded in what it may displace, so a bound on filing never
becomes a bound on the record.** The first two sides are about the entry's
*size*; the third is about what the cap *spends* to stay inside it. Three
assertions:

- **(A) Size.** No deferred entry exceeds `QUEUE_KIT_ENTRY_LINE_CAP` **counted**
  lines. An entry's **extent** is the lead line through the line before the next
  bullet at the same or shallower indent — the same extent
  the `queue-index` arm's `--extent` yields, so the range the gate measures is the
  range an eviction deletes. Its **count** is that extent less **at most one**
  line matching the `recurrence:` declaration grammar (§The tag algebra). Extent
  and count differ by that one discounted line and by nothing else, which is what
  keeps the equality above a statement about the *range* while the cap binds the
  *count*. A sub-task nests inside its parent's extent and is measured as its own
  entry too — and claims its own at-most-one discount, so a parent whose extent
  holds two declarations still discounts one.
- **(B) Icebox shape.** Every icebox entry is exactly one line; a continuation
  line under an icebox bullet is a violation. Skips clean when
  `QUEUE_KIT_ICEBOX_SECTION` is empty, the empty-knob behavior
  `check-queue-slug-liveness` originates.
- **(C) Cost present.** Every top-level deferred entry carries a
  `Cost while deferred` bold lead-in. Deliberately **not** applied to the
  icebox: a one-line entry cannot carry the field and does not need to —
  membership in the tier is itself the cost declaration (low, non-rotting, no
  live trigger). The requirement binds exactly where the section does not
  already imply the answer.

Calibration: `QUEUE_KIT_ENTRY_LINE_CAP` defaults to `50`. The cap's job is to
keep compression from regrowing rather than to force the initial cut, so it is
calibrated at the tail rather than the body — above the 75th percentile of a
real pool, with no natural break in the distribution to read a value off. It is
a stated policy with a stated purpose rather than a derived one, and it is a
knob for exactly that reason. The scan is line-local: the cost field's bold
lead-in must sit on one line, the same reason a tag must sit on its lead line
(§check-tag-lead-line), and a lead-in split by a reflow reads as absent. (C)
binds top-level entries only — a sub-task is covered by its parent's costing.

**The active sections are uncapped**, and the reason is structural: an active
entry's residency is one iteration by the drain rule, so it has no carry to
cap. The carry problem is the deferred pool's alone.

**Clean-path headroom.** When all three assertions hold, the clean line is
followed by one line per Deferred entry — a sub-task included, since it is
measured as its own entry under (A) — naming that entry's headroom to the cap:
the same count assertion A already derives, one subtraction away, so exposing
it costs no new computation and mints no new name. This is exposure, not a new
capability: the enforcing member already computes each entry's count in order
to enforce the cap, so a session sizing an edit reads the measurement instead
of hand-rolling it — a hand-rolled probe carries the risk of a second spelling
of the cap, a second implementation of the parse, and a mismeasured entry
boundary that reads as a false cap overrun. The rejected
alternative was a `--headroom <slug>` mode on the `queue-index` arm: it mints a
name against that arm's standing no-fourth-mode refusal, where this
exposure spends nothing beyond a print at the compiled assertion's existing
computation.

**No collision counter is minted either, and this line is where that decision is
recorded** rather than left as the absence of one. A count of how often the cap
displaced a mandated write would be a second measurement of a quantity this
headroom line already prints, maintained by hand, against a criterion (the split
test below) that reads an entry's *composition* rather than its collision
history. What a firing count would have been evidence for, the split criterion
decides directly.

*Why one `recurrence:` line is discounted, and why exactly one.* The line is
**fixed-shape and width-bounded** — §The tag algebra rules its ceiling to be
`check-queue-wrap`'s budget and rules reaching that ceiling the *correct*
complaint, so the discount cannot let an entry grow without bound. It is exactly
the **generated-shaped content** that argued content was otherwise spent to seat,
so making the count blind to it removes that trade rather than arbitrating it.
And it is **one line, not a grammar-wide exemption**: the declaration's own rule
is one line per entry with dates appended, so one line is the whole of what that
form can claim — §The tag algebra refuses the one-line-per-recurrence variant
partly *because* it grows an entry linearly against this cap, and a grammar-wide
exemption would retire that refusal's ground as a side effect. The match is the
declaration's own grammar — the `recurrence:` lead token, a slug, then at least
one ISO date, the shape drift-kit's `kpi-incident-recurrence` already reads —
with no entry-boundary or self-slug condition added, since the at-most-one bound
is what does the scoping. The discount narrows the **count** only and removes
nothing from the scan, so (B), which counts icebox continuation lines, and (C),
which asserts a lead-in is *present*, cannot flip; (A) itself reds on
*exceeding* the cap and is monotone in the count, so a smaller count reds
strictly less.

**A mandated write is the class the cap has no self-served answer for.** A
mandated write is a write on a deferred entry that another governed contract
obliges the writing session to make **in the commit it is already making**, and
whose obligation is citable to that contract by name. Two instances exist, both
already in the tree rather than invented here: a judged `recurrence:` date and
the grounds it is read from, which must land in the commit the judging session is
already making because that commit is the audit artifact (lifecycle-kit/SPEC.md
§The committed gap inbox); and a ruling recorded onto the entry it rules, under
the recording-in-the-moment rule below. Its **producers** are the sessions those
two contracts already bind — any session that judges a recurrence, the gap-inbox
drain being its mechanized instance, and any session recording a
ruling — both running today with no new trigger, no new field and nothing to
configure; its **consumers** are that same session reading this section when
assertion A blocks it, a path the gate's failure text already routes it down, and
lifecycle-kit's judgment contract, which cites the class by name for the write it
mandates. **What is not a mandated write, because a class that excludes nothing
is not a class:** the session's own evidence for a claim, a further ground for a
claim already made, a cross-reference it would like to add, a correction it could
equally make in a later commit. All of those keep the reliefs they have. The test
is not *how important is this* — every session believes its write matters — but
*does a named contract oblige this write in this commit*.

**Compression is lossless; relocation is self-served for a mandated write, and
the split stays authorization-gated.** An entry that will not fit is compressed
by *answering* grounds, never by dropping or summarizing them away; a ground that
survives unanswered is relocated into a distinct linked entry. The gate cannot
hold this: it sees an entry's current extent, and judging whether a removed line
was answered or discarded is semantic. So the rule is a stated authoring
contract — and that third relief is **two acts wearing one name**.

- **Relocating grounds** into an entry that **already exists and already owns the
  ground's subject** is self-served when, and only when, what the cap blocks is a
  mandated write. The relocating session cites the mandating contract in the
  commit that makes the relocation, and names the target with a
  single-backticked slug in the entry body — already a citation under §The tag
  algebra and already aggregated by `bin/queue-edges.sh`, so the link needs no
  declaration of its own. A `relocated:` declaration is refused on the ground
  §The tag algebra refuses `relates:` on: it would cost a counted line against
  this very cap, on the entry least able to pay it.
- **Splitting the unit** — minting a *new* entry to hold what would not fit —
  carries its own authorization, unchanged: a parent session (the iteration
  lead), or the operator in the absence of one, grants permission to split and
  issues the recipe with the ruling. A session blocked by the cap does not
  self-serve *that*, the same lineage as a `check-stage-entry` assertion C waiver
  (lifecycle-kit/SPEC.md §check-stage-entry), and the gate's failure text
  therefore cites this section rather than inlining the recipe.

  **An entry is a split candidate when it carries two or more deliverables that
  can take different dispositions** — where *different dispositions* means one
  could be promoted, deferred, declined or closed while the other stays open. The
  test is executed by reading the entry's deliverable statements and asking
  whether ruling one leaves the other unruled. If it does, the entry is two units
  wearing one slug, and its grounds are two ground-sets sharing one cap.

  **The counter-class is what makes it a class**, and it is stated with the
  criterion rather than left to inference: an entry accumulating **further
  grounds for one deliverable** — recurrence evidence, a repeated measurement, an
  answered objection, a corroborating firing — is **not** a split candidate
  however hard it collides. Those are one unit's history, and splitting them
  mints exactly the ranking peer competing with its own parent that the
  two-acts paragraph below names as the sharpest displacement shape. Compression
  by answering is the correct relief there and stays the default.

  *Why dispositionability, and not size, age or ruling-count.* **Size** is the cap
  itself, so it would make the criterion circular. **Age** is refuted by the
  measurements a colliding pool actually produces — the collider is the most
  recently ruled-on entry rather than the longest-lived, which is the opposite of
  what a first reading of a collision log suggests. **Ruling-count** is the near
  miss and the one worth stating, because collisions genuinely do concentrate
  where rulings accumulate; it is refused because it is a **symptom shared by
  both classes**. An entry accumulating rulings on one deliverable collides
  exactly as hard as one accumulating rulings on two, so a criterion that cannot
  separate them licenses the split that mints a competing peer. Dispositionability
  separates them and is decidable by inspection, which is what an authoring
  contract needs.

  **The criterion is read at the collision, and it is an input to the
  authorization rather than a licence.** The blocked session states which side of
  the test its entry falls on when it asks; the authorizing session rules with
  that statement in front of it. What changes is the **shape of the ask**, never
  who answers it. Without a test the authorizing session has only the blocked
  session's summary of its own entry, so the answer is a judgment call re-derived
  per firing — which is the mechanism behind the pattern a collision log shows:
  relief was always available, so the question was always answered by the
  cheapest relief rather than by composition. **A denied split is not a dead
  end**, stated so the criterion does not read as gating relief: an entry that
  fails the test still has compression by answering and the self-served
  relocation, which are the reliefs it should have been using, and an entry that
  passes and is denied anyway gets the denial's ground recorded on it as any
  other ruling would be.

  **An authorized split records itself through the citation grammar already
  here** — the parent names the child and the child names the parent, each with
  the single-backticked slug §The tag algebra already rules a citation. A
  `split-of:` declaration is refused on exactly the ground `relocated:` is
  refused on above, so the two refusals read as one rule: it would cost a counted
  line against this very cap, on the two entries least able to pay it — the
  parent that just overflowed and the child carrying what would not fit. The
  backticked slug costs no line of its own, because it rides prose the split is
  writing anyway. **Bidirectional rather than child-to-parent alone**, because
  the failure a one-way citation leaves is the one this whole class is about: a
  reader meeting the **parent** sees an entry whose deliverable set silently
  shrank, with no surviving statement of what left or why, and re-derives the
  missing half as a gap — the same information loss the compression rule forbids,
  arriving through a split instead of through a deletion.

  **The pair is an aggregated edge only while both slugs are live**, on §The tag
  algebra's resolution rule, and the bound is stated because the parent is the
  end that reaches it first: a `## Done` entry is a bare slug, so it leaves the
  live set and takes the parent's citation with the rest of its prose, and the
  edge silently stops being an edge. That is the correct outcome rather than a
  loss — the parent-side citation exists to stop a *live* parent reading as an
  entry whose deliverable set shrank, and a completed parent has no open
  deliverable set to misread. What it means is that the split's durable record is
  the **child's** citation, and the parent's is owed for exactly as long as the
  parent is open.

*Why the line falls between the two acts.* Grounds are not rankable work, so
moving them into an entry that already owns their subject mints no ranking peer;
a new entry *is* a new unit, competing for the scope attention that ranks both,
which is a scope-class judgment and keeps the authorization. That is what stops
the sharpest displacement shape — an entry too full to hold its own design half,
so the half is filed as a separate entry competing with its own parent — from
becoming routine under a blanket relief. And the authorization is safe to drop
for the narrow act because a mandated write is by construction not
discretionary: its obligation belongs to another surface and is citable, so a
reader can check the trigger even though no gate reads it, and the act is
net-negative on the parent by construction — a relocation that does not reduce
the parent's extent is not a relocation, which assertion A enforces from the
other side. **The honest limit, stated because it is real:** where no existing
entry owns the ground's subject, the self-served relief is unavailable and the
session is back to compressing by answering, or to asking. That is the correct
outcome rather than a hole — needing to mint an entry *is* the signal that a
unit, not a ground, is what would not fit.

*Why the cap is not widened for exceptional content.* Assertion A's bound is
the amendment-inlining line above, not a length preference, so raising the
number moves the number without moving the line — and the entries that would
claim an exception are the likeliest ungoverned amendments. A conditional cap
collapses back into authorization anyway, or it is the self-issued exemption
the delegation doctrine already names as the standard failure mode. The
recurrence discount is **not** that widening and must not be read as one: it
changes what the count includes, never the number, and it is unconditional —
no entry claims it by being exceptional.

**A ruling the operator restates from memory is filed in the moment.** The
compression rule above is an authoring contract, so a break in it is silent, and
it surfaces in one recognisable form: the operator supplies a closed decision
from memory because no surface carries it any more. That restatement is the
moment the content is in hand and the cheapest it will ever be, so it is filed
*then*, as a **recording entry** — never noted for a later pass. Deferring the
filing is not a cheaper version of it: the ruling stays exactly where it was
lost from, and the next session pays the same restatement over again. A
recording entry is the queue's holding pen for a decision in flight, never its
home; its deliverable is the ruling landed in the surface that owns it, one
owner per fact as everywhere else, and the entry is done when that surface
carries it. The gate cannot hold this either, for the same reason it cannot hold
compression: nothing distinguishes a sentence that was recalled from one that
was read.

### check-queue-wrap

Invariant: no line exceeds the `QUEUE_KIT_WRAP_BUDGET` gate floor (default
100 columns; the authoring target is ~80). The tools key on the column-0
`- ` lead, so an unwrapped runaway that reflows to column 0 corrupts the
parse; the tripwire fires before that lands.

Calibration: three exemptions mirror the wrapping convention — table rows,
fenced-code blocks, and lines whose own longest token itself exceeds the budget
(URL, path). That third one is narrower than "over budget because of one long
token": a 101-column line whose longest token is 55 is **not** exempt, because
the token still wraps. Width is Unicode code points, not bytes.

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
section — the section the `queue-index` arm now reads, which retires the old "parsed
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
**Single backticks are not an exemption**, and the consequence is worth stating
where an author meets it: an entry body *discussing* a tag it does not carry —
`` `[roadmap:]` `` inside a sentence about the roadmap arm — reds exactly as a
reflowed tag would, because the class table matches the bracketed token and the
lead-class rule only tolerates what the lead line already carries. So prose
about a tag spells its name without the brackets. Widening the exemption to
inline code is refused: the entry-body citation grammar above already puts
single backticks to work as a *live* claim (§The tag algebra), so a scan that
skipped them would have to skip the citation surface too.
Residual accepted gap: a same-class duplicate sliding off while one stays on
the lead — negligible severity, not reflow-realistic.

### check-task-names

Invariant: every entry in a live task section — active, deferred, and a
configured icebox — and every sub-task leads with a valid kebab-case slug,
unique across the file; every done-section entry is a bare slug only; every
`[blocked-by: X]` resolves to a live task, and one pointing at a done slug is
flagged stale.

Calibration: the live task sections share one namespace so resolution works
regardless of where the target lives — an iceboxed slug is a legal blocker
target, because it is unbuilt. The non-bold indented bullet is the documented
prose-note escape; done slugs are validated as tokens but not cross-checked
against the live namespace. Help texts cite this SPEC.

### check-task-conservation

Invariant: every live slug (active + deferred + icebox, sub-tasks included)
present at HEAD is still present in the working tree — live or done. A slug
vanishing from both is a lost task: the absence class diff-review reliably
misses (you notice wrong things present, not right things missing).
Half-applied moves, botched renames, and undocumented withdrawals all fire.

**Eviction to the icebox is conserved by construction** — the slug stays in the
live set, so the gate stays silent and no sanctioned-disappearance escape had
to be invented. The done side is where care is owed: a done entry is matched as
a **bare `- <slug>` line and nothing else** — no bold, no tag, no trailing
prose. An entry *carried* into the done section with its
`- **<slug>** [design-pending] — …` shape intact therefore matches neither the
done grammar nor the live one, lands in no set, and reds here as a lost task.
Dispositioning an entry to done is a **rewrite to its slug**, not a relocation
of the entry; the tag is dropped by that reduction rather than swapped.

Calibration: diffs `git show HEAD:<queue>` against the worktree, hence
`no-fixture:` (a committed fixture has HEAD == worktree; the bad case needs
an uncommitted deletion — infeasible, not a stopgap). No git baseline ⇒ clean
exit (nothing to compare is not a violation). Clearing the done section is
safe: only HEAD's *live* slugs are conserved. A rename intentionally fires —
move the old slug to done, sweep refs.

**The gate is a binary subcommand, and the `no-fixture:` reason survived the
port intact — what it does not excuse is parity.** The reason is structural
rather than a stopgap, so it stays true on either substrate; but a member with
no pair still owes the proof that both implementations agree, and it is paid by
the constructed scenario gate-sdk/SPEC.md §The port-candidate criteria specifies
for exactly this shape: throwaway repositories, the worktree mutated without
committing, both implementations run over the same case and compared byte for
byte. `git` is the one external program the gate invokes — the sanctioned floor
exception — and the branch that reports a clean exit when there is no repository
at all is part of what the scenario covers. The same absence of a fixture case
is why gate-sdk/SPEC.md §check-gate-output reads this member's output contract
off its implementation module rather than off its descriptor.

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
"revisit when" as normal vocabulary and is exempt, and the icebox inherits that
exemption for the same reason: forward-looking phrasing is what a parked entry
is *for*). FP-bearing by
construction (parsing prose intent); the blocking grade is justified by a
silent pick attested in production use and the bounded scope.

### check-queue-slug-liveness

Invariant: on every prose surface named by `QUEUE_KIT_PROSE_SURFACE_GLOBS`,
every slug-shaped bold-code token — `` **`<token>`** `` whose token matches the
slug grammar (`[a-z0-9][a-z0-9-]*`, so a `--flag` mention falls outside) —
resolves against the queue's live slug set (`lib/queue.sh`'s `queue_live_slugs`,
active + deferred + icebox — a page citing an iceboxed task stays green,
correctly: the task is dormant, not landed). A token naming no live task is a
dead membership claim,
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

`queue-config.sh` — the consumer config template: a two-line `# spec:` pointer
to the §Layout knob table (which now includes `QUEUE_KIT_LESSON_SINKS` and its
local-overlay reminder), so the table stays the one owner of the knob roster
rather than a parallel copy in the template drifting against it.
`TASK-QUEUE.md` — a starter queue skeleton: the sections in default order,
one example entry per grammar shape shown under `Technical Debt` (the
`[spec:]`-gated `New Features` carries teaching prose only, since a spec-ready
example would dangle a ref). It ships battery-clean when copied verbatim — the
starter-template conformance contract owned by gate-sdk/SPEC.md §Consumer smoke
— and carries lifecycle-kit's iteration header as inert scaffold so a
combined-tree copy clears the stage gates too.

Shipping this file is also what makes this kit the queue format's owner as far
as an adopter's install is concerned: `checkwright init` seeds their queue from
the template of the first kit in their profile that ships one, and falls back to
an inline skeleton only when none does (installer/README.md §What init seeds).
The declaration is the artifact rather than a roster naming this kit. Until that
selection was resolved over the whole profile at once, no profile reached this
kit first and no adopter ever received the file.

## Out of scope

<!-- prose-enum-exempt: names the two amendment-lifecycle tags specifically; [blocked-by:] is a dependency tag outside that lifecycle, not a dropped task-tag member -->
The amendment lifecycle around `[design-pending]`/`[spec:]` (section-wide
enforcement, promotion procedure, `check-amendment-queue`) is canon-kit's
scope. Code-comment TODO scanning (`TODO(task:<slug>)` resolution against
the queue) couples to source-file conventions and is canon-kit's, on the
governed comment surface (`check-todo-task-liveness`). Task-output readers and delegation tooling are delegation-kit's
scope. A consumer's protocol prose, deferred-subsection taxonomy, and task
bodies are rule content and never ship.
