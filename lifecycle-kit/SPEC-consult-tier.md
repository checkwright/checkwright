# SPEC amendment: consult-tier

**`/consult` governs the tier of what it dispatches and asserts nothing about its own.** lifecycle-kit/templates/consult.md step 7 tells a consultation to name the tier of every dispatch, on the ground that "a consultation runs at the judgment tier". Nothing in the template has the session check that premise. The operator's direction on the entry is that a consultation must run on the top model and verify that at entry. A consultation answered on a cheaper model reads, in its landed rulings, like one answered on the model the skill was built for: a thin consultation looks like a short one.

**The skill declares its floor as a class, and the kit never spells a model.** The class is **judgment**, delegation-kit's highest tier class (delegation-kit/SPEC.md §The tier binding, which tier-model-binding adds). The consumer's binding maps it to a model. The check is delegation-kit's `--model-verdict --expect judgment` (§model-verdict, which session-model-identity-verification adds), so this unit lands after both, in their batch or a later one. The entry's `[blocked-by: session-model-identity-verification]` states the harder of the two orderings.

**Measured at authoring.**

- The template carries exactly two slots, `entry-reading` and `landing-surfaces` (`grep -o '\*<[a-z-]*:' lifecycle-kit/templates/consult.md`), and §templates/consult.md states that consumer residue stays in exactly those two.
- The only binding shim naming the template is `.claude/commands/consult.md` (`git grep -l 'lifecycle-kit/templates/consult.md'` over `.claude/`).
- A lead dispatches a consultation only while no stage session is live (lifecycle-kit/templates/lead.md §The escalation protocol), and hotfix-dispatch keeps a hotfix session and a consultation from being live together. The check runs before the consultation dispatches anything of its own. A bare `--model-verdict` in a dispatched consultation therefore reads the consultation's own transcript, the newest under its lead's `subagents/`. A top-level consultation reads its own by the harness's session id.

**A slot was weighed and refused.** A `tier-floor` slot would let a consumer pick the floor. It would also add a third residue slot where §templates/consult.md rules there are two, and make `check-skill-binding` red every adopter's existing consult shim on upgrade until it binds the new slot. The consumer already chooses everything that varies: which model the judgment class is. Binding judgment to a cheaper model is how a consumer runs consultations there, and it does so in its binding rather than in a skill that would then state a floor it does not hold.

## What changes

### (1) The consult template verifies its tier first

lifecycle-kit/templates/consult.md gains a first-step paragraph between its opening paragraphs and `## Session ritual` {mechanical}. **Not yet applied.**

> **First step — verify your tier.** A consultation runs on the judgment class. Run `--model-verdict --expect judgment` on the gate binary `GATE_SDK_NATIVE_BIN` names (delegation-kit/SPEC.md §model-verdict; without delegation-kit, skip this). On exit 1, stop before reading in. Say that this session is below its floor, quoting the verdict line, so it is restarted on the judgment class: to the operator, or, dispatched, to the lead in one escalation block. On exit 2 the tier is unverified and not refused: state the verdict line in your first message and proceed.

### (2) §templates/consult.md owns the floor's grounds

lifecycle-kit/SPEC.md §templates/consult.md gains a paragraph after its tier-selection paragraph ("Dispatch safety is inherited by citation…") {mechanical}. **Not yet applied.**

> **The template declares its own floor and checks it at entry.** A consultation's rulings are the tree's most expensive answers to get wrong. A consultation run on a cheaper model leaves rulings indistinguishable from one run on the model the skill was built for, since a thin consultation reads as a short one. So the template names the floor as delegation-kit's **judgment** class and never as a model: the consumer's tier binding owns the model (delegation-kit/SPEC.md §The tier binding), and the kit ships no model name (gate-sdk/SPEC.md §The provenance seam). It is a class rather than a slot because the class is kit vocabulary and the model is the consumer's to bind. A slot would be a third residue beside the two below, redding every adopter's shim for a choice the binding already offers. An unverifiable tier proceeds and says so, on `usage-verdict`'s fail-soft rule: no reading is not a wrong reading. A lead dispatches a consultation only while no stage or hotfix session is live, and no hotfix beside it (§templates/lead.md). The check runs first, before the consultation dispatches anything. Those two facts make the bare verdict's pick the consultation's own transcript.

### (3) The lead dispatches a consultation on the judgment class

lifecycle-kit/templates/lead.md §The escalation protocol, the consult paragraph: "dispatch the consult skill as the agent type the slot names" becomes "dispatch the consult skill as the agent type the slot names, on the judgment class" {mechanical}. **Not yet applied.** This repository's `consult-session` definition declares `tier: judgment` through tier-model-binding delta 6, so no dispatch here names a `model`.

## Producers and consumers

- **The first-step check.** Producer: every consultation, operator-started or lead-dispatched, at its first step.
- **Consumers.**
  - The operator, reading an exit-1 stop in a top-level session.
  - The lead, reading an exit-1 escalation from a dispatched one and re-dispatching on the judgment class. That is the escalate-on-discovery re-dispatch shape (lifecycle-kit/templates/lead.md §Economics), since a resume keeps the paused session's tier.
  - The session's own first message, carrying an exit-2 line.
- **The escalation's fields.** It carries the verdict line as its Evidence. The lead's answer is a re-dispatch, so no field goes unread.
- **Enabling config.** The judgment class must be bound, or the check reads `UNKNOWN` and proceeds. This repository binds it in tier-model-binding delta 6.

## Existing sections updated

- `lifecycle-kit/templates/consult.md`: the first step (delta 1). tier-model-binding's tier-reading edit changes step 7 of the same file.
- `lifecycle-kit/SPEC.md` §templates/consult.md (delta 2).
- `lifecycle-kit/templates/lead.md` §The escalation protocol (delta 3). tier-model-binding's tier-reading edit and hotfix-dispatch change other sections of the same file.
- `docs/lifecycle-kit/SPEC.md`: the generated mirror (delta 2).
- `.workflow/surface-ceiling.txt` — the grown `lifecycle-kit/SPEC.md`, `lifecycle-kit/templates/consult.md` and `lifecycle-kit/templates/lead.md` rows re-stamped with `bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling`, which `check-surface-ratchet` demands with the growth (all deltas).

The roster came from `git grep -l 'templates/consult.md'` over the tracked tree outside `docs/posts`, for the template's readers and shims.

## Retired spellings

- None — the amendment adds a step and renames nothing.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the first-step check and its two exits.
- [ ] **Instruction surfaces: instruction only** — the template's first step carries the command, the two acts and the skip condition; the grounds sit in §templates/consult.md.
- [ ] **Merged with no information lost** — §templates/consult.md reads as one section, the refused slot landing as its ground.
- [ ] **Amendment deleted** — this file removed on merge; `ls lifecycle-kit/SPEC-consult-tier.md` finds nothing.
- [ ] **Removals propagated** — nothing retired.
- [ ] **Gaps filed** — a cross-component gap found during the work filed with `--emit file-gap`.
- [ ] **The entry moves** — `consult-tier-declaration` moves to Done in the landing commit, a stage before the drain stage.
