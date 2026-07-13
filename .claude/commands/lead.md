Execute the template at lifecycle-kit/templates/lead.md, applying the bindings below.

## Bindings

**ruling-config** — `.claude/agents/stage-session.md` (dispatch `subagent_type:
stage-session`); its §Ruling classes section states the escalate-vs-decide
roster this repo's stage sessions read. The standing dispatch-model choice
lives here too — the generic template stays roster-agnostic: a build session
rides Opus, executing design the earlier stages settled; a design-ruling stage
dispatch inherits the lead's own class. Re-judge this line when the harness
model roster churns.

**escalation-guard** — left inert here, the optional-guard default this repo
already keeps for `guard-kit/templates/wakeup-guard.sh` (unwired in
`.claude/settings.json`). A session running the lead model wires
`guard-kit/templates/escalation-guard.sh` on SendMessage per guard-kit/SPEC.md
§wakeup-guard.
