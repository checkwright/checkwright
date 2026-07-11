Execute the template at lifecycle-kit/templates/lead.md, applying the bindings below.

## Bindings

**ruling-config** — `.claude/agents/stage-session.md` (dispatch `subagent_type:
stage-session`); its §Ruling classes section states the escalate-vs-decide
roster this repo's stage sessions read.

**escalation-guard** — left inert here, the optional-guard default this repo
already keeps for `guard-kit/templates/wakeup-guard.sh` (unwired in
`.claude/settings.json`). A session running the lead model wires
`guard-kit/templates/escalation-guard.sh` on SendMessage per guard-kit/SPEC.md
§wakeup-guard.
