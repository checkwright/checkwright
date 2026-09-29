# SPEC amendment: gate-customer-value

Nothing asks whether a shipped gate serves an adopter. The provenance seam keeps project content out of kits, and the install disposition decides whether `init` registers a gate, but neither decides whether a gate belongs in the payload at all. This amendment states that rule in gate-sdk and runs the audit's first slice: site-kit's five gates, audited and fixed.

One queue entry pairs it: [gate-customer-value-audit](TASK-QUEUE.md#gate-customer-value-audit). Its deliverable is a corpus, every shipped gate (123 descriptors: `git ls-files '*/checks/*.gate'` outside `gate-tests/`; this repo's own consumer-declared members sit under `scripts/` and are not shipped). This slice covers 5 of them, so the entry's terminal move is a demotion carrying the audit's progress (the Definition of Done).

**The rulings.**

- **Two questions, three outcomes.** A shipped gate is *valued* when its subject is a surface an adopter holds, not one only the publishing repository has. It is *configurable* when every consumer value it reads is a knob, the rule gate-sdk/SPEC.md §The port-candidate criteria already states. Valued and configurable: kept. Valued, held by a literal: made generic, the literal becoming a knob whose default is the convention's value. Not valued: withheld. Its descriptor moves to the publisher's own gates dir as a consumer-declared member and its rule stays in the crate, so it leaves the payload, `init`'s rosters and the install-smoke legs with no new mechanism.
- **The verdict is judgement, so no gate holds it.** Whose surface a subject is cannot be decided mechanically. The rule is stated where a gate author reads what the payload carries, and the audit applies it kit by kit. The configurability half already has an owner rule, and its mechanical slice is the deferred `consumer-value-literal-gate`'s, not this one's.
- **The record is the verdict table below plus the entry's progress line.** A kept gate's state is its presence in the payload, its knobs are derivable (`--emit knob-roster`), and a finding is fixed in the same slice. So a durable per-gate table would restate what the tree already says. What outlives this file is which kits have been audited, and that rides the demoted entry.
- **site-kit's audience holds every one of its subjects.** Its SPEC opens on *a docs site served from the repository (GitHub Pages and equivalents)*, an adopter class of its own, and the kit ships in the `full` profile only (`installer/profiles.list`). No gate is withheld. Two literals are found and made generic.
- **The seam.** Kit mechanism: the rule, the new knob, the resolved message. Consumer config: the knob's value, where this repo keeps the default. No private rule content.

**The audit record — site-kit** (probe: each gate's `.gate` descriptor, its module under `native/src/gates/docs_*.rs` grepped for `const` and string literals, site-kit/SPEC.md §Knob defaults, and `installer/profiles.list`):

| Gate | Subject | Consumer values read | Literal found | Verdict |
|---|---|---|---|---|
| `check-docs-cname-parity` | the host a repo-served site declares in its CNAME file | `SITE_KIT_CNAME`, `SITE_KIT_ALIASES`, `SITE_KIT_SCAN_ROOT`, `SITE_KIT_EXEMPT_PATHS` | the finding and `help:` lines name `docs/CNAME` whatever `SITE_KIT_CNAME` holds; the bad fixture's CNAME is `tree/CNAME` | kept; made generic (delta 3) |
| `check-docs-render-fidelity` | pages rendered by the Pages parser | `SITE_KIT_DOCS_DIR`, `SITE_KIT_RENDERER`, `SITE_KIT_RENDERER_BATCH` | the underscore-segment exclusion is Jekyll's own publish rule, intrinsic to the audience | kept |
| `check-docs-liquid-parse` | files a Jekyll build runs Liquid over | `SITE_KIT_DOCS_DIR`, `SITE_KIT_LIQUID_PARSER` | `TEMPLATE_DIRS` fixes `_layouts`, `_includes`, `_posts`, which a Jekyll site may rename in its config | kept; made generic (delta 2) |
| `check-docs-collapsible` | `<details>` regions under kramdown | `SITE_KIT_DOCS_DIR` | `markdown="1"` is kramdown's own syntax, intrinsic | kept |
| `check-docs-highlight-coverage` | a layout's highlight overrides against its theme | `SITE_KIT_HIGHLIGHT_TOKENS`, `SITE_KIT_HIGHLIGHT_OVERRIDES`, `SITE_KIT_HIGHLIGHT_SCOPE` | none | kept |

No verdict moves a customer-OS leg. `init` registers only `check-docs-highlight-coverage`, at the `full` profile and disarmed. The other four reach Windows and macOS runners only through the crate's own test legs, which test the binary every adopter receives.

**Refused.**

- **A per-gate declaration line** (a `# value:`-style descriptor field). Once the audit completes it would read the same on every shipped gate, a tag carrying no information its presence does not (canon-kit/SPEC.md §The amendment lifecycle's test for a state tag).
- **Withholding site-kit wholesale.** Its subjects are an adopter class's, and a kit in `full` alone already costs no other profile anything.
- **A template-dir knob relative to the docs dir.** The descriptor's `couples=` cannot root a list knob's members under another knob, so the gate's trigger would drift from its corpus. The knob takes repo-relative pathspecs, as `SITE_KIT_HIGHLIGHT_OVERRIDES` does.

## What changes

### (1) What earns a gate its payload place {design-bearing}

**Not yet applied.** gate-sdk/SPEC.md §Consumer payload gains, after its opening paragraph:

> **A kit gate ships because it serves an adopter, configurably.** Its subject is a surface an adopter holds, not one only the publishing repository has, and every consumer value it reads is a knob whose default is its convention's value (§The port-candidate criteria). A gate failing the first is withheld: its descriptor moves to the publisher's gates dir as a consumer-declared member and its rule stays in the crate, so it leaves the payload, `init`'s rosters and the install smoke. A gate failing only the second is made generic. Whose surface a subject is stays judgement, which no gate holds. It is asked of a new kit gate when it is authored, and of the shipped set by audit.

### (2) The Liquid template set is a knob {design-bearing}

**Not yet applied.** site-kit/SPEC.md §Knob defaults gains, after `SITE_KIT_LIQUID_PARSER`:

> - `SITE_KIT_LIQUID_TEMPLATES` — array of repo-relative pathspecs naming the tracked files `check-docs-liquid-parse` reads as Liquid templates, default `("docs/_layouts/*" "docs/_includes/*" "docs/_posts/*")`, Jekyll's three template directories under the default docs dir. A site that renames them in its Jekyll config, or moves its docs dir, sets this beside `SITE_KIT_DOCS_DIR`.

site-kit/SPEC.md §check-docs-liquid-parse: the corpus bullet opening *every tracked file under the docs dir's own `_layouts/`, `_includes/` and `_posts/`* becomes *every tracked file `SITE_KIT_LIQUID_TEMPLATES` names, Jekyll's template directories at the default. Every underscore directory it does not name stays excluded, as render-fidelity excludes it.* The **The `# graph:` couples** sentence becomes *render-fidelity's plus `knob:SITE_KIT_LIQUID_TEMPLATES`.*

`native/src/gates/docs_liquid_parse.rs` drops `TEMPLATE_DIRS`. `classify` reads a file as a template when one of the knob's pathspecs selects it, resolved as git resolves a pathspec (`git ls-files -- <pathspec>`), the way `check-docs-highlight-coverage` resolves `SITE_KIT_HIGHLIGHT_OVERRIDES`. The corpus stays the docs dir's tracked files, as it is today, and `classify` is handed the repository-relative path the pathspecs are written against rather than the path relative to the docs dir, so a pathspec selecting a file outside the docs dir selects nothing. The module's `KNOBS` declaration gains the row. `native/src/knobs/site_kit.rs` gains the default. `site-kit/checks/check-docs-liquid-parse.gate`'s `couples=` replaces its three `knob:SITE_KIT_DOCS_DIR/_layouts/*`, `/_includes/*` and `/_posts/*` tokens with `knob:SITE_KIT_LIQUID_TEMPLATES`, and its `# spec:` line names the knob in place of the three directories. The fixture pair gains a `good/` case whose knob file renames the layouts directory and whose renamed template parses, and a `bad/` case where a broken template under the renamed directory reds. The unit tests of `classify` take the default set and a renamed one.

### (3) The CNAME messages name the configured file {mechanical}

**Not yet applied.** `native/src/gates/docs_cname_parity.rs`' finding header and `help:` line print the resolved `SITE_KIT_CNAME` path where they print `docs/CNAME`. Its module comment and `site-kit/checks/check-docs-cname-parity.gate`'s `# spec:` line say *the host in the CNAME file `SITE_KIT_CNAME` names*. site-kit/SPEC.md §check-docs-cname-parity already states the invariant in the knob's terms and is unchanged.

## Producers and consumers

Probes: `git grep -n "TEMPLATE_DIRS\|docs/CNAME"` over `native/src` and `site-kit/`; `git grep -n SITE_KIT_LIQUID` over the tree; site-kit's gate-test directories listed; the descriptor `couples=` read.

- **The payload-place rule** (delta 1). Producer: the session authoring a kit gate, and the audit's slices. Consumer: that session's judgement and the audit's verdict table. It adds no gate, so no roster reads it.
- **`SITE_KIT_LIQUID_TEMPLATES`** (delta 2). Producer: site-kit's defaults table, set in every deployed configuration, since this repo keeps the default. Readers: `check-docs-liquid-parse`'s `classify`. Roster-holding readers: site-kit's static table and `--emit knob-roster`; the module's `KNOBS` declaration, read by the knob-file derivation, `check-reads-couples` and `check-gate-substrate-parity`; `check-knob-citation`, satisfied by the §Knob defaults bullet; `check-graph`'s admissibility loop, which admits the `knob:` token only because the member declares the knob. `--emit port-blockers` is untouched, because the knob names files, not a program.
- **The corpus under the default** (delta 2). The pathspecs name the three directories the literal named, so the default corpus is unchanged. **Red conditions:** the fixture pair's `good/` case asserts a clean verdict with a count, so a knob that stopped selecting reds it.
- **The CNAME message** (delta 3). Consumer: the session reading a red. The `bad/` case's `expect.txt` pins the alias line alone, not the header, so no expectation moves.

## Existing sections updated

Roster probe: the probes above, plus `git grep -n "_layouts\|_includes\|_posts" -- site-kit/ native/src/gates/docs_liquid_parse.rs`.

- `gate-sdk/SPEC.md` — §Consumer payload (delta 1).
- `site-kit/SPEC.md` — §Knob defaults, §check-docs-liquid-parse (delta 2).
- `native/src/gates/docs_liquid_parse.rs` (delta 2).
- `native/src/knobs/site_kit.rs` (delta 2).
- `site-kit/checks/check-docs-liquid-parse.gate` (delta 2).
- `site-kit/gate-tests/check-docs-liquid-parse/` — the new fixture cases (delta 2).
- `native/src/gates/docs_cname_parity.rs` (delta 3).
- `site-kit/checks/check-docs-cname-parity.gate` (delta 3).
- `docs/gate-sdk/SPEC.md` — the generated on-site mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (delta 1).
- `docs/site-kit/SPEC.md` — mirror (delta 2).
- `docs/check-graph.html` — the graph artifact draws the liquid gate's new couple, regenerated with `bash gate-sdk/bin/run-gates.sh --emit graph > docs/check-graph.html` (delta 2).
- `.workflow/release-declarations.md` — Behavior changes gains a bullet led by **site-kit/SPEC.md §check-docs-liquid-parse**: the Liquid template set is the new knob `SITE_KIT_LIQUID_TEMPLATES`, defaulting to the three directories it read before, so a site with renamed template directories sets it (delta 2).
- `TASK-QUEUE.md` — the entry demotes carrying the progress line (delta 1).

## Retired spellings

- `TEMPLATE_DIRS` — the crate literal the knob replaces (delta 2).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery green** — `bash gate-sdk/bin/build-native.sh`, the full battery, and the site-kit and gate-sdk fixture suites green on the landing commit.
- [ ] **Demoted, not Done** — the entry is a corpus and this slice is one increment, so it leaves by `--queue demote gate-customer-value-audit` before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), dropping its `[spec:]` tag. In the same commit its body gains one progress line: *Audited: site-kit, 5 gates (5 kept, 2 made generic, 0 withheld); the other 118 shipped gates remain, kit by kit, under gate-sdk/SPEC.md §Consumer payload's rule.* The entry is compressed there if the queue's size cap binds.
