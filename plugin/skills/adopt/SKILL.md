---
name: adopt
description: Fits Checkwright's knobs to this repository's layout and triages the first reds after an install, with the user. Run it only when the user asks for it.
---

Execute the template at `gate-sdk/templates/adopt.md` in this repository.

- If this repository has a skill of its own that binds `gate-sdk/templates/adopt.md`, run that skill instead.
- Otherwise, take each `*<slot: …>*` placeholder's own text as its binding.
- If `gate-sdk/templates/adopt.md` does not exist, Checkwright is not installed here, or not with the kit that ships this template. Say so, and stop: the `install` skill installs it.
