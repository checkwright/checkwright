# SPEC amendment: hotfix-dispatch

**No tracked agent definition exists for the operator-ruled hotfix path of Scope-gated intake (doctrine-kit/DOCTRINE.md, rule 11).** A lead dispatching one picks the general-purpose type and restates the standing policy in the prompt. That is the policy-is-config tell (lifecycle-kit/templates/lead.md §Policy is config, not prose): the same sentences appearing in two dispatch prompts.

**The undeclared type is also confined.** `agent-dispatch-guard` D4 sends an undeclared type into a worktree (delegation-kit/SPEC.md §The delegation model). There a crate-source commit is refused (gate-sdk/SPEC.md §check-crate-arms) and `--emit file-gap` refuses (§The committed gap inbox). So a hotfix touching the crate needed a second dispatch to land.

**The fix is policy as config: a hotfix type, declared mutating, that the lead template names.**

- **A definition** carries the policy once.
- **The mutating roster** lets the type commit on the shared tree.
- **The lead template** states when the dispatch is made and what its prompt carries.

**Measured at authoring.**

- `ls .claude/agents/` lists four definitions and no hotfix type.
- `scripts/delegation-config.knobs` declares `stage-session`, `edit-sweep` and `consult-session` mutating.
- lifecycle-kit/templates/lead.md carries four slots (`grep -o '\*<[a-z-]*:'`): `open-authorization-channel`, `consult-dispatch`, `ruling-config` and `escalation-guard`. The one shim binding it is `.claude/commands/lead.md`.
- The queue has no delete verb (`bash gate-sdk/bin/run-gates.sh --queue` lists promote, done, clear-done, icebox, recur, demote, thaw and split). So the doctrine's "the queue entry is deleted in that commit" is a hand edit of the queue file, held by the queue gates at commit.

**A new slot was weighed and refused.** A `hotfix-dispatch` slot beside `consult-dispatch` would make `check-skill-binding` red every adopter's lead shim on upgrade until it binds the slot, which pairs slots by name. The `ruling-config` slot already names "the tracked agent-definition the lead dispatches". Widening its description to the hotfix type changes no slot name, so no existing binding reds. A consumer that names no hotfix type dispatches none, and the template states what happens then.

**A parallel hotfix track stays refused** (lifecycle-kit/SPEC.md §Deviation transitions). This is one ruled fix per dispatch, never a standing lane. It is dispatched only while no stage session is live, so it contends on no live stage surface. That is the condition a consultation dispatch already takes, for the same reason.

## What changes

### (1) The hotfix type's definition

`.claude/agents/hotfix-session.md` is created, carrying the policy the prompts restated {mechanical}. **Not yet applied.** Its frontmatter depends on tier-model-binding's landing.

- **If tier-model-binding delta 6 lands in the same batch or earlier,** the frontmatter carries `tier: judgment` and the `model:` line `--emit agent-tiers --write` generates.
- **If this delta lands first,** it carries `model: opus` alone, and tier-model-binding delta 6's roster, re-derived by listing `.claude/agents/`, adds its `tier:`.

The frontmatter, in the first case:

```yaml
---
name: hotfix-session
description: An operator-ruled hotfix a live iteration lead dispatches (doctrine-kit/DOCTRINE.md, Scope-gated intake) — one minimal, test-and-doc-complete fix of an impacting failure, landed in one commit on the shared tree. It is not a stage session. Use this type only when the operator has ruled a hotfix; any other fix is filed and enters through scope.
tier: judgment
model: opus
---
```

The body:

> You are an operator-ruled hotfix dispatched by a live iteration lead. Your prompt names the ruling, the failure, and the queue entry or gap bullet the fix disposes. Fix that failure and nothing else (doctrine-kit/DOCTRINE.md, rule 11).
>
> ## Not a stage
>
> Run no `--enter-stage`, write no stamp, and make no queue move. A hotfix belongs to no iteration. Dispatch no stage, consultation or hotfix session; a read-only fan-out stays sanctioned.
>
> ## The one commit
>
> - The fix, its test and its owner-doc correction land in one commit. Minimal is measured against the failure modes your change would create, never against the smallest diff.
> - Where your prompt names a queue entry, delete its section in that commit, never moving it to the done section. Where it names a gap-inbox bullet, remove the bullet in that commit. The message names the ruling and the slug.
> - Before committing, run the full battery (`bash gate-sdk/bin/run-gates.sh --run`), the fixture suite of every kit your edit reaches, and `bash gate-sdk/bin/build-native.sh` when it touches `native/`. Commit with the only-paths form (CLAUDE.md §This repo is governed by its own kits).
> - A gate in your way means the fix does not fit the convention: escalate it, never weaken the gate.
>
> ## Stop on a design question
>
> Anything your prompt's ruling does not settle is not yours: a contract change, a new governed name, a second failure. Journal it, author none of it, and escalate to the lead (`to: "main"`), one Question / Options / Recommendation / Evidence block per question, all in one turn end. A finding outside the fix goes to `--emit file-gap`.
>
> ## Journal and return
>
> Journal to the path your dispatch grants, per delegation-kit/templates/agent-execution.md's **Resume journal — agent writes, scratch reset sweeps** and **Findings you will act on are durable before you act on them** bullets, and append `DONE` as its last line. Your final message is the contract: the commit hash, the gates and suites you ran with their verdicts, and what you filed. Do not end your turn with work still in flight, and do not end it to wait; the **Background + notification, never poll** bullet on the same surface owns how to wait in-turn.

### (2) The type is declared mutating

`scripts/delegation-config.knobs` gains `DELEGATION_KIT_MUTATING_TYPES[] = hotfix-session` after `consult-session`'s line {mechanical}. **Not yet applied.** D4 then admits the type unisolated. D5 passes it on its definition's `model:`. `scripts/lifecycle-config.knobs`' `LIFECYCLE_KIT_STAGE_SESSION_TYPES` is left naming `stage-session` alone, since a hotfix stamps nothing.

### (3) The lead template names the dispatch

lifecycle-kit/templates/lead.md gains a paragraph in §The escalation protocol after the consult-dispatch slot, and the `ruling-config` slot's description widens {mechanical}. **Not yet applied.**

> **Dispatch an operator-ruled hotfix only while no stage session and no consultation is live.** When the operator rules a hotfix (doctrine-kit/DOCTRINE.md, Scope-gated intake), dispatch it as the hotfix type your ruling-config names. Its prompt names the ruling, the failure, and the queue entry or gap bullet the fix disposes, and nothing else, and grants its journal path. Verify its one commit as you verify any agent commit, and dispatch no stage session and no consultation while it is live. Where your ruling-config names no hotfix type, dispatch none: the operator lands the hotfix in a session of their own.

The slot becomes:

> *<ruling-config: the tracked agent-definitions the lead dispatches and the roster each carries — for the stage-session type, and for a hotfix type where this consumer has one: its path, the subagent type the dispatch names, and where its ruling classes are stated.>*

### (4) §templates/lead.md owns the grounds

lifecycle-kit/SPEC.md §templates/lead.md re-phrases its slot roster's `ruling-config` bullet and gains a paragraph after the roster {mechanical}. **Not yet applied.** The bullet becomes:

> - `ruling-config` — the tracked agent definitions the dispatch names, the stage-session type and a hotfix type where the consumer has one, each carrying its standing dispatch policy: the ruling-class roster, the posture and the tier assignment, and everything else true of every dispatch rather than improvised per prompt;

The paragraph:

> **An operator-ruled hotfix is a dispatch, never a track.** The refused parallel hotfix track (§Deviation transitions) is a standing lane that contends on live stage surfaces. Doctrine-kit's valve is one ruled fix, and the lead dispatches it only while no stage session is live, the condition a consultation dispatch takes. A hotfix and a consultation are never live together either: a dispatched consultation checks its tier by the newest transcript under the lead (§templates/consult.md), and a live hotfix would out-date it. Its type is never one `LIFECYCLE_KIT_STAGE_SESSION_TYPES` lists, whose stamp-before-write rule refuses a session that stamps nothing (§check-dispatch-entry). Its standing policy is the hotfix type's definition, never a prompt, on §Policy is config: not a stage, one commit, the battery and every reached suite, stop on a design question. The type is declared mutating, because isolation refuses a crate-source commit (gate-sdk/SPEC.md §check-crate-arms) and a gap filing (§The committed gap inbox), both of which a hotfix may need. The type is named in the `ruling-config` slot rather than a slot of its own, since `check-skill-binding` pairs slots by name and a new one would red every adopter's shim for a type most consumers never dispatch.

### (5) This repository's lead binding names the type

`.claude/commands/lead.md`'s ruling-config binding gains, after its first sentence, "The hotfix type is `.claude/agents/hotfix-session.md` (dispatch `subagent_type: hotfix-session`); its §Stop on a design question is its escalation roster." {mechanical} **Not yet applied.**

## Producers and consumers

- **The hotfix type.**
  - Producer: the lead, dispatching on an operator's hotfix ruling while no stage session and no consultation is live.
  - Consumers:
    - The dispatched session, which reads its definition.
    - `agent-dispatch-guard`, D4 through the mutating roster and D5 through `model:`.
    - `check-agent-tier-explicit`, assertion A and, under the binding, assertion B.
    - `--emit agent-tiers`, once tier-model-binding lands.
- **The hotfix's commit.** Producer: the hotfix session. Consumers: the lead, which verifies it (delegation-kit/SPEC.md §Verify after every agent commit), and the pre-commit battery.
- **The hotfix's escalation.** Producer: the hotfix session on a design question. Consumer: the lead, which relays it to the operator, since a hotfix's ruling is the operator's.
- **The widened slot.**
  - Its reader is `check-skill-binding`, which pairs by name and is unaffected.
  - This repository's binding names the type (delta 5).
  - An adopter's binding that names none is the template's "dispatch none".

## Existing sections updated

- `.claude/agents/hotfix-session.md`, new (delta 1).
- `scripts/delegation-config.knobs` (delta 2).
- `lifecycle-kit/templates/lead.md`: §The escalation protocol and the `ruling-config` slot (delta 3). tier-model-binding's tier-reading edit and consult-tier's consult-dispatch edit change other passages of the same file.
- `lifecycle-kit/SPEC.md` §templates/lead.md, the slot roster's `ruling-config` bullet and the new paragraph (delta 4).
- `.claude/commands/lead.md` (delta 5). tier-model-binding's repository binding and model-verdict's prompt-line edit change the same binding's tiering text.
- `docs/lifecycle-kit/SPEC.md`: the generated mirror (delta 4).
- `.workflow/surface-ceiling.txt` — the new `.claude/agents/hotfix-session.md` row and the grown `lifecycle-kit/templates/lead.md`, `lifecycle-kit/SPEC.md` and `.claude/commands/lead.md` rows stamped with `bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling`, which `check-surface-ratchet` demands with the growth (deltas 1, 3, 4 and 5).

The roster came from `git grep -n -i 'hotfix'` over the tracked tree outside `docs/posts` and the queue, and from `git grep -n 'MUTATING_TYPES'` for the roster's readers.

## Retired spellings

- None — the slot keeps its name, and nothing is renamed.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the type, its commit, its escalation and the widened slot.
- [ ] **Instruction surfaces: instruction only** — the definition and the template paragraph carry no grounds; they sit in §templates/lead.md.
- [ ] **Merged with no information lost** — §templates/lead.md reads the hotfix dispatch beside the consult dispatch as one rule for when a lead dispatches a non-stage session.
- [ ] **Amendment deleted** — this file removed on merge; `ls lifecycle-kit/SPEC-hotfix-dispatch.md` finds nothing.
- [ ] **Removals propagated** — nothing retired.
- [ ] **Gaps filed** — a cross-component gap found during the work filed with `--emit file-gap`.
- [ ] **The entry moves** — `hotfix-agent-definition` moves to Done in the landing commit, a stage before the drain stage.
