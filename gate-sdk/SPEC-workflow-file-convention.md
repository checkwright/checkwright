# SPEC amendment: workflow-file-convention

Gives `GATE_SDK_WORKFLOW_DIR` a stated surface contract — a two-tier membership
rule, a header requirement on one tier, and an extension rule — plus one gate,
`check-workflow-tiering`, for the mechanizable half. gate-sdk owns the knob, so
gate-sdk owns the directory's contract; the kits that write into it own their
individual files.

This lands as a **fix with a backlog**, not a greenfield boundary proof: two
tracked files violate the header requirement today and are renamed by the same
unit that states it.

## What changes

### New section: gate-sdk/SPEC.md §The workflow directory

**Two tiers, partitioned by tracking.** Every member of the workflow directory
is either a **checked projection** (tracked, committed, gate-read) or **local
capture** (gitignored, advisory, drained by a named reclaim path). A member that
is neither tracked nor ignored is the drift state the gate refuses: an
uncommitted file no reviewer sees and no `.gitignore` line accounts for.

**The header requirement follows tracking, and here is why it must.** Local
capture's reclaim path is whole-file truncation (`: > <file>`, the shape every
capture log's close-stage step uses), which erases a header on every drain. A
header requirement on that tier would fight the tier's own reclaim mechanism —
either the header is re-seeded on every clear, adding a writer to a surface
whose whole point is that any appender may write it, or it decays to a rule
violated by correct operation. So: **checked projections carry the header;
local capture carries none.** That asymmetry is the substantive content of the
tracked-vs-gitignored axis, not a convenience.

**The header form.** A checked projection's first line is `# contract: `
followed by one of two ruled payloads:

- **Pointer form** — `<owner-path> §<section>`, optionally followed by
  ` — <grammar or gloss>`. The path is a tracked file and the section resolves;
  `check-spec-pointer` already owns that resolution, and the em-dash tail is
  already stripped before heading matching, so a grammar line may ride the same
  header as its pointer. This is the default form.
- **Version-marker form** — `<format-name> v<N>` (`^[a-z0-9-]+ v[0-9]+$`), used
  only where a gate parses the header itself as a wire-format version. The
  owning SPEC states that it does.

Requiring *some* header would gate nothing — the tracked side already satisfies
it — so the requirement is the **prefix with a ruled payload**. Both payload
forms are machine-recognizable, which is what keeps the rule a rule rather than
a description.

**The extension rule keys on writer and reader, not on tier.** These are two
independent axes, and conflating them is why the convention resisted statement:
a directory holds tracked `.md` (`gap-inbox.md`) beside gitignored `.md`
(`essay-harvest.md`), so no extension tracks tracking.

- `.txt` — a **record file with a stated line grammar** that a gate or `bin/`
  affordance parses field-wise.
- `.md` — a **prose surface a human reads and dispositions**, machine-read only
  for emptiness or for a bullet count.
- `.log` — an **append-only capture stream** written by tooling at the moment of
  an event, triaged in bulk and cleared wholesale; no per-line grammar contract,
  which is exactly why nothing parses it field-wise.

A new file's extension is therefore determined, not chosen: ask which of the
three describes its writer and its reader.

### New gate: `check-workflow-tiering`

Invariant, over `GATE_SDK_WORKFLOW_DIR`: (A) **partition totality** — every
member is tracked or ignored, never neither; (B) **header presence and form** —
every tracked member's first line is `# contract: ` with a payload matching the
pointer form or the version-marker form.

It deliberately does **not** re-resolve the pointer's path and heading:
`check-spec-pointer` already does, and a second resolver is the parallel copy
canon-kit's own tiering rule bans. The division is presence-and-shape here,
resolution there — the same split `check-comment-tier` and `check-spec-pointer`
already hold between them.

Fail-closed on an unreadable member and on a workflow directory that does not
exist while the knob names one. No new knob: `GATE_SDK_WORKFLOW_DIR` is the
whole configuration surface. Tier `precommit`; `# graph:` manifest couples it to
the workflow dir and `.gitignore`.

Calibration: the extension rule above is **not** gated. Deciding whether a file
is a record, a prose surface, or a capture stream is the judgment the rule
exists to guide, and mechanizing it would mean inferring a writer's intent from
a file's bytes. Under the enforcement-first carve-out it takes the other
disposition available to an un-gateable class — a cadenced review entry on the
consumer's audit roster, due at each workflow-directory addition — rather than
being stated and forgotten.

### The backlog this unit clears

Two tracked members carry a descriptive `#` header with no `# contract:` prefix
and are rewritten to the pointer form, keeping their existing grammar text as
the em-dash tail so nothing is lost:

- `release-sweep-evidence.txt` — owner is the consumer's release runbook step
  that writes it.
- `validate-baseline.txt` — owner is evidence-kit's held-constant baseline
  contract (`EVIDENCE_KIT_BASELINE_FILE`). Two literal sites move with it, both
  verified at the write site: the seed in **evidence-kit/smoke/install.sh**
  (the smoke is the shipped example of the convention and must demonstrate it)
  and the documented seed snippet in **evidence-kit/README.md**, whose
  docs-site projection regenerates rather than being edited. The smoke's
  nested-fixture baseline and **evidence-kit/smoke/violation.sh** are *not*
  sites: the former writes its own throwaway header inside a temp consumer, and
  the latter appends a data line and no header at all.

The rewrite is parse-safe, checked rather than assumed: `ek_data_lines`
(evidence-kit/lib/evidence.sh) drops every line whose first non-blank character
is `#`, so `check-evidence-baseline` reads the same rows before and after. That
verification is the point — the whole reason `validate-evidence.txt` is exempt
below is that a header change there *would* have been parsed.

A third member, `validate-evidence.txt`, carries `# contract: evidence-manifest
v1` — the prefix, but a wire-format version marker that `check-evidence-manifest`
parses rather than a doc pointer. It is **not** renamed; it is the case the
version-marker payload form exists for, and evidence-kit/SPEC.md already states
that its header is a version marker rather than a pointer. Had the rule demanded
the pointer form outright, the correct outcome would have been breaking a gate
to satisfy a convention.

## Producers and consumers

**New state: the two-tier membership contract.**

- *Producer* — every kit `bin/` affordance and gate that creates a workflow-dir
  file, plus the consumer's `.gitignore` for the capture tier. Enabling config:
  `GATE_SDK_WORKFLOW_DIR` already defaults to `.workflow` and is already read by
  every kit that writes there, so the contract is live on vendor rather than on
  a new setting.
- *Consumer* — `check-workflow-tiering` at pre-commit; and human authors, for
  whom the SPEC section is the answer to "what extension, and does it need a
  header".

**New interface: the ruled header payload forms.**

- *Producer* — the file's creating affordance seeds the header (the pattern
  `file-gap.sh` already follows: seed the contract header when the file does not
  exist).
- *Consumers* — (1) `check-workflow-tiering` reads the prefix and the payload
  shape; (2) `check-spec-pointer` reads the pointer payload and resolves its path
  and `§section`; (3) `check-evidence-manifest` reads the version-marker payload
  as a format version.

**Fields and their named readers.**

- `<owner-path>` — read by `check-spec-pointer` at path resolution, and by a
  reader who needs the file's contract.
- `§<section>` — read by `check-spec-pointer` at heading resolution.
- ` — <grammar tail>` — read by a human authoring a new line into the file;
  stripped by `check-spec-pointer` before heading matching, so it has a reader
  and no gate mis-parses it.
- `<format-name> v<N>` — read by the parsing gate that owns the format
  (`check-evidence-manifest` today) as its wire-version check.

No field is added that lacks a reader. In particular the tier is **not** marked
in the file: it is derived from `git check-ignore`, which is why the axis was
chosen — a marker would be a second source for a fact git already owns.

## Existing sections updated

- **gate-sdk/SPEC.md §Layout and configuration** — the `GATE_SDK_WORKFLOW_DIR`
  line gains a pointer to the new §The workflow directory rather than restating
  it.
- **gate-sdk/SPEC.md §Per-component contracts** gains
  `### check-workflow-tiering`.
- **canon-kit/SPEC.md §check-spec-pointer** and **§check-comment-tier** — the
  blessed directive set widens from `.workflow/*.txt` `# contract:` headers to
  every **tracked** workflow-dir member, so a tracked `.md` projection's header
  (`gap-inbox.md`) is resolved like its `.txt` siblings instead of falling
  outside the directive set on extension alone. The tracked qualifier is what
  keeps capture logs out. The change lands at **one** site —
  `_spec_comment_surface` in `canon-kit/lib/spec.sh`, where the `*.txt` glob is
  literal — which both gates read; the two SPEC sections document it, they do
  not each own a copy. Widening admits exactly one new member here
  (`gap-inbox.md`), whose header is already pointer-form and resolves.
- **The consumer's canon config** — `CANON_KIT_COMMENT_WHITELIST` drops
  `release-sweep-evidence.txt` and `validate-baseline.txt` once their headers are
  pointer-form, retaining only the version-marker file. The whitelist's stated
  reason ("a format-doc header, not a spec pointer") stops being true for the two
  as this unit lands, so leaving them whitelisted would preserve a false comment.
- **evidence-kit/SPEC.md** — the baseline section states the file's header in its
  new pointer form; the manifest section's existing statement that its header is
  a version marker gains the citation to §The workflow directory naming that as a
  ruled payload form rather than an exception.
- **The consumer's release runbook** — the release-sweep evidence step states the
  file's new header.
- **The consumer's `gates.list`**, the generated pre-commit hook,
  `docs/check-graph.html`, and `docs/enforcement.md` all take the new gate.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
