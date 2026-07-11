# canon-kit — spec discipline for agent-authored components

One canonical spec per component, deltas as short-lived amendment files, and
a star topology across the prose surfaces: every fact has exactly one owning
surface, and every other surface cites it — never restates it. The problem
the kit solves: when coding agents author the specs, design rationale gets
re-derived under build pressure unless it is captured at scope, and a
parallel copy of any gated fact is an un-gateable second source that drifts
silently. The remedy is a lifecycle (amendments are written at scope,
merged and deleted at build) and gates over the copy-shaped failure modes.

The kit carries the lifecycle, the checklists, and its gates; a consumer's
surface names, banned headings, and scan languages are config, with this
repo's layout as the defaults. Requires [gate-sdk](../gate-sdk/) (the gates
follow its four contracts and resolve through its registry); the queue
lifecycle gate reads the tag syntax [queue-kit](../queue-kit/) defines.

## The spec model

Every component has exactly one canonical spec file (default `SPEC.md`) —
complete, current, the single source of truth. A spec has three jobs, and
"copying the code into prose" is not one of them:

1. **Owns semantics** — invariants, ordering, error behavior; the prose no
   other artifact carries.
2. **Names contracts** — every interface is named, and each name is tied to
   the implementation by a gate; a named-but-ungated contract is the rot
   vector.
3. **Never copies a structural definition** — reference the code, do not
   copy it. Anything readable from the code (exact types, field layouts,
   config values) is implementation, not spec content; an ungated verbatim
   copy drifts silently the moment the code changes
   (`check-spec-embedded-source`).

A **SPEC amendment** (default glob `SPEC-*.md`, in the owning component's
directory) is the delta artifact for a designed-but-unimplemented change:
it describes only what is added or changed, is named after the feature
(`SPEC-sqlite.md`, never `SPEC-PHASE3-SQLITE.md`), and is a transition
artifact — merged into the canonical spec and deleted when the work
completes; an amendment never outlives its implementation. The one
sanctioned copy exemption: an amendment may embed a wire-contract delta
(e.g. a fenced proto block) until merge, because it is the design home for
a contract that does not exist yet; the canonical spec cites the contract
file, never re-embeds it. A ruling with no owning component (governance,
workflow, gates) is a **root-level amendment**: same lifecycle, lives at
the repo root, merges into the consumer's rulebook instead of a component
spec.

## The amendment lifecycle

Spec-writing is a scope-stage activity — build sessions never author specs.
A feature task is therefore in exactly one of two states, marked with the
queue tags whose syntax queue-kit defines:

- **Design-pending** — no amendment yet; the entry sits in the deferred
  section tagged `[needs-spec]`, excluded from selection. Every deferred
  entry carries the tag: all deferred work is design-pending by definition.
- **Spec-ready** — the amendment exists; the entry sits in the feature
  section tagged `[spec: <ref>]`, eligible for selection.

The **bidirectional rule**: every feature entry in the active queue carries
a `[spec:]` ref that resolves to a file on disk, and every amendment on
disk has a queue entry pointing at it. Writing the amendment *is* promoting
the deferred entry — without the pairing, design rationale and ruled-out
alternatives get re-derived under build pressure. Technical debt needs no
amendment (it fixes behavior to an existing spec); a debt task that needs a
design *ruling* is design work — it goes to the deferred section
`[needs-spec]` until scope rules on it. Enforced by
`check-amendment-queue`.

The feature/debt litmus is **new names**: a task that adds any name to a
governed surface — a script, a config knob, a file or directory convention,
a tag, a contract another component must honor — is a feature and needs the
amendment, however small the diff; debt converges behavior on names the
spec already carries. The tell for misfiling: a queue entry whose body
carries more than a few lines of design ruling is an amendment inlined
where this gate cannot see it — the entry format has no home for causal
completeness, and the rationale evaporates when the done section is
cleared. The gate cannot decide which section a task belongs in (that is
the semantic residue); it enforces the pairing once the section is chosen,
so the litmus runs at filing time, in the scope skill's triage.

### The causal-completeness check

Before an amendment is ready, every new state, event, and interface it
introduces passes four points:

1. **Producer named and reachable** — what code path, call, or timer
   triggers it; a named producer whose enabling config no deployed
   configuration actually sets is dead everywhere but unit tests.
2. **Consumer named** — what component receives it, by what mechanism
   (which stream, which call, which poll).
3. **Existing integration sections updated** — any affected spec section
   describing the prior flow is updated in the amendment itself.
4. **Every field has a named reader** — for each field on a new message,
   name the consumer that reads it and the transition where it is read; a
   field with no reader is removed, and a field read at one transition is
   not populated at others.

A cross-component causal gap that surfaces during build is not a deferred
TODO: stop, resolve it that session, update the spec before resuming.
lifecycle-kit's scope/build skill templates carry these hooks; canon-kit
supplies the checklist they invoke and the gate behind the promotion rule.

### Merging an amendment (on task completion)

1. Read the canonical spec fully, then the amendment.
2. Integrate — do not append: each addition lands in its proper section,
   and the merged spec reads as one coherent document a reader who never
   saw the amendment can use alone.
3. Delete the amendment file; verify none remain for the component.
4. Move the queue entry to `## Done`, dropping its `[spec:]` tag — the
   amendment it referenced is gone, and `check-amendment-queue` requires
   every `[spec:]` ref to resolve to a file on disk.
5. Propagate removals (grep every spec for names the change retired), file
   discovered gaps as debt tasks, and commit the merge with the work.

The shipped amendment template ends in a Definition-of-Done checklist that
includes causal completeness, merged-with-no-information-lost, the
file-deleted assertions, and gap filing.

## Content tiering — the star topology

Every governed prose surface owns exactly one content tier and *points to*
— never restates — a fact owned by another surface. Which surfaces exist
and what each owns is the consumer's tier contract (each consumer's is its
own instance); the kit's rules are the topology itself:

- **One owner per fact.** A parallel copy of a gated fact is the defect —
  a second source no gate reads, drifting silently. The fix keeps each
  surface's tier and replaces the foreign-tier slab with a pointer.
- **Definitions have one home.** A canonical bold-lead-in definition of a
  glossary term belongs only to the glossary; another surface may *use*
  the term, explain *why* it exists, or carry its local mechanism — never
  the definition (`check-surface-duplication`, with per-site valves for a
  surface that legitimately introduces a concept).
- **Quantitative literals are code-owned.** A count or enumerated set
  transcribed into prose is a parallel copy; cite the owning source
  instead (`check-manifest-count` bans a bare cardinal quantifying a
  governed collection in a manifest — the collection is the count's owner).
  A literal stays verbatim only when load-bearing, and then only
  gate-coupled.
- **Comments cite, never restate.** The code surface is a tier too: a
  full-line comment on a governed source is a machine or reason directive,
  rides the contiguous run a directive opens, or is exempt — design
  rationale lives in the owning SPEC section, not re-derived in a comment
  block; the pointer names where the why lives and couples the code to it —
  never relocating the why into the comment (`check-comment-tier`; the
  FP-prone trailing-comment judgment stays a review tripwire).
- **Honest mechanizability.** Only the structural sub-rules gate (banned
  headings, fence density, duplicate definitions, verbatim copies,
  temporal-narration markers, restated collection totals); the core
  judgment — is this sentence a definition or a narration, a why or a
  mechanism — is FP-prone and stays a review tripwire, explicitly not a
  blocking gate.

## Layout and configuration

The kit is vendored beside gate-sdk (conventionally at `canon-kit/`); its
gates are registered in the consumer's `gates.list` by name — each
where its surface exists (a consumer with no glossary does not register
`check-surface-duplication`) — and resolve through gate-sdk's multi-kit
path.

Brand tokens and generic vocabulary sit on opposite sides of a seam. Brand
tokens carry the kit's identity and move with the kit name: the kit dir
(`canon-kit/`), the `CANON_KIT_` env-knob prefix, the discovered config
filename (`canon-config.sh`, the `<kit>-config.sh` convention), and the
docs-site page dir (`docs/canon-kit/`). Generic vocabulary names the spec
*artifact* discipline rather than the brand and is fixed regardless of the
kit's name: the `SPEC.md` canonical-spec filename, the `SPEC-*.md` amendment
glob, the `spec:` and `contract:` source directives, the `[spec:]` and
`[needs-spec]` queue tags, the `check-spec-*` gate names, and the internal
`lib/spec.sh` loader (where "spec" names the discipline). The consumer gate
`check-kit-ref-liveness` (a check-skeleton copy in `scripts/` — the
dangling-reference hazard is a kit author's, so it is not templated into
gate-sdk) holds the brand-token side honest tree-wide: every slash- or
line-anchored `<name>-kit` / `gate-sdk` path segment must name a
`gate_kit_roots` dir, and every live-prefix kit knob must resolve to a tracked
kit knob (check-docs-cmd's resolver, reused so the non-uniform knob-prefix map
has one home). It valves the surfaces that legitimately name design-ahead or
frozen brands — `gate-tests/` fixture bodies, `docs/posts/*`, the generated
trajectory data, `SPEC-*.md` amendments, and the queue — so a rename cannot
leave a dangle without turning a gate red.

Config follows queue-kit's pattern: copy `templates/canon-config.sh` into
the gates dir as `canon-config.sh` (or point `CANON_KIT_CONFIG_FILE`
elsewhere) and override any knob; defaults fill what the consumer left
unset, and the loader exits 2 on a malformed config. Knobs:

- `CANON_KIT_SPEC_NAME` — canonical spec filename, default `SPEC.md`.
- `CANON_KIT_AMENDMENT_GLOB` — default `SPEC-*.md`.
- `CANON_KIT_QUEUE_FILE` — default `${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}`.
- `CANON_KIT_FEATURE_SECTIONS` — array, default `("New Features")`: active
  sections whose entries require `[spec:]`.
- `CANON_KIT_ACTIVE_SECTIONS` — array, default
  `("New Features" "Technical Debt")`: sections where `[needs-spec]` is a
  violation.
- `CANON_KIT_DEFERRED_SECTION` — default `Deferred`: the section whose
  every entry requires `[needs-spec]`.
- `CANON_KIT_DOD_HEADING` — default `Definition of Done`;
  `CANON_KIT_DOD_MODE` — `exactly-one` (the default) or `at-most-one`
  (a reference-spec corpus like this repo's kits carries no DoD).
- `CANON_KIT_SCAN_KIT_ROOTS` — `0` (default) or `1`. At `0` the shared finders
  skip vendored kit roots (`gate_kit_roots`): a kit's `SPEC.md`/`SPEC-*.md` is
  a dependency's documentation, not governed content, so the `exactly-one`
  default holds out of the box on a tree that merely vendored the kits beside
  gate-sdk. Set `1` when the kit SPECs are the consumer's own first-party
  content (this repo does, and keeps `at-most-one` for them).
- `CANON_KIT_BANNED_HEADINGS` — array, default
  `("Directory Structure" "Public API" "Cargo.toml Dependencies")`;
  `CANON_KIT_DERIVABLE_DENSITY` — default `60` (percent fenced);
  `CANON_KIT_DERIVABLE_POINTER_REGEX` — the index-pointer marker that exempts
  a shed section, default `pub-index|proto-index` (consumer index tooling).
- `CANON_KIT_EMBED_THRESHOLD` / `CANON_KIT_EMBED_MINLINES` — defaults `0.70`
  / `8`; `CANON_KIT_EMBED_LANGS` — the scanned fence-language → source
  mapping, one `kind|fence-alias,…|file-glob,…` entry per language family,
  default = the bundled list; `CANON_KIT_EMBED_ILLUSTRATIVE` — fences
  illustrative-by-default, default `(json)`; `CANON_KIT_EMBED_WIRE_KIND` — the
  one fence an amendment may embed as a not-yet-merged contract delta,
  default `proto`.
- `CANON_KIT_GLOSSARY_FILE` — default `GLOSSARY.md`;
  `CANON_KIT_DUP_SURFACES` — array of surfaces scanned for foreign
  definitions, default `(VISION.md)` plus every component spec.
- `CANON_KIT_MANIFEST_FILES` — array of globs, default empty ⇒ derive the
  manifest set (canonical specs, `README.md` at any depth, `CLAUDE.md`);
  `CANON_KIT_TEMPORAL_MARKERS` — the temporal-narration marker set scanned by
  `check-manifest-temporal`, default a generic-English list (`previously`,
  `formerly`, `renamed from`, …), matched case-insensitively;
  `CANON_KIT_TEMPORAL_EXEMPT_SECTIONS` — array of heading names whose whole
  section is exempt, default empty (this repo sets `Out of scope`).
- `CANON_KIT_MDREF_EXCLUDE` — array of globs, default empty: manifest-set docs
  `check-md-refs` skips (a consumer's generated documentation whose links a
  build tool owns).
- `CANON_KIT_COUNT_COLLECTIONS` — array of collection-noun plurals
  `check-manifest-count` treats as growing governed sets, default
  `("gates" "meta-gates" "checks" "kits" "stages" "rules" "KPIs")` (a consumer
  appends its own governed plurals); `CANON_KIT_COUNT_WEDGE_WORDS` — how many
  words may sit between the cardinal and the noun, default `2`;
  `CANON_KIT_COUNT_ALLOWED_PHRASES` — exact-phrase allowlist for fixed named
  sets a doc may cite inline, default empty (a consumer names its own fixed
  sets, and only a phrase whose noun it governs needs the valve).
- `CANON_KIT_ENUM_SETS_CMD` — a consumer command emitting the governed sets
  `check-prose-enum` holds, one `<set-name>`⇥`<member>` line per member,
  default empty ⇒ clean skip (no declared sets). This repo sets
  `bash scripts/enum-sets.sh`, which derives the queue tag sets from
  queue-kit's own parser rather than restating them.
- `CANON_KIT_COMMENT_MACHINE` / `CANON_KIT_COMMENT_REASON` — arrays, default
  empty: extra directive prefixes appended to the built-in kit-mechanism
  roster (a consumer's product vocabulary). `CANON_KIT_COMMENT_SURFACE` —
  array of globs, default empty ⇒ derive: shell sources under the root (kit
  roots per `CANON_KIT_SCAN_KIT_ROOTS`, `templates/` pruned) plus the
  `${GATE_SDK_WORKFLOW_DIR:-.workflow}/*.txt` state files.
  `CANON_KIT_COMMENT_POSITIONAL` — the language construct roster for
  positional rescue, default empty (the kit is language-agnostic; a Rust
  consumer sets `.unwrap( .expect( unsafe #[allow(`).
  `CANON_KIT_COMMENT_WHITELIST` — array of globs, default empty: the
  consumer's not-yet-swept sources, its array tagged `# exception-list:`
  with a per-entry `# until: <drain-task>`. `CANON_KIT_COMMENT_RUN_CAP` —
  positive integer, default 3: total physical comment lines a directive
  blesses (its own line plus continuations, blank `#` lines counted).

Cross-kit note: the section knobs carry the same defaults as queue-kit's;
the knobs are independent (either kit runs without the other), so a
consumer renaming its sections sets both. Valve and marker spellings <!-- prose-enum-exempt: names the two amendment-lifecycle tags specifically; [blocked-by:] is a dependency tag outside that lifecycle, not a dropped task-tag member -->
(`[needs-spec]`, `[spec:]`, `vision-introduces` / `spec-introduces`,
`spec-embedded-source-exempt: <reason>`) are mechanism, not config.

## Per-component contracts

### lib/spec.sh

The sourced config loader plus shared adapters: section-regex builders for
the queue-facing gates (queue-kit's rule — both sides of a section boundary
must parse identically) plus the queue-resolution pass `spec_queue_slugs` built
on them — one queue walk emitting a live slug for a bold lead-in bullet in an
active or deferred section and a done slug for a bare-slug bullet outside them,
so `check-todo-task-liveness` and `check-deprecation-task` resolve a
`task: <slug>` binding through one grammar (each caller builds its own live/done
map and fail-closes on the awk status), the canonical-spec/amendment finders the
spec-scanning gates share, the manifest-set finder (`spec_manifest_files`) the
narration-gate family shares — canonical specs plus `README.md`/`CLAUDE.md`,
amendments excluded — so its members read one identical set, and the governed
comment-surface adapters (`spec_comment_whitelisted` plus two finders that
`check-comment-tier` and `check-spec-pointer` scan through), and the
count adapter that the restated-total gates share, so a consumer's
`CANON_KIT_COUNT_COLLECTIONS` vocabulary enters once and every such gate matches
the same shapes (§check-manifest-count). The adapter has two halves: the shell
regex builders (`SPEC_COUNT_CARDINAL_RE` plus `spec_count_noun_alt`,
`spec_count_quantifier_re`, `spec_count_range_re`, and the lowercased
`spec_count_phraselist`), and `spec_count_awk_lib` — an awk source fragment a
gate prepends to its own program, exposing `sk_count_hit(text)`, which returns
the offending span or the empty string. The boundary rule (no match glued to an
adjacent word) and the mechanical exemptions (comparator, `all but`, partitive,
`per`-phrase, inline code, allowed phrases) live in that fragment alone, so no
sibling can drift from another in what it considers a total; each supplies its
own surface walk (fences and per-site markers for the manifest gate, the comment
classifier for its sibling).

`sk_count_hit` judges one physical line, so a total whose cardinal and noun
straddle a prose wrap evades it — a blind spot both siblings inherited from the
shared matcher, and one this repo's own §lib/spec.sh prose sat in. Two further
awk adapters close it and factor the walk the manifest-prose gates share.
`spec_para_accum_awk` is the **paragraph accumulator**: a caller feeds a logical
paragraph's lines in order (`sk_para_reset`, `sk_para_add(fnr, text)`), and
`_sk_join` reads a window back. On it the count fragment builds
`sk_para_wrapped()`, which reports the first total whose span crosses a line
boundary via `SK_WRAP_FNR` / `SK_WRAP_SPAN`, at the span's *first* physical line
— where the cardinal sits, and where the operator edits — and returns false for
a span already on one line, so the caller's per-line scan stays the sole
reporter of unwrapped totals and no hit is counted twice. A joined span carries
its prefix with it, so the exemptions hold across the wrap unchanged: a
comparator before the break still reads as a bound.

`spec_manifest_walk_awk` is the **manifest-prose walk driver** both
restated-manifest gates (check-manifest-count, check-prose-enum) prepend: it
tracks fences, resets the paragraph on a blank line, and skips a per-site exempt
window (the line or the one above; the marker regex arrives in `SK_EXEMPT`),
calling the caller's `sk_on_line(file, fnr, raw)` on each prose line and
`sk_on_pflush()` at each boundary, over the shared accumulator. One walk, one
exemption behavior for the manifest-prose surface — the count gate's
`sk_on_line` reports the per-line total and its `sk_on_pflush` runs the wrap
detector; the enum gate reads the whole flushed paragraph. `check-comment-tier`
keeps its **caller-owned comment walk**, driving the same accumulator by hand
because its surface disagrees — a comment block's end or an exempt line ends its
paragraph, not a blank line or fence. The enum gate's declared sets arrive
through `spec_enum_sets`, which runs the consumer's `CANON_KIT_ENUM_SETS_CMD` and
echoes its validated `<set>`⇥`<member>` lines, fail-closing (exit 2) on a
command error or an unparsable line (§check-prose-enum). The comment
gates read *different* surfaces: `spec_comment_surface` prunes `templates/`
shell sources, and `spec_comment_surface_with_templates` keeps them.
`check-spec-pointer` scans the pruned set — a template's `spec:` line is a
placeholder unresolvable by design (§check-spec-pointer). `check-comment-tier`
scans the with-templates set — a copied-out template's `spec:` pointer resolves
against the vendored kit path (kit SPECs travel with the vendor-whole install),
so its comments are governed like any source (§check-comment-tier). Which
finder a gate uses is kit contract, not consumer config — a consumer wanting
the old blanket exemption shadows the gate. The canonical-spec/amendment
finders skip a `templates/` skeleton (a copyable stub, not governed content —
the same rationale as the gate-tests prune) and, unless
`CANON_KIT_SCAN_KIT_ROOTS=1`, any vendored kit root under
the scan root (a dependency's docs; an ancestor kit root — the case when a
kit's own gate fixture dir is the scan root — never prunes). Values and
adapters only, never gate structure.

### check-amendment-queue

Invariant: the bidirectional rule holds — (a) no feature-section entry
without `[spec:]` and no `[needs-spec]` anywhere in the active sections
(entries or prose; a prose mention masks the tag's absence); (b) every
deferred entry carries `[needs-spec]`, and a deferred entry already
carrying `[spec:]` must be promoted; (c) every amendment on disk pairs
with a `[spec:]` queue entry and every `[spec:]` ref resolves to a file.

Calibration: a ref is a bare amendment basename (searched tree-wide) or a
repo-relative path (resolved directly — the generalization that lets a
consumer point a task at any design artifact, e.g. this repo's kit-SPEC
drafts). Sub-bullets and prose notes are outside the entry grammar;
`precommit` tier.

### check-spec-dod-singleton

Invariant: no canonical spec carries the configured Definition-of-Done
heading more than once (a duplicate checklist is the two-sources defect on
the completion contract); under `exactly-one` mode a spec with none is
also flagged.

Calibration: heading match is level-insensitive within `##`–`####`;
`at-most-one` exists because a reference-spec corpus legitimately has no
DoD. The good/bad pair covers the DoD count; `check-spec-dod-singleton.test.sh`
covers the finder's kit-root scoping — a DoD-less vendored kit `SPEC.md` is
pruned by default (so `exactly-one` holds on a vendored tree) and re-included
by `CANON_KIT_SCAN_KIT_ROOTS=1`. `align-only` tier.

### check-spec-derivable-section

Invariant: no canonical-spec section under a banned heading (the
configured code-derivable set) whose body exceeds the density budget in
fenced lines — such a section is a code dump that drifts; it sheds to a
one-line index pointer (which is exempt).

Calibration: density counts non-blank lines; the heading set and budget
are config because what is derivable depends on the consumer's index
tooling. `align-only` tier.

### check-spec-embedded-source

Invariant: no fenced block in a canonical spec verbatim-copies a tracked
source file above the overlap threshold — cite the path instead. Overlap
detection needs no author opt-in; the amendment wire-delta exemption and
the per-site `spec-embedded-source-exempt: <reason>` marker are the two
valves.

Calibration: blocks shorter than `CANON_KIT_EMBED_MINLINES` are ignored;
languages in `CANON_KIT_EMBED_ILLUSTRATIVE` are skipped by default; the
threshold is calibrated against real specs, not synthetic fixtures.
`precommit` tier.

### check-manifest-temporal

Invariant: no temporal-narration marker in governed manifest prose outside an
exempt site. A manifest states current behavior only — history is derivable
from git, and a `formerly…` line is standing context cost documenting the old
cost, taxing every session that reads it. This gate mechanizes the lexical
share of that judgment; context-kit's close-stage brevity pass keeps the
semantic residue (*is this sentence about the past?*); a by-eye
narration-marker KPI is superseded by this gate (drift-kit/SPEC.md
§Out of scope).

The scanned set is the shared `spec_manifest_files` finder (§lib/spec.sh):
canonical specs, `README.md` at any depth, and `CLAUDE.md`; amendments are
excluded by construction (a transition artifact describes change — that is its
nature). Markers are `CANON_KIT_TEMPORAL_MARKERS`, matched case-insensitively;
fenced code blocks are skipped and a marker inside an inline-code span is a
meta-reference, not narration — so a gate-output example or this section's own
vocabulary may name one. Three valves suppress a legitimately past line: a
per-site `manifest-temporal-exempt: <reason>` comment on the line or the one
above; `CANON_KIT_TEMPORAL_EXEMPT_SECTIONS` — heading names whose whole
section (subsections included) is exempt, this repo's config naming
`Out of scope` (a deliberate-absence ruling may narrate what the kit
excludes); and `CANON_KIT_TEMPORAL_EXEMPT_PATHS` — path
globs whose whole file is exempt, for an immutable dated-narrative surface a
heading name cannot address (this repo's config naming `docs/posts/*`, the
dated announcement posts, which take link and command resolution but not
narration governance). Producer: the generated pre-commit hook /
`run-gates.sh`; consumer: the committing operator via the output contract; each
marker hit is read at the single scan transition (file, line, marker in the
message), no persistent state. Fail-closed on an unreadable manifest.

Calibration: the marker set is tuned against this repo as the FP corpus — bare
`used to` is excluded (it collides with instrumental "used to build/filter").
At build every hit is dispositioned: reword (preferred — narration is standing
cost), section-exempt (provenance), or site-exempt with reason. `precommit`
tier.

### check-manifest-count

Invariant: no bare cardinal quantifying a governed collection noun
in manifest prose outside an exempt site. A pinned total for a *growing*
collection — `six gates`, `seven meta-gates` — is a second source no gate reads:
the count's owner is the collection itself (`gates.list`, a `checks/` dir, the
stages config), and a restated total drifts the moment the collection grows.
Ban, don't validate — a validating gate carries
the standing token cost context-kit's brevity machinery rejects plus FP-prone
entity mapping; a lexical tripwire eliminates the copy. The motivating find: this
repo landed one gate and left the same total in four disagreeing copies (across
two READMEs and a SPEC), caught only by close-stage review.

The scanned set is the shared `spec_manifest_files` finder (§lib/spec.sh) —
canonical specs, `README.md`, `CLAUDE.md`; amendments excluded, fenced blocks
skipped, an inline-code cardinal a meta-reference (so this section may name its
own examples). Both the grammar and the matcher come from the shared count
adapter, and the prose walk itself is the shared `spec_manifest_walk_awk`
driver (§lib/spec.sh) — this gate supplies only two hooks: `sk_on_line` runs
the matcher per line, `sk_on_pflush` runs the paragraph-join window over the
accumulator. So it, its `check-prose-enum` sibling, and its comment-tier cousin
read one vocabulary and one exemption behavior, and a total wrapped across a
prose break is caught and reported at its first physical line; a blank line, a
fence, and a `manifest-count-exempt:` site each end the paragraph — an exempted
line cannot join its neighbours into a total. The cardinal
grammar is digit sequences and the spelled
`two`…`twelve`, case-insensitive; `one` is deliberately outside it — singleton and
cardinality-rule idioms ("one owner per fact", "one iteration per kit") are
invariants, not totals. Collection nouns are `CANON_KIT_COUNT_COLLECTIONS`
(default the plurals the kits themselves grow: `gates`, `meta-gates`, `checks`,
`kits`, `stages`, `rules`, `KPIs`) — the one place consumer vocabulary enters,
and it enters as config.

Two match shapes carry the cardinal to the noun. The *quantifier* shape allows
up to `CANON_KIT_COUNT_WEDGE_WORDS` (default `2`) modifiers wedged between them,
so `nine generic rules` pins a total as surely as the adjacent `six gates` —
adjacency is the zero-wedge case of one regex, not a branch of its own. The
*noun-then-range* shape (`rules 1-8`, `gates 1-42`) pins both endpoints of an
ordered collection and rots on every append. Five exempt contexts, mechanical
first: a threshold/comparator
on the same line (`≥ ≤ > <`, `at least` / `at most` / `up to` / `more than` /
`fewer than`, or a following `per`-phrase) — a bound is a rule, not a total; the
partition idiom `all but <cardinal>`; a partitive marker (`of`, `out of`) on
either side of the match — in `three of the twelve gates` neither cardinal is a
restated total, so a wedge containing the marker and the denominator it
introduces are both exempt, and the markers are fixed generic-English mechanism
rather than config; `CANON_KIT_COUNT_ALLOWED_PHRASES`, an
exact-phrase allowlist for fixed named sets a doc legitimately enumerates (default
empty — fixed-set naming is consumer judgment, config not mechanism, and the
valve only bites on a phrase whose noun the consumer governs);
and the per-site `manifest-count-exempt: <reason>` on the line or the one above.
Producer: the generated pre-commit hook / `run-gates.sh`; consumer: the
committing operator via the output contract; each hit read at the single scan
transition (file, line, matched span in the message), no persistent state.
Fail-closed on an unreadable manifest.

Calibration shares the sibling's FP corpus and procedure
(§check-manifest-temporal): the default noun list is tuned against this tree,
every hit dispositioned — reword to cite the owning collection (preferred),
extend `CANON_KIT_COUNT_ALLOWED_PHRASES` (a genuinely fixed set), or site-exempt
with reason. The good/bad pair covers every match shape and the mechanical
exemptions;
`check-manifest-count.test.sh` covers the config-driven paths (a consumer-governed
noun and the allowlist containment) the stock defaults cannot reach. `precommit`
tier.

### check-prose-enum

Invariant: within one manifest-prose paragraph, a delimited hand list naming two
or more members of one declared governed set must name every member of that set,
unless an exempt site holds; a member absent from the paragraph is reported as
omitted. This is the enumeration sibling of check-manifest-count's restated
totals and the prose analog of check-kit-enum's hand-list doctrine: the count
gate catches a bare cardinal over a growing collection, but a row that
*enumerates instead of counting* drifts just as silently when the set grows.
The attested drift: a README row's `blocked-by/needs-spec/spec` tag algebra went
incomplete when a sibling tag landed — same restatement, different surface form,
no scanner. Engagement mirrors check-kit-enum — two members are a list, a lone
member (or two scattered across a paragraph, not adjacent) is a mention, not a
list.

Set declarations are consumer config, never gate literals (the provenance seam):
`CANON_KIT_ENUM_SETS_CMD` (default empty — clean skip) names a consumer command
emitting one `<set-name>`⇥`<member>` line per member, loaded through
`spec_enum_sets` (§lib/spec.sh); a command that fails or a line that does not
parse is fail-closed (exit 2). A member matches word-bounded — bracketed
(`[spec:]`) or bare (`spec`), neither an alphanumeric nor a hyphen abutting, so a
stem never matches inside a longer tag — and the attested drift used bare stems.

The scanned set and the paragraph walk are the shared `spec_manifest_files`
finder and `spec_manifest_walk_awk` driver (§lib/spec.sh); this gate's
`sk_on_pflush` hook judges each flushed paragraph. Two or more members present
count as a *hand list* only when a run of them is chained by list separators
(comma, slash, brackets, whitespace, or `and`/`or`); members merely co-occurring
in one paragraph with prose between them stay mentions. Omitted members are set
members absent from the whole paragraph — the sibling that landed but never
joined the list. Exempt contexts follow the count gate's family shape,
mechanical markers first, per-site tag last: a subset marker in the window
(`e.g.`, `such as`, `among them` — the list declares itself partial), a
partitive marker on the match (`some of`, `one of`, `of the set` — a selection,
not an enumeration), and the per-site `prose-enum-exempt: <reason>` on the line
or the one above. The corrective names both legitimate fixes: cite the owning
set by name, or complete the enumeration — never trim to a silent subset.

Producer: the generated pre-commit hook / `run-gates.sh`, coupled to the
manifest set plus the consumer's config and kit sources (a set change re-fires
the gate over the docs). Consumer: the committing operator via the output
contract — file, line, set name, count, and the omitted members all read once at
the scan transition. `CANON_KIT_ENUM_SETS_CMD` is read at startup; both fields of
each emitted line are read at match time (set name in the report, member in the
matcher). This repo's consumer config is `scripts/enum-sets.sh`, which derives
the queue task-tag set and the Lessons-channel set from queue-kit's own
lead-line tag parser plus `QUEUE_KIT_LESSON_TAGS` — derived, not restated; the
two roles are separate sets because a paragraph naming one role's tags is not
enumerating the other. A set a consumer can only hand-list is that consumer's
own drift to own; the kit contract asks only for the emit grammar.

Calibration follows the count gate's procedure: tuned against this tree, every
hit dispositioned — cite the set, complete the list, or site-exempt with reason.
The good/bad pair covers a bare comma hand list dropping a member and the
marked-subset cases; `check-prose-enum.test.sh` covers the config-driven paths
(the empty-default skip, the fail-closed escapes, bracketed matching, multi-set
independence, the exempt escapes) the pair cannot reach. `precommit` tier.

### check-knob-citation

Invariant: no kit knob stated *with its value* in governed manifest prose
outside the owning kit's SPEC. This is the enforcement half of the
de-literalization rule (doctrine-kit/DOCTRINE.md §Methodology-maintenance
rules): a knob default copied into a README or a sibling SPEC is a second source
no gate reads, and it drifts the moment the owner changes. Where the count and
enum siblings ban a restated *total* and a restated *enumeration*, this bans a
restated *knob value*.

The knob-token vocabulary is derived, never listed — the provenance seam holds
(a kit literal carrying a private vocabulary would publish it). Each
`gate_kit_roots` member yields two SCREAMING_SNAKE prefix forms: the dir name
uppercased with hyphens mapped to underscores (`gate-sdk` → `GATE_SDK_`,
`delegation-kit` → `DELEGATION_KIT_`), and for a `-kit`-suffixed dir the
suffix-dropped short form too (`lifecycle-kit` → `LIFECYCLE_`), so a roster
carrying the short prefix and its lone long-form knob are both governed. Both
forms derive mechanically, so adding a kit widens the vocabulary with no edit
here and the gate ships no term list. A SCREAMING_SNAKE token is a candidate
only when it starts with a derived prefix; the short form widens the token
space, and the value-marker leg below is what holds the false-positive rate.

A candidate fires on the value-marker triad, every leg required: a
derived-prefix token; a same-line value marker; and a surface that is not the
token's owning SPEC (`<owning-kit>/` joined to `CANON_KIT_SPEC_NAME`). The
marker is one of two shapes — `=` appended directly to the token
(`<KIT>_<KNOB>=<value>`), or the word "default" bound within a short window to a
value literal (a backticked value, a quoted string, or a number). Bare knob
names stay legal everywhere: prose cites the name and points at the owning
roster, the fixed instance the gate must never flag; a bare number with no knob
token in reach never fires — a tripwire left to human judgment, not a gate rule.
Code and config are out of scope: code owns values, and only the prose manifest
is scanned.

Two calibrations hold the false-positive rate against this tree, the corpus the
leg is tuned on. A token named inside a `${…}` shell parameter expansion is a
name citation — another knob's default *expression*, a fallback source — never a
value statement of itself, so expansions are blanked before the token scan. And
the "default" leg binds: the word must be followed within a short window by a
literal that is not itself a backticked knob name, so a stated default like
`default \`docs/CNAME\`` fires while a knob cited bare after the verb ("these
paths default through `<KIT>_<KNOB>`") does not.

The scanned set is the shared `spec_manifest_files` finder and the prose walk is
the shared `spec_manifest_walk_awk` driver (§lib/spec.sh); this gate supplies the
`sk_on_line` hook (the per-line triad) and leaves `sk_on_pflush` unused — the
marker is same-line, so no paragraph join is needed. Fences are skipped, and a
`knob-citation-exempt: <reason>` on the line or the one above is the per-site
valve for a genuine local restatement (a wiring example whose value is the
citing kit's own path). Producer: the generated pre-commit hook / `run-gates.sh`,
coupled to the manifest set and the kit SPECs; consumer: the committing operator
via the output contract — file, line, the knob token, and the owning SPEC the
value belongs in, each read once at the scan transition, no persistent state.
Fail-closed on the awk status.

Calibration follows the family procedure (§check-manifest-temporal): every hit
dispositioned — cite the knob by name and point at its SPEC roster (preferred),
or exempt a genuine local restatement with reason. The good/bad pair covers both
marker shapes, the short-derived prefix, the `${…}` name citation, the
default-as-verb non-hit, and the per-site valve; `check-knob-citation.test.sh`
covers the owning-SPEC exemption (a kit may state its own knob's value in its own
SPEC but not in its README) the fixture pair cannot reach. `precommit` tier.

### check-surface-duplication

Invariant: no unvalved bold-lead-in definition of a glossary term on a
configured non-glossary surface — the canonical definition form belongs
only to the glossary; elsewhere it is a second definition that drifts.
Valves `vision-introduces:` / `spec-introduces:` (same line or the line
above) make a legitimate narrative or local introduction a reviewable
decision in the diff.

Calibration: a bold lead-in naming a component (a directory owning a
canonical spec) is never flagged — a component's definition lives in its
own spec, so there is nothing to restate; gate-test fixtures are excluded;
exits 2 when the glossary file is absent (register the gate only where the
topology exists). `align-only` tier.

### check-comment-tier

Invariant: every full-line comment on a governed source is one of — a
machine directive (a comment a tool parses: `graph:`, `shellcheck`,
`contract:`), a reason directive (a spec pointer, usage synopsis, or
positional justification: `spec:`, `usage:`, `exception-list:`,
`no-fixture:`, `assertion`, `permanent:`, `TODO(task:`,
`TODO(spec-ambiguity)`, which blesses a bounded window — its own line plus
continuation lines up to `CANON_KIT_COMMENT_RUN_CAP` total physical comment
lines, blank `#` lines counted; a directive mid-run opens a fresh window
from its own line, and every comment line beyond a window classifies on its
own), `comment-tier-exempt: <reason>`, or the
comment immediately above a positional construct from the language roster.
Anything else is flagged. Code is the WHAT, its SPEC the WHY — the seam the
align stage checks each side against; a comment that restates the code, or
paraphrases the SPEC section it cites, is deleted, not blessed. A `spec:`
directive earns its place by the coupling it makes — a bare pointer binding
this code site to the requirement that governs it, as `contract:` / `assertion`
/ `graph:` bind a site to a manifest contract, an enumerated assertion, or the
gate graph. The binding is the value: either side's drift breaks it visibly
(gate-checked both sides for `graph:` and `assertion`; for `spec:` /
`contract:` `check-spec-pointer` gate-checks the forward side — the target
resolves — leaving only the reverse, an uncovered requirement, a review
concern), which is why it blesses only its own one-line binding, never a
relocated block — restatement couples nothing, it just forks the why into a
second copy no gate reads. `comment-tier-exempt:
<reason>` is the honest directive for a genuinely-local fact below SPEC altitude
that neither tier owns.

One shape overrides the window: a full-line comment carrying a **restated
collection total** — the count grammar of §check-manifest-count, over the same
`CANON_KIT_COUNT_COLLECTIONS` vocabulary — is flagged even where a directive
blesses it, and positional rescue does not reach it either. A count is never
directive wording. A directive's blessing covers its own wording physically
wrapped, never a total pinned beside it, and such a total is exactly the second
source the manifest gate bans: `# rules 1-8` sat stale in this repo's own guard
while its ruleset grew past eight, invisible because no gate read comments for
counts. The fix is deleting the count or citing the owning collection; the sole
valve is `comment-tier-exempt: <reason>`, whose window suppresses the override
as it suppresses the tier rule. The override reads a comment block as a
paragraph, not a line at a time (the adapter's join window, §lib/spec.sh): a
directive's own wrapped wording is exactly where a total hides from a per-line
scan, and an exempt line ends the join rather than laundering its neighbours.
The weighed alternative — a source-coupled
numeral scan with an allowlist — is rejected: legitimate numerals abound in
source (exit codes, indices, field positions) and the false-positive rate would
exceed the catch. Counts in *code* stay a review concern; counts in *prose*,
wherever the prose lives, are gated.

Calibration: the built-in roster is Checkwright's own kit-mechanism
directive names; `CANON_KIT_COMMENT_MACHINE` / `_REASON` append a consumer's
product vocabulary (the same split as `check-graph`'s vocab — the mechanism
ships, the rule content is config). The window cap is
`CANON_KIT_COMMENT_RUN_CAP` (default 3 — one wrapped sentence): a
within-window continuation is the directive's own wording physically
wrapped, blessed as such, while prose beyond the window is presumed
relocated restatement and deleted. `comment-tier-exempt:` is reserved for a
genuinely-local fact neither tier owns, and exempting a restatement rather
than deleting it is itself the defect — a long roster (a `usage:` option
list, a header) restructures into directive-anchored short paragraphs or
trims, never launders prose past the cap. The count-shape override reads the
shared count adapter (§lib/spec.sh) rather than a second grammar, so it
inherits that gate's carve-outs unchanged: a comparator bound or a `per`-phrase
in a directive is a rule and not a total, a partitive proportion exempts both
its cardinals, and a cardinal in inline code is a meta-reference — which is why
a directive may still say `at most three checks per run` or name a
`` `six gates` `` example. No knob is added: the noun vocabulary enters through
`CANON_KIT_COUNT_COLLECTIONS` and the wedge window through
`CANON_KIT_COUNT_WEDGE_WORDS`. The default surface is shell
(`#`) — `templates/` stubs included: a copied-out template's `spec:` line
resolves against the vendored kit path (kit SPECs travel with the
vendor-whole install), so its comments are directives like any source and
this gate governs them (`spec_comment_surface_with_templates`), where
`check-spec-pointer` exempts them as placeholders-by-design — with the
`.workflow/*.txt` state files blessing only `contract:`/`see`;
slash-comment parsing (`//`, `/* */`, doc-comments, heredoc skipping) ships
as mechanism and activates when a consumer widens `CANON_KIT_COMMENT_SURFACE`
to a language that needs it. Positional rescue is language-agnostic — its
construct roster (`CANON_KIT_COMMENT_POSITIONAL`) is consumer config, empty
by default so inert on the shell surface (a Rust consumer supplies the
`unwrap`/`allow`-class tokens; a consumer's language is not a kit
literal). Trailing inline comments are out of scope by construction — the
FP-prone half of the judgment stays a review tripwire. Not-yet-swept
sources ride `CANON_KIT_COMMENT_WHITELIST` (its array tagged
`# exception-list:`, each `# until:` a live drain task per
`check-gate-exemption-tasks`), draining kit by kit. `precommit` tier.

### check-spec-pointer

Invariant, two passes, forward direction only: every `spec:` / `contract:`
pointer **directive** on a governed source resolves, and every free-prose
`<path>.md §<heading>` **citation** on a governed manifest resolves. The
directive set is exactly what `check-comment-tier` blesses by shape: full-line
`spec:` / `contract:` comments on the governed sources, plus the
`.workflow/*.txt` `# contract:` headers. A directive's target grammar is
`<path> [§<heading>]`: `<path>` (repo-relative) must be a tracked file, and when
a `§<heading>` fragment is present the file must carry a matching markdown
heading; a pointer without `§` resolves file-only. Reddens on a missing or
untracked target file, on a named heading the target lacks, and — fail-closed —
on a directive that matched the pointer shape but carries no target path.

The prose-citation pass closes the gap where a citation woven into a sentence
(`context-kit/SPEC.md §The memory-off doctrine`) resolved neither file nor
heading while the structured directive and the markdown-link anchor were both
verdict-checked. It scans the manifest set (`spec_manifest_files`, the same
surface the manifest-narration gate family reads) over a blank-line-delimited
paragraph join so a heading that wraps a line reassembles, and resolves the
heading through the one shared `heading_present` helper — one heading-resolution
path, two callers. Free prose runs on past the heading with no delimiter, so
the citation pass matches a heading as a boundary-anchored **prefix** of the
fragment where the directive pass matches it whole. A cited path that is not a
tracked file is out of this pass's scope — path liveness stays with the gates
that own it (`check-md-refs`, `check-kit-ref-liveness`); ruling only on headings
of resolvable files is what holds the false-positive rate at the directive
pass's level.

`check-comment-tier` owns the directive's *shape*; this gate adds *resolution*
on top, the binding a `spec:` pointer makes being only as good as its liveness
— a renamed or deleted heading leaves every inbound pointer or citation
dangling, otherwise caught only on review. Heading match tolerates a trailing
`(qualifier)` on either side: a pointer narrowing a section to a labelled point
(`§check-graph (assertion G)` → the `check-graph` heading) or a heading carrying
a locator the pointer omits (`§The guard framework` → `## The guard framework
(lib/guard.sh)`). Two `§` carve-outs keep the false-positive floor and must not
be conflated: a directive's em-dash *prose tail* (`spec: <path> §<h> — <gloss>`)
strips at the ` — ` so a `§` in the gloss is not read as a heading, while a
free-prose citation *is* a heading marker — the citation pass fires on a
tracked `.md` path immediately followed by `§`, and a bare `§` with no tracked
path before it (the deliberate non-citation use) never fires. Fenced code
blocks are skipped in both passes — a quoted example is not a citation.

Calibration: forward direction only. The reverse — flagging a requirement with
no inbound pointer as uncovered code — needs a "what counts as a requirement"
notion that risks false positives against the cheap-and-FP-free bar, so it is
ruled out (a separate task with its own ruling if ever wanted; this gate
reserves no syntax for it). Configuration is shared with `check-comment-tier` —
the `spec_comment_whitelisted` adapter in `lib/spec.sh` and the same
`CANON_KIT_COMMENT_*` knobs, so no new config knob — but this gate scans
`spec_comment_surface`, which prunes `templates/` shell sources where
`check-comment-tier` keeps them. The split is by design, not a "SPECs don't
travel" claim: a template's `spec: <your SPEC> §check-<area>` line is a
placeholder the consumer fills in, unresolvable *by design* in the kit's own
tree — resolving it there would check the consumer's homework against a stub.
The tier gate still governs those template comments by *shape* (they must be
directives), so each gate draws the `templates/` line where its own semantics
put it. `precommit` tier.

Retention ruling: the standing doubt — forward-only checking plus the
basename↔§heading convention make the pointer largely redundant, its gloss a
restatement risk — is answered and the roster slot kept. The convention
derives a section's *name*, never its liveness: absent an inbound pointer,
renaming or deleting a SPEC heading reddens nothing, and the pointer is the
only mechanized code→prose coupling (the reverse direction stays a review
concern per the paragraph above). Sites away from a file's header bind
sections no naming convention can derive. Narrowing the directive to a bare
pointer buys nothing: the gloss is already capped at the one-line binding
(§check-comment-tier), and a gloss restating the cited section is deleted
under that doctrine, not re-gated here. Dropping the slot would also orphan
the citation coverage the convention carries — the reason a dedicated
script↔doc citation gate stays unbuilt.

### check-todo-task-liveness

Invariant: every `TODO(task: <slug>)` marker on a governed source resolves to a
live queue task. A `<slug>` naming an active or deferred task resolves; a slug
sitting in `Done` is **stale** (the work finished, the marker did not); a slug
absent from the queue is **unresolved** (a typo or an unfiled task). Stale and
unresolved both redden — a marker referencing nothing is a dangling forward
reference, the source-side twin of a `blocked-by` tag left pointing at a
completed task.

`check-comment-tier` owns the marker's *shape* — it blesses `TODO(task:` as a
reason directive — and this gate adds *resolution* on top, exactly as
`check-spec-pointer` adds resolution to the `spec:` shape the same tier gate
blesses. It closes the liveness gap those siblings already guard elsewhere:
`check-task-names` flags a `blocked-by` gone stale on a done slug and
`check-gate-exemption-tasks` resolves an exemption's `# until:` slug against the
live set, but a `TODO(task:)` bound to a cleared task passed forever because
nothing read the source side. The marker requires a resolvable slug after the
colon, so a tool carrying the bare roster literal (`check-comment-tier`'s own
directive name) never self-matches, and full-line versus trailing placement is
immaterial: resolution governs the referent, not the comment tier. Bare
`TODO`/`FIXME`/`HACK` markers are out of scope — trailing-comment scanning is a
separate ruling if the need attests.

Placement: the marker is a comment directive on the governed comment surface, so
it is canon-kit's, not queue-kit's (which disclaims source-file conventions in
its Out of scope). The gate scans `spec_comment_surface`, pruning `templates/`
as placeholders-by-design like `check-spec-pointer`, and reads the queue through
`CANON_KIT_QUEUE_FILE` with no new knob — the live/done split is the shared
`spec_queue_slugs` adapter (§lib/spec.sh), one queue walk reading a bare-slug
bullet outside the active and deferred sections as the queue's done shape. That
`task: <slug>` binding grammar — a `task:` key naming a slug that must resolve
to a live queue entry — is one grammar both liveness gates share:
`check-deprecation-task` is the twin, resolving the same binding on a
deprecation marker through the same adapter (§check-deprecation-task). Latent at landing: no such marker exists in
the tree yet — the gate ships before the first one, so a future `TODO(task:)`
cannot outlive its task silently. A queue-read failure is fail-closed (exit 2).
`precommit` tier.

### check-deprecation-task

Invariant: every deprecation marker on a governed source binds a `task: <slug>`
that resolves to a live queue task. The marker vocabulary is consumer config —
`CANON_KIT_DEPRECATION_MARKERS`, a roster of regexes (one per element) matched
against the governed comment surface. A language's marker spelling is never a
kit literal: the roster ships empty, so a repo that sets none is clean-skipped
(the `check-graph`/`graph-vocab` seam — the kit ships the resolution mechanism,
the consumer names its own `#[deprecated]`, `@deprecated`, or `@Deprecated`).
The scan itself stays consumer toolchain — a clippy/ESLint-class linter already
inventories deprecation markers; this gate adds the governance coupling no
linter ships: a deprecated surface that names no decommission task, or names a
done or absent one, has nothing tracking its removal.

A marker line carrying no `task: <slug>` is **unbound**; a bound slug sitting in
`Done` is **stale** (the decommission finished, the marker did not); a bound
slug absent from the queue is **unresolved**. All three redden — each finding
names the file, the line, the matched marker, and the offending slug. The
binding grammar and the queue-resolution pass are `check-todo-task-liveness`'s:
the same `task: <slug>` key over the same `spec_queue_slugs` adapter
(§check-todo-task-liveness, §lib/spec.sh), one grammar both liveness gates
share. The gate scans `spec_comment_surface` (templates pruned as
placeholders-by-design) and reads the queue through `CANON_KIT_QUEUE_FILE`, no
new knob beyond the roster. An unreadable queue or source is fail-closed
(exit 2); an empty roster is the clean skip, not an error.

This repo sets no roster, so the gate clean-skips here — the good/bad fixture
pair and `check-deprecation-task.test.sh` carry the resolved and reddened paths
under a fixture-local roster (the `check-manifest-count` config-path precedent).
The release-boundary disposition walk over the standing marker inventory
(decommission now, re-justify and carry the task forward, or un-deprecate) is
lifecycle-kit's `release-sweep` skill template, and the between-major backlog
trend over the same roster is drift-kit's `kpi-deprecated-surface` example
(lifecycle-kit SPEC §templates, drift-kit SPEC §Out of scope). `precommit` tier.

### check-spec-fence-balance

Invariant: every governed markdown file carries an even count of code-fence
delimiters (lines opening with ```` ``` ````). The fence-skipping parsers across
the kits — `check-spec-embedded-source`, `check-tag-lead-line`, the queue
scanners — all toggle a fence flag line by line; an odd count leaves the flag
stuck and the rest of the file is read *inside* a phantom fence, so every later
finding silently fails open. This gate turns that silent hole into a red. The
surface is the manifest set (`spec_manifest_files`) plus the configured queue
file (`CANON_KIT_QUEUE_FILE`) — two motivating parsers (`check-tag-lead-line`,
`check-queue-wrap`) scan the queue, which the manifest set excludes — with no
new knob. A grep error (not a no-match) is fail-closed (exit 2).

### check-md-refs

Invariant: every internal markdown link in the governed doc set resolves. A
relative-path target (with the source file's directory as the base) must be a
tracked file, or a directory holding tracked files; a `#anchor` — alone
(same-file) or trailing a path — must match the GitHub heading slug of a
heading in the target file. External URLs (`scheme://`, `mailto:`) are out of
scope: the network is not a gate dependency. The doc set is the manifest set
(kit READMEs and CLAUDE.md included) minus the `CANON_KIT_MDREF_EXCLUDE` globs
(default empty, for a consumer's generated docs); the scan runs over tracked
sources only, so an untracked local-only file (`BRIEF.local.md`) is a legitimate
link *source* that is never scanned and, being git-ignored-and-present, a
legitimate *target* that resolves without being tracked. A grep error is
fail-closed (exit 2). The link extractor is purely syntactic — it matches the
bracket-then-paren link shape (a `]` immediately followed by a parenthesized
target) without stripping code spans, so a literal markdown link written in
governed prose is scanned as a real link even inside inline backticks; to name
such a link in prose without tripping the gate, separate the `]` and the `(`
(a space, or a line break — the scan is per-line). The amendment `SPEC-*.md`
files escape only by lying outside the scanned doc set, not by any code-span
exemption. Links are this gate's charge; the sibling `check-docs-cmd` takes the
invoked commands and env knobs written inside fences and backticks, over the
same governed doc set (one shared set, no second knob).

### check-docs-link-convention

Invariant: every relative markdown link on a docs-site page obeys the
downward-citation *shape* — the resolution of those links is `check-md-refs`'
charge, and this gate owns shape alone. Each rule is scoped to the docs tree
because the "kit page" it turns on (a `<root>/<kit>/index.md`) exists only
there:

- **No directory-target link.** A relative link whose target names a directory
  (a trailing `/`, or a path that resolves to a tracked directory) must instead
  name the file — `kit/index.md`, never `kit/`. A bare directory link is
  ambiguous about what the page is citing and defeats anchor-level citation.
- **Anchored kit back-links.** On a kit page (`<root>/<kit>/index.md`), a link
  back to that same kit's own `README.md` or `SPEC.md` must carry a `#section`
  anchor. A page cites downward into a *named* section, never at the whole spec —
  the anti-restatement doctrine expressed as a link shape.

The scanned tree is `CANON_KIT_LINK_ROOT` (default `docs`; the fixture pair
overrides it with a positional arg pointing at a synthetic tree), walked for
every `*.md`. The link extractor is the same syntactic bracket-then-paren match
`check-md-refs` uses; `scheme://` and `mailto:` targets are out of scope, and a
pure `#anchor` (no path) satisfies neither rule. Per-site valve: a
`docs-link-exempt: <reason>` HTML comment on the link line or the one directly
above suppresses that one finding — for the rare legitimate directory link a
consumer's layout demands. A missing scan root is fail-closed (exit 2).

### check-docs-cmd

Invariant: every invoked repo-relative `.sh` path and every kit-prefixed env
knob written inside a fence or inline backticks in the governed doc set resolves
against the tree — the command/knob analog of `check-md-refs`, since a broken
`bash <path>` line or a retired knob name drifts silently where a broken link
would be caught. Two assertions:

- **(A) invoked command paths.** Inside a fenced block, a `.sh` path in
  *invocation* position — the first word of a `;`/`|`/`&&`-separated segment, or
  the first non-flag argument when that word is `bash`/`sh`/`source`/`.` — must
  resolve to a tracked file, tried doc-directory-relative first (a kit SPEC's
  own `bin/x.sh`) then repo-root-relative (a cross-kit `gate-sdk/bin/x.sh`).
  Only invocations are checked, so a path in argument position — a `cp
  templates/x.sh scripts/x.sh` install *destination*, which the consumer
  creates and this repo need not track — is never a finding. That is the
  deliberate calibration: the invariant's failure mode is a broken invocation,
  and scoping to the two named forms (a bare `<dir>/…/<name>.sh` and the `bash
  <path>` form) drops the hypothetical-install-target class by construction,
  with no whole-file exemption.
- **(B) env knobs.** Any backticked or fenced ALL-CAPS name carrying a kit
  prefix (the roster is each `gate_kit_roots` member's basename uppercased,
  hyphens to underscores, trailing `_`: `gate-sdk` → `GATE_SDK_`) must occur in
  the kits' tracked *code* — their shell sources and config templates, never
  their own prose, so a knob name-dropped only in markdown cannot self-satisfy.
  The corpus is the union across all kits, not the prefix owner alone: a
  namespaced knob may be read by a dependent kit (`GATE_SDK_LIB` is gate-sdk's,
  resolved in delegation-kit and evidence-kit), and the prefix marks scope, not
  location. A family stem — a caps run ending `_` because a placeholder or glob
  follows it (`EVIDENCE_KIT_RUN_<suite>`, `CANON_KIT_COMMENT_*`) — resolves when
  any code name extends it. Names with no kit prefix are out of scope, so
  generic shell vars never false-positive.

The governed doc set is exactly `check-md-refs`' — the manifest set minus
`CANON_KIT_MDREF_EXCLUDE` — shared, with no gate-specific knob. Prose outside
fences and backticks is never scanned; a hypothetical example path is written
unfenced, or its whole doc joins the per-file `CANON_KIT_MDREF_EXCLUDE` valve.
The knob set is built by a repo-root-anchored `git grep`, so it holds when the
fixture runner invokes from a case directory. Not a git repository, or a
`git grep` that errors, is fail-closed (exit 2). The `# graph:` manifest couples
the doc set to `scripts/*.sh` and every kit's shell sources (`kit:*.sh`), so a
script rename or a knob retirement re-fires the gate over the docs.

### templates/

`canon-config.sh` — the consumer config template documenting every knob.
`SPEC-amendment.md` — the amendment skeleton: delta sections plus the
Definition-of-Done checklist (causal completeness, no information lost on
merge, amendment deleted, none remaining, gaps filed as debt).

## Out of scope

A consumer's tier contract — which surfaces exist and what each owns — is its
own instance; canon-kit ships the topology rules, not the table. Glossary and
vision structural gates (`check-glossary-tiering`,
`check-glossary-entry-types`, `check-vision-tiering`,
`check-retired-term-coupling`) encode one instance's surface roles, entry
taxonomy, and ubiquitous-language couplings — rule content; their generic
cross-surface axis ships here as `check-surface-duplication`.
`check-comment-tier` splits: a consumer's *product* directive vocabulary
(`glossary:`, `diagram:`, `domain-enum`, …) is rule content, supplied as
`CANON_KIT_COMMENT_*` config, while the classifier, the kit-mechanism directive
roster, and the surface/positional parsing machinery ship here — the comment
surface is one of the enforced tiering surfaces. A `check-root-tiering` is a
pure consumer-filename allowlist with no mechanism residue. Global-constant
literal gates (the pagination-literal pattern) are per-constant rule content.
Diagram/spec annotation couplings are unclaimed, a later kit's scope.
