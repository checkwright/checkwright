---
name: release-sweep
description: Runs Checkwright's deprecation walk at a major release, giving every deprecation marker a disposition. Run it only when the user asks for it.
---

Execute the template at `lifecycle-kit/templates/release-sweep.md` in this repository.

- If this repository has a skill of its own that binds `lifecycle-kit/templates/release-sweep.md`, run that skill instead.
- Otherwise, take each `*<slot: …>*` placeholder's own text as its binding.
- If `lifecycle-kit/templates/release-sweep.md` does not exist, Checkwright is not installed here, or not with the kit that ships this template. Say so, and stop: the `install` skill installs it.
