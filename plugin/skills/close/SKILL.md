---
name: close
description: Runs Checkwright's close stage, which harvests an iteration's lessons, housekeeps and decides its release. Run it only when the user asks for it.
---

Execute the template at `lifecycle-kit/templates/stages/close.md` in this repository.

- If this repository has a skill of its own that binds `lifecycle-kit/templates/stages/close.md`, run that skill instead.
- Otherwise, take each `*<slot: …>*` placeholder's own text as its binding.
- If `lifecycle-kit/templates/stages/close.md` does not exist, Checkwright is not installed here, or not with the kit that ships this template. Say so, and stop: the `install` skill installs it.
