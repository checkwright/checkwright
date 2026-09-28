---
name: scope
description: Runs Checkwright's scope stage, which bounds an iteration's units and promotes them into the active queue. Run it only when the user asks for it.
---

Execute the template at `lifecycle-kit/templates/stages/scope.md` in this repository.

- If this repository has a skill of its own that binds `lifecycle-kit/templates/stages/scope.md`, run that skill instead.
- Otherwise, take each `*<slot: …>*` placeholder's own text as its binding.
- If `lifecycle-kit/templates/stages/scope.md` does not exist, Checkwright is not installed here, or not with the kit that ships this template. Say so, and stop: the `install` skill installs it.
