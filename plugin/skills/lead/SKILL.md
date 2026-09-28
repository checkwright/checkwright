---
name: lead
description: Runs Checkwright's iteration lead, a live session that dispatches the stage sessions and answers their escalations. Run it only when the user asks for it.
---

Execute the template at `lifecycle-kit/templates/lead.md` in this repository.

- If this repository has a skill of its own that binds `lifecycle-kit/templates/lead.md`, run that skill instead.
- Otherwise, take each `*<slot: …>*` placeholder's own text as its binding.
- If `lifecycle-kit/templates/lead.md` does not exist, Checkwright is not installed here, or not with the kit that ships this template. Say so, and stop: the `install` skill installs it.
