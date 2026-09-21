# SPEC amendment: doc-path-tokens

The discriminator this amendment applies is the entry's: **a gate may know a
document's shape and never its path, because a path is always config.** Knob
indirection in `couples=` has since landed (`knob:<NAME>`), so no descriptor is
forced to freeze a consumer path. What remains is a corpus of frozen literals, each
a per-gate choice. **This amendment runs the survey the entry asks for. It rules
each class of literal and adds one grammar form to the token set. A
directory-valued knob then roots a path, and no path class is left with a frozen
spelling for want of a token.** The format change the entry left open is owed, and
it is exactly that one form.

**The survey (2026-09-21).** The corpus is every tracked kit-shipped descriptor,
`git ls-files '*.gate'` less `gate-tests/` and less the consumer's own `scripts/`.
For each descriptor, the survey took every `couples=` and `trigger=` token that is
neither `kit:`, `knob:`, a crate path, a leading-`*` glob nor a path inside the
descriptor's own kit. The census, by occurrence:

| Literal | Occurrences | Class |
|---|---|---|
| `TASK-QUEUE.md` | 24 | file knob |
| `scripts/*.sh`, `scripts/check-*.sh`, `scripts/kpi-*.sh`, `scripts/*.gate`, `scripts/*-config.knobs` | 21, 2, 1, 1, 1 | gates-dir rooted |
| `scripts/gates.list` | 9 | gates-dir rooted |
| `.workflow/*` | 6 | workflow-dir rooted |
| `CLAUDE.md` | 6 | file knob |
| `SPEC-*.md`, `SPEC.md` | 5, 1 | file knob (amendment glob, spec name) |
| `docs/*.md`, `docs/*/index.md`, `docs/posts/*.md`, `docs/_config.yml`, `docs/CNAME` | 5, 2, 2, 4, 1 | docs-dir rooted, or file knob where one exists |
| `.claude/commands/*.md`, `.claude/agents/*.md`, `.claude/settings.json` | 5, 2, 3 | dir or file knob |
| `.workflow/WORKFLOW-STATE.txt` and the other `.workflow/` files | 5, 6 | file knob |
| `.github/workflows/*.yml`, `*.yaml`, `.github/ISSUE_TEMPLATE/*.yml` | 5, 4, 4 | platform-fixed |
| `.gitignore`, `.gitattributes`, `SECURITY.md` | 3, 1, 1 | platform-fixed |
| `scripts/git-hooks/*`, `scripts/graph-theme/*`, `scripts/gate-tests/*` and the other named `scripts/` files | 1 each | dir or file knob |
| `README.md`, `ROADMAP.md`, `GLOSSARY.md`, `VISION.md`, `docs/enforcement.md`, `docs/footprint.md`, `docs/check-graph.html` | 2, 2, 1, 1, 1, 1, 1 | file knob |
| `.tmp/*.run`, `.tmp/run-validate.lock` | 1, 1 | tmp-dir rooted, file knob |
| `lifecycle-kit/templates/stages/*.md`, `delegation-kit/templates/agent-execution.md` | 2, 1 | kit-own, stays |

The survey re-runs the entry's count and supersedes it: the corpus has grown since
the filing, and `scripts/gates.list` alone rose from 5 to 9. Every `scripts/`-hosted
descriptor is outside the corpus, since a consumer's own descriptor naming the
consumer's own document is that consumer's configuration. There are 17 such
descriptors (`git ls-files 'scripts/*.gate'`).

**Why a rooted form, not a knob per path.** Most of the non-platform literals sit
under a directory some knob already configures. That directory is the gates dir
(`GATE_SDK_GATES_DIR`, which defaults to `scripts`), the workflow dir, the tmp dir,
the docs dir or the harness's agent and skill dirs. A row per path would mint many
knobs for one fact already configured once. The existing `knob:` token expands a
knob's *members* into covering patterns and cannot append a segment.
`knob:<NAME>/<glob>` is the smallest form that says "this glob, under whatever that
directory is". It is one pass and one member, like the existing token.

**Why `scripts/gates.list` is not kit-own.** gate-sdk owns the file's grammar, but
not its location: it lives in the consumer's configured gates dir. By the
discriminator, the name is the shape and the directory is config. So the rooted
token carries it, as `knob:GATE_SDK_GATES_DIR/gates.list`.

## What changes

**Batching.** Delta 1 lands before or with delta 3, since delta 3's tokens need the
form. This amendment's descriptor edits touch the same first lines that the
module-coupling unit of this iteration edits. The two land in either order, one
after the other.

### (1) `knob:<NAME>/<glob>` roots a glob at a directory-valued knob {design-bearing}

**Not yet applied.** It is added to §The `# graph:` manifest's special-token rules and
implemented in `registry::expand_couples`, sharing the existing token's resolution
order and one-pass bound.

- **Referent.** `<NAME>` is either a scalar row the member declares, not `.words()`,
  resolving to one path, or one of the two locators, `GATE_SDK_GATES_DIR` and
  `GATE_SDK_ROOT`. Every member resolves its gates dir, so declaring a locator
  would add a declaration no reader could fail. A packed or indexed row is refused
  in rooted position, because a root is one directory.
- **Expansion.** The knob's value, with any trailing `/` trimmed, then `/`, then
  `<glob>`. The result takes the existing token's covering-pattern conversion.
  `<glob>` is held to the literal-glob character set that
  `registry::COUPLES_PREFIXES`' validator already applies to a remainder.
- **Admissibility.** `check-graph`'s live-registry loop admits it on the same test
  as a bare `knob:` token, or on the locator exemption above. An empty value is a
  refusal at expansion, not an empty expansion, because a root that resolves to
  nothing would silently re-root the glob at the repository root.
- The emitter bakes the expansion into the generated hook as it does for `knob:`.
  So an adopter who relocates their gates dir and regenerates the hook gets
  triggers under the new dir.

The `check-graph` fixture pair gains a `good/` descriptor with a rooted token, and
`bad/` descriptors with a rooted token on an undeclared knob and on an indexed row.

### (2) §The `# graph:` manifest states the discriminator and its three literal classes {mechanical}

**Not yet applied.** After the `knob:` token's rules, add a paragraph stating the
following.

- A kit-shipped descriptor names a consumer path through a token, never as a literal.
- A literal stays in three cases. The first is a path fixed by a tool outside the
  kits: git's `.gitignore` and `.gitattributes`, GitHub's `.github/` tree and
  `SECURITY.md`. The second is a path inside a kit root, which is kit content. The
  third is any path in a descriptor the consumer owns.
- The grounds: a literal freezes one consumer's layout into every adopter's
  trigger set, and the hook then never fires on the adopter's real file.

### (3) Every kit-shipped descriptor's consumer-path literal becomes a token {mechanical}

**Not yet applied.** For each surveyed occurrence, build applies the first rule that
yields a value:

1. **File knob.** The member declares a knob whose value, under this repo's
   configuration, is exactly the literal. Replace the literal with `knob:<NAME>`.
   Examples: `GATE_SDK_QUEUE_FILE` or a kit's derived `*_QUEUE_FILE` for
   `TASK-QUEUE.md`, `CONTEXT_KIT_BREVITY_FILE` for `check-brevity`'s `CLAUDE.md`,
   `EVIDENCE_KIT_STATE_FILE` for `.workflow/WORKFLOW-STATE.txt`.
2. **Rooted.** A directory knob or locator the member reads, plus a remainder, is
   exactly the literal. Replace the literal with `knob:<NAME>/<remainder>`.
   Examples: `knob:GATE_SDK_GATES_DIR/*.sh`, `knob:GATE_SDK_WORKFLOW_DIR/*`.
3. **Kept literal.** The path is one of delta 2's three kept classes.
4. **Hardcoded.** None of the above holds. The member's code then reads a consumer
   path no knob configures, which is a code defect beneath the descriptor. The
   literal stays, and build files the member as a gap with its path.

**The oracle.** A before-and-after comparison of `--emit-git-hooks pre-commit` under
this repo's configuration: every pattern a member triggered on before must be
matched by one it triggers on after. The covering conversion only widens. A
comparison under a relocated `GATE_SDK_WORKFLOW_DIR` then shows each rooted member
moving with the knob. `check-graph`'s admissibility loop reds a token naming a knob
the member does not declare, so a mis-mapped member cannot land green.

`check-brevity`'s entry-proposed fix, `couples=CLAUDE.md,AGENTS.md`, is refused by
this rule. It freezes a second consumer path. `knob:CONTEXT_KIT_BREVITY_FILE`
follows the file an AGENTS.md consumer configures.

## Producers and consumers

- **The rooted token (delta 1).** Producer: descriptor authors, starting with delta
  3's conversions in this tree. Consumers: `registry::expand_couples`, and through
  it the hook emitter, `check-graph`'s couples-to-hook parity and the graph
  projection. Parity holds by construction, since both operands pass through one
  expansion. It adds no knob, so no knob roster changes. **Release
  declaration:** every converted kit-shipped gate changes what its hook triggers on
  in a vendored tree. That is a Behavior-changes bullet in
  `.workflow/release-declarations.md`, and its remedy is to regenerate the hooks
  after upgrading.
- **Point 5.** No corpus narrows. A trigger set can only widen, and the oracle holds
  that.
- **Point 6.** The members are the survey's occurrences, enumerated by the probe
  named above, and each one's value is the output of delta 3's rule. A member the
  rule sends to class 4 is narrowed out of this delta and filed. Its count is build's
  measurement, because it needs each member's code read.

## Existing sections updated

Rosters from the survey probe above, `git ls-files 'scripts/*.gate'`, and reading
§The `# graph:` manifest and §check-graph.

- gate-sdk/SPEC.md §The `# graph:` manifest (deltas 1 and 2) and §check-graph
  (delta 1).
- `native/src/registry.rs`, `native/src/gates/graph.rs` and
  `gate-sdk/gate-tests/check-graph/` (delta 1).
- Every kit-shipped `.gate` descriptor carrying a surveyed literal, under
  `canon-kit/checks/`, `context-kit/checks/`, `delegation-kit/checks/`,
  `doctrine-kit/checks/`, `evidence-kit/checks/`, `gate-sdk/checks/`,
  `guard-kit/checks/`, `lifecycle-kit/checks/`, `queue-kit/checks/` and
  `site-kit/checks/` (delta 3).
- `.workflow/release-declarations.md` (delta 3).
<!-- update-target-exempt: generated projections, regenerated by their freshness gates' printed commands -->
- `docs/gate-sdk/SPEC.md`, `scripts/git-hooks/pre-commit`, `docs/check-graph.html`.

## Retired spellings

- None — a literal replaced by a token is a value change on a line, not a retired
  name; the paths themselves stay live in the consumer's tree.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds for the rooted token.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** The discriminator paragraph carries the
      grounds, and the survey's class counts reach the SPEC as a rule, never as a
      maintained table.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `gates-must-not-bind-to-document-paths` moves to Done in the
      merge commit, at a stage before the drain stage, with each class-4 member
      filed.
- [ ] **Fails closed.** `bad/` reds on a rooted token naming an undeclared knob and
      on one naming an indexed row, and an empty-valued root refuses.
- [ ] **Trigger containment.** The before-and-after hook comparison is run and every
      prior pattern is matched.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
