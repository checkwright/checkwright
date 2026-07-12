---
title: Where Checkwright sits
nav_order: 5
---

# Where Checkwright sits

A coding agent runs inside a stack of layers, and where a tool sits in that
stack decides what it can and cannot promise. This page places Checkwright in
that stack — the verification layer under agent orchestration, subordinate to
the harness that runs the model — and states honestly which harnesses it runs
under and which it merely adapts to. It owns no contract; each invariant below
is owned by the kit that enforces it, cited downward so it stays in one place.

## The layer model

Read the stack from the model up:

1. **The model** — the language model that generates the work, token by token.
2. **The harness** — the agent runtime that loops the model over tools: read a
   file, write one, run a command.
3. **The harness prompt** — the closed system instructions the harness ships.
   The model cannot see around them, and project-level config loads *under*
   them: it can add, but it cannot override.
4. **Checkwright** — harness-loaded project content (CLAUDE.md, the stage
   skills) *plus* gates that run outside the model entirely. It shapes and
   audits the behavior the lower layers produce.

Checkwright is layer-4 content, and that subordination is the whole design
constraint. A layer-4 instruction is a request the harness may honor; it is not
a guarantee. The worked ceiling: upstream Claude Code issue #75214, where
project-level config cannot lift the harness's Task ask-first default — an
instruction no layer-4 artifact can override, because the default lives in the
closed layer-3 prompt. So an instruction is exactly the wrong place to put
anything that must hold. That is why the enforcement half of the methodology
lives in gates — programs that run outside the model, that a harness cannot
re-interpret or decline. The instructions shape; the gates enforce. This keeps
the "verification layer under agent orchestration" framing that
[Agent orchestration](orchestration.md) establishes: Checkwright sits beneath
the coordinating layer and claims no security or trust-layer ground.

## The tiered compatibility claim

Compatibility here is tiered, not blanket — the honest split matters more than a
broad claim that would not survive contact:

- **Tier one — runs anywhere.** The gate battery is bare bash over a coreutils
  toolchain. No gate reads a harness surface, so the battery runs under any
  harness, under any CI, or under no harness at all. This is the layer that
  does the enforcing, and it has no harness dependency to compromise.
- **Tier two — Claude-Code-native, adapter-shaped elsewhere.** The lifecycle
  stage skills, the CLAUDE.md load convention, and the settings pins are built
  for Claude Code. For other rules-file conventions — AGENTS.md, `.cursorrules`,
  and kin — they are an adaptation surface, not tested compatibility: the load
  triggers and the settings mechanism would have to be ported. The page names
  this as work to do, and claims no compatibility that does not exist today.

The division is deliberate. The part that must be portable — enforcement — is;
the part that is convention rides whichever harness convention you already run,
once adapted.

## The memory-off position

Durable guidance belongs in tracked manifests — the CLAUDE.md, the kit SPECs,
committed briefs — that the whole team and every fresh session read identically
from the tree. It does not belong in per-user harness memory, which forks
silently: one operator's remembered fact is invisible to the next session and to
every teammate, so the guidance an agent acts on stops being auditable from the
repository alone. Checkwright therefore keeps harness auto-memory off and holds
it off with a gate, generalized across whichever memory convention a harness
offers rather than tied to one product's feature. The position is stated here;
the invariant is owned by `context-kit/SPEC.md §The memory-off doctrine` and
enforced by `check-memory-off`.

## The design-philosophy contrast

Approaches that make a coding agent follow a process tend to cluster into
recognizable stances, and it is worth naming how Checkwright's design differs
from each in kind. The contrast is drawn at the level of stance, not feature: a
scored capability table would be a maintained artifact that rots the moment
either side changes — the maintain-the-derivable trap the methodology exists to
reject — so what follows compares design commitments, not checkboxes.

**Against the rules-file stance.** The common approach writes the process down
as a conventions file the agent is asked to follow at load time. Checkwright
loads such a file too, but treats it as the *shaping* half only — an instruction
is a layer-4 request the harness may honor, never the guarantee the worked
ceiling above shows it cannot be. The enforcing half lives in gates that run
outside the model. This is where the provenance seam pays off: because the
mechanism is a gate and the private rule content it polices stays consumer
config, the mechanism packages and ships as a kit without publishing anyone's
rules — where a single conventions file fuses process and private content into
one artifact you can neither share without leaking nor keep without
re-authoring.

**Against the harness-memory stance.** The other approach lets the agent
accumulate learned process in per-user memory that carries across sessions.
Checkwright holds that memory off (the position above) and puts the process in a
state machine instead: which stage a task sits in, and the evidence advancing
it, are tracked files that every session and every teammate reads identically
from the tree. Remembered process forks silently and answers only to the
operator who taught it; a state machine over the tree answers to the repository.
The machine is owned by `lifecycle-kit/SPEC.md §The state machine`.

Both other stances put what governs the work where only the model, or only one
operator, can see it. Checkwright's wager is the opposite: enforcement outside
the model, state on the tree, so the process that produced any commit is
auditable from the repository alone.

## Where to go next

- [Why Checkwright](methodology.md) — the delivery-methodology essay behind the
  mechanism.
- [The kits](index.md#the-kits) — one page per kit, in reading order.
