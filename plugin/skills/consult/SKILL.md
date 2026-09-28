---
name: consult
description: Runs Checkwright's consult session, where the operator rules on strategy and each ruling lands in a governed surface. Run it only when the user asks for it.
---

Execute the template at `lifecycle-kit/templates/consult.md` in this repository.

- If this repository has a skill of its own that binds `lifecycle-kit/templates/consult.md`, run that skill instead.
- Otherwise, take each `*<slot: …>*` placeholder's own text as its binding.
- If `lifecycle-kit/templates/consult.md` does not exist, Checkwright is not installed here, or not with the kit that ships this template. Say so, and stop: the `install` skill installs it.
