---
title: Install
---

# Install

Vendor the kit, then:

1. Copy the config into place.

2. Give the queue file its header and each evidence file its skeleton — the
   stage-stamp file and the lesson-disposition file
   (`LESSON_EVIDENCE_FILE`, both boundary-reset to their header).
   The queue header line:

       ## Iteration: —  [stage: scope]

   The stage-stamp skeleton:

       # contract: lifecycle-kit/SPEC.md §check-stage-evidence

       ---

   The lesson-disposition skeleton:

       # contract: lifecycle-kit/SPEC.md §check-lesson-disposition

3. Adopt the skills in your agent-skill directory.
