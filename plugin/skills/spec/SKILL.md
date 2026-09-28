---
name: spec
description: Runs Checkwright's spec stage, which authors the design amendments for the features an iteration promotes. Run it only when the user asks for it.
---

Execute the template at `lifecycle-kit/templates/stages/spec.md` in this repository.

- If this repository has a skill of its own that binds `lifecycle-kit/templates/stages/spec.md`, run that skill instead.
- Otherwise, take each `*<slot: …>*` placeholder's own text as its binding.
- If `lifecycle-kit/templates/stages/spec.md` does not exist, Checkwright is not installed here, or not with the kit that ships this template. Say so, and stop: the `install` skill installs it.
