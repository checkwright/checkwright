# SPEC amendment: policy-choice

A kit that encodes a policy, such as how far a marker may sit from what it marks, has been shipping this project's one choice as the kit's behavior. An adopter who would set it differently meets a red they can only bypass. doctrine-kit/DOCTRINE.md has the provenance seam (content moves to config) and widest-true-tier placement, but no rule that turns a policy into a consumer choice. So every design session defaults to its own tree's preference, and the operator pays for each correction.

**The ruling: a methodology rule, Policy-as-choice, with a discriminator, and a cadence rather than a gate.** A kit behavior that encodes a policy ships as a consumer-selectable set of alternatives, `off` among them where the gate's other assertions survive it, a per-record exception valve where one fits, and the consumer binds its choice. The discriminator is whether another value keeps the gate's stated contract true. A grammar the kit ships, a contract invariant, an external limit and a value that changes no verdict are not policy. Where a structural bound exists (the token, the clause, the sentence) it beats a count. The rule is a methodology rule rather than a craft rule because it bears on every kit-surface design edit, and the digest is where a design session meets it before it writes the literal.

**Three units of this iteration ride the one amendment,** because each is the rule applied to one question and splitting them would state the rule three times:

- `consumer-policy-rule-absent` — the rule, its digest and its cadence (deltas 1, 2 and 6).
- `kit-knob-consumer-adapter-convention` — the rule on a knob's config surface: a knob ships a working value, and a consumer adapter extends the set rather than being its only working member (deltas 3, 4 and 5).
- `matching-window-roster-absent` — the rule on matching windows, and the roster refused with its ground (delta 7).

The two single-calibration units, `msg-uuid-reach-unbounded` and `measured-claim-span-unbounded`, carry their own amendments, uuid-reach in gate-sdk and claim-span in canon-kit, which apply the rule and do not depend on its text landing first.

**Enforcement is a cadence, not a gate, and the census is why.** The audit the entry's direction ordered ran at this spec: read-only sweeps over the 117 gate modules `--list` attributes to a kit, each read beside its kit's knob table and SPEC section, and over the 110 gate files deleted in history. Findings, verdict-affecting and baked with no knob: `MARKER_REACH` (100 code points, `native/src/gates/knob_citation.rs`), an inline 400-character search window (`knob_default_coupling.rs`), `WINDOW` (8 lines, `assertion_strength.rs`), the measured-claim paragraph span, the session-UUID pattern's `.*`, and `CANON_SPEC_PRUNE`'s `docs/*` (`native/src/spec.rs`). The finding that settles the enforcement question: the baked windows are named constants, inline literals, a paragraph walk and a pattern quantifier in about equal measure, and `SPAN_CAP` (`queue_prose_precondition.rs`), which the filings counted as a window, is the width of the excerpt the finding message quotes and changes no verdict. A scan over named constants would reach part of the class and mislabel part of what it reached, which is a stand-in rather than a check. So the rule is judgment, and its cadence is an audit-roster class due on the events that introduce a literal bound. The history sweep found **no** true decommission: 106 of the 110 deletions are ports, two are fixture files, and the other two are a rename and a same-day revert. The cost of baked policy shows instead as recalibrations of live gates after false reds (`check-queue-prose-precondition`'s phrase list twice, `check-action-run-shell`'s reach, the account-identification pattern's reach). The census is filed as this iteration's survey record block "which kit gates bake a policy…", and its follow-ups are in the gap inbox.

**What this amendment does not do.** It converts no baked calibration it found into a knob. The entry's direction has the audit supply the rule's worked examples and follow-up entries, and each conversion is its own unit with a fixture and a binding. Five follow-ups are filed: the three baked windows, the spec-prune layout literal, eight SPEC-grounded fixed values owing a ruling under the discriminator, three knobs that accept no `off`, and a census of the policy knobs this repo inherits by default rather than binds.

## What changes

### (1) DOCTRINE.md gains methodology rule 13, Policy-as-choice, and the craft register renumbers {design-bearing}

**Not yet applied.** In `doctrine-kit/DOCTRINE.md`, after rule 12 (Probe-before-assertion) and its digest, insert as the last rule of `## Methodology-maintenance rules`:

> 13. **Policy-as-choice.** A kit behavior that encodes a policy ships as a consumer-selectable set of alternatives, never as its author's one choice baked in. A policy is a calibration another adopter could reasonably set differently while the gate's stated contract stays true: a threshold, a cap, a reach or window, an age, a class set, a phrase list, a fail-or-skip disposition. The set holds `off` wherever the gate's other assertions survive without that calibration, and a per-record exception valve where one record can legitimately sit outside it; `off` for a calibration that is the gate's whole predicate is unregistering the gate, which the registry already offers. The kit's table carries the default: for a calibration older than its knob, the value it shipped with, so an upgrade moves no verdict; for a new one, the kit's recommendation with its ground. A consumer that holds a choice binds it in its own config, and the kit's author is such a consumer. Three things are not policy, because another value breaks something other than taste: a grammar the kit ships (a marker's spelling, a tag's syntax, the heading of a file the kit ships), a contract invariant or an external limit (an engine's bound, a standard's format), and a value that changes no verdict (a quoted excerpt's width). Where a structural bound exists (the token, the clause, the sentence a grammar already defines) it beats a count, because it leaves nothing to choose. A knob's alternatives are values the kit ships working; a consumer-authored adapter extends them and is never the only working configuration ([gate-sdk/SPEC.md](../gate-sdk/SPEC.md) §The knob file). *Under agent work:* a session designing a kit feature sees one tree, its own, so that tree's preference reads as the obvious value and ships as everyone's; an adopter meets it as a red they can only bypass, and each bypass teaches bypassing. The discriminator costs a knob row at design time and a correction afterwards. *Enforced by:* judgment with a capture mechanism, not a gate. A baked calibration is as often an inline literal as a named constant, and a constant named like a window can be a display width, so a scan over constants is a stand-in reaching part of the class and mislabelling part of what it reaches (the Enforcement-first false-positive carve-out). Its cadence is a consumer audit-roster class due when a gate gains or changes a literal bound or a kit template gains a pattern ([lifecycle-kit/SPEC.md](../lifecycle-kit/SPEC.md) §The audit roster), and every calibration the rule turns into a knob is listed by `--emit knob-roster` ([gate-sdk/SPEC.md](../gate-sdk/SPEC.md) §The knob file).
>
>     *Digest:* a kit ships a calibration as a consumer-selectable set, off among them, never its author's one choice; each consumer binds its own.

The engineering-craft rules renumber from 13–26 to 14–27. Rule "Read the subject, never a stand-in for it" cites "rule 24's printed set size"; that citation becomes "the printed set size of *An assertion reports the size of the set it ranged over*", naming its referent rather than its number (canon-kit/SPEC.md §check-amendment-retired-spelling states why a renumber is dissolved this way rather than detected).

**If `probe-before-assertion-doctrine` lands in the same build batch,** its amendment rewrites rule 12's body and digest; the two edits touch different rules and land in either order. **If it lands in a different batch,** nothing here changes, since the probe-corpus amendment cites rules by name.

### (2) The always-loaded digest gains the rule {mechanical}

**Not yet applied.** `CLAUDE.md` §Delivery doctrine gains, after the Probe-before-assertion bullet, the bullet `- **Policy-as-choice** — ` followed by the digest text of delta 1 verbatim (check-doctrine-registration assertions B, C and F). Running `--install-doctrine` produces the same line; either route is fine. The always-loaded file grows by one line, so `check-surface-ratchet` reds the growing commit and prints the ceiling re-stamp, which that commit carries (context-kit/SPEC.md §The surface ratchet).

### (3) gate-sdk/SPEC.md §The knob file: a knob that selects behavior ships a working value {design-bearing}

**Not yet applied.** After the paragraph beginning "**A command knob** (`<KIT>_…_CMD`)", add:

> **A knob that selects behavior ships a working value.** Its alternatives are values the kit implements: a bundled adapter named bare in the knob's own vocabulary, a kit-shipped `--emit` arm reached through the front-end (`bash gate-sdk/bin/run-gates.sh --emit <arm> [<operand>…]`), or `off`, which a command knob spells as its empty value. A consumer command extends that set and is never its only working member, so no kit default names a program the kit does not ship. The front-end form is how a kit reaches a bundled adapter, so a new kit-shipped adapter is an `--emit` arm rather than a further bare name, and a consumer's configured value names it the way it names its own command. A knob whose empty default disarms its gate is declared on the gate with `# armed-by:` (§The install disposition), so `doctor` names the disarmed state and an adopter never reads a clean line as coverage.

### (4) evidence-kit/SPEC.md §The evidence adapters: the deferred refusal becomes the convention's instance {mechanical}

**Not yet applied.** Replace the paragraph beginning "**A third built-in adapter beside `exit-code` and `libtest` is refused**" with:

> **A third bare-name adapter beside `exit-code` and `libtest` is refused.** A kit-shipped parser is an `--emit` arm reached through the front-end, the convention gate-sdk/SPEC.md §The knob file names, which is how a parser value reaches `parse-gates-log` and `parse-smoke-log`. A consumer whose parser the kit does not ship keeps authoring a command, and that command extends the set rather than replacing a working default, since `exit-code` is the default.

### (5) The five command-knob claim gates declare their arming knob {mechanical}

**Not yet applied.** Each of these descriptors gains one `# armed-by:` line naming the command knob whose empty value makes its gate print clean and assert nothing:

- `canon-kit/checks/check-measured-claim.gate` — `# armed-by: CANON_KIT_MEASURED_CLAIMS_CMD`
- `canon-kit/checks/check-unmarked-claim.gate` — `# armed-by: CANON_KIT_CLAIM_CLASSES_CMD`
- `canon-kit/checks/check-prose-enum.gate` — `# armed-by: CANON_KIT_ENUM_SETS_CMD`
- `canon-kit/checks/check-payload-claim.gate` — `# armed-by: CANON_KIT_PAYLOAD_CLAIMS_CMD`
- `canon-kit/checks/check-install-claim.gate` — `# armed-by: CANON_KIT_INSTALL_TRANSPORTS_CMD`

Each member's empty-value branch was read at authoring: the module's first branch prints its clean line when `spec::command(<knob>)` is empty. `check-install-claim` also skips on an empty section pattern. The arming declaration takes one knob, so that second disarm stays undeclared, and the honest limit §The install disposition already states covers it. The three other `*_CMD` knobs (`CONTEXT_KIT_HOOK_CMD`, `DELEGATION_KIT_REFRESH_CMD`, `DELEGATION_KIT_LIVENESS_CMD`) are read by hooks, not gates, so no declaration has a home for them. `check-install-disposition`'s arming count moves from five to ten. This repo sets all five knobs, so its `doctor` output is unchanged.

### (6) This repo's audit roster gains the `baked-calibration` class, with this spec's census as its first sweep {mechanical}

**Not yet applied.** `.workflow/audit-roster.txt` gains a block:

```
class: baked-calibration
scope: a kit gate or kit-shipped template that bakes a policy (a threshold, cap, reach or window, age, class set, phrase list or fail-or-skip disposition) with no knob, per doctrine-kit/DOCTRINE.md Policy-as-choice; un-gateable, since a scan over literals cannot tell a policy bound from an index offset or a display width. Corpus: the gate modules and kit templates the range added or changed, read at each literal compared against a length, count, offset or date, and each pattern quantifier; for each, apply the rule's discriminator (does another value keep the gate's stated contract true?) and its three exclusions (kit-shipped grammar, contract invariant or external limit, verdict-neutral value). A structural bound beats a count, so a finding's remedy names one where the grammar has it. A finding is filed as a gap naming its knob and default, never converted in the sweep.
due: a gate module gaining or changing a literal bound; a kit template gaining or changing a pattern; a new gate
last: consumer-policy-seam spec
corpus: the 117 gate modules --list attributes to a kit, each with its kit's knob table and SPEC section, plus gate-sdk/templates/msg-patterns.list and portability-patterns.list, plus the 110 gate files deleted in history (106 ports, 2 fixture files, 1 rename, 1 revert); delegated to four worktree sweeps with no binary
hits: 6
declined: kit-owned grammar (marker, valve and tag spellings, the shared line-or-above exempt window, closed vocabularies other kit mechanism keys on), external formats (git abbreviation floor, sha256 length, CommonMark headings, HTML elements, errno), verdict-neutral widths (SPAN_CAP, LEAD_WIDTH, fence_run TAIL, excerpt truncations), check-crate-arms (install: never, no adopter); eight SPEC-grounded fixed values the sweeps split on are filed as one gap for a ruling rather than counted here
```

`hits: 6` counts the six verdict-affecting baked calibrations the preamble names. `check-audit-roster` holds the block's shape.

### (7) gate-sdk/SPEC.md §Calibration lessons: a matching window is a calibration, and no window roster is kept {design-bearing}

**Not yet applied.** Add to the bullet list of §Calibration lessons (paid for, now design rules):

> - **A matching window is a calibration.** A span over which a gate relates two things it matched (a marker to its claim, a knob name to a nearby value, a lead-in to a token) is bounded by structure where the grammar already has one (the token, the clause, the sentence), and otherwise by a knob with `off` where the gate's other assertions survive it; a bare count baked into a module is the defect doctrine-kit's Policy-as-choice rule names. **No roster of windows is kept, because none is derivable:** a window's bound is as often an inline literal as a named constant, and a constant named like a window can be a display width, so no scan decides membership, and a hand-kept list is what Derivation-first forbids. The knob-backed windows are listed by `--emit knob-roster` with every other knob; the in-code residue is a consumer audit-roster sweep's.

## Producers and consumers

- **The rule and its digest** (deltas 1 and 2). Producer: `doctrine-kit/DOCTRINE.md`. Consumers: `--install-doctrine`, which derives the bullet from the `*Digest:*` trailer; `check-doctrine-registration`, whose assertion B reds `CLAUDE.md` until the bullet exists, C until it names a rule, E until the rule carries one trailer, and F until the bullet text matches it; `--emit stage-rules`, which reads the craft register by `*Stages:*` trailer and prints each rule's number at run time, so the renumber moves its output without a code change; every session through the always-loaded digest.
- **The renumbered craft register** (delta 1). No gate reads a craft rule's number. Citations of a craft rule by number were probed with `git grep -n -E "(DOCTRINE|doctrine)[^.]{0,40}(rule|§) ?[0-9]+|rule [0-9]+'s" -- ':!TASK-QUEUE.md'`, which returns 217 lines — the `rule [0-9]+'s` alternative alone also matches guard-kit's own, unrelated numbered ruleset, read by hand to isolate the doctrine-kit citations. The one intra-doctrine citation is rewritten by name in delta 1. The one outside it the probe's first alternative catches, `.workflow/release-declarations.md`'s "doctrine-kit rule 18" bullet, is in the pending release's Behavior changes, so it would publish a stale number; it is rewritten to name the rule (update target below). A second, spelled `doctrine-kit/DOCTRINE.md rule 1` in `.workflow/audit-roster.txt`, sits outside the probe's reach — its `.md` breaks the `[^.]{0,40}` run before `rule` — and was found instead by `grep -n "rule 1"`; it cites a methodology rule the insertion does not move, so nothing there changes.
- **The adapter convention** (delta 3). Producer: gate-sdk/SPEC.md. Consumers: a kit author minting a knob, and delta 4's repointed paragraph. No gate reads it; the arming half has one (delta 5).
- **The arming declarations** (delta 5). Consumers: `check-install-disposition` assertion D, which reds a knob no static kit declares or one of another kit, so all five pass as `CANON_KIT_*` rows of canon-kit's table; `doctor`, which prints a `disarmed` line for a registered member whose knob resolves empty. Point 6: each of the five members' satisfying value is named in delta 5.
- **The roster class** (delta 6). Producer: this repo's tracked roster. Consumers: the close stage's roster review, which reads `due` and `last`; `check-audit-roster`, which holds the seven-key shape; the next sweeping session, which reads `scope`.
- **The window rule** (delta 7). Consumer: a gate author. No gate reads it.
- **Point 5.** No delta narrows a corpus.

## Existing sections updated

Rosters from `git grep -n -c "Probe-before-assertion\|Methodology-maintenance rules" -- ':!TASK-QUEUE.md'`, the numbered-citation probe above, `git grep -n "third built-in adapter"`, and `git grep -n "_CMD" -- 'canon-kit/checks/*.gate'` (no hits: no claim gate declares its arming knob today), run 2026-09-22.

- `doctrine-kit/DOCTRINE.md`, the new rule 13, the craft renumber and the Read-the-subject rule's citation (delta 1).
- `CLAUDE.md` §Delivery doctrine (delta 2).
- `gate-sdk/SPEC.md` §The knob file (delta 3) and §Calibration lessons (paid for, now design rules) (delta 7).
- `evidence-kit/SPEC.md` §The evidence adapters (delta 4).
- `canon-kit/checks/check-measured-claim.gate`, `check-unmarked-claim.gate`, `check-prose-enum.gate`, `check-payload-claim.gate`, `check-install-claim.gate` (delta 5).
- `.workflow/audit-roster.txt` (delta 6).
- `.workflow/release-declarations.md`: the "doctrine-kit rule 18" bullet renamed to the rule's name (delta 1); a Behavior changes bullet each for the new doctrine rule, which reds `check-doctrine-registration` assertion B on an adopter's agent file until `--install-doctrine` re-runs (delta 1), and for the adapter convention and the arming declarations (deltas 3 and 5).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/doctrine-kit/DOCTRINE.md`.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/evidence-kit/SPEC.md`.

## Retired spellings

- `Naming the convention is the deliverable of a queued entry` — the deferral the convention discharges (delta 4).

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the rule, the convention, the arming declarations and the roster class.
- [ ] **Instruction surfaces: instruction only.** The `CLAUDE.md` bullet carries the digest and nothing else.
- [ ] **Merged with no information lost.** The evidence-kit paragraph is re-phrased, not appended to; the rule's grounds live in its own body.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md` at the root).
- [ ] **Entries moved.** `consumer-policy-rule-absent`, `kit-knob-consumer-adapter-convention` and `matching-window-roster-absent` move to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` is green on the declared spelling.
- [ ] **Gaps filed.** The census follow-ups are in the gap inbox; any gap found during the work is filed there too.
