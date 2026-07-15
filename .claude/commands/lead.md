Execute the template at lifecycle-kit/templates/lead.md, applying the bindings below.

## Bindings

**ruling-config** — `.claude/agents/stage-session.md` (dispatch `subagent_type:
stage-session`); its §Ruling classes section states the escalate-vs-decide
roster this repo's stage sessions read. The standing dispatch-model choice
lives here too — the generic template stays roster-agnostic: this repo runs
the split posture. The lead session rides Opus; the agent definition's
`model: opus` frontmatter is the stage default, so no stage inherits the
dispatcher's tier by accident (measured leakage was the single largest budget
drain). Only the scope dispatch overrides with `model: fable` — dispatched
fresh, not forked — and that agent remains resumable as the intent oracle.
Re-judge this line when the harness model roster churns.

**escalation-guard** — left inert here, the optional-guard default this repo
already keeps for `guard-kit/templates/wakeup-guard.sh` (unwired in
`.claude/settings.json`). A session running the lead model wires
`guard-kit/templates/escalation-guard.sh` on SendMessage per guard-kit/SPEC.md
§wakeup-guard.
