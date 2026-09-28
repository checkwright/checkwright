---
name: scope
description: Runs the fixture's scope stage. Run it only when the user asks for it.
---

Execute the template at `templates/scope.md` in this repository.

- If this repository has a skill of its own that binds `templates/scope.md`, run that skill instead.
- Otherwise, take each `*<slot: …>*` placeholder's own text as its binding.
- If `templates/scope.md` does not exist, Checkwright is not installed here, or not with the kit that ships this template. Say so, and stop: the `install` skill installs it.
