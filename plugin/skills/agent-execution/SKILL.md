---
name: agent-execution
description: Runs Checkwright's protocol for executing a delegated agent, covering its safety rules, its resume journal and the checks after each agent commit. Run it only when the user asks for it.
---

Execute the template at `delegation-kit/templates/agent-execution.md` in this repository.

- If this repository has a skill of its own that binds `delegation-kit/templates/agent-execution.md`, run that skill instead.
- Otherwise, take each `*<slot: …>*` placeholder's own text as its binding.
- If `delegation-kit/templates/agent-execution.md` does not exist, Checkwright is not installed here, or not with the kit that ships this template. Say so, and stop: the `install` skill installs it.
