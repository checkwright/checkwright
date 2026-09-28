---
name: align
description: Runs Checkwright's align stage, the cross-spec audit that closes an iteration's design before build. Run it only when the user asks for it.
---

Execute the template at `lifecycle-kit/templates/stages/align.md` in this repository.

- If this repository has a skill of its own that binds `lifecycle-kit/templates/stages/align.md`, run that skill instead.
- Otherwise, take each `*<slot: …>*` placeholder's own text as its binding.
- If `lifecycle-kit/templates/stages/align.md` does not exist, Checkwright is not installed here, or not with the kit that ships this template. Say so, and stop: the `install` skill installs it.
