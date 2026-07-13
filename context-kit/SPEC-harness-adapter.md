# SPEC amendment: harness-adapter

The truthful widening of the Tier-two compatibility claim
(docs/positioning.md §The tiered compatibility claim). Today that section
names the always-loaded load convention a Claude-Code-native adaptation
surface. This amendment turns the load-convention half of that sentence into
configuration: a consumer whose harness reads `AGENTS.md` (or any other
always-loaded agent file) runs every kit mechanism by setting existing knobs —
and the claim is exercised by a shipped smoke, never merely asserted.

## What changes

**1. Agent-file knob coverage — a two-line convergence, not a framework.**
The contract: no kit mechanism resolves the always-loaded agent file by
literal; each reads its kit's knob, defaulting to `CLAUDE.md`. The full kit
sweep (verified at scope) found exactly two surviving literals; everything
else already conforms:

- `gate-sdk/checks/check-root-tiering.sh` — the root allowlist carries a
  `CLAUDE.md` literal; it becomes the new knob **`GATE_SDK_AGENT_FILE`**
  (default `CLAUDE.md`), the `GATE_SDK_QUEUE_FILE` precedent on the same
  allowlist line. gate-sdk/SPEC.md's knob roster gains the entry.
- `lifecycle-kit/checks/check-shim-restatement.sh` — the dedup-corpus probe
  hardcodes `CLAUDE.md`; it probes `$LIFECYCLE_KIT_AGENT_FILE` instead (the
  knob already exists in `lifecycle-kit/lib/stages.sh`, read by
  `check-lifecycle-registration`). No new name; lifecycle-kit/SPEC.md
  §check-shim-restatement's corpus description names the knob.
- Already conformant, listed as the pattern the two converge on:
  `LIFECYCLE_KIT_AGENT_FILE`, `CONTEXT_KIT_SURFACES`,
  `CONTEXT_KIT_BREVITY_FILE`, `DOCTRINE_KIT_AGENT_FILE`, and canon-kit's
  manifest default branch, which `CANON_KIT_MANIFEST_FILES` already overrides
  whole.

**2. The AGENTS.md consumer smoke — the word "tested" in the claim.** A smoke
under `context-kit/smoke/` builds a scratch consumer (the gate-sdk
consumer-smoke mechanics) whose agent file is `AGENTS.md` and whose config
sets the knobs above plus `CONTEXT_KIT_SURFACES=("AGENTS.md")`, then asserts:
the battery passes, `bin/always-loaded.sh` and `bin/footprint.sh` measure the
`AGENTS.md` surface, and `check-root-tiering` accepts it at root while
rejecting a stray second agent file. Registered per the kit-landing checklist
(gate-sdk/SPEC.md §Consumer smoke) and joining the README battery roster.

**3. The adapter recipe — a docs/install.md section.** "Running under an
AGENTS.md harness": the knob set, where the kit-injected blocks land, how the
stage skills run without the Claude shim grammar (the skill templates are
plain markdown executed by path; `.claude/` shims are one binding, not the
mechanism), and two honest limits stated plainly — (a) the settings pins, the
session-context hook wiring, and memory-off enforcement remain
Claude-Code-native (no cross-harness settings surface exists to port to);
(b) generated pre-commit trigger lists carry the manifests' default literals,
so a nondefault agent file means adjusting the affected `# graph:` trigger
lines and regenerating, or relying on full-battery runs.

**4. The positioning split.** docs/positioning.md §The tiered compatibility
claim re-words Tier two: the load convention moves from "adaptation surface,
not tested compatibility" to configuration exercised by the shipped smoke;
the stage-skill auto-load bindings and the settings pins remain the
Claude-Code-native residue, named as such. The **`positioning-harness-emphasis`**
debt task then foregrounds the widened claim on the lead pages.

**Ruled out** (the context-kit/SPEC.md §Index-first reading YAGNI precedent):
a CLAUDE.md↔AGENTS.md mirror emitter with a parity gate — scaffolding until a
dual-harness consumer exists, since a single-harness consumer names one file
and has no second copy to hold in parity; a per-harness settings-pin port —
no standard target; harness plugin packaging — the plugin-marketplace rung's.

## Producers and consumers

- **`GATE_SDK_AGENT_FILE`** — producer: consumer env/config (unset in this
  repo: the default is this repo's layout, so no config edit lands here);
  reader: `check-root-tiering`'s allowlist assembly at gate run. One field,
  one reader.
- **`LIFECYCLE_KIT_AGENT_FILE`** (existing) — gains a second reader:
  `check-shim-restatement`'s corpus probe, read where the corpus is built.
- **The smoke** — producer: the validate battery invocation (README §This
  repo, governed roster); consumer: the validate stage's evidence run, plus
  the positioning claim that cites it.
- **The recipe section** — producer: docs/install.md; consumer: an adopting
  operator on a non-Claude harness; its commands and links resolve under the
  same canon config as every docs page.
- **The Tier-two rewording** — consumer: the
  **`positioning-harness-emphasis`** pass and every reader of the
  compatibility claim; no new field.

## Existing sections updated

- docs/positioning.md §The tiered compatibility claim — the Tier-two split
  (change 4).
- gate-sdk/SPEC.md §check-root-tiering prose + the knob roster —
  `GATE_SDK_AGENT_FILE`.
- lifecycle-kit/SPEC.md §check-shim-restatement — corpus described via the
  knob, not the literal.
- context-kit/SPEC.md — the smoke joins the kit's test-surface prose; the
  `CONTEXT_KIT_SURFACES` entry states the agent-file-name-agnostic contract
  and cites the smoke.
- README §This repo, governed — the new smoke joins the battery roster.
- docs/install.md — the new recipe section; docs mirror regenerated for the
  touched kit SPECs (`scripts/gen-docs-mirror.sh --write`).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
