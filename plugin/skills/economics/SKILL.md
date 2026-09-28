---
name: economics
description: Runs Checkwright's post-iteration economics report on what the iteration cost. Run it only when the user asks for it.
---

Execute the template at `drift-kit/templates/economics.md` in this repository.

- If this repository has a skill of its own that binds `drift-kit/templates/economics.md`, run that skill instead.
- Otherwise, take each `*<slot: …>*` placeholder's own text as its binding.
- If `drift-kit/templates/economics.md` does not exist, Checkwright is not installed here, or not with the kit that ships this template. Say so, and stop: the `install` skill installs it.
