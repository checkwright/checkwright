# SPEC amendment: comment-tier-id-directive

Pairs `comment-tier-id-directive`. A consumer that names its rules by id often declares each one in a code comment, such as `// rule R-1: a request carries one owner`, which `check-rule-id-coverage` reads as the declaration. Once `CANON_KIT_COMMENT_SURFACE` reaches that source, `check-comment-tier` flags the same comment as prose. The consumer's only valve today is `CANON_KIT_COMMENT_REASON`, whose entries match as a substring or a bounded bare word, so blessing `rule` blesses every comment that holds the word. This amendment makes a comment that `check-rule-id-coverage` reads as a declaration a reason directive. The gate reads the id shape that gate already declares, and needs no knob of its own.

**What the tree does today**, read from `canon-kit/SPEC.md`, `native/src/gates/comment_tier.rs` and `native/src/gates/rule_id_coverage.rs`:

- **A declaring comment is flagged as prose.** In a scratch tree with `CANON_KIT_COMMENT_SURFACE=**/*.rs` and `CANON_KIT_RULE_ID_DECLARE_GLOBS=src/*.rs`, a `// rule R-1: …` line in `src/a.rs` reds `COMMENT-TIER: … src/a.rs:1: rule R-1: …`, exit 1. The same line in a file outside the declare globs reds the same way (probe: run `check-comment-tier` on such a scratch tree through the binary).
- **The extras knobs match a literal, not a shape.** `CANON_KIT_COMMENT_MACHINE` and `_REASON` append literal tokens: a `keyword:` matches as a substring and a bare word is non-alphanumeric-bounded (canon-kit/SPEC.md §check-comment-tier, the closing paragraph). No entry can say "`rule`, then an id, then a colon" without also blessing every other comment holding `rule:` (probe: `git grep -n "reads no regex engine" -- canon-kit/SPEC.md`).
- **The id shape is already declared, and validated.** `CANON_KIT_RULE_ID_DECLARES` holds one one-group ERE per rule family, matched over the declaring files `CANON_KIT_RULE_ID_DECLARE_GLOBS` names, through the pruning walk (§check-rule-id-coverage, *The two sides*). The kit table validator refuses malformed rule-id config at exit 2 for every canon-kit gate, `check-comment-tier` among them (probe: `git grep -n "rule_id_refusals" -- native/src/knobs/canon_kit.rs`).
- **This repo declares no rule family**, so its own verdict cannot move (probe: `git grep -n "CANON_KIT_RULE_ID" -- scripts`, which finds none).

## What changes

### (1) A rule-id declaration is a reason directive {design-bearing} {user-facing: operator direction 2026-11-24, lead-relayed (not a ruling), on the set record: the gate reads check-rule-id-coverage's declared id shapes rather than taking a new knob}

§check-comment-tier's reason-directive bullet is rephrased, and a calibration bullet is added after the extras bullet. **Not yet applied:**

> - a **reason directive**, a spec pointer, usage synopsis or positional justification: `spec:`, `usage:`, `exception-list:`, `no-fixture:`, `no-port:`, `port-until:`, `assertion`, `permanent:`, `TODO(task:`, `TODO(spec-ambiguity)`, or a **rule-id declaration** (below). It blesses a bounded window, its own line plus continuation lines up to `CANON_KIT_COMMENT_RUN_CAP` total physical comment lines, blank `#` lines counted. A directive mid-run opens a fresh window from its own line, and each comment line beyond a window classifies on its own;

> - A **rule-id declaration** is a full-line comment that §check-rule-id-coverage reads as declaring an id: a line of one of its declaring files that holds a match of a family's `CANON_KIT_RULE_ID_DECLARES` pattern. The declaring files and the match are that gate's own, so the two gates cannot disagree about which comments declare. The pattern is matched against the whole line, comment leader included, and it applies whatever roster the file classifies under. The same comment in a file outside the declaring set is read like any other prose. An empty `CANON_KIT_RULE_ID_DECLARES` adds nothing, and malformed rule-id config refuses at exit 2 through the table validator (§Layout and configuration), so this gate inherits that gate's configuration rather than carrying its own.

- **Why a declaration earns the blessing.** The tier rule blesses a directive for the coupling it makes, and `graph:` and `assertion` are blessed because a gate checks them both ways. `check-rule-id-coverage` checks a declaration both ways too: an id no spec cites reds as *uncited*, and a cited id nothing declares reds as *undeclared*. A declaring comment is therefore a binding, not a restatement. Outside the declaring set the same text binds to nothing, which is why it stays flagged there.
- **Why no knob of its own.** A comment-tier pattern knob would be a second copy of the id shape the consumer already declares, and the two could drift: an id shape added to one gate would still red in the other. Reading the declare patterns makes the shape single-sourced (doctrine-kit/DOCTRINE.md, Content-tiering).
- **Why the whole line.** `check-rule-id-coverage` matches each line whole, so a consumer's pattern may anchor on the comment leader. A match against the comment body alone would not be that gate's match.
- **Honest limit.** The coupling is gated only where `check-rule-id-coverage` is registered. A consumer setting the rule-id knobs without registering that gate blesses its declaring comments without the check that earns the blessing, the same exposure a `spec:` pointer has where `check-spec-pointer` is unregistered.

### (2) The gate compiles the declare patterns {mechanical}

The closing paragraph of §check-comment-tier is rephrased. **Not yet applied:**

> **Declared by `check-comment-tier.gate`, dispatching to the binary.** Its `couples=` carries `*.gate` as a **bare glob** beside the shell and workflow spellings, since the derived corpus scans both declaration forms. The implementation couples through `knob:CANON_KIT_COMMENT_SURFACE`, which carries whatever language a tree names there. A token naming this repo's crate directory would publish one consumer's layout into a kit file (gate-sdk/SPEC.md §The provenance seam) and be false for every other. `knob:CANON_KIT_RULE_ID_DECLARE_GLOBS` couples the declaring set the gate walks. The directive roster is a set of literal tokens, matched as a substring for a `keyword:` and non-alphanumeric-bounded for a bare word, which is joining rather than interpreting. The one pattern the gate compiles is a rule family's declare ERE, through the one-group matcher §check-rule-id-coverage uses (gate-sdk/SPEC.md §The POSIX ERE matcher).

- **Why the sentence is rewritten, not appended to.** "The gate reads no regex engine" becomes false the moment the declare patterns are read. The roster's literal matching stays true and stays stated.

## Producers and consumers

- **`CANON_KIT_RULE_ID_DECLARES` and `CANON_KIT_RULE_ID_DECLARE_GLOBS`** (existing knobs) — producer: consumer config. New consumer: `check-comment-tier`, beside `check-rule-id-coverage`. The gate-table row for `check-comment-tier` in `native/src/gates/mod.rs` declares both, `CANON_KIT_RULE_ID_SPEC_GLOBS`, `_CITES` and `_HOMES` (which the table validator reads), and a `glob:knob:CANON_KIT_RULE_ID_DECLARE_GLOBS` root row, so `check-reads-couples` sees the walk.
- **The declaring set and the declare match** (existing derivation in `native/src/gates/rule_id_coverage.rs`) — producer: that module, whose file walk and family compile become crate-visible to share. Consumer: `comment_tier.rs`, which marks each corpus file in the set and, on a full-line comment of such a file, treats a declare match as a reason directive opening the window.
- **The clean and violation lines** — unchanged in grammar. A declaring comment no longer appears among the violations, and the clean line's governed-source count is unchanged by this delta, since the corpus is not widened.
- **Point 5 (narrowing).** Nothing narrows. The delta only blesses lines that were flagged, and only where a rule family is configured. A tree with no family sees byte-identical output.
- **Point 6 (every member).** The members are every comment style the walker records a comment from, since the declaration must bless in each: `#` lines, `//` lines and `/* */` blocks, opened or continued, the four `record` call sites in `Walk::line` (probe: `git grep -n "self.record(" -- native/src/gates/comment_tier.rs`). The test sits before roster classification, so it reaches every roster `classify` returns: the shell roster, the txt roster and the workflow tier (probe: `git grep -n "return (Style::" -- native/src/gates/comment_tier.rs`).

## Existing sections updated

Roster from `git grep -n -e "check-comment-tier" -e "CANON_KIT_RULE_ID_DECLARES"` over the tracked tree, the posts, the queue and the generated mirrors excluded, narrowed to the sites that state the gate's directive roster, its reads or its matcher.

- `canon-kit/SPEC.md` §check-comment-tier — the reason-directive bullet, the new calibration bullet, and the closing paragraph (deltas 1 and 2). §check-rule-id-coverage, the paragraph on its valve, gains one sentence: a declaring comment is a reason directive to §check-comment-tier, so the declaration needs no exempt tag (delta 1).
- `native/src/gates/comment_tier.rs` — the declaration test and its unit tests: a declaring comment blessed with its window, the same comment outside the declaring set flagged, and an empty family set changing nothing (delta 1).
- `native/src/gates/rule_id_coverage.rs` — the walk and family compile shared (delta 1).
- `native/src/gates/mod.rs` — the `check-comment-tier` row's knob roster and root rows (delta 2).
- `canon-kit/checks/check-comment-tier.gate` — `knob:CANON_KIT_RULE_ID_DECLARE_GLOBS` in `couples=` (delta 2).
- `canon-kit/gate-tests/check-comment-tier/good/` — a rule family configured in `scripts/canon-config.knobs`, and a new declaring `.rs` file whose `// rule R-1: …` comment and its wrapped continuation pass. `expect.txt`'s governed-source count moves by that file (delta 1).
- `canon-kit/gate-tests/check-comment-tier/bad/` — the same family configured, and a new `.rs` file outside the declare globs whose identical comment is flagged, its line added to `expect.txt` (delta 1).
- Generated projections the section moves: `docs/canon-kit/SPEC.md` and `docs/llms-full.txt`, each regenerated by the command its gate prints on red (docs/generated-projections.md) (all deltas).
- `.workflow/release-declarations.md` — *Behavior changes* gains `check-comment-tier`: a comment `check-rule-id-coverage` reads as a declaration is now a reason directive, so a consumer that blessed its declarations through `CANON_KIT_COMMENT_REASON` or exempted them can drop that entry (delta 1).

## Retired spellings

- None — the directive is an addition, and the rewritten sentence retires a claim, not a spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the declaration directive and the knobs it reads.
- [ ] **Instruction surfaces: instruction only** — no template changes; the release declaration states the action an adopter takes.
- [ ] **Merged with no information lost** — the bullet, the calibration bullet and the closing paragraph land in §check-comment-tier, and the sentence in §check-rule-id-coverage; the merged spec reads as one document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — none declared; `check-amendment-retired-spelling` stays clean.
- [ ] **Entry moved** — `comment-tier-id-directive` to Done with `--queue done` in the build batch's merge commit, before the drain stage. Its oracle is local: the unit tests, the canon-kit fixture suite and the battery.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
