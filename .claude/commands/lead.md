Execute the template at lifecycle-kit/templates/lead.md, applying the bindings below.

## Bindings

**ruling-config** — `.claude/agents/stage-session.md` (dispatch `subagent_type:
stage-session`); its §Ruling classes holds the escalate-vs-decide roster.
Posture: **Split** (template §The lead model). Lead and every stage — scope
included — ride Opus via the agent's `model: opus` frontmatter default, so no
stage inherits the dispatcher's tier (measured leakage was the largest budget
drain) — **except `validate`**, which the lead dispatches with a `model: sonnet`
override. validate's rows are *mechanical oracle-running* (run the battery,
report), the tier-downgradeable class the template §Economics
tier-differentiation rule names; scope, `spec`, align, build, and close stay on
Opus, their generative and verificational judgment being what justifies the
tier. Re-judge every tier when the harness model roster churns.

**escalation-guard** — inert, the optional-guard default (as
`guard-kit/templates/wakeup-guard.sh`, unwired in `.claude/settings.json`). A
lead-model session wires `guard-kit/templates/escalation-guard.sh` on
SendMessage per guard-kit/SPEC.md §wakeup-guard.
